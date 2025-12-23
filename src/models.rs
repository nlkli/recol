use crate::color;
use crate::color::Color;
use crate::utils::as_array_ref;
use serde::{Deserialize, Serialize};

pub const ANSI_NC: usize = 8;
pub const COLOR_SCHEME_NC: usize = 1 + 1 + 2 + 2 + ANSI_NC * 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ColorScheme {
    pub background: [String; 5],
    pub foreground: [String; 4],
    pub selection: SelectionColors,
    pub cursor: CursorColors,
    pub base: AnsiColors,
    pub bright: AnsiColors,
    pub dim: AnsiColors,
}

impl ColorScheme {
    /// Color order:
    /// 1. bg
    /// 2. fg
    /// 3. sel
    /// 4. sel_text
    /// 5. cur
    /// 6. cur_text
    /// 7. base (8 ansi colors)
    /// 8. bright (8 ansi colors)
    pub fn from_colors_slice(c: &[Color; COLOR_SCHEME_NC]) -> Self {
        let bg = c[0];
        let fg = c[1];
        let sel = c[2];
        let sel_text = c[3];
        let cur = c[4];
        let cur_text = c[5];
        let base = AnsiColors::from_colors_slice(as_array_ref(&c[6..6 + ANSI_NC]));
        let bright = AnsiColors::from_colors_slice(as_array_ref(&c[6 + ANSI_NC..]));

        const SHADE_F: f32 = 0.15;
        let dim = base.for_each(|c| Color::shade(c, -SHADE_F));

        const BGS: [f32; 4] = [-4., 6., 12., 23.];
        const FGS: [f32; 3] = [6., -23., -45.];
        const SEL_S: f32 = 16.;

        let is_light = bg.luminance() > 50.;

        let m = if is_light { -1. } else { 1. };
        let z = if is_light { 0. } else { 100. };

        let bg0 = if (bg.to_hsl().2 + BGS[0] * m - z) * (-m) - 1. < 100. {
            bg.brighten(BGS[0] * m)
        } else {
            bg.brighten(-BGS[0] * m)
        };
        let bg2 = bg.brighten(BGS[1] * m);
        let bg3 = bg.brighten(BGS[2] * m);
        let bg4 = bg.brighten(BGS[3] * m);
        let fg0 = if (fg.to_hsl().2 + FGS[0] * m - z) * (-m) - 1. > 0. {
            fg.brighten(FGS[0] * m)
        } else {
            fg.brighten(-FGS[0] * m)
        };
        let fg2 = fg.brighten(FGS[1] * m);
        let fg3 = fg.brighten(FGS[2] * m);

        let sel0 = sel.brighten(SEL_S * m);

        Self {
            background: [
                bg0.to_css(),
                bg.to_css(),
                bg2.to_css(),
                bg3.to_css(),
                bg4.to_css(),
            ],
            foreground: [fg0.to_css(), fg.to_css(), fg2.to_css(), fg3.to_css()],
            selection: SelectionColors {
                sel0: sel0.to_css(),
                sel: sel.to_css(),
                text: sel_text.to_css(),
            },
            cursor: CursorColors {
                cur: cur.to_css(),
                text: cur_text.to_css(),
            },
            base,
            bright,
            dim,
        }
    }

    pub fn from_bytes(b: &[u8; COLOR_SCHEME_NC * 3]) -> Self {
        let colors = b
            .chunks_exact(3)
            .map(|s| Color::from_bytes(as_array_ref(s)))
            .collect::<Vec<_>>();
        Self::from_colors_slice(as_array_ref(&colors))
    }


    pub fn to_colors_array(&self) -> [Color; COLOR_SCHEME_NC] {
        let mut colors = Vec::with_capacity(COLOR_SCHEME_NC);

        colors.push(color!(&self.background[1]));
        colors.push(color!(&self.foreground[1]));
        colors.push(color!(&self.selection.sel));
        colors.push(color!(&self.selection.text));
        colors.push(color!(&self.cursor.cur));
        colors.push(color!(&self.cursor.text));
        colors.extend_from_slice(&self.base.to_colors_array());
        colors.extend_from_slice(&self.bright.to_colors_array());

        *as_array_ref(&colors)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(COLOR_SCHEME_NC * 3);
        for c in self.to_colors_array() {
            buf.extend_from_slice(&c.to_bytes());
        }
        buf
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SelectionColors {
    pub sel0: String,
    pub sel: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CursorColors {
    pub cur: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnsiColors {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,

    pub orange: String,
    pub pink: String,
}

impl Default for AnsiColors {
    fn default() -> Self {
        Self {
            black: "#000000".into(),
            red: "#c91b00".into(),
            green: "#00c200".into(),
            yellow: "#c7c400".into(),
            blue: "#2225c4".into(),
            magenta: "#ca30c7".into(),
            cyan: "#00c5c7".into(),
            white: "#ffffff".into(),
            orange: "#c7c400".into(),
            pink: "#c91b00".into(),
        }
    }
}

impl AnsiColors {
    /// Color order:
    /// 1. black
    /// 2. red
    /// 3. green
    /// 4. yellow
    /// 5. blue
    /// 6. magenta
    /// 7. cyan
    /// 8. white
    pub fn from_colors_slice(c: &[Color; ANSI_NC]) -> Self {
        Self {
            black: c[0].to_css(),
            red: c[1].to_css(),
            green: c[2].to_css(),
            yellow: c[3].to_css(),
            blue: c[4].to_css(),
            magenta: c[5].to_css(),
            cyan: c[6].to_css(),
            white: c[7].to_css(),

            orange: c[1].blend(&c[3], 0.5).to_css(),
            pink: c[1].blend(&c[7], 0.5).to_css(),
        }
    }

    pub fn from_bytes(b: &[u8; ANSI_NC * 3]) -> Self {
        let colors = b
            .chunks_exact(3)
            .map(|s| Color::from_bytes(as_array_ref(s)))
            .collect::<Vec<_>>();
        Self::from_colors_slice(as_array_ref(&colors))
    }

    pub fn to_colors_array(&self) -> [Color; ANSI_NC] {
        [
            color!(&self.black),
            color!(&self.red),
            color!(&self.green),
            color!(&self.yellow),
            color!(&self.blue),
            color!(&self.magenta),
            color!(&self.cyan),
            color!(&self.white),
        ]
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(ANSI_NC * 3);
        for c in self.to_colors_array() {
            buf.extend_from_slice(&c.to_bytes());
        }
        buf
    }

    pub fn for_each(&self, f: fn(&Color) -> Color) -> Self {
        let mut colors = self.to_colors_array();
        colors.iter_mut().for_each(|c| *c = f(c));
        Self::from_colors_slice(&colors)
    }
}
