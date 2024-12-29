use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{elem, Content, Packed, Show, Str, StyleChain};

#[elem]
pub struct AlignPointElem {
    #[positional]
    pub name: Str,
}
//
//impl Show for Packed<AlignPointElem> {
//    fn show(&self, _engine: &mut Engine, _styles: StyleChain) -> SourceResult<Content> {
//
//    }
//}
