use crate::border_symbols::symbol_set;
use crate::{BorderSymbol, BorderSymbolSet, Side};
use dyn_clone::clone_box;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::prelude::{Style, Widget};
use ratatui::widgets::BorderType;
use std::fmt::{Debug, Formatter};

///
/// Border for a Block.
///
/// Borders are rendered as four lines along the sides.
///
/// ![schematics](https://raw.githubusercontent.com/thscharler/ratatui-block/refs/heads/master/diagram/blockborder.png)
///
/// The BorderSymbolSet maps geometrical positions along
/// the lines to the actual glyphs used.
///
/// ![schematics](https://raw.githubusercontent.com/thscharler/ratatui-block/refs/heads/master/diagram/border_symbol_1.png)
///
///
pub struct BlockBorder {
    border_style: Style,
    symbol_set: Box<dyn BorderSymbolSet>,

    // prebuilt border.
    pub(crate) prefab: Option<PrefabBorder>,
}

/// Contains the data for a prefabricated block for some specific
/// dimensions.
#[derive(Debug, Clone)]
pub(crate) struct PrefabBorder {
    width: u16,
    height: u16,
    symbols: Vec<BorderSymbol>,
}

impl Clone for BlockBorder {
    fn clone(&self) -> Self {
        Self {
            border_style: self.border_style,
            symbol_set: clone_box(self.symbol_set.as_ref()),
            prefab: self.prefab.clone(),
        }
    }
}

impl Debug for BlockBorder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockBorder")
            .field("border_style", &self.border_style)
            .field("symbol_set", &"..dyn..")
            .field("border", &self.prefab)
            .finish()
    }
}

impl BlockBorder {
    /// Create a block border.
    pub fn new() -> Self {
        Self {
            border_style: Default::default(),
            symbol_set: symbol_set(BorderType::Plain),
            prefab: None,
        }
    }

    ///
    /// New block border for the given area.
    ///
    /// The resulting border can only be rendered for an area of the
    /// same size.
    ///
    pub fn from_area(area: Rect) -> Self {
        Self {
            border_style: Default::default(),
            symbol_set: symbol_set(BorderType::Plain),
            prefab: Some(PrefabBorder::new(area)),
        }
    }

    ///
    /// New block border for a Block that is part of some bigger layout.
    ///
    /// Given all the areas of the layout and each border type,
    /// this creates a border that is connected at the edges.
    ///
    /// The resulting border can only be rendered for an area of the same size.
    ///
    pub fn from_layout(areas: &[Rect], borders: &[BorderType], select: usize) -> Self {
        create_connected_border(areas, borders, select)
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
    pub fn border_set(mut self, border_set: Box<dyn BorderSymbolSet>) -> Self {
        self.symbol_set = border_set;
        self
    }

    ///
    /// Return the symbol at the given position along the border.
    ///
    /// When using the returned BorderSymbol you must be aware,
    /// that the corners are rendered with the top and bottom lines.
    ///
    /// x
    ///
    /// __Panic__
    ///
    /// Panics if the dimensions of the area don't match a prefabricated border.
    /// Panics if the given position doesn't lie on the border.
    ///
    pub fn get_symbol(&self, area: Rect, position: Position) -> BorderSymbol {
        if let Some(border) = self.prefab.as_ref() {
            assert!(area.width == border.width && area.height == border.height);
            assert!(area.left() == position.x || area.right().saturating_sub(1) == position.x);
            assert!(area.top() == position.y || area.bottom().saturating_sub(1) == position.y);

            if area.top() == position.y {
                border.symbols[position.x as usize]
            } else if area.bottom().saturating_sub(1) == position.y {
                border.symbols
                    [(border.width + border.height.saturating_sub(2) + position.x) as usize]
            } else if area.right().saturating_sub(1) == position.x {
                border.symbols[(border.width + position.y.saturating_sub(1)) as usize]
            } else if area.left() == position.x {
                border.symbols[(border.width * 2
                    + border.height.saturating_sub(2)
                    + position.y.saturating_sub(1)) as usize]
            } else {
                panic!("position not on the border");
            }
        } else {
            if area.top() == position.y {
                if area.left() == position.x {
                    BorderSymbol::StartCornerRegular
                } else if area.right().saturating_sub(1) == position.x {
                    BorderSymbol::SideRegular
                } else {
                    BorderSymbol::EndCornerRegular
                }
            } else if area.bottom().saturating_sub(1) == position.y {
                if area.left() == position.x {
                    BorderSymbol::StartCornerRegular
                } else if area.right().saturating_sub(1) == position.x {
                    BorderSymbol::SideRegular
                } else {
                    BorderSymbol::EndCornerRegular
                }
            } else if area.right().saturating_sub(1) == position.x {
                BorderSymbol::SideRegular
            } else if area.left() == position.x {
                BorderSymbol::SideRegular
            } else {
                panic!("position not on the border");
            }
        }
    }
}

impl Widget for BlockBorder {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &BlockBorder {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if let Some(border) = self.prefab.as_ref() {
            render_block_prefab(
                border,
                self.border_style,
                self.symbol_set.as_ref(),
                area,
                buf,
            );
        } else {
            render_block_direct(self.border_style, self.symbol_set.as_ref(), area, buf);
        }
    }
}

pub(crate) fn render_block_prefab(
    border: &PrefabBorder,
    style: Style,
    symbols: &dyn BorderSymbolSet,
    area: Rect,
    buf: &mut Buffer,
) {
    let (
        top, //
        right,
        bottom,
        left,
    ) = border.split_render();

    for (i, (top_sym, bottom_sym)) in top.iter().zip(bottom.iter()).enumerate() {
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x + i as u16, //
            area.y,
        )) {
            cell.set_style(style);
            cell.set_symbol(symbols.symbol(Side::Top, *top_sym));
        }
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x + i as u16,
            area.y + area.height.saturating_sub(1),
        )) {
            cell.set_style(style);
            cell.set_symbol(symbols.symbol(Side::Bottom, *bottom_sym));
        }
    }

    for (i, (left_sym, right_sym)) in left.iter().zip(right.iter()).enumerate() {
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x, //
            area.y + 1 + i as u16,
        )) {
            cell.set_style(style);
            cell.set_symbol(symbols.symbol(Side::Left, *left_sym));
        }
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x + area.width.saturating_sub(1), //
            area.y + 1 + i as u16,
        )) {
            cell.set_style(style);
            cell.set_symbol(symbols.symbol(Side::Right, *right_sym));
        }
    }
}

pub(crate) fn render_block_direct(
    style: Style,
    symbols: &dyn BorderSymbolSet,
    area: Rect,
    buf: &mut Buffer,
) {
    // not prepared for a specific size.
    if let Some(cell) = buf.cell_mut(Position::new(
        area.x, //
        area.y,
    )) {
        cell.set_style(style);
        cell.set_symbol(symbols.symbol(Side::Top, BorderSymbol::StartCornerRegular));
    }

    let top_sym = symbols.symbol(Side::Top, BorderSymbol::SideRegular);
    let bottom_sym = symbols.symbol(Side::Bottom, BorderSymbol::SideRegular);
    for x in 1..area.width.saturating_sub(1) {
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x + x, //
            area.y,
        )) {
            cell.set_style(style);
            cell.set_symbol(top_sym);
        }
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x + x,
            area.y + area.height.saturating_sub(1),
        )) {
            cell.set_style(style);
            cell.set_symbol(bottom_sym);
        }
    }

    if let Some(cell) = buf.cell_mut(Position::new(
        area.x + area.width.saturating_sub(1), //
        area.y,
    )) {
        cell.set_style(style);
        cell.set_symbol(symbols.symbol(Side::Top, BorderSymbol::EndCornerRegular));
    }

    if let Some(cell) = buf.cell_mut(Position::new(
        area.x,
        area.y + area.height.saturating_sub(1),
    )) {
        cell.set_style(style);
        cell.set_symbol(symbols.symbol(Side::Bottom, BorderSymbol::StartCornerRegular));
    }

    let left_sym = symbols.symbol(Side::Left, BorderSymbol::SideRegular);
    let right_sym = symbols.symbol(Side::Right, BorderSymbol::SideRegular);
    for y in 1..area.height.saturating_sub(1) {
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x, //
            area.y + y,
        )) {
            cell.set_style(style);
            cell.set_symbol(left_sym);
        }
        if let Some(cell) = buf.cell_mut(Position::new(
            area.x + area.width.saturating_sub(1), //
            area.y + y,
        )) {
            cell.set_style(style);
            cell.set_symbol(right_sym);
        }
    }

    if let Some(cell) = buf.cell_mut(Position::new(
        area.x + area.width.saturating_sub(1),
        area.y + area.height.saturating_sub(1),
    )) {
        cell.set_style(style);
        cell.set_symbol(symbols.symbol(Side::Bottom, BorderSymbol::EndCornerRegular));
    }
}

impl PrefabBorder {
    /// Fill in a regular border for the given area.
    pub(crate) fn new(area: Rect) -> Self {
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

        PrefabBorder {
            width: area.width,
            height: area.height,
            symbols,
        }
    }

    ///
    /// Split into border parts.
    ///
    /// (top_left, top, top_right, right, bottom_left, bottom, bottom_right, left)
    ///
    #[inline(always)]
    pub(crate) fn split_mut(
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
        let (top, rest) = rest.split_at_mut(self.width.saturating_sub(2) as usize);
        let (top_right, rest) = rest.split_at_mut(1);
        let (right, rest) = rest.split_at_mut(self.height.saturating_sub(2) as usize);
        let (bottom_left, rest) = rest.split_at_mut(1);
        let (bottom, rest) = rest.split_at_mut(self.width.saturating_sub(2) as usize);
        let (bottom_right, rest) = rest.split_at_mut(1);
        let (left, rest) = rest.split_at_mut(self.height.saturating_sub(2) as usize);

        debug_assert!(rest.is_empty());

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
    /// (top, right, bottom, left)
    ///
    #[inline(always)]
    pub(crate) fn split_render(
        &self,
    ) -> (
        &[BorderSymbol],
        &[BorderSymbol],
        &[BorderSymbol],
        &[BorderSymbol],
    ) {
        let rest = self.symbols.as_slice();
        let (top, rest) = rest.split_at(self.width as usize);
        let (right, rest) = rest.split_at(self.height.saturating_sub(2) as usize);
        let (bottom, rest) = rest.split_at(self.width as usize);
        let (left, rest) = rest.split_at(self.height.saturating_sub(2) as usize);

        debug_assert!(rest.is_empty());

        (top, right, bottom, left)
    }
}

/// Create a connected border.
///
/// Given the layout and the border type for each area
/// creates a BlockBorder for the selected area.
/// This border has all the necessary connections to the
/// other borders.
#[inline]
fn create_connected_border(areas: &[Rect], borders: &[BorderType], n: usize) -> BlockBorder {
    let own_border = borders[n];
    let area = areas[n];
    let area_x1 = area.x;
    let area_y1 = area.y;
    let area_x2 = area.x + area.width.saturating_sub(1);
    let area_y2 = area.y + area.height.saturating_sub(1);

    let mut block = BlockBorder::from_area(area).border_type(own_border);

    let (
        top_left, //
        top,
        top_right,
        right,
        bottom_left,
        bottom,
        bottom_right,
        left,
    ) = block.prefab.as_mut().expect("border").split_mut();

    for (i, test) in areas.iter().enumerate() {
        let other_border = borders[i];

        let x1 = test.x;
        let y1 = test.y;
        let x2 = test.x + test.width.saturating_sub(1);
        let y2 = test.y + test.height.saturating_sub(1);

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

#[inline]
fn create_horizontal_side(
    start_corner: &mut BorderSymbol,
    block: &mut [BorderSymbol],
    end_corner: &mut BorderSymbol,
    p1: usize,
    p2: usize,
    area_p1: usize,
    area_p2: usize,
    parallel_side: Side,
    perpendicular_side: Side,
    other_border: BorderType,
) {
    if p1 < area_p1 && p2 < area_p1 {
        // left out
    } else if p1 < area_p1 && p2 == area_p1 {
        // corner/corner
        start_corner.join_outward(perpendicular_side.opposite(), other_border);
        start_corner.prolong(parallel_side, other_border);
    } else if p1 < area_p1 && p2 < area_p2 {
        // left overhanging
        start_corner.prolong(parallel_side, other_border);
        // for i in 0..(p2 - area_p1) {
        //     block[i].overlap(parallel_side, other_border);
        // }
        block[p2 - area_p1 - 1].join_outward(perpendicular_side.opposite(), other_border);
    } else if p1 < area_p1 && p2 == area_p2 {
        // right corner/right corner, overhanging to the left.
        start_corner.prolong(parallel_side, other_border);
        // for i in 0..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.join_outward(perpendicular_side.opposite(), other_border);
    } else if p1 < area_p1 && p2 > area_p2 {
        // overhang on both sides
        start_corner.prolong(parallel_side, other_border);
        // for i in 0..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.prolong(parallel_side, other_border);
    } else if p1 == area_p1 && p2 < area_p2 {
        // left corner/left corner, ends inside
        start_corner.join_outward(perpendicular_side, other_border);
        // for i in 0..p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        block[p2 - area_p1 - 1].join_outward(perpendicular_side.opposite(), other_border);
    } else if p1 == area_p1 && p2 == area_p2 {
        // full overlap
        start_corner.join_outward(perpendicular_side, other_border);
        // for i in 0..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.join_outward(perpendicular_side.opposite(), other_border);
    } else if p1 == area_p1 && p2 > area_p2 {
        // left corner/left corner, overhanging to the right.
        start_corner.join_outward(perpendicular_side, other_border);
        // for i in 0..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.prolong(parallel_side, other_border);
    } else if p1 < area_p2 && p2 < area_p2 {
        // partial overlap
        block[p1 - area_p1 - 1].join_outward(perpendicular_side, other_border);
        // for i in p1 - area_p1..p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        block[p2 - area_p1 - 1].join_outward(perpendicular_side.opposite(), other_border);
    } else if p1 < area_p2 && p2 == area_p2 {
        // start inside, right corner/right corner.
        block[p1 - area_p1 - 1].join_outward(perpendicular_side, other_border);
        // for i in p1 - area_p1..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.join_outward(perpendicular_side.opposite(), other_border);
    } else if p1 < area_p2 && p2 > area_p2 {
        // start inside, overhang to the right.
        block[p1 - area_p1 - 1].join_outward(perpendicular_side, other_border);
        // for i in p1 - area_p1..area_p2 - area_p1 - 1 {
        //     block[i].overlap(parallel_side, other_border);
        // }
        end_corner.prolong(parallel_side, other_border);
    } else if p1 == area_p2 && p2 > area_p2 {
        // left corner/right corner
        end_corner.join_outward(perpendicular_side, other_border);
        end_corner.prolong(parallel_side, other_border);
    } else {
        // right out
    }
}

#[inline]
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
