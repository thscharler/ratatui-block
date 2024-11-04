use crate::{CrossJoint, Joint, JointCorner, JointPos, JointScale, JointSide, NewBlock};
use ratatui::layout::Rect;
use ratatui::widgets::{Block, BorderType};
use std::rc::Rc;

pub fn create_border(layout: Rc<[Rect]>, n: usize, border_type: BorderType) -> NewBlock<'static> {
    let area = layout[n];
    let area_x1 = area.x;
    let area_y1 = area.y;
    let area_x2 = area.right().saturating_sub(1);
    let area_y2 = area.bottom().saturating_sub(1);

    let mut block = NewBlock {
        block: Block::bordered().border_type(border_type),
        joints: vec![],
        cross: vec![],
    };

    for test in layout.iter() {
        let test_x1 = test.x;
        let test_y1 = test.y;
        let test_x2 = test.right().saturating_sub(1);
        let test_y2 = test.bottom().saturating_sub(1);

        // test above
        if test_y2 == area_y1 {
            if test_x1 == area_x2 {
                block.cross.push(CrossJoint {
                    join_border: border_type,
                    join_border2: border_type,
                    corner: JointCorner::TopRightCross,
                });
            } else if test_x1 >= area_x1 && test_x1 <= area_x2 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Top,
                    scale: JointScale::Out,
                    mirrored: false,
                    pos: JointPos::Pos(test_x1 - area_x1),
                });
            } else if test_x1 < area_x1 && test_x2 > area_x1 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Top,
                    scale: JointScale::Out,
                    mirrored: false,
                    pos: JointPos::ProlongStart,
                });
            }

            if test_x2 == area_x1 {
                block.cross.push(CrossJoint {
                    join_border: border_type,
                    join_border2: border_type,
                    corner: JointCorner::TopLeftCross,
                });
            } else if test_x2 >= area_x1 && test_x2 <= area_x2 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Top,
                    scale: JointScale::Out,
                    mirrored: true,
                    pos: JointPos::Pos(test_x2 - area_x1),
                });
            } else if test_x2 > area_x2 && test_x1 < area_x2 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Top,
                    scale: JointScale::Out,
                    mirrored: true,
                    pos: JointPos::ProlongEnd,
                });
            }
        }

        // test below
        if test_y1 == area_y2 {
            if test_x1 == area_x2 {
                block.cross.push(CrossJoint {
                    join_border: border_type,
                    join_border2: border_type,
                    corner: JointCorner::BottomRightCross,
                });
            } else if test_x1 >= area_x1 && test_x1 <= area_x2 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Bottom,
                    scale: JointScale::Out,
                    mirrored: false,
                    pos: JointPos::Pos(test_x1 - area_x1),
                });
            } else if test_x1 < area_x1 && test_x2 > area_x1 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Bottom,
                    scale: JointScale::Out,
                    mirrored: false,
                    pos: JointPos::ProlongStart,
                });
            }

            if test_x2 == area_x1 {
                block.cross.push(CrossJoint {
                    join_border: border_type,
                    join_border2: border_type,
                    corner: JointCorner::BottomLeftCross,
                });
            } else if test_x2 >= area_x1 && test_x2 <= area_x2 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Bottom,
                    scale: JointScale::Out,
                    mirrored: true,
                    pos: JointPos::Pos(test_x2 - area_x1),
                });
            } else if test_x2 > area_x2 && test_x1 < area_x2 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Bottom,
                    scale: JointScale::Out,
                    mirrored: true,
                    pos: JointPos::ProlongEnd,
                });
            }
        }

        // test left
        if test_x2 == area_x1 {
            if test_y1 == area_y2 {
                block.cross.push(CrossJoint {
                    join_border: border_type,
                    join_border2: border_type,
                    corner: JointCorner::BottomLeftCross,
                });
            } else if test_y1 >= area_y1 && test_y1 <= area_y2 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Left,
                    scale: JointScale::Out,
                    mirrored: false,
                    pos: JointPos::Pos(test_y1 - area_y1),
                });
            } else if test_y1 < area_y1 && test_y2 > area_y1 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Left,
                    scale: JointScale::Out,
                    mirrored: false,
                    pos: JointPos::ProlongStart,
                });
            }

            if test_y2 == area_y1 {
                block.cross.push(CrossJoint {
                    join_border: border_type,
                    join_border2: border_type,
                    corner: JointCorner::TopLeftCross,
                });
            } else if test_y2 >= area_y1 && test_y2 <= area_y2 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Left,
                    scale: JointScale::Out,
                    mirrored: true,
                    pos: JointPos::Pos(test_y2 - area_y1),
                });
            } else if test_y2 > area_y2 && test_y1 < area_y2 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Left,
                    scale: JointScale::Out,
                    mirrored: true,
                    pos: JointPos::ProlongEnd,
                });
            }
        }

        // test right
        if test_x1 == area_x2 {
            if test_y1 == area_y2 {
                block.cross.push(CrossJoint {
                    join_border: border_type,
                    join_border2: border_type,
                    corner: JointCorner::BottomRightCross,
                });
            } else if test_y1 >= area_y1 && test_y1 <= area_y2 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Right,
                    scale: JointScale::Out,
                    mirrored: false,
                    pos: JointPos::Pos(test_y1 - area_y1),
                });
            } else if test_y1 < area_y1 && test_y2 > area_y1 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Right,
                    scale: JointScale::Out,
                    mirrored: false,
                    pos: JointPos::ProlongStart,
                });
            }

            if test_y2 == area_y1 {
                block.cross.push(CrossJoint {
                    join_border: border_type,
                    join_border2: border_type,
                    corner: JointCorner::TopRightCross,
                });
            } else if test_y2 >= area_y1 && test_y2 <= area_y2 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Right,
                    scale: JointScale::Out,
                    mirrored: true,
                    pos: JointPos::Pos(test_y2 - area_y1),
                });
            } else if test_y2 > area_y2 && test_y1 < area_y2 {
                block.joints.push(Joint {
                    border: border_type,
                    side: JointSide::Right,
                    scale: JointScale::Out,
                    mirrored: true,
                    pos: JointPos::ProlongEnd,
                });
            }
        }
    }

    block
}
