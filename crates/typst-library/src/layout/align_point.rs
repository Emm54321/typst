use crate::foundations::{elem, Str};

#[elem]
pub struct AlignPointElem {
    #[positional]
    pub name: Str,
}
