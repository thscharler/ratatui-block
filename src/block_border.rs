use crate::border_symbols::{symbol_set, PlainSymbolSet};
use crate::{BorderSymbol, BorderSymbolSet, Side};
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::prelude::{Style, Widget};
use ratatui::widgets::BorderType;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

///
/// Border for a Block.
///
/// ![symbol organization](https://raw.githubusercontent.com/thscharler/ratatui-block/refs/heads/master/blockborder.png)
///
#[derive(Clone)]
pub struct BlockBorder {
    border_style: Style,
    symbol_set: Rc<dyn BorderSymbolSet>,

    area: Rect,
    symbols: Vec<BorderSymbol>,
}

impl Debug for BlockBorder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockBorder")
            .field("border_style", &self.border_style)
            .field("symbol_set", &"..dyn..")
            .field("symbols", &self.symbols)
            .finish()
    }
}

impl BlockBorder {
    ///
    /// New block border for the given area.
    ///
    pub fn new(area: Rect) -> Self {
        base_border(area)
    }

    ///
    /// New block border for a Block that is part of some bigger layout.
    ///
    /// Given all the areas of the layout and each border type,
    /// this creates a border that is connected at the edges.
    ///
    /// TODO: if the borders overlap more than just exactly for the border the result is undefined.
    ///
    pub fn from_layout(areas: &[Rect], borders: &[BorderType], select: usize) -> Self {
        connected_border(areas, borders, select)
    }

    ///
    /// Border style for the border.
    ///
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    ///
    /// Sets the border type used.
    ///
    pub fn border_type(mut self, border: BorderType) -> Self {
        self.symbol_set = symbol_set(border);
        self
    }

    ///
    /// Use a BorderSymbolSet.
    ///
    pub fn border_set(mut self, border_set: Rc<dyn BorderSymbolSet>) -> Self {
        self.symbol_set = border_set;
        self
    }

    ///
    /// Area covered by the BlockBorder.
    ///
    pub fn get_area(&self) -> Rect {
        self.area
    }

    /// BorderSymbol for the top-left corner.
    pub fn top_left(&self) -> &BorderSymbol {
        self.split().0
    }

    /// BorderSymbol's for the top border.
    /// Slice with area.width-2 items.
    pub fn top(&self) -> &[BorderSymbol] {
        self.split().1
    }

    /// BorderSymbol for the top-right corner.
    pub fn top_right(&self) -> &BorderSymbol {
        self.split().2
    }

    /// BorderSymbol's for the right border.
    /// Slice with area.height-2 items.
    pub fn right(&self) -> &[BorderSymbol] {
        self.split().3
    }

    /// BorderSymbol for the bottom-right corner.
    pub fn bottom_right(&self) -> &BorderSymbol {
        self.split().4
    }

    /// BorderSymbol's for the bottom border.
    /// Slice with area.width-2 items.
    pub fn bottom(&self) -> &[BorderSymbol] {
        self.split().5
    }

    /// BorderSymbol for the bottom-left corner.
    pub fn bottom_left(&self) -> &BorderSymbol {
        self.split().6
    }

    /// BorderSymbols for the left border.
    /// Slice with area.height-2 items.
    pub fn left(&self) -> &[BorderSymbol] {
        self.split().7
    }

    /// BorderSymbol for the top-left corner.
    pub fn top_left_mut(&mut self) -> &mut BorderSymbol {
        self.split_mut().0
    }

    /// BorderSymbol's for the top border.
    /// Slice with area.width-2 items.
    pub fn top_mut(&mut self) -> &mut [BorderSymbol] {
        self.split_mut().1
    }

    /// BorderSymbol for the top-right corner.
    pub fn top_right_mut(&mut self) -> &mut BorderSymbol {
        self.split_mut().2
    }

    /// BorderSymbol's for the right border.
    /// Slice with area.height-2 items.
    pub fn right_mut(&mut self) -> &mut [BorderSymbol] {
        self.split_mut().3
    }

    /// BorderSymbol for the bottom-right corner.
    pub fn bottom_right_mut(&mut self) -> &mut BorderSymbol {
        self.split_mut().4
    }

    /// BorderSymbol's for the bottom border.
    /// Slice with area.width-2 items.
    pub fn bottom_mut(&mut self) -> &mut [BorderSymbol] {
        self.split_mut().5
    }

    /// BorderSymbol for the bottom-left corner.
    pub fn bottom_left_mut(&mut self) -> &mut BorderSymbol {
        self.split_mut().6
    }

    /// BorderSymbols for the left border.
    /// Slice with area.height-2 items.
    pub fn left_mut(&mut self) -> &mut [BorderSymbol] {
        self.split_mut().7
    }

    /// All BorderSymbols.
    pub fn get_symbols(&self) -> &[BorderSymbol] {
        self.symbols.as_slice()
    }

    ///
    /// Split into border parts.
    ///
    /// (top_left, top, top_right, right, bottom_left, bottom, bottom_right, left)
    ///
    #[inline(always)]
    pub fn split_mut(
        &mut self,
    ) -> (
        &mut BorderSymbol,
        &mut [BorderSymbol],
        &mut BorderSymbol,
        &mut [BorderSymbol],
        &mut BorderSymbol,
        &mut [BorderSymbol],
        &mut BorderSymbol,
        &mut [BorderSymbol],
    ) {
        let (top_left, rest) = self.symbols.split_at_mut(1);
        let (top, rest) = rest.split_at_mut(self.area.width.saturating_sub(2) as usize);
        let (top_right, rest) = rest.split_at_mut(1);
        let (right, rest) = rest.split_at_mut(self.area.height.saturating_sub(2) as usize);
        let (bottom_left, rest) = rest.split_at_mut(1);
        let (bottom, rest) = rest.split_at_mut(self.area.width.saturating_sub(2) as usize);
        let (bottom_right, rest) = rest.split_at_mut(1);
        let (left, rest) = rest.split_at_mut(self.area.height.saturating_sub(2) as usize);

        assert!(rest.is_empty());

        (
            &mut top_left[0],
            top,
            &mut top_right[0],
            right,
            &mut bottom_left[0],
            bottom,
            &mut bottom_right[0],
            left,
        )
    }

    ///
    /// Split into border parts.
    ///
    /// (top_left, top, top_right, right, bottom_left, bottom, bottom_right, left)
    ///
    #[inline(always)]
    pub fn split(
        &self,
    ) -> (
        &BorderSymbol,
        &[BorderSymbol],
        &BorderSymbol,
        &[BorderSymbol],
        &BorderSymbol,
        &[BorderSymbol],
        &BorderSymbol,
        &[BorderSymbol],
    ) {
        let (top_left, rest) = self.symbols.split_at(1);
        let (top, rest) = rest.split_at(self.area.width.saturating_sub(2) as usize);
        let (top_right, rest) = rest.split_at(1);
        let (right, rest) = rest.split_at(self.area.height.saturating_sub(2) as usize);
        let (bottom_left, rest) = rest.split_at(1);
        let (bottom, rest) = rest.split_at(self.area.width.saturating_sub(2) as usize);
        let (bottom_right, rest) = rest.split_at(1);
        let (left, rest) = rest.split_at(self.area.height.saturating_sub(2) as usize);

        assert!(rest.is_empty());

        (
            &top_left[0],
            top,
            &top_right[0],
            right,
            &bottom_left[0],
            bottom,
            &bottom_right[0],
            left,
        )
    }
}

impl Widget for &BlockBorder {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let (
            top_left, //
            top,
            top_right,
            right,
            bottom_left,
            bottom,
            bottom_right,
            left,
        ) = self.split();

        if let Some(cell) = buf.cell_mut(Position::new(
            area.x, //
            area.y,
        )) {
            cell.set_style(self.border_style);
            cell.set_symbol(self.symbol_set.symbol(Side::Top, *top_left));
        }
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x + area.width.saturating_sub(1), //
            area.y,
        )) {
            cell.set_style(self.border_style);
            cell.set_symbol(self.symbol_set.symbol(Side::Top, *top_right));
        }
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x,
            area.y + area.height.saturating_sub(1),
        )) {
            cell.set_style(self.border_style);
            cell.set_symbol(self.symbol_set.symbol(Side::Bottom, *bottom_left));
        }
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x + area.width.saturating_sub(1),
            area.y + area.height.saturating_sub(1),
        )) {
            cell.set_style(self.border_style);
            cell.set_symbol(self.symbol_set.symbol(Side::Bottom, *bottom_right));
        }

        for (i, symbol) in top.iter().enumerate() {
            if let Some(cell) = buf.cell_mut(Position::new(
                area.x + 1 + i as u16, //
                area.y,
            )) {
                cell.set_style(self.border_style);
                cell.set_symbol(self.symbol_set.symbol(Side::Top, *symbol));
            }
        }
        for (i, symbol) in bottom.iter().enumerate() {
            if let Some(cell) = buf.cell_mut(Position::new(
                area.x + 1 + i as u16,
                area.y + area.height.saturating_sub(1),
            )) {
                cell.set_style(self.border_style);
                cell.set_symbol(self.symbol_set.symbol(Side::Bottom, *symbol));
            }
        }
        for (i, symbol) in left.iter().enumerate() {
            if let Some(cell) = buf.cell_mut(Position::new(
                area.x, //
                area.y + 1 + i as u16,
            )) {
                cell.set_style(self.border_style);
                cell.set_symbol(self.symbol_set.symbol(Side::Left, *symbol));
            }
        }
        for (i, symbol) in right.iter().enumerate() {
            if let Some(cell) = buf.cell_mut(Position::new(
                area.x + area.width.saturating_sub(1), //
                area.y + 1 + i as u16,
            )) {
                cell.set_style(self.border_style);
                cell.set_symbol(self.symbol_set.symbol(Side::Right, *symbol));
            }
        }
    }
}

///
/// Create a basic border for the area.
///
fn base_border(area: Rect) -> BlockBorder {
    let mut symbols = Vec::with_capacity(area.width as usize * 2 + area.height as usize * 2);
    symbols.push(BorderSymbol::StartCornerRegular);
    if area.width > 2 {
        for _ in 0..area.width - 2 {
            symbols.push(BorderSymbol::SideRegular)
        }
    }
    symbols.push(BorderSymbol::EndCornerRegular);
    if area.height > 2 {
        for _ in 0..area.height - 2 {
            symbols.push(BorderSymbol::SideRegular)
        }
    }
    symbols.push(BorderSymbol::StartCornerRegular);
    if area.width > 2 {
        for _ in 0..area.width - 2 {
            symbols.push(BorderSymbol::SideRegular)
        }
    }
    symbols.push(BorderSymbol::EndCornerRegular);
    if area.height > 2 {
        for _ in 0..area.height - 2 {
            symbols.push(BorderSymbol::SideRegular)
        }
    }

    BlockBorder {
        border_style: Default::default(),
        symbol_set: Rc::new(PlainSymbolSet),
        area,
        symbols,
    }
}

/// Create a connected border.
///
/// Given the layout and the border type for each area
/// creates a BlockBorder for the selected area.
/// This border has all the necessary connections to the
/// other borders.
fn connected_border(areas: &[Rect], borders: &[BorderType], n: usize) -> BlockBorder {
    let own_border = borders[n];
    let area = areas[n];
    let area_x1 = area.x;
    let area_y1 = area.y;
    let area_x2 = area.x + area.width.saturating_sub(1);
    let area_y2 = area.y + area.height.saturating_sub(1);

    assert!(area_x1 <= area_x2);
    assert!(area_y1 <= area_y2);

    let mut block = base_border(area).border_type(own_border);

    let (
        top_left, //
        top,
        top_right,
        right,
        bottom_left,
        bottom,
        bottom_right,
        left,
    ) = block.split_mut();

    for (i, test) in areas.iter().enumerate() {
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
                top_left,
                top,
                top_right,
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
                bottom_left,
                bottom,
                bottom_right,
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
                top_left,
                left,
                bottom_left,
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
                top_right,
                right,
                bottom_right,
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

#[inline(always)]
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
        // for i in 0..(p2 - area_p1) {
        //     block[i].overlap(parallel_side, other_border);
        // }
        block[p2 - area_p1 - 1].join_outward(start_side.opposite(), other_border);
    } else if p1 < area_p1 && p2 == area_p2 {
        // right corner/right corner, overhanging to the left.
        start_corner.prolong(parallel_side, other_border);
        // for i in 0..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.join_outward(start_side.opposite(), other_border);
    } else if p1 < area_p1 && p2 > area_p2 {
        // overhang on both sides
        start_corner.prolong(parallel_side, other_border);
        // for i in 0..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.prolong(parallel_side, other_border);
    } else if p1 == area_p1 && p2 < area_p2 {
        // left corner/left corner, ends inside
        start_corner.join_outward(start_side, other_border);
        // for i in 0..p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        block[p2 - area_p1 - 1].join_outward(start_side.opposite(), other_border);
    } else if p1 == area_p1 && p2 == area_p2 {
        // full overlap
        start_corner.join_outward(start_side, other_border);
        // for i in 0..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.join_outward(start_side.opposite(), other_border);
    } else if p1 == area_p1 && p2 > area_p2 {
        // left corner/left corner, overhanging to the right.
        start_corner.join_outward(start_side, other_border);
        // for i in 0..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.prolong(parallel_side, other_border);
    } else if p1 < area_p2 && p2 < area_p2 {
        // partial overlap
        block[p1 - area_p1 - 1].join_outward(start_side, other_border);
        // for i in p1 - area_p1..p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        block[p2 - area_p1 - 1].join_outward(start_side.opposite(), other_border);
    } else if p1 < area_p2 && p2 == area_p2 {
        // start inside, right corner/right corner.
        block[p1 - area_p1 - 1].join_outward(start_side, other_border);
        // for i in p1 - area_p1..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.join_outward(start_side.opposite(), other_border);
    } else if p1 < area_p2 && p2 > area_p2 {
        // start inside, overhang to the right.
        block[p1 - area_p1 - 1].join_outward(start_side, other_border);
        // for i in p1 - area_p1..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.prolong(parallel_side, other_border);
    } else if p1 == area_p2 && p2 > area_p2 {
        // left corner/right corner
        end_corner.join_outward(start_side, other_border);
        end_corner.prolong(parallel_side, other_border);
    } else {
        // right out
    }
}

#[inline(always)]
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
        // for i in 0..(p2 - area_p1) {
        //     block[i].overlap(parallel_side, other_border);
        // }
        block[p2 - area_p1 - 1].join_outward(start_side.opposite(), other_border);
    } else if p1 < area_p1 && p2 == area_p2 {
        // right corner/right corner, overhanging to the left.
        start_corner.join_outward(parallel_side, other_border);
        // for i in 0..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.prolong(start_side.opposite(), other_border);
    } else if p1 < area_p1 && p2 > area_p2 {
        // overhang on both sides
        start_corner.join_outward(parallel_side, other_border);
        // for i in 0..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.join_outward(parallel_side, other_border);
    } else if p1 == area_p1 && p2 < area_p2 {
        // left corner/left corner, ends inside
        start_corner.prolong(start_side, other_border);
        // for i in 0..p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        block[p2 - area_p1 - 1].join_outward(start_side.opposite(), other_border);
    } else if p1 == area_p1 && p2 == area_p2 {
        // full overlap
        start_corner.prolong(start_side, other_border);
        // for i in 0..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.prolong(start_side.opposite(), other_border);
    } else if p1 == area_p1 && p2 > area_p2 {
        // left corner/left corner, overhanging to the right.
        start_corner.prolong(start_side, other_border);
        // for i in 0..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.join_outward(parallel_side, other_border);
    } else if p1 < area_p2 && p2 < area_p2 {
        // partial overlap
        block[p1 - area_p1 - 1].join_outward(start_side, other_border);
        // for i in p1 - area_p1..p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        block[p2 - area_p1 - 1].join_outward(start_side.opposite(), other_border);
    } else if p1 < area_p2 && p2 == area_p2 {
        // start inside, right corner/right corner.
        block[p1 - area_p1 - 1].join_outward(start_side, other_border);
        // for i in p1 - area_p1..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.prolong(start_side.opposite(), other_border);
    } else if p1 < area_p2 && p2 > area_p2 {
        // start inside, overhang to the right.
        block[p1 - area_p1 - 1].join_outward(start_side, other_border);
        // for i in p1 - area_p1..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.join_outward(parallel_side, other_border);
    } else if p1 == area_p2 && p2 > area_p2 {
        // left corner/right corner
        end_corner.prolong(start_side, other_border);
        end_corner.join_outward(parallel_side, other_border);
    } else {
        // right out
    }
}
