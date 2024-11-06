use ratatui::symbols::border::{
    QUADRANT_TOP_LEFT_BOTTOM_LEFT_BOTTOM_RIGHT, QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_RIGHT,
};
use ratatui::widgets::BorderType;

pub(crate) fn bottom_left_cross_joint(
    border: BorderType,
    down: BorderType,
    left: BorderType,
) -> &'static str {
    use ratatui::widgets::BorderType::*;

    match (border, down, left) {
        (Plain | Rounded, Thick, Thick) => "╅",
        (Plain | Rounded, _, Thick) => "┽",
        (Plain | Rounded, Thick, _) => "╁",
        (Plain | Rounded, _, _) => "┼",

        (Double, _, _) => "╬",

        (Thick, Thick, Thick) => "╋",
        (Thick, _, Thick) => "╇",
        (Thick, Thick, _) => "╊",
        (Thick, _, _) => "╄",

        (QuadrantInside, QuadrantInside, QuadrantInside) => "▞",
        (QuadrantInside, _, QuadrantInside) => "▜",
        (QuadrantInside, QuadrantInside, _) => "▜",
        (QuadrantInside, _, _) => "▜",

        (QuadrantOutside, _, _) => "▙",
    }
}
