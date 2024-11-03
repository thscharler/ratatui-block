use crate::mini_salsa::theme::THEME;
use crate::mini_salsa::{layout_grid, run_ui, setup_logging, MiniSalsaState};
use rat_event::{ct_event, Outcome};
use ratatui::layout::{Constraint, Layout, Rect, Spacing};
use ratatui::prelude::Widget;
use ratatui::style::{Style, Styled};
use ratatui::widgets::{Block, BorderType};
use ratatui::{crossterm, Frame};
use ratatui_block::{render_joint, Joint, JointSide};

mod mini_salsa;

fn main() -> Result<(), anyhow::Error> {
    setup_logging()?;

    let mut data = Data {};
    let mut state = State {
        area: Default::default(),
        border: Default::default(),
        side: JointSide::Top(0),
        joint: Joint::Out(BorderType::Plain),
        hor_neighbour: Default::default(),
        vert_neighbour: Default::default(),
        base_joint: Joint::Out(BorderType::Plain),
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
    joint: Joint,

    hor_neighbour: BorderType,
    vert_neighbour: BorderType,
    base_joint: Joint,
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
        .border_style(Style::new().fg(THEME.orange[2]))
        .render(layout[1][1], buf);

    render_joint(state.border, state.side, state.joint, layout[1][1], buf);

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
    format!("joint={:?}", state.joint).render(txt_area, buf);
    txt_area.y += 1;
    format!("side={:?}", state.side).render(txt_area, buf);

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
            state.joint = match state.joint {
                Joint::Out(b) => {
                    state.base_joint = Joint::In(b);
                    Joint::In(b)
                }
                Joint::In(b) => {
                    state.base_joint = Joint::Through(b);
                    Joint::Through(b)
                }
                Joint::Through(b) => {
                    state.base_joint = Joint::Out(b);
                    Joint::Out(b)
                }
                Joint::Corner(v, h) => Joint::Corner(v, h),
                Joint::Manual(_) => unimplemented!(),
            };
            Outcome::Changed
        }
        ct_event!(keycode press F(5)) => {
            state.side = match state.side {
                JointSide::Top(p) => {
                    if p >= state.area.width.saturating_sub(1) {
                        state.joint = Joint::Corner(state.vert_neighbour, state.hor_neighbour);
                        JointSide::TopRight
                    } else {
                        JointSide::Top(p + 1)
                    }
                }
                JointSide::TopRight => {
                    state.joint = state.base_joint.border(state.hor_neighbour);
                    JointSide::Right(0)
                }
                JointSide::Right(p) => {
                    if p >= state.area.height.saturating_sub(1) {
                        state.joint = Joint::Corner(state.vert_neighbour, state.hor_neighbour);
                        JointSide::BottomRight
                    } else {
                        JointSide::Right(p + 1)
                    }
                }
                JointSide::BottomRight => {
                    state.joint = state.base_joint.border(state.vert_neighbour);
                    JointSide::Bottom(state.area.width.saturating_sub(1))
                }
                JointSide::Bottom(p) => {
                    if p == 0 {
                        state.joint = Joint::Corner(state.vert_neighbour, state.hor_neighbour);
                        JointSide::BottomLeft
                    } else {
                        JointSide::Bottom(p - 1)
                    }
                }
                JointSide::BottomLeft => {
                    state.joint = state.base_joint.border(state.hor_neighbour);
                    JointSide::Left(state.area.height.saturating_sub(1))
                }
                JointSide::Left(p) => {
                    if p == 0 {
                        state.joint = Joint::Corner(state.vert_neighbour, state.hor_neighbour);
                        JointSide::TopLeft
                    } else {
                        JointSide::Left(p - 1)
                    }
                }
                JointSide::TopLeft => {
                    state.joint = state.base_joint.border(state.vert_neighbour);
                    JointSide::Top(0)
                }
            };
            Outcome::Changed
        }
        ct_event!(keycode press SHIFT-F(5)) => {
            state.side = match state.side {
                JointSide::Top(p) => {
                    if p == 0 {
                        state.joint = Joint::Corner(state.vert_neighbour, state.hor_neighbour);
                        JointSide::TopLeft
                    } else {
                        JointSide::Top(p - 1)
                    }
                }
                JointSide::TopLeft => {
                    state.joint = state.base_joint.border(state.hor_neighbour);
                    JointSide::Left(0)
                }
                JointSide::Left(p) => {
                    if p >= state.area.height.saturating_sub(1) {
                        state.joint = Joint::Corner(state.vert_neighbour, state.hor_neighbour);
                        JointSide::BottomLeft
                    } else {
                        JointSide::Left(p + 1)
                    }
                }
                JointSide::BottomLeft => {
                    state.joint = state.base_joint.border(state.vert_neighbour);
                    JointSide::Bottom(0)
                }
                JointSide::Bottom(p) => {
                    if p >= state.area.width.saturating_sub(1) {
                        state.joint = Joint::Corner(state.vert_neighbour, state.hor_neighbour);
                        JointSide::BottomRight
                    } else {
                        JointSide::Bottom(p + 1)
                    }
                }
                JointSide::BottomRight => {
                    state.joint = state.base_joint.border(state.hor_neighbour);
                    JointSide::Right(state.area.height.saturating_sub(1))
                }
                JointSide::Right(p) => {
                    if p == 0 {
                        state.joint = Joint::Corner(state.vert_neighbour, state.hor_neighbour);
                        JointSide::TopRight
                    } else {
                        JointSide::Right(p - 1)
                    }
                }
                JointSide::TopRight => {
                    state.joint = state.base_joint.border(state.vert_neighbour);
                    JointSide::Top(state.area.width.saturating_sub(1))
                }
            };
            Outcome::Changed
        }
        _ => Outcome::Continue,
    };

    Ok(r)
}
