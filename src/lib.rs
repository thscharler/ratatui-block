#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{BorderType, Widget};

/// A joint connection as part of a Block.
#[derive(Debug, Clone, Copy)]
pub struct Joint {
    /// The border of the Block that should be amended.
    pub(crate) own_border: BorderType,
    /// The second/other Border that will be connected.
    pub(crate) other_border: BorderType,
    /// Which side of the area.
    pub(crate) side: JointSide,
    /// Joint mark.
    pub(crate) mark: JointKind,
    /// Mirrored joint. This is needed for QuadrantInside and QuadrantOutside.
    /// Those have mirrored glyphs on each side.
    pub(crate) mirrored: bool,
    /// Position for the join.
    pub(crate) pos: JointPosition,
}

/// What kind of connection is needed; includes manual ones.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum JointKind {
    /// Outward join.
    #[default]
    Outward,
    /// Inward join.
    Inward,
    /// Through join.
    Through,
    /// Manual join.
    Manual(&'static str),
}

/// Position of the joints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointPosition {
    /// Draw a cross joint at the start of the side.
    ///
    /// The border type here is the border in the direction
    /// of the side line.
    ///
    /// The border perpendicular to the side is Joint::other_side.
    CrossStart(BorderType),
    /// Prolong the border along the side before start.
    ProlongStart,
    /// Draw a perpendicular joint at the start.
    Start,
    /// Draw a perpendicular joint at some position from start.
    /// Position 0 and width-1 are auto-converted to Start/End.
    Pos(u16),
    /// Draw a perpendicular joint at the end.
    End,
    /// Prolong the border along the side after the end.
    ProlongEnd,
    /// Draw a cross joint at the end.
    ///
    /// The border type here is the border in the direction
    /// of the side line.
    ///
    /// The border perpendicular to the side is Joint::other_side.
    CrossEnd(BorderType),
}

/// Names for the sides of an area.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointSide {
    /// Joint along the top side.
    Top,
    /// Joint along the right side.
    Right,
    /// Joint along bottom side.
    Bottom,
    /// Joint along the left side.
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

impl Joint {
    pub fn new(side: JointSide, pos: JointPosition) -> Self {
        Self {
            own_border: Default::default(),
            other_border: Default::default(),
            side,
            mark: Default::default(),
            mirrored: false,
            pos,
        }
    }

    pub fn border(mut self, own: BorderType) -> Self {
        self.own_border = own;
        self
    }

    pub fn get_border(&self) -> BorderType {
        self.own_border
    }

    pub fn other(mut self, other: BorderType) -> Self {
        self.other_border = other;
        self
    }

    pub fn get_other(&self) -> BorderType {
        self.other_border
    }

    pub fn side(mut self, side: JointSide) -> Self {
        self.side = side;
        self
    }

    pub fn get_side(&self) -> JointSide {
        self.side
    }

    pub fn mark(mut self, mark: JointKind) -> Self {
        self.mark = mark;
        self
    }

    pub fn get_mark(&self) -> JointKind {
        self.mark
    }

    pub fn outward(mut self) -> Self {
        self.mark = JointKind::Outward;
        self
    }

    pub fn inward(mut self) -> Self {
        self.mark = JointKind::Inward;
        self
    }

    pub fn through(mut self) -> Self {
        self.mark = JointKind::Through;
        self
    }

    pub fn manual(mut self, mark: &'static str) -> Self {
        self.mark = JointKind::Manual(mark);
        self
    }

    pub fn mirrored(mut self, mirrored: bool) -> Self {
        self.mirrored = mirrored;
        self
    }

    pub fn is_mirrored(&self) -> bool {
        self.mirrored
    }

    pub fn joint_pos(mut self, pos: JointPosition) -> Self {
        self.pos = pos;
        self
    }

    pub fn get_joint_pos(&self) -> JointPosition {
        self.pos
    }

    pub fn cross_start(mut self, extending_border: BorderType) -> Self {
        self.pos = JointPosition::CrossStart(extending_border);
        self
    }

    pub fn prolong_start(mut self) -> Self {
        self.pos = JointPosition::ProlongStart;
        self
    }

    pub fn start(mut self) -> Self {
        self.pos = JointPosition::Start;
        self
    }

    pub fn pos(mut self, pos: u16) -> Self {
        self.pos = JointPosition::Pos(pos);
        self
    }

    pub fn end(mut self) -> Self {
        self.pos = JointPosition::End;
        self
    }

    pub fn prolong_end(mut self) -> Self {
        self.pos = JointPosition::ProlongEnd;
        self
    }

    pub fn cross_end(mut self, extending_border: BorderType) -> Self {
        self.pos = JointPosition::CrossEnd(extending_border);
        self
    }

    /// Get the location of the mark respective to the given area.
    /// Out of bounds marks return None.
    #[inline]
    pub fn locate(&self, area: Rect) -> Option<u16> {
        match self.side {
            JointSide::Top | JointSide::Bottom => match self.pos {
                JointPosition::CrossStart(_) => Some(0),
                JointPosition::ProlongStart => Some(0),
                JointPosition::Start => Some(0),
                JointPosition::Pos(n) => {
                    if n == 0 {
                        Some(n)
                    } else if n < area.width - 1 {
                        Some(n)
                    } else if n == area.width - 1 {
                        Some(n)
                    } else {
                        None
                    }
                }
                JointPosition::End => Some(area.width - 1),
                JointPosition::ProlongEnd => Some(area.width - 1),
                JointPosition::CrossEnd(_) => Some(area.width - 1),
            },
            JointSide::Right | JointSide::Left => match self.pos {
                JointPosition::CrossStart(_) => Some(0),
                JointPosition::ProlongStart => Some(0),
                JointPosition::Start => Some(0),
                JointPosition::Pos(n) => {
                    if n == 0 {
                        Some(n)
                    } else if n < area.height - 1 {
                        Some(n)
                    } else if n == area.height - 1 {
                        Some(n)
                    } else {
                        None
                    }
                }
                JointPosition::End => Some(area.height - 1),
                JointPosition::ProlongEnd => Some(area.height - 1),
                JointPosition::CrossEnd(_) => Some(area.height - 1),
            },
        }
    }

    // Normalize respective to area.
    // Replaces Pos(n) with Start/End if matching.
    #[inline]
    fn normalized(&self, area: Rect) -> Joint {
        match self.side {
            JointSide::Top | JointSide::Bottom => match self.pos {
                JointPosition::Pos(n) => {
                    if n == 0 {
                        Self {
                            pos: JointPosition::Start,
                            ..*self
                        }
                    } else if n == area.width - 1 {
                        Self {
                            pos: JointPosition::End,
                            ..*self
                        }
                    } else {
                        *self
                    }
                }
                _ => *self,
            },
            JointSide::Right | JointSide::Left => match self.pos {
                JointPosition::Pos(n) => {
                    if n == 0 {
                        Self {
                            pos: JointPosition::Start,
                            ..*self
                        }
                    } else if n == area.height - 1 {
                        Self {
                            pos: JointPosition::End,
                            ..*self
                        }
                    } else {
                        *self
                    }
                }
                _ => *self,
            },
        }
    }
}

impl Widget for Joint {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        render_joint(&self, area, buf);
    }
}

impl Widget for &Joint {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        render_joint(self, area, buf);
    }
}
