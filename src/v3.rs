//!
//! size_of says
//!
//! ```txt/plain
//! (BorderGlyph, u16) 8
//! AreaPointConnect 8
//! BorderGlyph 5
//! Side 1
//! Position 2
//! BorderType 1
//! ```
//!

use ratatui::widgets::BorderType;

/// Experimental AreaBorder.
///
/// This would be
/// ```rust no_run
///
///     use ratatui_block::v3::BlockBorder;
///     use ratatui::widgets::BorderType;
///     use ratatui_block::v3::{BorderGlyph, Kind, Side};
///
///     let width = 20;
///     let height = 10;
///
///     let border = BlockBorder {
///         glyphs: vec![
///             (
///                 BorderGlyph {
///                     side: Side::Top,
///                     kind: Kind::RegularStart,
///                     own_border: BorderType::Plain,
///                     other_border: Default::default(),
///                 },
///                 1,
///             ),
///             (
///                 BorderGlyph {
///                     side: Side::Top,
///                     kind: Kind::Regular,
///                     own_border: BorderType::Plain,
///                     other_border: Default::default(),
///                 },
///                 width-2,
///             ),
///             (
///                 BorderGlyph {
///                     side: Side::Top,
///                     kind: Kind::RegularEnd,
///                     own_border: BorderType::Plain,
///                     other_border: Default::default(),
///                 },
///                 1,
///             ),
///             (
///                 BorderGlyph {
///                     side: Side::Right,
///                     kind: Kind::Regular,
///                     own_border: BorderType::Plain,
///                     other_border: Default::default(),
///                 },
///                 height-2,
///             ),
///             (
///                 BorderGlyph {
///                     side: Side::Bottom,
///                     kind: Kind::RegularStart,
///                     own_border: BorderType::Plain,
///                     other_border: Default::default(),
///                 },
///                 1,
///             ),
///             (
///                 BorderGlyph {
///                     side: Side::Bottom,
///                     kind: Kind::Regular,
///                     own_border: BorderType::Plain,
///                     other_border: Default::default(),
///                 },
///                 width-2,
///             ),
///             (
///                 BorderGlyph {
///                     side: Side::Bottom,
///                     kind: Kind::RegularEnd,
///                     own_border: BorderType::Plain,
///                     other_border: Default::default(),
///                 },
///                 1,
///             ),
///             (
///                 BorderGlyph {
///                     side: Side::Left,
///                     kind: Kind::Regular,
///                     own_border: BorderType::Plain,
///                     other_border: Default::default(),
///                 },
///                 height-2,
///             ),
///         ],
///     };
///
///
///
/// ```
///
/// Todo: impl Widget
///
#[derive(Debug, Clone)]
pub struct BlockBorder {
    /// Area border described as glyphs.
    /// The second value is a repeat.
    /// Rendering of the glyphs starts at the beginning of
    /// each side, and the glyphs for one side are stacked
    /// from there-on.
    ///
    /// The glyphs need not be ordered by side here.
    pub glyphs: Vec<(BorderGlyph, u16)>,
}

///
/// Experimental: Single manual connection point.
///
/// Todo: impl Widget
///
#[derive(Debug, Clone)]
pub struct BlockPointConnect {
    /// Glyph
    pub glyph: BorderGlyph,
    /// position from the start of BorderGlyph::side
    pub position: u16,
}

/// Denotes one glyph used to render a block.
#[derive(Debug, Clone, Copy)]
pub struct BorderGlyph {
    /// Which side of the area.
    pub side: Side,
    /// Type of glyph.
    pub kind: Kind,
    /// The main border of the Block that is rendered.
    pub own_border: BorderType,
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
    /// [BorderGlyph::other_border](BorderGlyph::other_border).
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
    Overlap(BorderType),
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
    /// [BorderGlyph::other_border](BorderGlyph::other_border).
    /// __Example for Top__
    ///
    /// `┼`
    CrossEnd(BorderType),
}
