use typst_library::diag::SourceResult;
use typst_library::engine::Engine;
use typst_library::foundations::StyleChain;
use typst_library::layout::{Axes, FixedAlignment, Fragment, Frame, Regions, Size};

use crate::align_engine::AlignmentInfos;

use super::{layout_cell, Cell};

#[derive(Clone, Debug, Default)]
pub struct CellInfos {
    frame: Option<(Size, Frame, Axes<FixedAlignment>)>,
}

impl CellInfos {}

#[derive(Debug)]
pub struct GridInfos {
    columns: usize,
    cells: Vec<CellInfos>,
    pub horiz_align_engine: Option<AlignmentInfos>,
}

impl GridInfos {
    pub fn new(columns: usize, rows: usize) -> Self {
        Self {
            columns,
            cells: vec![Default::default(); columns * rows],
            horiz_align_engine: None,
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut CellInfos {
        self.cells.get_mut(y * self.columns + x).unwrap()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn layout_cell(
        &mut self,
        engine: &mut Engine,
        cell: &Cell,
        x: usize,
        y: usize,
        disambiguator: usize,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<(Fragment, Axes<FixedAlignment>)> {
        let entry = self.get_mut(x, y);
        if let Some((size, frame, align)) = &entry.frame {
            if regions.size.x == size.x && regions.size.y == size.y {
                return Ok((Fragment::frame(frame.clone()), *align));
            }
        }
        let fragment = layout_cell(cell, engine, disambiguator, styles, regions)?;
        let align = cell.align;
        if fragment.len() == 1 {
            let frame = fragment.as_slice().first().unwrap().clone();
            entry.frame = Some((Size::new(regions.size.x, frame.height()), frame, align));
        }
        //TODO: remove align
        Ok((fragment, align))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn layout_cell_frame_ref<'a>(
        &'a mut self,
        engine: &mut Engine,
        cell: &Cell,
        x: usize,
        y: usize,
        disambiguator: usize,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<(&'a Frame, Axes<FixedAlignment>)> {
        let entry = self.get_mut(x, y);
        let mut insert = true;
        {
            if let Some((size, _frame, _align)) = &entry.frame {
                if regions.size.x == size.x && regions.size.y == size.y {
                    insert = false;
                }
            }
        }
        if insert {
            let fragment = layout_cell(cell, engine, disambiguator, styles, regions)?;
            let align = cell.align;
            let frame = fragment.into_frame();
            let (_size, frame, align) = entry.frame.insert((
                Size::new(regions.size.x, frame.height()),
                frame,
                align,
            ));
            //TODO: remove align
            Ok((frame, *align))
        } else {
            let (_size, frame, align) = &entry.frame.as_ref().unwrap();
            Ok((frame, *align))
        }
    }
}
