//! Color scheme types and their binary serialization.
//!
//! # Color order in `ColorScheme` binary representation
//!
//! A [`ColorScheme`] is serialized as exactly [`COLOR_SCHEME_NC`] colors,
//! each [`COLOR_SIZE`] bytes, in the following order:
//!
//! ```text
//! Index  Field                   Group
//! ─────────────────────────────────────────────────────
//!   0    background              terminal
//!   1    foreground              terminal
//! ─────────────────────────────────────────────────────
//!   2    selection background    selection
//!   3    selection foreground    selection
//! ─────────────────────────────────────────────────────
//!   4    cursor background       cursor
//!   5    cursor foreground       cursor
//! ─────────────────────────────────────────────────────
//!   6    black                   ANSI base (normal)
//!   7    red
//!   8    green
//!   9    yellow
//!  10    blue
//!  11    magenta
//!  12    cyan
//!  13    white
//! ─────────────────────────────────────────────────────
//!  14    black                   ANSI bright
//!  15    red
//!  16    green
//!  17    yellow
//!  18    blue
//!  19    magenta
//!  20    cyan
//!  21    white
//! ─────────────────────────────────────────────────────
//! ```
//!
//! `orange` and `pink` in [`AnsiColors`] are **derived** (blended at runtime)
//! and are not stored in the binary.

use crate::{COLOR_SIZE, Color, CssColor, Error, Result, ThemeAdjustment};
use serde::{Deserialize, Serialize};

/// Number of colors stored per [`ColorScheme`]: 2 terminal + 2 selection +
/// 2 cursor + 8 base ANSI + 8 bright ANSI.
pub const COLOR_SCHEME_NC: usize = 2 + 2 + 2 + 8 + 8;

/// Byte size of a serialized [`ColorScheme`].
pub const COLOR_SCHEME_SIZE: usize = COLOR_SCHEME_NC * COLOR_SIZE;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub is_light: bool,
    pub colors: ColorScheme,
}

impl Theme {
    pub fn new(name: impl Into<String>, is_light: bool, colors: ColorScheme) -> Self {
        Self {
            name: name.into(),
            is_light,
            colors,
        }
    }

    /// Byte length of the serialized form produced by [`bytes`](Self::bytes).
    pub fn size(&self) -> usize {
        1 + self.name.len() + 1 + COLOR_SCHEME_SIZE
    }

    /// Serialize to the binary layout described in [`colorschemes`](crate::colorschemes).
    pub fn bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.size());
        buf.push(self.name.len() as u8);
        buf.extend_from_slice(self.name.as_bytes());
        buf.push(u8::from(self.is_light));
        buf.extend_from_slice(&self.colors.bytes());
        buf
    }

    pub fn print_palette(&self) {
        crate::print_palette(&self.colors.as_colors_array()[0..14]);
    }
}

impl TryFrom<&[u8]> for Theme {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        // Minimum: name_len(1) + name(name_len) + is_light(1) + colors
        let min_len = |name_len: usize| 1 + name_len + 1 + COLOR_SCHEME_SIZE;

        if bytes.is_empty() || bytes.len() < min_len(bytes[0] as usize) {
            return Err(Error::InvalidLength {
                src: "Theme::try_from".into(),
                expected: min_len(0),
                got: bytes.len(),
            });
        }

        let name_len = bytes[0] as usize;
        let name = String::from_utf8(bytes[1..1 + name_len].to_vec())?;
        let is_light = bytes[1 + name_len] != 0;
        let colors = ColorScheme::try_from(&bytes[1 + name_len + 1..][..COLOR_SCHEME_SIZE])?;

        Ok(Self::new(name, is_light, colors))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ColorScheme {
    pub bg: CssColor,
    pub fg: CssColor,
    pub selection: SelectionColors,
    pub cursor: CursorColors,
    /// Normal ANSI palette (indices 0–7), plus derived `orange` and `pink`.
    pub base: AnsiColors,
    /// Bright ANSI palette (indices 8–15), plus derived `orange` and `pink`.
    pub bright: AnsiColors,
}

impl ColorScheme {
    /// Build a [`ColorScheme`] from the canonical color array.
    ///
    /// Index mapping matches the binary layout documented in this module.
    /// `orange` and `pink` are blended from adjacent palette entries and are
    /// not part of the stored array.
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
                // Derived: not stored in binary.
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
                // Derived: not stored in binary.
                orange: c[15].blend(&c[17], 0.5).css(),
                pink: c[15].blend(&c[21], 0.5).css(),
            },
        }
    }

    /// Return the 22 stored colors in binary order (derived colors excluded).
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

    /// Serialize to a flat byte buffer (`COLOR_SCHEME_SIZE` bytes).
    pub fn bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(COLOR_SCHEME_SIZE);
        for color in self.as_colors_array() {
            buf.extend_from_slice(&color.bytes());
        }
        buf
    }

    pub fn try_from_bytes(b: &[u8]) -> Result<Self> {
        Self::try_from(b)
    }

    /// Expand this scheme into an [`AdvancedColorScheme`] using the given
    /// brightness/blend parameters.
    pub fn into_advanced(self, param: Option<AdvancedColorSchemeParam>) -> AdvancedColorScheme {
        let param = param.unwrap_or_default();

        let bg_color = self.bg.color();
        let fg_color = self.fg.color();
        let bg_lum = bg_color.hsl().2;
        let is_light = bg_lum > 50.0;

        // Direction multiplier: +1 for dark themes (brighten = lighter),
        // -1 for light themes (brighten = darker).
        let m = if is_light { -1.0_f32 } else { 1.0_f32 };
        // Luminance boundary (0 for dark, 100 for light).
        let z = if is_light { 0.0_f32 } else { 100.0_f32 };
        const GAP: f32 = 2.0;

        // bg[0] is always slightly *outside* the main bg to create contrast;
        // if the naive direction would clip against the boundary, flip it.
        let bg0 = if (bg_lum + param.bg0_brighten * m - z) * (-m) - GAP < 100.0 {
            bg_color.brighten(param.bg0_brighten * m).css()
        } else {
            bg_color.brighten(-param.bg0_brighten * m).css()
        };
        let bg = [
            bg0,
            self.bg,
            bg_color.brighten(param.bg2_brighten).css(),
            bg_color.brighten(param.bg3_brighten).css(),
            bg_color.brighten(param.bg4_brighten).css(),
        ];

        // fg[0] applies the same boundary-aware flip logic.
        let fg0 = if (fg_color.hsl().2 + param.fg0_brighten * m - z) * (-m) - GAP > 0.0 {
            fg_color.brighten(param.fg0_brighten * m).css()
        } else {
            fg_color.brighten(-param.fg0_brighten * m).css()
        };
        let fg = [
            fg0,
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
            comment: fg_color.blend(&bg_color, param.comment_blend).css(),
        }
    }

    #[inline]
    pub fn apply_adjustment(&mut self, adjust: &ThemeAdjustment) {
        adjust.apply(self);
    }

    pub fn apply_adjustments(&mut self, adjusts: &[ThemeAdjustment]) {
        adjusts.iter().for_each(|a| self.apply_adjustment(a));
    }
}

impl TryFrom<&[u8]> for ColorScheme {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != COLOR_SCHEME_SIZE {
            return Err(Error::InvalidLength {
                src: "ColorScheme::try_from".into(),
                expected: COLOR_SCHEME_SIZE,
                got: bytes.len(),
            });
        }

        let mut colors = [Color::default(); COLOR_SCHEME_NC];
        for (i, chunk) in bytes.chunks_exact(COLOR_SIZE).enumerate() {
            colors[i] = Color::try_from(chunk)?;
        }
        Ok(Self::from_color_slice(&colors))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SelectionColors {
    pub bg: CssColor,
    pub fg: CssColor,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CursorColors {
    pub bg: CssColor,
    pub fg: CssColor,
}

/// Eight standard ANSI colors plus two derived blends (`orange`, `pink`).
///
/// `orange` = blend(red, yellow, 50 %)  
/// `pink`   = blend(red, white,  50 %)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnsiColors {
    pub black: CssColor,
    pub red: CssColor,
    pub green: CssColor,
    pub yellow: CssColor,
    pub blue: CssColor,
    pub magenta: CssColor,
    pub cyan: CssColor,
    pub white: CssColor,
    /// Derived: `blend(red, yellow, 50 %)`. Not stored in binary.
    pub orange: CssColor,
    /// Derived: `blend(red, white, 50 %)`. Not stored in binary.
    pub pink: CssColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffColors {
    pub add: CssColor,
    pub delete: CssColor,
    pub change: CssColor,
    pub text: CssColor,
}

/// Expanded color scheme with additional derived palette entries for editor use.
///
/// # Background gradient (`bg`)
///
/// ```text
/// bg[0]  — slightly outside main bg (contrast, e.g. sidebar/status bar)
/// bg[1]  — main background  (= ColorScheme::bg)
/// bg[2]  — bg + bg2_brighten
/// bg[3]  — bg + bg3_brighten
/// bg[4]  — bg + bg4_brighten
/// ```
///
/// # Foreground gradient (`fg`)
///
/// ```text
/// fg[0]  — slightly outside main fg (e.g. bold text)
/// fg[1]  — main foreground  (= ColorScheme::fg)
/// fg[2]  — fg + fg2_brighten
/// fg[3]  — fg + fg3_brighten
/// ```
///
/// # Alternative selection (`alt_selection`)
///
/// ```text
/// alt_selection[0]  — blend(bg, fg,     code_selection_blend)
/// alt_selection[1]  — blend(bg, cursor, code_selection_blend)
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedColorScheme {
    pub bg: [CssColor; 5],
    pub fg: [CssColor; 4],
    pub selection: SelectionColors,
    pub alt_selection: [CssColor; 2],
    pub cursor: CursorColors,
    pub base: AnsiColors,
    pub bright: AnsiColors,
    /// Base palette shaded by `dim_shade` (for dimmed/inactive text).
    pub dim: AnsiColors,
    pub diff: DiffColors,
    /// Blend of fg toward bg — typically used for comments.
    pub comment: CssColor,
}

/// Tuning knobs for [`ColorScheme::into_advanced`].
///
/// All `*_brighten` values are additive HSL lightness deltas (positive =
/// lighter, negative = darker). For light themes the sign is flipped
/// automatically so the semantic direction is always preserved.
///
/// All `*_blend` values are in `[0.0, 1.0]` where `0.0` = first color,
/// `1.0` = second color.
#[derive(Debug, Clone)]
pub struct AdvancedColorSchemeParam {
    /// bg[0] lightness offset (boundary-aware).
    pub bg0_brighten: f32,
    pub bg2_brighten: f32,
    pub bg3_brighten: f32,
    pub bg4_brighten: f32,
    /// fg[0] lightness offset (boundary-aware).
    pub fg0_brighten: f32,
    pub fg2_brighten: f32,
    pub fg3_brighten: f32,
    /// Blend factor for both `alt_selection` entries.
    pub code_selection_blend: f32,
    /// Shade factor applied to every base color to produce `dim`.
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
            bg0_brighten: -4.31,
            bg2_brighten: 6.0,
            bg3_brighten: 12.1,
            bg4_brighten: 23.2,
            fg0_brighten: 6.0,
            fg2_brighten: -23.2,
            fg3_brighten: -44.0,
            code_selection_blend: 0.155,
            dim_shade: 0.155,
            diff_add_blend: 0.35,
            diff_delete_blend: 0.35,
            diff_change_blend: 0.35,
            diff_text_blend: 0.4,
            comment_blend: 0.4,
        }
    }
}
