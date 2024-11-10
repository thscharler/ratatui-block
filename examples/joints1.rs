use crate::mini_salsa::theme::THEME;
use crate::mini_salsa::{layout_grid, run_ui, setup_logging, MiniSalsaState};
use rat_event::{ct_event, Outcome};
use ratatui::layout::{Constraint, Layout, Rect, Spacing};
use ratatui::prelude::Widget;
use ratatui::style::{Style, Styled};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType};
use ratatui::{crossterm, Frame};
use ratatui_block::block_connect::BlockConnect;
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

    b_side: Side,
    b_symbol: BorderSymbol,

    mono: bool,
}

impl State {
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

    let l_main = layout_grid::<3, 3>(
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

    state.area = l_main[1][1];

    buf.set_style(area, THEME.gray(0));

    Block::bordered()
        .border_type(state.border)
        .render(l_main[1][1], buf);

    let mut b_area = Rect::new(0, 0, 1, 1);
    match (state.b_side, state.b_symbol) {
        (Side::Top, BorderSymbol::StartCornerAngled(_, _))
        | (Side::Top, BorderSymbol::StartCornerRegular)
        | (Side::Top, BorderSymbol::StartCornerProlonged(_, _))
        | (Side::Top, BorderSymbol::StartCornerCrossed(_, _, _, _)) => {
            b_area.x = l_main[1][1].x;
            b_area.y = l_main[1][1].y;
        }
        (Side::Top, BorderSymbol::SideRegular)
        | (Side::Top, BorderSymbol::SideOverlap(_, _))
        | (Side::Top, BorderSymbol::SideOutward(_, _))
        | (Side::Top, BorderSymbol::SideInward(_, _))
        | (Side::Top, BorderSymbol::SideCrossed(_, _, _, _))
        | (Side::Top, BorderSymbol::Cross(_, _, _, _, _, _, _, _)) => {
            b_area.x = l_main[1][1].x + l_main[1][1].width / 2;
            b_area.y = l_main[1][1].y;
        }
        (Side::Top, BorderSymbol::EndCornerRegular)
        | (Side::Top, BorderSymbol::EndCornerAngled(_, _))
        | (Side::Top, BorderSymbol::EndCornerProlonged(_, _))
        | (Side::Top, BorderSymbol::EndCornerCrossed(_, _, _, _)) => {
            b_area.x = l_main[1][1].x + l_main[1][1].width.saturating_sub(1);
            b_area.y = l_main[1][1].y;
        }

        (Side::Bottom, BorderSymbol::StartCornerAngled(_, _))
        | (Side::Bottom, BorderSymbol::StartCornerRegular)
        | (Side::Bottom, BorderSymbol::StartCornerProlonged(_, _))
        | (Side::Bottom, BorderSymbol::StartCornerCrossed(_, _, _, _)) => {
            b_area.x = l_main[1][1].x;
            b_area.y = l_main[1][1].y + l_main[1][1].height.saturating_sub(1);
        }
        (Side::Bottom, BorderSymbol::SideRegular)
        | (Side::Bottom, BorderSymbol::SideOverlap(_, _))
        | (Side::Bottom, BorderSymbol::SideOutward(_, _))
        | (Side::Bottom, BorderSymbol::SideInward(_, _))
        | (Side::Bottom, BorderSymbol::SideCrossed(_, _, _, _))
        | (Side::Bottom, BorderSymbol::Cross(_, _, _, _, _, _, _, _)) => {
            b_area.x = l_main[1][1].x + l_main[1][1].width / 2;
            b_area.y = l_main[1][1].y + l_main[1][1].height.saturating_sub(1);
        }
        (Side::Bottom, BorderSymbol::EndCornerRegular)
        | (Side::Bottom, BorderSymbol::EndCornerAngled(_, _))
        | (Side::Bottom, BorderSymbol::EndCornerProlonged(_, _))
        | (Side::Bottom, BorderSymbol::EndCornerCrossed(_, _, _, _)) => {
            b_area.x = l_main[1][1].x + l_main[1][1].width.saturating_sub(1);
            b_area.y = l_main[1][1].y + l_main[1][1].height.saturating_sub(1);
        }

        (Side::Right, BorderSymbol::StartCornerAngled(_, _))
        | (Side::Right, BorderSymbol::StartCornerRegular)
        | (Side::Right, BorderSymbol::StartCornerProlonged(_, _))
        | (Side::Right, BorderSymbol::StartCornerCrossed(_, _, _, _)) => {
            b_area.x = l_main[1][1].x + l_main[1][1].width.saturating_sub(1);
            b_area.y = l_main[1][1].y;
        }
        (Side::Right, BorderSymbol::SideRegular)
        | (Side::Right, BorderSymbol::SideOverlap(_, _))
        | (Side::Right, BorderSymbol::SideOutward(_, _))
        | (Side::Right, BorderSymbol::SideInward(_, _))
        | (Side::Right, BorderSymbol::SideCrossed(_, _, _, _))
        | (Side::Right, BorderSymbol::Cross(_, _, _, _, _, _, _, _)) => {
            b_area.x = l_main[1][1].x + l_main[1][1].width.saturating_sub(1);
            b_area.y = l_main[1][1].y + l_main[1][1].height / 2;
        }
        (Side::Right, BorderSymbol::EndCornerRegular)
        | (Side::Right, BorderSymbol::EndCornerAngled(_, _))
        | (Side::Right, BorderSymbol::EndCornerProlonged(_, _))
        | (Side::Right, BorderSymbol::EndCornerCrossed(_, _, _, _)) => {
            b_area.x = l_main[1][1].x + l_main[1][1].width.saturating_sub(1);
            b_area.y = l_main[1][1].y + l_main[1][1].height.saturating_sub(1);
        }

        (Side::Left, BorderSymbol::StartCornerAngled(_, _))
        | (Side::Left, BorderSymbol::StartCornerRegular)
        | (Side::Left, BorderSymbol::StartCornerProlonged(_, _))
        | (Side::Left, BorderSymbol::StartCornerCrossed(_, _, _, _)) => {
            b_area.x = l_main[1][1].x;
            b_area.y = l_main[1][1].y;
        }
        (Side::Left, BorderSymbol::SideRegular)
        | (Side::Left, BorderSymbol::SideOverlap(_, _))
        | (Side::Left, BorderSymbol::SideOutward(_, _))
        | (Side::Left, BorderSymbol::SideInward(_, _))
        | (Side::Left, BorderSymbol::SideCrossed(_, _, _, _))
        | (Side::Left, BorderSymbol::Cross(_, _, _, _, _, _, _, _)) => {
            b_area.x = l_main[1][1].x;
            b_area.y = l_main[1][1].y + l_main[1][1].height / 2;
        }
        (Side::Left, BorderSymbol::EndCornerRegular)
        | (Side::Left, BorderSymbol::EndCornerAngled(_, _))
        | (Side::Left, BorderSymbol::EndCornerProlonged(_, _))
        | (Side::Left, BorderSymbol::EndCornerCrossed(_, _, _, _)) => {
            b_area.x = l_main[1][1].x;
            b_area.y = l_main[1][1].y + l_main[1][1].height.saturating_sub(1);
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
            .border_style(Style::new().fg(THEME.redpink[3]))
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
    use BorderSymbol::*;
    use Side::*;

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
        ct_event!(scroll down) | ct_event!(keycode press Right) => {
            use BorderSymbol::*;
            state.b_symbol = match state.b_symbol {
                StartCornerRegular => StartCornerAngled(Top, state.angled()),
                StartCornerAngled(Top, _) => StartCornerAngled(Right, state.angled()),
                StartCornerAngled(Right, _) => StartCornerAngled(Bottom, state.angled()),
                StartCornerAngled(Bottom, _) => StartCornerAngled(Left, state.angled()),
                StartCornerAngled(Left, _) => StartCornerProlonged(Top, state.prolonged()),
                StartCornerProlonged(Top, _) => StartCornerProlonged(Right, state.prolonged()),
                StartCornerProlonged(Right, _) => StartCornerProlonged(Bottom, state.prolonged()),
                StartCornerProlonged(Bottom, _) => StartCornerProlonged(Left, state.prolonged()),
                StartCornerProlonged(Left, _) => {
                    StartCornerCrossed(Top, state.angled(), Top, state.prolonged())
                }
                StartCornerCrossed(Top, _, Top, _) => {
                    StartCornerCrossed(Top, state.angled(), Right, state.prolonged())
                }
                StartCornerCrossed(Top, _, Right, _) => {
                    StartCornerCrossed(Top, state.angled(), Bottom, state.prolonged())
                }
                StartCornerCrossed(Top, _, Bottom, _) => {
                    StartCornerCrossed(Top, state.angled(), Left, state.prolonged())
                }
                StartCornerCrossed(Top, _, Left, _) => {
                    StartCornerCrossed(Right, state.angled(), Top, state.prolonged())
                }
                StartCornerCrossed(Right, _, Top, _) => {
                    StartCornerCrossed(Right, state.angled(), Right, state.prolonged())
                }
                StartCornerCrossed(Right, _, Right, _) => {
                    StartCornerCrossed(Right, state.angled(), Bottom, state.prolonged())
                }
                StartCornerCrossed(Right, _, Bottom, _) => {
                    StartCornerCrossed(Right, state.angled(), Left, state.prolonged())
                }
                StartCornerCrossed(Right, _, Left, _) => {
                    StartCornerCrossed(Bottom, state.angled(), Top, state.prolonged())
                }
                StartCornerCrossed(Bottom, _, Top, _) => {
                    StartCornerCrossed(Bottom, state.angled(), Right, state.prolonged())
                }
                StartCornerCrossed(Bottom, _, Right, _) => {
                    StartCornerCrossed(Bottom, state.angled(), Bottom, state.prolonged())
                }
                StartCornerCrossed(Bottom, _, Bottom, _) => {
                    StartCornerCrossed(Bottom, state.angled(), Left, state.prolonged())
                }
                StartCornerCrossed(Bottom, _, Left, _) => {
                    StartCornerCrossed(Left, state.angled(), Top, state.prolonged())
                }
                StartCornerCrossed(Left, _, Top, _) => {
                    StartCornerCrossed(Left, state.angled(), Right, state.prolonged())
                }
                StartCornerCrossed(Left, _, Right, _) => {
                    StartCornerCrossed(Left, state.angled(), Bottom, state.prolonged())
                }
                StartCornerCrossed(Left, _, Bottom, _) => {
                    StartCornerCrossed(Left, state.angled(), Left, state.prolonged())
                }
                StartCornerCrossed(Left, _, Left, _) => SideRegular,

                SideRegular => SideOverlap(Top, state.angled()),
                SideOverlap(Top, _) => SideOverlap(Right, state.angled()),
                SideOverlap(Right, _) => SideOverlap(Bottom, state.angled()),
                SideOverlap(Bottom, _) => SideOverlap(Left, state.angled()),
                SideOverlap(Left, _) => SideOutward(Top, state.angled()),
                SideOutward(Top, _) => SideOutward(Right, state.angled()),
                SideOutward(Right, _) => SideOutward(Bottom, state.angled()),
                SideOutward(Bottom, _) => SideOutward(Left, state.angled()),
                SideOutward(Left, _) => SideInward(Top, state.angled()),
                SideInward(Top, _) => SideInward(Right, state.angled()),
                SideInward(Right, _) => SideInward(Bottom, state.angled()),
                SideInward(Bottom, _) => SideInward(Left, state.angled()),
                SideInward(Left, _) => SideCrossed(Top, state.angled(), Top, state.angled()),
                SideCrossed(Top, _, Top, _) => {
                    SideCrossed(Top, state.angled(), Right, state.angled())
                }
                SideCrossed(Top, _, Right, _) => {
                    SideCrossed(Top, state.angled(), Bottom, state.angled())
                }
                SideCrossed(Top, _, Bottom, _) => {
                    SideCrossed(Top, state.angled(), Left, state.angled())
                }
                SideCrossed(Top, _, Left, _) => {
                    SideCrossed(Right, state.angled(), Top, state.angled())
                }
                SideCrossed(Right, _, Top, _) => {
                    SideCrossed(Right, state.angled(), Right, state.angled())
                }
                SideCrossed(Right, _, Right, _) => {
                    SideCrossed(Right, state.angled(), Bottom, state.angled())
                }
                SideCrossed(Right, _, Bottom, _) => {
                    SideCrossed(Right, state.angled(), Left, state.angled())
                }
                SideCrossed(Right, _, Left, _) => {
                    SideCrossed(Bottom, state.angled(), Top, state.angled())
                }
                SideCrossed(Bottom, _, Top, _) => {
                    SideCrossed(Bottom, state.angled(), Right, state.angled())
                }
                SideCrossed(Bottom, _, Right, _) => {
                    SideCrossed(Bottom, state.angled(), Bottom, state.angled())
                }
                SideCrossed(Bottom, _, Bottom, _) => {
                    SideCrossed(Bottom, state.angled(), Left, state.angled())
                }
                SideCrossed(Bottom, _, Left, _) => {
                    SideCrossed(Left, state.angled(), Top, state.angled())
                }
                SideCrossed(Left, _, Top, _) => {
                    SideCrossed(Left, state.angled(), Right, state.angled())
                }
                SideCrossed(Left, _, Right, _) => {
                    SideCrossed(Left, state.angled(), Bottom, state.angled())
                }
                SideCrossed(Left, _, Bottom, _) => {
                    SideCrossed(Left, state.angled(), Left, state.angled())
                }
                SideCrossed(Left, _, Left, _) => Cross {
                    0: Top,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Top,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Top, _, _, _, Top, _, _, _) => Cross {
                    0: Top,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Right,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Top, _, _, _, Right, _, _, _) => Cross {
                    0: Top,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Bottom,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Top, _, _, _, Bottom, _, _, _) => Cross {
                    0: Top,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Left,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Top, _, _, _, Left, _, _, _) => Cross {
                    0: Right,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Top,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Right, _, _, _, Top, _, _, _) => Cross {
                    0: Right,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Right,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Right, _, _, _, Right, _, _, _) => Cross {
                    0: Right,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Bottom,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Right, _, _, _, Bottom, _, _, _) => Cross {
                    0: Right,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Left,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Right, _, _, _, Left, _, _, _) => Cross {
                    0: Bottom,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Top,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Bottom, _, _, _, Top, _, _, _) => Cross {
                    0: Bottom,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Right,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Bottom, _, _, _, Right, _, _, _) => Cross {
                    0: Bottom,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Bottom,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Bottom, _, _, _, Bottom, _, _, _) => Cross {
                    0: Bottom,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Left,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Bottom, _, _, _, Left, _, _, _) => Cross {
                    0: Left,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Top,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Left, _, _, _, Top, _, _, _) => Cross {
                    0: Left,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Right,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Left, _, _, _, Right, _, _, _) => Cross {
                    0: Left,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Bottom,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Left, _, _, _, Bottom, _, _, _) => Cross {
                    0: Left,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Left,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                Cross(Left, _, _, _, Left, _, _, _) => EndCornerRegular,
                EndCornerRegular => EndCornerAngled(Top, state.angled()),
                EndCornerAngled(Top, _) => EndCornerAngled(Right, state.angled()),
                EndCornerAngled(Right, _) => EndCornerAngled(Bottom, state.angled()),
                EndCornerAngled(Bottom, _) => EndCornerAngled(Left, state.angled()),
                EndCornerAngled(Left, _) => EndCornerProlonged(Top, state.prolonged()),
                EndCornerProlonged(Top, _) => EndCornerProlonged(Right, state.prolonged()),
                EndCornerProlonged(Right, _) => EndCornerProlonged(Bottom, state.prolonged()),
                EndCornerProlonged(Bottom, _) => EndCornerProlonged(Left, state.prolonged()),
                EndCornerProlonged(Left, _) => {
                    EndCornerCrossed(Top, state.angled(), Top, state.prolonged())
                }

                EndCornerCrossed(Top, _, Top, _) => {
                    EndCornerCrossed(Top, state.angled(), Right, state.prolonged())
                }
                EndCornerCrossed(Top, _, Right, _) => {
                    EndCornerCrossed(Top, state.angled(), Bottom, state.prolonged())
                }
                EndCornerCrossed(Top, _, Bottom, _) => {
                    EndCornerCrossed(Top, state.angled(), Left, state.prolonged())
                }
                EndCornerCrossed(Top, _, Left, _) => {
                    EndCornerCrossed(Right, state.angled(), Top, state.prolonged())
                }
                EndCornerCrossed(Right, _, Top, _) => {
                    EndCornerCrossed(Right, state.angled(), Right, state.prolonged())
                }
                EndCornerCrossed(Right, _, Right, _) => {
                    EndCornerCrossed(Right, state.angled(), Bottom, state.prolonged())
                }
                EndCornerCrossed(Right, _, Bottom, _) => {
                    EndCornerCrossed(Right, state.angled(), Left, state.prolonged())
                }
                EndCornerCrossed(Right, _, Left, _) => {
                    EndCornerCrossed(Bottom, state.angled(), Top, state.prolonged())
                }
                EndCornerCrossed(Bottom, _, Top, _) => {
                    EndCornerCrossed(Bottom, state.angled(), Right, state.prolonged())
                }
                EndCornerCrossed(Bottom, _, Right, _) => {
                    EndCornerCrossed(Bottom, state.angled(), Bottom, state.prolonged())
                }
                EndCornerCrossed(Bottom, _, Bottom, _) => {
                    EndCornerCrossed(Bottom, state.angled(), Left, state.prolonged())
                }
                EndCornerCrossed(Bottom, _, Left, _) => {
                    EndCornerCrossed(Left, state.angled(), Top, state.prolonged())
                }
                EndCornerCrossed(Left, _, Top, _) => {
                    EndCornerCrossed(Left, state.angled(), Right, state.prolonged())
                }
                EndCornerCrossed(Left, _, Right, _) => {
                    EndCornerCrossed(Left, state.angled(), Bottom, state.prolonged())
                }
                EndCornerCrossed(Left, _, Bottom, _) => {
                    EndCornerCrossed(Left, state.angled(), Left, state.prolonged())
                }
                EndCornerCrossed(Left, _, Left, _) => {
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
        ct_event!(scroll up) | ct_event!(keycode press Left) => {
            use BorderSymbol::*;
            state.b_symbol = match state.b_symbol {
                StartCornerRegular => {
                    state.b_side = match state.b_side {
                        Top => Left,
                        Right => Top,
                        Bottom => Right,
                        Left => Bottom,
                    };
                    EndCornerCrossed(Top, state.angled(), Top, state.prolonged())
                }
                StartCornerAngled(_, _) => StartCornerRegular,
                StartCornerProlonged(_, _) => StartCornerAngled(Top, state.angled()),
                StartCornerCrossed(_, _, _, _) => StartCornerProlonged(Top, state.prolonged()),
                SideRegular => StartCornerCrossed(Top, state.angled(), Top, state.prolonged()),
                SideOverlap(_, _) => SideRegular,
                SideOutward(_, _) => SideOverlap(Top, state.angled()),
                SideInward(_, _) => SideOutward(Top, state.angled()),
                SideCrossed(_, _, _, _) => SideInward(Top.opposite(), state.angled()),
                Cross(_, _, _, _, _, _, _, _) => {
                    SideCrossed(Top, state.angled(), Top, state.angled())
                }
                EndCornerRegular => Cross {
                    0: Top,
                    1: state.angled(),
                    2: state.b_side.opposite(),
                    3: state.prolonged(),
                    4: Top,
                    5: state.angled(),
                    6: state.b_side.opposite(),
                    7: state.prolonged(),
                },
                EndCornerAngled(_, _) => EndCornerRegular,
                EndCornerProlonged(_, _) => EndCornerAngled(Top, state.angled()),
                EndCornerCrossed(_, _, _, _) => EndCornerProlonged(Top, state.prolonged()),
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
