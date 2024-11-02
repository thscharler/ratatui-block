use crate::block_joint::Joint;
use ratatui::symbols::border::{QUADRANT_BOTTOM_RIGHT, QUADRANT_TOP_RIGHT};
use ratatui::widgets::BorderType;
use ratatui::widgets::BorderType::{QuadrantInside, QuadrantOutside};

pub(crate) fn bottom_left_down_joint(border: BorderType, kind: Joint) -> &'static str {
    use ratatui::widgets::BorderType::*;

    match (border, kind) {
        (Plain | Rounded, Joint::In(_)) => "└",
        (
            Plain | Rounded,
            Joint::Out(Plain | Rounded | Double | QuadrantInside | QuadrantOutside)
            | Joint::Through(Plain | Rounded | Double | QuadrantInside | QuadrantOutside),
        ) => "├",
        (Plain | Rounded, Joint::Out(Thick) | Joint::Through(Thick)) => "┟",

        (Double, Joint::In(_)) => "╚",
        (Double, Joint::Out(_) | Joint::Through(_)) => "╠",

        (Thick, Joint::In(_)) => "┗",
        (
            Thick,
            Joint::Out(Plain | Rounded | Double) | Joint::Through(Plain | Rounded | Double),
        ) => "┡",
        (
            Thick,
            Joint::Out(Thick | QuadrantInside | QuadrantOutside)
            | Joint::Through(Thick | QuadrantInside | QuadrantOutside),
        ) => "┣",

        (QuadrantInside, _) => QUADRANT_BOTTOM_RIGHT,
        (QuadrantOutside, _) => QUADRANT_TOP_RIGHT,

        (_, Joint::Manual(c)) => c,
    }
}
