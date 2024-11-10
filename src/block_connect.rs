use crate::border_symbols::{symbol_set, PlainSymbolSet};
use crate::{BorderSymbol, BorderSymbolSet, Side};
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::Style;
use ratatui::widgets::{BorderType, Widget};
use std::rc::Rc;

///
/// Add a connection point to an existing border.
///
/// This widget can render any BorderSymbol from the given BorderSymbolSet.
/// The exact placement is up to the caller, it renders the glyph at
/// (area.x, area.y).
///
#[derive(Clone)]
pub struct BlockConnect {
    border_style: Style,
    symbol_set: Rc<dyn BorderSymbolSet>,
    side: Side,
    symbol: BorderSymbol,
}

impl BlockConnect {
    ///
    pub fn new() -> Self {
        Self {
            border_style: Default::default(),
            symbol_set: Rc::new(PlainSymbolSet),
            side: Side::Top,
            symbol: BorderSymbol::StartCornerRegular,
        }
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
    /// For which side of the area is this meant.
    ///
    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    ///
    /// What kind of symbol.
    ///
    pub fn symbol(mut self, symbol: BorderSymbol) -> Self {
        self.symbol = symbol;
        self
    }
}

impl Widget for BlockConnect {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if let Some(cell) = buf.cell_mut(Position::new(area.x, area.y)) {
            cell.set_style(self.border_style);
            cell.set_symbol(self.symbol_set.symbol(self.side, self.symbol));
        }
    }
}
