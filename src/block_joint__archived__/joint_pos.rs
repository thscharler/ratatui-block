// use crate::block_joint__archived__::JointPosition;
// use log::debug;
// use ratatui::layout::Rect;
//
// pub(crate) fn horizontal_joint_pos(pos: JointPosition, area: Rect) -> Option<u16> {
//     let p = match pos {
//         JointPosition::FromStart(n) => n,
//         JointPosition::FromEnd(n) => area.width.saturating_sub(n),
//         JointPosition::AtPos(n) => n,
//     };
//
//     debug!("hpos {:?} {:?} -> {:?}", pos, area, p);
//
//     if p < area.width {
//         Some(p)
//     } else {
//         None
//     }
// }
//
// pub(crate) fn vertical_joint_pos(pos: JointPosition, area: Rect) -> Option<u16> {
//     let p = match pos {
//         JointPosition::FromStart(n) => n,
//         JointPosition::FromEnd(n) => area.height.saturating_sub(n),
//         JointPosition::AtPos(n) => n,
//     };
//
//     if p < area.height {
//         Some(p)
//     } else {
//         None
//     }
// }
