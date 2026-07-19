use crate::{Color, ColorScheme, CssColor};

/// Represents a group of theme colors that can be selected for operations.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ThemeColorGroup {
    /// All colors in the theme.
    #[default]
    All,
    /// UI colors (background, foreground, selection, cursor).
    UI,
    /// All background colors.
    Background,
    /// Base background color.
    BaseBackground,
    /// Selection background color.
    SelectionBackground,
    /// Cursor background color.
    CursorBackground,
    /// All foreground colors.
    Foreground,
    /// Base foreground color.
    BaseForeground,
    /// Selection foreground color.
    SelectionForeground,
    /// Cursor foreground color.
    CursorForeground,
    /// Selection colors (background and foreground).
    Selection,
    /// Cursor colors (background and foreground).
    Cursor,
    /// All ANSI palette colors.
    Palette,
    /// Black ANSI colors.
    Black,
    /// Red ANSI colors.
    Red,
    /// Green ANSI colors.
    Green,
    /// Yellow ANSI colors.
    Yellow,
    /// Blue ANSI colors.
    Blue,
    /// Magenta ANSI colors.
    Magenta,
    /// Cyan ANSI colors.
    Cyan,
    /// White ANSI colors.
    White,
    /// Orange ANSI colors.
    Orange,
    /// Pink ANSI colors.
    Pink,
    /// All text colors (foregrounds + palette).
    Text,
}

impl std::str::FromStr for ThemeColorGroup {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "u" | "ui" => Ok(Self::UI),
            "b" | "bg" => Ok(Self::Background),
            "f" | "fg" => Ok(Self::Foreground),
            "s" | "sel" => Ok(Self::Selection),
            "c" | "cur" => Ok(Self::Cursor),
            "p" | "pal" => Ok(Self::Palette),
            "bb" | "base-bg" => Ok(Self::BaseBackground),
            "bf" | "base-fg" => Ok(Self::BaseForeground),
            "sb" | "sel-bg" => Ok(Self::SelectionBackground),
            "sf" | "sel-fg" => Ok(Self::SelectionForeground),
            "cb" | "cur-bg" => Ok(Self::CursorBackground),
            "cf" | "cur-fg" => Ok(Self::CursorForeground),
            "t" | "text" => Ok(Self::Text),
            "black" => Ok(Self::Black),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "yellow" => Ok(Self::Yellow),
            "blue" => Ok(Self::Blue),
            "magenta" => Ok(Self::Magenta),
            "cyan" => Ok(Self::Cyan),
            "white" => Ok(Self::White),
            "orange" => Ok(Self::Orange),
            "pink" => Ok(Self::Pink),
            _ => Err(()),
        }
    }
}

impl ThemeColorGroup {
    pub fn select_colors<'a>(&self, cs: &'a mut ColorScheme) -> Vec<&'a mut CssColor> {
        match self {
            ThemeColorGroup::All => vec![
                &mut cs.bg,
                &mut cs.fg,
                &mut cs.selection.bg,
                &mut cs.selection.fg,
                &mut cs.cursor.bg,
                &mut cs.cursor.fg,
                &mut cs.base.black,
                &mut cs.base.red,
                &mut cs.base.green,
                &mut cs.base.yellow,
                &mut cs.base.blue,
                &mut cs.base.magenta,
                &mut cs.base.cyan,
                &mut cs.base.white,
                &mut cs.base.orange,
                &mut cs.base.pink,
                &mut cs.bright.black,
                &mut cs.bright.red,
                &mut cs.bright.green,
                &mut cs.bright.yellow,
                &mut cs.bright.blue,
                &mut cs.bright.magenta,
                &mut cs.bright.cyan,
                &mut cs.bright.white,
                &mut cs.bright.orange,
                &mut cs.bright.pink,
            ],
            ThemeColorGroup::UI => vec![
                &mut cs.bg,
                &mut cs.fg,
                &mut cs.selection.bg,
                &mut cs.selection.fg,
                &mut cs.cursor.bg,
                &mut cs.cursor.fg,
            ],
            ThemeColorGroup::Background => {
                vec![&mut cs.bg, &mut cs.selection.bg, &mut cs.cursor.bg]
            }
            ThemeColorGroup::Foreground => {
                vec![&mut cs.fg, &mut cs.selection.fg, &mut cs.cursor.fg]
            }
            ThemeColorGroup::BaseBackground => {
                vec![&mut cs.bg]
            }
            ThemeColorGroup::BaseForeground => {
                vec![&mut cs.fg]
            }
            ThemeColorGroup::Selection => vec![&mut cs.selection.bg, &mut cs.selection.fg],
            ThemeColorGroup::Cursor => vec![&mut cs.cursor.bg, &mut cs.cursor.fg],
            ThemeColorGroup::SelectionBackground => {
                vec![&mut cs.selection.bg]
            }
            ThemeColorGroup::SelectionForeground => {
                vec![&mut cs.selection.fg]
            }
            ThemeColorGroup::CursorBackground => {
                vec![&mut cs.cursor.bg]
            }
            ThemeColorGroup::CursorForeground => {
                vec![&mut cs.cursor.fg]
            }
            ThemeColorGroup::Palette => vec![
                &mut cs.base.black,
                &mut cs.base.red,
                &mut cs.base.green,
                &mut cs.base.yellow,
                &mut cs.base.blue,
                &mut cs.base.magenta,
                &mut cs.base.cyan,
                &mut cs.base.white,
                &mut cs.base.orange,
                &mut cs.base.pink,
                &mut cs.bright.black,
                &mut cs.bright.red,
                &mut cs.bright.green,
                &mut cs.bright.yellow,
                &mut cs.bright.blue,
                &mut cs.bright.magenta,
                &mut cs.bright.cyan,
                &mut cs.bright.white,
                &mut cs.bright.orange,
                &mut cs.bright.pink,
            ],
            ThemeColorGroup::Text => vec![
                &mut cs.fg,
                &mut cs.selection.fg,
                &mut cs.cursor.fg,
                &mut cs.base.black,
                &mut cs.base.red,
                &mut cs.base.green,
                &mut cs.base.yellow,
                &mut cs.base.blue,
                &mut cs.base.magenta,
                &mut cs.base.cyan,
                &mut cs.base.white,
                &mut cs.base.orange,
                &mut cs.base.pink,
                &mut cs.bright.black,
                &mut cs.bright.red,
                &mut cs.bright.green,
                &mut cs.bright.yellow,
                &mut cs.bright.blue,
                &mut cs.bright.magenta,
                &mut cs.bright.cyan,
                &mut cs.bright.white,
                &mut cs.bright.orange,
                &mut cs.bright.pink,
            ],
            Self::Black => vec![&mut cs.base.black, &mut cs.bright.black],
            Self::Red => vec![&mut cs.base.red, &mut cs.bright.red],
            Self::Green => vec![&mut cs.base.green, &mut cs.bright.green],
            Self::Yellow => vec![&mut cs.base.yellow, &mut cs.bright.yellow],
            Self::Blue => vec![&mut cs.base.blue, &mut cs.bright.blue],
            Self::Magenta => vec![&mut cs.base.magenta, &mut cs.bright.magenta],
            Self::Cyan => vec![&mut cs.base.cyan, &mut cs.bright.cyan],
            Self::White => vec![&mut cs.base.white, &mut cs.bright.white],
            Self::Orange => vec![&mut cs.base.orange, &mut cs.bright.orange],
            Self::Pink => vec![&mut cs.base.pink, &mut cs.bright.pink],
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ThemeAdjustment {
    #[default]
    None,

    /// Shift lightness in HSL space.
    ///
    /// Range: [-100, 100]
    ///
    /// 0 = unchanged
    /// +100 = maximum brightening (toward HSL lightness 100)
    /// -100 = maximum darkening (toward HSL lightness 0)
    ///
    /// Note: HSL lightness is not perceptually uniform; equal steps can
    /// look uneven across hues. For a perceptually flat brightness ramp,
    /// use `Exposure` instead.
    Brightness(ThemeColorGroup, f32),

    /// Increase or decrease tonal contrast around the midpoint.
    ///
    /// Range: [-100, 100]
    ///
    /// 0 = unchanged
    /// +100 = maximum contrast (colors pushed away from mid-lightness)
    /// -100 = minimum contrast (colors pulled toward mid-lightness)
    ///
    /// Operates on HSL lightness; the shift is proportional to each
    /// color's distance from 50% lightness, so near-mid colors change
    /// little while very light/dark colors change the most.
    Contrast(ThemeColorGroup, f32),

    /// Increase or decrease contrast by scaling each RGB channel around mid-gray (127).
    ///
    /// Range: [-100, 100]
    ///
    /// 0 = unchanged
    /// +100 = maximum expansion (channels pushed away from mid-gray)
    /// -100 = maximum compression (channels pulled toward mid-gray)
    ///
    /// Unlike `Contrast` (which operates on HSL lightness), this scales
    /// R/G/B channels independently, which can slightly shift hue and
    /// saturation. Handy for punching up a low-contrast ANSI palette, but
    /// works on any color group.
    ChannelContrast(ThemeColorGroup, f32),

    /// Adjust HSV saturation uniformly.
    ///
    /// Range: [-100, 100]
    ///
    /// -100 = grayscale
    /// +100 = maximum saturation
    Saturation(ThemeColorGroup, f32),

    /// Increase weakly saturated colors while preserving already vivid colors.
    ///
    /// Range: [-100, 100]
    ///
    /// Unlike `Saturation`, the effect scales down as source saturation
    /// increases, avoiding clipping of already-vivid colors.
    Vibrance(ThemeColorGroup, f32),

    /// Rotate hue.
    ///
    /// Range: [-180°, 180°]
    Hue(ThemeColorGroup, f32),

    /// Blue ↔ Orange white balance.
    ///
    /// Range: [-100, 100]
    ///
    /// +100 = shift toward orange (raises red, lowers blue)
    /// -100 = shift toward blue (raises blue, lowers red)
    Temperature(ThemeColorGroup, f32),

    /// Green ↔ Magenta white balance.
    ///
    /// Range: [-100, 100]
    ///
    /// +100 = shift toward magenta (lowers green, raises red/blue)
    /// -100 = shift toward green (raises green, lowers red/blue)
    Tint(ThemeColorGroup, f32),

    /// Multiplicative linear-light brightness adjustment (photographic exposure).
    ///
    /// Range: [-100, 100]
    ///
    /// 0 = unchanged
    /// +100 = double the linear-light value (brighter, highlights move most)
    /// -100 = halve the linear-light value (darker, shadows move least)
    ///
    /// Differs from `Brightness` (HSL lightness offset) and `Gamma` (power
    /// curve): this scales raw channel values, so bright colors shift more
    /// in absolute terms than dark ones — closer to how a camera exposure
    /// control behaves.
    Exposure(ThemeColorGroup, f32),

    /// Gamma correction.
    ///
    /// Range: [0.25, 4.0]
    ///
    /// 1.0 = unchanged
    /// >1.0 = brighter midtones
    /// <1.0 = darker midtones
    Gamma(ThemeColorGroup, f32),

    /// Raise or lower the black point.
    ///
    /// Range: [-100, 100]
    ///
    /// +100 = maximum lift (darkest color raised toward white)
    /// -100 = no lift; negative values are clamped to no-op territory
    ///        below the natural floor of 0
    BlackPoint(ThemeColorGroup, f32),

    /// Raise or lower the white point.
    ///
    /// Range: [-100, 100]
    ///
    /// +100 = maximum pull-down (lightest color lowered toward black)
    /// -100 = no change; negative values are clamped to no-op territory
    ///        above the natural ceiling of 255
    WhitePoint(ThemeColorGroup, f32),

    /// Invert lightness, turning a light color scheme into a dark one (or vice versa).
    ///
    /// Flips HSL lightness (`l` -> `100 - l`) while keeping hue and saturation
    /// untouched. Useful for deriving a dark variant of a light theme, or
    /// the reverse, without hand-picking every color again.
    Invert(ThemeColorGroup),
}

#[derive(Debug, Clone)]
pub struct ParseThemeAdjustmentError(String);

impl std::fmt::Display for ParseThemeAdjustmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseThemeAdjustmentError {}

impl std::str::FromStr for ThemeAdjustment {
    type Err = ParseThemeAdjustmentError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim();
        let mut parts = s.splitn(2, '=');
        let key = parts
            .next()
            .ok_or_else(|| ParseThemeAdjustmentError("missing '='".into()))?;
        let value_str = parts
            .next()
            .ok_or_else(|| ParseThemeAdjustmentError("missing value after '='".into()))?
            .trim();

        let value = value_str
            .parse::<f32>()
            .map_err(|_| ParseThemeAdjustmentError(format!("invalid number: '{}'", value_str)))?;

        let mut key_parts = key.trim().splitn(2, '.');
        let first = key_parts.next().unwrap();
        let (group, adjust) = if let Some(second) = key_parts.next() {
            (
                first.parse::<ThemeColorGroup>().map_err(|_| {
                    ParseThemeAdjustmentError(format!("unknown group: '{}'", first))
                })?,
                second,
            )
        } else {
            (ThemeColorGroup::All, first)
        };

        match adjust.to_lowercase().as_str() {
            "b" | "br" | "brightness" => Ok(Self::Brightness(group, value)),
            "e" | "exposure" => Ok(Self::Exposure(group, value)),
            "c" | "contrast" => Ok(Self::Contrast(group, value)),
            "cc" | "channel-contrast" => Ok(Self::ChannelContrast(group, value)),
            "s" | "sat" | "saturation" => Ok(Self::Saturation(group, value)),
            "v" | "vib" | "vibrance" => Ok(Self::Vibrance(group, value)),
            "h" | "hue" => Ok(Self::Hue(group, value)),
            "t" | "temp" | "temperature" => Ok(Self::Temperature(group, value)),
            "ti" | "tint" => Ok(Self::Tint(group, value)),
            "g" | "gamma" => Ok(Self::Gamma(group, value)),
            "bp" | "black-point" => Ok(Self::BlackPoint(group, value)),
            "wp" | "white-point" => Ok(Self::WhitePoint(group, value)),
            // Invert doesn't need a magnitude, but the `key=value` format
            // requires one; the value is accepted and ignored (e.g. `invert=1`).
            "i" | "invert" => Ok(Self::Invert(group)),
            _ => Err(ParseThemeAdjustmentError(format!(
                "unknown adjustment: '{}'",
                adjust
            ))),
        }
    }
}

impl ThemeAdjustment {
    pub fn apply(&self, cs: &mut ColorScheme) {
        match self {
            ThemeAdjustment::None => {}

            ThemeAdjustment::Brightness(group, v) => {
                for css in group.select_colors(cs) {
                    *css = css.color().lighten(*v).css();
                }
            }

            ThemeAdjustment::Contrast(group, v) => {
                let factor = *v / 100.0;
                for css in group.select_colors(cs) {
                    let color = css.color();
                    let (h, s, l) = color.hsl();
                    let new_l = (50.0 + (l - 50.0) * (1.0 + factor)).clamp(0.0, 100.0);
                    *css = Color::from_hsl(h, s, new_l).css();
                }
            }

            ThemeAdjustment::ChannelContrast(group, v) => {
                let factor = *v / 100.0;
                let mid = 127.0;
                for css in group.select_colors(cs) {
                    let color = css.color();
                    let (r, g, b) = color.rgb();
                    let adjust = |c: u8| -> u8 {
                        (mid + (c as f32 - mid) * (1.0 + factor))
                            .clamp(0.0, 255.0)
                            .round() as u8
                    };
                    *css = Color::from_rgb(adjust(r), adjust(g), adjust(b)).css();
                }
            }

            ThemeAdjustment::Saturation(group, v) => {
                for css in group.select_colors(cs) {
                    *css = css.color().saturate(*v).css();
                }
            }

            ThemeAdjustment::Vibrance(group, v) => {
                let amount = *v / 100.0;
                for css in group.select_colors(cs) {
                    let color = css.color();
                    let (h, s, val) = color.hsv();
                    let scale = 1.0 - (s / 100.0);
                    let adjusted_s = s + amount * 100.0 * scale;
                    *css = Color::from_hsv(h, adjusted_s.clamp(0.0, 100.0), val).css();
                }
            }

            ThemeAdjustment::Hue(group, v) => {
                for css in group.select_colors(cs) {
                    *css = css.color().rotate(*v, 360.0).css();
                }
            }

            ThemeAdjustment::Temperature(group, v) => {
                let factor = *v / 100.0;
                for css in group.select_colors(cs) {
                    let color = css.color();
                    let (r, g, b) = color.rgb();
                    let new_r = (r as f32 + factor * 100.0).clamp(0.0, 255.0).round() as u8;
                    let new_b = (b as f32 - factor * 100.0).clamp(0.0, 255.0).round() as u8;
                    *css = Color::from_rgb(new_r, g, new_b).css();
                }
            }

            ThemeAdjustment::Tint(group, v) => {
                let factor = *v / 100.0;
                for css in group.select_colors(cs) {
                    let color = css.color();
                    let (r, g, b) = color.rgb();
                    let new_g = (g as f32 - factor * 100.0).clamp(0.0, 255.0).round() as u8;
                    let new_r = (r as f32 + factor * 50.0).clamp(0.0, 255.0).round() as u8;
                    let new_b = (b as f32 + factor * 50.0).clamp(0.0, 255.0).round() as u8;
                    *css = Color::from_rgb(new_r, new_g, new_b).css();
                }
            }

            ThemeAdjustment::Gamma(group, v) => {
                let gamma = v.clamp(0.25, 4.0);
                for css in group.select_colors(cs) {
                    let color = css.color();
                    let (r, g, b) = color.rgb();
                    let adjust = |c: u8| -> u8 {
                        ((c as f32 / 255.0).powf(1.0 / gamma) * 255.0).round() as u8
                    };
                    *css = Color::from_rgb(adjust(r), adjust(g), adjust(b)).css();
                }
            }

            ThemeAdjustment::BlackPoint(group, v) => {
                let shift = *v / 100.0;
                for css in group.select_colors(cs) {
                    let color = css.color();
                    let (r, g, b) = color.rgb();
                    let adjust = |c: u8| -> u8 {
                        let normalized = c as f32 / 255.0;
                        let adjusted = normalized + shift * (1.0 - normalized);
                        (adjusted.clamp(0.0, 1.0) * 255.0).round() as u8
                    };
                    *css = Color::from_rgb(adjust(r), adjust(g), adjust(b)).css();
                }
            }

            ThemeAdjustment::WhitePoint(group, v) => {
                let shift = *v / 100.0;
                for css in group.select_colors(cs) {
                    let color = css.color();
                    let (r, g, b) = color.rgb();
                    let adjust = |c: u8| -> u8 {
                        let normalized = c as f32 / 255.0;
                        let adjusted = normalized - shift * normalized;
                        (adjusted.clamp(0.0, 1.0) * 255.0).round() as u8
                    };
                    *css = Color::from_rgb(adjust(r), adjust(g), adjust(b)).css();
                }
            }

            ThemeAdjustment::Exposure(group, v) => {
                let factor = 2f32.powf(*v / 100.0); // -100..100 -> ×0.5..×2.0
                for css in group.select_colors(cs) {
                    let color = css.color();
                    let (r, g, b) = color.rgb();
                    let adjust =
                        |c: u8| -> u8 { (c as f32 * factor).clamp(0.0, 255.0).round() as u8 };
                    *css = Color::from_rgb(adjust(r), adjust(g), adjust(b)).css();
                }
            }

            ThemeAdjustment::Invert(group) => {
                for css in group.select_colors(cs) {
                    let color = css.color();
                    let (h, s, l) = color.hsl();
                    *css = Color::from_hsl(h, s, 100.0 - l).css();
                }
            }
        }
    }
}
