use crate::block_joint::Joint;
use crate::flip_sides::flip_sides;
use crate::top::top_joint;
use ratatui::widgets::BorderType;

pub(crate) fn bottom_joint(border: BorderType, kind: Joint) -> &'static str {
    let (border, kind) = flip_sides(border, kind);
    top_joint(border, kind)
}
