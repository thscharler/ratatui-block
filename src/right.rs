use crate::block_joint::Joint;
use crate::flip_sides::flip_sides;
use crate::left::left_joint;
use ratatui::symbols::block::FULL;
use ratatui::symbols::border::{QUADRANT_RIGHT_HALF, QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT};
use ratatui::widgets::BorderType;

pub(crate) fn right_joint(border: BorderType, kind: Joint) -> &'static str {
    use ratatui::widgets::BorderType::*;

    match (border, kind) {
        (QuadrantInside, _) => QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT,
        (QuadrantOutside, _) => QUADRANT_RIGHT_HALF,

        _ => {
            let (border, kind) = flip_sides(border, kind);
            left_joint(border, kind)
        }
    }
}
