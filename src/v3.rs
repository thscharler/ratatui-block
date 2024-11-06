use ratatui::widgets::BorderType;

/// Experimental AreaBorder.
#[derive(Debug, Clone)]
pub struct AreaBorder {
    /// Area border described as glyphs.
    /// The second value is a repeat.
    pub glyphs: Vec<(BorderGlyph, u16)>,
}

/// Denotes one glyph used to render a block.
#[derive(Debug, Clone, Copy)]
pub struct BorderGlyph {
    /// Which side of the area.
    pub side: Side,
    /// Position of the glyph.
    pub pos: Position,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    /// Regular start corner.
    RegularStart,
    /// Draw a 90° outward joint at the start.
    ///
    /// Connects the start corner of this area with the
    /// start corner of the other area.
    AngleStartStart,
    /// Draw a 90° outward joint at the start.
    ///
    /// Connects the start corner of this area with the
    /// end corner of the other area.
    AngleStartEnd,
    /// Prolong the border along the side before start.
    ProlongStart,
    /// Draw a cross joint at the start of the side.
    ///
    /// The border type here is the border in the direction
    /// of the side line.
    ///
    /// The border at 90° outward from the side is
    /// [BorderGlyph::other_border](BorderGlyph::other_border).
    CrossStart(BorderType),

    /// Regular border at position.
    ///
    /// Position 0 and width-1 are auto-converted to Start/End.
    Regular(u16),
    /// Draw a 90° outward joint at some position.
    ///
    /// There is a difference if this joins with the left or the
    /// right side of the other area. This joins the start.
    ///
    /// Position 0 and width-1 are auto-converted to Start/End.
    AngleOutwardStart(u16),
    /// Draw a 90° outward joint at some position.
    ///
    /// There is a difference if this joins with the left or the
    /// right side of the other area. This joins the end.
    ///
    /// Position 0 and width-1 are auto-converted to Start/End.
    AngleOutwardEnd(u16),
    /// Draw a 90° inward joint at some position.
    ///
    /// Position 0 and width-1 are auto-converted to Start/End.
    AngleInwardStart(u16),
    /// Draw a 90° inward joint at some position.
    ///
    /// Position 0 and width-1 are auto-converted to Start/End.
    AngleInwardEnd(u16),
    /// Draw a cross joint at some position.
    ///
    /// The border type here is the border at 90° to the inward.
    /// The border at 90° outward from the side is Joint::other_side.
    Cross(BorderType),
    /// ?? maybe ??
    /// Overlap of the two borders.
    Overlap(BorderType),

    /// Regular end corner.
    RegularEnd,
    /// Draw a 90° outward joint at the end.
    ///
    /// Connects the end corner of this area with the
    /// start corner of the other area.
    AngleEndStart,
    /// Draw a 90° outward joint at the end.
    ///
    /// Connects the end corner of this area with the
    /// end corner of the other area.
    AngleEndEnd,
    /// Prolong the border along the side after the end.
    ProlongEnd,
    /// Draw a cross joint at the end.
    ///
    /// The border type here is the border in the direction
    /// of the side line.
    ///
    /// The border at 90° outward from the side is
    /// [BorderGlyph::other_border](BorderGlyph::other_border).
    CrossEnd(BorderType),
}
