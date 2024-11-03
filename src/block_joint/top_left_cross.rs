use ratatui::symbols::border::{
    QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT, QUADRANT_TOP_RIGHT_BOTTOM_LEFT_BOTTOM_RIGHT,
};
use ratatui::widgets::BorderType;

pub(crate) fn top_left_cross_joint(
    border: BorderType,
    up: BorderType,
    left: BorderType,
) -> &'static str {
    use ratatui::widgets::BorderType::*;

    match (border, up, left) {
        (Plain | Rounded, Thick, Thick) => "╃",
        (Plain | Rounded, _, Thick) => "┽",
        (Plain | Rounded, Thick, _) => "╀",
        (Plain | Rounded, _, _) => "┼",

        (Double, _, _) => "╬",

        (Thick, Thick, Thick) => "╋",
        (Thick, _, Thick) => "╈",
        (Thick, Thick, _) => "╊",
        (Thick, _, _) => "\u{2546}",

        (QuadrantInside, _, _) => QUADRANT_TOP_RIGHT_BOTTOM_LEFT_BOTTOM_RIGHT,
        (QuadrantOutside, _, _) => QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT,
    }
}
