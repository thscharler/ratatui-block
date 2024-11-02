use crate::block_joint::Joint;
use crate::flip_sides::flip_sides;
use crate::left::left_joint;
use ratatui::widgets::BorderType;

pub(crate) fn right_joint(border: BorderType, kind: Joint) -> &'static str {
    let (border, kind) = flip_sides(border, kind);
    left_joint(border, kind)
}
