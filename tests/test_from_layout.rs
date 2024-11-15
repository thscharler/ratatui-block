use ratatui::layout::{Constraint, Layout, Rect, Spacing};
use ratatui::widgets::BorderType;
use ratatui_block::block_border::BlockBorder;
use std::hint::black_box;
use std::time::SystemTime;

#[test]
fn from_layout() -> Result<(), anyhow::Error> {
    let layout = layout_grid::<5, 5>(
        Rect::new(0, 0, 80, 25),
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

    let area = layout[2][2];
    let above1 = Rect::new(layout[0][1].x, layout[0][1].y, 13, layout[2][2].height);
    let above2 = Rect::new(layout[0][1].x + 12, layout[0][1].y, 13, layout[2][2].height);
    let below1 = Rect::new(layout[0][3].x, layout[0][3].y, 13, layout[2][2].height);
    let below2 = Rect::new(layout[0][3].x + 12, layout[0][3].y, 13, layout[2][2].height);
    // all areas
    let all = vec![area, above2, above1, below2, below1];
    let borders = vec![
        BorderType::Plain,
        BorderType::Double,
        BorderType::Thick,
        BorderType::Double,
        BorderType::Plain,
    ];

    // new block
    let tt = SystemTime::now();
    for _ in 0..100_000 {
        _ = black_box(BlockBorder::from_layout(
            all.as_slice(),
            borders.as_slice(),
            0,
        ));
    }
    eprintln!("tt {:?}", tt.elapsed()?.div_f64(100_000f64));

    Ok(())
}

pub fn layout_grid<const X: usize, const Y: usize>(
    area: Rect,
    horizontal: Layout,
    vertical: Layout,
) -> [[Rect; Y]; X] {
    let hori = horizontal.split(Rect::new(area.x, 0, area.width, 0));
    let vert = vertical.split(Rect::new(0, area.y, 0, area.height));

    let mut res = [[Rect::default(); Y]; X];
    for x in 0..X {
        let coldata = &mut res[x];
        for y in 0..Y {
            coldata[y].x = hori[x].x;
            coldata[y].width = hori[x].width;
            coldata[y].y = vert[y].y;
            coldata[y].height = vert[y].height;
        }
    }

    res
}
