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

    /// Photographic exposure: scales linear-light value, gamma-correct.
    /// Range: [-100, 100] (±1 stop). +100 = double linear light, -100 = half.
    /// Highlights move more than shadows in absolute terms — unlike
    /// `Brightness` (HSL offset) or `Gamma` (power curve).
    Exposure(ThemeColorGroup, f32),

    /// Blend each RGB channel toward black or white.
    /// Range: [-100, 100]. 0 = unchanged.
    /// -100 = fully black, +100 = fully white.
    /// Unlike `Brightness` (HSL lightness shift), this blends raw RGB
    /// channels directly toward an endpoint — a fade/wash effect rather
    /// than a perceptual lightness change.
    Fade(ThemeColorGroup, f32),

    /// Shifts lightness uniformly for all colors in the group.
    /// `-100..100`: negative darkens, positive lightens.
    Brightness(ThemeColorGroup, f32),

    /// Applies a nonlinear midtone curve; endpoints (black/white) stay fixed.
    /// `-100..100`: negative darkens midtones, positive brightens them.
    Gamma(ThemeColorGroup, f32),

    /// Spreads or compresses lightness around a pivot point.
    /// `-100..100`: negative reduces contrast, positive increases it.
    /// Optional pivot lightness (`0..100`, default `50`) to contrast around.
    Contrast(ThemeColorGroup, f32, Option<f32>),

    /// Scales saturation uniformly for all colors in the group.
    /// `-100..100`: negative desaturates, positive saturates.
    Saturation(ThemeColorGroup, f32),

    /// Scales saturation non-uniformly: weakly saturated colors are boosted
    /// more, near-neutral grays and already-vivid colors are protected.
    /// `-100..100`: negative reduces vibrance, positive increases it.
    Vibrance(ThemeColorGroup, f32),

    /// Rotates hue by an angle for all colors in the group.
    /// `-100..100`, mapped to `-180°..180°`.
    Hue(ThemeColorGroup, f32),

    /// Shifts colors along the blue↔yellow axis (Lab b*).
    /// `-100..100`: negative cools (blue), positive warms (yellow).
    Temperature(ThemeColorGroup, f32),

    /// Shifts colors along the green↔magenta axis (Lab a*).
    /// `-100..100`: negative shifts green, positive shifts magenta.
    Tint(ThemeColorGroup, f32),

    /// Pulls every color in the group toward the group's average lightness
    /// and/or chroma, reducing visual imbalance within the palette.
    /// `-100..100`: strength of the pull toward the mean (0 = no change).
    /// `NormalizeChannel` selects whether lightness, chroma, or both are affected.
    Normalize(ThemeColorGroup, f32, NormalizeChannel),

    /// Flips lightness around a pivot while preserving hue and saturation,
    /// e.g. to derive a light theme from a dark one.
    /// `-100..100`: blend strength toward the fully inverted color (sign ignored).
    /// Optional pivot lightness (`0..100`, default `50`) to invert around.
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

        Ok(match adjust {
            "b" | "brightness" => Self::Brightness(group, value),
            "e" | "exposure" => Self::Exposure(group, value),
            "c" | "contrast" => Self::Contrast(group, value, None),
            "s" | "sat" | "saturation" => Self::Saturation(group, value),
            "v" | "vib" | "vibrance" => Self::Vibrance(group, value),
            "h" | "hue" => Self::Hue(group, value),
            "t" | "temp" | "temperature" => Self::Temperature(group, value),
            "ti" | "tint" => Self::Tint(group, value),
            "g" | "gamma" => Self::Gamma(group, value),
            "f" | "fade" => Self::Fade(group, value),
            "i" | "invert" => Self::Invert(group),
            "n" | "norm" | "normalize" => {
                Self::Normalize(group, value, NormalizeChannel::Lightness)
            }
            "nb" | "norm-both" => Self::Normalize(group, value, NormalizeChannel::Both),
            "nc" | "norm-chroma" => Self::Normalize(group, value, NormalizeChannel::Chroma),
            _ => {
                return Err(ParseThemeAdjustmentError(format!(
                    "unknown adjustment: '{adjust}'",
                )));
            }
        })
    }
}

impl ThemeAdjustment {
    pub fn apply(&self, cs: &mut ColorScheme) {
        match self {
            ThemeAdjustment::None => {}

            ThemeAdjustment::Fade(group, v) => {
                let shift = pct(*v) / 100.0;
                for css in group.select_colors(cs) {
                    *css = css.color().shade(shift).css();
                }
            }

            ThemeAdjustment::Exposure(group, v) => {
                let factor = 2f32.powf(pct(*v) / 100.0);

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
                let adjust = |c: f32| encode((decode(c) * factor).clamp(0.0, 1.0));

                for_each_rgb(group, cs, |r, g, b| (adjust(r), adjust(g), adjust(b)));
            }

            ThemeAdjustment::Brightness(group, v) => {
                let delta = pct(*v);
                for_each_hsl(group, cs, |h, s, l| (h, s, (l + delta).clamp(0.0, 100.0)));
            }

            ThemeAdjustment::Gamma(group, v) => {
                // v = 100 -> exponent 0.5 (brighten midtones)
                // v = -100 -> exponent 2.0 (darken midtones)
                let exponent = 2f32.powf(-pct(*v) / 100.0);
                for_each_hsl(group, cs, |h, s, l| {
                    let normalized = (l / 100.0).clamp(0.0, 1.0);
                    (h, s, (normalized.powf(exponent) * 100.0).clamp(0.0, 100.0))
                });
            }

            ThemeAdjustment::Contrast(group, v, pivot) => {
                // Maps -100..100 onto a spread factor of 0.0..2.0 around `pivot`.
                let factor = pct(-*v) / 100.0 + 1.0;
                let pivot = pivot.unwrap_or(50.0);
                for_each_hsl(group, cs, |h, s, l| {
                    (h, s, (pivot + (l - pivot) * factor).clamp(0.0, 100.0))
                });
            }

            ThemeAdjustment::Saturation(group, v) => {
                let v = pct(*v);
                for_each_hsv(group, cs, |h, s, val| {
                    let new_s = if v >= 0.0 {
                        s + (100.0 - s) * (v / 100.0)
                    } else {
                        s + s * (v / 100.0)
                    };
                    (h, new_s, val)
                });
            }

            ThemeAdjustment::Vibrance(group, v) => {
                let amount = pct(*v) / 100.0;
                for_each_hsv(group, cs, |h, s, val| {
                    // Parabolic weight: zero at s=0 (protects grays) and
                    // s=100 (protects already-vivid accents), peaking at s=50.
                    let weight = 4.0 * (s / 100.0) * (1.0 - s / 100.0);
                    let delta = if amount >= 0.0 {
                        (100.0 - s) * amount * weight
                    } else {
                        s * amount * weight
                    };
                    (h, (s + delta).clamp(0.0, 100.0), val)
                });
            }

            ThemeAdjustment::Hue(group, v) => {
                let degrees = pct(*v) / 100.0 * 180.0;
                for_each_hsv(group, cs, |h, s, val| {
                    ((h + degrees).rem_euclid(360.0), s, val)
                });
            }

            ThemeAdjustment::Temperature(group, v) => {
                let shift = pct(*v) * TEMP_TINT_SCALE;
                for_each_lab(group, cs, |l, a, b| (l, a, b + shift));
            }

            ThemeAdjustment::Tint(group, v) => {
                // Positive shifts toward magenta, negative toward green.
                let shift = pct(*v) * TEMP_TINT_SCALE;
                for_each_lab(group, cs, |l, a, b| (l, a + shift, b));
            }

            ThemeAdjustment::Normalize(group, v, channel) => {
                let strength = pct(*v) / 100.0;

                let samples: Vec<(f32, f32, f32)> = group
                    .select_colors(cs)
                    .iter()
                    .map(|css| css.color().hsl())
                    .collect();
                if samples.is_empty() {
                    return;
                }

                let mean_l = samples.iter().map(|(_, _, l)| l).sum::<f32>() / samples.len() as f32;
                let mean_s = samples.iter().map(|(_, s, _)| s).sum::<f32>() / samples.len() as f32;

                for (css, (h, s, l)) in group.select_colors(cs).into_iter().zip(samples) {
                    let new_l = match channel {
                        NormalizeChannel::Lightness | NormalizeChannel::Both => {
                            (l + (mean_l - l) * strength).clamp(0.0, 100.0)
                        }
                        NormalizeChannel::Chroma => l,
                    };
                    let new_s = match channel {
                        NormalizeChannel::Chroma | NormalizeChannel::Both => {
                            (s + (mean_s - s) * strength).clamp(0.0, 100.0)
                        }
                        NormalizeChannel::Lightness => s,
                    };
                    *css = Color::from_hsl(h, new_s, new_l).css();
                }
            }

            ThemeAdjustment::Invert(group) => {
                for_each_hsl(group, cs, |h, s, l| (h, s, 100.0 - l));
            }
        }
    }
}

/// Clamps an adjustment value to the standard `[-100, 100]` input range.
#[inline]
fn pct(v: f32) -> f32 {
    v.clamp(-100.0, 100.0)
}

/// Applies `f` to each color's HSL components and writes the result back.
fn for_each_hsl(
    group: &ThemeColorGroup,
    cs: &mut ColorScheme,
    mut f: impl FnMut(f32, f32, f32) -> (f32, f32, f32),
) {
    for css in group.select_colors(cs) {
        let (h, s, l) = css.color().hsl();
        let (h, s, l) = f(h, s, l);
        *css = Color::from_hsl(h, s, l).css();
    }
}

/// Applies `f` to each color's HSV components and writes the result back.
fn for_each_hsv(
    group: &ThemeColorGroup,
    cs: &mut ColorScheme,
    mut f: impl FnMut(f32, f32, f32) -> (f32, f32, f32),
) {
    for css in group.select_colors(cs) {
        let (h, s, v) = css.color().hsv();
        let (h, s, v) = f(h, s, v);
        *css = Color::from_hsv(h, s, v).css();
    }
}

/// Applies `f` to each color's CIE Lab components and writes the result back.
fn for_each_lab(
    group: &ThemeColorGroup,
    cs: &mut ColorScheme,
    mut f: impl FnMut(f32, f32, f32) -> (f32, f32, f32),
) {
    for css in group.select_colors(cs) {
        let (l, a, b) = css.color().lab();
        let (l, a, b) = f(l, a, b);
        *css = Color::from_lab(l, a, b).css();
    }
}

/// Applies `f` to each color's raw RGB channels and writes the result back.
fn for_each_rgb(
    group: &ThemeColorGroup,
    cs: &mut ColorScheme,
    mut f: impl FnMut(f32, f32, f32) -> (f32, f32, f32),
) {
    for css in group.select_colors(cs) {
        let c = css.color();
        let (r, g, b) = f(c.r, c.g, c.b);
        *css = Color::new(r, g, b).css();
    }
}

/// Scales the -100..100 input range onto a perceptually reasonable
/// Lab a*/b* excursion (empirically ~±80 covers the practical gamut).
const TEMP_TINT_SCALE: f32 = 0.8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NormalizeChannel {
    Lightness,
    Chroma,
    Both,
}
