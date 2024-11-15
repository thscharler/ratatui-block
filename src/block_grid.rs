use crate::block_border::render_block_direct;
use crate::border_symbols::{symbol_set, PlainSymbolSet};
use crate::{BorderSymbol, BorderSymbolSet, Side};
use dyn_clone::clone_box;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{BorderType, Widget};
use std::fmt::{Debug, Formatter};

///
/// Renders a block border and a connected grid inside.
///
///
pub struct BlockGrid {
    outer_style: Style,
    outer_set: Box<dyn BorderSymbolSet>,

    horizontal_style: Style,
    horizontal_side: Side,
    horizontal_set: Box<dyn BorderSymbolSet>,

    vertical_style: Style,
    vertical_side: Side,
    vertical_set: Box<dyn BorderSymbolSet>,

    // x coordinates for the vertical lines.
    // relative to the area.
    vertical: Vec<u16>,
    // y coordinates for the horizontal lines.
    // relative to the area.
    horizontal: Vec<u16>,
}

impl Clone for BlockGrid {
    fn clone(&self) -> Self {
        Self {
            outer_style: self.outer_style,
            outer_set: clone_box(self.outer_set.as_ref()),
            horizontal_style: self.horizontal_style,
            horizontal_side: self.horizontal_side,
            horizontal_set: clone_box(self.horizontal_set.as_ref()),
            vertical_style: self.vertical_style,
            vertical_side: self.vertical_side,
            vertical_set: clone_box(self.vertical_set.as_ref()),
            vertical: self.vertical.clone(),
            horizontal: self.horizontal.clone(),
        }
    }
}

impl Debug for BlockGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockGrid")
            .field("outer_style", &self.outer_style)
            .field("outer_border_set", &"outer_border_set")
            .field("horizontal_style", &self.horizontal_style)
            .field("horizontal_side", &self.horizontal_side)
            .field("horizontal_set", &"horizontal_set")
            .field("vertical_style", &self.vertical_style)
            .field("vertical_side", &self.vertical_side)
            .field("vertical_set", &"vertical_set")
            .field("horizontal", &self.horizontal)
            .field("vertical", &self.vertical)
            .finish()
    }
}

impl BlockGrid {
    pub fn new() -> Self {
        Self {
            outer_style: Default::default(),
            outer_set: Box::new(PlainSymbolSet),
            horizontal_style: Default::default(),
            horizontal_side: Side::Left,
            horizontal_set: Box::new(PlainSymbolSet),
            vertical_style: Default::default(),
            vertical_side: Side::Top,
            vertical_set: Box::new(PlainSymbolSet),
            vertical: vec![],
            horizontal: vec![],
        }
    }

    /// Create a grid line for every single cell gap between the
    /// given areas.
    ///
    /// If the areas overlap, or if they have overlapping x- or y-ranges
    /// the results may not be what you expect.
    pub fn from_layout(layout: &[Rect]) -> Self {
        create_grid(layout)
    }

    ///
    /// Border style for the border.
    ///
    pub fn border_style(mut self, style: Style) -> Self {
        self.outer_style = style;
        self
    }

    ///
    /// Border style for the border.
    ///
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.outer_set = symbol_set(border_type);
        self
    }

    ///
    /// Border style for the border.
    ///
    pub fn border_set(mut self, border_set: impl BorderSymbolSet + 'static) -> Self {
        self.outer_set = Box::new(border_set);
        self
    }

    ///
    /// Border style for the border.
    ///
    pub fn horizontal_style(mut self, style: Style) -> Self {
        self.horizontal_style = style;
        self
    }

    /// Add a horizontal side.
    pub fn horizontal_side(mut self, side: Side) -> Self {
        self.horizontal_side = side;
        self
    }

    ///
    /// Sets the border type used.
    ///
    pub fn horizontal_border_type(mut self, border: BorderType) -> Self {
        self.horizontal_set = symbol_set(border);
        self
    }

    ///
    /// Use a BorderSymbolSet.
    ///
    pub fn horizontal_border_set(mut self, border_set: impl BorderSymbolSet + 'static) -> Self {
        self.horizontal_set = Box::new(border_set);
        self
    }

    /// Add a horizontal divider.
    pub fn horizontal(mut self, pos: u16) -> Self {
        self.horizontal.push(pos);
        self
    }

    ///
    /// Border style for the border.
    ///
    pub fn vertical_style(mut self, style: Style) -> Self {
        self.vertical_style = style;
        self
    }

    /// Add a vertical side.
    pub fn vertical_side(mut self, side: Side) -> Self {
        self.vertical_side = side;
        self
    }

    ///
    /// Sets the border type used.
    ///
    pub fn vertical_border_type(mut self, border: BorderType) -> Self {
        self.vertical_set = symbol_set(border);
        self
    }

    ///
    /// Use a BorderSymbolSet.
    ///
    pub fn vertical_border_set(mut self, border_set: impl BorderSymbolSet + 'static) -> Self {
        self.vertical_set = Box::new(border_set);
        self
    }

    /// Add a vertical divider.
    pub fn vertical(mut self, pos: u16) -> Self {
        self.vertical.push(pos);
        self
    }
}

impl Widget for BlockGrid {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &BlockGrid {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // render the block .
        render_block_direct(self.outer_style, self.outer_set.as_ref(), area, buf);

        // render connections
        for x in self.vertical.iter().copied() {
            if x == 0 || x >= area.width.saturating_sub(2) {
                continue;
            }
            if let Some(cell) = buf.cell_mut((area.x + x, area.y)) {
                cell.set_symbol(self.outer_set.symbol(
                    Side::Top,
                    BorderSymbol::SideInward(self.vertical_side, self.vertical_set.border_type()),
                ));
            }
            if let Some(cell) = buf.cell_mut((area.x + x, area.y + area.height.saturating_sub(1))) {
                cell.set_symbol(self.outer_set.symbol(
                    Side::Bottom,
                    BorderSymbol::SideInward(self.vertical_side, self.vertical_set.border_type()),
                ));
            }
        }
        for y in self.horizontal.iter().copied() {
            if y == 0 || y >= area.height.saturating_sub(2) {
                continue;
            }
            if let Some(cell) = buf.cell_mut((area.x, area.y + y)) {
                cell.set_symbol(self.outer_set.symbol(
                    Side::Left,
                    BorderSymbol::SideInward(
                        self.horizontal_side,
                        self.horizontal_set.border_type(),
                    ),
                ));
            }
            if let Some(cell) = buf.cell_mut((area.x + area.width.saturating_sub(1), area.y + y)) {
                cell.set_symbol(self.outer_set.symbol(
                    Side::Right,
                    BorderSymbol::SideInward(
                        self.horizontal_side,
                        self.horizontal_set.border_type(),
                    ),
                ));
            }
        }

        // render vertical
        for x in self.vertical.iter().copied() {
            if x == 0 || x >= area.width.saturating_sub(2) {
                continue;
            }
            for y in area.y + 1..area.y + area.height.saturating_sub(1) {
                if let Some(cell) = buf.cell_mut(Position::new(area.x + x, y)) {
                    cell.set_style(self.vertical_style);
                    cell.set_symbol(
                        self.vertical_set
                            .symbol(self.vertical_side, BorderSymbol::SideRegular),
                    );
                }
            }
        }
        // render horizontal
        for y in self.horizontal.iter().copied() {
            if y == 0 || y >= area.height.saturating_sub(2) {
                continue;
            }
            for x in area.x + 1..area.x + area.width.saturating_sub(1) {
                if let Some(cell) = buf.cell_mut(Position::new(x, area.y + y)) {
                    cell.set_style(self.horizontal_style);
                    cell.set_symbol(
                        self.horizontal_set
                            .symbol(self.horizontal_side, BorderSymbol::SideRegular),
                    );
                }
            }
        }
        // render crossings
        for x in self.vertical.iter().copied() {
            if x == 0 || x >= area.width.saturating_sub(2) {
                continue;
            }
            for y in self.horizontal.iter().copied() {
                if y == 0 || y >= area.height.saturating_sub(2) {
                    continue;
                }
                if let Some(cell) = buf.cell_mut(Position::new(area.x + x, area.y + y)) {
                    cell.set_style(Style::new().red());
                    cell.set_symbol(self.horizontal_set.symbol(
                        self.horizontal_side,
                        BorderSymbol::SideCrossed(
                            self.vertical_side,
                            self.vertical_set.border_type(),
                            self.vertical_side,
                            self.vertical_set.border_type(),
                        ),
                    ));
                }
            }
        }
    }
}

/// Create a grid from the gaps left by the given areas.
fn create_grid(areas: &[Rect]) -> BlockGrid {
    let mut grid = BlockGrid::new();

    let mut y_coord = Vec::new();
    let mut x_coord = Vec::new();

    for area in areas {
        y_coord.push(area.top());
        y_coord.push(area.bottom().saturating_sub(1));
        x_coord.push(area.left());
        x_coord.push(area.right().saturating_sub(1));
    }

    y_coord.sort();
    y_coord.dedup();
    x_coord.sort();
    x_coord.dedup();

    let mut idx = 1;
    let min_y = y_coord.get(0).copied().unwrap_or_default();
    loop {
        if idx + 1 >= y_coord.len() {
            break;
        }

        // detect gaps
        // only jumps from odd to even indexes mark area boundaries
        if idx % 2 == 1 && y_coord[idx] + 2 == y_coord[idx + 1] {
            grid = grid.horizontal(y_coord[idx] + 1 - min_y);
        }

        idx += 1;
    }

    let mut idx = 1;
    let min_x = x_coord.get(0).copied().unwrap_or_default();
    loop {
        if idx + 1 >= x_coord.len() {
            break;
        }

        // detect gaps
        // only jumps from odd to even indexes mark area boundaries
        if idx % 2 == 1 && x_coord[idx] + 2 == x_coord[idx + 1] {
            grid = grid.vertical(x_coord[idx] + 1 - min_x);
        }

        idx += 1;
    }

    grid
}
