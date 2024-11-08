use crate::v4::{BlockBorder, BorderSymbol, Side};
use ratatui::layout::Rect;
use ratatui::widgets::BorderType;

pub fn create_border(layout: &[Rect], borders: &[BorderType], n: usize) -> BlockBorder {
    let own_border = borders[n];
    let area = layout[n];
    let area_x1 = area.x;
    let area_y1 = area.y;
    let area_x2 = area.x + area.width.saturating_sub(1);
    let area_y2 = area.y + area.height.saturating_sub(1);

    assert!(area_x1 <= area_x2);
    assert!(area_y1 <= area_y2);

    let mut block = BlockBorder::with_area(own_border, area);

    for (i, test) in layout.iter().enumerate() {
        let other_border = borders[i];

        let x1 = test.x;
        let y1 = test.y;
        let x2 = test.x + test.width.saturating_sub(1);
        let y2 = test.y + test.height.saturating_sub(1);

        assert!(x1 <= x2);
        assert!(y1 <= y2);

        // test above
        if y2 == area_y1 {
            create_horizontal_side(
                &mut block.top_left,
                block.top.as_mut_slice(),
                &mut block.top_right,
                x1 as usize,
                x2 as usize,
                area_x1 as usize,
                area_x2 as usize,
                Side::Bottom,
                Side::Left,
                other_border,
            );
        }
        // test below
        if y1 == area_y2 {
            create_horizontal_side(
                &mut block.bottom_left,
                block.bottom.as_mut_slice(),
                &mut block.bottom_right,
                x1 as usize,
                x2 as usize,
                area_x1 as usize,
                area_x2 as usize,
                Side::Top,
                Side::Left,
                other_border,
            )
        }
        // test left
        if x2 == area_x1 {
            create_vertical_side(
                &mut block.top_left,
                block.left.as_mut_slice(),
                &mut block.bottom_left,
                y1 as usize,
                y2 as usize,
                area_y1 as usize,
                area_y2 as usize,
                Side::Right,
                Side::Top,
                other_border,
            )
        } // test right
        if x1 == area_x2 {
            create_vertical_side(
                &mut block.top_right,
                block.right.as_mut_slice(),
                &mut block.bottom_right,
                y1 as usize,
                y2 as usize,
                area_y1 as usize,
                area_y2 as usize,
                Side::Left,
                Side::Top,
                other_border,
            )
        }
    }

    block
}

fn create_horizontal_side(
    start_corner: &mut BorderSymbol,
    block: &mut [BorderSymbol],
    end_corner: &mut BorderSymbol,
    p1: usize,
    p2: usize,
    area_p1: usize,
    area_p2: usize,
    parallel_side: Side,
    start_side: Side,
    other_border: BorderType,
) {
    if p1 < area_p1 && p2 < area_p1 {
        // left out
    } else if p1 < area_p1 && p2 == area_p1 {
        // corner/corner
        start_corner.join_outward(start_side.opposite(), other_border);
        start_corner.prolong(parallel_side, other_border);
    } else if p1 < area_p1 && p2 < area_p2 {
        // left overhanging
        start_corner.prolong(parallel_side, other_border);
        for i in 0..(p2 - area_p1) {
            block[i].overlap(parallel_side, other_border);
        }
        block[p2 - area_p1 - 1].join_outward(start_side.opposite(), other_border);
    } else if p1 < area_p1 && p2 == area_p2 {
        // right corner/right corner, overhanging to the left.
        start_corner.prolong(parallel_side, other_border);
        for i in 0..area_p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        end_corner.join_outward(start_side.opposite(), other_border);
    } else if p1 < area_p1 && p2 > area_p2 {
        // overhang on both sides
        start_corner.prolong(parallel_side, other_border);
        for i in 0..area_p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        end_corner.prolong(parallel_side, other_border);
    } else if p1 == area_p1 && p2 < area_p2 {
        // left corner/left corner, ends inside
        start_corner.join_outward(start_side, other_border);
        for i in 0..p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        block[p2 - area_p1 - 1].join_outward(start_side.opposite(), other_border);
    } else if p1 == area_p1 && p2 == area_p2 {
        // full overlap
        start_corner.join_outward(start_side, other_border);
        for i in 0..area_p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        end_corner.join_outward(start_side.opposite(), other_border);
    } else if p1 == area_p1 && p2 > area_p2 {
        // left corner/left corner, overhanging to the right.
        start_corner.join_outward(start_side, other_border);
        for i in 0..area_p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        end_corner.prolong(parallel_side, other_border);
    } else if p1 < area_p2 && p2 < area_p2 {
        // partial overlap
        block[p1 - area_p1 - 1].join_outward(start_side, other_border);
        for i in p1 - area_p1..p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        block[p2 - area_p1 - 1].join_outward(start_side.opposite(), other_border);
    } else if p1 < area_p2 && p2 == area_p2 {
        // start inside, right corner/right corner.
        block[p1 - area_p1 - 1].join_outward(start_side, other_border);
        for i in p1 - area_p1..area_p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        end_corner.join_outward(start_side.opposite(), other_border);
    } else if p1 < area_p2 && p2 > area_p2 {
        // start inside, overhang to the right.
        block[p1 - area_p1 - 1].join_outward(start_side, other_border);
        for i in p1 - area_p1..area_p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        end_corner.prolong(parallel_side, other_border);
    } else if p1 == area_p2 && p2 > area_p2 {
        // left corner/right corner
        end_corner.join_outward(start_side, other_border);
        end_corner.prolong(parallel_side, other_border);
    } else {
        // right out
    }
}

fn create_vertical_side(
    start_corner: &mut BorderSymbol,
    block: &mut [BorderSymbol],
    end_corner: &mut BorderSymbol,
    p1: usize,
    p2: usize,
    area_p1: usize,
    area_p2: usize,
    parallel_side: Side,
    start_side: Side,
    other_border: BorderType,
) {
    if p1 < area_p1 && p2 < area_p1 {
        // left out
    } else if p1 < area_p1 && p2 == area_p1 {
        // corner/corner
        start_corner.prolong(start_side.opposite(), other_border);
        start_corner.join_outward(parallel_side, other_border);
    } else if p1 < area_p1 && p2 < area_p2 {
        // left overhanging
        start_corner.join_outward(parallel_side, other_border);
        for i in 0..(p2 - area_p1) {
            block[i].overlap(parallel_side, other_border);
        }
        block[p2 - area_p1 - 1].join_outward(start_side.opposite(), other_border);
    } else if p1 < area_p1 && p2 == area_p2 {
        // right corner/right corner, overhanging to the left.
        start_corner.join_outward(parallel_side, other_border);
        for i in 0..area_p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        end_corner.prolong(start_side.opposite(), other_border);
    } else if p1 < area_p1 && p2 > area_p2 {
        // overhang on both sides
        start_corner.join_outward(parallel_side, other_border);
        for i in 0..area_p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        end_corner.join_outward(parallel_side, other_border);
    } else if p1 == area_p1 && p2 < area_p2 {
        // left corner/left corner, ends inside
        start_corner.prolong(start_side, other_border);
        for i in 0..p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        block[p2 - area_p1 - 1].join_outward(start_side.opposite(), other_border);
    } else if p1 == area_p1 && p2 == area_p2 {
        // full overlap
        start_corner.prolong(start_side, other_border);
        for i in 0..area_p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        end_corner.prolong(start_side.opposite(), other_border);
    } else if p1 == area_p1 && p2 > area_p2 {
        // left corner/left corner, overhanging to the right.
        start_corner.prolong(start_side, other_border);
        for i in 0..area_p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        end_corner.join_outward(parallel_side, other_border);
    } else if p1 < area_p2 && p2 < area_p2 {
        // partial overlap
        block[p1 - area_p1 - 1].join_outward(start_side, other_border);
        for i in p1 - area_p1..p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        block[p2 - area_p1 - 1].join_outward(start_side.opposite(), other_border);
    } else if p1 < area_p2 && p2 == area_p2 {
        // start inside, right corner/right corner.
        block[p1 - area_p1 - 1].join_outward(start_side, other_border);
        for i in p1 - area_p1..area_p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        end_corner.prolong(start_side.opposite(), other_border);
    } else if p1 < area_p2 && p2 > area_p2 {
        // start inside, overhang to the right.
        block[p1 - area_p1 - 1].join_outward(start_side, other_border);
        for i in p1 - area_p1..area_p2 - area_p1 - 1 {
            block[i].overlap(parallel_side, other_border);
        }
        end_corner.join_outward(parallel_side, other_border);
    } else if p1 == area_p2 && p2 > area_p2 {
        // left corner/right corner
        end_corner.prolong(start_side, other_border);
        end_corner.join_outward(parallel_side, other_border);
    } else {
        // right out
    }
}
