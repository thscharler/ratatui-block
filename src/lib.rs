//! Block borders.

pub mod block_border;
pub mod block_connect;
pub mod block_grid;
pub mod border_symbols;

use ratatui::widgets::BorderType;
use std::fmt::Debug;

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
    ///
    /// Get the actual symbol occurring along one side of the area.
    ///
    /// side: Which side of the area.
    /// symbol: Symbol definition.
    ///
    fn symbol(&self, side: Side, symbol: BorderSymbol) -> &'static str;

    ///
    /// Get the symbol for a 4-way crossing of lines.
    ///
    /// The number of parameters comes from handling Quadrant crossings.
    /// ![schematics](https://raw.githubusercontent.com/thscharler/ratatui-block/refs/heads/master/diagram/quadrant_inside.png)
    ///
    /// __Remark__
    /// Not needed for drawing Rects or grids, but a bit of a missing link otherwise.
    ///
    fn crossing(
        &self,
        top_side: Side,
        top: BorderType,
        right_side: Side,
        right: BorderType,
        bottom_side: Side,
        bottom: BorderType,
        left_side: Side,
        left: BorderType,
    ) -> &'static str;
}

/// Symbol descriptor.
///
/// Describes the symbols geometrically as they occur along one
/// side of the area.
///
/// Schematics for the connection.
///
/// ![schematics](https://raw.githubusercontent.com/thscharler/ratatui-block/refs/heads/master/diagram/border_symbol_1.png)
///
/// Which side of the other area is connected may also influence
/// the actual glyph.
///
/// ![connection sides](https://raw.githubusercontent.com/thscharler/ratatui-block/refs/heads/master/diagram/border_symbol_2.png)
///
/// Construction for PlainBorderSet. Note that some of the glyphs do not
/// exist in unicode and are replaced with single line versions.
///
/// ![plain_border_set](https://raw.githubusercontent.com/thscharler/ratatui-block/refs/heads/master/diagram/plain_symbol_set.png)
///
/// Construction for QuadrantInsideBorderSet
///
/// ![quadrant_inside_border_set](https://raw.githubusercontent.com/thscharler/ratatui-block/refs/heads/master/diagram/q_inside_start.png)
/// ![quadrant_inside_border_set](https://raw.githubusercontent.com/thscharler/ratatui-block/refs/heads/master/diagram/q_inside_end.png)
/// ![quadrant_inside_border_set](https://raw.githubusercontent.com/thscharler/ratatui-block/refs/heads/master/diagram/q_inside_outside_start.png)
/// ![quadrant_inside_border_set](https://raw.githubusercontent.com/thscharler/ratatui-block/refs/heads/master/diagram/q_inside_outside_end.png)
///
///
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderSymbol {
    /// Regular start corner.
    StartCornerRegular,
    /// Start corner with an extension perpendicular to the line.
    StartCornerAngled(Side, BorderType),
    /// Start corner that goes on in the direction of the line.
    StartCornerProlonged(Side, BorderType),
    /// Crossing at the start.
    /// The first value is the border perpendicular to the line,
    /// the second goes in the direction of the line.
    StartCornerCrossed(Side, BorderType, Side, BorderType),

    /// Regular side.
    SideRegular,
    /// Regular side, but overlapping with some other border.
    SideOverlap(Side, BorderType),
    /// Side with a connection point outwards.
    SideOutward(Side, BorderType),
    /// Side with a connection point inwards.
    SideInward(Side, BorderType),
    /// Side with a crossing.
    /// The first value is the border outwards, the second
    /// goes inwards.
    SideCrossed(Side, BorderType, Side, BorderType),

    /// Regular end corner.
    EndCornerRegular,
    /// End corner with an extension perpendicular to the line.
    EndCornerAngled(Side, BorderType),
    /// End corner that goes on in the direction of the line.
    EndCornerProlonged(Side, BorderType),
    /// Crossing at the end.
    /// The first value is the border perpendicular to the line,
    /// the second goes in the direction of the line.
    EndCornerCrossed(Side, BorderType, Side, BorderType),
    //
    // /// 4-way crossing of up to 4 different border types
    // /// along the side.
    // ///
    // /// This is only defined for Side::Cross
    // ///
    // /// Borders are (angled_outward, forward, angled_inward, backward).
    // Cross(BorderType, BorderType, BorderType, BorderType),
}

impl Side {
    /// Give the opposite side.
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
