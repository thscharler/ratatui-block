use crate::mini_salsa::theme::THEME;
use crate::mini_salsa::{layout_grid, run_ui, setup_logging, MiniSalsaState};
use log::debug;
use rat_event::{ct_event, Outcome};
use ratatui::layout::{Constraint, Direction, Layout, Position, Rect, Spacing};
use ratatui::prelude::Widget;
use ratatui::style::{Style, Styled};
use ratatui::widgets::{Block, BorderType};
use ratatui::{crossterm, Frame};
use ratatui_block::v3::{BorderGlyph, Side};
use ratatui_block::{create_border, render_joint, v3};
use std::rc::Rc;

mod mini_salsa;

fn main() -> Result<(), anyhow::Error> {
    setup_logging()?;

    debug!("BorderGlyph {}", size_of::<BorderGlyph>());
    debug!("Side {}", size_of::<Side>());
    debug!("Position {}", size_of::<v3::Position>());
    debug!("BorderType {}", size_of::<BorderType>());

    let mut data = Data {};
    let mut state = State {
        max_offset: 0,
        direction: Direction::Horizontal,
        mono: false,
        border: BorderType::QuadrantInside,
        other: BorderType::QuadrantInside,
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
    direction: Direction,
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
    if state.direction == Direction::Horizontal {
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

        Block::bordered()
            .border_type(state.other)
            .render(above, buf);
        Block::bordered()
            .border_type(state.other)
            .render(below, buf);

        // all areas
        all = [above, area, below].iter().copied().collect::<Rc<[Rect]>>();
    } else {
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

        Block::bordered().border_type(state.other).render(left, buf);
        Block::bordered()
            .border_type(state.other)
            .render(right, buf);

        // all areas
        all = [left, area, right].iter().copied().collect::<Rc<[Rect]>>();
    }

    // new block
    let bbb = create_border(all.as_ref(), &[state.other, state.border, state.other], 1);
    if state.mono {
        bbb.block.render(all[1], buf);
    } else {
        bbb.block
            .border_style(Style::new().fg(THEME.orange[2]))
            .render(all[1], buf);
    }
    for joint in bbb.joints.iter() {
        render_joint(joint, all[1], buf);
    }

    state.joint_area = Rect::new(l0[0].x, l0[0].bottom() - 7, l0[0].width, 7);
    state.joint_max = bbb.joints.len();
    buf.set_style(state.joint_area, THEME.cyan(1));

    if let Some(joint) = bbb.joints.get(state.joint_idx) {
        let joint = joint.normalized(all[1]);

        let mut area = state.joint_area;
        area.height = 1;
        format!("#{:?} of {:?}", state.joint_idx + 1, bbb.joints.len()).render(area, buf);
        area.y += 1;
        format!("{:?}", joint.get_border()).render(area, buf);
        area.y += 1;
        format!("{:?}", joint.get_kind()).render(area, buf);
        area.y += 1;
        format!("{:?}", joint.get_side()).render(area, buf);
        area.y += 1;
        format!("{:?}", joint.get_position()).render(area, buf);
        area.y += 1;
        format!("{:?}", joint.is_mirrored()).render(area, buf);
        area.y += 1;
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
    "F4: direction"
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
            state.direction = match state.direction {
                Direction::Horizontal => Direction::Vertical,
                Direction::Vertical => Direction::Horizontal,
            };
            Outcome::Changed
        }
        ct_event!(keycode press Right) if state.direction == Direction::Horizontal => {
            if state.offset < state.max_offset {
                state.offset += 1;
            }
            Outcome::Changed
        }
        ct_event!(keycode press Left) if state.direction == Direction::Horizontal => {
            if state.offset > 0 {
                state.offset -= 1;
            }
            Outcome::Changed
        }
        ct_event!(keycode press Down) if state.direction == Direction::Vertical => {
            if state.offset < state.max_offset {
                state.offset += 1;
            }
            Outcome::Changed
        }
        ct_event!(keycode press Up) if state.direction == Direction::Vertical => {
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

        ct_event!(keycode press Down) if state.direction == Direction::Horizontal => {
            state.joint_idx = (state.joint_idx + 1).clamp(0, state.joint_max.saturating_sub(1));
            Outcome::Changed
        }
        ct_event!(keycode press Up) => {
            state.joint_idx = state.joint_idx.saturating_sub(1);
            Outcome::Changed
        }
        ct_event!(keycode press Right) if state.direction == Direction::Vertical => {
            state.joint_idx = (state.joint_idx + 1).clamp(0, state.joint_max.saturating_sub(1));
            Outcome::Changed
        }
        ct_event!(keycode press Left) if state.direction == Direction::Vertical => {
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

        _ => Outcome::Continue,
    };

    Ok(r)
}
