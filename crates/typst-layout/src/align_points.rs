use std::collections::hash_map::Entry;
use std::collections::HashMap;

use typst_library::layout::{Abs, AlignPointId};

#[derive(Debug, Default)]
pub struct AlignPointsEngine {
    positioned_points: HashMap<AlignPointId, PointType>,
    remaining: Vec<(Abs, Abs, Vec<AlignItem>)>,
}

#[derive(Clone, Copy, Debug)]
pub struct PointInfo {
    pub position: Abs,
    pub before: Abs,
    pub after: Abs,
    pub min_pos: Abs,
    pub max_pos: Abs,
}

impl PointInfo {
    fn translate(&mut self, offset: Abs) {
        self.position += offset;
        self.before += offset;
        self.after -= offset;
        self.min_pos += offset;
        self.max_pos += offset;
    }
}

impl Default for PointInfo {
    fn default() -> Self {
        Self {
            position: Abs::zero(),
            before: Abs::zero(),
            after: Abs::zero(),
            min_pos: Abs::zero(),
            max_pos: Abs::inf(),
        }
    }
}

#[derive(Debug)]
enum PointType {
    Parent { info: PointInfo },
    Child { parent: AlignPointId, offset: Abs },
}

#[derive(Clone, Debug)]
pub struct AlignItem {
    pub id: AlignPointId,
    pub position: Abs,
    pub before: Abs,
    pub after: Abs,
}

impl AlignPointsEngine {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_positioned_point(&mut self, id: AlignPointId, info: PointInfo) {
        self.positioned_points.insert(id, PointType::Parent { info });
    }

    /// Add a group of align points.
    /// Takes the minimal position of an element of the group and a list of align points.
    /// Each align point has an id, a position, and needed space before and after the position.
    pub fn add_group(&mut self, min_pos: Abs, max_pos: Abs, group: Vec<AlignItem>) {
        self.remaining.push((min_pos, max_pos, group));
    }

    pub fn is_empty(&self) -> bool {
        self.remaining.is_empty() && self.positioned_points.is_empty()
    }

    pub fn compute_positions(&mut self) {
        //println!("compute {:?}", self.remaining);
        loop {
            let mut changed = false;
            let mut k = 0;
            while k < self.remaining.len() {
                let &(min_pos, max_pos, ref align_points) = &self.remaining[k];
                let mut found = None;
                for item in align_points {
                    if let Some((parent, offset)) = self.get_parent(&item.id) {
                        found = Some((
                            parent,
                            offset,
                            item.position,
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
                    let mut new_info = self.get_infos(&parent).unwrap();
                    //println!("new_parent_position {new_parent_position:?} max_before {max_before:?} max_after {max_after:?}");
                    //let new_ref_position = new_parent_position + parent_to_ref_offset;
                    for item in align_points {
                        //println!("set {id:?}: {old_position:?} {before:?} {after:?}");
                        let offset =
                            item.position - old_ref_position + parent_to_ref_offset;
                        //println!(
                        //    "offset {offset:?} before {:?} after {:?}",
                        //    before - offset,
                        //    after + offset
                        //);
                        new_info.before.set_max(item.before - offset);
                        new_info.after.set_max(item.after + offset);
                        new_info.min_pos.set_max(min_group_pos - offset + item.before);
                        new_info.max_pos.set_min(max_group_pos - offset - item.after);
                        if let Entry::Vacant(e) = self.positioned_points.entry(item.id) {
                            e.insert(PointType::Child { parent: parent.clone(), offset });
                        }
                    }
                    //println!("max_before {max_before:?} max_after {max_after:?}");
                    let info = self.get_infos_mut(&parent).unwrap();
                    info.before = new_info.before;
                    info.after = new_info.after;
                    info.min_pos = new_info.min_pos;
                    info.max_pos = new_info.max_pos;
                    changed = true;
                } else {
                    k += 1;
                }
            }
            if !changed {
                if let Some(&(min_pos, max_pos, ref align_points)) =
                    &self.remaining.first()
                {
                    if let Some(item) = align_points.first() {
                        //println!("set position for {id:?} to {position:?}");
                        self.add_positioned_point(
                            item.id.clone(),
                            PointInfo {
                                position: item.position,
                                before: item.before,
                                after: item.after,
                                min_pos,
                                max_pos,
                            },
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
        for (_id, ty) in &mut self.positioned_points.iter_mut() {
            if let PointType::Parent { info } = ty {
                if info.position < info.min_pos {
                    info.position = info.min_pos;
                }
            }
        }
    }

    fn get_parent(&self, id: &AlignPointId) -> Option<(AlignPointId, Abs)> {
        self.positioned_points.get(id).map(|info| match info {
            PointType::Parent { .. } => (id.clone(), Abs::zero()),
            PointType::Child { parent, offset } => (parent.clone(), *offset),
        })
    }

    pub fn has_point(&self, id: &AlignPointId) -> bool {
        self.positioned_points.contains_key(id)
    }

    pub fn get_infos(&self, id: &AlignPointId) -> Option<PointInfo> {
        self.positioned_points.get(id).map(|ty| match *ty {
            PointType::Parent { info } => info,
            PointType::Child { ref parent, offset } => {
                let mut info = self.get_infos(parent).unwrap();
                info.translate(offset);
                info
            }
        })
    }

    pub fn get_position(&self, id: &AlignPointId) -> Option<Abs> {
        self.positioned_points.get(id).map(|ty| match *ty {
            PointType::Parent { info } => info.position,
            PointType::Child { ref parent, offset } => {
                self.get_position(parent).unwrap() + offset
            }
        })
    }

    pub fn get_pos_range(&self, id: &AlignPointId) -> Option<(Abs, Abs, Abs)> {
        self.positioned_points.get(id).map(|ty| match *ty {
            PointType::Parent { info } => (info.position, info.min_pos, info.max_pos),
            PointType::Child { ref parent, offset } => {
                let (position, min_pos, max_pos) = self.get_pos_range(parent).unwrap();
                (position + offset, min_pos + offset, max_pos + offset)
            }
        })
    }

    pub fn get_range(&self, id: &AlignPointId) -> Option<(Abs, Abs)> {
        self.positioned_points.get(id).map(|ty| match *ty {
            PointType::Parent { info } => (info.min_pos, info.max_pos),
            PointType::Child { ref parent, offset } => {
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
        if let Some(ty) = self.positioned_points.get_mut(id) {
            match ty {
                PointType::Parent { info } => {
                    let min_pos = info.min_pos;
                    let max_pos = info.max_pos;
                    let old_min =
                        std::mem::replace(&mut info.min_pos, new_min_pos.max(min_pos));
                    let old_max =
                        std::mem::replace(&mut info.max_pos, new_max_pos.min(max_pos));
                    Some((old_min, old_max))
                }
                PointType::Child { ref parent, offset } => {
                    let offset = *offset;
                    let parent = parent.clone();
                    let Some(PointType::Parent { info }) =
                        self.positioned_points.get_mut(&parent)
                    else {
                        unreachable!()
                    };
                    let min_pos = info.min_pos;
                    let max_pos = info.max_pos;
                    let old_min = std::mem::replace(
                        &mut info.min_pos,
                        (new_min_pos - offset).max(min_pos),
                    ) + offset;
                    let old_max = std::mem::replace(
                        &mut info.max_pos,
                        (new_max_pos - offset).min(max_pos),
                    ) + offset;
                    Some((old_min, old_max))
                }
            }
        } else {
            None
        }
    }

    pub fn group_ranges(&self) -> impl '_ + Iterator<Item = (Abs, Abs)> {
        self.positioned_points.values().filter_map(|ty| match ty {
            PointType::Parent { info } => Some((info.min_pos, info.max_pos)),
            PointType::Child { .. } => None,
        })
    }

    pub fn group_sizes(&self) -> impl '_ + Iterator<Item = Abs> {
        self.positioned_points.values().filter_map(|ty| match ty {
            PointType::Parent { info } => Some(info.before + info.after),
            PointType::Child { .. } => None,
        })
    }

    pub fn get_group_size(&self, id: &AlignPointId) -> Option<Abs> {
        self.get_parent(id).and_then(|(parent, _offset)| {
            if let PointType::Parent { info } = &self.positioned_points[&parent] {
                Some(info.before + info.after)
            } else {
                None
            }
        })
    }

    pub fn get_group_id(&self, id: &AlignPointId) -> Option<AlignPointId> {
        self.get_parent(id).map(|(parent, _offset)| parent)
    }

    fn get_infos_mut(&mut self, id: &AlignPointId) -> Option<&mut PointInfo> {
        self.get_parent(id).and_then(|(parent, _offset)| {
            if let Some(PointType::Parent { info }) =
                self.positioned_points.get_mut(&parent)
            {
                Some(info)
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
