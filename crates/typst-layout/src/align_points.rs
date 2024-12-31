use std::collections::hash_map::Entry;
use std::collections::HashMap;

use typst_library::foundations::Str;
use typst_library::layout::Abs;

#[derive(Debug, Default)]
pub struct AlignPointsEngine {
    positioned_points: HashMap<Str, PointInfo>,
}

#[derive(Debug)]
enum PointInfo {
    Parent { position: Abs, before: Abs, after: Abs },
    Child { parent: Str, offset: Abs },
}

impl AlignPointsEngine {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_positioned_point(
        &mut self,
        name: Str,
        position: Abs,
        before: Abs,
        after: Abs,
    ) {
        self.positioned_points
            .insert(name, PointInfo::Parent { position, before, after });
    }

    pub fn compute_positions(
        &mut self,
        groups: impl IntoIterator<Item = impl IntoIterator<Item = (Str, Abs, Abs, Abs)>>,
    ) {
        let mut remaining: Vec<Vec<(Str, Abs, Abs, Abs)>> = Default::default();
        for align_points in groups {
            remaining.push(align_points.into_iter().collect::<Vec<_>>());
        }
        //println!("compute {remaining:?}");
        loop {
            let mut changed = false;
            let mut k = 0;
            while k < remaining.len() {
                let align_points: &[(Str, Abs, Abs, Abs)] = &remaining[k];
                let mut found = None;
                for &(ref name, position, _before, _after) in align_points {
                    if let Some((parent, offset)) = self.get_parent(name) {
                        found =
                            Some((parent, offset, position, remaining.swap_remove(k)));
                        //println!("found {found:?}");
                        break;
                    }
                }
                if let Some((
                    parent,
                    parent_to_ref_offset,
                    old_ref_position,
                    align_points,
                )) = found
                {
                    //println!("parent {parent} parent_to_ref_offset {parent_to_ref_offset:?} old_ref_position {old_ref_position:?}");
                    //println!("align_points {align_points:?}");
                    //TODO: error if there are conflicting align points.
                    let (_new_parent_position, mut max_before, mut max_after) =
                        self.get_position(&parent).unwrap();
                    //println!("new_parent_position {new_parent_position:?} max_before {max_before:?} max_after {max_after:?}");
                    //let new_ref_position = new_parent_position + parent_to_ref_offset;
                    for (name, old_position, before, after) in align_points {
                        //println!("set {name}: {old_position:?} {before:?} {after:?}");
                        let offset =
                            old_position - old_ref_position + parent_to_ref_offset;
                        //println!(
                        //    "offset {offset:?} before {:?} after {:?}",
                        //    before + offset,
                        //    after - offset
                        //);
                        max_before.set_max(before - offset);
                        max_after.set_max(after + offset);
                        if let Entry::Vacant(e) = self.positioned_points.entry(name) {
                            e.insert(PointInfo::Child { parent: parent.clone(), offset });
                        }
                    }
                    //println!("max_before {max_before:?} max_after {max_after:?}");
                    let (_position, before, after) =
                        self.get_group_size_mut(&parent).unwrap();
                    *before = max_before;
                    *after = max_after;
                    changed = true;
                } else {
                    k += 1;
                }
            }
            if !changed {
                if let Some(align_points) = remaining.first() {
                    if let Some(&(ref name, position, before, after)) =
                        align_points.first()
                    {
                        //println!("set position for {name} to {position:?}");
                        self.add_positioned_point(name.clone(), position, before, after);
                    }
                } else {
                    break;
                }
            }
        }
    }

    pub fn clip_positions(&mut self) {
        //println!("before clip: {self:?}");
        for (_name, info) in &mut self.positioned_points.iter_mut() {
            if let PointInfo::Parent { position, before, .. } = info {
                //println!("clip {name} {:?} {:?}", position, before);
                if *before > *position {
                    //println!("adjust");
                    *position = *before;
                }
            }
        }
        //println!("after clip: {self:?}");
    }

    fn get_parent(&self, name: &Str) -> Option<(Str, Abs)> {
        self.positioned_points.get(name).map(|info| match info {
            PointInfo::Parent { .. } => (name.clone(), Abs::zero()),
            PointInfo::Child { parent, offset } => (parent.clone(), *offset),
        })
    }

    pub fn get_position(&self, name: &Str) -> Option<(Abs, Abs, Abs)> {
        self.positioned_points.get(name).map(|info| match info {
            &PointInfo::Parent { position, before, after } => (position, before, after),
            &PointInfo::Child { ref parent, offset } => {
                let (position, before, after) = self.get_position(parent).unwrap();
                (position + offset, before + offset, after - offset)
            }
        })
    }

    pub fn group_sizes(&self) -> impl '_ + Iterator<Item = (Abs, Abs)> {
        self.positioned_points.values().filter_map(|info| match info {
            PointInfo::Parent { before, after, .. } => Some((*before, *after)),
            PointInfo::Child { .. } => None,
        })
    }

    pub fn get_group_size(&self, name: &Str) -> Option<Abs> {
        self.get_parent(name).and_then(|(parent, _offset)| {
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
        name: &Str,
    ) -> Option<(&mut Abs, &mut Abs, &mut Abs)> {
        self.get_parent(name).and_then(|(parent, _offset)| {
            if let Some(PointInfo::Parent { position, before, after }) =
                self.positioned_points.get_mut(&parent)
            {
                Some((position, before, after))
            } else {
                None
            }
        })
    }

    pub fn adjust_positions<'a>(
        &mut self,
        groups: impl IntoIterator<Item = (&'a mut Abs, impl IntoIterator<Item = (Str, Abs)>)>,
    ) {
        for (position, align_points) in groups {
            for (name, pos) in align_points {
                if let Some((position1, ..)) = self.get_position(&name) {
                    *position += position1 - pos;
                    break;
                }
            }
        }
    }
}
