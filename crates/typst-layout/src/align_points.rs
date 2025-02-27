use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

use typst_library::layout::{Abs, AlignPointId};

macro_rules! debugln {
    ($($t:tt)*) => {
        //println!($($t)*)
    };
}

macro_rules! debug {
    ($($t:tt)*) => {
        //print!($($t)*)
    };
}

// TODO: use SmallVec and some small hashmaps version to avoid allocations
// in the usual case where only one or two align points are used.

// TODO: optimize for the usual case of inline layout: 1 zone, 1 align point (baseline).

#[derive(Debug, Default)]
pub struct AlignPointsEngine {
    id_to_node: HashMap<AlignPointId, usize>,
    nodes: Vec<Node>,
    requirements: Vec<HashSet<usize>>,
    groups: Vec<GroupInfo>,
    is_rtl: bool,
}

enum NodeType {
    ZoneSeparator(usize),
    AlignPoint(AlignPointId),
}

impl std::fmt::Debug for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NodeType::ZoneSeparator(k) => k.fmt(f),
            NodeType::AlignPoint(id) => id.fmt(f),
        }
    }
}

#[derive(Debug)]
struct Node {
    ty: NodeType,
    position: Abs,
    edges: HashMap<usize, Relation>,
}

#[derive(Clone, Copy)]
struct Relation {
    min_offset: Abs,
    max_offset: Abs,
}

impl Relation {
    fn is_fixed(&self) -> bool {
        self.max_offset.approx_eq(self.min_offset)
    }
}

impl std::fmt::Debug for Relation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{:.2}, {:.2}]", self.min_offset.to_mm(), self.max_offset.to_mm())
    }
}

#[derive(Debug)]
struct GroupInfo {
    parent: usize,
    depth: usize,
}

impl GroupInfo {
    fn new(k: usize) -> Self {
        Self { parent: k, depth: 0 }
    }
}

impl AlignPointsEngine {
    pub fn new(zones: usize, is_rtl: bool) -> Self {
        let mut nodes = Vec::with_capacity(zones + 8);
        let mut groups = Vec::with_capacity(zones + 8);
        let mut requirements = Vec::with_capacity(zones + 8);
        for k in 0..=zones {
            nodes.push(Node {
                ty: NodeType::ZoneSeparator(k),
                position: Abs::zero(),
                edges: Default::default(),
            });
            groups.push(GroupInfo::new(k));
        }
        requirements.resize_with(zones + 1, Default::default);
        let mut r = Self {
            id_to_node: Default::default(),
            nodes,
            requirements,
            groups,
            is_rtl,
        };
        for k in 0..zones {
            r.add_edge(k, k + 1, Abs::zero(), Abs::inf());
        }
        r
    }

    pub fn add_point(
        &mut self,
        id: AlignPointId,
        span: Range<usize>,
        mut before: Abs,
        mut after: Abs,
    ) {
        let k = match self.id_to_node.entry(id.clone()) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let k = self.nodes.len();
                e.insert(k);
                self.nodes.push(Node {
                    ty: NodeType::AlignPoint(id),
                    position: Abs::zero(),
                    edges: Default::default(),
                });
                self.groups.push(GroupInfo { parent: k, depth: 0 });
                self.requirements.push(Default::default());
                k
            }
        };
        if self.is_rtl {
            std::mem::swap(&mut before, &mut after);
        }
        self.add_edge(span.start, k, before, Abs::inf());
        self.add_edge(k, span.end, after, Abs::inf());
    }

    pub fn add_relation(
        &mut self,
        parent: AlignPointId,
        child: AlignPointId,
        delta: Abs,
    ) {
        debugln!("add relation: {parent:?} to {child:?} >= {:.2}", delta.to_mm());
        let i = self.id_to_node[&parent];
        let j = self.id_to_node[&child];
        if delta >= Abs::zero() {
            self.add_edge(i, j, delta, delta);
        } else {
            self.add_edge(j, i, -delta, -delta);
        }
        self.merge_groups(i, j);
    }

    fn find_group(&self, mut k: usize) -> usize {
        while self.groups[k].parent != k {
            k = self.groups[k].parent;
        }
        k
    }

    fn merge_groups(&mut self, i: usize, j: usize) {
        let i = self.find_group(i);
        let j = self.find_group(j);
        if i != j {
            match self.groups[i].depth.cmp(&self.groups[j].depth) {
                Ordering::Less => {
                    self.groups[i].parent = j;
                }
                Ordering::Equal => {
                    self.groups[i].parent = j;
                    self.groups[j].depth += 1;
                }
                Ordering::Greater => {
                    self.groups[j].parent = i;
                }
            }
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, min_offset: Abs, max_offset: Abs) {
        debug_assert!(max_offset.fits(min_offset));
        match self.nodes[from].edges.entry(to) {
            Entry::Occupied(mut e) => {
                let rel = e.get_mut();
                rel.min_offset.set_max(min_offset);
                rel.max_offset.set_min(max_offset);
                /*if !rel.max_offset.fits(rel.min_offset) {
                    // TODO: error? ignore?
                    eprintln!("Incompatible constraints");
                }*/
            }
            Entry::Vacant(e) => {
                e.insert(Relation { min_offset, max_offset });
                self.requirements[to].insert(from);
            }
        }
    }

    pub fn set_zone_size(&mut self, zone: usize, size: Abs) {
        self.add_edge(zone, zone + 1, size, size);
    }

    pub fn set_min_zone_size(&mut self, zone: usize, min_size: Abs) {
        self.add_edge(zone, zone + 1, min_size, Abs::inf());
    }

    pub fn set_min_span_size(&mut self, zones: Range<usize>, min_size: Abs) {
        self.add_edge(zones.start, zones.end, min_size, Abs::inf());
    }

    pub fn is_empty(&self) -> bool {
        self.id_to_node.is_empty()
    }

    pub fn compute_positions(&mut self) {
        if !self.is_empty() {
            debugln!("compute:");
            debugln!("{:?}", self.relations());
        }

        // Topological sort.
        debugln!("Requirements: {:?}", self.requirements);
        let mut order = Vec::with_capacity(self.nodes.len());
        let mut next = Vec::with_capacity(self.nodes.len());
        next.push(0);
        while let Some(i) = next.pop() {
            order.push(i);
            for &j in self.nodes[i].edges.keys() {
                self.requirements[j].remove(&i);
                if self.requirements[j].is_empty() {
                    next.push(j);
                }
            }
        }
        debugln!("Order: {order:?}");
        debugln!("Remaining requirements: {:?}", self.requirements);
        if order.len() != self.nodes.len() {
            debugln!("Circular dependencies.");
            return;
        }

        for k in 0..self.groups.len() {
            self.groups[k].parent = self.find_group(k);
        }
        let mut positions = vec![Abs::zero(); self.nodes.len()];
        // Allow multiple passes, but in most cases 1 or 2 is enough.
        for pass in 1..20 {
            debugln!("Pass {pass}");
            let mut changed = false;
            for &k1 in &order {
                let node1 = &self.nodes[k1];
                for (&k2, relation) in &node1.edges {
                    if relation.is_fixed() {
                        if !positions[k2].approx_eq(positions[k1] + relation.min_offset) {
                            if positions[k2] < positions[k1] + relation.min_offset {
                                debugln!(
                                    "set {:?} to {:?}+{:.2} ({:.2} -> {:.2})",
                                    self.nodes[k2].ty,
                                    self.nodes[k1].ty,
                                    relation.min_offset.to_mm(),
                                    positions[k2].to_mm(),
                                    (positions[k1] + relation.min_offset).to_mm()
                                );
                                positions[k2] = positions[k1] + relation.min_offset;
                            } else {
                                debugln!(
                                    "set {:?} to {:?}-{:.2} ({:.2} -> {:.2})",
                                    self.nodes[k1].ty,
                                    self.nodes[k2].ty,
                                    relation.min_offset.to_mm(),
                                    positions[k1].to_mm(),
                                    (positions[k2] - relation.min_offset).to_mm()
                                );
                                positions[k1] = positions[k2] - relation.min_offset;
                            }
                            changed = true;
                        }
                    } else {
                        let offset = positions[k2] - positions[k1];
                        if !offset.fits(relation.min_offset) {
                            debugln!(
                                "push {:?} {:.2} from {:?} ({:.2} -> {:.2})",
                                self.nodes[k2].ty,
                                relation.min_offset.to_mm(),
                                self.nodes[k1].ty,
                                positions[k2].to_mm(),
                                (positions[k1] + relation.min_offset).to_mm(),
                            );
                            positions[k2] = positions[k1] + relation.min_offset;
                            changed = true;
                        }
                    }
                }
            }
            debug!("  ->");
            for (node, pos) in self.nodes.iter().zip(&positions) {
                debug!(" {:?}:{:.2}", node.ty, pos.to_mm());
            }
            debugln!();
            if !changed {
                for (node, &p) in self.nodes.iter_mut().zip(&positions) {
                    node.position = p;
                }
                return;
            }
        }
        debugln!("Can't compute positions.");
    }

    pub fn get_position(&self, id: &AlignPointId) -> Abs {
        let k = self.id_to_node[id];
        self.nodes[k].position
    }

    pub fn get_zone_position(&self, zone: usize) -> Abs {
        self.nodes[zone].position
    }

    pub fn get_zone_size(&self, zone: usize) -> Abs {
        self.nodes[zone + 1].position - self.nodes[zone].position
    }

    pub fn get_extra_space(&self, id: &AlignPointId) -> Abs {
        let k = self.id_to_node[id];
        let mut extra_right = Abs::inf();
        let g = self.groups[k].parent;
        for (node, group) in self.nodes.iter().zip(&self.groups) {
            if group.parent == g {
                let p = node.position;
                for (&target, relation) in &node.edges {
                    if self.groups[target].parent != g {
                        extra_right.set_min(
                            (self.nodes[target].position - p) - relation.min_offset,
                        );
                    }
                }
            }
        }
        extra_right
    }

    #[cfg(debug_assertions)]
    pub fn relations(&self) -> impl '_ + std::fmt::Debug {
        struct Relations<'a>(&'a AlignPointsEngine);
        impl std::fmt::Debug for Relations<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                for node in &self.0.nodes {
                    write!(f, "{:?}:", node.ty)?;
                    for (&k, rel) in &node.edges {
                        write!(f, " {:?}:{rel:?}", self.0.nodes[k].ty)?;
                    }
                    writeln!(f)?;
                }
                Ok(())
            }
        }
        Relations(self)
    }

    #[cfg(debug_assertions)]
    pub fn positions(&self) -> impl '_ + std::fmt::Debug {
        struct Positions<'a>(&'a AlignPointsEngine);
        impl std::fmt::Debug for Positions<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let mut a = self
                    .0
                    .nodes
                    .iter()
                    .map(|node| (node.position, &node.ty))
                    .collect::<Vec<_>>();
                a.sort_unstable_by_key(|p| p.0);
                for (p, id) in a {
                    writeln!(f, "{id:?}: {:.2}", p.to_mm())?;
                }
                Ok(())
            }
        }
        Positions(self)
    }
}
