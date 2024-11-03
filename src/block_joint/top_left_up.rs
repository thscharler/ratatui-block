use crate::block_joint::Joint;
use ratatui::widgets::BorderType;

pub(crate) fn top_left_up_joint(border: BorderType, joint: Joint) -> &'static str {
    use ratatui::widgets::BorderType::*;

    match (border, joint) {
        (Plain | Rounded, Joint::In(_)) => "┌",
        (
            Plain | Rounded,
            Joint::Out(Plain | Rounded | Double | QuadrantInside | QuadrantOutside)
            | Joint::Through(Plain | Rounded | Double | QuadrantInside | QuadrantOutside),
        ) => "├",
        (Plain | Rounded, Joint::Out(Thick) | Joint::Through(Thick)) => "┞",

        (Double, Joint::In(_)) => "╔",
        (Double, Joint::Out(_) | Joint::Through(_)) => "╠",

        (Thick, Joint::In(_)) => "┏",
        (
            Thick,
            Joint::Out(Plain | Rounded | Double) | Joint::Through(Plain | Rounded | Double),
        ) => "┢",
        (
            Thick,
            Joint::Out(Thick | QuadrantInside | QuadrantOutside)
            | Joint::Through(Thick | QuadrantInside | QuadrantOutside),
        ) => "┣",

        (QuadrantInside, Joint::Out(QuadrantInside)) => "▐",
        (QuadrantInside, Joint::AltOut(QuadrantInside)) => "▚",
        (QuadrantInside, _) => "▐",

        (QuadrantOutside, _) => "▛",

        (_, Joint::Manual(c)) => c,
        (_, Joint::Corner(_, _)) => "⚠",

        (_, Joint::AltIn(_)) => "⚠",
        (_, Joint::AltOut(_)) => "⚠",
        (_, Joint::AltThrough(_)) => "⚠",
    }
}
