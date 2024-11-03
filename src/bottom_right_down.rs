use crate::block_joint::Joint;
use ratatui::symbols::border::{QUADRANT_LEFT_HALF, QUADRANT_TOP_RIGHT_BOTTOM_LEFT_BOTTOM_RIGHT};
use ratatui::widgets::BorderType;

pub(crate) fn bottom_right_down_joint(border: BorderType, joint: Joint) -> &'static str {
    use ratatui::widgets::BorderType::*;

    match (border, joint) {
        (Plain | Rounded, Joint::In(_)) => "┘",
        (
            Plain | Rounded,
            Joint::Out(Plain | Rounded | Double | QuadrantInside | QuadrantOutside)
            | Joint::Through(Plain | Rounded | Double | QuadrantInside | QuadrantOutside),
        ) => "┤",
        (Plain | Rounded, Joint::Out(Thick) | Joint::Through(Thick)) => "┧",

        (Double, Joint::In(_)) => "╝",
        (Double, Joint::Out(_) | Joint::Through(_)) => "╣",

        (Thick, Joint::In(_)) => "┛",
        (
            Thick,
            Joint::Out(Plain | Rounded | Double) | Joint::Through(Plain | Rounded | Double),
        ) => "┩",
        (
            Thick,
            Joint::Out(Thick | QuadrantInside | QuadrantOutside)
            | Joint::Through(Thick | QuadrantInside | QuadrantOutside),
        ) => "┫",

        (QuadrantInside, _) => QUADRANT_LEFT_HALF,
        (QuadrantOutside, _) => QUADRANT_TOP_RIGHT_BOTTOM_LEFT_BOTTOM_RIGHT,

        (_, Joint::Manual(c)) => c,
        (_, Joint::Corner(_, _)) => "⚠",
    }
}
