use crate::block_joint::Joint;
use crate::flip_sides::flip_sides;
use crate::top::top_joint;
use ratatui::symbols::border::{
    QUADRANT_BOTTOM_HALF, QUADRANT_TOP_LEFT_BOTTOM_LEFT_BOTTOM_RIGHT,
    QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT,
};
use ratatui::widgets::BorderType;

pub(crate) fn bottom_joint(border: BorderType, joint: Joint) -> &'static str {
    use ratatui::widgets::BorderType::*;

    match (border, joint) {
        (QuadrantInside, _) => QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT,
        (QuadrantOutside, Joint::In(_) | Joint::Through(_)) => {
            QUADRANT_TOP_LEFT_BOTTOM_LEFT_BOTTOM_RIGHT
        }
        (QuadrantOutside, _) => QUADRANT_BOTTOM_HALF,

        _ => {
            let (border, joint) = flip_sides(border, joint);
            top_joint(border, joint)
        }
    }
}
