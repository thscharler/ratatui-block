use crate::mini_salsa::theme::THEME;
use crate::mini_salsa::{layout_grid, run_ui, setup_logging, MiniSalsaState};
use log::debug;
use rat_event::{ct_event, Outcome};
use ratatui::layout::{Constraint, Layout, Rect, Spacing};
use ratatui::prelude::Widget;
use ratatui::style::Styled;
use ratatui::text::Line;
use ratatui::widgets::BorderType;
use ratatui::{crossterm, Frame};
use ratatui_block::block_grid::BlockGrid;
use ratatui_block::Side;

mod mini_salsa;

fn main() -> Result<(), anyhow::Error> {
    setup_logging()?;

    let mut data = Data {};
    let mut state = State {
        area: Default::default(),

        border: BorderType::Plain,
        inner: BorderType::Plain,

        hside: Side::Left,
        vside: Side::Top,

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

    border: BorderType,
    inner: BorderType,

    hside: Side,
    vside: Side,

    mono: bool,
}

impl State {}

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

    state.area = l0[1];

    buf.set_style(area, THEME.gray(0));

    let mut gr = BlockGrid::new(state.area)
        .border_type(state.border)
        .inner_border_type(state.inner)
        .horizontal_side(state.hside)
        .vertical_side(state.vside);
    for a in l_main {
        gr = gr.vertical(a[0].right() - state.area.left());
    }
    for a in l_main[0] {
        gr = gr.horizontal(a.bottom() - state.area.top());
    }
    gr.render(state.area, buf);

    let mut txt_area = l0[0];
    txt_area.y += 2;
    txt_area.height = 1;

    "F1: outer border"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F2: inner border"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;

    Line::from(format!("inner={:?}", state.inner)).render(txt_area, buf);
    txt_area.y += 1;
    Line::from(format!("border={:?}", state.border)).render(txt_area, buf);
    txt_area.y += 1;
    Line::from(format!("hside={:?}", state.hside)).render(txt_area, buf);
    txt_area.y += 1;
    Line::from(format!("vside={:?}", state.vside)).render(txt_area, buf);
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
            state.inner = match state.inner {
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
            state.hside = match state.hside {
                Side::Top => Side::Right,
                Side::Right => Side::Bottom,
                Side::Bottom => Side::Left,
                Side::Left => Side::Top,
            };
            Outcome::Changed
        }
        ct_event!(keycode press F(4)) => {
            state.vside = match state.vside {
                Side::Top => Side::Right,
                Side::Right => Side::Bottom,
                Side::Bottom => Side::Left,
                Side::Left => Side::Top,
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
