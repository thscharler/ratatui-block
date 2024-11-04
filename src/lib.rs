#![allow(dead_code)]

use ratatui::widgets::BorderType;

/// A joint in a Block border.
#[derive(Debug, Clone, Copy)]
pub struct Joint {
    /// The border type to join with.
    /// This is the border for areas outward of the side.
    pub border: BorderType,
    /// Side of the area.
    pub side: JointSide,
    /// Joint mark.
    pub mark: JointMark,
    /// Mirrored joint. This is needed for QuadrantInside and QuadrantOutside.
    /// Those have mirrored glyphs on each side.
    pub mirrored: bool,
    /// Position for the join.
    pub pos: JointPos,
}

/// Marktype for the joints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointMark {
    /// Inward join.
    In,
    /// Outward join.
    Out,
    /// Through join.
    Through,
    /// Manual join.
    Manual(&'static str),
}

/// Position of the joints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointPos {
    /// Draw a cross joint at the start.
    /// The border type is for an area onwards the direction of the side.
    StartCross(BorderType),
    /// Prolong the border along the main axis before start.
    ProlongStart,
    /// Draw a perpendicular joint at the start.
    Start,
    /// Draw a joint at position from start.
    /// Position 0 and width-1 are translated to Start/End.
    Pos(u16),
    /// Draw a perpendicular joint at the end.
    End,
    /// Prolong the border along the main axis after the end.
    ProlongEnd,
    /// Draw a cross joint at the end.
    /// The border type is for an area onwards the direction of the side.
    EndCross(BorderType),
}

/// Sides for the joints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointSide {
    /// Join from the top side upwards.
    ///
    /// Relative position to the left corner.
    /// 0 and width-1 are detected as corners.
    Top,
    /// Join from the right side right.
    ///
    /// Relative position to the top corner.
    /// 0 and height-1 are detected as corners.
    Right,
    /// Join from the bottom side downwards.
    ///
    /// Relative position to the left corner.
    /// 0 and width-1 are detected as corners.
    Bottom,
    /// Join from the left side left.
    ///
    /// Relative position to the top corner.
    /// 0 and height-1 are detected as corners.
    Left,
}

// mod block_joint;
// mod create_border;
mod new_block;
mod new_border;
mod new_joint;

// pub use block_joint::render_joint;
// pub use create_border::create_border;
pub use new_block::NewBlock;
pub use new_border::create_border;
pub use new_joint::render_joint;
