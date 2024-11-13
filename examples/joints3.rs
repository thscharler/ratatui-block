use crate::mini_salsa::theme::THEME;
use crate::mini_salsa::{layout_grid, run_ui, setup_logging, MiniSalsaState};
use rat_event::{ct_event, Outcome};
use ratatui::layout::{Constraint, Layout, Rect, Spacing};
use ratatui::prelude::Widget;
use ratatui::style::{Style, Styled};
use ratatui::widgets::BorderType;
use ratatui::{crossterm, Frame};
use ratatui_block::block_border::BlockBorder;
use ratatui_block::border_symbols::{symbol_set, AsciiSymbolSet, StarSymbolSet};
use std::rc::Rc;

mod mini_salsa;

fn main() -> Result<(), anyhow::Error> {
    setup_logging()?;

    let mut data = Data {};
    let mut state = State {
        area: Default::default(),

        border: BorderType::Plain,
        neighbour: BorderType::Plain,

        variant: 0,
        mono: false,
    };

    run_ui(
        "╒═╤═╛3",
        handle_buttons,
        repaint_buttons,
        &mut data,
        &mut state,
    )
}

struct Data {}

struct State {
    area: Rect,

    variant: u8,
    border: BorderType,
    neighbour: BorderType,
    mono: bool,
}

impl State {}

const MAX_VARIANT: u8 = 8;

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

    let l_plane_a = layout_grid::<5, 5>(
        l0[1],
        Layout::horizontal([
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Fill(1),
            Constraint::Length(10),
            Constraint::Length(10),
        ])
        .spacing(Spacing::Overlap(1)),
        Layout::vertical([
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Length(5),
        ])
        .spacing(Spacing::Overlap(1)),
    );

    state.area = l_main[1][1];

    buf.set_style(area, THEME.gray(0));

    let mut areas = Vec::new();
    let mut borders = Vec::new();
    if state.variant == 0 {
        // none
    } else if state.variant == 1 {
        areas.push(l_plane_a[0][0]);
        areas.push(l_plane_a[4][0]);
        areas.push(l_plane_a[0][4]);
        areas.push(l_plane_a[4][4]);
    } else if state.variant == 2 {
        areas.push(l_plane_a[1][0]);
        areas.push(l_plane_a[3][0]);
        areas.push(l_plane_a[1][4]);
        areas.push(l_plane_a[3][4]);
    } else if state.variant == 3 {
        areas.push(l_plane_a[0][1]);
        areas.push(l_plane_a[4][1]);
        areas.push(l_plane_a[0][3]);
        areas.push(l_plane_a[4][3]);
    } else if state.variant == 4 {
        areas.push(l_plane_a[0][0]);
        areas.push(l_plane_a[4][0]);
        areas.push(l_plane_a[0][4]);
        areas.push(l_plane_a[4][4]);

        areas.push(l_plane_a[1][0]);
        areas.push(l_plane_a[3][0]);
        areas.push(l_plane_a[1][4]);
        areas.push(l_plane_a[3][4]);
    } else if state.variant == 5 {
        areas.push(l_plane_a[0][0]);
        areas.push(l_plane_a[4][0]);
        areas.push(l_plane_a[0][4]);
        areas.push(l_plane_a[4][4]);

        areas.push(l_plane_a[0][1]);
        areas.push(l_plane_a[4][1]);
        areas.push(l_plane_a[0][3]);
        areas.push(l_plane_a[4][3]);
    } else if state.variant == 6 {
        areas.push(l_plane_a[0][0]);
        areas.push(l_plane_a[4][0]);
        areas.push(l_plane_a[0][4]);
        areas.push(l_plane_a[4][4]);

        areas.push(l_plane_a[1][0]);
        areas.push(l_plane_a[3][0]);
        areas.push(l_plane_a[1][4]);
        areas.push(l_plane_a[3][4]);

        areas.push(l_plane_a[0][1]);
        areas.push(l_plane_a[4][1]);
        areas.push(l_plane_a[0][3]);
        areas.push(l_plane_a[4][3]);
    } else if state.variant == 7 {
        areas.push(l_plane_a[0][0].union(l_plane_a[1][0]));
        areas.push(l_plane_a[4][0].union(l_plane_a[3][0]));
        areas.push(l_plane_a[0][4].union(l_plane_a[1][4]));
        areas.push(l_plane_a[4][4].union(l_plane_a[3][4]));
    } else if state.variant == 8 {
        areas.push(l_plane_a[0][0].union(l_plane_a[0][1]));
        areas.push(l_plane_a[4][0].union(l_plane_a[4][1]));
        areas.push(l_plane_a[0][4].union(l_plane_a[0][3]));
        areas.push(l_plane_a[4][4].union(l_plane_a[4][3]));
    }

    borders.extend(areas.iter().map(|v| state.neighbour));

    areas.push(state.area);
    borders.push(state.border);

    // debug!("******************************");

    for i in 0..areas.len().saturating_sub(1) {
        let mut bb = BlockBorder::from_layout(areas.as_slice(), borders.as_slice(), i);
        bb.render(areas[i], buf);
    }

    let mut bb = BlockBorder::from_layout(areas.as_slice(), borders.as_slice(), areas.len() - 1);
    // bb = bb.border_set(Rc::new(AsciiSymbolSet));
    if !state.mono {
        bb = bb.border_style(Style::new().fg(THEME.orange[3]));
    }
    bb.render(areas[areas.len() - 1], buf);

    let mut txt_area = l0[0];
    txt_area.y += 2;
    txt_area.height = 1;

    "F1: main border"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F2: other border"
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

    format!("variant={:?}", state.variant).render(txt_area, buf);
    txt_area.y += 1;
    format!("neighbour={:?}", state.neighbour).render(txt_area, buf);
    txt_area.y += 1;
    format!("border={:?}", state.border).render(txt_area, buf);
    txt_area.y += 1;

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
            state.neighbour = match state.neighbour {
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
            if state.variant < MAX_VARIANT {
                state.variant = state.variant + 1;
            }
            Outcome::Changed
        }
        ct_event!(scroll up) | ct_event!(keycode press Left) => {
            if state.variant > 0 {
                state.variant = state.variant - 1;
            }
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
