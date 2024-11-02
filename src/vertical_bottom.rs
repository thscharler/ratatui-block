use crate::block_joint::Joint;
use crate::flip_sides::flip_sides;
use crate::vertical_top::vertical_top_joint;
use ratatui::widgets::BorderType;

pub(crate) fn vertical_bottom_joint(border: BorderType, kind: Joint) -> &'static str {
    let (border, kind) = flip_sides(border, kind);
    vertical_top_joint(border, kind)
}
