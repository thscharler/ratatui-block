use ratatui::style::{Color, Style};

#[derive(Debug, Default, Clone)]
pub struct Scheme {
    pub white: [Color; 4],
    pub black: [Color; 4],
    pub gray: [Color; 4],

    pub red: [Color; 4],
    pub orange: [Color; 4],
    pub yellow: [Color; 4],
    pub limegreen: [Color; 4],
    pub green: [Color; 4],
    pub bluegreen: [Color; 4],
    pub cyan: [Color; 4],
    pub blue: [Color; 4],
    pub deepblue: [Color; 4],
    pub purple: [Color; 4],
    pub magenta: [Color; 4],
    pub redpink: [Color; 4],

    pub primary: [Color; 4],
    pub secondary: [Color; 4],
}

impl Scheme {
    /// Create a style from the given white shade.
    /// n is `0..=3`
    pub fn white(&self, n: usize) -> Style {
        self.style(self.white[n])
    }

    /// Create a style from the given black shade.
    /// n is `0..=3`
    pub fn black(&self, n: usize) -> Style {
        self.style(self.black[n])
    }

    /// Create a style from the given gray shade.
    /// n is `0..=3`
    pub fn gray(&self, n: usize) -> Style {
        self.style(self.gray[n])
    }

    /// Create a style from the given red shade.
    /// n is `0..=3`
    pub fn red(&self, n: usize) -> Style {
        self.style(self.red[n])
    }

    /// Create a style from the given orange shade.
    /// n is `0..=3`
    pub fn orange(&self, n: usize) -> Style {
        self.style(self.orange[n])
    }

    /// Create a style from the given yellow shade.
    /// n is `0..=3`
    pub fn yellow(&self, n: usize) -> Style {
        self.style(self.yellow[n])
    }

    /// Create a style from the given limegreen shade.
    /// n is `0..=3`
    pub fn limegreen(&self, n: usize) -> Style {
        self.style(self.limegreen[n])
    }

    /// Create a style from the given green shade.
    /// n is `0..=3`
    pub fn green(&self, n: usize) -> Style {
        self.style(self.green[n])
    }

    /// Create a style from the given bluegreen shade.
    /// n is `0..=3`
    pub fn bluegreen(&self, n: usize) -> Style {
        self.style(self.bluegreen[n])
    }

    /// Create a style from the given cyan shade.
    /// n is `0..=3`
    pub fn cyan(&self, n: usize) -> Style {
        self.style(self.cyan[n])
    }

    /// Create a style from the given blue shade.
    /// n is `0..=3`
    pub fn blue(&self, n: usize) -> Style {
        self.style(self.blue[n])
    }

    /// Create a style from the given deepblue shade.
    /// n is `0..=3`
    pub fn deepblue(&self, n: usize) -> Style {
        self.style(self.deepblue[n])
    }

    /// Create a style from the given purple shade.
    /// n is `0..=3`
    pub fn purple(&self, n: usize) -> Style {
        self.style(self.purple[n])
    }

    /// Create a style from the given magenta shade.
    /// n is `0..=3`
    pub fn magenta(&self, n: usize) -> Style {
        self.style(self.magenta[n])
    }

    /// Create a style from the given redpink shade.
    /// n is `0..=3`
    pub fn redpink(&self, n: usize) -> Style {
        self.style(self.redpink[n])
    }

    /// Create a style from the given primary shade.
    /// n is `0..=3`
    pub fn primary(&self, n: usize) -> Style {
        self.style(self.primary[n])
    }

    /// Create a style from the given secondary shade.
    /// n is `0..=3`
    pub fn secondary(&self, n: usize) -> Style {
        self.style(self.secondary[n])
    }

    /// Focus style
    pub fn focus(&self) -> Style {
        self.style(self.primary[2])
    }

    /// Selection style
    pub fn select(&self) -> Style {
        self.style(self.secondary[1])
    }

    pub fn secondary_text(&self) -> Style {
        Style::new().fg(self.secondary[1])
    }

    /// Text field style.
    pub fn text_input(&self) -> Style {
        self.style(self.gray[3])
    }

    /// Focused text field style.
    pub fn text_focus(&self) -> Style {
        self.style(self.primary[0])
    }

    /// Text selection style.
    pub fn text_select(&self) -> Style {
        self.style(self.secondary[0])
    }

    pub fn table_base(&self) -> Style {
        Style::default().fg(self.white[1]).bg(self.black[0])
    }

    pub fn table_header(&self) -> Style {
        Style::default().fg(self.white[1]).bg(self.blue[2])
    }

    pub fn table_footer(&self) -> Style {
        Style::default().fg(self.white[1]).bg(self.blue[2])
    }

    /// Container base
    pub fn container(&self) -> Style {
        Style::default().fg(self.gray[0]).bg(self.black[1])
    }

    /// Container arrows
    pub fn container_arrow(&self) -> Style {
        Style::default().fg(self.secondary[0]).bg(self.black[1])
    }

    /// Data display style. Used for lists, tables, ...
    pub fn data_base(&self) -> Style {
        Style::default().fg(self.white[0]).bg(self.black[1])
    }

    /// Background for dialogs.
    pub fn dialog_base(&self) -> Style {
        Style::default().fg(self.white[2]).bg(self.gray[1])
    }

    /// Style for the status line.
    pub fn status_base(&self) -> Style {
        Style::default().fg(self.white[0]).bg(self.black[2])
    }

    /// Base style for lists.
    pub fn list_base(&self) -> Style {
        self.data_base()
    }

    /// Base style for buttons.
    pub fn button_base(&self) -> Style {
        self.style(self.gray[2])
    }

    /// Armed style for buttons.
    pub fn button_armed(&self) -> Style {
        self.style(self.secondary[0])
    }

    pub fn block(&self) -> Style {
        Style::default().fg(self.gray[1]).bg(self.black[1])
    }

    pub fn block_title(&self) -> Style {
        Style::default().fg(self.secondary[1])
    }

    /// Complete StatusLineStyle for a StatusLine with 3 indicator fields.
    /// This is what I need for the
    /// [minimal](https://github.com/thscharler/rat-salsa/blob/master/examples/minimal.rs)
    /// example, which shows timings for Render/Event/Action.
    pub fn statusline_style(&self) -> Vec<Style> {
        vec![
            self.status_base(),
            Style::default()
                .fg(self.text_color(self.white[0]))
                .bg(self.blue[3]),
            Style::default()
                .fg(self.text_color(self.white[0]))
                .bg(self.blue[2]),
            Style::default()
                .fg(self.text_color(self.white[0]))
                .bg(self.blue[1]),
        ]
    }

    /// Calculate a style based on the bg color.
    pub fn style(&self, color: Color) -> Style {
        Style::new().bg(color).fg(self.text_color(color))
    }

    /// Linear interpolation between the two colors.
    pub const fn linear4(c0: u32, c1: u32) -> [Color; 4] {
        // 1/3
        const fn i1(a: u8, b: u8) -> u8 {
            if a < b {
                a + (b - a) / 3
            } else {
                a - (a - b) / 3
            }
        }
        // 2/3
        const fn i2(a: u8, b: u8) -> u8 {
            if a < b {
                b - (b - a) / 3
            } else {
                b + (a - b) / 3
            }
        }

        let r0 = (c0 >> 16) as u8;
        let g0 = (c0 >> 8) as u8;
        let b0 = c0 as u8;

        let r3 = (c1 >> 16) as u8;
        let g3 = (c1 >> 8) as u8;
        let b3 = c1 as u8;

        let r1 = i1(r0, r3);
        let g1 = i1(g0, g3);
        let b1 = i1(b0, b3);

        let r2 = i2(r0, r3);
        let g2 = i2(g0, g3);
        let b2 = i2(b0, b3);

        [
            Color::Rgb(r0, g0, b0),
            Color::Rgb(r1, g1, b1),
            Color::Rgb(r2, g2, b2),
            Color::Rgb(r3, g3, b3),
        ]
    }

    /// This gives back `white[3]` or `black[0]` for text foreground
    /// providing good contrast to the given background.
    ///
    /// This converts RGB to grayscale and takes the grayscale value
    /// of VGA cyan as threshold, which is about 105 out of 255.
    /// This point is a bit arbitrary, just based on what I
    /// perceive as acceptable. But it produces a good reading
    /// contrast in my experience.
    ///
    /// For the named colors it takes the VGA equivalent as a base.
    /// For indexed colors it splits the range in half as an estimate.
    pub fn text_color(&self, color: Color) -> Color {
        match color {
            Color::Reset => self.white[3],
            Color::Black => self.white[3],        //0
            Color::Red => self.white[3],          //1
            Color::Green => self.white[3],        //2
            Color::Yellow => self.white[3],       //3
            Color::Blue => self.white[3],         //4
            Color::Magenta => self.white[3],      //5
            Color::Cyan => self.white[3],         //6
            Color::Gray => self.black[0],         //7
            Color::DarkGray => self.white[3],     //8
            Color::LightRed => self.black[0],     //9
            Color::LightGreen => self.black[0],   //10
            Color::LightYellow => self.black[0],  //11
            Color::LightBlue => self.white[3],    //12
            Color::LightMagenta => self.black[0], //13
            Color::LightCyan => self.black[0],    //14
            Color::White => self.black[0],        //15
            Color::Rgb(r, g, b) => {
                // The formula used in the GIMP is Y = 0.3R + 0.59G + 0.11B;
                let grey = r as f32 * 0.3f32 + g as f32 * 0.59f32 + b as f32 * 0.11f32;
                if grey >= 105f32 {
                    self.black[0]
                } else {
                    self.white[3]
                }
            }
            Color::Indexed(n) => match n {
                0..=6 => self.white[3],
                7 => self.black[0],
                8 => self.white[3],
                9..=11 => self.black[0],
                12 => self.white[3],
                13..=15 => self.black[0],
                v @ 16..=231 => {
                    if (v - 16) % 36 < 18 {
                        self.white[3]
                    } else {
                        self.black[0]
                    }
                }
                v @ 232..=255 => {
                    if (v - 232) % 24 < 12 {
                        self.white[3]
                    } else {
                        self.black[0]
                    }
                }
            },
        }
    }
}

/// Imperial scheme.
///
/// Uses purple and gold for primary/secondary.
/// Other colors are bright, strong and slightly smudged.
///
pub const THEME: Scheme = Scheme {
    primary: Scheme::linear4(0x300057, 0x8c00fd),
    secondary: Scheme::linear4(0x574b00, 0xffde00),

    white: Scheme::linear4(0xdedfe3, 0xf6f6f3),
    black: Scheme::linear4(0x0f1014, 0x2a2b37),
    gray: Scheme::linear4(0x3b3d4e, 0x6e7291),

    red: Scheme::linear4(0x480f0f, 0xd22d2d),
    orange: Scheme::linear4(0x482c0f, 0xd4812b),
    yellow: Scheme::linear4(0x756600, 0xffde00),
    limegreen: Scheme::linear4(0x2c4611, 0x80ce31),
    green: Scheme::linear4(0x186218, 0x32cd32),
    bluegreen: Scheme::linear4(0x206a52, 0x3bc49a),
    cyan: Scheme::linear4(0x0f2c48, 0x2bd4d4),
    blue: Scheme::linear4(0x162b41, 0x2b81d4),
    deepblue: Scheme::linear4(0x202083, 0x3232cd),
    purple: Scheme::linear4(0x4d008b, 0x8c00fd),
    magenta: Scheme::linear4(0x401640, 0xbd42bd),
    redpink: Scheme::linear4(0x47101d, 0xc33c5b),
};
