use crate::block_joint::Joint;
use crate::top_left_left::top_left_left_joint;
use crate::top_left_up::top_left_up_joint;
use ratatui::symbols::border::{QUADRANT_BOTTOM_RIGHT, QUADRANT_TOP_RIGHT};
use ratatui::widgets::BorderType;

pub(crate) fn top_left_cross_joint(
    border: BorderType,
    up_kind: Joint,
    left_kind: Joint,
) -> &'static str {
    use ratatui::widgets::BorderType::*;

    match (border, up_kind, left_kind) {
        (_, Joint::In(_), _) => top_left_left_joint(border, left_kind),
        (_, _, Joint::In(_)) => top_left_up_joint(border, up_kind),

        (
            Plain | Rounded,
            Joint::Out(Thick) | Joint::Through(Thick),
            Joint::Out(Thick) | Joint::Through(Thick),
        ) => "╃",
        (
            Plain | Rounded,
            Joint::Out(Thick) | Joint::Through(Thick),
            Joint::Out(_) | Joint::Through(_),
        ) => "╀",
        (
            Plain | Rounded,
            Joint::Out(_) | Joint::Through(_),
            Joint::Out(Thick) | Joint::Through(Thick),
        ) => "┽",
        (
            Plain | Rounded,
            Joint::Out(_) | Joint::Through(_), //
            Joint::Out(_) | Joint::Through(_),
        ) => "┼",

        (
            Double,
            Joint::Out(_) | Joint::Through(_), //
            Joint::Out(_) | Joint::Through(_),
        ) => "╬",

        (
            Thick,
            Joint::Out(Thick) | Joint::Through(Thick),
            Joint::Out(Thick) | Joint::Through(Thick),
        ) => "╋",
        (
            Thick,
            Joint::Out(Thick) | Joint::Through(Thick), //
            Joint::Out(_) | Joint::Through(_),
        ) => "╊",
        (
            Thick,
            Joint::Out(_) | Joint::Through(_), //
            Joint::Out(Thick) | Joint::Through(Thick),
        ) => "╈",
        (
            Thick,
            Joint::Out(_) | Joint::Through(_), //
            Joint::Out(_) | Joint::Through(_),
        ) => "╅",

        (QuadrantInside, _, _) => QUADRANT_BOTTOM_RIGHT,
        (QuadrantOutside, _, _) => QUADRANT_TOP_RIGHT,

        (_, Joint::Manual(c), Joint::Manual(_)) => c,
        (_, Joint::Manual(c), _) => c,
        (_, _, Joint::Manual(d)) => d,
    }
}
