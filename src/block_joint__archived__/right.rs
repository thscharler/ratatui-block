use crate::block_joint::flip_sides::flip_sides;
use crate::block_joint::left::left_joint;
use crate::block_joint::Joint;
use ratatui::symbols::border::{
    QUADRANT_RIGHT_HALF, QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT,
    QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_RIGHT,
};
use ratatui::widgets::BorderType;

pub(crate) fn right_joint(border: BorderType, joint: Joint) -> &'static str {
    use ratatui::widgets::BorderType::*;

    match (border, joint) {
        (QuadrantInside, _) => QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT,

        (QuadrantOutside, Joint::In(_) | Joint::Through(_)) => {
            QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_RIGHT
        }
        (QuadrantOutside, _) => QUADRANT_RIGHT_HALF,

        (_, Joint::AltIn(_)) => "⚠",
        (_, Joint::AltOut(_)) => "⚠",
        (_, Joint::AltThrough(_)) => "⚠",

        _ => {
            let (border, joint) = flip_sides(border, joint);
            left_joint(border, joint)
        }
    }
}
