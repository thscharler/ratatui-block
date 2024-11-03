use ratatui::symbols::border::{
    QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT, QUADRANT_TOP_RIGHT_BOTTOM_LEFT_BOTTOM_RIGHT,
};
use ratatui::widgets::BorderType;

pub(crate) fn bottom_right_cross_joint(
    border: BorderType,
    down: BorderType,
    right: BorderType,
) -> &'static str {
    use ratatui::widgets::BorderType::*;

    match (border, down, right) {
        (Plain | Rounded, Thick, Thick) => "╆",
        (Plain | Rounded, _, Thick) => "┾",
        (Plain | Rounded, Thick, _) => "╁",
        (Plain | Rounded, _, _) => "┼",

        (Double, _, _) => "╬",

        (Thick, Thick, Thick) => "╋",
        (Thick, _, Thick) => "╇",
        (Thick, Thick, _) => "╉",
        (Thick, _, _) => "╃",

        (QuadrantInside, _, _) => QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT,
        (QuadrantOutside, _, _) => QUADRANT_TOP_RIGHT_BOTTOM_LEFT_BOTTOM_RIGHT,
    }
}
