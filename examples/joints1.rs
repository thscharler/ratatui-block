use crate::mini_salsa::theme::THEME;
use crate::mini_salsa::{layout_grid, run_ui, setup_logging, MiniSalsaState};
use log::debug;
use rat_event::{ct_event, Outcome};
use ratatui::layout::{Constraint, Layout, Rect, Spacing};
use ratatui::prelude::Widget;
use ratatui::style::{Style, Styled};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType};
use ratatui::{crossterm, Frame};
use ratatui_block::block_connect::BlockConnect;
use ratatui_block::BorderSymbol::{EndCornerRegular, SideRegular, StartCornerRegular};
use ratatui_block::{BorderSymbol, Side};

mod mini_salsa;

fn main() -> Result<(), anyhow::Error> {
    setup_logging()?;

    let mut data = Data {};
    let mut state = State {
        area: Default::default(),

        border: BorderType::Plain,
        hor_neighbour: BorderType::Plain,
        vert_neighbour: BorderType::Plain,

        first: true,
        cross: false,

        b_side: Side::Top,
        b_symbol: BorderSymbol::StartCornerRegular,

        mono: false,
    };

    run_ui(
        "╒═╤═╛",
        handle_buttons,
        repaint_buttons,
        &mut data,
        &mut state,
    )
}

struct Data {}

struct State {
    area: Rect,

    border: BorderType,
    hor_neighbour: BorderType,
    vert_neighbour: BorderType,

    first: bool,
    cross: bool,

    b_side: Side,
    b_symbol: BorderSymbol,

    mono: bool,
}

impl State {
    fn angled_side(&self) -> Side {
        match (self.b_side, self.first) {
            (Side::Top, true) => Side::Left,
            (Side::Top, false) => Side::Right,
            (Side::Bottom, true) => Side::Left,
            (Side::Bottom, false) => Side::Right,
            (Side::Right, true) => Side::Top,
            (Side::Right, false) => Side::Bottom,
            (Side::Left, true) => Side::Top,
            (Side::Left, false) => Side::Bottom,
        }
    }

    fn angled(&self) -> BorderType {
        match self.b_side {
            Side::Top | Side::Bottom => self.vert_neighbour,
            Side::Right | Side::Left => self.hor_neighbour,
        }
    }

    fn prolonged(&self) -> BorderType {
        match self.b_side {
            Side::Top | Side::Bottom => self.hor_neighbour,
            Side::Right | Side::Left => self.vert_neighbour,
        }
    }
}

fn repaint_buttons(
    frame: &mut Frame<'_>,
    area: Rect,
    _data: &mut Data,
    _istate: &mut MiniSalsaState,
    state: &mut State,
) -> Result<(), anyhow::Error> {
    let buf = frame.buffer_mut();

    let l0 = Layout::horizontal([
        Constraint::Length(25), //
        Constraint::Fill(1),
    ])
    .split(area);

    let layout = layout_grid::<3, 3>(
        l0[1],
        Layout::horizontal([
            Constraint::Length(10),
            Constraint::Fill(1),
            Constraint::Length(10),
        ])
        .spacing(Spacing::Overlap(1)),
        Layout::vertical([
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(5),
        ])
        .spacing(Spacing::Overlap(1)),
    );

    state.area = layout[1][1];

    buf.set_style(area, THEME.gray(0));

    for a in 0..3 {
        for b in 0..3 {
            if a != 1 && b == 1 {
                Block::bordered()
                    .border_type(state.hor_neighbour)
                    .render(layout[a][b], buf);
            }
            if a == 1 && b != 1 {
                Block::bordered()
                    .border_type(state.vert_neighbour)
                    .render(layout[a][b], buf);
            }
        }
    }

    Block::bordered()
        .border_type(state.border)
        .render(layout[1][1], buf);

    let mut b_area = Rect::new(0, 0, 1, 1);
    match (state.b_side, state.b_symbol) {
        (Side::Top, BorderSymbol::StartCornerAngled(_, _))
        | (Side::Top, BorderSymbol::StartCornerRegular)
        | (Side::Top, BorderSymbol::StartCornerProlonged(_, _))
        | (Side::Top, BorderSymbol::StartCornerCrossed(_, _, _, _)) => {
            b_area.x = layout[1][1].x;
            b_area.y = layout[1][1].y;
        }
        (Side::Top, BorderSymbol::SideRegular)
        | (Side::Top, BorderSymbol::SideOverlap(_, _))
        | (Side::Top, BorderSymbol::SideOutward(_, _))
        | (Side::Top, BorderSymbol::SideInward(_, _))
        | (Side::Top, BorderSymbol::SideCrossed(_, _, _, _))
        | (Side::Top, BorderSymbol::Cross(_, _, _, _, _, _, _, _)) => {
            b_area.x = layout[1][1].x + layout[1][1].width / 2;
            b_area.y = layout[1][1].y;
        }
        (Side::Top, BorderSymbol::EndCornerRegular)
        | (Side::Top, BorderSymbol::EndCornerAngled(_, _))
        | (Side::Top, BorderSymbol::EndCornerProlonged(_, _))
        | (Side::Top, BorderSymbol::EndCornerCrossed(_, _, _, _)) => {
            b_area.x = layout[1][1].x + layout[1][1].width.saturating_sub(1);
            b_area.y = layout[1][1].y;
        }

        (Side::Bottom, BorderSymbol::StartCornerAngled(_, _))
        | (Side::Bottom, BorderSymbol::StartCornerRegular)
        | (Side::Bottom, BorderSymbol::StartCornerProlonged(_, _))
        | (Side::Bottom, BorderSymbol::StartCornerCrossed(_, _, _, _)) => {
            b_area.x = layout[1][1].x;
            b_area.y = layout[1][1].y + layout[1][1].height.saturating_sub(1);
        }
        (Side::Bottom, BorderSymbol::SideRegular)
        | (Side::Bottom, BorderSymbol::SideOverlap(_, _))
        | (Side::Bottom, BorderSymbol::SideOutward(_, _))
        | (Side::Bottom, BorderSymbol::SideInward(_, _))
        | (Side::Bottom, BorderSymbol::SideCrossed(_, _, _, _))
        | (Side::Bottom, BorderSymbol::Cross(_, _, _, _, _, _, _, _)) => {
            b_area.x = layout[1][1].x + layout[1][1].width / 2;
            b_area.y = layout[1][1].y + layout[1][1].height.saturating_sub(1);
        }
        (Side::Bottom, BorderSymbol::EndCornerRegular)
        | (Side::Bottom, BorderSymbol::EndCornerAngled(_, _))
        | (Side::Bottom, BorderSymbol::EndCornerProlonged(_, _))
        | (Side::Bottom, BorderSymbol::EndCornerCrossed(_, _, _, _)) => {
            b_area.x = layout[1][1].x + layout[1][1].width.saturating_sub(1);
            b_area.y = layout[1][1].y + layout[1][1].height.saturating_sub(1);
        }

        (Side::Right, BorderSymbol::StartCornerAngled(_, _))
        | (Side::Right, BorderSymbol::StartCornerRegular)
        | (Side::Right, BorderSymbol::StartCornerProlonged(_, _))
        | (Side::Right, BorderSymbol::StartCornerCrossed(_, _, _, _)) => {
            b_area.x = layout[1][1].x + layout[1][1].width.saturating_sub(1);
            b_area.y = layout[1][1].y;
        }
        (Side::Right, BorderSymbol::SideRegular)
        | (Side::Right, BorderSymbol::SideOverlap(_, _))
        | (Side::Right, BorderSymbol::SideOutward(_, _))
        | (Side::Right, BorderSymbol::SideInward(_, _))
        | (Side::Right, BorderSymbol::SideCrossed(_, _, _, _))
        | (Side::Right, BorderSymbol::Cross(_, _, _, _, _, _, _, _)) => {
            b_area.x = layout[1][1].x + layout[1][1].width.saturating_sub(1);
            b_area.y = layout[1][1].y + layout[1][1].height / 2;
        }
        (Side::Right, BorderSymbol::EndCornerRegular)
        | (Side::Right, BorderSymbol::EndCornerAngled(_, _))
        | (Side::Right, BorderSymbol::EndCornerProlonged(_, _))
        | (Side::Right, BorderSymbol::EndCornerCrossed(_, _, _, _)) => {
            b_area.x = layout[1][1].x + layout[1][1].width.saturating_sub(1);
            b_area.y = layout[1][1].y + layout[1][1].height.saturating_sub(1);
        }

        (Side::Left, BorderSymbol::StartCornerAngled(_, _))
        | (Side::Left, BorderSymbol::StartCornerRegular)
        | (Side::Left, BorderSymbol::StartCornerProlonged(_, _))
        | (Side::Left, BorderSymbol::StartCornerCrossed(_, _, _, _)) => {
            b_area.x = layout[1][1].x;
            b_area.y = layout[1][1].y;
        }
        (Side::Left, BorderSymbol::SideRegular)
        | (Side::Left, BorderSymbol::SideOverlap(_, _))
        | (Side::Left, BorderSymbol::SideOutward(_, _))
        | (Side::Left, BorderSymbol::SideInward(_, _))
        | (Side::Left, BorderSymbol::SideCrossed(_, _, _, _))
        | (Side::Left, BorderSymbol::Cross(_, _, _, _, _, _, _, _)) => {
            b_area.x = layout[1][1].x;
            b_area.y = layout[1][1].y + layout[1][1].height / 2;
        }
        (Side::Left, BorderSymbol::EndCornerRegular)
        | (Side::Left, BorderSymbol::EndCornerAngled(_, _))
        | (Side::Left, BorderSymbol::EndCornerProlonged(_, _))
        | (Side::Left, BorderSymbol::EndCornerCrossed(_, _, _, _)) => {
            b_area.x = layout[1][1].x;
            b_area.y = layout[1][1].y + layout[1][1].height.saturating_sub(1);
        }
    }

    if state.mono {
        BlockConnect::new()
            .border_type(state.border)
            .side(state.b_side)
            .symbol(state.b_symbol)
            .render(b_area, buf);
    } else {
        BlockConnect::new()
            .border_style(Style::new().fg(THEME.limegreen[3]))
            .border_type(state.border)
            .side(state.b_side)
            .symbol(state.b_symbol)
            .render(b_area, buf);
    }

    let mut txt_area = l0[0];
    txt_area.y += 2;
    txt_area.height = 1;

    "F1: main border"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F2: horizontal neighbours"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F3: vertical neighbours"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F4: first/last"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "Left/Right: position"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F8: mono"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 2;

    format!("hor={:?}", state.hor_neighbour).render(txt_area, buf);
    txt_area.y += 1;
    format!("vert={:?}", state.vert_neighbour).render(txt_area, buf);
    txt_area.y += 1;
    format!("border={:?}", state.border).render(txt_area, buf);
    txt_area.y += 1;
    format!("side={:?}", state.b_side).render(txt_area, buf);
    txt_area.y += 1;
    txt_area.height = 10;
    Text::from(format!("symbol={:#?}", state.b_symbol)).render(txt_area, buf);

    Ok(())
}

fn handle_buttons(
    event: &crossterm::event::Event,
    _data: &mut Data,
    _istate: &mut MiniSalsaState,
    state: &mut State,
) -> Result<Outcome, anyhow::Error> {
    let r = match event {
        ct_event!(keycode press F(1)) => {
            state.border = match state.border {
                BorderType::Plain => BorderType::Rounded,
                BorderType::Rounded => BorderType::Double,
                BorderType::Double => BorderType::Thick,
                BorderType::Thick => BorderType::QuadrantInside,
                BorderType::QuadrantInside => BorderType::QuadrantOutside,
                BorderType::QuadrantOutside => BorderType::Plain,
            };
            Outcome::Changed
        }
        ct_event!(keycode press F(2)) => {
            state.hor_neighbour = match state.hor_neighbour {
                BorderType::Plain => BorderType::Rounded,
                BorderType::Rounded => BorderType::Double,
                BorderType::Double => BorderType::Thick,
                BorderType::Thick => BorderType::QuadrantInside,
                BorderType::QuadrantInside => BorderType::QuadrantOutside,
                BorderType::QuadrantOutside => BorderType::Plain,
            };
            Outcome::Changed
        }
        ct_event!(keycode press F(3)) => {
            state.vert_neighbour = match state.vert_neighbour {
                BorderType::Plain => BorderType::Rounded,
                BorderType::Rounded => BorderType::Double,
                BorderType::Double => BorderType::Thick,
                BorderType::Thick => BorderType::QuadrantInside,
                BorderType::QuadrantInside => BorderType::QuadrantOutside,
                BorderType::QuadrantOutside => BorderType::Plain,
            };
            Outcome::Changed
        }
        ct_event!(keycode press F(4)) => {
            state.first = !state.first;
            Outcome::Changed
        }
        ct_event!(keycode press F(5)) => {
            state.cross = !state.cross;
            Outcome::Changed
        }
        ct_event!(keycode press Right) => {
            use BorderSymbol::*;
            state.b_symbol = match state.b_symbol {
                StartCornerRegular => StartCornerAngled(state.angled_side(), state.angled()),
                StartCornerAngled(_, _) => {
                    StartCornerProlonged(state.angled_side(), state.prolonged())
                }
                StartCornerProlonged(_, _) => StartCornerCrossed(
                    state.angled_side(),
                    state.angled(),
                    state.b_side.opposite(),
                    state.prolonged(),
                ),
                StartCornerCrossed(_, _, _, _) => SideRegular,

                SideRegular => SideOverlap(state.angled_side(), state.angled()),
                SideOverlap(_, _) => SideOutward(state.angled_side(), state.angled()),
                SideOutward(_, _) => SideInward(state.angled_side().opposite(), state.angled()),
                SideInward(_, _) => SideCrossed(
                    state.angled_side(),
                    state.angled(),
                    state.angled_side(),
                    state.angled(),
                ),
                SideCrossed(_, _, _, _) => Cross {
                    0: state.angled_side(),
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: state.angled_side(),
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(_, _, _, _, _, _, _, _) => EndCornerRegular,
                EndCornerRegular => EndCornerAngled(state.angled_side(), state.angled()),
                EndCornerAngled(_, _) => EndCornerProlonged(state.angled_side(), state.prolonged()),
                EndCornerProlonged(_, _) => EndCornerCrossed(
                    state.angled_side(),
                    state.angled(),
                    state.b_side.opposite(),
                    state.prolonged(),
                ),
                EndCornerCrossed(_, _, _, _) => {
                    state.b_side = match state.b_side {
                        Side::Top => Side::Right,
                        Side::Right => Side::Bottom,
                        Side::Bottom => Side::Left,
                        Side::Left => Side::Top,
                    };
                    StartCornerRegular
                }
            };
            Outcome::Changed
        }
        ct_event!(keycode press Left) => {
            use BorderSymbol::*;
            state.b_symbol = match state.b_symbol {
                StartCornerRegular => {
                    state.b_side = match state.b_side {
                        Side::Top => Side::Right,
                        Side::Right => Side::Bottom,
                        Side::Bottom => Side::Left,
                        Side::Left => Side::Top,
                    };
                    EndCornerCrossed(
                        state.angled_side(),
                        state.angled(),
                        state.b_side.opposite(),
                        state.prolonged(),
                    )
                }
                StartCornerAngled(_, _) => StartCornerRegular,
                StartCornerProlonged(_, _) => {
                    StartCornerAngled(state.angled_side(), state.angled())
                }
                StartCornerCrossed(_, _, _, _) => {
                    StartCornerProlonged(state.angled_side(), state.prolonged())
                }
                SideRegular => StartCornerCrossed(
                    state.angled_side(),
                    state.angled(),
                    state.b_side.opposite(),
                    state.prolonged(),
                ),
                SideOverlap(_, _) => SideRegular,
                SideOutward(_, _) => SideOverlap(state.angled_side(), state.angled()),
                SideInward(_, _) => SideOutward(state.angled_side(), state.angled()),
                SideCrossed(_, _, _, _) => {
                    SideInward(state.angled_side().opposite(), state.angled())
                }
                Cross(_, _, _, _, _, _, _, _) => SideCrossed(
                    state.angled_side(),
                    state.angled(),
                    state.angled_side(),
                    state.angled(),
                ),
                EndCornerRegular => Cross {
                    0: state.angled_side(),
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: state.angled_side(),
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                EndCornerAngled(_, _) => EndCornerRegular,
                EndCornerProlonged(_, _) => EndCornerAngled(state.angled_side(), state.angled()),
                EndCornerCrossed(_, _, _, _) => {
                    EndCornerProlonged(state.angled_side(), state.prolonged())
                }
            };
            Outcome::Changed
        }

        ct_event!(keycode press F(8)) => {
            state.mono = !state.mono;
            Outcome::Changed
        }
        _ => Outcome::Continue,
    };

    Ok(r)
}
