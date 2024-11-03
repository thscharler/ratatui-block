#![allow(dead_code)]

use ratatui::widgets::BorderType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointSide {
    /// Join from the top side upwards.
    ///
    /// Relative position to the left corner.
    /// 0 and width-1 are detected as corners.
    Top(u16),
    /// Join from the right side right.
    ///
    /// Relative position to the top corner.
    /// 0 and height-1 are detected as corners.
    Right(u16),
    /// Join from the bottom side downwards.
    ///
    /// Relative position to the left corner.
    /// 0 and width-1 are detected as corners.
    Bottom(u16),
    /// Join from the left side left.
    ///
    /// Relative position to the top corner.
    /// 0 and height-1 are detected as corners.
    Left(u16),

    /// Join top-left corner with another bottom-right corner.
    TopLeft,
    /// Join top-right corner with another bottom-left corner.
    TopRight,
    /// Join bottom-right corner with another top-left corner.
    BottomRight,
    /// Join bottom-left corner with another top-right corner.
    BottomLeft,
}

impl JointSide {
    /// Set the position for `Top`/`Right`/`Bottom`/`Left` variants.
    pub fn pos(self, pos: u16) -> Self {
        match self {
            JointSide::Top(_) => JointSide::Top(pos),
            JointSide::Right(_) => JointSide::Right(pos),
            JointSide::Bottom(_) => JointSide::Bottom(pos),
            JointSide::Left(_) => JointSide::Left(pos),
            v => v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Joint {
    /// Join on the outside.
    Out(BorderType),
    /// Join on the inside.
    In(BorderType),
    /// Join through the side.
    Through(BorderType),
    /// Corner/corner joints.
    /// Might involve 2 more areas so two foreign border-types.
    Corner(BorderType, BorderType),
    /// Manual joint.
    Manual(&'static str),
}

impl Joint {
    /// Change the connected border for `Out`/`In`/`Through`.
    pub fn border(self, border: BorderType) -> Self {
        match self {
            Joint::Out(_) => Joint::Out(border),
            Joint::In(_) => Joint::In(border),
            Joint::Through(_) => Joint::Through(border),
            Joint::Corner(v, h) => Joint::Corner(v, h),
            Joint::Manual(c) => Joint::Manual(c),
        }
    }

    /// Change the connected borders for `Corner`.
    pub fn corner(self, vertical: BorderType, horizontal: BorderType) -> Self {
        match self {
            Joint::Out(b) => Joint::Out(b),
            Joint::In(b) => Joint::In(b),
            Joint::Through(b) => Joint::Through(b),
            Joint::Corner(_, _) => Joint::Corner(vertical, horizontal),
            Joint::Manual(c) => Joint::Manual(c),
        }
    }
}

mod block_joint;
mod create_border;
mod new_block;

pub use block_joint::render_joint;
pub use create_border::create_border;
pub use new_block::NewBlock;
