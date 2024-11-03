use crate::block_joint::Joint;
use ratatui::symbols::border::{
    QUADRANT_LEFT_HALF, QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT,
    QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_RIGHT, QUADRANT_TOP_RIGHT_BOTTOM_LEFT_BOTTOM_RIGHT,
};
use ratatui::widgets::BorderType;
use ratatui::widgets::BorderType::{Plain, QuadrantInside, Rounded};

pub(crate) const fn left_joint(border: BorderType, joint: Joint) -> &'static str {
    use ratatui::widgets::BorderType::*;

    match (border, joint) {
        (Plain | Rounded, Joint::In(Plain | Rounded)) => "├",
        (Plain | Rounded, Joint::In(Double)) => "╞",
        (Plain | Rounded, Joint::In(Thick)) => "┝",
        (Plain | Rounded, Joint::In(QuadrantInside | QuadrantOutside)) => "├",
        (Plain | Rounded, Joint::Out(Plain | Rounded)) => "┤",
        (Plain | Rounded, Joint::Out(Double)) => "╡",
        (Plain | Rounded, Joint::Out(Thick)) => "┥",
        (Plain | Rounded, Joint::Out(QuadrantInside | QuadrantOutside)) => "┤",
        (Plain | Rounded, Joint::Through(Plain | Rounded)) => "┼",
        (Plain | Rounded, Joint::Through(Double)) => "╪",
        (Plain | Rounded, Joint::Through(Thick)) => "┿",
        (Plain | Rounded, Joint::Through(QuadrantInside | QuadrantOutside)) => "┼",

        (Double, Joint::In(Plain | Rounded)) => "╟",
        (Double, Joint::In(Double)) => "╠",
        (Double, Joint::In(Thick)) => "╟",
        (Double, Joint::In(QuadrantInside | QuadrantOutside)) => "╟",
        (Double, Joint::Out(Plain | Rounded)) => "╢",
        (Double, Joint::Out(Double)) => "╣",
        (Double, Joint::Out(Thick)) => "╢",
        (Double, Joint::Out(QuadrantInside | QuadrantOutside)) => "╣",
        (Double, Joint::Through(Plain | Rounded)) => "╫",
        (Double, Joint::Through(Double)) => "╬",
        (Double, Joint::Through(Thick)) => "╫",
        (Double, Joint::Through(QuadrantInside | QuadrantOutside)) => "╬",

        (Thick, Joint::In(Plain | Rounded)) => "┠",
        (Thick, Joint::In(Double)) => "┠",
        (Thick, Joint::In(Thick)) => "┣",
        (Thick, Joint::In(QuadrantInside | QuadrantOutside)) => "┣",
        (Thick, Joint::Out(Plain | Rounded)) => "┨",
        (Thick, Joint::Out(Double)) => "┨",
        (Thick, Joint::Out(Thick)) => "┫",
        (Thick, Joint::Out(QuadrantInside | QuadrantOutside)) => "┫",
        (Thick, Joint::Through(Plain | Rounded)) => "╂",
        (Thick, Joint::Through(Double)) => "╂",
        (Thick, Joint::Through(Thick)) => "╋",
        (Thick, Joint::Through(QuadrantInside | QuadrantOutside)) => "╋",

        (QuadrantInside, Joint::Out(_) | Joint::Through(_)) => "▜",
        (QuadrantInside, Joint::AltOut(_) | Joint::AltThrough(_)) => "▟",
        (QuadrantInside, Joint::In(_) | Joint::AltIn(_)) => "▐",

        (QuadrantOutside, Joint::In(_) | Joint::Through(_)) => "▛",
        (QuadrantOutside, _) => "▌",

        (_, Joint::Manual(c)) => c,
        (_, Joint::Corner(_, _)) => "⚠",
        (_, Joint::AltIn(_)) => "⚠",
        (_, Joint::AltOut(_)) => "⚠",
        (_, Joint::AltThrough(_)) => "⚠",
    }
}
