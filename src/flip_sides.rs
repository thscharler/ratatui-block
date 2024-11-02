use crate::block_joint::Joint;
use ratatui::widgets::BorderType;

// Switch between top/bottom, left/right
// The sides are mirrored.
pub(crate) fn flip_sides(border: BorderType, kind: Joint) -> (BorderType, Joint) {
    use ratatui::widgets::BorderType::*;

    (
        match border {
            Plain => Plain,
            Rounded => Rounded,
            Double => Double,
            Thick => Thick,
            QuadrantInside => QuadrantOutside,
            QuadrantOutside => QuadrantInside,
        },
        match kind {
            Joint::Out(v) => Joint::In(v),
            Joint::In(v) => Joint::Out(v),
            Joint::Through(v) => Joint::Through(v),
            Joint::Manual(v) => Joint::Manual(v),
        },
    )
}
