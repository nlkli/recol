use crate::{COLOR_SIZE, Color, CssColor, Error, Result};
use serde::{Deserialize, Serialize};

/// Number of base colors used to build a color scheme.
pub const COLOR_SCHEME_NC: usize = 1 + 1 + 2 + 2 + 8 * 2;

/// Size of a color scheme in bytes.
pub const COLOR_SCHEME_SIZE: usize = COLOR_SCHEME_NC * COLOR_SIZE;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub is_light: bool,
    pub colors: ColorScheme,
}

impl TryFrom<&[u8]> for Theme {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        if bytes.len() == 0 || bytes.len() < 1 + (bytes[0] as usize) + 1 + COLOR_SCHEME_SIZE {
            return Err(Error::InvalidLength {
                src: "Theme::try_from::<&[u8]>".into(),
                expected: 3,
                got: bytes.len(),
            });
        }

        let name_size = bytes[0] as usize;
        let name = String::from_utf8(bytes[1..1 + name_size].to_vec())?;
        let is_light = bytes[1 + name_size] != 0;
        let colors = ColorScheme::try_from(
            &bytes[1 + name_size + 1..1 + name_size + 1 + COLOR_SCHEME_SIZE],
        )?;

        Ok(Self::new(name, is_light, colors))
    }
}

impl Theme {
    pub fn new(name: impl Into<String>, is_light: bool, colors: ColorScheme) -> Self {
        Self {
            name: name.into(),
            is_light,
            colors,
        }
    }

    pub fn try_from_bytes(b: &[u8]) -> Result<Self> {
        Self::try_from(b)
    }

    pub fn size(&self) -> usize {
        1 + self.name.len() + 1 + COLOR_SCHEME_SIZE
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.size());
        buf.push(self.name.len() as u8);
        buf.extend_from_slice(self.name.as_bytes());
        buf.push(if self.is_light { 1 } else { 0 });
        buf.extend_from_slice(&self.colors.bytes());
        buf
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub bg: CssColor,
    pub fg: CssColor,

    pub selection: SelectionColors,

    pub cursor: CursorColors,

    pub base: AnsiColors,
    pub bright: AnsiColors,
}

impl TryFrom<&[u8]> for ColorScheme {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != COLOR_SCHEME_SIZE {
            return Err(Error::InvalidLength {
                src: "ColorScheme::try_from::<&[u8]>".into(),
                expected: 3,
                got: bytes.len(),
            });
        }
        let mut chunks = bytes.chunks_exact(COLOR_SIZE);
        Ok(Self::from_color_slice(&[
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
            Color::try_from(chunks.next().unwrap())?,
        ]))
    }
}

impl ColorScheme {
    pub fn from_color_slice(c: &[Color; COLOR_SCHEME_NC]) -> Self {
        Self {
            bg: c[0].css(),
            fg: c[1].css(),
            selection: SelectionColors {
                bg: c[2].css(),
                fg: c[3].css(),
            },
            cursor: CursorColors {
                bg: c[4].css(),
                fg: c[5].css(),
            },
            base: AnsiColors {
                black: c[6].css(),
                red: c[7].css(),
                green: c[8].css(),
                yellow: c[9].css(),
                blue: c[10].css(),
                magenta: c[11].css(),
                cyan: c[12].css(),
                white: c[13].css(),

                orange: c[7].blend(&c[9], 0.5).css(),
                pink: c[7].blend(&c[13], 0.5).css(),
            },
            bright: AnsiColors {
                black: c[14].css(),
                red: c[15].css(),
                green: c[16].css(),
                yellow: c[17].css(),
                blue: c[18].css(),
                magenta: c[19].css(),
                cyan: c[20].css(),
                white: c[21].css(),

                orange: c[15].blend(&c[17], 0.5).css(),
                pink: c[15].blend(&c[21], 0.5).css(),
            },
        }
    }

    pub fn try_from_bytes(b: &[u8]) -> Result<Self> {
        Self::try_from(b)
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(ANSI_SIZE);
        for c in self.as_colors_array() {
            buf.extend_from_slice(&c.bytes());
        }
        buf
    }

    pub fn as_colors_array(&self) -> [Color; COLOR_SCHEME_NC] {
        [
            self.bg.color(),
            self.fg.color(),
            self.selection.bg.color(),
            self.selection.fg.color(),
            self.cursor.bg.color(),
            self.cursor.fg.color(),
            self.base.black.color(),
            self.base.red.color(),
            self.base.green.color(),
            self.base.yellow.color(),
            self.base.blue.color(),
            self.base.magenta.color(),
            self.base.cyan.color(),
            self.base.white.color(),
            self.bright.black.color(),
            self.bright.red.color(),
            self.bright.green.color(),
            self.bright.yellow.color(),
            self.bright.blue.color(),
            self.bright.magenta.color(),
            self.bright.cyan.color(),
            self.bright.white.color(),
        ]
    }

    pub fn into_advanced(self, param: AdvancedColorSchemeParam) -> AdvancedColorScheme {
        let bg_color = self.bg.color();
        let fg_color = self.fg.color();

        const GAP: f32 = 2.;

        let bg_lum = bg_color.hsl().2;
        let is_light = bg_lum > 50.;
        let m = if is_light { -1. } else { 1. };
        let z = if is_light { 0. } else { 100. };

        let bg = [
            if (bg_lum + param.bg0_brighten * m - z) * (-m) - GAP < 100. {
                bg_color.brighten(param.bg0_brighten * m).css()
            } else {
                bg_color.brighten(-param.bg0_brighten * m).css()
            },
            self.bg,
            bg_color.brighten(param.bg2_brighten).css(),
            bg_color.brighten(param.bg3_brighten).css(),
            bg_color.brighten(param.bg4_brighten).css(),
        ];
        let fg = [
            if (fg_color.hsl().2 + param.fg0_brighten * m - z) * (-m) - GAP > 0. {
                fg_color.brighten(param.fg0_brighten * m).css()
            } else {
                fg_color.brighten(-param.fg0_brighten * m).css()
            },
            self.fg,
            fg_color.brighten(param.fg2_brighten).css(),
            fg_color.brighten(param.fg3_brighten).css(),
        ];

        let alt_selection = [
            bg_color.blend(&fg_color, param.code_selection_blend).css(),
            bg_color
                .blend(&self.cursor.bg.color(), param.code_selection_blend)
                .css(),
        ];

        let dim = AnsiColors {
            black: self.base.black.color().shade(param.dim_shade).css(),
            red: self.base.red.color().shade(param.dim_shade).css(),
            green: self.base.green.color().shade(param.dim_shade).css(),
            yellow: self.base.yellow.color().shade(param.dim_shade).css(),
            blue: self.base.blue.color().shade(param.dim_shade).css(),
            magenta: self.base.magenta.color().shade(param.dim_shade).css(),
            cyan: self.base.cyan.color().shade(param.dim_shade).css(),
            white: self.base.white.color().shade(param.dim_shade).css(),
            orange: self.base.orange.color().shade(param.dim_shade).css(),
            pink: self.base.pink.color().shade(param.dim_shade).css(),
        };

        let diff = DiffColors {
            add: self
                .base
                .green
                .color()
                .blend(&bg_color, param.diff_add_blend)
                .css(),
            delete: self
                .base
                .red
                .color()
                .blend(&bg_color, param.diff_delete_blend)
                .css(),
            change: self
                .base
                .blue
                .color()
                .blend(&bg_color, param.diff_change_blend)
                .css(),
            text: self
                .base
                .magenta
                .color()
                .blend(&bg_color, param.diff_text_blend)
                .css(),
        };

        let comment = fg_color.blend(&bg_color, param.comment_blend).css();

        AdvancedColorScheme {
            bg,
            fg,
            selection: self.selection,
            alt_selection,
            cursor: self.cursor,
            base: self.base,
            bright: self.bright,
            dim,
            diff,
            comment,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdvancedColorSchemeParam {
    pub bg0_brighten: f32,
    pub bg2_brighten: f32,
    pub bg3_brighten: f32,
    pub bg4_brighten: f32,
    pub fg0_brighten: f32,
    pub fg2_brighten: f32,
    pub fg3_brighten: f32,
    pub code_selection_blend: f32,
    pub dim_shade: f32,
    pub diff_add_blend: f32,
    pub diff_delete_blend: f32,
    pub diff_change_blend: f32,
    pub diff_text_blend: f32,
    pub comment_blend: f32,
}

impl Default for AdvancedColorSchemeParam {
    fn default() -> Self {
        Self {
            bg0_brighten: -4.3,
            bg2_brighten: 6.,
            bg3_brighten: 12.,
            bg4_brighten: 23.,
            fg0_brighten: 6.,
            fg2_brighten: -23.,
            fg3_brighten: -44.,
            code_selection_blend: 0.15,
            dim_shade: 0.15,
            diff_add_blend: 0.33,
            diff_delete_blend: 0.33,
            diff_change_blend: 0.33,
            diff_text_blend: 0.4,
            comment_blend: 0.4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedColorScheme {
    pub bg: [CssColor; 5],
    pub fg: [CssColor; 4],

    pub selection: SelectionColors,
    pub alt_selection: [CssColor; 2],

    pub cursor: CursorColors,

    pub base: AnsiColors,
    pub bright: AnsiColors,
    pub dim: AnsiColors,

    pub diff: DiffColors,
    pub comment: CssColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionColors {
    pub bg: CssColor,
    pub fg: CssColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorColors {
    pub bg: CssColor,
    pub fg: CssColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnsiColors {
    pub black: CssColor,
    pub red: CssColor,
    pub green: CssColor,
    pub yellow: CssColor,
    pub blue: CssColor,
    pub magenta: CssColor,
    pub cyan: CssColor,
    pub white: CssColor,

    pub orange: CssColor,
    pub pink: CssColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffColors {
    pub add: CssColor,
    pub delete: CssColor,
    pub change: CssColor,
    pub text: CssColor,
}
