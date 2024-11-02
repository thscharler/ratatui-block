use crate::mini_salsa::theme::THEME;
use crate::mini_salsa::{layout_grid, run_ui, setup_logging, MiniSalsaState};
use rat_event::{ct_event, ConsumedEvent, HandleEvent, Outcome};
use ratatui::layout::{Constraint, Layout, Rect, Spacing};
use ratatui::prelude::Widget;
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType};
use ratatui::{crossterm, Frame};
use ratatui_block::block_joint::{render_joint, Joint, JointPosition, JointSide};

mod mini_salsa;

fn main() -> Result<(), anyhow::Error> {
    setup_logging()?;

    let mut data = Data {};
    let mut state = State {
        area: Default::default(),
        border: Default::default(),
        side: JointSide::Top,
        pos: JointPosition::FromStart(0),
        joint: Joint::Out(BorderType::Plain),
        join_border: Default::default(),
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
    side: JointSide,
    pos: JointPosition,
    joint: Joint,

    join_border: BorderType,
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
            Constraint::Length(15),
            Constraint::Fill(1),
            Constraint::Length(15),
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

    for a in 0..3 {
        for b in 0..3 {
            if a != 1 || b != 1 {
                Block::bordered()
                    .border_type(state.join_border)
                    .render(layout[a][b], buf);
            }
        }
    }

    Block::bordered()
        .border_type(state.border)
        .border_style(Style::new().fg(THEME.orange[2]))
        .render(layout[1][1], buf);

    render_joint(
        state.border,
        state.side,
        state.pos,
        state.joint,
        layout[1][1],
        buf,
    );

    let mut txt_area = l0[0];
    txt_area.y += 2;
    txt_area.height = 1;

    format!("{:?}", state.border).render(txt_area, buf);
    txt_area.y += 1;
    format!("{:?}", state.joint).render(txt_area, buf);
    txt_area.y += 1;
    format!("{:?}", state.side).render(txt_area, buf);
    txt_area.y += 1;
    format!("{:?}", state.pos).render(txt_area, buf);

    txt_area.y += 2;
    format!("{:?}", state.join_border).render(txt_area, buf);

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
            state.join_border = match state.join_border {
                BorderType::Plain => BorderType::Rounded,
                BorderType::Rounded => BorderType::Double,
                BorderType::Double => BorderType::Thick,
                BorderType::Thick => BorderType::QuadrantInside,
                BorderType::QuadrantInside => BorderType::QuadrantOutside,
                BorderType::QuadrantOutside => BorderType::Plain,
            };

            state.joint = match state.joint {
                Joint::Out(_) => Joint::Out(state.join_border),
                Joint::In(_) => Joint::In(state.join_border),
                Joint::Through(_) => Joint::Through(state.join_border),
                Joint::Manual(_) => unimplemented!(),
            };
            Outcome::Changed
        }
        ct_event!(keycode press F(3)) => {
            state.joint = match state.joint {
                Joint::Out(_) => Joint::In(state.join_border),
                Joint::In(_) => Joint::Through(state.join_border),
                Joint::Through(_) => Joint::Out(state.join_border),
                Joint::Manual(_) => unimplemented!(),
            };
            Outcome::Changed
        }
        ct_event!(keycode press F(4)) => {
            state.pos = match state.pos {
                JointPosition::FromStart(p) => {
                    let pos = match state.side {
                        JointSide::Top => {
                            if p >= state.area.width.saturating_sub(1) {
                                state.side = JointSide::Right;
                                0
                            } else {
                                p + 1
                            }
                        }
                        JointSide::Right => {
                            if p >= state.area.height.saturating_sub(1) {
                                state.side = JointSide::Bottom;
                                state.area.width.saturating_sub(1)
                            } else {
                                p + 1
                            }
                        }
                        JointSide::Bottom => {
                            if p == 0 {
                                state.side = JointSide::Left;
                                state.area.height.saturating_sub(1)
                            } else {
                                p - 1
                            }
                        }
                        JointSide::Left => {
                            if p == 0 {
                                state.side = JointSide::Top;
                                0
                            } else {
                                p - 1
                            }
                        }
                    };
                    JointPosition::FromStart(pos)
                }
                JointPosition::FromEnd(_) => {
                    unimplemented!()
                }
                JointPosition::AtPos(_) => {
                    unimplemented!()
                }
            };
            Outcome::Changed
        }
        ct_event!(keycode press SHIFT-F(4)) => {
            state.pos = match state.pos {
                JointPosition::FromStart(p) => {
                    let pos = match state.side {
                        JointSide::Top => {
                            if p == 0 {
                                state.side = JointSide::Left;
                                0
                            } else {
                                p - 1
                            }
                        }
                        JointSide::Left => {
                            if p >= state.area.height.saturating_sub(1) {
                                state.side = JointSide::Bottom;
                                0
                            } else {
                                p + 1
                            }
                        }
                        JointSide::Bottom => {
                            if p >= state.area.width.saturating_sub(1) {
                                state.side = JointSide::Right;
                                state.area.height.saturating_sub(1)
                            } else {
                                p + 1
                            }
                        }
                        JointSide::Right => {
                            if p == 0 {
                                state.side = JointSide::Top;
                                state.area.width.saturating_sub(1)
                            } else {
                                p - 1
                            }
                        }
                    };
                    JointPosition::FromStart(pos)
                }
                JointPosition::FromEnd(_) => {
                    unimplemented!()
                }
                JointPosition::AtPos(_) => {
                    unimplemented!()
                }
            };
            Outcome::Changed
        }
        _ => Outcome::Continue,
    };

    Ok(r)
}
