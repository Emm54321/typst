use std::collections::hash_map::Entry;
use std::collections::HashMap;

use typst_library::foundations::Str;
use typst_library::layout::Abs;

#[derive(Debug, Default)]
pub struct AlignPointsHandler {
    positioned_points: HashMap<Str, Abs>,
}

impl AlignPointsHandler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_positioned_point(&mut self, name: Str, offset: Abs) {
        self.positioned_points.insert(name, offset);
    }

    pub fn compute_deltas(
        &mut self,
        groups: impl IntoIterator<Item = impl IntoIterator<Item = (Str, Abs)>>,
    ) {
        let mut remaining: Vec<Vec<(Str, Abs)>> = Default::default();
        for align_points in groups {
            remaining.push(align_points.into_iter().collect::<Vec<_>>());
        }
        loop {
            let mut changed = false;
            let mut k = 0;
            while k < remaining.len() {
                let align_points: &[(Str, Abs)] = &remaining[k];
                let mut delta = None;
                for &(ref name, position) in align_points {
                    if let Some(&target) = self.positioned_points.get(name) {
                        delta = Some(target - position);
                    }
                }
                if let Some(delta) = delta {
                    let v = remaining.swap_remove(k);
                    //TODO: error if there are conflicting align points.
                    for (name, position) in v {
                        if let Entry::Vacant(e) = self.positioned_points.entry(name) {
                            e.insert(position + delta);
                        }
                    }
                    changed = true;
                } else {
                    k += 1;
                }
            }
            if !changed {
                //TODO: what if there are still unpositioned groups?
                break;
            }
        }
    }

    pub fn adjust_positions<'a>(
        &mut self,
        groups: impl IntoIterator<Item = (&'a mut Abs, impl IntoIterator<Item = (Str, Abs)>)>,
    ) {
        for (position, align_points) in groups {
            for (name, pos) in align_points {
                if let Some(&p) = self.positioned_points.get(&name) {
                    *position += p - pos;
                    break;
                }
            }
        }
    }
}
