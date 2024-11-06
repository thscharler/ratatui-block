use crate::{Joint, JointKind, JointPosition, JointSide, NewBlock};
use ratatui::layout::Rect;
use ratatui::widgets::{Block, BorderType};

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
pub fn create_border(layout: &[Rect], borders: &[BorderType], n: usize) -> NewBlock<'static> {
    let own_border = borders[n];
    let area = layout[n];
    let area_x1 = area.x;
    let area_y1 = area.y;
    let area_x2 = area.right().saturating_sub(1);
    let area_y2 = area.bottom().saturating_sub(1);

    let mut block = NewBlock {
        block: Block::bordered().border_type(own_border),
        border_type: own_border,
        joints: vec![],
    };

    for (i, test) in layout.iter().enumerate() {
        let other_border = borders[i];

        let test_x1 = test.x;
        let test_y1 = test.y;
        let test_x2 = test.right().saturating_sub(1);
        let test_y2 = test.bottom().saturating_sub(1);

        // test above
        if test_y2 == area_y1 {
            if test_x1 == area_x2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Top,
                    kind: JointKind::Outward,
                    mirrored: false,
                    pos: JointPosition::CrossEnd(other_border),
                });
            } else if test_x1 >= area_x1 && test_x1 <= area_x2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Top,
                    kind: JointKind::Outward,
                    mirrored: false,
                    pos: JointPosition::Pos(test_x1 - area_x1),
                });
            } else if test_x1 < area_x1 && test_x2 > area_x1 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Top,
                    kind: JointKind::Outward,
                    mirrored: false,
                    pos: JointPosition::ProlongStart,
                });
            }

            if test_x2 == area_x1 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Top,
                    kind: JointKind::Outward,
                    mirrored: false,
                    pos: JointPosition::CrossStart(other_border),
                });
            } else if test_x2 >= area_x1 && test_x2 <= area_x2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Top,
                    kind: JointKind::Outward,
                    mirrored: true,
                    pos: JointPosition::Pos(test_x2 - area_x1),
                });
            } else if test_x2 > area_x2 && test_x1 < area_x2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Top,
                    kind: JointKind::Outward,
                    mirrored: true,
                    pos: JointPosition::ProlongEnd,
                });
            }

            if other_border == BorderType::QuadrantInside {
                add_top_quadrant_inside(
                    test_x1,
                    test_x2,
                    area_x1,
                    area_x2,
                    other_border,
                    &mut block,
                );
            }
        }

        // test below
        if test_y1 == area_y2 {
            if test_x1 == area_x2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Bottom,
                    kind: JointKind::Outward,
                    mirrored: false,
                    pos: JointPosition::CrossEnd(other_border),
                });
            } else if test_x1 >= area_x1 && test_x1 <= area_x2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Bottom,
                    kind: JointKind::Outward,
                    mirrored: false,
                    pos: JointPosition::Pos(test_x1 - area_x1),
                });
            } else if test_x1 < area_x1 && test_x2 > area_x1 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Bottom,
                    kind: JointKind::Outward,
                    mirrored: false,
                    pos: JointPosition::ProlongStart,
                });
            }

            if test_x2 == area_x1 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Bottom,
                    kind: JointKind::Outward,
                    mirrored: false,
                    pos: JointPosition::CrossStart(other_border),
                });
            } else if test_x2 >= area_x1 && test_x2 <= area_x2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Bottom,
                    kind: JointKind::Outward,
                    mirrored: true,
                    pos: JointPosition::Pos(test_x2 - area_x1),
                });
            } else if test_x2 > area_x2 && test_x1 < area_x2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Bottom,
                    kind: JointKind::Outward,
                    mirrored: true,
                    pos: JointPosition::ProlongEnd,
                });
            }

            if other_border == BorderType::QuadrantInside {
                add_bottom_quadrant_inside(
                    test_x1,
                    test_x2,
                    area_x1,
                    area_x2,
                    other_border,
                    &mut block,
                );
            }
        }

        // test left
        if test_x2 == area_x1 {
            if test_y1 == area_y2 {
                // already added as Bottom/StartCross
            } else if test_y1 >= area_y1 && test_y1 <= area_y2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Left,
                    kind: JointKind::Outward,
                    mirrored: false,
                    pos: JointPosition::Pos(test_y1 - area_y1),
                });
            } else if test_y1 < area_y1 && test_y2 > area_y1 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Left,
                    kind: JointKind::Outward,
                    mirrored: false,
                    pos: JointPosition::ProlongStart,
                });
            }

            if test_y2 == area_y1 {
                // already added as Top/StartCross
            } else if test_y2 >= area_y1 && test_y2 <= area_y2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Left,
                    kind: JointKind::Outward,
                    mirrored: true,
                    pos: JointPosition::Pos(test_y2 - area_y1),
                });
            } else if test_y2 > area_y2 && test_y1 < area_y2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Left,
                    kind: JointKind::Outward,
                    mirrored: true,
                    pos: JointPosition::ProlongEnd,
                });
            }

            if other_border == BorderType::QuadrantInside {
                add_left_quadrant_inside(
                    test_y1,
                    test_y2,
                    area_y1,
                    area_y2,
                    other_border,
                    &mut block,
                );
            }
        }

        // test right
        if test_x1 == area_x2 {
            if test_y1 == area_y2 {
                // already added as Bottom/EndCross
            } else if test_y1 >= area_y1 && test_y1 <= area_y2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Right,
                    kind: JointKind::Outward,
                    mirrored: false,
                    pos: JointPosition::Pos(test_y1 - area_y1),
                });
            } else if test_y1 < area_y1 && test_y2 > area_y1 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Right,
                    kind: JointKind::Outward,
                    mirrored: false,
                    pos: JointPosition::ProlongStart,
                });
            }

            if test_y2 == area_y1 {
                // already added as Top/EndCross
            } else if test_y2 >= area_y1 && test_y2 <= area_y2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Right,
                    kind: JointKind::Outward,
                    mirrored: true,
                    pos: JointPosition::Pos(test_y2 - area_y1),
                });
            } else if test_y2 > area_y2 && test_y1 < area_y2 {
                block.joints.push(Joint {
                    own_border,
                    other_border,
                    side: JointSide::Right,
                    kind: JointKind::Outward,
                    mirrored: true,
                    pos: JointPosition::ProlongEnd,
                });
            }

            if other_border == BorderType::QuadrantInside {
                add_right_quadrant_inside(
                    test_y1,
                    test_y2,
                    area_y1,
                    area_y2,
                    other_border,
                    &mut block,
                );
            }
        }
    }

    block
}

fn add_top_quadrant_inside(
    test_x1: u16,
    test_x2: u16,
    area_x1: u16,
    area_x2: u16,
    border: BorderType,
    block: &mut NewBlock,
) {
    if test_x1 == area_x1 {
        // block.joints.push(
        //     Joint::new(JointSide::Top, JointPosition::Pos(test_x1 - area_x1))
        //         .border(border)
        //         .other(border)
        //         .manual("▐"),
        // )
    } else if test_x1 > area_x1 && test_x2 < area_x2 {
        // block.joints.push(
        //     Joint::new(JointSide::Top, JointPosition::Pos(test_x1 - area_x1))
        //         .border(border)
        //         .other(border)
        //         .manual("▟"),
        // );
    } else if test_x1 == area_x2 {
        // block.joints.push(
        //     Joint::new(JointSide::Top, JointPosition::Pos(test_x1 - area_x1))
        //         .border(border)
        //         .other(border)
        //         .manual("▞"),
        // );
    }

    for x in test_x1 + 1..test_x2 {
        if x == area_x1 {
            // block.joints.push(
            //     Joint::new(JointSide::Top, JointPosition::Pos(x - area_x1))
            //         .border(border)
            //         .other(border)
            //         .manual("▜"),
            // )
        } else if x > area_x1 && x < area_x2 {
            block.joints.push(
                Joint::new(JointSide::Top, JointPosition::Pos(x - area_x1))
                    .border(border)
                    .other(border)
                    .kind(JointKind::Manual("█")),
            );
        } else if x == area_x2 {
            // block.joints.push(
            //     Joint::new(JointSide::Top, JointPosition::Pos(x - area_x1))
            //         .border(border)
            //         .other(border)
            //         .manual("▛"),
            // );
        }
    }
}

fn add_bottom_quadrant_inside(
    test_x1: u16,
    test_x2: u16,
    area_x1: u16,
    area_x2: u16,
    border: BorderType,
    block: &mut NewBlock,
) {
    if test_x1 == area_x1 {
        // block.joints.push(
        //     Joint::new(JointSide::Bottom, JointPosition::Pos(test_x1 - area_x1))
        //         .border(border)
        //         .other(border)
        //         .manual("▐"),
        // )
    } else if test_x1 > area_x1 && test_x2 < area_x2 {
        // block.joints.push(
        //     Joint::new(JointSide::Bottom, JointPosition::Pos(test_x1 - area_x1))
        //         .border(border)
        //         .other(border)
        //         .manual("▜"),
        // );
    } else if test_x1 == area_x2 {
        // block.joints.push(
        //     Joint::new(JointSide::Bottom, JointPosition::Pos(test_x1 - area_x1))
        //         .border(border)
        //         .other(border)
        //         .manual("▚"),
        // );
    }

    for x in test_x1 + 1..test_x2 {
        if x == area_x1 {
            // block.joints.push(
            //     Joint::new(JointSide::Bottom, JointPosition::Pos(x - area_x1))
            //         .border(border)
            //         .other(border)
            //         .manual("▟"),
            // )
        } else if x > area_x1 && x < area_x2 {
            block.joints.push(
                Joint::new(JointSide::Bottom, JointPosition::Pos(x - area_x1))
                    .border(border)
                    .other(border)
                    .kind(JointKind::Manual("█")),
            );
        } else if x == area_x2 {
            // block.joints.push(
            //     Joint::new(JointSide::Bottom, JointPosition::Pos(x - area_x1))
            //         .border(border)
            //         .other(border)
            //         .manual("▙"),
            // );
        }
    }
}

fn add_left_quadrant_inside(
    test_y1: u16,
    test_y2: u16,
    area_y1: u16,
    area_y2: u16,
    border: BorderType,
    block: &mut NewBlock,
) {
    if test_y1 == area_y1 {
        // block.joints.push(
        //     Joint::new(JointSide::Left, JointPosition::Pos(test_y1 - area_y1))
        //         .border(border)
        //         .other(border)
        //         .manual("▄"),
        // )
    } else if test_y1 > area_y1 && test_y2 < area_y2 {
        // no special handling necessary.
    } else if test_y1 == area_y2 {
        // block.joints.push(
        //     Joint::new(JointSide::Left, JointPosition::Pos(test_y1 - area_y1))
        //         .border(border)
        //         .other(border)
        //         .manual("▞"),
        // );
    }

    for x in test_y1 + 1..test_y2 {
        if x == area_y1 {
            // block.joints.push(
            //     Joint::new(JointSide::Left, JointPosition::Pos(x - area_y1))
            //         .border(border)
            //         .other(border)
            //         .manual("▙"),
            // )
        } else if x > area_y1 && x < area_y2 {
            block.joints.push(
                Joint::new(JointSide::Left, JointPosition::Pos(x - area_y1))
                    .border(border)
                    .other(border)
                    .kind(JointKind::Manual("█")),
            );
        } else if x == area_y2 {
            // block.joints.push(
            //     Joint::new(JointSide::Left, JointPosition::Pos(x - area_y1))
            //         .border(border)
            //         .other(border)
            //         .manual("▛"),
            // );
        }
    }
}

fn add_right_quadrant_inside(
    test_y1: u16,
    test_y2: u16,
    area_y1: u16,
    area_y2: u16,
    border: BorderType,
    block: &mut NewBlock,
) {
    if test_y1 == area_y1 {
        // block.joints.push(
        //     Joint::new(JointSide::Right, JointPosition::Pos(test_y1 - area_y1))
        //         .border(border)
        //         .other(border)
        //         .manual("▄"),
        // )
    } else if test_y1 > area_y1 && test_y2 < area_y2 {
        // no special handling necessary.
    } else if test_y1 == area_y2 {
        // block.joints.push(
        //     Joint::new(JointSide::Right, JointPosition::Pos(test_y1 - area_y1))
        //         .border(border)
        //         .other(border)
        //         .manual("▚"),
        // );
    }

    for x in test_y1 + 1..test_y2 {
        if x == area_y1 {
            // block.joints.push(
            //     Joint::new(JointSide::Right, JointPosition::Pos(x - area_y1))
            //         .border(border)
            //         .other(border)
            //         .manual("▟"),
            // )
        } else if x > area_y1 && x < area_y2 {
            block.joints.push(
                Joint::new(JointSide::Right, JointPosition::Pos(x - area_y1))
                    .border(border)
                    .other(border)
                    .kind(JointKind::Manual("█")),
            );
        } else if x == area_y2 {
            // block.joints.push(
            //     Joint::new(JointSide::Right, JointPosition::Pos(x - area_y1))
            //         .border(border)
            //         .other(border)
            //         .manual("▜"),
            // );
        }
    }
}
