use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
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
pub enum Joint {
    Out(BorderType),
    In(BorderType),
    Through(BorderType),
    Manual(&'static str),
}

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

fn render_top(
    _border: BorderType,
    pos: JointPosition,
    _kind: Joint,
    area: Rect,
    _buf: &mut Buffer,
) {
    let Some(_pos) = horizontal_joint_pos(pos, area) else {
        return;
    };
}

fn render_right(
    _border: BorderType,
    _pos: JointPosition,
    _kind: Joint,
    _area: Rect,
    _buf: &mut Buffer,
) {
}

fn render_bottom(
    _border: BorderType,
    _pos: JointPosition,
    _kind: Joint,
    _area: Rect,
    _buf: &mut Buffer,
) {
}

fn render_left(
    _border: BorderType,
    _pos: JointPosition,
    _kind: Joint,
    _area: Rect,
    _buf: &mut Buffer,
) {
}

fn horizontal_joint_pos(pos: JointPosition, area: Rect) -> Option<u16> {
    let p = match pos {
        JointPosition::FromStart(n) => area.left().saturating_add(n),
        JointPosition::FromEnd(n) => area.right().saturating_sub(n),
        JointPosition::AtPos(n) => n,
    };

    if p < area.width {
        Some(p)
    } else {
        None
    }
}

fn vertical_joint_pos(pos: JointPosition, area: Rect) -> Option<u16> {
    let p = match pos {
        JointPosition::FromStart(n) => area.top().saturating_add(n),
        JointPosition::FromEnd(n) => area.bottom().saturating_sub(n),
        JointPosition::AtPos(n) => n,
    };

    if p < area.height {
        Some(p)
    } else {
        None
    }
}
