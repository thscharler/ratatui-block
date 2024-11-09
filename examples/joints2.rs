use crate::mini_salsa::theme::THEME;
use crate::mini_salsa::{layout_grid, run_ui, setup_logging, MiniSalsaState};
use log::debug;
use rat_event::{ct_event, Outcome};
use ratatui::layout::{Constraint, Layout, Position, Rect, Spacing};
use ratatui::prelude::Widget;
use ratatui::style::{Style, Styled};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType};
use ratatui::{crossterm, Frame};
use ratatui_block::block_border::BlockBorder;
use std::hint::black_box;
use std::time::SystemTime;

mod mini_salsa;

fn main() -> Result<(), anyhow::Error> {
    setup_logging()?;

    let mut data = Data {};
    let mut state = State {
        max_offset: 0,
        variation: 0,
        mono: false,
        border: BorderType::Plain,
        other: BorderType::Plain,
        offset: 0,
        joint_area: Default::default(),
        joint_idx: 0,
        joint_max: 0,
    };

    run_ui(
        "╒═╤═╛2",
        handle_buttons,
        repaint_buttons,
        &mut data,
        &mut state,
    )
}

struct Data {}

struct State {
    variation: u8,
    mono: bool,
    border: BorderType,
    other: BorderType,
    max_offset: u16,
    offset: u16,

    joint_area: Rect,
    joint_idx: usize,
    joint_max: usize,
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
    let borders;
    match state.variation {
        0 => {
            // bigger above, smaller below
            state.max_offset = layout[0][0].union(layout[4][0]).width;

            let area = layout[2][2];
            let above = Rect::new(
                layout[0][1].x + state.offset,
                layout[0][1].y,
                13,
                layout[2][2].height,
            );
            let below = Rect::new(
                layout[0][3].x + state.offset,
                layout[0][3].y,
                5,
                layout[2][2].height,
            );
            // all areas
            all = vec![area, above, below];
            borders = vec![state.border, state.other, state.other];
        }
        1 => {
            // bigger below, smaller above
            state.max_offset = layout[0][0].union(layout[4][0]).width;

            let area = layout[2][2];
            let above = Rect::new(
                layout[0][1].x + state.offset,
                layout[0][1].y,
                5,
                layout[2][2].height,
            );
            let below = Rect::new(
                layout[0][3].x + state.offset,
                layout[0][3].y,
                13,
                layout[2][2].height,
            );
            // all areas
            all = vec![area, above, below];
            borders = vec![state.border, state.other, state.other];
        }
        2 => {
            // same,same
            state.max_offset = layout[0][0].union(layout[4][0]).width;

            let area = layout[2][2];
            let above = Rect::new(
                layout[0][1].x + state.offset,
                layout[0][1].y,
                layout[2][2].width,
                layout[2][2].height,
            );
            let below = Rect::new(
                layout[0][3].x + state.offset,
                layout[0][3].y,
                layout[2][2].width,
                layout[2][2].height,
            );
            // all areas
            all = vec![area, above, below];
            borders = vec![state.border, state.other, state.other];
        }
        3 => {
            // bigger left, smaller right
            state.max_offset = layout[0][0].union(layout[0][4]).height;

            let area = layout[2][2];
            let left = Rect::new(
                layout[1][0].x,
                layout[1][0].y + state.offset,
                layout[2][2].width,
                9,
            );
            let right = Rect::new(
                layout[3][0].x,
                layout[3][0].y + state.offset,
                layout[2][2].width,
                4,
            );
            // all areas
            all = vec![area, left, right];
            borders = vec![state.border, state.other, state.other];
        }
        4 => {
            // bigger right, smaller left
            state.max_offset = layout[0][0].union(layout[0][4]).height;

            let area = layout[2][2];
            let left = Rect::new(
                layout[1][0].x,
                layout[1][0].y + state.offset,
                layout[2][2].width,
                4,
            );
            let right = Rect::new(
                layout[3][0].x,
                layout[3][0].y + state.offset,
                layout[2][2].width,
                9,
            );
            // all areas
            all = vec![area, left, right];
            borders = vec![state.border, state.other, state.other];
        }
        5 => {
            // same, same
            state.max_offset = layout[0][0].union(layout[0][4]).height;

            let area = layout[2][2];
            let left = Rect::new(
                layout[1][0].x,
                layout[1][0].y + state.offset,
                layout[2][2].width,
                layout[2][2].height,
            );
            let right = Rect::new(
                layout[3][0].x,
                layout[3][0].y + state.offset,
                layout[2][2].width,
                layout[2][2].height,
            );
            // all areas
            all = vec![area, left, right];
            borders = vec![state.border, state.other, state.other];
        }

        6 => {
            // both above
            state.max_offset = layout[0][0].union(layout[4][0]).width;

            let area = layout[2][2];
            let above1 = Rect::new(
                layout[0][1].x + state.offset,
                layout[0][1].y,
                13,
                layout[2][2].height,
            );
            let above2 = Rect::new(
                layout[0][1].x + state.offset + 12,
                layout[0][1].y,
                13,
                layout[2][2].height,
            );
            let below1 = Rect::new(
                layout[0][3].x + state.offset,
                layout[0][3].y,
                13,
                layout[2][2].height,
            );
            let below2 = Rect::new(
                layout[0][3].x + state.offset + 12,
                layout[0][3].y,
                13,
                layout[2][2].height,
            );
            // all areas
            all = vec![area, above2, above1, below2, below1];
            borders = vec![
                state.border,
                state.other,
                BorderType::Thick,
                state.other,
                state.border,
            ];
        }

        _ => {
            unreachable!()
        }
    }

    for i in 1..all.len() {
        Block::bordered()
            .border_type(borders[i])
            .render(all[i], buf);
    }
    Block::bordered()
        .border_type(borders[0])
        .render(all[0], buf);

    // new block
    let tt = SystemTime::now();
    for _ in 0..100_000 {
        _ = black_box(BlockBorder::from_layout(
            all.as_slice(),
            borders.as_slice(),
            0,
        ));
    }
    debug!("tt {:?}", tt.elapsed()?.as_secs_f64() * 1e9 / 100_000.);

    let mut bbb = BlockBorder::from_layout(all.as_slice(), borders.as_slice(), 0);

    // bbb = bbb.border_set(Rc::new(OldSymbolSet {
    //     symbol_set: border::PROPORTIONAL_WIDE,
    // }));

    if state.mono {
        (&bbb).render(all[0], buf);
    } else {
        bbb = bbb.border_style(Style::new().fg(THEME.orange[2]));
        (&bbb).render(all[0], buf);
    }

    let mut txt_area = l0[0];
    txt_area.y += 2;
    txt_area.height = 1;

    "F1: border"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F2: other border"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F4: variation"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "Left/Right: position"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F8: monochrome"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 2;

    format!("border={:?}", state.border).render(txt_area, buf);
    txt_area.y += 1;
    format!("other={:?}", state.other).render(txt_area, buf);
    txt_area.y += 1;
    format!("offset={:?}", state.offset).render(txt_area, buf);
    txt_area.y += 1;
    format!("dir={:?}", state.variation).render(txt_area, buf);
    txt_area.y += 1;
    format!("area[0]={}", rect_dbg2(all[0])).render(txt_area, buf);
    txt_area.y += 1;
    format!("area[1]={}", rect_dbg2(all[1])).render(txt_area, buf);
    txt_area.y += 1;
    format!("area[2]={}", rect_dbg2(all[2])).render(txt_area, buf);

    state.joint_area = Rect::new(l0[0].x, l0[0].bottom() - 9, l0[0].width, 9);

    state.joint_max = (bbb.get_area().width * 2 + bbb.get_area().height * 2) as usize;
    buf.set_style(state.joint_area, THEME.cyan(1));

    if state.joint_idx < state.joint_max {
        let top_start = 0;
        let top_end = bbb.get_area().width as usize - 1;
        let right_start = bbb.get_area().width as usize;
        let right_end = bbb.get_area().width as usize + bbb.get_area().height as usize - 3;
        let bottom_start = bbb.get_area().width as usize + bbb.get_area().height as usize - 2;
        let bottom_end = bbb.get_area().width as usize + bbb.get_area().height as usize - 2
            + bbb.get_area().width as usize
            - 1;
        let left_start = bbb.get_area().width as usize + bbb.get_area().height as usize - 2
            + bbb.get_area().width as usize;
        let left_end = bbb.get_area().width as usize + bbb.get_area().height as usize - 2
            + bbb.get_area().width as usize
            + bbb.get_area().height as usize
            - 3;

        let (name, border_sym) = if top_start == state.joint_idx {
            ("top left", *bbb.top_left())
        } else if (top_start + 1..top_end).contains(&state.joint_idx) {
            ("top", bbb.top()[state.joint_idx - (top_start + 1)])
        } else if top_end == state.joint_idx {
            ("top_right", *bbb.top_right())
        } else if (right_start..=right_end).contains(&state.joint_idx) {
            ("right", bbb.right()[state.joint_idx - right_start])
        } else if bottom_start == state.joint_idx {
            ("bottom left", *bbb.bottom_left())
        } else if (bottom_start + 1..bottom_end).contains(&state.joint_idx) {
            ("bottom", bbb.bottom()[state.joint_idx - (bottom_start + 1)])
        } else if bottom_end == state.joint_idx {
            ("bottom right", *bbb.bottom_right())
        } else if (left_start..=left_end).contains(&state.joint_idx) {
            ("left", bbb.left()[state.joint_idx - left_start])
        } else {
            unreachable!("invalid idx");
        };

        let mut area = state.joint_area;
        area.height = 1;
        format!("#{:?} of {:?}", state.joint_idx + 1, state.joint_max).render(area, buf);
        area.y += 1;
        format!("{}", name).render(area, buf);
        area.y += 1;
        area.height = 8;
        Text::from(format!("{:#?}", border_sym)).render(area, buf);
    }

    Ok(())
}

const VARIANT_COUNT: u8 = 7;

pub fn rect_dbg(area: Rect) -> String {
    use std::fmt::Write;
    let mut buf = String::new();
    _ = write!(buf, "{}:{}+{}+{}", area.x, area.y, area.width, area.height);
    buf
}

pub fn rect_dbg2(area: Rect) -> String {
    use std::fmt::Write;
    let mut buf = String::new();
    _ = write!(
        buf,
        "{}:{}-{}:{}",
        area.x,
        area.y,
        area.x + area.width.saturating_sub(1),
        area.y + area.height.saturating_sub(1)
    );
    buf
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
            state.other = match state.other {
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
            state.variation = (state.variation + 1) % VARIANT_COUNT;
            Outcome::Changed
        }
        ct_event!(keycode press SHIFT-F(4)) => {
            state.offset = 0;
            state.variation = if state.variation > 0 {
                state.variation - 1
            } else {
                VARIANT_COUNT - 1
            };
            Outcome::Changed
        }
        ct_event!(keycode press Right) | ct_event!(keycode press Down) => {
            if state.offset < state.max_offset {
                state.offset += 1;
            }
            Outcome::Changed
        }
        ct_event!(keycode press Left) | ct_event!(keycode press Up) => {
            if state.offset > 0 {
                state.offset -= 1;
            }
            Outcome::Changed
        }
        ct_event!(keycode press F(8)) => {
            state.mono = !state.mono;
            Outcome::Changed
        }

        ct_event!(scroll down for x, y) if state.joint_area.contains(Position::new(*x, *y)) => {
            state.joint_idx = (state.joint_idx + 1).clamp(0, state.joint_max.saturating_sub(1));
            Outcome::Changed
        }
        ct_event!(scroll up for x, y) if state.joint_area.contains(Position::new(*x, *y)) => {
            state.joint_idx = state.joint_idx.saturating_sub(1);
            Outcome::Changed
        }
        ct_event!(scroll down for _x, _y) => {
            if state.offset < state.max_offset {
                state.offset += 1;
            }
            Outcome::Changed
        }
        ct_event!(scroll up for _x, _y) => {
            if state.offset > 0 {
                state.offset -= 1;
            }
            Outcome::Changed
        }

        ct_event!(keycode press PageDown) => {
            state.joint_idx = (state.joint_idx + 1).clamp(0, state.joint_max.saturating_sub(1));
            Outcome::Changed
        }
        ct_event!(keycode press PageUp) => {
            state.joint_idx = state.joint_idx.saturating_sub(1);
            Outcome::Changed
        }

        _ => Outcome::Continue,
    };

    Ok(r)
}
