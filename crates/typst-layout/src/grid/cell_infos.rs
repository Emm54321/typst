use typst_library::diag::SourceResult;
use typst_library::engine::Engine;
use typst_library::foundations::StyleChain;
use typst_library::layout::{Fragment, Frame, Regions, Size};

use crate::align_points::AlignPointsEngine;

use super::Cell;

#[derive(Clone, Debug, Default)]
pub struct CellInfos {
    frame: Option<(Size, Frame)>,
}

impl CellInfos {}

#[derive(Debug)]
pub struct GridInfos {
    columns: usize,
    rows: usize,
    cells: Vec<CellInfos>,
    pub horiz_align_engine: AlignPointsEngine,
}

impl GridInfos {
    pub fn new(columns: usize, rows: usize) -> Self {
        Self {
            columns,
            rows,
            cells: vec![Default::default(); columns * rows],
            horiz_align_engine: Default::default(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &CellInfos {
        &self.cells[y * self.columns + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut CellInfos {
        self.cells.get_mut(y * self.columns + x).unwrap()
    }

    pub fn get_frame(&self, x: usize, y: usize) -> Option<(Size, &Frame)> {
        self.get(x, y).frame.as_ref().map(|(size, frame)| (*size, frame))
    }

    pub fn set_frame(&mut self, x: usize, y: usize, size: Size, frame: Frame) {
        self.get_mut(x, y).frame = Some((size, frame))
    }

    pub fn layout_cell(
        &mut self,
        engine: &mut Engine,
        cell: &Cell,
        x: usize,
        y: usize,
        disambiguator: usize,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment> {
        println!("layout {x:?},{y:?} for {:?}", regions.size);
        let entry = self.get_mut(x, y);
        if let Some((size, frame)) = &entry.frame {
            if regions.size.x == size.x && regions.size.y >= size.y {
                println!("cached frame");
                return Ok(Fragment::frame(frame.clone()));
            }
        }
        let fragment = cell.layout(engine, disambiguator, styles, regions)?;
        println!("{} frames", fragment.len());
        if fragment.len() == 1 {
            let frame = fragment.as_slice().first().unwrap().clone();
            println!("frame size {:?}", frame.size());
            entry.frame = Some((Size::new(regions.size.x, frame.height()), frame));
        }
        Ok(fragment)
    }

    pub fn layout_cell_frame_ref<'a>(
        &'a mut self,
        engine: &mut Engine,
        cell: &Cell,
        x: usize,
        y: usize,
        disambiguator: usize,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<&'a Frame> {
        println!("layout frame ref {x:?},{y:?} for {:?}", regions.size);
        let entry = self.get_mut(x, y);
        let mut insert = true;
        {
            if let Some((size, _frame)) = &entry.frame {
                if regions.size.x == size.x && regions.size.y >= size.y {
                    println!("cached frame");
                    insert = false;
                    //return Ok(frame); confuses the borrow checker
                }
            }
        }
        if insert {
            let frame = cell.layout(engine, disambiguator, styles, regions)?.into_frame();
            println!("frame size {:?}", frame.size());
            Ok(&entry
                .frame
                .insert((Size::new(regions.size.x, frame.height()), frame))
                .1)
        } else {
            Ok(&entry.frame.as_ref().unwrap().1)
        }
    }
}
