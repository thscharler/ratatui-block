use crate::block_joint::{Joint, Positional};
use ratatui::widgets::BorderType;
use ratatui::widgets::BorderType::{Double, QuadrantInside, QuadrantOutside, Thick};

pub(crate) const fn top_joint(border: BorderType, joint: Joint, pos: Positional) -> &'static str {
    use ratatui::widgets::BorderType::*;

    if pos == Positional::Begin {
    } else if pos == Positional::Middle {
        match (border, joint) {
            (Plain | Rounded, Joint::In(Plain | Rounded | QuadrantInside | QuadrantOutside)) => "┬",
            (Plain | Rounded, Joint::In(Double)) => "╥",
            (Plain | Rounded, Joint::In(Thick)) => "┰",
            (Plain | Rounded, Joint::Out(Plain | Rounded | QuadrantInside | QuadrantOutside)) => {
                "┴"
            }
            (Plain | Rounded, Joint::Out(Double)) => "╨",
            (Plain | Rounded, Joint::Out(Thick)) => "┸",
            (
                Plain | Rounded,
                Joint::Through(Plain | Rounded | QuadrantInside | QuadrantOutside),
            ) => "┼",
            (Plain | Rounded, Joint::Through(Double)) => "╫",
            (Plain | Rounded, Joint::Through(Thick)) => "╂",

            (Double, Joint::In(Plain | Rounded | Thick)) => "╤",
            (Double, Joint::In(Double | QuadrantInside | QuadrantOutside)) => "╦",
            (Double, Joint::Out(Plain | Rounded | Thick)) => "╧",
            (Double, Joint::Out(Double | QuadrantInside | QuadrantOutside)) => "╩",
            (Double, Joint::Through(Plain | Rounded | Thick)) => "╪",
            (Double, Joint::Through(Double | QuadrantInside | QuadrantOutside)) => "╬",

            (Thick, Joint::In(Plain | Rounded)) => "┯",
            (Thick, Joint::In(Thick | Double | QuadrantInside | QuadrantOutside)) => "┳",
            (Thick, Joint::Out(Plain | Rounded)) => "┷",
            (Thick, Joint::Out(Thick | Double | QuadrantInside | QuadrantOutside)) => "┻",
            (Thick, Joint::Through(Plain | Rounded)) => "┿",
            (Thick, Joint::Through(Thick | Double | QuadrantInside | QuadrantOutside)) => "╋",

            (QuadrantInside, Joint::Out(_) | Joint::Through(_)) => "▟",
            (QuadrantInside, Joint::AltOut(_) | Joint::AltThrough(_)) => "▙",
            (QuadrantInside, Joint::In(_) | Joint::AltIn(_)) => "▄",

            (QuadrantOutside, Joint::In(_) | Joint::Through(_)) => "▛",
            (QuadrantOutside, Joint::AltIn(_) | Joint::AltThrough(_)) => "▜",
            (QuadrantOutside, _) => "▀",

            (_, Joint::Manual(c)) => c,
            (_, Joint::Corner(_, _)) => "⚠",
            (_, Joint::AltIn(_)) => "⚠",
            (_, Joint::AltOut(_)) => "⚠",
            (_, Joint::AltThrough(_)) => "⚠",
        }
    } else if pos = Positional::End {
    }
}
