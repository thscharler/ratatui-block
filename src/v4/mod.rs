//! Block borders.
//!
//! ![schematic](../../../../illustration.png)
//!

pub mod border_symbols;
mod create_border;

use crate::v4::create_border::{base_border, connected_border, symbol_set};
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::prelude::Widget;
use ratatui::style::Style;
use ratatui::widgets::BorderType;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub struct BlockBorder {
    border_style: Style,
    symbol_set: Rc<dyn BorderSymbolSet>,

    area: Rect,
    symbols: Vec<BorderSymbol>,
}

// #[derive(Debug, Clone)]
// pub struct BlockGrid {
//     pub border_style: Style,
//     pub border: BorderType,
//
//     pub top: Vec<BorderSymbol>,
//     pub bottom: Vec<BorderSymbol>,
//     pub left: Vec<BorderSymbol>,
//     pub right: Vec<BorderSymbol>,
//     pub inner_vertical: Vec<BorderSymbol>,
//     pub inner_horizontal: Vec<BorderSymbol>,
// }
//
// #[derive(Debug, Clone)]
// pub struct BlockConnect {
//     pub border_style: Style,
//     pub border: BorderType,
//     pub connect: BorderSymbol,
// }

/// Names for the sides of an area.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// Border along the top side.
    Top,
    /// Border along bottom side.
    Bottom,
    /// Border along the right side.
    Right,
    /// Border along the left side.
    Left,
}

/// Symbol set trait
pub trait BorderSymbolSet {
    fn symbol(&self, side: Side, symbol: BorderSymbol) -> &'static str;
}

/// Symbol descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderSymbol {
    StartCornerRegular,
    StartCornerAngled(Side, BorderType),
    StartCornerProlonged(Side, BorderType),
    StartCornerCrossed(Side, BorderType, Side, BorderType),

    SideRegular,
    SideOverlap(Side, BorderType),
    SideOutward(Side, BorderType),
    SideInward(Side, BorderType),
    SideCrossed(Side, BorderType, Side, BorderType),

    EndCornerRegular,
    EndCornerAngled(Side, BorderType),
    EndCornerProlonged(Side, BorderType),
    EndCornerCrossed(Side, BorderType, Side, BorderType),
}

impl Side {
    pub fn opposite(&self) -> Self {
        match self {
            Side::Top => Side::Bottom,
            Side::Right => Side::Left,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
        }
    }
}

impl BorderSymbol {
    /// Adds an outward connection to the given border.
    ///
    /// Replaces any existing outward connection.
    /// Changes the BorderSymbol to add an outward connection if possible.
    pub fn join_outward(&mut self, side: Side, border: BorderType) {
        use BorderSymbol::*;

        *self = match *self {
            StartCornerRegular => StartCornerAngled(side, border),
            StartCornerAngled(_, _) => StartCornerAngled(side, border),
            StartCornerProlonged(prolong_side, prolong_border) => {
                StartCornerCrossed(side, border, prolong_side, prolong_border)
            }
            StartCornerCrossed(_, _, prolong_side, prolong_border) => {
                StartCornerCrossed(side, border, prolong_side, prolong_border)
            }
            SideRegular => SideOutward(side, border),
            SideOverlap(_, _) => SideOutward(side, border),
            SideOutward(_, _) => SideOutward(side, border),
            SideInward(inward_side, inward_border) => {
                SideCrossed(side, border, inward_side, inward_border)
            }
            SideCrossed(_, _, inward_side, inward_border) => {
                SideCrossed(side, border, inward_side, inward_border)
            }
            EndCornerRegular => EndCornerAngled(side, border),
            EndCornerAngled(_, _) => EndCornerAngled(side, border),
            EndCornerProlonged(prolong_side, prolong_border) => {
                EndCornerCrossed(side, border, prolong_side, prolong_border)
            }
            EndCornerCrossed(_, _, prolong_side, prolong_border) => {
                EndCornerCrossed(side, border, prolong_side, prolong_border)
            }
        }
    }

    /// Adds an inward connection to the given border.
    ///
    /// Replaces any existing inward connection.
    /// Changes the BorderSymbol to add an inward connection if possible.
    pub fn join_inward(&mut self, side: Side, border: BorderType) {
        use BorderSymbol::*;

        *self = match *self {
            StartCornerRegular => *self,
            StartCornerAngled(_, _) => *self,
            StartCornerProlonged(_, _) => *self,
            StartCornerCrossed(_, _, _, _) => *self,
            SideRegular => SideInward(side, border),
            SideOverlap(_, _) => SideInward(side, border),
            SideOutward(outward_side, outward_border) => {
                SideCrossed(outward_side, outward_border, side, border)
            }
            SideInward(_, _) => SideInward(side, border),
            SideCrossed(outward_side, outward_border, _, _) => {
                SideCrossed(outward_side, outward_border, side, border)
            }
            EndCornerRegular => *self,
            EndCornerAngled(_, _) => *self,
            EndCornerProlonged(_, _) => *self,
            EndCornerCrossed(_, _, _, _) => *self,
        }
    }

    /// Changes the BorderSymbol to add an overlapping area if possible.
    ///
    /// Does nothing if this is not possible for the current symbol.
    pub fn overlap(&mut self, side: Side, border: BorderType) {
        use BorderSymbol::*;

        *self = match *self {
            StartCornerRegular => *self,
            StartCornerAngled(_, _) => *self,
            StartCornerProlonged(_, _) => *self,
            StartCornerCrossed(_, _, _, _) => *self,
            SideRegular => SideOverlap(side, border),
            SideOverlap(_, _) => SideOverlap(side, border),
            SideOutward(_, _) => *self,
            SideInward(_, _) => *self,
            SideCrossed(_, _, _, _) => *self,
            EndCornerRegular => *self,
            EndCornerAngled(_, _) => *self,
            EndCornerProlonged(_, _) => *self,
            EndCornerCrossed(_, _, _, _) => *self,
        }
    }

    /// Changes the BorderSymbol to prolong the border along the side.
    ///
    /// Does nothing if this is not possible for the current symbol.
    pub fn prolong(&mut self, side: Side, border: BorderType) {
        use BorderSymbol::*;

        *self = match *self {
            StartCornerRegular => StartCornerProlonged(side, border),
            StartCornerAngled(angle_side, angle_border) => {
                StartCornerCrossed(angle_side, angle_border, side, border)
            }
            StartCornerProlonged(_, _) => StartCornerProlonged(side, border),
            StartCornerCrossed(angle_side, angle_border, _, _) => {
                StartCornerCrossed(angle_side, angle_border, side, border)
            }
            SideRegular => *self,
            SideOverlap(_, _) => *self,
            SideOutward(_, _) => *self,
            SideInward(_, _) => *self,
            SideCrossed(_, _, _, _) => *self,
            EndCornerRegular => EndCornerProlonged(side, border),
            EndCornerAngled(angle_side, angle_border) => {
                EndCornerCrossed(angle_side, angle_border, side, border)
            }
            EndCornerProlonged(_, _) => EndCornerProlonged(side, border),
            EndCornerCrossed(angle_side, angle_border, _, _) => {
                EndCornerCrossed(angle_side, angle_border, side, border)
            }
        }
    }
}

impl Debug for BlockBorder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockBorder")
            .field("border_style", &self.border_style)
            .field("symbol_set", &"..dyn..")
            .field("symbols", &self.symbols)
            .finish()
    }
}

impl BlockBorder {
    ///
    /// New block border for the given area.
    ///
    pub fn new(area: Rect) -> Self {
        base_border(area)
    }

    ///
    /// New block border for a Block that is part of some bigger layout.
    ///
    /// Given all the areas of the layout and each border type,
    /// this creates a border that is connected at the edges.
    ///
    /// TODO: if the borders overlap more than just exactly for the border the result is undefined.
    ///
    pub fn from_layout(areas: &[Rect], borders: &[BorderType], select: usize) -> Self {
        connected_border(areas, borders, select)
    }

    ///
    /// Border style for the border.
    ///
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    ///
    /// Sets the border type used.
    ///
    pub fn border_type(mut self, border: BorderType) -> Self {
        self.symbol_set = symbol_set(border);
        self
    }

    ///
    /// Use a BorderSymbolSet.
    ///
    pub fn border_set(mut self, border_set: Rc<dyn BorderSymbolSet>) -> Self {
        self.symbol_set = border_set;
        self
    }

    ///
    /// Area covered by the BlockBorder.
    ///
    pub fn get_area(&self) -> Rect {
        self.area
    }

    /// BorderSymbol for the top-left corner.
    pub fn top_left(&self) -> &BorderSymbol {
        self.split().0
    }

    /// BorderSymbol's for the top border.
    /// Slice with area.width-2 items.
    pub fn top(&self) -> &[BorderSymbol] {
        self.split().1
    }

    /// BorderSymbol for the top-right corner.
    pub fn top_right(&self) -> &BorderSymbol {
        self.split().2
    }

    /// BorderSymbol's for the right border.
    /// Slice with area.height-2 items.
    pub fn right(&self) -> &[BorderSymbol] {
        self.split().3
    }

    /// BorderSymbol for the bottom-right corner.
    pub fn bottom_right(&self) -> &BorderSymbol {
        self.split().4
    }

    /// BorderSymbol's for the bottom border.
    /// Slice with area.width-2 items.
    pub fn bottom(&self) -> &[BorderSymbol] {
        self.split().5
    }

    /// BorderSymbol for the bottom-left corner.
    pub fn bottom_left(&self) -> &BorderSymbol {
        self.split().6
    }

    /// BorderSymbols for the left border.
    /// Slice with area.height-2 items.
    pub fn left(&self) -> &[BorderSymbol] {
        self.split().7
    }

    /// BorderSymbol for the top-left corner.
    pub fn top_left_mut(&mut self) -> &mut BorderSymbol {
        self.split_mut().0
    }

    /// BorderSymbol's for the top border.
    /// Slice with area.width-2 items.
    pub fn top_mut(&mut self) -> &mut [BorderSymbol] {
        self.split_mut().1
    }

    /// BorderSymbol for the top-right corner.
    pub fn top_right_mut(&mut self) -> &mut BorderSymbol {
        self.split_mut().2
    }

    /// BorderSymbol's for the right border.
    /// Slice with area.height-2 items.
    pub fn right_mut(&mut self) -> &mut [BorderSymbol] {
        self.split_mut().3
    }

    /// BorderSymbol for the bottom-right corner.
    pub fn bottom_right_mut(&mut self) -> &mut BorderSymbol {
        self.split_mut().4
    }

    /// BorderSymbol's for the bottom border.
    /// Slice with area.width-2 items.
    pub fn bottom_mut(&mut self) -> &mut [BorderSymbol] {
        self.split_mut().5
    }

    /// BorderSymbol for the bottom-left corner.
    pub fn bottom_left_mut(&mut self) -> &mut BorderSymbol {
        self.split_mut().6
    }

    /// BorderSymbols for the left border.
    /// Slice with area.height-2 items.
    pub fn left_mut(&mut self) -> &mut [BorderSymbol] {
        self.split_mut().7
    }

    /// All BorderSymbols.
    pub fn get_symbols(&self) -> &[BorderSymbol] {
        self.symbols.as_slice()
    }

    ///
    /// Split into border parts.
    ///
    /// (top_left, top, top_right, right, bottom_left, bottom, bottom_right, left)
    ///
    #[inline(always)]
    pub fn split_mut(
        &mut self,
    ) -> (
        &mut BorderSymbol,
        &mut [BorderSymbol],
        &mut BorderSymbol,
        &mut [BorderSymbol],
        &mut BorderSymbol,
        &mut [BorderSymbol],
        &mut BorderSymbol,
        &mut [BorderSymbol],
    ) {
        let (top_left, rest) = self.symbols.split_at_mut(1);
        let (top, rest) = rest.split_at_mut(self.area.width.saturating_sub(2) as usize);
        let (top_right, rest) = rest.split_at_mut(1);
        let (right, rest) = rest.split_at_mut(self.area.height.saturating_sub(2) as usize);
        let (bottom_left, rest) = rest.split_at_mut(1);
        let (bottom, rest) = rest.split_at_mut(self.area.width.saturating_sub(2) as usize);
        let (bottom_right, rest) = rest.split_at_mut(1);
        let (left, rest) = rest.split_at_mut(self.area.height.saturating_sub(2) as usize);

        assert!(rest.is_empty());

        (
            &mut top_left[0],
            top,
            &mut top_right[0],
            right,
            &mut bottom_left[0],
            bottom,
            &mut bottom_right[0],
            left,
        )
    }

    ///
    /// Split into border parts.
    ///
    /// (top_left, top, top_right, right, bottom_left, bottom, bottom_right, left)
    ///
    #[inline(always)]
    pub fn split(
        &self,
    ) -> (
        &BorderSymbol,
        &[BorderSymbol],
        &BorderSymbol,
        &[BorderSymbol],
        &BorderSymbol,
        &[BorderSymbol],
        &BorderSymbol,
        &[BorderSymbol],
    ) {
        let (top_left, rest) = self.symbols.split_at(1);
        let (top, rest) = rest.split_at(self.area.width.saturating_sub(2) as usize);
        let (top_right, rest) = rest.split_at(1);
        let (right, rest) = rest.split_at(self.area.height.saturating_sub(2) as usize);
        let (bottom_left, rest) = rest.split_at(1);
        let (bottom, rest) = rest.split_at(self.area.width.saturating_sub(2) as usize);
        let (bottom_right, rest) = rest.split_at(1);
        let (left, rest) = rest.split_at(self.area.height.saturating_sub(2) as usize);

        assert!(rest.is_empty());

        (
            &top_left[0],
            top,
            &top_right[0],
            right,
            &bottom_left[0],
            bottom,
            &bottom_right[0],
            left,
        )
    }
}

impl Widget for &BlockBorder {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let (
            top_left, //
            top,
            top_right,
            right,
            bottom_left,
            bottom,
            bottom_right,
            left,
        ) = self.split();

        if let Some(cell) = buf.cell_mut(Position::new(
            area.x, //
            area.y,
        )) {
            cell.set_style(self.border_style);
            cell.set_symbol(self.symbol_set.symbol(Side::Top, *top_left));
        }
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x + area.width.saturating_sub(1), //
            area.y,
        )) {
            cell.set_style(self.border_style);
            cell.set_symbol(self.symbol_set.symbol(Side::Top, *top_right));
        }
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x,
            area.y + area.height.saturating_sub(1),
        )) {
            cell.set_style(self.border_style);
            cell.set_symbol(self.symbol_set.symbol(Side::Bottom, *bottom_left));
        }
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x + area.width.saturating_sub(1),
            area.y + area.height.saturating_sub(1),
        )) {
            cell.set_style(self.border_style);
            cell.set_symbol(self.symbol_set.symbol(Side::Bottom, *bottom_right));
        }

        for (i, symbol) in top.iter().enumerate() {
            if let Some(cell) = buf.cell_mut(Position::new(
                area.x + 1 + i as u16, //
                area.y,
            )) {
                cell.set_style(self.border_style);
                cell.set_symbol(self.symbol_set.symbol(Side::Top, *symbol));
            }
        }
        for (i, symbol) in bottom.iter().enumerate() {
            if let Some(cell) = buf.cell_mut(Position::new(
                area.x + 1 + i as u16,
                area.y + area.height.saturating_sub(1),
            )) {
                cell.set_style(self.border_style);
                cell.set_symbol(self.symbol_set.symbol(Side::Bottom, *symbol));
            }
        }
        for (i, symbol) in left.iter().enumerate() {
            if let Some(cell) = buf.cell_mut(Position::new(
                area.x, //
                area.y + 1 + i as u16,
            )) {
                cell.set_style(self.border_style);
                cell.set_symbol(self.symbol_set.symbol(Side::Left, *symbol));
            }
        }
        for (i, symbol) in right.iter().enumerate() {
            if let Some(cell) = buf.cell_mut(Position::new(
                area.x + area.width.saturating_sub(1), //
                area.y + 1 + i as u16,
            )) {
                cell.set_style(self.border_style);
                cell.set_symbol(self.symbol_set.symbol(Side::Right, *symbol));
            }
        }
    }
}
