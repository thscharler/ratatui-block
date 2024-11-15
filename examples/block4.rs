use crate::mini_salsa::theme::THEME;
use crate::mini_salsa::{layout_grid, run_ui, setup_logging, MiniSalsaState};
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

        hside: Side::Top,
        hborder: BorderType::Plain,
        vside: Side::Right,

        vborder: BorderType::Plain,
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

    hside: Side,
    hborder: BorderType,
    vside: Side,
    vborder: BorderType,

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
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]),
        Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]),
    );

    state.area = l0[1];
    buf.set_style(area, THEME.gray(0));

    let l_grid = layout_grid::<3, 3>(
        l_main[1][1],
        Layout::horizontal([
            Constraint::Length(10),
            Constraint::Fill(1),
            Constraint::Length(10),
        ])
        .spacing(Spacing::Space(1)),
        Layout::vertical([
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(5),
        ])
        .spacing(Spacing::Space(1)),
    );

    BlockGrid::from_layout(&[
        l_grid[0][0],
        l_grid[1][0],
        l_grid[2][0],
        l_grid[0][1],
        l_grid[0][2],
    ])
    .border_type(state.border)
    .horizontal_border_type(state.hborder)
    .horizontal_side(state.hside)
    .vertical_border_type(state.vborder)
    .vertical_side(state.vside)
    .render(l_main[1][1], buf);

    // let mut gr = BlockGrid::new()
    //     .border_type(state.border)
    //     .horizontal_border_type(state.hborder)
    //     .horizontal_side(state.hside)
    //     .vertical_border_type(state.vborder)
    //     .vertical_side(state.vside);
    // for a in l_main {
    //     gr = gr.vertical(a[0].right() - state.area.left());
    // }
    // for a in l_main[0] {
    //     gr = gr.horizontal(a.bottom() - state.area.top());
    // }
    // gr.render(state.area, buf);

    let mut txt_area = l0[0];
    txt_area.y += 2;
    txt_area.height = 1;

    "F1: outer border"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F3: horizontal border"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F4: vertical border"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F5: horizontal side"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F6: vertical side"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;

    Line::from(format!("hborder={:?}", state.hborder)).render(txt_area, buf);
    txt_area.y += 1;
    Line::from(format!("vborder={:?}", state.vborder)).render(txt_area, buf);
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
        ct_event!(keycode press F(3)) => {
            state.hborder = match state.hborder {
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
            state.vborder = match state.vborder {
                BorderType::Plain => BorderType::Rounded,
                BorderType::Rounded => BorderType::Double,
                BorderType::Double => BorderType::Thick,
                BorderType::Thick => BorderType::QuadrantInside,
                BorderType::QuadrantInside => BorderType::QuadrantOutside,
                BorderType::QuadrantOutside => BorderType::Plain,
            };
            Outcome::Changed
        }
        ct_event!(keycode press F(5)) => {
            state.hside = match state.hside {
                Side::Top => Side::Right,
                Side::Right => Side::Bottom,
                Side::Bottom => Side::Left,
                Side::Left => Side::Top,
            };
            Outcome::Changed
        }
        ct_event!(keycode press F(6)) => {
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
