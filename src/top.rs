use crate::block_joint::Joint;
use ratatui::symbols::border::{QUADRANT_BOTTOM_HALF, QUADRANT_TOP_HALF};
use ratatui::widgets::BorderType;

pub(crate) fn top_joint(border: BorderType, kind: Joint) -> &'static str {
    use ratatui::widgets::BorderType::*;

    match (border, kind) {
        (Plain | Rounded, Joint::In(Plain | Rounded)) => "┬",
        (Plain | Rounded, Joint::In(Double)) => "╥",
        (Plain | Rounded, Joint::In(Thick)) => "┰",
        (Plain | Rounded, Joint::In(QuadrantInside | QuadrantOutside)) => "┬",
        (Plain | Rounded, Joint::Out(Plain | Rounded)) => "┴",
        (Plain | Rounded, Joint::Out(Double)) => "╨",
        (Plain | Rounded, Joint::Out(Thick)) => "┸",
        (Plain | Rounded, Joint::Out(QuadrantInside | QuadrantOutside)) => "┴",
        (Plain | Rounded, Joint::Through(Plain | Rounded)) => "┼",
        (Plain | Rounded, Joint::Through(Double)) => "╫",
        (Plain | Rounded, Joint::Through(Thick)) => "╂",
        (Plain | Rounded, Joint::Through(QuadrantInside | QuadrantOutside)) => "┼",

        (Double, Joint::In(Plain | Rounded)) => "╤",
        (Double, Joint::In(Double)) => "╦",
        (Double, Joint::In(Thick)) => "╤",
        (Double, Joint::In(QuadrantInside | QuadrantOutside)) => "╦",
        (Double, Joint::Out(Plain | Rounded)) => "╧",
        (Double, Joint::Out(Double)) => "╩",
        (Double, Joint::Out(Thick)) => "╧",
        (Double, Joint::Out(QuadrantInside | QuadrantOutside)) => "╩",
        (Double, Joint::Through(Plain | Rounded)) => "╪",
        (Double, Joint::Through(Double)) => "╬",
        (Double, Joint::Through(Thick)) => "╪",
        (Double, Joint::Through(QuadrantInside | QuadrantOutside)) => "╬",

        (Thick, Joint::In(Plain | Rounded)) => "┯",
        (Thick, Joint::In(Double)) => "┯",
        (Thick, Joint::In(Thick)) => "┳",
        (Thick, Joint::In(QuadrantInside | QuadrantOutside)) => "┳",
        (Thick, Joint::Out(Plain | Rounded)) => "┷",
        (Thick, Joint::Out(Double)) => "┷",
        (Thick, Joint::Out(Thick)) => "┻",
        (Thick, Joint::Out(QuadrantInside | QuadrantOutside)) => "┻",
        (Thick, Joint::Through(Plain | Rounded)) => "┿",
        (Thick, Joint::Through(Double)) => "┿",
        (Thick, Joint::Through(Thick)) => "╋",
        (Thick, Joint::Through(QuadrantInside | QuadrantOutside)) => "╋",

        (QuadrantInside, _) => QUADRANT_BOTTOM_HALF,
        (QuadrantOutside, _) => QUADRANT_TOP_HALF,

        (_, Joint::Manual(c)) => c,
    }
}
