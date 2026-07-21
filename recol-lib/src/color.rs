use crate::{Error, Result};
use serde::{Deserialize, Serialize};

/// Size of Color in bytes.
pub const COLOR_SIZE: usize = 3;

#[inline(always)]
fn clamp(v: f32, min: f32, max: f32) -> f32 {
    v.clamp(min, max)
}

#[inline(always)]
fn to_u8(v: f32) -> u8 {
    (v * 255.0).round() as u8
}

/// An RGB color stored as sRGB-encoded (gamma-corrected) floats in `[0.0, 1.0]`.
/// Use `lab()`/`from_lab()` (or the internal `srgb_to_linear`/`linear_to_srgb`
/// helpers) to work in linear light.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl TryFrom<&[u8]> for Color {
    type Error = Error;

    /// Constructs a [`Color`] from a three-byte RGB slice.
    fn try_from(bytes: &[u8]) -> Result<Self> {
        match bytes {
            [r, g, b] => Ok(Self::from_rgb(*r, *g, *b)),
            _ => Err(Error::InvalidLength {
                src: "Color::try_from::<&[u8]>".into(),
                expected: COLOR_SIZE,
                got: bytes.len(),
            }),
        }
    }
}

impl std::str::FromStr for Color {
    type Err = Error;

    /// Parses a CSS hex color string (`#rrggbb` or `rrggbb`).
    fn from_str(s: &str) -> Result<Self> {
        let hex = s.trim_start_matches('#');

        if hex.len() != 6 {
            return Err(Error::InvalidLength {
                src: "Color::from_str".into(),
                expected: 6,
                got: hex.len(),
            });
        }

        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(Error::InvalidHex(hex.into()));
        }

        let value = u32::from_str_radix(hex, 16).expect("already validated");
        Ok(Self::from_hex(value))
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.css())
    }
}

impl Color {
    /// Creates a color from linear RGB floats, clamping each channel to `[0.0, 1.0]`.
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: clamp(r, 0.0, 1.0),
            g: clamp(g, 0.0, 1.0),
            b: clamp(b, 0.0, 1.0),
        }
    }

    /// Creates a color from 8-bit RGB components.
    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    /// Creates a color from a packed 24-bit hex value (`0xRRGGBB`).
    #[inline]
    pub fn from_hex(hex: u32) -> Self {
        Self::from_rgb(
            ((hex >> 16) & 0xff) as u8,
            ((hex >> 8) & 0xff) as u8,
            (hex & 0xff) as u8,
        )
    }

    /// Creates a color from HSV components.
    ///
    /// - `h` – hue in degrees (wrapped to `[0°, 360°)`)
    /// - `s` – saturation in `[0, 100]`
    /// - `v` – value/brightness in `[0, 100]`
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let h = h.rem_euclid(360.0);
        let s = clamp(s, 0.0, 100.0) / 100.0;
        let v = clamp(v, 0.0, 100.0) / 100.0;

        let channel = |n: f32| {
            let k = (n + h / 60.0).rem_euclid(6.0);
            v - v * s * clamp(k.min(4.0 - k), 0.0, 1.0)
        };

        Self::new(channel(5.0), channel(3.0), channel(1.0))
    }

    /// Creates a color from HSL components.
    ///
    /// - `h` – hue in degrees (wrapped to `[0°, 360°)`)
    /// - `s` – saturation in `[0, 100]`
    /// - `l` – lightness in `[0, 100]`
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let h = h.rem_euclid(360.0);
        let s = clamp(s, 0.0, 100.0) / 100.0;
        let l = clamp(l, 0.0, 100.0) / 100.0;

        let a = s * l.min(1.0 - l);

        let channel = |n: f32| {
            let k = (n + h / 30.0).rem_euclid(12.0);
            l - a * clamp((k - 3.0).min(9.0 - k), -1.0, 1.0)
        };

        Self::new(channel(0.0), channel(8.0), channel(4.0))
    }

    /// Converts CIE L*a*b* back to a `Color`, clamping to the sRGB gamut.
    pub fn from_lab(l: f32, a: f32, b: f32) -> Self {
        let fy = (l + 16.0) / 116.0;
        let fx = a / 500.0 + fy;
        let fz = fy - b / 200.0;

        let x = D65_XN * lab_f_inv(fx);
        let y = D65_YN * lab_f_inv(fy);
        let z = D65_ZN * lab_f_inv(fz);

        let r = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
        let g = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
        let bl = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;

        Color::new(
            linear_to_srgb(r.clamp(0.0, 1.0)),
            linear_to_srgb(g.clamp(0.0, 1.0)),
            linear_to_srgb(bl.clamp(0.0, 1.0)),
        )
    }

    #[inline]
    pub fn try_from_bytes(b: &[u8]) -> Result<Self> {
        Self::try_from(b)
    }

    /// Parses a CSS hex color string (`#rrggbb` or `rrggbb`).
    #[inline]
    pub fn try_from_css(s: &str) -> Result<Self> {
        s.parse()
    }

    /// Returns the color as `(r, g, b)` bytes.
    #[inline]
    pub fn rgb(&self) -> (u8, u8, u8) {
        (to_u8(self.r), to_u8(self.g), to_u8(self.b))
    }

    #[inline]
    pub fn bytes(&self) -> Vec<u8> {
        let (r, g, b) = self.rgb();
        vec![r, g, b]
    }

    /// Returns the color as a packed `0xRRGGBB` integer.
    #[inline]
    pub fn hex(&self) -> u32 {
        let (r, g, b) = self.rgb();
        (r as u32) << 16 | (g as u32) << 8 | b as u32
    }

    /// Returns the color as a lowercase CSS hex string (e.g. `"#1a2b3c"`).
    #[inline]
    pub fn css(&self) -> CssColor {
        CssColor(format!("#{:06x}", self.hex()))
    }

    fn hue(&self) -> f32 {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == self.r {
            60.0 * (self.g - self.b) / delta
        } else if max == self.g {
            60.0 * ((self.b - self.r) / delta + 2.0)
        } else {
            60.0 * ((self.r - self.g) / delta + 4.0)
        };

        h.rem_euclid(360.0)
    }

    /// Returns `(hue °, saturation %, value %)`.
    pub fn hsv(&self) -> (f32, f32, f32) {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max - min;

        let s = if max == 0.0 { 0.0 } else { delta / max };
        (self.hue(), s * 100.0, max * 100.0)
    }

    /// Returns `(hue °, saturation %, lightness %)`.
    pub fn hsl(&self) -> (f32, f32, f32) {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let l = (max + min) / 2.0;
        let delta = max - min;

        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        (self.hue(), s * 100.0, l * 100.0)
    }

    /// Returns the relative luminance as defined by WCAG 2.x.
    pub fn luminance(&self) -> f32 {
        0.2126 * srgb_to_linear(self.r)
            + 0.7152 * srgb_to_linear(self.g)
            + 0.0722 * srgb_to_linear(self.b)
    }

    /// Converts a `Color` to CIE L*a*b* (D65 white point).
    pub fn lab(self) -> (f32, f32, f32) {
        let (r, g, b) = (
            srgb_to_linear(self.r),
            srgb_to_linear(self.g),
            srgb_to_linear(self.b),
        );

        let x = (0.4124564 * r + 0.3575761 * g + 0.1804375 * b) / D65_XN;
        let y = (0.2126729 * r + 0.7151522 * g + 0.0721750 * b) / D65_YN;
        let z = (0.0193339 * r + 0.1191920 * g + 0.9503041 * b) / D65_ZN;

        let (fx, fy, fz) = (lab_f(x), lab_f(y), lab_f(z));
        (116.0 * fy - 16.0, 500.0 * (fx - fy), 200.0 * (fy - fz))
    }

    /// Linearly interpolates between `self` and `other`.
    ///
    /// `f = 0.0` returns `self`; `f = 1.0` returns `other`.
    pub fn blend(&self, other: &Color, f: f32) -> Color {
        Color::new(
            self.r + (other.r - self.r) * f,
            self.g + (other.g - self.g) * f,
            self.b + (other.b - self.b) * f,
        )
    }

    /// Lightens (`f > 0`) or darkens (`f < 0`) the color by blending toward
    /// white or black by the given proportion `|f|` in `[0.0, 1.0]`.
    pub fn shade(&self, f: f32) -> Color {
        let target = if f >= 0.0 { 1.0 } else { 0.0 };
        let p = f.abs();
        Color::new(
            self.r + (target - self.r) * p,
            self.g + (target - self.g) * p,
            self.b + (target - self.b) * p,
        )
    }

    /// Adjusts HSV *value* by `v` percentage points.
    pub fn brighten(&self, v: f32) -> Color {
        let (h, s, val) = self.hsv();
        Color::from_hsv(h, s, clamp(val + v, 0.0, 100.0))
    }

    /// Adjusts HSL *lightness* by `v` percentage points.
    pub fn lighten(&self, v: f32) -> Color {
        let (h, s, val) = self.hsl();
        Color::from_hsl(h, s, clamp(val + v, 0.0, 100.0))
    }

    /// Adjusts HSV *saturation* by `v` percentage points.
    pub fn saturate(&self, v: f32) -> Color {
        let (h, s, val) = self.hsv();
        Color::from_hsv(h, clamp(s + v, 0.0, 100.0), val)
    }

    /// Rotates the hue by `v` degrees.
    pub fn rotate(&self, v: f32, rhs: f32) -> Color {
        let (h, s, val) = self.hsv();
        Color::from_hsv((h + v).rem_euclid(rhs), s, val)
    }
}

/// A validated CSS hex color string (e.g. `#1a2b3c`).
///
/// The inner `String` is always a lowercase 7-character string of the form `#rrggbb`.
/// The only way to construct this type is via [`CssColor::new`] or [`FromStr`],
/// both of which enforce the invariant.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CssColor(String);
use std::io::Write;

pub fn print_palette(colors: &[Color]) {
    print!("\x1b[48;2;90;90;90m");
    for _ in colors {
        print!("    ");
    }
    println!("\x1b[0m");
    for _ in 0..2 {
        for c in colors {
            let (r, g, b) = c.rgb();
            print!("\x1b[48;2;{};{};{}m    \x1b[0m", r, g, b);
        }
        println!();
    }
    print!("\x1b[48;2;90;90;90m");
    for _ in colors {
        print!("    ");
    }
    println!("\x1b[0m");

    std::io::stdout().flush().unwrap();
}

impl Default for CssColor {
    fn default() -> Self {
        Self("#000000".into())
    }
}

impl CssColor {
    pub fn new(s: &str) -> Result<Self> {
        Ok(s.parse::<Color>()?.css())
    }

    pub fn color(&self) -> Color {
        self.as_str()
            .parse()
            .expect("CssColor invariant guarantees validity")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Into<String> for CssColor {
    fn into(self) -> String {
        self.0
    }
}

impl std::str::FromStr for CssColor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl std::fmt::Display for CssColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

const D65_XN: f32 = 0.95047;
const D65_YN: f32 = 1.00000;
const D65_ZN: f32 = 1.08883;

fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

fn lab_f(t: f32) -> f32 {
    if t > 0.008856 {
        t.powf(1.0 / 3.0)
    } else {
        7.787 * t + 16.0 / 116.0
    }
}

fn lab_f_inv(t: f32) -> f32 {
    let t3 = t * t * t;
    if t3 > 0.008856 {
        t3
    } else {
        (t - 16.0 / 116.0) / 7.787
    }
}
