use crate::mini_salsa::theme::THEME;
use crate::mini_salsa::{layout_grid, run_ui, setup_logging, MiniSalsaState};
use log::debug;
use rat_event::{ct_event, Outcome};
use ratatui::layout::{Constraint, Direction, Layout, Rect, Spacing};
use ratatui::prelude::Widget;
use ratatui::style::{Style, Styled};
use ratatui::widgets::{Block, BorderType};
use ratatui::{crossterm, Frame};
use ratatui_block::{create_border, render_joint};
use std::rc::Rc;

mod mini_salsa;

fn main() -> Result<(), anyhow::Error> {
    setup_logging()?;

    let mut data = Data {};
    let mut state = State {
        max: 0,
        direction: Direction::Horizontal,
        border: BorderType::Plain,
        offset: 0,
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
    max: u16,

    direction: Direction,

    border: BorderType,
    offset: u16,
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

    let layout = layout_grid::<5, 5>(
        l0[1],
        Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(11),
            Constraint::Length(11),
            Constraint::Length(11),
            Constraint::Fill(1),
        ])
        .spacing(Spacing::Overlap(1)),
        Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(7),
            Constraint::Length(7),
            Constraint::Length(7),
            Constraint::Fill(1),
        ])
        .spacing(Spacing::Overlap(1)),
    );

    buf.set_style(area, THEME.deepblue(0));

    let all;
    if state.direction == Direction::Horizontal {
        state.max = layout[0][0].union(layout[4][0]).width;

        let area = layout[2][2];
        let above = Rect::new(
            layout[0][1].x + state.offset,
            layout[0][1].y,
            layout[2][2].width + 2,
            layout[2][2].height,
        );
        let below = Rect::new(
            layout[0][3].x + state.offset,
            layout[0][3].y,
            layout[2][2].width / 2,
            layout[2][2].height,
        );

        Block::bordered()
            .border_type(state.border)
            .render(above, buf);
        Block::bordered()
            .border_type(state.border)
            .render(below, buf);

        // all areas
        all = [above, area, below].iter().copied().collect::<Rc<[Rect]>>();
    } else {
        state.max = layout[0][0].union(layout[0][4]).height;

        let area = layout[2][2];
        let left = Rect::new(
            layout[1][0].x,
            layout[1][0].y + state.offset,
            layout[2][2].width,
            layout[2][2].height + 2,
        );
        let right = Rect::new(
            layout[3][0].x,
            layout[3][0].y + state.offset,
            layout[2][2].width,
            layout[2][2].height / 2,
        );

        Block::bordered()
            .border_type(state.border)
            .render(left, buf);
        Block::bordered()
            .border_type(state.border)
            .render(right, buf);

        // all areas
        all = [left, area, right].iter().copied().collect::<Rc<[Rect]>>();
    }

    // new block
    let bbb = create_border(all.clone(), 1, state.border);
    debug!("new block {:#?}", bbb);
    bbb.block
        .border_style(Style::new().fg(THEME.orange[2]))
        .render(all[1], buf);
    for (j, js) in bbb.joints {
        render_joint(state.border, js, j, all[1], buf);
    }

    let mut txt_area = l0[0];
    txt_area.y += 2;
    txt_area.height = 1;

    "F1: border"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    // txt_area.y += 1;
    // "F2: horizontal neighbours"
    //     .set_style(THEME.secondary_text())
    //     .render(txt_area, buf);
    // txt_area.y += 1;
    // "F3: vertical neighbours"
    //     .set_style(THEME.secondary_text())
    //     .render(txt_area, buf);
    // txt_area.y += 1;
    "F4: direction"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F5: advance position"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "Shift+F5: reduce position"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 2;

    format!("border={:?}", state.border).render(txt_area, buf);
    txt_area.y += 1;
    format!("offset={:?}", state.offset).render(txt_area, buf);
    txt_area.y += 1;
    format!("dir={:?}", state.direction).render(txt_area, buf);

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

        ct_event!(keycode press F(4)) => {
            state.offset = 0;
            state.direction = match state.direction {
                Direction::Horizontal => Direction::Vertical,
                Direction::Vertical => Direction::Horizontal,
            };
            Outcome::Changed
        }
        ct_event!(keycode press F(5)) => {
            if state.offset < state.max {
                state.offset += 1;
            }
            Outcome::Changed
        }
        ct_event!(keycode press SHIFT-F(5)) => {
            if state.offset > 0 {
                state.offset -= 1;
            }
            Outcome::Changed
        }
        _ => Outcome::Continue,
    };

    Ok(r)
}
