use crate::block_joint::Joint;
use crate::flip_sides::flip_sides;
use crate::horizontal_left::horizontal_left_joint;
use ratatui::widgets::BorderType;

pub(crate) fn horizontal_right_joint(border: BorderType, kind: Joint) -> &'static str {
    let (border, kind) = flip_sides(border, kind);
    horizontal_left_joint(border, kind)
}
