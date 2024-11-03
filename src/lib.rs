#![allow(dead_code)]

use ratatui::widgets::BorderType;

/// A joint in a Block border.
#[derive(Debug, Clone, Copy)]
pub struct Joint {
    // regular join border
    pub border: BorderType,
    // what side of the area.
    pub side: JointSide,
    // what type of joint
    pub scale: JointScale,
    // mirrored joint
    pub mirrored: bool,
    // position for the join
    pub pos: JointPos,
}

/// A cross joint at the corner of a Block border.
#[derive(Debug, Clone)]
pub struct CrossJoint {
    // regular join border
    pub join_border: BorderType,
    // second join border for cross joints in the corners.
    pub join_border2: BorderType,
    // what corner
    pub corner: JointCorner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointScale {
    /// Inward join.
    In,
    /// Outward join.
    Out,
    /// Through join.
    Through,
    /// Manual join.
    Manual(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointPos {
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
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointCorner {
    /// Join top-left corner with another bottom-right corner.
    TopLeftCross,
    /// Join top-right corner with another bottom-left corner.
    TopRightCross,
    /// Join bottom-right corner with another top-left corner.
    BottomRightCross,
    /// Join bottom-left corner with another top-right corner.
    BottomLeftCross,
}

// mod block_joint;
// mod create_border;
mod new_block;
mod new_joint;

// pub use block_joint::render_joint;
// pub use create_border::create_border;
pub use new_block::NewBlock;
pub use new_joint::render_joint;
