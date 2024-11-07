//!
//! size_of says
//!
//! ```txt/plain
//! (BorderGlyph, u16, u16) 8
//! AreaPointConnect 8
//! BorderGlyph 4
//! Side 1
//! Position 2
//! BorderType 1
//! ```
//!

mod border_symbols;
mod create_border;

pub use border_symbols::*;
pub use create_border::*;

use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::Style;
use ratatui::widgets::{BorderType, Widget};

/// Experimental AreaBorder.
///
/// This would be
/// ```rust no_run
///
///     use ratatui_block::v3::BlockBorder;
///     use ratatui::widgets::BorderType;
///     use ratatui_block::v3::{BorderSymbol, Kind, Side};
///
///     let width = 20;
///     let height = 10;
///
///     let border = BlockBorder {
///         border_style: Default::default(),
///         own_border: BorderType::Plain,
///         symbols: vec![
///             (
///                 BorderSymbol {
///                     side: Side::Top,
///                     kind: Kind::RegularStart,
///                     other_border: Default::default(),
///                 },
///                 0, 1
///             ),
///             (
///                 BorderSymbol {
///                     side: Side::Top,
///                     kind: Kind::Regular,
///                     other_border: Default::default(),
///                 },
///                 1, width-2,
///             ),
///             (
///                 BorderSymbol {
///                     side: Side::Top,
///                     kind: Kind::RegularEnd,
///                     other_border: Default::default(),
///                 },
///                 width-1, 1,
///             ),
///             (
///                 BorderSymbol {
///                     side: Side::Right,
///                     kind: Kind::Regular,
///                     other_border: Default::default(),
///                 },
///                 1, height-2,
///             ),
///             (
///                 BorderSymbol {
///                     side: Side::Bottom,
///                     kind: Kind::RegularStart,
///                     other_border: Default::default(),
///                 },
///                 height-1, 1,
///             ),
///             (
///                 BorderSymbol {
///                     side: Side::Bottom,
///                     kind: Kind::Regular,
///                     other_border: Default::default(),
///                 },
///                 1, width-2,
///             ),
///             (
///                 BorderSymbol {
///                     side: Side::Bottom,
///                     kind: Kind::RegularEnd,
///                     other_border: Default::default(),
///                 },
///                 width-1, 1,
///             ),
///             (
///                 BorderSymbol {
///                     side: Side::Left,
///                     kind: Kind::Regular,
///                     other_border: Default::default(),
///                 },
///                 1, height-2,
///             ),
///         ],
///     };
///
///
///
/// ```
#[derive(Debug, Clone)]
pub struct BlockBorder {
    /// Border style.
    pub border_style: Style,
    /// The main border of the Block that is rendered.
    pub own_border: BorderType,
    /// Debug.
    pub debug: String,
    /// Area border described as glyphs.
    /// The second value is the offset, the third is the repeat.
    /// Rendering of the glyphs starts at the beginning of
    /// each side, and the glyphs for one side are stacked
    /// from there-on.
    ///
    /// The glyphs need not be ordered by side here.
    pub symbols: Vec<(BorderSymbol, u16, u16)>,
}

///
/// Experimental: Single manual connection point.
#[derive(Debug, Clone)]
pub struct BlockPointConnect {
    /// The main border of the Block that is rendered.
    pub own_border: BorderType,
    /// Glyph
    pub symbol: BorderSymbol,
    /// position from the start of BorderGlyph::side
    pub position: u16,
}

/// Denotes one symbol used to render a block.
#[derive(Debug, Clone, Copy)]
pub struct BorderSymbol {
    /// Which side of the area.
    pub side: Side,
    /// Type of glyph.
    pub kind: Kind,
    /// The second/other Border that will be connected.
    pub other_border: BorderType,
}

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

/// Positions along one side of the block's area.
///
/// Examples below area for Plain/Plain borders.
/// This leads to duplicate glyphs here, but for other
/// BorderTypes such as QuadrantInside those would
/// be different glyphs. ... more later ...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Regular start corner.
    ///
    /// __Example for Top__
    ///
    /// `┌`
    RegularStart,
    /// Draw a 90° outward joint at the start.
    ///
    /// Connects the start corner of this area with the
    /// start corner of the other area.
    ///
    /// __Example for Top __
    ///
    /// `├`
    AngleStartStart,
    /// Draw a 90° outward joint at the start.
    ///
    /// Connects the start corner of this area with the
    /// end corner of the other area.
    ///
    /// __Example for Top__
    ///
    /// `├`
    AngleStartEnd,
    /// Prolong the border along the side before start.
    ///
    /// __Example for Top__
    ///
    /// `┬`
    ProlongStart,
    /// Draw a cross joint at the start of the side.
    ///
    /// The border type here is the border in the direction
    /// of the side line.
    ///
    /// The border at 90° outward from the side is
    /// [BorderGlyph::other_border](BorderSymbol::other_border).
    ///
    /// __Example for Top__
    ///
    /// `┼`
    CrossStart(BorderType),

    /// Regular border at position.
    ///
    /// Position 0 and width-1 are auto-converted to Start/End.
    ///
    /// __Example for Top__
    ///
    /// `─`
    Regular,
    /// Draw a 90° outward joint at some position.
    ///
    /// There is a difference if this joins with the left or the
    /// right side of the other area. This joins the start.
    ///
    /// Position 0 and width-1 are auto-converted to Start/End.
    ///
    /// __Example for Top__
    ///
    /// `┴`
    AngleOutwardStart,
    /// Draw a 90° outward joint at some position.
    ///
    /// There is a difference if this joins with the left or the
    /// right side of the other area. This joins the end.
    ///
    /// Position 0 and width-1 are auto-converted to Start/End.
    ///
    /// __Example for Top__
    ///
    /// `┴`
    AngleOutwardEnd,
    /// Draw a 90° inward joint at some position.
    ///
    /// Position 0 and width-1 are auto-converted to Start/End.
    ///
    /// __Example for Top__
    ///
    /// `┬`
    AngleInwardStart,
    /// Draw a 90° inward joint at some position.
    ///
    /// Position 0 and width-1 are auto-converted to Start/End.
    ///
    /// __Example for Top__
    ///
    /// `┬`
    AngleInwardEnd,
    /// Draw a cross joint at some position.
    ///
    /// The border type here is the border at 90° to the inward.
    /// The border at 90° outward from the side is Joint::other_side.
    ///
    /// __Example for Top__
    ///
    /// `┼`
    Cross(BorderType),
    /// ?? maybe ??
    /// Overlap of the two borders.
    ///
    /// __Example for Top__
    ///
    /// `─`
    Overlap,
    /// Regular end corner.
    ///
    /// __Example for Top__
    ///
    /// `┐`
    RegularEnd,
    /// Draw a 90° outward joint at the end.
    ///
    /// Connects the end corner of this area with the
    /// start corner of the other area.
    ///
    /// __Example for Top__
    ///
    /// `┤`
    AngleEndStart,
    /// Draw a 90° outward joint at the end.
    ///
    /// Connects the end corner of this area with the
    /// end corner of the other area.
    ///
    /// __Example for Top__
    ///
    /// `┤`
    AngleEndEnd,
    /// Prolong the border along the side after the end.
    ///
    /// __Example for Top__
    ///
    /// `┬`
    ProlongEnd,
    /// Draw a cross joint at the end.
    ///
    /// The border type here is the border in the direction
    /// of the side line.
    ///
    /// The border at 90° outward from the side is
    /// [BorderGlyph::other_border](BorderSymbol::other_border).
    /// __Example for Top__
    ///
    /// `┼`
    CrossEnd(BorderType),
}

impl BorderSymbol {
    pub fn new(side: Side, kind: Kind, other_border: BorderType) -> Self {
        Self {
            side,
            kind,
            other_border,
        }
    }
}

impl Widget for &BlockPointConnect {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let sym = match self.own_border {
            BorderType::Plain => plain_symbol(self.symbol),
            BorderType::Rounded => "X",
            BorderType::Double => "X",
            BorderType::Thick => "X",
            BorderType::QuadrantInside => "X",
            BorderType::QuadrantOutside => "X",
        };

        match self.symbol.side {
            Side::Top => {
                if let Some(cell) = buf.cell_mut(Position::new(area.x + self.position, area.y)) {
                    cell.set_symbol(sym);
                }
            }
            Side::Right => {
                if let Some(cell) = buf.cell_mut(Position::new(
                    area.right().saturating_sub(1),
                    area.top() + self.position,
                )) {
                    cell.set_symbol(sym);
                }
            }
            Side::Bottom => {
                if let Some(cell) = buf.cell_mut(Position::new(
                    area.left() + self.position,
                    area.bottom().saturating_sub(1),
                )) {
                    cell.set_symbol(sym);
                }
            }
            Side::Left => {
                if let Some(cell) =
                    buf.cell_mut(Position::new(area.left(), area.top() + self.position))
                {
                    cell.set_symbol(sym);
                }
            }
        }
    }
}

impl BlockBorder {
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }
}

impl Widget for &BlockBorder {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        for (border_sym, position, repeat) in self.symbols.iter().copied() {
            let sym = match self.own_border {
                BorderType::Plain => plain_symbol(border_sym),
                BorderType::Rounded => "X",
                BorderType::Double => "X",
                BorderType::Thick => "X",
                BorderType::QuadrantInside => "X",
                BorderType::QuadrantOutside => "X",
            };

            match border_sym.side {
                Side::Top => {
                    for x in area.x + position..area.x + position + repeat {
                        if let Some(cell) = buf.cell_mut(Position::new(x, area.y)) {
                            cell.set_style(self.border_style);
                            cell.set_symbol(sym);
                        }
                    }
                }
                Side::Right => {
                    for y in area.y + position..area.y + position + repeat {
                        if let Some(cell) =
                            buf.cell_mut(Position::new((area.x + area.width).saturating_sub(1), y))
                        {
                            cell.set_style(self.border_style);
                            cell.set_symbol(sym);
                        }
                    }
                }
                Side::Bottom => {
                    for x in area.x + position..area.x + position + repeat {
                        if let Some(cell) =
                            buf.cell_mut(Position::new(x, (area.y + area.height).saturating_sub(1)))
                        {
                            cell.set_style(self.border_style);
                            cell.set_symbol(sym);
                        }
                    }
                }
                Side::Left => {
                    for y in area.y + position..area.y + position + repeat {
                        if let Some(cell) = buf.cell_mut(Position::new(area.x, y)) {
                            cell.set_style(self.border_style);
                            cell.set_symbol(sym);
                        }
                    }
                }
            }
        }
    }
}
