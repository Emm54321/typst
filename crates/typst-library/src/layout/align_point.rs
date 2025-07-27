use std::sync::atomic::{AtomicUsize, Ordering};

use smallvec::SmallVec;

use typst_utils::Numeric;

use crate::foundations::{Str, elem};
use crate::layout::{Abs, Point};

static UNIQUE_ID: AtomicUsize = AtomicUsize::new(0);

/// An align point identifier. Usually a name, but can be some unique id.
#[derive(Clone, Eq, Hash, PartialEq)]
pub enum AlignPointId {
    Named(Str),
    Unique(usize),
}

impl AlignPointId {
    /// Create an unique align point id.
    pub fn unique() -> Self {
        Self::Unique(UNIQUE_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl From<Str> for AlignPointId {
    fn from(name: Str) -> Self {
        Self::Named(name)
    }
}

impl From<&Str> for AlignPointId {
    fn from(name: &Str) -> Self {
        Self::Named(name.clone())
    }
}

impl std::fmt::Debug for AlignPointId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AlignPointId::Named(s) => s.fmt(f),
            AlignPointId::Unique(x) => write!(f, "<{x}>"),
        }
    }
}

/// Element used for alignment.
///
/// Align points with the same name will be aligned in stacks, grids, or inline contexts.
//
// TODO: possible parameters:
// - priority
// - some kind of scope
// - some single use flag
#[elem]
pub struct AlignPointElem {
    /// The name of the align point.
    #[positional]
    pub name: Str,

    /// Should it be used for horizontal alignment?
    #[default(true)]
    pub horizontal: bool,

    /// Should it be used for vertical alignment?
    #[default(true)]
    pub vertical: bool,
}

/// Hold a set of align points.
#[derive(Clone, Default, Hash)]
pub struct AlignPoints {
    // TODO: use some better data structure (e.g. some small hashmap).
    points: SmallVec<[(Point, AlignPointId, bool, bool); 2]>,
}

impl AlignPoints {
    /// Returns `true` if there is no align point.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Returns `true` if there is an horizontal align point.
    pub fn has_horizontal_points(&self) -> bool {
        self.points
            .iter()
            .any(|(_point, _id, horizontal, _vertical)| *horizontal)
    }

    /// Returns `true` if there is a vertical align point.
    pub fn has_vertical_points(&self) -> bool {
        self.points
            .iter()
            .any(|(_point, _id, _horizontal, vertical)| *vertical)
    }

    /// Returns an iterator over align points.
    ///
    /// The iterator item is a tuple with:
    /// - the point position;
    /// - the point id;
    /// - a boolean telling if the point can be used or horizontal alignment.
    /// - a boolean telling if the point can be used or vertical alignment.
    pub fn iter(&self) -> std::slice::Iter<'_, (Point, AlignPointId, bool, bool)> {
        self.points.iter()
    }

    /// Returns an iterator over horizontal align points.
    ///
    /// The items retured are pairs with the point x position and the point id.
    pub fn iter_horizontal(&self) -> impl '_ + Iterator<Item = (Abs, &AlignPointId)> {
        self.points.iter().filter_map(|(point, id, horizontal, _vertical)| {
            if *horizontal { Some((point.x, id)) } else { None }
        })
    }

    /// Returns an iterator over vertical align points.
    ///
    /// The items retured are pairs with the point y position and the point id.
    pub fn iter_vertical(&self) -> impl '_ + Iterator<Item = (Abs, &AlignPointId)> {
        self.points.iter().filter_map(
            |(point, id, _horizontal, vertical)| {
                if *vertical { Some((point.y, id)) } else { None }
            },
        )
    }

    /// Returns a horizontal align point, if any.
    pub fn first_horizontal(&self) -> Option<(Abs, &AlignPointId)> {
        self.points.iter().find_map(|(point, id, horizontal, _vertical)| {
            horizontal.then_some((point.x, id))
        })
    }

    /// Returns a vertical align point, if any.
    pub fn first_vertical(&self) -> Option<(Abs, &AlignPointId)> {
        self.points.iter().find_map(|(point, id, _horizontal, vertical)| {
            vertical.then_some((point.y, id))
        })
    }

    /// Clears all align points.
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// Add an align point.
    pub fn add(
        &mut self,
        position: Point,
        id: AlignPointId,
        horizontal: bool,
        vertical: bool,
    ) {
        // TODO: use some small hashmap to store align points.
        if let Some(k) = self.points.iter().position(|p| p.1 == id) {
            self.points[k].0 = position;
            self.points[k].2 = horizontal;
            self.points[k].3 = vertical;
        } else {
            self.points.push((position, id, horizontal, vertical));
        }
    }

    /// Move all align points by an offset.
    pub fn translate(&mut self, offset: Point) {
        if !offset.is_zero() {
            for (position, ..) in self.points.iter_mut() {
                *position += offset;
            }
        }
    }

    /// Take all align points from a frame, move them with an offset, and
    /// add them to this set.
    pub fn take(&mut self, offset: Point, other: &mut AlignPoints) {
        // Optimize for usual simple cases.
        if !other.points.is_empty() {
            if self.points.is_empty() {
                self.points = std::mem::take(&mut other.points);
                self.translate(offset);
            } else {
                for (position, id, horizontal, vertical) in other.points.drain(..) {
                    self.add(offset + position, id, horizontal, vertical);
                }
            }
        }
    }

    /// Sort by position.
    /// Needed only to make layout consistent across runs when points are added in random
    /// order due to iteration over hashmaps. This should not be needed when there are
    /// no conflicts amoung align points.
    pub fn sort(&mut self) {
        self.points.sort_unstable_by_key(|t| (t.0.y, t.0.x));
    }
}
