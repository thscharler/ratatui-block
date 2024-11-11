use crate::block_border::BlockBorder;
use crate::border_symbols::{symbol_set, PlainSymbolSet};
use crate::{BorderSymbol, BorderSymbolSet, Side};
use log::debug;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{BorderType, Widget};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub struct BlockGrid {
    block: BlockBorder,

    inner_style: Style,
    outer_border_type: BorderType,
    inner_border_type: BorderType,
    inner_set: Rc<dyn BorderSymbolSet>,

    horizontal_side: Side,
    vertical_side: Side,

    vertical: Vec<u16>,
    horizontal: Vec<u16>,
}

impl Debug for BlockGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockGrid")
            .field("block", &self.block)
            .field("inner_style", &self.inner_style)
            .field("outer_border_type", &self.outer_border_type)
            .field("inner_border_type", &self.inner_border_type)
            .field("inner_set", &"..dyn..")
            .field("vertical", &self.vertical)
            .field("horizontal", &self.horizontal)
            .finish()
    }
}

impl BlockGrid {
    pub fn new(area: Rect) -> Self {
        Self {
            block: BlockBorder::new(area),
            inner_style: Default::default(),
            outer_border_type: Default::default(),
            inner_border_type: Default::default(),
            inner_set: Rc::new(PlainSymbolSet),
            horizontal_side: Side::Left,
            vertical_side: Side::Top,
            vertical: vec![],
            horizontal: vec![],
        }
    }

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
    pub fn border_set(mut self, border_set: Rc<dyn BorderSymbolSet>) -> Self {
        self.block = self.block.border_set(border_set);
        self
    }

    ///
    /// Sets the border type used.
    ///
    pub fn inner_border_type(mut self, border: BorderType) -> Self {
        self.inner_border_type = border;
        self.inner_set = symbol_set(border);
        self
    }

    ///
    /// Use a BorderSymbolSet.
    ///
    pub fn inner_border_set(mut self, border_set: Rc<dyn BorderSymbolSet>) -> Self {
        self.inner_set = border_set;
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
        for p in self.vertical.iter().copied() {
            if p > 0 && p < self.block.get_area().width.saturating_sub(2) {
                self.block.top_mut()[p as usize - 1] =
                    BorderSymbol::SideInward(self.vertical_side, self.inner_border_type);
                self.block.bottom_mut()[p as usize - 1] =
                    BorderSymbol::SideInward(self.vertical_side, self.inner_border_type);
            }
        }
        for p in self.horizontal.iter().copied() {
            if p > 0 && p < self.block.get_area().height.saturating_sub(2) {
                self.block.left_mut()[p as usize - 1] =
                    BorderSymbol::SideInward(self.horizontal_side, self.inner_border_type);
                self.block.right_mut()[p as usize - 1] =
                    BorderSymbol::SideInward(self.horizontal_side, self.inner_border_type);
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

        self.block.render(area, buf);

        for x in self.vertical.iter().copied() {
            if x > 0 && x < area.width.saturating_sub(2) {
                for y in area.y + 1..area.y + area.height.saturating_sub(1) {
                    if let Some(cell) = buf.cell_mut(Position::new(area.x + x, y)) {
                        cell.set_style(self.inner_style);
                        cell.set_symbol(
                            self.inner_set
                                .symbol(self.vertical_side, BorderSymbol::SideRegular),
                        );
                    }
                }
            }
        }
        for y in self.horizontal.iter().copied() {
            if y > 0 && y < area.height.saturating_sub(2) {
                for x in area.x + 1..area.x + area.width.saturating_sub(1) {
                    if let Some(cell) = buf.cell_mut(Position::new(x, area.y + y)) {
                        cell.set_style(self.inner_style);
                        cell.set_symbol(
                            self.inner_set
                                .symbol(self.horizontal_side, BorderSymbol::SideRegular),
                        );
                    }
                }
            }
        }
        for x in self.vertical.iter().copied() {
            if x > 0 && x < area.width.saturating_sub(2) {
                for y in self.horizontal.iter().copied() {
                    if y > 0 && y < area.height.saturating_sub(2) {
                        if let Some(cell) = buf.cell_mut(Position::new(area.x + x, area.y + y)) {
                            cell.set_style(Style::new().red());
                            cell.set_symbol(self.inner_set.crossing(
                                self.vertical_side,
                                self.inner_border_type,
                                self.horizontal_side,
                                self.inner_border_type,
                                self.vertical_side,
                                self.inner_border_type,
                                self.horizontal_side,
                                self.inner_border_type,
                            ));
                        }
                    }
                }
            }
        }
    }
}
