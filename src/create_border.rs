use crate::{Joint, JointSide, NewBlock};
use ratatui::layout::Rect;
use ratatui::widgets::{Block, BorderType};
use std::rc::Rc;

/// Create one border for the given layout.
///
/// __Assumptions__
///
/// * All areas are rendered using the same border_type.
///
///         This is just for simplification of the code here.
///         Might be lifted in a future version, or with a parallel impl.
///
/// * The requested area n is the last one that will be rendered.
///
///         This might render a few borders doubly, but this way
///         the actual order of rendering becomes mote.
///
/// * The requested area will connect to all adjacent areas.
///
///         This is a necessary constraint, and can be easily
///         avoided by creating the relevant subset before calling
///         the fn.
///
pub fn create_border(layout: Rc<[Rect]>, n: usize, border_type: BorderType) -> NewBlock<'static> {
    let area = layout[n];
    let area_x1 = area.x;
    let area_y1 = area.y;
    let area_x2 = area.right().saturating_sub(1);
    let area_y2 = area.bottom().saturating_sub(1);

    let mut block = NewBlock {
        block: Block::bordered().border_type(border_type),
        joints: vec![],
    };

    for test in layout.iter() {
        let test_x1 = test.x;
        let test_y1 = test.y;
        let test_x2 = test.right().saturating_sub(1);
        let test_y2 = test.bottom().saturating_sub(1);

        // test above
        if test_y2 == area_y1 {
            if test_x1 == area_x2 {
                block
                    .joints
                    .push((Joint::Corner(border_type, border_type), JointSide::TopRight));
            } else if test_x1 >= area_x1 && test_x1 <= area_x2 {
                block
                    .joints
                    .push((Joint::Out(border_type), JointSide::Top(test_x1 - area_x1)))
            } else if test_x1 < area_x1 && test_x2 > area_x1 {
                block
                    .joints
                    .push((Joint::Out(border_type), JointSide::Left(0)));
            }

            if test_x2 == area_x1 {
                block
                    .joints
                    .push((Joint::Corner(border_type, border_type), JointSide::TopLeft));
            } else if test_x2 >= area_x1 && test_x2 <= area_x2 {
                block
                    .joints
                    .push((Joint::Out(border_type), JointSide::Top(test_x2 - area_x1)));
            } else if test_x2 > area_x2 && test_x1 < area_x2 {
                block
                    .joints
                    .push((Joint::Out(border_type), JointSide::Right(0)));
            }
        }

        // test below
        if test_y1 == area_y2 {
            if test_x1 == area_x2 {
                block.joints.push((
                    Joint::Corner(border_type, border_type),
                    JointSide::BottomRight,
                ));
            } else if test_x1 >= area_x1 && test_x1 <= area_x2 {
                block.joints.push((
                    Joint::Out(border_type),
                    JointSide::Bottom(test_x1 - area_x1),
                ))
            } else if test_x1 < area_x1 && test_x2 > area_x1 {
                block
                    .joints
                    .push((Joint::Out(border_type), JointSide::Left(area_y2 - area_y1)))
            }

            if test_x2 == area_x1 {
                block.joints.push((
                    Joint::Corner(border_type, border_type),
                    JointSide::BottomLeft,
                ));
            } else if test_x2 >= area_x1 && test_x2 <= area_x2 {
                block.joints.push((
                    Joint::Out(border_type),
                    JointSide::Bottom(test_x2 - area_x1),
                ))
            } else if test_x2 > area_x2 && test_x1 < area_x2 {
                block
                    .joints
                    .push((Joint::Out(border_type), JointSide::Right(area_y2 - area_y1)))
            }
        }

        // test left
        if test_x2 == area_x1 {
            if test_y1 == area_y2 {
                block.joints.push((
                    Joint::Corner(border_type, border_type),
                    JointSide::BottomLeft,
                ));
            } else if test_y1 >= area_y1 && test_y1 <= area_y2 {
                block
                    .joints
                    .push((Joint::Out(border_type), JointSide::Left(test_y1 - area_y1)))
            } else if test_y1 < area_y1 && test_y2 > area_y1 {
                block
                    .joints
                    .push((Joint::Out(border_type), JointSide::Top(0)))
            }

            if test_y2 == area_y1 {
                block
                    .joints
                    .push((Joint::Corner(border_type, border_type), JointSide::TopLeft));
            } else if test_y2 >= area_y1 && test_y2 <= area_y2 {
                block
                    .joints
                    .push((Joint::Out(border_type), JointSide::Left(test_y2 - area_y1)))
            } else if test_y2 > area_y2 && test_y1 < area_y2 {
                block
                    .joints
                    .push((Joint::Out(border_type), JointSide::Bottom(0)))
            }
        }

        // test right
        if test_x1 == area_x2 {
            if test_y1 == area_y2 {
                block.joints.push((
                    Joint::Corner(border_type, border_type),
                    JointSide::BottomRight,
                ));
            } else if test_y1 >= area_y1 && test_y1 <= area_y2 {
                block
                    .joints
                    .push((Joint::Out(border_type), JointSide::Right(test_y1 - area_y1)))
            } else if test_y1 < area_y1 && test_y2 > area_y1 {
                block
                    .joints
                    .push((Joint::Out(border_type), JointSide::Top(area_x2 - area_x1)))
            }

            if test_y2 == area_y1 {
                block
                    .joints
                    .push((Joint::Corner(border_type, border_type), JointSide::TopRight));
            } else if test_y2 >= area_y1 && test_y2 <= area_y2 {
                block
                    .joints
                    .push((Joint::Out(border_type), JointSide::Right(test_y2 - area_y1)))
            } else if test_y2 > area_y2 && test_y1 < area_y2 {
                block.joints.push((
                    Joint::Out(border_type),
                    JointSide::Bottom(area_x2 - area_x1),
                ))
            }
        }
    }

    block
}
