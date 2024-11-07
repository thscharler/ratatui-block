use crate::v3::{BlockBorder, BorderSymbol, Kind, Side};
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

    let mut block = BlockBorder {
        border_style: Default::default(),
        own_border,
        debug: "".to_string(),
        symbols: vec![],
    };

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
            create_one_side(
                &mut block,
                x1,
                x2,
                area_x1,
                area_x2,
                other_border,
                Side::Top,
            );
        }
        if y1 == area_y2 {

        }
    }

    block
}

fn create_one_side(
    block: &mut BlockBorder,
    x1: u16,
    x2: u16,
    area_x1: u16,
    area_x2: u16,
    other_border: BorderType,
    side: Side,
) {
    if x1 < area_x1 && x2 < area_x1 {
        block.debug = "before left".into();
        block.symbols.push((
            BorderSymbol::new(side, Kind::RegularStart, other_border),
            0,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Regular, other_border),
            1,
            (area_x2 - area_x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::RegularEnd, other_border),
            area_x2 - area_x1,
            1,
        ));
    } else if x1 < area_x1 && x2 == area_x1 {
        block.debug = "first contact".into();
        block.symbols.push((
            BorderSymbol::new(side, Kind::CrossStart(other_border), other_border),
            0,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Regular, other_border),
            1,
            (area_x2 - area_x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::RegularEnd, other_border),
            area_x2 - area_x1,
            1,
        ));
    } else if x1 < area_x1 && x2 < area_x2 {
        block.debug = "left overhang".into();
        block.symbols.push((
            BorderSymbol::new(side, Kind::ProlongStart, other_border),
            0,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Overlap, other_border),
            1,
            (x2 - area_x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::AngleOutwardEnd, other_border),
            x2 - area_x1,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Regular, other_border),
            x2 - area_x1 + 1,
            (area_x2 - x2).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::RegularEnd, other_border),
            area_x2 - area_x1,
            1,
        ));
    } else if x1 < area_x1 && x2 == area_x2 {
        block.debug = "right full contact".into();
        block.symbols.push((
            BorderSymbol::new(side, Kind::ProlongStart, other_border),
            0,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Overlap, other_border),
            1,
            (x2 - area_x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::AngleEndEnd, other_border),
            (area_x2 - area_x1),
            1,
        ));
    } else if x1 < area_x1 && x2 > area_x2 {
        block.debug = "both overhang".into();
        block.symbols.push((
            BorderSymbol::new(side, Kind::ProlongStart, other_border),
            0,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Overlap, other_border),
            1,
            (area_x2 - area_x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::ProlongEnd, other_border),
            (area_x2 - area_x1),
            1,
        ));
    } else if x1 == area_x1 && x2 < area_x2 {
        block.debug = "touch left".into();
        block.symbols.push((
            BorderSymbol::new(side, Kind::AngleStartStart, other_border),
            0,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Overlap, other_border),
            1,
            (x2 - area_x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::AngleOutwardEnd, other_border),
            x2 - area_x1,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Regular, other_border),
            x2 - area_x1 + 1,
            (area_x2 - x2).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::RegularEnd, other_border),
            x2,
            1,
        ));
    } else if x1 == area_x1 && x2 == area_x2 {
        block.debug = "full overlap".into();
        block.symbols.push((
            BorderSymbol::new(side, Kind::AngleStartStart, other_border),
            0,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Overlap, other_border),
            1,
            (x2 - area_x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::AngleEndEnd, other_border),
            area_x2 - area_x1,
            1,
        ));
    } else if x1 == area_x1 && x2 > area_x2 {
        block.debug = "left full contact".into();
        block.symbols.push((
            BorderSymbol::new(side, Kind::AngleStartStart, other_border),
            0,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Overlap, other_border),
            1,
            (area_x2 - area_x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::ProlongEnd, other_border),
            area_x2 - area_x1,
            1,
        ));
    } else if x1 < area_x2 && x2 < area_x2 {
        block.debug = "middling contact".into();
        block.symbols.push((
            BorderSymbol::new(side, Kind::RegularStart, other_border),
            0,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Regular, other_border),
            1,
            (x1 - area_x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::AngleOutwardStart, other_border),
            x1 - area_x1,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Overlap, other_border),
            x1 - area_x1 + 1,
            x2 - x1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::AngleOutwardEnd, other_border),
            x2 - area_x1,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Regular, other_border),
            x2 - area_x1 + 1,
            (area_x2 - x2).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::RegularEnd, other_border),
            area_x2 - area_x1,
            1,
        ));
    } else if x1 < area_x2 && x2 == area_x2 {
        block.debug = "touch right".into();
        block.symbols.push((
            BorderSymbol::new(side, Kind::RegularStart, other_border),
            0,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Regular, other_border),
            1,
            (x1 - area_x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::AngleOutwardStart, other_border),
            x1 - area_x1,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Overlap, other_border),
            x1 - area_x1 + 1,
            x2 - x1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::AngleEndEnd, other_border),
            x2 - area_x1,
            1,
        ));
    } else if x1 < area_x2 && x2 > area_x2 {
        block.debug = "right overhang".into();
        block.symbols.push((
            BorderSymbol::new(side, Kind::RegularStart, other_border),
            0,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Regular, other_border),
            1,
            (x1 - area_x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::AngleOutwardStart, other_border),
            x1 - area_x1,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Overlap, other_border),
            x1 - area_x1 + 1,
            (area_x2 - x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::ProlongEnd, other_border),
            area_x2 - area_x1,
            1,
        ));
    } else if x1 == area_x2 && x2 > area_x2 {
        block.debug = "last contact".into();
        block.symbols.push((
            BorderSymbol::new(side, Kind::RegularStart, other_border),
            0,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Regular, other_border),
            1,
            (x1 - area_x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::CrossEnd(other_border), other_border),
            area_x2 - area_x1,
            1,
        ));
    } else {
        block.debug = "beyond right".into();
        block.symbols.push((
            BorderSymbol::new(side, Kind::RegularStart, other_border),
            0,
            1,
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::Regular, other_border),
            1,
            (area_x2 - area_x1).saturating_sub(1),
        ));
        block.symbols.push((
            BorderSymbol::new(side, Kind::RegularEnd, other_border),
            area_x2 - area_x1,
            1,
        ));
    }
}
