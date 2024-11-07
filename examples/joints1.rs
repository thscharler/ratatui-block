use crate::mini_salsa::theme::THEME;
use crate::mini_salsa::{layout_grid, run_ui, setup_logging, MiniSalsaState};
use rat_event::{ct_event, Outcome};
use ratatui::layout::{Constraint, Layout, Rect, Spacing};
use ratatui::prelude::Widget;
use ratatui::style::{Style, Styled};
use ratatui::widgets::{Block, BorderType};
use ratatui::{crossterm, Frame};
use ratatui_block::v2::{render_joint, Joint, JointKind, JointPosition, JointSide};

mod mini_salsa;

fn main() -> Result<(), anyhow::Error> {
    setup_logging()?;

    let mut data = Data {};
    let mut state = State {
        area: Default::default(),

        border: BorderType::Plain,
        hor_neighbour: BorderType::Plain,
        vert_neighbour: BorderType::Plain,

        joint: Joint::new(JointSide::Top, JointPosition::CrossStart(BorderType::Plain)),

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

    joint: Joint,

    mono: bool,
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

    buf.set_style(area, THEME.deepblue(0));

    if !state.joint.is_mirrored() {
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
    } else {
        for a in 0..3 {
            for b in 0..3 {
                if (a == 0 || a == 2) && (b == 0 || b == 2) {
                    Block::bordered()
                        .border_type(state.hor_neighbour)
                        .render(layout[a][b], buf);
                }
            }
        }
    }

    if state.mono {
        Block::bordered()
            .border_type(state.border)
            .render(layout[1][1], buf);
    } else {
        Block::bordered()
            .border_type(state.border)
            .border_style(Style::new().fg(THEME.orange[2]))
            .render(layout[1][1], buf);
    }

    render_joint(&state.joint, layout[1][1], buf);

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
    "F4: joint type"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "Left/Right: position"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F6: mirror"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 1;
    "F8: mono"
        .set_style(THEME.secondary_text())
        .render(txt_area, buf);
    txt_area.y += 2;

    format!("border={:?}", state.border).render(txt_area, buf);
    txt_area.y += 1;
    format!("joint={:?}", state.joint.get_border()).render(txt_area, buf);
    txt_area.y += 1;
    format!("scale={:?}", state.joint.get_kind()).render(txt_area, buf);
    txt_area.y += 1;
    format!("side={:?}", state.joint.get_side()).render(txt_area, buf);
    txt_area.y += 1;
    format!("pos={:?}", state.joint.get_position()).render(txt_area, buf);
    txt_area.y += 1;
    format!("mirror={:?}", state.joint.is_mirrored()).render(txt_area, buf);

    txt_area.y += 2;
    format!("hor={:?}", state.hor_neighbour).render(txt_area, buf);
    txt_area.y += 1;
    format!("vert={:?}", state.vert_neighbour).render(txt_area, buf);

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
            state.joint = state.joint.border(state.border);
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
            state.joint = state.joint.other(state.hor_neighbour);
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
            state.joint = state.joint.kind(match state.joint.get_kind() {
                JointKind::Inward => JointKind::Outward,
                JointKind::Outward => JointKind::Through,
                JointKind::Through => JointKind::Inward,
                JointKind::Manual(c) => JointKind::Manual(c),
            });
            Outcome::Changed
        }
        ct_event!(keycode press Right) => {
            match state.joint.get_side() {
                JointSide::Top => {
                    let next = match state.joint.get_position() {
                        JointPosition::CrossStart(_) => JointPosition::ProlongStart,
                        JointPosition::ProlongStart => JointPosition::Start,
                        JointPosition::Start => JointPosition::Pos(1),
                        JointPosition::Pos(n) => {
                            if n < state.area.width.saturating_sub(2) {
                                JointPosition::Pos(n + 1)
                            } else {
                                JointPosition::End
                            }
                        }
                        JointPosition::End => JointPosition::ProlongEnd,
                        JointPosition::ProlongEnd => JointPosition::CrossEnd(state.vert_neighbour),
                        JointPosition::CrossEnd(_) => {
                            state.joint = state.joint.side(JointSide::Right);
                            JointPosition::CrossStart(state.hor_neighbour)
                        }
                    };
                    state.joint = state.joint.position(next);
                }
                JointSide::Right => {
                    let next = match state.joint.get_position() {
                        JointPosition::CrossStart(_) => JointPosition::ProlongStart,
                        JointPosition::ProlongStart => JointPosition::Start,
                        JointPosition::Start => JointPosition::Pos(1),
                        JointPosition::Pos(n) => {
                            if n < state.area.height.saturating_sub(2) {
                                JointPosition::Pos(n + 1)
                            } else {
                                JointPosition::End
                            }
                        }
                        JointPosition::End => JointPosition::ProlongEnd,
                        JointPosition::ProlongEnd => JointPosition::CrossEnd(state.hor_neighbour),
                        JointPosition::CrossEnd(_) => {
                            state.joint = state.joint.side(JointSide::Bottom);
                            JointPosition::CrossEnd(state.vert_neighbour)
                        }
                    };
                    state.joint = state.joint.position(next);
                }
                JointSide::Bottom => {
                    let next = match state.joint.get_position() {
                        JointPosition::CrossEnd(_) => JointPosition::ProlongEnd,
                        JointPosition::ProlongEnd => JointPosition::End,
                        JointPosition::End => {
                            JointPosition::Pos(state.area.width.saturating_sub(2))
                        }
                        JointPosition::Pos(n) => {
                            if n > 1 {
                                JointPosition::Pos(n - 1)
                            } else {
                                JointPosition::Start
                            }
                        }
                        JointPosition::Start => JointPosition::ProlongStart,
                        JointPosition::ProlongStart => {
                            JointPosition::CrossStart(state.vert_neighbour)
                        }
                        JointPosition::CrossStart(_) => {
                            state.joint = state.joint.side(JointSide::Left);
                            JointPosition::CrossEnd(state.hor_neighbour)
                        }
                    };
                    state.joint = state.joint.position(next);
                }
                JointSide::Left => {
                    let next = match state.joint.get_position() {
                        JointPosition::CrossEnd(_) => JointPosition::ProlongEnd,
                        JointPosition::ProlongEnd => JointPosition::End,
                        JointPosition::End => {
                            JointPosition::Pos(state.area.height.saturating_sub(2))
                        }
                        JointPosition::Pos(n) => {
                            if n > 1 {
                                JointPosition::Pos(n - 1)
                            } else {
                                JointPosition::Start
                            }
                        }
                        JointPosition::Start => JointPosition::ProlongStart,
                        JointPosition::ProlongStart => {
                            JointPosition::CrossStart(state.hor_neighbour)
                        }
                        JointPosition::CrossStart(_) => {
                            state.joint = state.joint.side(JointSide::Top);
                            JointPosition::CrossStart(state.vert_neighbour)
                        }
                    };
                    state.joint = state.joint.position(next);
                }
            };
            Outcome::Changed
        }
        ct_event!(keycode press Left) => {
            match state.joint.get_side() {
                JointSide::Top => {
                    let next = match state.joint.get_position() {
                        JointPosition::CrossEnd(_) => JointPosition::ProlongEnd,
                        JointPosition::ProlongEnd => JointPosition::End,
                        JointPosition::End => {
                            JointPosition::Pos(state.area.width.saturating_sub(2))
                        }
                        JointPosition::Pos(n) => {
                            if n > 1 {
                                JointPosition::Pos(n - 1)
                            } else {
                                JointPosition::Start
                            }
                        }
                        JointPosition::Start => JointPosition::ProlongStart,
                        JointPosition::ProlongStart => {
                            JointPosition::CrossStart(state.vert_neighbour)
                        }
                        JointPosition::CrossStart(_) => {
                            state.joint = state.joint.side(JointSide::Left);
                            JointPosition::CrossStart(state.hor_neighbour)
                        }
                    };
                    state.joint = state.joint.position(next);
                }
                JointSide::Left => {
                    let next = match state.joint.get_position() {
                        JointPosition::CrossStart(_) => JointPosition::ProlongStart,
                        JointPosition::ProlongStart => JointPosition::Start,
                        JointPosition::Start => JointPosition::Pos(1),
                        JointPosition::Pos(n) => {
                            if n < state.area.height.saturating_sub(2) {
                                JointPosition::Pos(n + 1)
                            } else {
                                JointPosition::End
                            }
                        }
                        JointPosition::End => JointPosition::ProlongEnd,
                        JointPosition::ProlongEnd => JointPosition::CrossEnd(state.hor_neighbour),
                        JointPosition::CrossEnd(_) => {
                            state.joint = state.joint.side(JointSide::Bottom);
                            JointPosition::CrossStart(state.vert_neighbour)
                        }
                    };
                    state.joint = state.joint.position(next);
                }
                JointSide::Bottom => {
                    let next = match state.joint.get_position() {
                        JointPosition::CrossStart(_) => JointPosition::ProlongStart,
                        JointPosition::ProlongStart => JointPosition::Start,
                        JointPosition::Start => JointPosition::Pos(1),
                        JointPosition::Pos(n) => {
                            if n < state.area.width.saturating_sub(2) {
                                JointPosition::Pos(n + 1)
                            } else {
                                JointPosition::End
                            }
                        }
                        JointPosition::End => JointPosition::ProlongEnd,
                        JointPosition::ProlongEnd => JointPosition::CrossEnd(state.vert_neighbour),
                        JointPosition::CrossEnd(_) => {
                            state.joint = state.joint.side(JointSide::Right);
                            JointPosition::CrossEnd(state.hor_neighbour)
                        }
                    };
                    state.joint = state.joint.position(next);
                }
                JointSide::Right => {
                    let next = match state.joint.get_position() {
                        JointPosition::CrossEnd(_) => JointPosition::ProlongEnd,
                        JointPosition::ProlongEnd => JointPosition::End,
                        JointPosition::End => {
                            JointPosition::Pos(state.area.height.saturating_sub(2))
                        }
                        JointPosition::Pos(n) => {
                            if n > 1 {
                                JointPosition::Pos(n - 1)
                            } else {
                                JointPosition::Start
                            }
                        }
                        JointPosition::Start => JointPosition::ProlongStart,
                        JointPosition::ProlongStart => {
                            JointPosition::CrossStart(state.hor_neighbour)
                        }
                        JointPosition::CrossStart(_) => {
                            state.joint = state.joint.side(JointSide::Top);
                            JointPosition::CrossEnd(state.vert_neighbour)
                        }
                    };
                    state.joint = state.joint.position(next);
                }
            };
            Outcome::Changed
        }
        ct_event!(keycode press F(6)) => {
            state.joint = state.joint.mirrored(!state.joint.is_mirrored());
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
