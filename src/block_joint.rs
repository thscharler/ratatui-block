use crate::bottom::bottom_joint;
use crate::bottom_left_cross::bottom_left_cross_joint;
use crate::bottom_left_down::bottom_left_down_joint;
use crate::bottom_left_left::bottom_left_left_joint;
use crate::bottom_right_cross::bottom_right_cross_joint;
use crate::bottom_right_down::bottom_right_down_joint;
use crate::bottom_right_right::bottom_right_right_joint;
use crate::left::left_joint;
use crate::right::right_joint;
use crate::top::top_joint;
use crate::top_left_cross::top_left_cross_joint;
use crate::top_left_left::top_left_left_joint;
use crate::top_left_up::top_left_up_joint;
use crate::top_right_cross::top_right_cross_joint;
use crate::top_right_right::top_right_right_joint;
use crate::top_right_up::top_right_up_joint;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
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

/// Render joints at some side of the area.
///
/// * border: Border of the Rect to decorate.
/// * side: Which side of the Rect.
/// * joint: What type of joint `Out`wards, `In`wards or `Through`.
///          Or a `Corner` joint.
///
/// ```rust no_run
/// # use ratatui::buffer::Buffer;
/// # use ratatui::layout::Rect;
/// # use ratatui::widgets::BorderType;
/// # use ratatui_block::block_joint::{render_joint, Joint, JointSide};
/// #
/// # let mut buf = &mut Buffer::default();
/// # let area = Rect::default();
///
/// // Top/left corner, upwards join.
/// render_joint(BorderType::Plain,
///     JointSide::Top(0),
///     Joint::Out(BorderType::Plain),
///     area, buf);
///
/// // Render a two-way joint to a `Double` area above and a
/// // `Plain` area to the left.
/// //
/// // Rendering is best-effort, as not all glyphs for this exist.
/// render_joint(BorderType::Plain,
///     JointSide::TopLeft,
///     Joint::Corner(BorderType::Double, BorderType::Plain),
///     area, buf);
///
/// // Render a corner joint to an area north-west.
/// //
/// // Rendering is best-effort, as not all glyphs for this exist.
/// render_joint(BorderType::Plain,
///     JointSide::TopLeft,
///     Joint::Corner(BorderType::Plain,BorderType::Plain),
///     area, buf);
///
/// ```
pub fn render_joint(
    border: BorderType,
    side: JointSide,
    joint: Joint,
    area: Rect,
    buf: &mut Buffer,
) {
    match (side, joint) {
        // sides
        (
            JointSide::Top(pos),
            Joint::In(_) | Joint::Out(_) | Joint::Through(_) | Joint::Manual(_),
        ) => render_top(border, pos, joint, area, buf),
        (
            JointSide::Right(pos),
            Joint::In(_) | Joint::Out(_) | Joint::Through(_) | Joint::Manual(_),
        ) => render_right(border, pos, joint, area, buf),
        (
            JointSide::Bottom(pos),
            Joint::In(_) | Joint::Out(_) | Joint::Through(_) | Joint::Manual(_),
        ) => render_bottom(border, pos, joint, area, buf),
        (
            JointSide::Left(pos),
            Joint::In(_) | Joint::Out(_) | Joint::Through(_) | Joint::Manual(_),
        ) => render_left(border, pos, joint, area, buf),

        // corner cases
        (JointSide::TopLeft, Joint::Corner(v, h)) => render_top_left(border, v, h, area, buf),
        (JointSide::TopRight, Joint::Corner(v, h)) => render_top_right(border, v, h, area, buf),
        (JointSide::BottomRight, Joint::Corner(v, h)) => {
            render_bottom_right(border, v, h, area, buf)
        }
        (JointSide::BottomLeft, Joint::Corner(v, h)) => render_bottom_left(border, v, h, area, buf),

        // manual corner cases
        (JointSide::TopLeft, Joint::Manual(c)) => render_top_left_manual(c, area, buf),
        (JointSide::TopRight, Joint::Manual(c)) => render_top_right_manual(c, area, buf),
        (JointSide::BottomRight, Joint::Manual(c)) => render_bottom_right_manual(c, area, buf),
        (JointSide::BottomLeft, Joint::Manual(c)) => render_bottom_left_manual(c, area, buf),

        // impossible matches
        (JointSide::Top(_), Joint::Corner(_, _)) => {
            render_top_left_manual("⚠", area, buf);
            render_top_right_manual("⚠", area, buf);
        }
        (JointSide::Right(_), Joint::Corner(_, _)) => {
            render_bottom_right_manual("⚠", area, buf);
            render_top_right_manual("⚠", area, buf);
        }
        (JointSide::Bottom(_), Joint::Corner(_, _)) => {
            render_bottom_left_manual("⚠", area, buf);
            render_bottom_right_manual("⚠", area, buf);
        }
        (JointSide::Left(_), Joint::Corner(_, _)) => {
            render_top_left_manual("⚠", area, buf);
            render_bottom_left_manual("⚠", area, buf);
        }

        (JointSide::TopLeft, _) => render_top_left_manual("⚠", area, buf),
        (JointSide::TopRight, _) => render_top_right_manual("⚠", area, buf),
        (JointSide::BottomRight, _) => render_bottom_right_manual("⚠", area, buf),
        (JointSide::BottomLeft, _) => render_bottom_left_manual("⚠", area, buf),
    }
}

fn render_top_left_manual(sym: &'static str, area: Rect, buf: &mut Buffer) {
    if let Some(cell) = buf.cell_mut(Position::new(area.left(), area.top())) {
        cell.set_symbol(sym);
    }
}

fn render_top_right_manual(sym: &'static str, area: Rect, buf: &mut Buffer) {
    if let Some(cell) = buf.cell_mut(Position::new(area.right().saturating_sub(1), area.top())) {
        cell.set_symbol(sym);
    }
}

fn render_bottom_left_manual(sym: &'static str, area: Rect, buf: &mut Buffer) {
    if let Some(cell) = buf.cell_mut(Position::new(area.left(), area.bottom().saturating_sub(1))) {
        cell.set_symbol(sym);
    }
}

fn render_bottom_right_manual(sym: &'static str, area: Rect, buf: &mut Buffer) {
    if let Some(cell) = buf.cell_mut(Position::new(
        area.right().saturating_sub(1),
        area.bottom().saturating_sub(1),
    )) {
        cell.set_symbol(sym);
    }
}

fn render_top_left(
    border: BorderType,
    vert_joint: BorderType,
    hor_joint: BorderType,
    area: Rect,
    buf: &mut Buffer,
) {
    let sym = top_left_cross_joint(border, vert_joint, hor_joint);
    if let Some(cell) = buf.cell_mut(Position::new(area.left(), area.top())) {
        cell.set_symbol(sym);
    }
}

fn render_top_right(
    border: BorderType,
    vert_joint: BorderType,
    hor_joint: BorderType,
    area: Rect,
    buf: &mut Buffer,
) {
    let sym = top_right_cross_joint(border, vert_joint, hor_joint);
    if let Some(cell) = buf.cell_mut(Position::new(area.right().saturating_sub(1), area.top())) {
        cell.set_symbol(sym);
    }
}

fn render_bottom_left(
    border: BorderType,
    vert_joint: BorderType,
    hor_joint: BorderType,
    area: Rect,
    buf: &mut Buffer,
) {
    let sym = bottom_left_cross_joint(border, vert_joint, hor_joint);
    if let Some(cell) = buf.cell_mut(Position::new(area.left(), area.bottom().saturating_sub(1))) {
        cell.set_symbol(sym);
    }
}

fn render_bottom_right(
    border: BorderType,
    vert_joint: BorderType,
    hor_joint: BorderType,
    area: Rect,
    buf: &mut Buffer,
) {
    let sym = bottom_right_cross_joint(border, vert_joint, hor_joint);
    if let Some(cell) = buf.cell_mut(Position::new(
        area.right().saturating_sub(1),
        area.bottom().saturating_sub(1),
    )) {
        cell.set_symbol(sym);
    }
}

fn render_top(border: BorderType, pos: u16, joint: Joint, area: Rect, buf: &mut Buffer) {
    let sym = if pos == 0 {
        top_left_up_joint(border, joint)
    } else if pos == area.width.saturating_sub(1) {
        top_right_up_joint(border, joint)
    } else {
        top_joint(border, joint)
    };

    if let Some(cell) = buf.cell_mut(Position::new(area.left() + pos, area.top())) {
        cell.set_symbol(sym);
    }
}

fn render_bottom(border: BorderType, pos: u16, joint: Joint, area: Rect, buf: &mut Buffer) {
    let sym = if pos == 0 {
        bottom_left_down_joint(border, joint)
    } else if pos == area.width.saturating_sub(1) {
        bottom_right_down_joint(border, joint)
    } else {
        bottom_joint(border, joint)
    };

    if let Some(cell) = buf.cell_mut(Position::new(
        area.left() + pos,
        area.bottom().saturating_sub(1),
    )) {
        cell.set_symbol(sym);
    }
}

fn render_right(border: BorderType, pos: u16, joint: Joint, area: Rect, buf: &mut Buffer) {
    let sym = if pos == 0 {
        top_right_right_joint(border, joint)
    } else if pos == area.height.saturating_sub(1) {
        bottom_right_right_joint(border, joint)
    } else {
        right_joint(border, joint)
    };

    if let Some(cell) = buf.cell_mut(Position::new(
        area.right().saturating_sub(1),
        area.top() + pos,
    )) {
        cell.set_symbol(sym);
    }
}

fn render_left(border: BorderType, pos: u16, joint: Joint, area: Rect, buf: &mut Buffer) {
    let sym = if pos == 0 {
        top_left_left_joint(border, joint)
    } else if pos == area.height.saturating_sub(1) {
        bottom_left_left_joint(border, joint)
    } else {
        left_joint(border, joint)
    };

    if let Some(cell) = buf.cell_mut(Position::new(area.left(), area.top() + pos)) {
        cell.set_symbol(sym);
    }
}
