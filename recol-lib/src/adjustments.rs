use crate::{Color, ColorScheme, CssColor};

/// Represents a group of theme colors that can be selected for operations.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ThemeColorGroup {
    #[default]
    All,
    UI,
    Background,
    BaseBackground,
    SelectionBackground,
    CursorBackground,
    Foreground,
    BaseForeground,
    SelectionForeground,
    CursorForeground,
    Selection,
    Cursor,
    Palette,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Orange,
    Pink,
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

#[derive(Debug, Clone)]
pub struct ParseThemeAdjustmentError(String);

impl std::fmt::Display for ParseThemeAdjustmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseThemeAdjustmentError {}

pub fn parse_theme_adjustments(s: &str) -> Result<Vec<ThemeAdjustment>, ParseThemeAdjustmentError> {
    let mut res = Vec::new();
    for part in s.split(",") {
        res.push(part.parse()?);
    }
    Ok(res)
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ThemeAdjustment {
    #[default]
    None,

    /// HSL lightness shift, proportional toward white (+) or black (-).
    /// Range: [-100, 100]. 0 = unchanged, +100 = white, -100 = black.
    /// Not perceptually uniform across hues — use `Exposure` for that.
    Brightness(ThemeColorGroup, f32),

    /// Tonal contrast around the midpoint, in HSL lightness.
    /// Range: [-100, 100]. +100 = max contrast, -100 = flat (all → L=50).
    /// Hue and saturation are never affected.
    Contrast(ThemeColorGroup, f32),

    /// HSV saturation shift, proportional toward full (+) or gray (-).
    /// Range: [-100, 100]. +100 = fully saturated, -100 = grayscale.
    Saturation(ThemeColorGroup, f32),

    /// Cap the most saturated colors, leaving weaker colors untouched.
    /// Range: [0, 100]. 0 = no effect, 100 = fully desaturated.
    /// Value = how much to shave off the saturation ceiling (ceiling = 100 - N).
    /// Handy for taming an overly acidic ANSI palette without affecting
    /// already-muted colors.
    SaturationCap(ThemeColorGroup, f32),

    /// Saturation shift that protects already-vivid colors.
    /// Range: [-100, 100].
    /// +: weak colors gain saturation fastest, vivid colors barely move.
    /// -: vivid colors lose saturation fastest, weak colors barely move.
    Vibrance(ThemeColorGroup, f32),

    /// Hue rotation, degrees. Range: [-180, 180].
    Hue(ThemeColorGroup, f32),

    /// Blue ↔ Orange white balance. Range: [-100, 100].
    /// +100 = orange (red up, blue down). -100 = blue (blue up, red down).
    Temperature(ThemeColorGroup, f32),

    /// Green ↔ Magenta white balance. Range: [-100, 100].
    /// +100 = magenta (green down). -100 = green (green up).
    Tint(ThemeColorGroup, f32),

    /// Photographic exposure: scales linear-light value, gamma-correct.
    /// Range: [-100, 100] (±1 stop). +100 = double linear light, -100 = half.
    /// Highlights move more than shadows in absolute terms — unlike
    /// `Brightness` (HSL offset) or `Gamma` (power curve).
    Exposure(ThemeColorGroup, f32),

    /// Gamma correction. Range: [0.25, 4.0]. 1.0 = unchanged.
    /// >1.0 brightens midtones, <1.0 darkens them.
    Gamma(ThemeColorGroup, f32),

    /// Blend each RGB channel toward black or white.
    /// Range: [-100, 100]. 0 = unchanged.
    /// -100 = fully black, +100 = fully white.
    /// Unlike `Brightness` (HSL lightness shift), this blends raw RGB
    /// channels directly toward an endpoint — a fade/wash effect rather
    /// than a perceptual lightness change.
    Fade(ThemeColorGroup, f32),

    /// Flip HSL lightness (`l → 100 - l`); hue/saturation untouched.
    /// Turns a light theme dark or vice versa.
    Invert(ThemeColorGroup),
}

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
            "s" | "sat" | "saturation" => Ok(Self::Saturation(group, value)),
            "sc" | "saturation-cap" => Ok(Self::SaturationCap(group, value)),
            "v" | "vib" | "vibrance" => Ok(Self::Vibrance(group, value)),
            "h" | "hue" => Ok(Self::Hue(group, value)),
            "t" | "temp" | "temperature" => Ok(Self::Temperature(group, value)),
            "ti" | "tint" => Ok(Self::Tint(group, value)),
            "g" | "gamma" => Ok(Self::Gamma(group, value)),
            "f" | "fade" => Ok(Self::Fade(group, value)),
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
                let v = v.clamp(-100.0, 100.0);
                for css in group.select_colors(cs) {
                    let (h, s, l) = css.color().hsl();
                    let new_l = if v >= 0.0 {
                        l + (100.0 - l) * (v / 100.0)
                    } else {
                        l + l * (v / 100.0)
                    };
                    *css = Color::from_hsl(h, s, new_l).css();
                }
            }

            ThemeAdjustment::Contrast(group, v) => {
                let factor = v.clamp(-100.0, 100.0) / 100.0;
                for css in group.select_colors(cs) {
                    let (h, s, l) = css.color().hsl();
                    let new_l = (50.0 + (l - 50.0) * (1.0 + factor)).clamp(0.0, 100.0);
                    *css = Color::from_hsl(h, s, new_l).css();
                }
            }

            ThemeAdjustment::Saturation(group, v) => {
                let v = v.clamp(-100.0, 100.0);
                for css in group.select_colors(cs) {
                    let (h, s, val) = css.color().hsv();
                    let new_s = if v >= 0.0 {
                        s + (100.0 - s) * (v / 100.0)
                    } else {
                        s + s * (v / 100.0)
                    };
                    *css = Color::from_hsv(h, new_s, val).css();
                }
            }

            ThemeAdjustment::SaturationCap(group, v) => {
                let ceiling = 100.0 - v.clamp(0.0, 100.0);
                for css in group.select_colors(cs) {
                    let (h, s, val) = css.color().hsv();
                    *css = Color::from_hsv(h, s.min(ceiling), val).css();
                }
            }

            ThemeAdjustment::Vibrance(group, v) => {
                let amount = v.clamp(-100.0, 100.0) / 100.0;
                for css in group.select_colors(cs) {
                    let (h, s, val) = css.color().hsv();
                    let scale = if amount >= 0.0 {
                        1.0 - s / 100.0
                    } else {
                        s / 100.0
                    };
                    let new_s = (s + amount * 100.0 * scale).clamp(0.0, 100.0);
                    *css = Color::from_hsv(h, new_s, val).css();
                }
            }

            ThemeAdjustment::Hue(group, v) => {
                let v = v.clamp(-180.0, 180.0);
                for css in group.select_colors(cs) {
                    *css = css.color().rotate(v, 360.0).css();
                }
            }

            ThemeAdjustment::Temperature(group, v) => {
                let factor = v.clamp(-100.0, 100.0) / 100.0;
                for css in group.select_colors(cs) {
                    let (r, g, b) = css.color().rgb();
                    let new_r = (r as f32 + factor * 100.0).clamp(0.0, 255.0).round() as u8;
                    let new_b = (b as f32 - factor * 100.0).clamp(0.0, 255.0).round() as u8;
                    *css = Color::from_rgb(new_r, g, new_b).css();
                }
            }

            ThemeAdjustment::Tint(group, v) => {
                let factor = v.clamp(-100.0, 100.0) / 100.0;
                for css in group.select_colors(cs) {
                    let (r, g, b) = css.color().rgb();
                    let new_g = (g as f32 - factor * 100.0).clamp(0.0, 255.0).round() as u8;
                    let new_r = (r as f32 + factor * 50.0).clamp(0.0, 255.0).round() as u8;
                    let new_b = (b as f32 + factor * 50.0).clamp(0.0, 255.0).round() as u8;
                    *css = Color::from_rgb(new_r, new_g, new_b).css();
                }
            }

            ThemeAdjustment::Exposure(group, v) => {
                let factor = 2f32.powf(v.clamp(-100.0, 100.0) / 100.0);

                // Exact sRGB transfer curve (matches Color::luminance's decode).
                let decode = |c: f32| {
                    if c > 0.04045 {
                        ((c + 0.055) / 1.055).powf(2.4)
                    } else {
                        c / 12.92
                    }
                };
                let encode = |c: f32| {
                    if c > 0.0031308 {
                        1.055 * c.powf(1.0 / 2.4) - 0.055
                    } else {
                        c * 12.92
                    }
                };

                for css in group.select_colors(cs) {
                    let (r, g, b) = css.color().rgb();
                    let adjust = |c: u8| -> u8 {
                        let linear = decode(c as f32 / 255.0);
                        let scaled = (linear * factor).clamp(0.0, 1.0);
                        (encode(scaled) * 255.0).round() as u8
                    };
                    *css = Color::from_rgb(adjust(r), adjust(g), adjust(b)).css();
                }
            }

            ThemeAdjustment::Gamma(group, v) => {
                let gamma = v.clamp(0.25, 4.0);
                for css in group.select_colors(cs) {
                    let (r, g, b) = css.color().rgb();
                    let adjust = |c: u8| -> u8 {
                        ((c as f32 / 255.0).powf(1.0 / gamma) * 255.0).round() as u8
                    };
                    *css = Color::from_rgb(adjust(r), adjust(g), adjust(b)).css();
                }
            }

            ThemeAdjustment::Fade(group, v) => {
                let shift = v.clamp(-100.0, 100.0) / 100.0;
                for css in group.select_colors(cs) {
                    *css = css.color().shade(shift).css();
                }
            }

            ThemeAdjustment::Invert(group) => {
                for css in group.select_colors(cs) {
                    let (h, s, l) = css.color().hsl();
                    *css = Color::from_hsl(h, s, 100.0 - l).css();
                }
            }
        }
    }
}
