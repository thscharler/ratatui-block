use crate::render_joint;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::widgets::BorderType;

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
    pub(crate) kind: JointKind,
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

impl Joint {
    pub fn new(side: JointSide, pos: JointPosition) -> Self {
        Self {
            own_border: Default::default(),
            other_border: Default::default(),
            side,
            kind: Default::default(),
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

    pub fn kind(mut self, kind: JointKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn get_kind(&self) -> JointKind {
        self.kind
    }

    pub fn mirrored(mut self, mirrored: bool) -> Self {
        self.mirrored = mirrored;
        self
    }

    pub fn is_mirrored(&self) -> bool {
        self.mirrored
    }

    pub fn position(mut self, pos: JointPosition) -> Self {
        self.pos = pos;
        self
    }

    pub fn get_position(&self) -> JointPosition {
        self.pos
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
    pub fn normalized(&self, area: Rect) -> Joint {
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
