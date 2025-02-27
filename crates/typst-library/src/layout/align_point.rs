use std::sync::atomic::{AtomicUsize, Ordering};

use crate::foundations::{elem, Str};

static UNIQUE_ID: AtomicUsize = AtomicUsize::new(0);

// Just contains the point name for now, but can be turned into
// some usize id with a map to make it Copy.
#[derive(Clone, Eq, Hash, PartialEq)]
pub enum AlignPointId {
    Named(Str),
    Unique(usize),
}

impl AlignPointId {
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

// Possible parameters:
// - priority
// - some kind of scope
#[elem]
pub struct AlignPointElem {
    #[positional]
    pub name: Str,

    #[default(true)]
    pub horizontal: bool,

    #[default(true)]
    pub vertical: bool,
}
