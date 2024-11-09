//! Block borders.

pub mod block_border;
pub mod border_symbols;

use ratatui::prelude::Widget;
use ratatui::widgets::BorderType;
use std::fmt::Debug;

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
///
/// ![symbol organization](https://raw.githubusercontent.com/thscharler/ratatui-block/refs/heads/master/illustration.svg)
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
