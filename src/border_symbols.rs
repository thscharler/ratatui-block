use crate::Side::{Bottom, Left, Right, Top};
use crate::{BorderSymbol, BorderSymbolSet, Side};
use log::debug;
use ratatui::symbols::border;
use ratatui::widgets::BorderType;
use ratatui::widgets::BorderType::{QuadrantInside, QuadrantOutside};
use std::rc::Rc;

///
/// Create the BorderSymbolSet for the given BorderType.
///
pub fn symbol_set(border: BorderType) -> Rc<dyn BorderSymbolSet> {
    match border {
        BorderType::Plain => Rc::new(PlainSymbolSet),
        BorderType::Rounded => Rc::new(RoundedSymbolSet),
        BorderType::Double => Rc::new(DoubleSymbolSet),
        BorderType::Thick => Rc::new(ThickSymbolSet),
        BorderType::QuadrantInside => Rc::new(QuadrantInsideSymbolSet),
        BorderType::QuadrantOutside => Rc::new(QuadrantOutsideSymbolSet),
    }
}

/// For manual borders.
///
/// This symbol set can have connections to other borders,
/// but it's not possible to have different connections depending
/// on the other border.
pub struct StaticSymbolSet {
    pub top_left_regular: &'static str,
    pub top_left_angled: &'static str,
    pub top_left_prolonged: &'static str,
    pub top_left_crossed: &'static str,

    pub top_regular: &'static str,
    pub top_overlap: &'static str,
    pub top_outward: &'static str,
    pub top_inward: &'static str,
    pub top_crossed: &'static str,

    pub top_right_regular: &'static str,
    pub top_right_angled: &'static str,
    pub top_right_prolonged: &'static str,
    pub top_right_crossed: &'static str,

    pub bottom_left_regular: &'static str,
    pub bottom_left_angled: &'static str,
    pub bottom_left_prolonged: &'static str,
    pub bottom_left_crossed: &'static str,

    pub bottom_regular: &'static str,
    pub bottom_overlap: &'static str,
    pub bottom_outward: &'static str,
    pub bottom_inward: &'static str,
    pub bottom_crossed: &'static str,

    pub bottom_right_regular: &'static str,
    pub bottom_right_angled: &'static str,
    pub bottom_right_prolonged: &'static str,
    pub bottom_right_crossed: &'static str,

    pub left_regular: &'static str,
    pub left_overlap: &'static str,
    pub left_outward: &'static str,
    pub left_inward: &'static str,
    pub left_crossed: &'static str,

    pub right_regular: &'static str,
    pub right_overlap: &'static str,
    pub right_outward: &'static str,
    pub right_inward: &'static str,
    pub right_crossed: &'static str,

    pub crossed: &'static str,
}

impl BorderSymbolSet for StaticSymbolSet {
    #[inline(always)]
    fn symbol(&self, side: Side, symbol: BorderSymbol) -> &'static str {
        match side {
            Side::Top => match symbol {
                BorderSymbol::StartCornerRegular => self.top_left_regular,
                BorderSymbol::StartCornerAngled(_, _) => self.top_left_angled,
                BorderSymbol::StartCornerProlonged(_, _) => self.top_left_prolonged,
                BorderSymbol::StartCornerCrossed(_, _, _, _) => self.top_left_crossed,
                BorderSymbol::SideRegular => self.top_regular,
                BorderSymbol::SideOverlap(_, _) => self.top_overlap,
                BorderSymbol::SideOutward(_, _) => self.top_outward,
                BorderSymbol::SideInward(_, _) => self.top_inward,
                BorderSymbol::SideCrossed(_, _, _, _) => self.top_crossed,
                BorderSymbol::EndCornerRegular => self.top_right_regular,
                BorderSymbol::EndCornerAngled(_, _) => self.top_right_angled,
                BorderSymbol::EndCornerProlonged(_, _) => self.top_right_prolonged,
                BorderSymbol::EndCornerCrossed(_, _, _, _) => self.top_right_crossed,
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => self.crossed,
            },
            Side::Bottom => match symbol {
                BorderSymbol::StartCornerRegular => self.bottom_left_regular,
                BorderSymbol::StartCornerAngled(_, _) => self.bottom_left_angled,
                BorderSymbol::StartCornerProlonged(_, _) => self.bottom_left_prolonged,
                BorderSymbol::StartCornerCrossed(_, _, _, _) => self.bottom_left_crossed,
                BorderSymbol::SideRegular => self.bottom_regular,
                BorderSymbol::SideOverlap(_, _) => self.bottom_overlap,
                BorderSymbol::SideOutward(_, _) => self.bottom_outward,
                BorderSymbol::SideInward(_, _) => self.bottom_inward,
                BorderSymbol::SideCrossed(_, _, _, _) => self.bottom_crossed,
                BorderSymbol::EndCornerRegular => self.bottom_right_regular,
                BorderSymbol::EndCornerAngled(_, _) => self.bottom_right_angled,
                BorderSymbol::EndCornerProlonged(_, _) => self.bottom_right_prolonged,
                BorderSymbol::EndCornerCrossed(_, _, _, _) => self.bottom_right_crossed,
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => self.crossed,
            },
            Side::Right => match symbol {
                BorderSymbol::StartCornerRegular => self.top_right_regular,
                BorderSymbol::StartCornerAngled(_, _) => self.top_right_angled,
                BorderSymbol::StartCornerProlonged(_, _) => self.top_right_prolonged,
                BorderSymbol::StartCornerCrossed(_, _, _, _) => self.top_right_crossed,
                BorderSymbol::SideRegular => self.right_regular,
                BorderSymbol::SideOverlap(_, _) => self.right_overlap,
                BorderSymbol::SideOutward(_, _) => self.right_outward,
                BorderSymbol::SideInward(_, _) => self.right_inward,
                BorderSymbol::SideCrossed(_, _, _, _) => self.right_crossed,
                BorderSymbol::EndCornerRegular => self.bottom_right_regular,
                BorderSymbol::EndCornerAngled(_, _) => self.bottom_right_angled,
                BorderSymbol::EndCornerProlonged(_, _) => self.bottom_right_prolonged,
                BorderSymbol::EndCornerCrossed(_, _, _, _) => self.bottom_right_crossed,
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => self.crossed,
            },
            Side::Left => match symbol {
                BorderSymbol::StartCornerRegular => self.top_left_regular,
                BorderSymbol::StartCornerAngled(_, _) => self.top_left_angled,
                BorderSymbol::StartCornerProlonged(_, _) => self.top_left_prolonged,
                BorderSymbol::StartCornerCrossed(_, _, _, _) => self.top_left_crossed,
                BorderSymbol::SideRegular => self.left_regular,
                BorderSymbol::SideOverlap(_, _) => self.left_overlap,
                BorderSymbol::SideOutward(_, _) => self.left_outward,
                BorderSymbol::SideInward(_, _) => self.left_inward,
                BorderSymbol::SideCrossed(_, _, _, _) => self.left_crossed,
                BorderSymbol::EndCornerRegular => self.bottom_left_regular,
                BorderSymbol::EndCornerAngled(_, _) => self.bottom_left_angled,
                BorderSymbol::EndCornerProlonged(_, _) => self.bottom_left_prolonged,
                BorderSymbol::EndCornerCrossed(_, _, _, _) => self.bottom_left_crossed,
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => self.crossed,
            },
        }
    }
}

/// Takes one of the old border::Set as base.
/// Acts just like an old-style border would.
pub struct OldSymbolSet {
    pub symbol_set: border::Set,
}

impl BorderSymbolSet for OldSymbolSet {
    #[inline(always)]
    fn symbol(&self, side: Side, symbol: BorderSymbol) -> &'static str {
        match side {
            Side::Top => match symbol {
                BorderSymbol::StartCornerRegular => self.symbol_set.top_left,
                BorderSymbol::StartCornerAngled(_, _) => self.symbol_set.top_left,
                BorderSymbol::StartCornerProlonged(_, _) => self.symbol_set.top_left,
                BorderSymbol::StartCornerCrossed(_, _, _, _) => self.symbol_set.top_left,
                BorderSymbol::SideRegular => self.symbol_set.horizontal_top,
                BorderSymbol::SideOverlap(_, _) => self.symbol_set.horizontal_top,
                BorderSymbol::SideOutward(_, _) => self.symbol_set.horizontal_top,
                BorderSymbol::SideInward(_, _) => self.symbol_set.horizontal_top,
                BorderSymbol::SideCrossed(_, _, _, _) => self.symbol_set.horizontal_top,
                BorderSymbol::EndCornerRegular => self.symbol_set.top_right,
                BorderSymbol::EndCornerAngled(_, _) => self.symbol_set.top_right,
                BorderSymbol::EndCornerProlonged(_, _) => self.symbol_set.top_right,
                BorderSymbol::EndCornerCrossed(_, _, _, _) => self.symbol_set.top_right,
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => &" ",
            },
            Side::Bottom => match symbol {
                BorderSymbol::StartCornerRegular => self.symbol_set.bottom_left,
                BorderSymbol::StartCornerAngled(_, _) => self.symbol_set.bottom_left,
                BorderSymbol::StartCornerProlonged(_, _) => self.symbol_set.bottom_left,
                BorderSymbol::StartCornerCrossed(_, _, _, _) => self.symbol_set.bottom_left,
                BorderSymbol::SideRegular => self.symbol_set.horizontal_bottom,
                BorderSymbol::SideOverlap(_, _) => self.symbol_set.horizontal_bottom,
                BorderSymbol::SideOutward(_, _) => self.symbol_set.horizontal_bottom,
                BorderSymbol::SideInward(_, _) => self.symbol_set.horizontal_bottom,
                BorderSymbol::SideCrossed(_, _, _, _) => self.symbol_set.horizontal_bottom,
                BorderSymbol::EndCornerRegular => self.symbol_set.bottom_right,
                BorderSymbol::EndCornerAngled(_, _) => self.symbol_set.bottom_right,
                BorderSymbol::EndCornerProlonged(_, _) => self.symbol_set.bottom_right,
                BorderSymbol::EndCornerCrossed(_, _, _, _) => self.symbol_set.bottom_right,
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => &" ",
            },
            Side::Right => match symbol {
                BorderSymbol::StartCornerRegular => self.symbol_set.top_right,
                BorderSymbol::StartCornerAngled(_, _) => self.symbol_set.top_right,
                BorderSymbol::StartCornerProlonged(_, _) => self.symbol_set.top_right,
                BorderSymbol::StartCornerCrossed(_, _, _, _) => self.symbol_set.top_right,
                BorderSymbol::SideRegular => self.symbol_set.vertical_right,
                BorderSymbol::SideOverlap(_, _) => self.symbol_set.vertical_right,
                BorderSymbol::SideOutward(_, _) => self.symbol_set.vertical_right,
                BorderSymbol::SideInward(_, _) => self.symbol_set.vertical_right,
                BorderSymbol::SideCrossed(_, _, _, _) => self.symbol_set.vertical_right,
                BorderSymbol::EndCornerRegular => self.symbol_set.bottom_right,
                BorderSymbol::EndCornerAngled(_, _) => self.symbol_set.bottom_right,
                BorderSymbol::EndCornerProlonged(_, _) => self.symbol_set.bottom_right,
                BorderSymbol::EndCornerCrossed(_, _, _, _) => self.symbol_set.bottom_right,
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => &" ",
            },
            Side::Left => match symbol {
                BorderSymbol::StartCornerRegular => self.symbol_set.top_left,
                BorderSymbol::StartCornerAngled(_, _) => self.symbol_set.top_left,
                BorderSymbol::StartCornerProlonged(_, _) => self.symbol_set.top_left,
                BorderSymbol::StartCornerCrossed(_, _, _, _) => self.symbol_set.top_left,
                BorderSymbol::SideRegular => self.symbol_set.vertical_left,
                BorderSymbol::SideOverlap(_, _) => self.symbol_set.vertical_left,
                BorderSymbol::SideOutward(_, _) => self.symbol_set.vertical_left,
                BorderSymbol::SideInward(_, _) => self.symbol_set.vertical_left,
                BorderSymbol::SideCrossed(_, _, _, _) => self.symbol_set.vertical_left,
                BorderSymbol::EndCornerRegular => self.symbol_set.bottom_left,
                BorderSymbol::EndCornerAngled(_, _) => self.symbol_set.bottom_left,
                BorderSymbol::EndCornerProlonged(_, _) => self.symbol_set.bottom_left,
                BorderSymbol::EndCornerCrossed(_, _, _, _) => self.symbol_set.bottom_left,
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => &" ",
            },
        }
    }
}

macro_rules! plain {
    () => {
        Plain | Rounded
    };
}

/// Plain border symbol set.
pub struct PlainSymbolSet;

impl BorderSymbolSet for PlainSymbolSet {
    // #[inline(always)]
    fn symbol(&self, side: Side, symbol: BorderSymbol) -> &'static str {
        use crate::Side::*;
        use BorderType::*;

        match side {
            Top => match symbol {
                BorderSymbol::StartCornerRegular => "┌",
                BorderSymbol::StartCornerAngled(_, Thick) => "┞",
                BorderSymbol::StartCornerAngled(_, _) => "├",
                BorderSymbol::StartCornerProlonged(_, Thick) => "┭",
                BorderSymbol::StartCornerProlonged(_, _) => "┬",
                BorderSymbol::StartCornerCrossed(_, Thick, _, Thick) => "╃",
                BorderSymbol::StartCornerCrossed(_, Thick, _, _) => "╀",
                BorderSymbol::StartCornerCrossed(_, _, _, Thick) => "┽",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "┼",
                BorderSymbol::SideRegular => "─",
                BorderSymbol::SideOverlap(_, _) => "─",
                BorderSymbol::SideOutward(_, Thick) => "┸",
                BorderSymbol::SideOutward(_, Double) => "╨",
                BorderSymbol::SideOutward(_, _) => "┴",
                BorderSymbol::SideInward(_, Thick) => "┰",
                BorderSymbol::SideInward(_, Double) => "╥",
                BorderSymbol::SideInward(_, _) => "┬",
                BorderSymbol::SideCrossed(_, Thick, _, Thick) => "╂",
                BorderSymbol::SideCrossed(_, Thick, _, _) => "╀",
                BorderSymbol::SideCrossed(_, _, _, Thick) => "╁",
                BorderSymbol::SideCrossed(_, Double, _, Double) => "╫",
                BorderSymbol::SideCrossed(_, _, _, _) => "┼",
                BorderSymbol::EndCornerRegular => "┐",
                BorderSymbol::EndCornerAngled(_, Thick) => "┦",
                BorderSymbol::EndCornerAngled(_, _) => "┤",
                BorderSymbol::EndCornerProlonged(_, Thick) => "┮",
                BorderSymbol::EndCornerProlonged(_, _) => "┬",
                BorderSymbol::EndCornerCrossed(_, Thick, _, Thick) => "╄",
                BorderSymbol::EndCornerCrossed(_, Thick, _, _) => "╀",
                BorderSymbol::EndCornerCrossed(_, _, _, Thick) => "┾",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "┼",
                // Double: outward, forward, inward, backward
                BorderSymbol::Cross(_, Double, _, Double, _, Double, _, Double) => "╬",
                BorderSymbol::Cross(_, Double, _, _, _, Double, _, _) => "╫",
                BorderSymbol::Cross(_, _, _, Double, _, _, _, Double) => "╪",
                // Thick: outward, forward, inward, backward
                BorderSymbol::Cross(_, Thick, _, Thick, _, Thick, _, Thick) => "╋",
                BorderSymbol::Cross(_, Thick, _, Thick, _, Thick, _, _) => "╊",
                BorderSymbol::Cross(_, Thick, _, Thick, _, _, _, Thick) => "╇",
                BorderSymbol::Cross(_, Thick, _, Thick, _, _, _, _) => "╄",
                BorderSymbol::Cross(_, Thick, _, _, _, Thick, _, Thick) => "╉",
                BorderSymbol::Cross(_, Thick, _, _, _, Thick, _, _) => "╂",
                BorderSymbol::Cross(_, Thick, _, _, _, _, _, Thick) => "╃",
                BorderSymbol::Cross(_, Thick, _, _, _, _, _, _) => "╀",
                BorderSymbol::Cross(_, _, _, Thick, _, Thick, _, Thick) => "╈",
                BorderSymbol::Cross(_, _, _, Thick, _, Thick, _, _) => "\u{2546}",
                BorderSymbol::Cross(_, _, _, Thick, _, _, _, Thick) => "┿",
                BorderSymbol::Cross(_, _, _, Thick, _, _, _, _) => "┾",
                BorderSymbol::Cross(_, _, _, _, _, Thick, _, Thick) => "╅",
                BorderSymbol::Cross(_, _, _, _, _, Thick, _, _) => "╁",
                BorderSymbol::Cross(_, _, _, _, _, _, _, Thick) => "┽",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "┼",
            },
            Bottom => match symbol {
                BorderSymbol::StartCornerRegular => "└",
                BorderSymbol::StartCornerAngled(_, Thick) => "┟",
                BorderSymbol::StartCornerAngled(_, _) => "├",
                BorderSymbol::StartCornerProlonged(_, Thick) => "┵",
                BorderSymbol::StartCornerProlonged(_, _) => "┴",
                BorderSymbol::StartCornerCrossed(_, Thick, _, Thick) => "╅",
                BorderSymbol::StartCornerCrossed(_, Thick, _, _) => "╁",
                BorderSymbol::StartCornerCrossed(_, _, _, Thick) => "┽",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "┼",
                BorderSymbol::SideRegular => "─",
                BorderSymbol::SideOverlap(_, _) => "─",
                BorderSymbol::SideOutward(_, Thick) => "┰",
                BorderSymbol::SideOutward(_, Double) => "╥",
                BorderSymbol::SideOutward(_, _) => "┬",
                BorderSymbol::SideInward(_, Thick) => "┸",
                BorderSymbol::SideInward(_, Double) => "╨",
                BorderSymbol::SideInward(_, _) => "┴",
                BorderSymbol::SideCrossed(_, Thick, _, Thick) => "╂",
                BorderSymbol::SideCrossed(_, Thick, _, _) => "╁",
                BorderSymbol::SideCrossed(_, _, _, Thick) => "╀",
                BorderSymbol::SideCrossed(_, Double, _, Double) => "╫",
                BorderSymbol::SideCrossed(_, _, _, _) => "┼",
                BorderSymbol::EndCornerRegular => "┘",
                BorderSymbol::EndCornerAngled(_, Thick) => "┧",
                BorderSymbol::EndCornerAngled(_, _) => "┤",
                BorderSymbol::EndCornerProlonged(_, Thick) => "┶",
                BorderSymbol::EndCornerProlonged(_, _) => "┴",
                BorderSymbol::EndCornerCrossed(_, Thick, _, Thick) => "╆",
                BorderSymbol::EndCornerCrossed(_, Thick, _, _) => "╁",
                BorderSymbol::EndCornerCrossed(_, _, _, Thick) => "┾",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "┼",
                // Double: outward, forward, inward, backward
                BorderSymbol::Cross(_, Double, _, Double, _, Double, _, Double) => "╬",
                BorderSymbol::Cross(_, Double, _, _, _, Double, _, _) => "╫",
                BorderSymbol::Cross(_, _, _, Double, _, _, _, Double) => "╪",
                // Thick: outward, forward, inward, backward
                BorderSymbol::Cross(_, Thick, _, Thick, _, Thick, _, Thick) => "╋",
                BorderSymbol::Cross(_, Thick, _, Thick, _, Thick, _, _) => "╊",
                BorderSymbol::Cross(_, Thick, _, Thick, _, _, _, Thick) => "╈",
                BorderSymbol::Cross(_, Thick, _, Thick, _, _, _, _) => "\u{2546}",
                BorderSymbol::Cross(_, Thick, _, _, _, Thick, _, Thick) => "╉",
                BorderSymbol::Cross(_, Thick, _, _, _, Thick, _, _) => "╂",
                BorderSymbol::Cross(_, Thick, _, _, _, _, _, Thick) => "╅",
                BorderSymbol::Cross(_, Thick, _, _, _, _, _, _) => "╁",
                BorderSymbol::Cross(_, _, _, Thick, _, Thick, _, Thick) => "╇",
                BorderSymbol::Cross(_, _, _, Thick, _, Thick, _, _) => "╄",
                BorderSymbol::Cross(_, _, _, Thick, _, _, _, Thick) => "┿",
                BorderSymbol::Cross(_, _, _, Thick, _, _, _, _) => "┾",
                BorderSymbol::Cross(_, _, _, _, _, Thick, _, Thick) => "╃",
                BorderSymbol::Cross(_, _, _, _, _, Thick, _, _) => "╀",
                BorderSymbol::Cross(_, _, _, _, _, _, _, Thick) => "┽",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "┼",
            },
            Right => match symbol {
                BorderSymbol::StartCornerRegular => "┐",
                BorderSymbol::StartCornerAngled(_, Thick) => "┮",
                BorderSymbol::StartCornerAngled(_, _) => "┬",
                BorderSymbol::StartCornerProlonged(_, Thick) => "┦",
                BorderSymbol::StartCornerProlonged(_, _) => "┤",
                BorderSymbol::StartCornerCrossed(_, Thick, _, Thick) => "╄",
                BorderSymbol::StartCornerCrossed(_, Thick, _, _) => "┾",
                BorderSymbol::StartCornerCrossed(_, _, _, Thick) => "╀",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "┼",
                BorderSymbol::SideRegular => "│",
                BorderSymbol::SideOverlap(_, _) => "│",
                BorderSymbol::SideOutward(_, Thick) => "┝",
                BorderSymbol::SideOutward(_, Double) => "╞",
                BorderSymbol::SideOutward(_, _) => "├",
                BorderSymbol::SideInward(_, Thick) => "┥",
                BorderSymbol::SideInward(_, Double) => "╡",
                BorderSymbol::SideInward(_, _) => "┤",
                BorderSymbol::SideCrossed(_, Thick, _, Thick) => "┿",
                BorderSymbol::SideCrossed(_, Thick, _, _) => "┾",
                BorderSymbol::SideCrossed(_, _, _, Thick) => "┽",
                BorderSymbol::SideCrossed(_, Double, _, Double) => "╪",
                BorderSymbol::SideCrossed(_, _, _, _) => "┼",
                BorderSymbol::EndCornerRegular => "┘",
                BorderSymbol::EndCornerAngled(_, Thick) => "┶",
                BorderSymbol::EndCornerAngled(_, _) => "┴",
                BorderSymbol::EndCornerProlonged(_, Thick) => "┧",
                BorderSymbol::EndCornerProlonged(_, _) => "┤",
                BorderSymbol::EndCornerCrossed(_, Thick, _, Thick) => "╆",
                BorderSymbol::EndCornerCrossed(_, Thick, _, _) => "┾",
                BorderSymbol::EndCornerCrossed(_, _, _, Thick) => "╁",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "┼",
                // Double: outward, forward, inward, backward
                BorderSymbol::Cross(_, Double, _, Double, _, Double, _, Double) => "╬",
                BorderSymbol::Cross(_, Double, _, _, _, Double, _, _) => "╪",
                BorderSymbol::Cross(_, _, _, Double, _, _, _, Double) => "╫",
                // Thick: outward, forward, inward, backward
                BorderSymbol::Cross(_, Thick, _, Thick, _, Thick, _, Thick) => "╋",
                BorderSymbol::Cross(_, Thick, _, Thick, _, Thick, _, _) => "╈",
                BorderSymbol::Cross(_, Thick, _, Thick, _, _, _, Thick) => "╊",
                BorderSymbol::Cross(_, Thick, _, Thick, _, _, _, _) => "\u{2546}",
                BorderSymbol::Cross(_, Thick, _, _, _, Thick, _, Thick) => "╇",
                BorderSymbol::Cross(_, Thick, _, _, _, Thick, _, _) => "┿",
                BorderSymbol::Cross(_, Thick, _, _, _, _, _, Thick) => "╄",
                BorderSymbol::Cross(_, Thick, _, _, _, _, _, _) => "┾",
                BorderSymbol::Cross(_, _, _, Thick, _, Thick, _, Thick) => "╉",
                BorderSymbol::Cross(_, _, _, Thick, _, Thick, _, _) => "╅",
                BorderSymbol::Cross(_, _, _, Thick, _, _, _, Thick) => "╂",
                BorderSymbol::Cross(_, _, _, Thick, _, _, _, _) => "╁",
                BorderSymbol::Cross(_, _, _, _, _, Thick, _, Thick) => "╃",
                BorderSymbol::Cross(_, _, _, _, _, Thick, _, _) => "┽",
                BorderSymbol::Cross(_, _, _, _, _, _, _, Thick) => "╀",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "┼",
            },
            Left => match symbol {
                BorderSymbol::StartCornerRegular => "┌",
                BorderSymbol::StartCornerAngled(_, Thick) => "┭",
                BorderSymbol::StartCornerAngled(_, _) => "┬",
                BorderSymbol::StartCornerProlonged(_, Thick) => "┞",
                BorderSymbol::StartCornerProlonged(_, _) => "├",
                BorderSymbol::StartCornerCrossed(_, Thick, _, Thick) => "╃",
                BorderSymbol::StartCornerCrossed(_, Thick, _, _) => "┽",
                BorderSymbol::StartCornerCrossed(_, _, _, Thick) => "╀",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "┼",
                BorderSymbol::SideRegular => "│",
                BorderSymbol::SideOverlap(_, _) => "│",
                BorderSymbol::SideOutward(_, Thick) => "┥",
                BorderSymbol::SideOutward(_, Double) => "╡",
                BorderSymbol::SideOutward(_, _) => "┤",
                BorderSymbol::SideInward(_, Thick) => "┝",
                BorderSymbol::SideInward(_, Double) => "╞",
                BorderSymbol::SideInward(_, _) => "├",
                BorderSymbol::SideCrossed(_, Thick, _, Thick) => "┿",
                BorderSymbol::SideCrossed(_, Thick, _, _) => "┽",
                BorderSymbol::SideCrossed(_, _, _, Thick) => "┾",
                BorderSymbol::SideCrossed(_, Double, _, Double) => "╪",
                BorderSymbol::SideCrossed(_, _, _, _) => "┼",
                BorderSymbol::EndCornerRegular => "└",
                BorderSymbol::EndCornerAngled(_, Thick) => "┵",
                BorderSymbol::EndCornerAngled(_, _) => "┴",
                BorderSymbol::EndCornerProlonged(_, Thick) => "┟",
                BorderSymbol::EndCornerProlonged(_, _) => "├",
                BorderSymbol::EndCornerCrossed(_, Thick, _, Thick) => "╆",
                BorderSymbol::EndCornerCrossed(_, Thick, _, _) => "┽",
                BorderSymbol::EndCornerCrossed(_, _, _, Thick) => "╁",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "┼",
                // Double: outward, forward, inward, backward
                BorderSymbol::Cross(_, Double, _, Double, _, Double, _, Double) => "╬",
                BorderSymbol::Cross(_, Double, _, _, _, Double, _, _) => "╪",
                BorderSymbol::Cross(_, _, _, Double, _, _, _, Double) => "╫",
                // Thick: outward, forward, inward, backward
                BorderSymbol::Cross(_, Thick, _, Thick, _, Thick, _, Thick) => "╋",
                BorderSymbol::Cross(_, Thick, _, Thick, _, Thick, _, _) => "╈",
                BorderSymbol::Cross(_, _, _, Thick, _, Thick, _, Thick) => "╊",
                BorderSymbol::Cross(_, _, _, Thick, _, Thick, _, _) => "\u{2546}",
                BorderSymbol::Cross(_, Thick, _, _, _, Thick, _, Thick) => "╇",
                BorderSymbol::Cross(_, Thick, _, _, _, Thick, _, _) => "┿",
                BorderSymbol::Cross(_, _, _, _, _, Thick, _, Thick) => "╄",
                BorderSymbol::Cross(_, _, _, _, _, Thick, _, _) => "┾",
                BorderSymbol::Cross(_, Thick, _, Thick, _, _, _, Thick) => "╉",
                BorderSymbol::Cross(_, Thick, _, Thick, _, _, _, _) => "╅",
                BorderSymbol::Cross(_, _, _, Thick, _, _, _, Thick) => "╂",
                BorderSymbol::Cross(_, _, _, Thick, _, _, _, _) => "╁",
                BorderSymbol::Cross(_, Thick, _, _, _, _, _, Thick) => "╃",
                BorderSymbol::Cross(_, Thick, _, _, _, _, _, _) => "┽",
                BorderSymbol::Cross(_, _, _, _, _, _, _, Thick) => "╀",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "┼",
            },
        }
    }
}

/// Rounded border symbol set.
pub struct RoundedSymbolSet;

impl BorderSymbolSet for RoundedSymbolSet {
    // #[inline(always)]
    fn symbol(&self, side: Side, symbol: BorderSymbol) -> &'static str {
        use crate::Side::*;

        match side {
            Top => match symbol {
                BorderSymbol::StartCornerRegular => "╭",
                BorderSymbol::EndCornerRegular => "╮",
                _ => PlainSymbolSet.symbol(side, symbol),
            },
            Bottom => match symbol {
                BorderSymbol::StartCornerRegular => "╰",
                BorderSymbol::EndCornerRegular => "╯",
                _ => PlainSymbolSet.symbol(side, symbol),
            },
            Right => match symbol {
                BorderSymbol::StartCornerRegular => "╮",
                BorderSymbol::EndCornerRegular => "╯",
                _ => PlainSymbolSet.symbol(side, symbol),
            },
            Left => match symbol {
                BorderSymbol::StartCornerRegular => "╭",
                BorderSymbol::EndCornerRegular => "╰",
                _ => PlainSymbolSet.symbol(side, symbol),
            },
        }
    }
}

/// Double border symbol set.
pub struct DoubleSymbolSet;

impl BorderSymbolSet for DoubleSymbolSet {
    // #[inline(always)]
    fn symbol(&self, side: Side, symbol: BorderSymbol) -> &'static str {
        use crate::Side::*;
        use BorderType::*;

        match side {
            Top => match symbol {
                BorderSymbol::StartCornerRegular => "╔",
                BorderSymbol::StartCornerAngled(_, _) => "╠",
                BorderSymbol::StartCornerProlonged(_, _) => "╦",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "╬",
                BorderSymbol::SideRegular => "═",
                BorderSymbol::SideOverlap(_, _) => "═",
                BorderSymbol::SideOutward(_, plain!()) => "╧",
                BorderSymbol::SideOutward(_, _) => "╩",
                BorderSymbol::SideInward(_, plain!()) => "╤",
                BorderSymbol::SideInward(_, _) => "╦",
                BorderSymbol::SideCrossed(_, plain!(), _, plain!()) => "╪",
                BorderSymbol::SideCrossed(_, _, _, _) => "╬",
                BorderSymbol::EndCornerRegular => "╗",
                BorderSymbol::EndCornerAngled(_, _) => "╣",
                BorderSymbol::EndCornerProlonged(_, _) => "╦",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "╬",
                // outward, forward, inward, backward
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, plain!(), _, plain!()) => "┼",
                BorderSymbol::Cross(_, plain!(), _, _, _, plain!(), _, _) => "╪",
                BorderSymbol::Cross(_, _, _, plain!(), _, _, _, plain!()) => "╫",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "╬",
            },
            Bottom => match symbol {
                BorderSymbol::StartCornerRegular => "╚",
                BorderSymbol::StartCornerAngled(_, _) => "╠",
                BorderSymbol::StartCornerProlonged(_, _) => "╩",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "╬",
                BorderSymbol::SideRegular => "═",
                BorderSymbol::SideOverlap(_, _) => "═",
                BorderSymbol::SideOutward(_, plain!()) => "╤",
                BorderSymbol::SideOutward(_, _) => "╦",
                BorderSymbol::SideInward(_, plain!()) => "╧",
                BorderSymbol::SideInward(_, _) => "╩",
                BorderSymbol::SideCrossed(_, plain!(), _, plain!()) => "╪",
                BorderSymbol::SideCrossed(_, _, _, _) => "╬",
                BorderSymbol::EndCornerRegular => "╝",
                BorderSymbol::EndCornerAngled(_, _) => "╣",
                BorderSymbol::EndCornerProlonged(_, _) => "╩",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "╬",
                // outward, forward, inward, backward
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, plain!(), _, plain!()) => "┼",
                BorderSymbol::Cross(_, plain!(), _, _, _, plain!(), _, _) => "╪",
                BorderSymbol::Cross(_, _, _, plain!(), _, _, _, plain!()) => "╫",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "╬",
            },
            Right => match symbol {
                BorderSymbol::StartCornerRegular => "╗",
                BorderSymbol::StartCornerAngled(_, _) => "╦",
                BorderSymbol::StartCornerProlonged(_, _) => "╣",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "╬",
                BorderSymbol::SideRegular => "║",
                BorderSymbol::SideOverlap(_, _) => "║",
                BorderSymbol::SideOutward(_, plain!()) => "╟",
                BorderSymbol::SideOutward(_, _) => "╠",
                BorderSymbol::SideInward(_, plain!()) => "╢",
                BorderSymbol::SideInward(_, _) => "╣",
                BorderSymbol::SideCrossed(_, plain!(), _, plain!()) => "╫",
                BorderSymbol::SideCrossed(_, _, _, _) => "╬",
                BorderSymbol::EndCornerRegular => "╝",
                BorderSymbol::EndCornerAngled(_, _) => "╩",
                BorderSymbol::EndCornerProlonged(_, _) => "╣",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "╬",
                // outward, forward, inward, backward
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, plain!(), _, plain!()) => "┼",
                BorderSymbol::Cross(_, plain!(), _, _, _, plain!(), _, _) => "╫",
                BorderSymbol::Cross(_, _, _, plain!(), _, _, _, plain!()) => "╪",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "╬",
            },
            Left => match symbol {
                BorderSymbol::StartCornerRegular => "╔",
                BorderSymbol::StartCornerAngled(_, _) => "╦",
                BorderSymbol::StartCornerProlonged(_, _) => "╠",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "╬",
                BorderSymbol::SideRegular => "║",
                BorderSymbol::SideOverlap(_, _) => "║",
                BorderSymbol::SideOutward(_, plain!()) => "╢",
                BorderSymbol::SideOutward(_, _) => "╣",
                BorderSymbol::SideInward(_, plain!()) => "╟",
                BorderSymbol::SideInward(_, _) => "╠",
                BorderSymbol::SideCrossed(_, plain!(), _, plain!()) => "╫",
                BorderSymbol::SideCrossed(_, _, _, _) => "╬",
                BorderSymbol::EndCornerRegular => "╚",
                BorderSymbol::EndCornerAngled(_, _) => "╩",
                BorderSymbol::EndCornerProlonged(_, _) => "╠",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "╬",
                // outward, forward, inward, backward
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, plain!(), _, plain!()) => "┼",
                BorderSymbol::Cross(_, plain!(), _, _, _, plain!(), _, _) => "╫",
                BorderSymbol::Cross(_, _, _, plain!(), _, _, _, plain!()) => "╪",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "╬",
            },
        }
    }
}

/// Thick border symbol set.
pub struct ThickSymbolSet;

impl BorderSymbolSet for ThickSymbolSet {
    // #[inline(always)]
    fn symbol(&self, side: Side, symbol: BorderSymbol) -> &'static str {
        use crate::Side::*;
        use BorderType::*;

        match side {
            Top => match symbol {
                BorderSymbol::StartCornerRegular => "┏",
                BorderSymbol::StartCornerAngled(_, plain!()) => "┢",
                BorderSymbol::StartCornerAngled(_, _) => "┣",
                BorderSymbol::StartCornerProlonged(_, plain!()) => "┲",
                BorderSymbol::StartCornerProlonged(_, _) => "┳",
                BorderSymbol::StartCornerCrossed(_, plain!(), _, plain!()) => "\u{2546}",
                BorderSymbol::StartCornerCrossed(_, plain!(), _, _) => "╈",
                BorderSymbol::StartCornerCrossed(_, _, _, plain!()) => "╊",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "╋",
                BorderSymbol::SideRegular => "━",
                BorderSymbol::SideOverlap(_, _) => "━",
                BorderSymbol::SideOutward(_, plain!()) => "┷",
                BorderSymbol::SideOutward(_, _) => "┻",
                BorderSymbol::SideInward(_, plain!()) => "┯",
                BorderSymbol::SideInward(_, _) => "┳",
                BorderSymbol::SideCrossed(_, plain!(), _, plain!()) => "┿",
                BorderSymbol::SideCrossed(_, plain!(), _, _) => "╈",
                BorderSymbol::SideCrossed(_, _, _, plain!()) => "╇",
                BorderSymbol::SideCrossed(_, _, _, _) => "╋",
                BorderSymbol::EndCornerRegular => "┓",
                BorderSymbol::EndCornerAngled(_, Plain) => "┪",
                BorderSymbol::EndCornerAngled(_, _) => "┫",
                BorderSymbol::EndCornerProlonged(_, Plain) => "┱",
                BorderSymbol::EndCornerProlonged(_, _) => "┳",
                BorderSymbol::EndCornerCrossed(_, plain!(), _, plain!()) => "╅",
                BorderSymbol::EndCornerCrossed(_, plain!(), _, _) => "╈",
                BorderSymbol::EndCornerCrossed(_, _, _, plain!()) => "╉",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "╋",
                // outward, forward, inward, backward
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, plain!(), _, plain!()) => "┼",
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, plain!(), _, _) => "┽",
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, _, _, plain!()) => "╁",
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, _, _, _) => "╅",
                BorderSymbol::Cross(_, plain!(), _, _, _, plain!(), _, plain!()) => "┾",
                BorderSymbol::Cross(_, plain!(), _, _, _, plain!(), _, _) => "┿",
                BorderSymbol::Cross(_, plain!(), _, _, _, _, _, plain!()) => "\u{2546}",
                BorderSymbol::Cross(_, plain!(), _, _, _, _, _, _) => "╈",
                BorderSymbol::Cross(_, _, _, plain!(), _, plain!(), _, plain!()) => "╀",
                BorderSymbol::Cross(_, _, _, plain!(), _, plain!(), _, _) => "╃",
                BorderSymbol::Cross(_, _, _, plain!(), _, _, _, plain!()) => "╂",
                BorderSymbol::Cross(_, _, _, plain!(), _, _, _, _) => "╉",
                BorderSymbol::Cross(_, _, _, _, _, plain!(), _, plain!()) => "╄",
                BorderSymbol::Cross(_, _, _, _, _, plain!(), _, _) => "╇",
                BorderSymbol::Cross(_, _, _, _, _, _, _, plain!()) => "╊",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "╋",
            },
            Bottom => match symbol {
                BorderSymbol::StartCornerRegular => "┗",
                BorderSymbol::StartCornerAngled(_, plain!()) => "┡",
                BorderSymbol::StartCornerAngled(_, _) => "┣",
                BorderSymbol::StartCornerProlonged(_, plain!()) => "┺",
                BorderSymbol::StartCornerProlonged(_, _) => "┹",
                BorderSymbol::StartCornerCrossed(_, plain!(), _, plain!()) => "╄",
                BorderSymbol::StartCornerCrossed(_, plain!(), _, _) => "╇",
                BorderSymbol::StartCornerCrossed(_, _, _, plain!()) => "╊",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "╋",
                BorderSymbol::SideRegular => "━",
                BorderSymbol::SideOverlap(_, _) => "━",
                BorderSymbol::SideOutward(_, plain!()) => "┯",
                BorderSymbol::SideOutward(_, _) => "┳",
                BorderSymbol::SideInward(_, plain!()) => "┷",
                BorderSymbol::SideInward(_, _) => "┻",
                BorderSymbol::SideCrossed(_, plain!(), _, plain!()) => "┿",
                BorderSymbol::SideCrossed(_, plain!(), _, _) => "╇",
                BorderSymbol::SideCrossed(_, _, _, plain!()) => "╈",
                BorderSymbol::SideCrossed(_, _, _, _) => "╋",
                BorderSymbol::EndCornerRegular => "┛",
                BorderSymbol::EndCornerAngled(_, plain!()) => "┩",
                BorderSymbol::EndCornerAngled(_, _) => "┫",
                BorderSymbol::EndCornerProlonged(_, plain!()) => "┹",
                BorderSymbol::EndCornerProlonged(_, _) => "┻",
                BorderSymbol::EndCornerCrossed(_, plain!(), _, plain!()) => "╃",
                BorderSymbol::EndCornerCrossed(_, plain!(), _, _) => "╇",
                BorderSymbol::EndCornerCrossed(_, _, _, plain!()) => "╉",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "╋",
                // outward, forward, inward, backward
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, plain!(), _, plain!()) => "┼",
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, plain!(), _, _) => "┽",
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, _, _, plain!()) => "╀",
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, _, _, _) => "╃",
                BorderSymbol::Cross(_, plain!(), _, _, _, plain!(), _, plain!()) => "┾",
                BorderSymbol::Cross(_, plain!(), _, _, _, plain!(), _, _) => "┿",
                BorderSymbol::Cross(_, plain!(), _, _, _, _, _, plain!()) => "╄",
                BorderSymbol::Cross(_, plain!(), _, _, _, _, _, _) => "╇",
                BorderSymbol::Cross(_, _, _, plain!(), _, plain!(), _, plain!()) => "╁",
                BorderSymbol::Cross(_, _, _, plain!(), _, plain!(), _, _) => "╅",
                BorderSymbol::Cross(_, _, _, plain!(), _, _, _, plain!()) => "╂",
                BorderSymbol::Cross(_, _, _, plain!(), _, _, _, _) => "╉",
                BorderSymbol::Cross(_, _, _, _, _, plain!(), _, plain!()) => "\u{2546}",
                BorderSymbol::Cross(_, _, _, _, _, plain!(), _, _) => "╈",
                BorderSymbol::Cross(_, _, _, _, _, _, _, plain!()) => "╊",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "╋",
            },
            Right => match symbol {
                BorderSymbol::StartCornerRegular => "┓",
                BorderSymbol::StartCornerAngled(_, plain!()) => "┱",
                BorderSymbol::StartCornerAngled(_, _) => "┳",
                BorderSymbol::StartCornerProlonged(_, plain!()) => "┪",
                BorderSymbol::StartCornerProlonged(_, _) => "┫",
                BorderSymbol::StartCornerCrossed(_, plain!(), _, plain!()) => "╅",
                BorderSymbol::StartCornerCrossed(_, plain!(), _, _) => "╈",
                BorderSymbol::StartCornerCrossed(_, _, _, plain!()) => "╉",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "╋",
                BorderSymbol::SideRegular => "┃",
                BorderSymbol::SideOverlap(_, _) => "┃",
                BorderSymbol::SideOutward(_, plain!()) => "┠",
                BorderSymbol::SideOutward(_, _) => "┣",
                BorderSymbol::SideInward(_, plain!()) => "┨",
                BorderSymbol::SideInward(_, _) => "┫",
                BorderSymbol::SideCrossed(_, plain!(), _, plain!()) => "╂",
                BorderSymbol::SideCrossed(_, plain!(), _, _) => "╉",
                BorderSymbol::SideCrossed(_, _, _, plain!()) => "╊",
                BorderSymbol::SideCrossed(_, _, _, _) => "╋",
                BorderSymbol::EndCornerRegular => "┛",
                BorderSymbol::EndCornerAngled(_, plain!()) => "┹",
                BorderSymbol::EndCornerAngled(_, _) => "┻",
                BorderSymbol::EndCornerProlonged(_, plain!()) => "┩",
                BorderSymbol::EndCornerProlonged(_, _) => "┫",
                BorderSymbol::EndCornerCrossed(_, plain!(), _, plain!()) => "╃",
                BorderSymbol::EndCornerCrossed(_, plain!(), _, _) => "╉",
                BorderSymbol::EndCornerCrossed(_, _, _, plain!()) => "╇",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "╋",
                // outward, forward, inward, backward
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, plain!(), _, plain!()) => "┼",
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, plain!(), _, _) => "╀",
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, _, _, plain!()) => "┽",
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, _, _, _) => "╃",
                BorderSymbol::Cross(_, plain!(), _, _, _, plain!(), _, plain!()) => "╁",
                BorderSymbol::Cross(_, plain!(), _, _, _, plain!(), _, _) => "╂",
                BorderSymbol::Cross(_, plain!(), _, _, _, _, _, plain!()) => "╅",
                BorderSymbol::Cross(_, plain!(), _, _, _, _, _, _) => "╉",
                BorderSymbol::Cross(_, _, _, plain!(), _, plain!(), _, plain!()) => "┾",
                BorderSymbol::Cross(_, _, _, plain!(), _, plain!(), _, _) => "╄",
                BorderSymbol::Cross(_, _, _, plain!(), _, _, _, plain!()) => "┿",
                BorderSymbol::Cross(_, _, _, plain!(), _, _, _, _) => "╇",
                BorderSymbol::Cross(_, _, _, _, _, plain!(), _, plain!()) => "\u{2546}",
                BorderSymbol::Cross(_, _, _, _, _, plain!(), _, _) => "╊",
                BorderSymbol::Cross(_, _, _, _, _, _, _, plain!()) => "╈",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "╋",
            },
            Left => match symbol {
                BorderSymbol::StartCornerRegular => "┏",
                BorderSymbol::StartCornerAngled(_, plain!()) => "┲",
                BorderSymbol::StartCornerAngled(_, _) => "┳",
                BorderSymbol::StartCornerProlonged(_, plain!()) => "┢",
                BorderSymbol::StartCornerProlonged(_, _) => "┣",
                BorderSymbol::StartCornerCrossed(_, plain!(), _, plain!()) => "\u{2546}",
                BorderSymbol::StartCornerCrossed(_, plain!(), _, _) => "╊",
                BorderSymbol::StartCornerCrossed(_, _, _, plain!()) => "╈",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "╋",
                BorderSymbol::SideRegular => "┃",
                BorderSymbol::SideOverlap(_, _) => "┃",
                BorderSymbol::SideOutward(_, plain!()) => "┨",
                BorderSymbol::SideOutward(_, _) => "┫",
                BorderSymbol::SideInward(_, plain!()) => "┠",
                BorderSymbol::SideInward(_, _) => "┣",
                BorderSymbol::SideCrossed(_, plain!(), _, plain!()) => "╂",
                BorderSymbol::SideCrossed(_, plain!(), _, _) => "╊",
                BorderSymbol::SideCrossed(_, _, _, plain!()) => "╉",
                BorderSymbol::SideCrossed(_, _, _, _) => "╋",
                BorderSymbol::EndCornerRegular => "┗",
                BorderSymbol::EndCornerAngled(_, plain!()) => "┺",
                BorderSymbol::EndCornerAngled(_, _) => "┻",
                BorderSymbol::EndCornerProlonged(_, plain!()) => "┡",
                BorderSymbol::EndCornerProlonged(_, _) => "┫",
                BorderSymbol::EndCornerCrossed(_, plain!(), _, plain!()) => "╄",
                BorderSymbol::EndCornerCrossed(_, plain!(), _, _) => "╊",
                BorderSymbol::EndCornerCrossed(_, _, _, plain!()) => "╇",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "╋",
                // outward, forward, inward, backward
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, plain!(), _, plain!()) => "┼",
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, plain!(), _, _) => "╀",
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, _, _, plain!()) => "┾",
                BorderSymbol::Cross(_, plain!(), _, plain!(), _, _, _, _) => "╄",
                BorderSymbol::Cross(_, plain!(), _, _, _, plain!(), _, plain!()) => "╁",
                BorderSymbol::Cross(_, plain!(), _, _, _, plain!(), _, _) => "╂",
                BorderSymbol::Cross(_, plain!(), _, _, _, _, _, plain!()) => "\u{2546}",
                BorderSymbol::Cross(_, plain!(), _, _, _, _, _, _) => "╊",
                BorderSymbol::Cross(_, _, _, plain!(), _, plain!(), _, plain!()) => "┽",
                BorderSymbol::Cross(_, _, _, plain!(), _, plain!(), _, _) => "╃",
                BorderSymbol::Cross(_, _, _, plain!(), _, _, _, plain!()) => "┿",
                BorderSymbol::Cross(_, _, _, plain!(), _, _, _, _) => "╇",
                BorderSymbol::Cross(_, _, _, _, _, plain!(), _, plain!()) => "╅",
                BorderSymbol::Cross(_, _, _, _, _, plain!(), _, _) => "╉",
                BorderSymbol::Cross(_, _, _, _, _, _, _, plain!()) => "╈",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "╋",
            },
        }
    }
}

/// Quadrant inside border symbol set.
pub struct QuadrantInsideSymbolSet;

impl BorderSymbolSet for QuadrantInsideSymbolSet {
    // #[inline(always)]
    fn symbol(&self, side: Side, symbol: BorderSymbol) -> &'static str {
        use crate::Side::*;
        use BorderType::*;

        match side {
            Top => match symbol {
                BorderSymbol::StartCornerRegular => "▗",
                BorderSymbol::StartCornerAngled(Left, QuadrantOutside) => "▙",
                BorderSymbol::StartCornerAngled(Right, QuadrantOutside) => "▐",
                BorderSymbol::StartCornerAngled(Left, QuadrantInside) => "▐",
                BorderSymbol::StartCornerAngled(Right, QuadrantInside) => "▙",
                BorderSymbol::StartCornerAngled(_, _) => "▐",
                BorderSymbol::StartCornerProlonged(Top, QuadrantOutside) => "▜",
                BorderSymbol::StartCornerProlonged(Bottom, QuadrantOutside) => "▄",
                BorderSymbol::StartCornerProlonged(Top, QuadrantInside) => "▄",
                BorderSymbol::StartCornerProlonged(Bottom, QuadrantInside) => "▜",
                BorderSymbol::StartCornerProlonged(_, _) => "▄",

                BorderSymbol::StartCornerCrossed(Right, QuadrantOutside, Top, QuadrantOutside) => {
                    "▜"
                }
                BorderSymbol::StartCornerCrossed(
                    Right,
                    QuadrantOutside,
                    Bottom,
                    QuadrantOutside,
                ) => "▟",
                BorderSymbol::StartCornerCrossed(
                    Left,
                    QuadrantOutside,
                    Bottom,
                    QuadrantOutside,
                ) => "▙",
                BorderSymbol::StartCornerCrossed(Left, QuadrantOutside, Top, QuadrantOutside) => {
                    "▚"
                }
                BorderSymbol::StartCornerCrossed(_, QuadrantOutside, _, QuadrantOutside) => "▗",
                BorderSymbol::StartCornerCrossed(Left, QuadrantInside, Bottom, QuadrantInside) => {
                    "▜"
                }
                BorderSymbol::StartCornerCrossed(Left, QuadrantInside, Top, QuadrantInside) => "▟",
                BorderSymbol::StartCornerCrossed(Right, QuadrantInside, Top, QuadrantInside) => "▙",
                BorderSymbol::StartCornerCrossed(Right, QuadrantInside, Bottom, QuadrantInside) => {
                    "▚"
                }
                BorderSymbol::StartCornerCrossed(_, QuadrantInside, _, QuadrantInside) => "▗",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "▟",
                BorderSymbol::SideRegular => "▄",
                BorderSymbol::SideOverlap(_, QuadrantInside) => "█",
                BorderSymbol::SideOverlap(_, _) => "▄",
                BorderSymbol::SideOutward(Left, QuadrantOutside) => "▙",
                BorderSymbol::SideOutward(Right, QuadrantOutside) => "▟",
                BorderSymbol::SideOutward(Left, _) => "▟",
                BorderSymbol::SideOutward(Right, _) => "▙",
                BorderSymbol::SideOutward(_, _) => "▄",
                BorderSymbol::SideInward(_, _) => "▄",
                BorderSymbol::SideCrossed(Left, _, _, _) => "▙",
                BorderSymbol::SideCrossed(Right, _, _, _) => "▟",
                BorderSymbol::SideCrossed(_, _, _, _) => "▄",
                BorderSymbol::EndCornerRegular => "▖",
                BorderSymbol::EndCornerAngled(Left, QuadrantOutside) => "▌",
                BorderSymbol::EndCornerAngled(Right, QuadrantOutside) => "▟",
                BorderSymbol::EndCornerAngled(Left, QuadrantInside) => "▟",
                BorderSymbol::EndCornerAngled(Right, QuadrantInside) => "▌",
                BorderSymbol::EndCornerAngled(_, _) => "▌",
                BorderSymbol::EndCornerProlonged(Top, QuadrantOutside) => "▛",
                BorderSymbol::EndCornerProlonged(Bottom, QuadrantOutside) => "▄",
                BorderSymbol::EndCornerProlonged(Top, QuadrantInside) => "▄",
                BorderSymbol::EndCornerProlonged(Bottom, QuadrantInside) => "▛",
                BorderSymbol::EndCornerProlonged(_, _) => "▄",
                BorderSymbol::EndCornerCrossed(Right, QuadrantOutside, Top, QuadrantOutside) => "▞",
                BorderSymbol::EndCornerCrossed(Right, QuadrantOutside, Bottom, QuadrantOutside) => {
                    "▟"
                }
                BorderSymbol::EndCornerCrossed(Left, QuadrantOutside, Top, QuadrantOutside) => "▛",
                BorderSymbol::EndCornerCrossed(Left, QuadrantOutside, Bottom, QuadrantOutside) => {
                    "▙"
                }
                BorderSymbol::EndCornerCrossed(_, QuadrantOutside, _, QuadrantOutside) => "▖",
                BorderSymbol::EndCornerCrossed(Right, QuadrantInside, Top, QuadrantInside) => "▙",
                BorderSymbol::EndCornerCrossed(Right, QuadrantInside, Bottom, QuadrantInside) => {
                    "▛"
                }
                BorderSymbol::EndCornerCrossed(Left, QuadrantInside, Top, QuadrantInside) => "▟",
                BorderSymbol::EndCornerCrossed(Left, QuadrantInside, Bottom, QuadrantInside) => "▞",
                BorderSymbol::EndCornerCrossed(_, QuadrantInside, _, QuadrantInside) => "▖",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "▙",

                BorderSymbol::Cross(Left, _, Bottom, _, _, _, _, _) => "▜",
                BorderSymbol::Cross(Left, _, Top, _, _, _, _, _) => "▟",
                BorderSymbol::Cross(Right, _, Bottom, _, _, _, _, _) => "▛",
                BorderSymbol::Cross(Right, _, Top, _, _, _, _, _) => "▙",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "▄",
            },
            Bottom => match symbol {
                BorderSymbol::StartCornerRegular => "▝",
                BorderSymbol::StartCornerAngled(Left, QuadrantOutside) => "▛",
                BorderSymbol::StartCornerAngled(Right, QuadrantOutside) => "▐",
                BorderSymbol::StartCornerAngled(Left, QuadrantInside) => "▐",
                BorderSymbol::StartCornerAngled(Right, QuadrantInside) => "▛",
                BorderSymbol::StartCornerAngled(_, _) => "▐",
                BorderSymbol::StartCornerProlonged(Top, QuadrantOutside) => "▀",
                BorderSymbol::StartCornerProlonged(Bottom, QuadrantOutside) => "▟",
                BorderSymbol::StartCornerProlonged(Top, QuadrantInside) => "▟",
                BorderSymbol::StartCornerProlonged(Bottom, QuadrantInside) => "▀",
                BorderSymbol::StartCornerProlonged(_, _) => "▀",
                BorderSymbol::StartCornerCrossed(Right, QuadrantOutside, Top, QuadrantOutside) => {
                    "▜"
                }
                BorderSymbol::StartCornerCrossed(
                    Right,
                    QuadrantOutside,
                    Bottom,
                    QuadrantOutside,
                ) => "▟",
                BorderSymbol::StartCornerCrossed(Left, QuadrantOutside, Top, QuadrantOutside) => {
                    "▛"
                }
                BorderSymbol::StartCornerCrossed(
                    Left,
                    QuadrantOutside,
                    Bottom,
                    QuadrantOutside,
                ) => "▞",
                BorderSymbol::StartCornerCrossed(_, QuadrantOutside, _, QuadrantOutside) => "▝",

                BorderSymbol::StartCornerCrossed(Left, QuadrantInside, Bottom, QuadrantInside) => {
                    "▜"
                }
                BorderSymbol::StartCornerCrossed(Left, QuadrantInside, Top, QuadrantInside) => "▟",
                BorderSymbol::StartCornerCrossed(Right, QuadrantInside, Bottom, QuadrantInside) => {
                    "▛"
                }
                BorderSymbol::StartCornerCrossed(_, QuadrantInside, _, QuadrantInside) => "▞",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "▜",
                BorderSymbol::SideRegular => "▀",
                BorderSymbol::SideOverlap(_, QuadrantInside) => "█",
                BorderSymbol::SideOverlap(_, _) => "▀",
                BorderSymbol::SideOutward(Left, QuadrantOutside) => "▛",
                BorderSymbol::SideOutward(Right, QuadrantOutside) => "▜",
                BorderSymbol::SideOutward(Left, _) => "▜",
                BorderSymbol::SideOutward(Right, _) => "▛",
                BorderSymbol::SideOutward(_, _) => "▀",
                BorderSymbol::SideInward(_, _) => "▀",
                BorderSymbol::SideCrossed(Left, _, _, _) => "▜",
                BorderSymbol::SideCrossed(Right, _, _, _) => "▛",
                BorderSymbol::SideCrossed(_, _, _, _) => "▀",
                BorderSymbol::EndCornerRegular => "▘",
                BorderSymbol::EndCornerAngled(Left, QuadrantOutside) => "▌",
                BorderSymbol::EndCornerAngled(Right, QuadrantOutside) => "▜",
                BorderSymbol::EndCornerAngled(Left, QuadrantInside) => "▜",
                BorderSymbol::EndCornerAngled(Right, QuadrantInside) => "▌",
                BorderSymbol::EndCornerAngled(_, _) => "▌",
                BorderSymbol::EndCornerProlonged(Top, QuadrantOutside) => "▀",
                BorderSymbol::EndCornerProlonged(Bottom, QuadrantOutside) => "▙",
                BorderSymbol::EndCornerProlonged(Top, QuadrantInside) => "▙",
                BorderSymbol::EndCornerProlonged(Bottom, QuadrantInside) => "▀",
                BorderSymbol::EndCornerProlonged(_, _) => "▀",
                BorderSymbol::EndCornerCrossed(Left, QuadrantOutside, Top, QuadrantOutside) => "▛",
                BorderSymbol::EndCornerCrossed(Left, QuadrantOutside, Bottom, QuadrantOutside) => {
                    "▙"
                }
                BorderSymbol::EndCornerCrossed(Right, QuadrantOutside, Bottom, QuadrantOutside) => {
                    "▚"
                }
                BorderSymbol::EndCornerCrossed(Right, QuadrantOutside, Top, QuadrantOutside) => "▜",
                BorderSymbol::EndCornerCrossed(_, QuadrantOutside, _, QuadrantOutside) => "▝",
                BorderSymbol::EndCornerCrossed(Right, QuadrantInside, Top, QuadrantInside) => "▙",
                BorderSymbol::EndCornerCrossed(Left, QuadrantInside, Bottom, QuadrantInside) => "▜",
                BorderSymbol::EndCornerCrossed(Right, QuadrantInside, Bottom, QuadrantInside) => {
                    "▛"
                }
                BorderSymbol::EndCornerCrossed(_, QuadrantInside, _, QuadrantInside) => "▚",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "▛",
                BorderSymbol::Cross(Left, _, Bottom, _, _, _, _, _) => "▜",
                BorderSymbol::Cross(Left, _, Top, _, _, _, _, _) => "▟",
                BorderSymbol::Cross(Right, _, Bottom, _, _, _, _, _) => "▛",
                BorderSymbol::Cross(Right, _, Top, _, _, _, _, _) => "▙",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "▄",
            },
            Right => match symbol {
                BorderSymbol::StartCornerRegular => "▖",
                BorderSymbol::StartCornerAngled(Top, QuadrantOutside) => "▛",
                BorderSymbol::StartCornerAngled(Bottom, QuadrantOutside) => "▄",
                BorderSymbol::StartCornerAngled(Top, QuadrantInside) => "▄",
                BorderSymbol::StartCornerAngled(Bottom, QuadrantInside) => "▛",
                BorderSymbol::StartCornerAngled(_, _) => "▄",
                BorderSymbol::StartCornerProlonged(Left, QuadrantOutside) => "▌",
                BorderSymbol::StartCornerProlonged(Right, QuadrantOutside) => "▌",
                BorderSymbol::StartCornerProlonged(Left, QuadrantInside) => "▟",
                BorderSymbol::StartCornerProlonged(Right, QuadrantInside) => "▌",
                BorderSymbol::StartCornerProlonged(_, _) => "▌",
                BorderSymbol::StartCornerCrossed(_, QuadrantOutside, _, QuadrantOutside) => "▙",
                BorderSymbol::StartCornerCrossed(_, QuadrantInside, _, QuadrantInside) => "▞",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "▙",
                BorderSymbol::SideRegular => "▌",
                BorderSymbol::SideOverlap(_, QuadrantInside) => "█",
                BorderSymbol::SideOverlap(_, _) => "▌",
                BorderSymbol::SideOutward(Top, QuadrantOutside) => "▛",
                BorderSymbol::SideOutward(Bottom, QuadrantOutside) => "▙",
                BorderSymbol::SideOutward(Top, _) => "▙",
                BorderSymbol::SideOutward(Bottom, _) => "▛",
                BorderSymbol::SideOutward(_, _) => "▌",
                BorderSymbol::SideInward(_, _) => "▌",
                BorderSymbol::SideCrossed(Top, _, _, _) => "▙",
                BorderSymbol::SideCrossed(Bottom, _, _, _) => "▛",
                BorderSymbol::SideCrossed(_, _, _, _) => "▌",
                BorderSymbol::EndCornerRegular => "▘",
                BorderSymbol::EndCornerAngled(Top, QuadrantOutside) => "▄",
                BorderSymbol::EndCornerAngled(Bottom, QuadrantOutside) => "▙",
                BorderSymbol::EndCornerAngled(Top, QuadrantInside) => "▙",
                BorderSymbol::EndCornerAngled(Bottom, QuadrantInside) => "▀",
                BorderSymbol::EndCornerAngled(_, _) => "▀",
                BorderSymbol::EndCornerProlonged(Left, QuadrantOutside) => "▌",
                BorderSymbol::EndCornerProlonged(Right, QuadrantOutside) => "▜",
                BorderSymbol::EndCornerProlonged(Left, QuadrantInside) => "▌",
                BorderSymbol::EndCornerProlonged(Right, QuadrantInside) => "▜",
                BorderSymbol::EndCornerProlonged(_, _) => "▌",
                BorderSymbol::EndCornerCrossed(_, QuadrantOutside, _, QuadrantOutside) => "▛",
                BorderSymbol::EndCornerCrossed(_, QuadrantInside, _, QuadrantInside) => "▚",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "▛",
                BorderSymbol::Cross(Left, _, Bottom, _, _, _, _, _) => "▜",
                BorderSymbol::Cross(Left, _, Top, _, _, _, _, _) => "▟",
                BorderSymbol::Cross(Right, _, Bottom, _, _, _, _, _) => "▛",
                BorderSymbol::Cross(Right, _, Top, _, _, _, _, _) => "▙",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "▌",
            },
            Left => match symbol {
                BorderSymbol::StartCornerRegular => "▗",
                BorderSymbol::StartCornerAngled(Top, QuadrantOutside) => "▄",
                BorderSymbol::StartCornerAngled(Bottom, QuadrantOutside) => "▟",
                BorderSymbol::StartCornerAngled(Top, QuadrantInside) => "▟",
                BorderSymbol::StartCornerAngled(Bottom, QuadrantInside) => "▄",
                BorderSymbol::StartCornerAngled(_, _) => "▄",
                BorderSymbol::StartCornerProlonged(Left, QuadrantOutside) => "▐",
                BorderSymbol::StartCornerProlonged(Right, QuadrantOutside) => "▐",
                BorderSymbol::StartCornerProlonged(Left, QuadrantInside) => "▐",
                BorderSymbol::StartCornerProlonged(Right, QuadrantInside) => "▛",
                BorderSymbol::StartCornerProlonged(_, _) => "▐",
                BorderSymbol::StartCornerCrossed(_, QuadrantOutside, _, QuadrantOutside) => "▟",
                BorderSymbol::StartCornerCrossed(_, QuadrantInside, _, QuadrantInside) => "▚",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "▟",
                BorderSymbol::SideRegular => "▐",
                BorderSymbol::SideOverlap(_, QuadrantInside) => "█",
                BorderSymbol::SideOverlap(_, _) => "▐",
                BorderSymbol::SideOutward(Top, QuadrantOutside) => "▜",
                BorderSymbol::SideOutward(Bottom, QuadrantOutside) => "▟",
                BorderSymbol::SideOutward(Top, _) => "▟",
                BorderSymbol::SideOutward(Bottom, _) => "▜",
                BorderSymbol::SideOutward(_, _) => "▐",
                BorderSymbol::SideInward(_, _) => "▐",
                BorderSymbol::SideCrossed(Top, _, _, _) => "▜",
                BorderSymbol::SideCrossed(Bottom, _, _, _) => "▟",
                BorderSymbol::SideCrossed(_, _, _, _) => "▐",
                BorderSymbol::EndCornerRegular => "▝",
                BorderSymbol::EndCornerAngled(Top, QuadrantOutside) => "▟",
                BorderSymbol::EndCornerAngled(Bottom, QuadrantOutside) => "▀",
                BorderSymbol::EndCornerAngled(Top, QuadrantInside) => "▀",
                BorderSymbol::EndCornerAngled(Bottom, QuadrantInside) => "▟",
                BorderSymbol::EndCornerAngled(_, _) => "▀",
                BorderSymbol::EndCornerProlonged(Left, QuadrantOutside) => "▙",
                BorderSymbol::EndCornerProlonged(Right, QuadrantOutside) => "▐",
                BorderSymbol::EndCornerProlonged(Left, QuadrantInside) => "▐",
                BorderSymbol::EndCornerProlonged(Right, QuadrantInside) => "▙",
                BorderSymbol::EndCornerProlonged(_, _) => "▐",
                BorderSymbol::EndCornerCrossed(_, QuadrantOutside, _, QuadrantOutside) => "▜",
                BorderSymbol::EndCornerCrossed(_, QuadrantInside, _, QuadrantInside) => "▞",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "▜",
                BorderSymbol::Cross(Left, _, Top, _, _, _, _, _) => "▟",
                BorderSymbol::Cross(Left, _, Bottom, _, _, _, _, _) => "▜",
                BorderSymbol::Cross(Right, _, Top, _, _, _, _, _) => "▙",
                BorderSymbol::Cross(Right, _, Bottom, _, _, _, _, _) => "▛",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "▐",
            },
        }
    }
}

/// Quadrant outside symbol set.
pub struct QuadrantOutsideSymbolSet;

impl BorderSymbolSet for QuadrantOutsideSymbolSet {
    fn symbol(&self, side: Side, symbol: BorderSymbol) -> &'static str {
        use crate::Side::*;
        use BorderType::*;

        let fff = match side {
            Top => match symbol {
                BorderSymbol::StartCornerRegular => "▛",
                BorderSymbol::StartCornerAngled(_, _) => "▛",
                BorderSymbol::StartCornerProlonged(_, _) => "▛",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "▛",
                BorderSymbol::SideRegular => "▀",
                BorderSymbol::SideOverlap(_, _) => "▀",
                BorderSymbol::SideOutward(_, _) => "▀",
                BorderSymbol::SideInward(Left, QuadrantOutside) => "▛",
                BorderSymbol::SideInward(Right, QuadrantOutside) => "▜",
                BorderSymbol::SideInward(Left, _) => "▜",
                BorderSymbol::SideInward(Right, _) => "▛",
                BorderSymbol::SideInward(_, _) => "▀",
                BorderSymbol::SideCrossed(Left, _, _, _) => "▛",
                BorderSymbol::SideCrossed(Right, _, _, _) => "▜",
                BorderSymbol::SideCrossed(_, _, _, _) => "▀",
                BorderSymbol::EndCornerRegular => "▜",
                BorderSymbol::EndCornerAngled(_, _) => "▜",
                BorderSymbol::EndCornerProlonged(_, _) => "▜",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "▜",
                BorderSymbol::Cross(Left, _, Top, _, _, _, _, _) => "▛",
                BorderSymbol::Cross(Left, _, Bottom, _, _, _, _, _) => "▙",
                BorderSymbol::Cross(Right, _, Top, _, _, _, _, _) => "▜",
                BorderSymbol::Cross(Right, _, Bottom, _, _, _, _, _) => "▟",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "▀",
            },
            Bottom => match symbol {
                BorderSymbol::StartCornerRegular => "▙",
                BorderSymbol::StartCornerAngled(_, _) => "▙",
                BorderSymbol::StartCornerProlonged(_, _) => "▙",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "▙",
                BorderSymbol::SideRegular => "▄",
                BorderSymbol::SideOverlap(_, _) => "▄",
                BorderSymbol::SideOutward(Left, _) => "▄",
                BorderSymbol::SideOutward(Right, _) => "▄",
                BorderSymbol::SideOutward(_, _) => "▄",
                BorderSymbol::SideInward(Left, QuadrantOutside) => "▙",
                BorderSymbol::SideInward(Right, QuadrantOutside) => "▟",
                BorderSymbol::SideInward(Left, _) => "▟",
                BorderSymbol::SideInward(Right, _) => "▙",
                BorderSymbol::SideInward(_, _) => "▄",
                BorderSymbol::SideCrossed(Left, _, _, _) => "▙",
                BorderSymbol::SideCrossed(Right, _, _, _) => "▟",
                BorderSymbol::SideCrossed(_, _, _, _) => "▄",
                BorderSymbol::EndCornerRegular => "▟",
                BorderSymbol::EndCornerAngled(_, _) => "▟",
                BorderSymbol::EndCornerProlonged(_, _) => "▟",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "▟",
                BorderSymbol::Cross(Left, _, Top, _, _, _, _, _) => "▛",
                BorderSymbol::Cross(Left, _, Bottom, _, _, _, _, _) => "▙",
                BorderSymbol::Cross(Right, _, Top, _, _, _, _, _) => "▜",
                BorderSymbol::Cross(Right, _, Bottom, _, _, _, _, _) => "▟",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "▄",
            },
            Right => match symbol {
                BorderSymbol::StartCornerRegular => "▜",
                BorderSymbol::StartCornerAngled(_, _) => "▜",
                BorderSymbol::StartCornerProlonged(_, _) => "▜",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "▜",
                BorderSymbol::SideRegular => "▐",
                BorderSymbol::SideOverlap(_, _) => "▐",
                BorderSymbol::SideOutward(_, _) => "▐",
                BorderSymbol::SideInward(Top, QuadrantOutside) => "▜",
                BorderSymbol::SideInward(Bottom, QuadrantOutside) => "▟",
                BorderSymbol::SideInward(Top, _) => "▟",
                BorderSymbol::SideInward(Bottom, _) => "▜",
                BorderSymbol::SideInward(_, _) => "▐",
                BorderSymbol::SideCrossed(Top, _, _, _) => "▜",
                BorderSymbol::SideCrossed(Bottom, _, _, _) => "▟",
                BorderSymbol::SideCrossed(_, _, _, _) => "▐",
                BorderSymbol::EndCornerRegular => "▜",
                BorderSymbol::EndCornerAngled(_, _) => "▜",
                BorderSymbol::EndCornerProlonged(_, _) => "▜",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "▜",
                BorderSymbol::Cross(Left, _, Top, _, _, _, _, _) => "▛",
                BorderSymbol::Cross(Left, _, Bottom, _, _, _, _, _) => "▙",
                BorderSymbol::Cross(Right, _, Top, _, _, _, _, _) => "▜",
                BorderSymbol::Cross(Right, _, Bottom, _, _, _, _, _) => "▟",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "▐",
            },
            Left => match symbol {
                BorderSymbol::StartCornerRegular => "▛",
                BorderSymbol::StartCornerAngled(_, _) => "▛",
                BorderSymbol::StartCornerProlonged(_, _) => "▛",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "▛",
                BorderSymbol::SideRegular => "▌",
                BorderSymbol::SideOverlap(_, _) => "▌",
                BorderSymbol::SideOutward(_, _) => "▌",
                BorderSymbol::SideInward(Top, QuadrantOutside) => "▛",
                BorderSymbol::SideInward(Bottom, QuadrantOutside) => "▙",
                BorderSymbol::SideInward(Top, _) => "▙",
                BorderSymbol::SideInward(Bottom, _) => "▛",
                BorderSymbol::SideInward(_, _) => "▌",
                BorderSymbol::SideCrossed(Top, _, _, _) => "▛",
                BorderSymbol::SideCrossed(Bottom, _, _, _) => "▙",
                BorderSymbol::SideCrossed(_, _, _, _) => "▌",
                BorderSymbol::EndCornerRegular => "▙",
                BorderSymbol::EndCornerAngled(_, _) => "▙",
                BorderSymbol::EndCornerProlonged(_, _) => "▙",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "▙",
                BorderSymbol::Cross(_, _, Left, _, Top, _, _, _) => "▛",
                BorderSymbol::Cross(_, _, Left, _, Bottom, _, _, _) => "▙",
                BorderSymbol::Cross(_, _, Right, _, Top, _, _, _) => "▜",
                BorderSymbol::Cross(_, _, Right, _, Bottom, _, _, _) => "▟",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "▐",
            },
        };

        debug!("{:?} {:?} => {:?}", side, symbol, fff);

        fff
    }
}

/// Uses plain ascii characters to draw a border. Uses '+', '-' and '|'.
pub struct AsciiSymbolSet;

impl BorderSymbolSet for AsciiSymbolSet {
    fn symbol(&self, side: Side, symbol: BorderSymbol) -> &'static str {
        use crate::Side::*;

        match side {
            Top => match symbol {
                BorderSymbol::StartCornerRegular => "+",
                BorderSymbol::StartCornerAngled(_, _) => "+",
                BorderSymbol::StartCornerProlonged(_, _) => "+",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "+",
                BorderSymbol::SideRegular => "-",
                BorderSymbol::SideOverlap(_, _) => "-",
                BorderSymbol::SideOutward(_, _) => "+",
                BorderSymbol::SideInward(_, _) => "+",
                BorderSymbol::SideCrossed(_, _, _, _) => "+",
                BorderSymbol::EndCornerRegular => "+",
                BorderSymbol::EndCornerAngled(_, _) => "+",
                BorderSymbol::EndCornerProlonged(_, _) => "+",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "+",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "+",
            },
            Bottom => match symbol {
                BorderSymbol::StartCornerRegular => "+",
                BorderSymbol::StartCornerAngled(_, _) => "+",
                BorderSymbol::StartCornerProlonged(_, _) => "+",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "+",
                BorderSymbol::SideRegular => "-",
                BorderSymbol::SideOverlap(_, _) => "-",
                BorderSymbol::SideOutward(_, _) => "+",
                BorderSymbol::SideInward(_, _) => "+",
                BorderSymbol::SideCrossed(_, _, _, _) => "+",
                BorderSymbol::EndCornerRegular => "+",
                BorderSymbol::EndCornerAngled(_, _) => "+",
                BorderSymbol::EndCornerProlonged(_, _) => "+",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "+",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "+",
            },
            Right => match symbol {
                BorderSymbol::StartCornerRegular => "+",
                BorderSymbol::StartCornerAngled(_, _) => "+",
                BorderSymbol::StartCornerProlonged(_, _) => "+",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "+",
                BorderSymbol::SideRegular => "|",
                BorderSymbol::SideOverlap(_, _) => "|",
                BorderSymbol::SideOutward(_, _) => "+",
                BorderSymbol::SideInward(_, _) => "+",
                BorderSymbol::SideCrossed(_, _, _, _) => "+",
                BorderSymbol::EndCornerRegular => "+",
                BorderSymbol::EndCornerAngled(_, _) => "+",
                BorderSymbol::EndCornerProlonged(_, _) => "+",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "+",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "+",
            },
            Left => match symbol {
                BorderSymbol::StartCornerRegular => "+",
                BorderSymbol::StartCornerAngled(_, _) => "+",
                BorderSymbol::StartCornerProlonged(_, _) => "+",
                BorderSymbol::StartCornerCrossed(_, _, _, _) => "+",
                BorderSymbol::SideRegular => "|",
                BorderSymbol::SideOverlap(_, _) => "|",
                BorderSymbol::SideOutward(_, _) => "+",
                BorderSymbol::SideInward(_, _) => "+",
                BorderSymbol::SideCrossed(_, _, _, _) => "+",
                BorderSymbol::EndCornerRegular => "+",
                BorderSymbol::EndCornerAngled(_, _) => "+",
                BorderSymbol::EndCornerProlonged(_, _) => "+",
                BorderSymbol::EndCornerCrossed(_, _, _, _) => "+",
                BorderSymbol::Cross(_, _, _, _, _, _, _, _) => "+",
            },
        }
    }
}

/// Draws a border using only '*'.
pub struct StarSymbolSet;

impl BorderSymbolSet for StarSymbolSet {
    fn symbol(&self, _side: Side, _symbol: BorderSymbol) -> &'static str {
        &"*"
    }
}
