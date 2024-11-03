use crate::block_joint::bottom::bottom_joint;
use crate::block_joint::bottom_left_cross::bottom_left_cross_joint;
use crate::block_joint::bottom_left_down::bottom_left_down_joint;
use crate::block_joint::bottom_left_left::bottom_left_left_joint;
use crate::block_joint::bottom_right_cross::bottom_right_cross_joint;
use crate::block_joint::bottom_right_down::bottom_right_down_joint;
use crate::block_joint::bottom_right_right::bottom_right_right_joint;
use crate::block_joint::left::left_joint;
use crate::block_joint::right::right_joint;
use crate::block_joint::top::top_joint;
use crate::block_joint::top_left_cross::top_left_cross_joint;
use crate::block_joint::top_left_left::top_left_left_joint;
use crate::block_joint::top_left_up::top_left_up_joint;
use crate::block_joint::top_right_cross::top_right_cross_joint;
use crate::block_joint::top_right_right::top_right_right_joint;
use crate::block_joint::top_right_up::top_right_up_joint;
use crate::{Joint, JointSide};
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::widgets::BorderType;
mod bottom;
mod bottom_left_cross;
mod bottom_left_down;
mod bottom_left_left;
mod bottom_right_cross;
mod bottom_right_down;
mod bottom_right_right;
mod flip_sides;
mod joint_pos;
mod left;
mod right;
mod top;
mod top_left_cross;
mod top_left_left;
mod top_left_up;
mod top_right_cross;
mod top_right_right;
mod top_right_up;

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
/// # use ratatui_block::{render_joint, Joint, JointSide};
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
///     JointSide::TopLeftCross,
///     Joint::Corner(BorderType::Double, BorderType::Plain),
///     area, buf);
///
/// // Render a corner joint to an area north-west.
/// //
/// // Rendering is best-effort, as not all glyphs for this exist.
/// render_joint(BorderType::Plain,
///     JointSide::TopLeftCross,
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
            Joint::In(_)
            | Joint::Out(_)
            | Joint::Through(_)
            | Joint::AltIn(_)
            | Joint::AltOut(_)
            | Joint::AltThrough(_)
            | Joint::Manual(_),
        ) => render_top(border, pos, joint, area, buf),
        (
            JointSide::Right(pos),
            Joint::In(_)
            | Joint::Out(_)
            | Joint::Through(_)
            | Joint::AltIn(_)
            | Joint::AltOut(_)
            | Joint::AltThrough(_)
            | Joint::Manual(_),
        ) => render_right(border, pos, joint, area, buf),
        (
            JointSide::Bottom(pos),
            Joint::In(_)
            | Joint::Out(_)
            | Joint::Through(_)
            | Joint::AltIn(_)
            | Joint::AltOut(_)
            | Joint::AltThrough(_)
            | Joint::Manual(_),
        ) => render_bottom(border, pos, joint, area, buf),
        (
            JointSide::Left(pos),
            Joint::In(_)
            | Joint::Out(_)
            | Joint::Through(_)
            | Joint::AltIn(_)
            | Joint::AltOut(_)
            | Joint::AltThrough(_)
            | Joint::Manual(_),
        ) => render_left(border, pos, joint, area, buf),

        // corner cases
        (JointSide::TopLeftCross, Joint::Corner(v, h)) => render_top_left(border, v, h, area, buf),
        (JointSide::TopRightCross, Joint::Corner(v, h)) => {
            render_top_right(border, v, h, area, buf)
        }
        (JointSide::BottomRightCross, Joint::Corner(v, h)) => {
            render_bottom_right(border, v, h, area, buf)
        }
        (JointSide::BottomLeftCross, Joint::Corner(v, h)) => {
            render_bottom_left(border, v, h, area, buf)
        }

        // manual corner cases
        (JointSide::TopLeftCross, Joint::Manual(c)) => render_top_left_manual(c, area, buf),
        (JointSide::TopRightCross, Joint::Manual(c)) => render_top_right_manual(c, area, buf),
        (JointSide::BottomRightCross, Joint::Manual(c)) => render_bottom_right_manual(c, area, buf),
        (JointSide::BottomLeftCross, Joint::Manual(c)) => render_bottom_left_manual(c, area, buf),

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

        (JointSide::TopLeftCross, _) => render_top_left_manual("⚠", area, buf),
        (JointSide::TopRightCross, _) => render_top_right_manual("⚠", area, buf),
        (JointSide::BottomRightCross, _) => render_bottom_right_manual("⚠", area, buf),
        (JointSide::BottomLeftCross, _) => render_bottom_left_manual("⚠", area, buf),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Positional {
    Begin,
    Middle,
    End,
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
