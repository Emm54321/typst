use crate::foundations::{elem, Str};

// Just contains the point name for now, but can be turned into
// some usize id with a map to make it Copy.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct AlignPointId(Str);

impl From<Str> for AlignPointId {
    fn from(name: Str) -> Self {
        Self(name)
    }
}

impl From<&Str> for AlignPointId {
    fn from(name: &Str) -> Self {
        Self(name.clone())
    }
}

impl std::fmt::Debug for AlignPointId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
