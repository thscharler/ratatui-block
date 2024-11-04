#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{BorderType, Widget};

/// A joint in a Block border.
#[derive(Debug, Clone, Copy)]
pub struct Joint {
    /// The main border to be decorated.
    pub(crate) own_border: BorderType,
    /// The border type to join with.
    /// This is the border for areas outward of the side.
    pub(crate) other_border: BorderType,
    /// Side of the area.
    pub(crate) side: JointSide,
    /// Joint mark.
    pub(crate) mark: JointMark,
    /// Mirrored joint. This is needed for QuadrantInside and QuadrantOutside.
    /// Those have mirrored glyphs on each side.
    pub(crate) mirrored: bool,
    /// Position for the join.
    pub(crate) pos: JointPos,
}

/// Marktype for the joints.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum JointMark {
    /// Outward join.
    #[default]
    Out,
    /// Inward join.
    In,
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
    CrossStart(BorderType),
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
    CrossEnd(BorderType),
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

impl Joint {
    pub fn new(side: JointSide, pos: JointPos) -> Self {
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

    pub fn mark(mut self, mark: JointMark) -> Self {
        self.mark = mark;
        self
    }

    pub fn get_mark(&self) -> JointMark {
        self.mark
    }

    pub fn outward(mut self) -> Self {
        self.mark = JointMark::Out;
        self
    }

    pub fn inward(mut self) -> Self {
        self.mark = JointMark::In;
        self
    }

    pub fn through(mut self) -> Self {
        self.mark = JointMark::Through;
        self
    }

    pub fn manual(mut self, mark: &'static str) -> Self {
        self.mark = JointMark::Manual(mark);
        self
    }

    pub fn mirrored(mut self, mirrored: bool) -> Self {
        self.mirrored = mirrored;
        self
    }

    pub fn is_mirrored(&self) -> bool {
        self.mirrored
    }

    pub fn joint_pos(mut self, pos: JointPos) -> Self {
        self.pos = pos;
        self
    }

    pub fn get_joint_pos(&self) -> JointPos {
        self.pos
    }

    pub fn cross_start(mut self, extending_border: BorderType) -> Self {
        self.pos = JointPos::CrossStart(extending_border);
        self
    }

    pub fn prolong_start(mut self) -> Self {
        self.pos = JointPos::ProlongStart;
        self
    }

    pub fn start(mut self) -> Self {
        self.pos = JointPos::Start;
        self
    }

    pub fn pos(mut self, pos: u16) -> Self {
        self.pos = JointPos::Pos(pos);
        self
    }

    pub fn end(mut self) -> Self {
        self.pos = JointPos::End;
        self
    }

    pub fn prolong_end(mut self) -> Self {
        self.pos = JointPos::ProlongEnd;
        self
    }

    pub fn cross_end(mut self, extending_border: BorderType) -> Self {
        self.pos = JointPos::CrossEnd(extending_border);
        self
    }

    /// Get the location of the mark respective to the given area.
    /// Out of bounds marks return None.
    #[inline]
    pub fn locate(&self, area: Rect) -> Option<u16> {
        match self.side {
            JointSide::Top | JointSide::Bottom => match self.pos {
                JointPos::CrossStart(_) => Some(0),
                JointPos::ProlongStart => Some(0),
                JointPos::Start => Some(0),
                JointPos::Pos(n) => {
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
                JointPos::End => Some(area.width - 1),
                JointPos::ProlongEnd => Some(area.width - 1),
                JointPos::CrossEnd(_) => Some(area.width - 1),
            },
            JointSide::Right | JointSide::Left => match self.pos {
                JointPos::CrossStart(_) => Some(0),
                JointPos::ProlongStart => Some(0),
                JointPos::Start => Some(0),
                JointPos::Pos(n) => {
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
                JointPos::End => Some(area.height - 1),
                JointPos::ProlongEnd => Some(area.height - 1),
                JointPos::CrossEnd(_) => Some(area.height - 1),
            },
        }
    }

    // Normalize respective to area.
    // Replaces Pos(n) with Start/End if matching.
    #[inline]
    fn normalized(&self, area: Rect) -> Joint {
        match self.side {
            JointSide::Top | JointSide::Bottom => match self.pos {
                JointPos::Pos(n) => {
                    if n == 0 {
                        Self {
                            pos: JointPos::Start,
                            ..*self
                        }
                    } else if n == area.width - 1 {
                        Self {
                            pos: JointPos::End,
                            ..*self
                        }
                    } else {
                        *self
                    }
                }
                _ => *self,
            },
            JointSide::Right | JointSide::Left => match self.pos {
                JointPos::Pos(n) => {
                    if n == 0 {
                        Self {
                            pos: JointPos::Start,
                            ..*self
                        }
                    } else if n == area.height - 1 {
                        Self {
                            pos: JointPos::End,
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
