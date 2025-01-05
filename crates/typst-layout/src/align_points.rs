use std::collections::hash_map::Entry;
use std::collections::HashMap;

use typst_library::layout::{Abs, AlignPointId};

#[derive(Debug, Default)]
pub struct AlignPointsEngine {
    positioned_points: HashMap<AlignPointId, PointInfo>,
    remaining: Vec<(Abs, Abs, Vec<(AlignPointId, Abs, Abs, Abs)>)>,
}

#[derive(Debug)]
enum PointInfo {
    Parent { position: Abs, before: Abs, after: Abs, min_pos: Abs, max_pos: Abs },
    Child { parent: AlignPointId, offset: Abs },
}

impl AlignPointsEngine {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clear(&mut self) {
        self.positioned_points.clear();
        self.remaining.clear();
    }

    pub fn add_positioned_point(
        &mut self,
        id: AlignPointId,
        position: Abs,
        before: Abs,
        after: Abs,
        min_pos: Abs,
        max_pos: Abs,
    ) {
        self.positioned_points
            .insert(id, PointInfo::Parent { position, before, after, min_pos, max_pos });
    }

    /// Add a group of align points.
    /// Takes the minimal position of an element of the group and a list of align points.
    /// Each align point has an id, a position, and needed space before and after the position.
    pub fn add_group(
        &mut self,
        min_pos: Abs,
        max_pos: Abs,
        group: impl IntoIterator<Item = (AlignPointId, Abs, Abs, Abs)>,
    ) {
        self.remaining
            .push((min_pos, max_pos, group.into_iter().collect::<Vec<_>>()));
    }

    pub fn compute_positions(&mut self) {
        //println!("compute {:?}", self.remaining);
        loop {
            let mut changed = false;
            let mut k = 0;
            while k < self.remaining.len() {
                let &(min_pos, max_pos, ref align_points) = &self.remaining[k];
                let mut found = None;
                for &(ref id, position, _before, _after) in align_points {
                    if let Some((parent, offset)) = self.get_parent(id) {
                        found = Some((
                            parent,
                            offset,
                            position,
                            min_pos,
                            max_pos,
                            self.remaining.swap_remove(k).2,
                        ));
                        //println!("found {found:?}");
                        break;
                    }
                }
                if let Some((
                    parent,
                    parent_to_ref_offset,
                    old_ref_position,
                    min_group_pos,
                    max_group_pos,
                    align_points,
                )) = found
                {
                    //println!("parent {parent:?} parent_to_ref_offset {parent_to_ref_offset:?} old_ref_position {old_ref_position:?}");
                    //println!("align_points {align_points:?}");
                    //TODO: error if there are conflicting align points.
                    let (
                        _new_parent_position,
                        mut max_before,
                        mut max_after,
                        mut new_min_pos,
                        mut new_max_pos,
                    ) = self.get_infos(&parent).unwrap();
                    //println!("new_parent_position {new_parent_position:?} max_before {max_before:?} max_after {max_after:?}");
                    //let new_ref_position = new_parent_position + parent_to_ref_offset;
                    for (id, old_position, before, after) in align_points {
                        //println!("set {id:?}: {old_position:?} {before:?} {after:?}");
                        let offset =
                            old_position - old_ref_position + parent_to_ref_offset;
                        //println!(
                        //    "offset {offset:?} before {:?} after {:?}",
                        //    before - offset,
                        //    after + offset
                        //);
                        max_before.set_max(before - offset);
                        max_after.set_max(after + offset);
                        new_min_pos.set_max(min_group_pos - offset + before);
                        new_max_pos.set_min(max_group_pos - offset - after);
                        if let Entry::Vacant(e) = self.positioned_points.entry(id) {
                            e.insert(PointInfo::Child { parent: parent.clone(), offset });
                        }
                    }
                    //println!("max_before {max_before:?} max_after {max_after:?}");
                    let (_position, before, after, min_parent_pos, max_parent_pos) =
                        self.get_group_size_mut(&parent).unwrap();
                    *before = max_before;
                    *after = max_after;
                    *min_parent_pos = new_min_pos;
                    *max_parent_pos = new_max_pos;
                    changed = true;
                } else {
                    k += 1;
                }
            }
            if !changed {
                if let Some(&(min_pos, max_pos, ref align_points)) =
                    &self.remaining.first()
                {
                    if let Some(&(ref id, position, before, after)) = align_points.first()
                    {
                        //println!("set position for {id:?} to {position:?}");
                        self.add_positioned_point(
                            id.clone(),
                            position,
                            before,
                            after,
                            min_pos,
                            max_pos,
                        );
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    pub fn clip_positions(&mut self) {
        //println!("before clip: {self:?}");
        for (_id, info) in &mut self.positioned_points.iter_mut() {
            if let PointInfo::Parent { position, min_pos, .. } = info {
                //println!("clip {id:?} {:?} {:?}", position, before);
                if *position < *min_pos {
                    //println!("adjust");
                    *position = *min_pos;
                }
            }
        }
        //println!("after clip: {self:?}");
    }

    fn get_parent(&self, id: &AlignPointId) -> Option<(AlignPointId, Abs)> {
        self.positioned_points.get(id).map(|info| match info {
            PointInfo::Parent { .. } => (id.clone(), Abs::zero()),
            PointInfo::Child { parent, offset } => (parent.clone(), *offset),
        })
    }

    pub fn get_infos(&self, id: &AlignPointId) -> Option<(Abs, Abs, Abs, Abs, Abs)> {
        self.positioned_points.get(id).map(|info| match info {
            &PointInfo::Parent { position, before, after, min_pos, max_pos } => {
                (position, before, after, min_pos, max_pos)
            }
            &PointInfo::Child { ref parent, offset } => {
                let (position, before, after, min_pos, max_pos) =
                    self.get_infos(parent).unwrap();
                (
                    position + offset,
                    before + offset,
                    after - offset,
                    min_pos + offset,
                    max_pos + offset,
                )
            }
        })
    }

    pub fn get_position(&self, id: &AlignPointId) -> Option<Abs> {
        self.positioned_points.get(id).map(|info| match info {
            &PointInfo::Parent { position, .. } => position,
            &PointInfo::Child { ref parent, offset } => {
                self.get_position(parent).unwrap() + offset
            }
        })
    }

    pub fn get_pos_range(&self, id: &AlignPointId) -> Option<(Abs, Abs, Abs)> {
        self.positioned_points.get(id).map(|info| match info {
            &PointInfo::Parent { position, min_pos, max_pos, .. } => {
                (position, min_pos, max_pos)
            }
            &PointInfo::Child { ref parent, offset } => {
                let (position, min_pos, max_pos) = self.get_pos_range(parent).unwrap();
                (position + offset, min_pos + offset, max_pos + offset)
            }
        })
    }

    pub fn get_range(&self, id: &AlignPointId) -> Option<(Abs, Abs)> {
        self.positioned_points.get(id).map(|info| match info {
            &PointInfo::Parent { min_pos, max_pos, .. } => (min_pos, max_pos),
            &PointInfo::Child { ref parent, offset } => {
                let (min_pos, max_pos) = self.get_range(parent).unwrap();
                (min_pos + offset, max_pos + offset)
            }
        })
    }

    pub fn set_pos_range(
        &mut self,
        id: &AlignPointId,
        new_min_pos: Abs,
        new_max_pos: Abs,
    ) -> Option<(Abs, Abs)> {
        if let Some(info) = self.positioned_points.get_mut(id) {
            match info {
                PointInfo::Parent { min_pos, max_pos, .. } => {
                    let old_min = std::mem::replace(min_pos, new_min_pos.max(*min_pos));
                    let old_max = std::mem::replace(max_pos, new_max_pos.min(*max_pos));
                    Some((old_min, old_max))
                }
                PointInfo::Child { ref parent, offset } => {
                    let offset = *offset;
                    let parent = parent.clone();
                    let Some(PointInfo::Parent { min_pos, max_pos, .. }) =
                        self.positioned_points.get_mut(&parent)
                    else {
                        unreachable!()
                    };
                    let old_min =
                        std::mem::replace(min_pos, (new_min_pos - offset).max(*min_pos))
                            + offset;
                    let old_max =
                        std::mem::replace(max_pos, (new_max_pos - offset).min(*max_pos))
                            + offset;
                    Some((old_min, old_max))
                }
            }
        } else {
            None
        }
    }

    pub fn group_sizes(&self) -> impl '_ + Iterator<Item = Abs> {
        self.positioned_points.values().filter_map(|info| match info {
            PointInfo::Parent { before, after, .. } => Some(*before + *after),
            PointInfo::Child { .. } => None,
        })
    }

    pub fn get_group_size(&self, id: &AlignPointId) -> Option<Abs> {
        self.get_parent(id).and_then(|(parent, _offset)| {
            if let &PointInfo::Parent { before, after, .. } =
                &self.positioned_points[&parent]
            {
                Some(before + after)
            } else {
                None
            }
        })
    }

    fn get_group_size_mut(
        &mut self,
        id: &AlignPointId,
    ) -> Option<(&mut Abs, &mut Abs, &mut Abs, &mut Abs, &mut Abs)> {
        self.get_parent(id).and_then(|(parent, _offset)| {
            if let Some(PointInfo::Parent { position, before, after, min_pos, max_pos }) =
                self.positioned_points.get_mut(&parent)
            {
                Some((position, before, after, min_pos, max_pos))
            } else {
                None
            }
        })
    }

    pub fn adjust_positions<'a>(
        &mut self,
        groups: impl IntoIterator<
            Item = (&'a mut Abs, impl IntoIterator<Item = (AlignPointId, Abs)>),
        >,
    ) {
        for (position, align_points) in groups {
            for (id, pos) in align_points {
                if let Some(position1) = self.get_position(&id) {
                    *position += position1 - pos;
                    break;
                }
            }
        }
    }
}
