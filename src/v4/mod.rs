mod border_symbols;
mod create_border;

pub use border_symbols::*;
pub use create_border::*;

use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::prelude::Widget;
use ratatui::style::Style;
use ratatui::widgets::BorderType;

#[derive(Debug, Clone)]
pub struct BlockBorder {
    pub border_style: Style,
    pub own_border: BorderType,

    pub top: Vec<BorderSymbol>,
    pub bottom: Vec<BorderSymbol>,
    pub left: Vec<BorderSymbol>,
    pub right: Vec<BorderSymbol>,

    pub top_left: BorderSymbol,
    pub top_right: BorderSymbol,
    pub bottom_left: BorderSymbol,
    pub bottom_right: BorderSymbol,
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
    /// Joint along the top side.
    Top,
    /// Joint along the right side.
    Right,
    /// Joint along bottom side.
    Bottom,
    /// Joint along the left side.
    Left,
}

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

    /// Changes the self variant ...
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

impl BlockBorder {
    pub fn with_area(own_border: BorderType, area: Rect) -> Self {
        let mut top_bottom = Vec::new();
        if area.width > 2 {
            for _ in 0..area.width - 2 {
                top_bottom.push(BorderSymbol::SideRegular)
            }
        }
        let mut left_right = Vec::new();
        if area.height > 2 {
            for _ in 0..area.height - 2 {
                left_right.push(BorderSymbol::SideRegular)
            }
        }

        Self {
            border_style: Default::default(),
            own_border,
            top: top_bottom.clone(),
            bottom: top_bottom,
            left: left_right.clone(),
            right: left_right.clone(),
            top_left: BorderSymbol::StartCornerRegular,
            top_right: BorderSymbol::EndCornerRegular,
            bottom_left: BorderSymbol::StartCornerRegular,
            bottom_right: BorderSymbol::EndCornerRegular,
        }
    }

    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    fn symbol(&self, side: Side, symbol: BorderSymbol) -> &'static str {
        match self.own_border {
            BorderType::Plain => plain_symbol(side, symbol),
            BorderType::Rounded => rounded_symbol(side, symbol),
            BorderType::Double => double_symbol(side, symbol),
            BorderType::Thick => thick_symbol(side, symbol),
            BorderType::QuadrantInside => ascii_symbol(side, symbol),
            BorderType::QuadrantOutside => star_symbol(side, symbol),
        }
    }
}

impl Widget for &BlockBorder {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x, //
            area.y,
        )) {
            cell.set_style(self.border_style);
            cell.set_symbol(self.symbol(Side::Top, self.top_left));
        }
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x + area.width.saturating_sub(1), //
            area.y,
        )) {
            cell.set_style(self.border_style);
            cell.set_symbol(self.symbol(Side::Top, self.top_right));
        }
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x,
            area.y + area.height.saturating_sub(1),
        )) {
            cell.set_style(self.border_style);
            cell.set_symbol(self.symbol(Side::Bottom, self.bottom_left));
        }
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x + area.width.saturating_sub(1),
            area.y + area.height.saturating_sub(1),
        )) {
            cell.set_style(self.border_style);
            cell.set_symbol(self.symbol(Side::Bottom, self.bottom_right));
        }

        for (i, symbol) in self.top.iter().enumerate() {
            if let Some(cell) = buf.cell_mut(Position::new(
                area.x + 1 + i as u16, //
                area.y,
            )) {
                cell.set_style(self.border_style);
                cell.set_symbol(self.symbol(Side::Top, *symbol));
            }
        }
        for (i, symbol) in self.bottom.iter().enumerate() {
            if let Some(cell) = buf.cell_mut(Position::new(
                area.x + 1 + i as u16,
                area.y + area.height.saturating_sub(1),
            )) {
                cell.set_style(self.border_style);
                cell.set_symbol(self.symbol(Side::Bottom, *symbol));
            }
        }
        for (i, symbol) in self.left.iter().enumerate() {
            if let Some(cell) = buf.cell_mut(Position::new(
                area.x, //
                area.y + 1 + i as u16,
            )) {
                cell.set_style(self.border_style);
                cell.set_symbol(self.symbol(Side::Left, *symbol));
            }
        }
        for (i, symbol) in self.right.iter().enumerate() {
            if let Some(cell) = buf.cell_mut(Position::new(
                area.x + area.width.saturating_sub(1), //
                area.y + 1 + i as u16,
            )) {
                cell.set_style(self.border_style);
                cell.set_symbol(self.symbol(Side::Right, *symbol));
            }
        }
    }
}
