use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::Style;
use ratatui::symbols::border;
use ratatui::widgets::{Block, BorderType, Widget};
use ratatui_block::block_border::BlockBorder;
use ratatui_block::border_symbols::OldSymbolSet;
use ratatui_block::{BorderSymbol, Side};
use std::hint::black_box;
use std::time::SystemTime;

#[test]
fn test_1() {
    let mut buf = Buffer::empty(Rect::new(0, 0, 80, 25));
    let buf = &mut buf;

    let area = Rect::new(1, 1, 40, 20);

    let et = SystemTime::now();
    for _ in 0..100_000 {
        black_box({
            let b = BlockBorder::new().border_set(Box::new(OldSymbolSet {
                border_type: BorderType::Plain,
                symbol_set: border::PLAIN,
            }));
            buf.set_style(area, Style::new());
            b.render(area, buf);
        });
    }
    eprintln!("direct {:?}", et.elapsed().unwrap().div_f64(100_000.));
}

#[test]
fn test_2() {
    let mut buf = Buffer::empty(Rect::new(0, 0, 80, 25));
    let buf = &mut buf;

    let area = Rect::new(1, 1, 40, 20);

    let et = SystemTime::now();
    for _ in 0..100_000 {
        black_box({
            let mut b = BlockBorder::new();
            b.set_symbol(
                area,
                Position::new(area.x + 1, area.y),
                BorderSymbol::SideInward(Side::Left, BorderType::Plain),
            );
            buf.set_style(area, Style::new());
            b.render(area, buf);
        });
    }
    eprintln!("prefab {:?}", et.elapsed().unwrap().div_f64(100_000.));
}

#[test]
fn test_3() {
    let mut buf = Buffer::empty(Rect::new(0, 0, 80, 25));
    let buf = &mut buf;

    let area = Rect::new(1, 1, 40, 20);

    let et = SystemTime::now();
    for _ in 0..100_000 {
        black_box({
            let b = Block::bordered();
            b.render(area, buf);
        });
    }
    eprintln!("block {:?}", et.elapsed().unwrap().div_f64(100_000.));
}
