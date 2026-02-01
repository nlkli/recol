//! Theme and color scheme representation.
//!
//! This module defines the binary format and data structures used to represent
//! color themes and color schemes.
//!
//! Themes are serialized into a compact byte layout and can be embedded into
//! the binary or loaded at runtime. The layout is optimized for fast parsing
//! and fixed-size color data.
//!
//! Color schemes are derived from a base set of colors and expanded into
//! background, foreground, selection, cursor, ANSI, bright, and dim variants.

use crate::color;
use crate::color::{Color, print_palette};
use crate::utils::as_array_ref;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

/// Number of base colors used to build a color scheme.
pub const COLOR_SCHEME_NC: usize = 1 + 1 + 2 + 2 + ANSI_NC * 2;
/// Size of a color scheme in bytes.
pub const COLOR_SCHEME_SIZE: usize = COLOR_SCHEME_NC * 3;
/// Number of ANSI colors.
pub const ANSI_NC: usize = 8;
/// Size of ANSI colors in bytes.
pub const ANSI_SIZE: usize = ANSI_NC * 3;

const DEFAULT_BG: &str = "#000000";
const DEFAULT_FG: &str = "#ffffff";
const DEFAULT_SEL: &str = "#808080";

/// Binary layout:
/// `[NAME_LEN u8] [NAME string] [IS_LIGHT u8] [COLOR_SCHEME bytes]`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub is_light: bool,
    pub colors: ColorScheme,
}

impl Theme {
    pub fn new(name: String, colors: ColorScheme) -> Self {
        let is_light = color!(&colors.background[1]).to_hsl().2 > 50.;
        Self {
            name,
            is_light,
            colors,
        }
    }

    pub fn from_bytes(b: &[u8]) -> io::Result<Self> {
        if b.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "too short"));
        }
        let name_size = b[0] as usize;
        let required = 1 + name_size + 1 + COLOR_SCHEME_SIZE;
        if b.len() < required {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid length"));
        }
        let name = String::from_utf8(b[1..1 + name_size].to_vec())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid utf8"))?;
        let is_light = b[1 + name_size] != 0;
        let colors = ColorScheme::from_bytes(as_array_ref(&b[2 + name_size..required]));

        Ok(Self {
            name,
            is_light,
            colors,
        })
    }

    /// Returns the total byte size of this theme.
    pub fn size(&self) -> usize {
        2 + self.name.len() + COLOR_SCHEME_SIZE
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.size());
        debug_assert!(self.name.len() <= u8::max as usize);
        buf.push(self.name.len() as u8);
        buf.extend_from_slice(self.name.as_bytes());
        buf.push(self.is_light as u8);
        buf.extend_from_slice(&self.colors.to_bytes());
        buf
    }

    pub fn write_bytes<W: Write>(&self, mut w: W) -> io::Result<usize> {
        let buf = self.to_bytes();
        w.write_all(&buf)?;
        Ok(buf.len())
    }

    pub fn prepare(
        &mut self,
        dim_shade_f: Option<f32>,
        comment_blend_f: Option<f32>,
        code_selection_blend_f: Option<f32>,
    ) -> &Self {
        self.colors
            .prepare(dim_shade_f, comment_blend_f, code_selection_blend_f);
        self
    }

    pub fn print_palette(&self) {
        let mut p = Vec::with_capacity(ANSI_NC + 2);
        p.push(color!(&self.colors.background[1]));
        p.push(color!(&self.colors.foreground[1]));
        p.extend_from_slice(&self.colors.base.to_colors_array());
        print_palette(&p);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub background: [String; 5],
    pub foreground: [String; 4],
    pub selection: SelectionColors,
    pub cursor: CursorColors,
    pub base: AnsiColors,
    pub bright: AnsiColors,

    dim: Option<AnsiColors>,
    diff: Option<DiffColors>,
    code_selection: Option<[String; 2]>,
    comment: Option<String>,
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
    pub fn from_color_slice(c: &[Color; COLOR_SCHEME_NC]) -> Self {
        let bg = c[0];
        let fg = c[1];
        let sel = c[2];
        let sel_text = c[3];
        let cur = c[4];
        let cur_text = c[5];
        let base = AnsiColors::from_color_slice(as_array_ref(&c[6..6 + ANSI_NC]));
        let bright = AnsiColors::from_color_slice(as_array_ref(&c[6 + ANSI_NC..]));

        //                    bg0   bg2  bg3  bg4
        const BGS: [f32; 4] = [-4.3, 6., 12., 23.];
        const FGS: [f32; 3] = [6., -23., -44.];
        const GAP: f32 = 2.;

        let bg_lum = bg.to_hsl().2;
        let is_light = bg_lum > 50.;

        let m = if is_light { -1. } else { 1. };
        let z = if is_light { 0. } else { 100. };

        let bg0 = if (bg_lum + BGS[0] * m - z) * (-m) - GAP < 100. {
            bg.brighten(BGS[0] * m)
        } else {
            bg.brighten(-BGS[0] * m)
        };
        let bg2 = bg.brighten(BGS[1] * m);
        let bg3 = bg.brighten(BGS[2] * m);
        let bg4 = bg.brighten(BGS[3] * m);
        let fg0 = if (fg.to_hsl().2 + FGS[0] * m - z) * (-m) - GAP > 0. {
            fg.brighten(FGS[0] * m)
        } else {
            fg.brighten(-FGS[0] * m)
        };
        let fg2 = fg.brighten(FGS[1] * m);
        let fg3 = fg.brighten(FGS[2] * m);

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
                bg: sel.to_css(),
                fg: sel_text.to_css(),
            },
            cursor: CursorColors {
                bg: cur.to_css(),
                fg: cur_text.to_css(),
            },
            base,
            bright,
            dim: None,
            comment: None,
            code_selection: None,
            diff: None,
        }
    }

    pub fn from_bytes(b: &[u8; COLOR_SCHEME_SIZE]) -> Self {
        let colors = b
            .chunks_exact(3)
            .map(|s| Color::from_bytes(as_array_ref(s)))
            .collect::<Vec<_>>();
        Self::from_color_slice(as_array_ref(&colors))
    }

    pub fn to_colors_array(&self) -> [Color; COLOR_SCHEME_NC] {
        let mut colors = Vec::with_capacity(COLOR_SCHEME_NC);

        colors.push(color!(&self.background[1]));
        colors.push(color!(&self.foreground[1]));
        colors.push(color!(&self.selection.bg));
        colors.push(color!(&self.selection.fg));
        colors.push(color!(&self.cursor.bg));
        colors.push(color!(&self.cursor.fg));
        colors.extend_from_slice(&self.base.to_colors_array());
        colors.extend_from_slice(&self.bright.to_colors_array());

        *as_array_ref(&colors)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(COLOR_SCHEME_SIZE);
        for c in self.to_colors_array() {
            buf.extend_from_slice(&c.to_bytes());
        }
        buf
    }

    /// If `shade_f` is `None`, a default shade factor (`-0.15`) is used.
    /// The result is computed once and stored for reuse.
    pub fn dim(&mut self, shade_f: Option<f32>) -> &AnsiColors {
        self.dim.get_or_insert({
            const SHADE_F: f32 = -0.15;
            let shade_f = shade_f.unwrap_or(SHADE_F);
            self.base.for_each(|c| c.shade(shade_f))
        })
    }

    /// Blends the foreground and background colors using `blend_f`.
    /// If `blend_f` is `None`, a default blend factor (`0.4`) is used.
    pub fn comment(&mut self, blend_f: Option<f32>) -> &str {
        self.comment.get_or_insert({
            const BLEND_F: f32 = 0.4;
            let blend_f = blend_f.unwrap_or(BLEND_F);
            color!(&self.foreground[1])
                .blend(&color!(&self.background[1]), blend_f)
                .to_css()
        })
    }

    pub fn diff(&mut self) -> &DiffColors {
        self.diff.get_or_insert({
            let bg = color!(&self.background[1]);
            DiffColors {
                add: color!(&self.base.green).blend(&bg, 0.2).to_css(),
                delete: color!(&self.base.red).blend(&bg, 0.2).to_css(),
                change: color!(&self.base.blue).blend(&bg, 0.2).to_css(),
                text: color!(&self.base.magenta).blend(&bg, 0.3).to_css(),
            }
        })
    }

    // for nvim
    pub fn code_selection(&mut self, blend_f: Option<f32>) -> &[String; 2] {
        self.code_selection.get_or_insert({
            const BLEND_F: f32 = 0.15;
            let blend_f = blend_f.unwrap_or(BLEND_F);
            let bg = color!(&self.background[1]);
            [
                bg.blend(&color!(&self.foreground[1]), blend_f).to_css(),
                bg.blend(&color!(&self.cursor.bg), blend_f * 1.6).to_css(),
            ]
        })
    }

    pub fn prepare(
        &mut self,
        dim_shade_f: Option<f32>,
        comment_blend_f: Option<f32>,
        code_selection_blend_f: Option<f32>,
    ) -> &Self {
        self.dim(dim_shade_f);
        self.comment(comment_blend_f);
        self.diff();
        self.code_selection(code_selection_blend_f);
        self
    }

    // pub fn print_palette(&self) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionColors {
    pub bg: String,
    pub fg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorColors {
    pub bg: String,
    pub fg: String,
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
    pub fn from_color_slice(c: &[Color; ANSI_NC]) -> Self {
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

    pub fn from_bytes(b: &[u8; ANSI_SIZE]) -> Self {
        let colors = b
            .chunks_exact(3)
            .map(|s| Color::from_bytes(as_array_ref(s)))
            .collect::<Vec<_>>();
        Self::from_color_slice(as_array_ref(&colors))
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
        let mut buf = Vec::with_capacity(ANSI_SIZE);
        for c in self.to_colors_array() {
            buf.extend_from_slice(&c.to_bytes());
        }
        buf
    }

    pub fn for_each<F>(&self, f: F) -> Self
    where
        F: Fn(&Color) -> Color,
    {
        let mut colors = self.to_colors_array();
        colors.iter_mut().for_each(|c| *c = f(c));
        Self::from_color_slice(&colors)
    }

    // pub fn print_palette(&self) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffColors {
    pub add: String,
    pub delete: String,
    pub change: String,
    pub text: String,
}
