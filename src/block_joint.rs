use crate::bottom::bottom_joint;
use crate::bottom_left_down::bottom_left_down_joint;
use crate::bottom_left_left::bottom_left_left_joint;
use crate::bottom_right_down::bottom_right_down_joint;
use crate::bottom_right_right::bottom_right_right_joint;
use crate::joint_pos::{horizontal_joint_pos, vertical_joint_pos};
use crate::left::left_joint;
use crate::right::right_joint;
use crate::top::top_joint;
use crate::top_left_cross::top_left_cross_joint;
use crate::top_left_left::top_left_left_joint;
use crate::top_left_up::top_left_up_joint;
use crate::top_right_cross::top_right_cross_joint;
use crate::top_right_right::top_right_right_joint;
use crate::top_right_up::top_right_up_joint;
use log::debug;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::widgets::BorderType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointSide {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointPosition {
    FromStart(u16),
    FromEnd(u16),
    AtPos(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Joint {
    Out(BorderType),
    In(BorderType),
    Through(BorderType),
    Manual(&'static str),
}

/// Render a cross joint in a corner of the area.
pub fn render_cross_joint(
    border: BorderType,
    pos: JointCorner,
    vert_kind: Joint,
    hor_kind: Joint,
    area: Rect,
    buf: &mut Buffer,
) {
    match pos {
        JointCorner::TopLeft => {
            let sym = top_left_cross_joint(border, vert_kind, hor_kind);
            if let Some(cell) = buf.cell_mut(Position::new(area.left(), area.top())) {
                cell.set_symbol(sym);
            }
        }
        JointCorner::TopRight => {
            let sym = top_right_cross_joint(border, vert_kind, hor_kind);
            if let Some(cell) =
                buf.cell_mut(Position::new(area.right().saturating_sub(1), area.top()))
            {
                cell.set_symbol(sym);
            }
        }
        JointCorner::BottomLeft => {
            let sym = top_left_cross_joint(border, vert_kind, hor_kind);
            if let Some(cell) =
                buf.cell_mut(Position::new(area.left(), area.bottom().saturating_sub(1)))
            {
                cell.set_symbol(sym);
            }
        }
        JointCorner::BottomRight => {
            let sym = top_left_cross_joint(border, vert_kind, hor_kind);
            if let Some(cell) = buf.cell_mut(Position::new(
                area.right().saturating_sub(1),
                area.bottom().saturating_sub(1),
            )) {
                cell.set_symbol(sym);
            }
        }
    }
}

/// Render joints at some side of the area.
pub fn render_joint(
    border: BorderType,
    side: JointSide,
    pos: JointPosition,
    kind: Joint,
    area: Rect,
    buf: &mut Buffer,
) {
    match side {
        JointSide::Top => render_top(border, pos, kind, area, buf),
        JointSide::Right => render_right(border, pos, kind, area, buf),
        JointSide::Bottom => render_bottom(border, pos, kind, area, buf),
        JointSide::Left => render_left(border, pos, kind, area, buf),
    }
}

fn render_top(border: BorderType, pos: JointPosition, kind: Joint, area: Rect, buf: &mut Buffer) {
    let Some(pos) = horizontal_joint_pos(pos, area) else {
        debug!("oob");
        return;
    };

    let sym = if pos == 0 {
        debug!("c0");
        top_left_up_joint(border, kind)
    } else if pos == area.width.saturating_sub(1) {
        debug!("c1");
        top_right_up_joint(border, kind)
    } else {
        debug!("c2");
        top_joint(border, kind)
    };
    debug!("=> {:?}", sym);

    if let Some(cell) = buf.cell_mut(Position::new(area.left() + pos, area.top())) {
        cell.set_symbol(sym);
    }
}

fn render_bottom(
    border: BorderType,
    pos: JointPosition,
    kind: Joint,
    area: Rect,
    buf: &mut Buffer,
) {
    let Some(pos) = horizontal_joint_pos(pos, area) else {
        return;
    };

    let sym = if pos == 0 {
        bottom_left_down_joint(border, kind)
    } else if pos == area.width.saturating_sub(1) {
        bottom_right_down_joint(border, kind)
    } else {
        bottom_joint(border, kind)
    };

    if let Some(cell) = buf.cell_mut(Position::new(
        area.left() + pos,
        area.bottom().saturating_sub(1),
    )) {
        cell.set_symbol(sym);
    }
}

fn render_right(border: BorderType, pos: JointPosition, kind: Joint, area: Rect, buf: &mut Buffer) {
    let Some(pos) = vertical_joint_pos(pos, area) else {
        return;
    };

    let sym = if pos == 0 {
        top_right_right_joint(border, kind)
    } else if pos == area.height.saturating_sub(1) {
        bottom_right_right_joint(border, kind)
    } else {
        right_joint(border, kind)
    };

    if let Some(cell) = buf.cell_mut(Position::new(
        area.right().saturating_sub(1),
        area.top() + pos,
    )) {
        cell.set_symbol(sym);
    }
}

fn render_left(border: BorderType, pos: JointPosition, kind: Joint, area: Rect, buf: &mut Buffer) {
    let Some(pos) = vertical_joint_pos(pos, area) else {
        return;
    };

    let sym = if pos == 0 {
        top_left_left_joint(border, kind)
    } else if pos == area.height.saturating_sub(1) {
        bottom_left_left_joint(border, kind)
    } else {
        left_joint(border, kind)
    };

    if let Some(cell) = buf.cell_mut(Position::new(area.left(), area.top() + pos)) {
        cell.set_symbol(sym);
    }
}
