use crate::block_border::BlockBorder;
use crate::border_symbols::{symbol_set, PlainSymbolSet};
use crate::{BorderSymbol, BorderSymbolSet, Side};
use dyn_clone::clone_box;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{BorderType, Widget};
use std::fmt::{Debug, Formatter};

pub struct BlockGrid {
    block: BlockBorder,
    outer_border_type: BorderType,

    inner_style: Style,

    horizontal_side: Side,
    horizontal_border: BorderType,
    horizontal_set: Box<dyn BorderSymbolSet>,
    vertical_side: Side,
    vertical_border: BorderType,
    vertical_set: Box<dyn BorderSymbolSet>,

    vertical: Vec<u16>,
    horizontal: Vec<u16>,
}

impl Clone for BlockGrid {
    fn clone(&self) -> Self {
        Self {
            block: self.block.clone(),
            outer_border_type: self.outer_border_type,
            inner_style: self.inner_style,
            horizontal_side: self.horizontal_side,
            horizontal_border: self.horizontal_border,
            horizontal_set: clone_box(self.horizontal_set.as_ref()),
            vertical_side: self.vertical_side,
            vertical_border: self.vertical_border,
            vertical_set: clone_box(self.vertical_set.as_ref()),
            vertical: self.vertical.clone(),
            horizontal: self.horizontal.clone(),
        }
    }
}

impl Debug for BlockGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockGrid")
            .field("block", &self.block)
            .field("inner_style", &self.inner_style)
            .field("outer_border_type", &self.outer_border_type)
            .field("inner_set", &"..dyn..")
            .field("vertical", &self.vertical)
            .field("horizontal", &self.horizontal)
            // todo:
            .finish()
    }
}

impl BlockGrid {
    pub fn new(area: Rect) -> Self {
        Self {
            block: BlockBorder::from_area(area),
            inner_style: Default::default(),
            outer_border_type: Default::default(),
            horizontal_side: Side::Left,
            horizontal_border: Default::default(),
            horizontal_set: Box::new(PlainSymbolSet),
            vertical_side: Side::Top,
            vertical_border: Default::default(),
            vertical_set: Box::new(PlainSymbolSet),
            vertical: vec![],
            horizontal: vec![],
        }
    }

    // TODO: from_layout()

    ///
    /// Border style for the border.
    ///
    pub fn border_style(mut self, style: Style) -> Self {
        self.block = self.block.border_style(style);
        self
    }

    ///
    /// Border style for the border.
    ///
    pub fn inner_style(mut self, style: Style) -> Self {
        self.inner_style = style;
        self
    }

    ///
    /// Sets the border type used.
    ///
    pub fn border_type(mut self, border: BorderType) -> Self {
        self.block = self.block.border_type(border);
        self.outer_border_type = border;
        self
    }

    ///
    /// Use a BorderSymbolSet.
    ///
    pub fn border_set(mut self, border_set: Box<dyn BorderSymbolSet>) -> Self {
        self.block = self.block.border_set(border_set);
        self
    }

    ///
    /// Sets the border type used.
    ///
    pub fn horizontal_border_type(mut self, border: BorderType) -> Self {
        self.horizontal_border = border;
        self.horizontal_set = symbol_set(border);
        self
    }

    ///
    /// Use a BorderSymbolSet.
    ///
    pub fn horizontal_border_set(mut self, border_set: Box<dyn BorderSymbolSet>) -> Self {
        self.horizontal_set = border_set;
        self
    }

    ///
    /// Sets the border type used.
    ///
    pub fn vertical_border_type(mut self, border: BorderType) -> Self {
        self.vertical_border = border;
        self.vertical_set = symbol_set(border);
        self
    }

    ///
    /// Use a BorderSymbolSet.
    ///
    pub fn vertical_border_set(mut self, border_set: Box<dyn BorderSymbolSet>) -> Self {
        self.vertical_set = border_set;
        self
    }

    /// Add a vertical side.
    pub fn vertical_side(mut self, side: Side) -> Self {
        self.vertical_side = side;
        self
    }

    /// Add a horizontal side.
    pub fn horizontal_side(mut self, side: Side) -> Self {
        self.horizontal_side = side;
        self
    }

    /// Add a vertical divider.
    pub fn vertical(mut self, pos: u16) -> Self {
        self.vertical.push(pos);
        self
    }

    /// Add a horizontal divider.
    pub fn horizontal(mut self, pos: u16) -> Self {
        self.horizontal.push(pos);
        self
    }
}

impl BlockGrid {
    fn layout(&mut self) {
        let border = self.block.prefab.as_mut().expect("border");
        let width = border.width();
        let height = border.height();
        let (_, top, _, right, _, bottom, _, left) = border.split_mut();

        for p in self.vertical.iter().copied() {
            if p > 0 && p < width.saturating_sub(2) {
                top[p as usize - 1] =
                    BorderSymbol::SideInward(self.vertical_side, self.vertical_border);
                bottom[p as usize - 1] =
                    BorderSymbol::SideInward(self.vertical_side, self.vertical_border);
            }
        }
        for p in self.horizontal.iter().copied() {
            if p > 0 && p < height.saturating_sub(2) {
                left[p as usize - 1] =
                    BorderSymbol::SideInward(self.horizontal_side, self.horizontal_border);
                right[p as usize - 1] =
                    BorderSymbol::SideInward(self.horizontal_side, self.horizontal_border);
            }
        }
    }
}

impl Widget for BlockGrid {
    fn render(mut self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.layout();

        // render the block and all connections.
        self.block.render(area, buf);

        // render vertical
        for x in self.vertical.iter().copied() {
            if x > 0 && x < area.width.saturating_sub(2) {
                for y in area.y + 1..area.y + area.height.saturating_sub(1) {
                    if let Some(cell) = buf.cell_mut(Position::new(area.x + x, y)) {
                        cell.set_style(self.inner_style);
                        cell.set_symbol(
                            self.vertical_set
                                .symbol(self.vertical_side, BorderSymbol::SideRegular),
                        );
                    }
                }
            }
        }
        // render horizontal
        for y in self.horizontal.iter().copied() {
            if y > 0 && y < area.height.saturating_sub(2) {
                for x in area.x + 1..area.x + area.width.saturating_sub(1) {
                    if let Some(cell) = buf.cell_mut(Position::new(x, area.y + y)) {
                        cell.set_style(self.inner_style);
                        cell.set_symbol(
                            self.horizontal_set
                                .symbol(self.horizontal_side, BorderSymbol::SideRegular),
                        );
                    }
                }
            }
        }
        // render crossings
        for x in self.vertical.iter().copied() {
            if x > 0 && x < area.width.saturating_sub(2) {
                for y in self.horizontal.iter().copied() {
                    if y > 0 && y < area.height.saturating_sub(2) {
                        if let Some(cell) = buf.cell_mut(Position::new(area.x + x, area.y + y)) {
                            cell.set_style(Style::new().red());
                            cell.set_symbol(self.horizontal_set.symbol(
                                self.horizontal_side,
                                BorderSymbol::SideCrossed(
                                    self.vertical_side,
                                    self.vertical_border,
                                    self.vertical_side,
                                    self.vertical_border,
                                ),
                            ));
                        }
                    }
                }
            }
        }
    }
}
