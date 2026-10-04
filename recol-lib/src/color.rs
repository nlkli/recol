use crate::{Error, Result};
use serde::{Deserialize, Serialize};

/// Size of Color in bytes.
pub const COLOR_SIZE: usize = 3;

/// An RGB color stored as sRGB-encoded (gamma-corrected) floats in `[0.0, 1.0]`.
///
/// Use [`Color::to_linear`]/[`Color::from_linear`] to work in linear light and
/// [`Color::lab`]/[`Color::from_lab`] to work in CIE L*a*b* (D65).
///
/// All constructors clamp channels to `[0.0, 1.0]` and map `NaN` to `0.0`, which
/// keeps the `Eq`/`Hash` implementations valid. The fields are public, so
/// building a `Color` with a struct literal bypasses this sanitising.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Eq for Color {}

impl std::hash::Hash for Color {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hex().hash(state);
    }
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
        // `strip_prefix` removes at most one '#', unlike `trim_start_matches`.
        let hex = s.strip_prefix('#').unwrap_or(s);

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
    /// Creates a color from sRGB-encoded (gamma-corrected) floats, clamping each
    /// channel to `[0.0, 1.0]`. `NaN` becomes `0.0`.
    ///
    /// For linear-light input use [`Color::from_linear`].
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: sat01(r),
            g: sat01(g),
            b: sat01(b),
        }
    }

    /// Creates a color from 8-bit sRGB components.
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

    /// Creates a color from linear-light RGB floats (clamped to `[0.0, 1.0]`),
    /// encoding them to sRGB.
    pub fn from_linear(r: f32, g: f32, b: f32) -> Self {
        Self::from_linear64(r as f64, g as f64, b as f64)
    }

    /// Creates a color from HSV components.
    ///
    /// - `h` – hue in degrees (wrapped to `[0°, 360°)`)
    /// - `s` – saturation in `[0, 100]`
    /// - `v` – value/brightness in `[0, 100]`
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let h = wrap_degrees(h);
        let s = s.clamp(0.0, 100.0) / 100.0;
        let v = v.clamp(0.0, 100.0) / 100.0;

        let channel = |n: f32| {
            let k = (n + h / 60.0).rem_euclid(6.0);
            v - v * s * k.min(4.0 - k).clamp(0.0, 1.0)
        };

        Self::new(channel(5.0), channel(3.0), channel(1.0))
    }

    /// Creates a color from HSL components.
    ///
    /// - `h` – hue in degrees (wrapped to `[0°, 360°)`)
    /// - `s` – saturation in `[0, 100]`
    /// - `l` – lightness in `[0, 100]`
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let h = wrap_degrees(h);
        let s = s.clamp(0.0, 100.0) / 100.0;
        let l = l.clamp(0.0, 100.0) / 100.0;

        let a = s * l.min(1.0 - l);

        let channel = |n: f32| {
            let k = (n + h / 30.0).rem_euclid(12.0);
            l - a * (k - 3.0).min(9.0 - k).clamp(-1.0, 1.0)
        };

        Self::new(channel(0.0), channel(8.0), channel(4.0))
    }

    /// Converts CIE L*a*b* (D65) to a `Color`.
    ///
    /// Colors outside the sRGB gamut are clipped per channel, which can shift
    /// the hue. Use [`Color::from_lab_gamut_mapped`] to preserve lightness and
    /// hue instead.
    pub fn from_lab(l: f32, a: f32, b: f32) -> Self {
        let (r, g, bl) = lab_to_linear(l as f64, a as f64, b as f64);
        Self::from_linear64(r, g, bl)
    }

    /// Converts CIE L*a*b* (D65) to a `Color`. Out-of-gamut colors are mapped
    /// into sRGB by reducing chroma at constant lightness and hue, rather than
    /// by clipping channels.
    pub fn from_lab_gamut_mapped(l: f32, a: f32, b: f32) -> Self {
        let (l, a, b) = (l as f64, a as f64, b as f64);

        if !(l.is_finite() && a.is_finite() && b.is_finite()) {
            return Self::from_lab(l as f32, a as f32, b as f32);
        }
        if l <= 0.0 {
            return Self::new(0.0, 0.0, 0.0);
        }
        if l >= 100.0 {
            return Self::new(1.0, 1.0, 1.0);
        }
        if lab_in_gamut(l, a, b) {
            return Self::from_lab(l as f32, a as f32, b as f32);
        }

        let chroma = a.hypot(b);
        if chroma < 1e-9 {
            return Self::from_lab(l as f32, a as f32, b as f32);
        }
        let (sin_h, cos_h) = (b / chroma, a / chroma);
        let (mut lo, mut hi) = (0.0_f64, chroma);
        for _ in 0..32 {
            let mid = 0.5 * (lo + hi);
            if lab_in_gamut(l, mid * cos_h, mid * sin_h) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        Self::from_lab(l as f32, (lo * cos_h) as f32, (lo * sin_h) as f32)
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
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    /// Returns the color as a lowercase CSS hex string (e.g. `"#1a2b3c"`).
    #[inline]
    pub fn css(&self) -> CssColor {
        CssColor(format!("#{:06x}", self.hex()))
    }

    /// Returns the channels as linear-light `(r, g, b)` in `[0.0, 1.0]`.
    pub fn to_linear(&self) -> (f32, f32, f32) {
        let (r, g, b) = self.linear64();
        (r as f32, g as f32, b as f32)
    }

    fn linear64(&self) -> (f64, f64, f64) {
        (
            srgb_to_linear(self.r as f64),
            srgb_to_linear(self.g as f64),
            srgb_to_linear(self.b as f64),
        )
    }

    fn from_linear64(r: f64, g: f64, b: f64) -> Self {
        Self::new(
            linear_to_srgb(unit64(r)) as f32,
            linear_to_srgb(unit64(g)) as f32,
            linear_to_srgb(unit64(b)) as f32,
        )
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

        wrap_degrees(h)
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

    /// Returns the relative luminance as defined by WCAG 2.x
    /// (Rec. 709 / sRGB coefficients, `Y` in `[0.0, 1.0]`).
    ///
    /// Uses the sRGB threshold `0.04045` (as in WCAG 2.2; WCAG 2.0/2.1 listed
    /// `0.03928`). The results are identical for 8-bit input.
    pub fn luminance(&self) -> f32 {
        let (r, g, b) = self.linear64();
        (0.2126 * r + 0.7152 * g + 0.0722 * b) as f32
    }

    /// Converts a `Color` to CIE L*a*b* (D65 white point).
    pub fn lab(self) -> (f32, f32, f32) {
        let (r, g, b) = self.linear64();

        let x = (0.4124564 * r + 0.3575761 * g + 0.1804375 * b) / D65_XN;
        let y = (0.2126729 * r + 0.7151522 * g + 0.0721750 * b) / D65_YN;
        let z = (0.0193339 * r + 0.1191920 * g + 0.9503041 * b) / D65_ZN;

        let (fx, fy, fz) = (lab_f(x), lab_f(y), lab_f(z));
        (
            (116.0 * fy - 16.0) as f32,
            (500.0 * (fx - fy)) as f32,
            (200.0 * (fy - fz)) as f32,
        )
    }

    /// Linearly interpolates between `self` and `other` in **gamma-encoded
    /// sRGB** (the usual CSS/UI gradient behaviour).
    ///
    /// `f = 0.0` returns `self`; `f = 1.0` returns `other`. The result is
    /// clamped to the sRGB gamut. See [`Color::blend_linear`] and
    /// [`Color::blend_lab`] for physically / perceptually based mixing.
    pub fn blend(&self, other: &Color, f: f32) -> Color {
        Color::new(
            self.r + (other.r - self.r) * f,
            self.g + (other.g - self.g) * f,
            self.b + (other.b - self.b) * f,
        )
    }

    /// Linearly interpolates between `self` and `other` in **linear light**
    /// (physically correct mixing of light: 50% black + white gives 50%
    /// linear luminance, i.e. `#bcbcbc`, not `#808080`).
    ///
    /// `f = 0.0` returns `self`; `f = 1.0` returns `other`.
    pub fn blend_linear(&self, other: &Color, f: f32) -> Color {
        let (r1, g1, b1) = self.linear64();
        let (r2, g2, b2) = other.linear64();
        let f = f as f64;
        Color::from_linear64(r1 + (r2 - r1) * f, g1 + (g2 - g1) * f, b1 + (b2 - b1) * f)
    }

    /// Linearly interpolates between `self` and `other` in **CIE L*a*b***
    /// (perceptually more uniform). Results outside the sRGB gamut are mapped
    /// in via chroma reduction ([`Color::from_lab_gamut_mapped`]).
    ///
    /// `f = 0.0` returns `self`; `f = 1.0` returns `other`.
    pub fn blend_lab(&self, other: &Color, f: f32) -> Color {
        let (l1, a1, b1) = self.lab();
        let (l2, a2, b2) = other.lab();
        Color::from_lab_gamut_mapped(l1 + (l2 - l1) * f, a1 + (a2 - a1) * f, b1 + (b2 - b1) * f)
    }

    /// Lightens (`f > 0`) or darkens (`f < 0`) the color by blending toward
    /// white or black (in gamma-encoded sRGB) by the proportion `|f|` in
    /// `[0.0, 1.0]`.
    pub fn shade(&self, f: f32) -> Color {
        let target = if f >= 0.0 { 1.0 } else { 0.0 };
        let p = f.abs();
        Color::new(
            self.r + (target - self.r) * p,
            self.g + (target - self.g) * p,
            self.b + (target - self.b) * p,
        )
    }

    /// Adjusts HSV *value* by `v` percentage points (result clamped to `[0, 100]`).
    pub fn brighten(&self, v: f32) -> Color {
        let (h, s, val) = self.hsv();
        Color::from_hsv(h, s, (val + v).clamp(0.0, 100.0))
    }

    /// Adjusts HSL *lightness* by `v` percentage points (result clamped to `[0, 100]`).
    pub fn lighten(&self, v: f32) -> Color {
        let (h, s, l) = self.hsl();
        Color::from_hsl(h, s, (l + v).clamp(0.0, 100.0))
    }

    /// Adjusts HSV *saturation* by `v` percentage points (result clamped to `[0, 100]`).
    pub fn saturate(&self, v: f32) -> Color {
        let (h, s, val) = self.hsv();
        Color::from_hsv(h, (s + v).clamp(0.0, 100.0), val)
    }

    /// Rotates the hue by `degrees` (any value; wrapped to `[0°, 360°)`).
    pub fn rotate_hue(&self, degrees: f32) -> Color {
        let (h, s, v) = self.hsv();
        Color::from_hsv(h + degrees, s, v)
    }

    /// Rotates the hue by `v` degrees.
    ///
    /// The `rhs` argument is ignored and kept only for API compatibility: the
    /// hue is always wrapped to `[0°, 360°)`. Previously any `rhs != 360.0`
    /// produced a wrong color (and `0.0` produced NaN). Prefer
    /// [`Color::rotate_hue`].
    pub fn rotate(&self, v: f32, _rhs: f32) -> Color {
        self.rotate_hue(v)
    }

    /// WCAG 2.x contrast ratio between two colors, in `[1.0, 21.0]`.
    pub fn wcag_contrast_ratio(&self, other: &Color) -> f32 {
        let l1 = self.luminance();
        let l2 = other.luminance();

        let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };

        (lighter + 0.05) / (darker + 0.05)
    }
}

/// Clamps to `[0.0, 1.0]`, mapping `NaN` to `0.0`.
#[inline(always)]
fn sat01(v: f32) -> f32 {
    if v.is_nan() { 0.0 } else { v.clamp(0.0, 1.0) }
}

#[inline(always)]
fn unit64(v: f64) -> f64 {
    if v.is_nan() { 0.0 } else { v.clamp(0.0, 1.0) }
}

/// Wraps an angle to `[0.0, 360.0)`. `rem_euclid` can return exactly `360.0`
/// for tiny negative inputs due to rounding; that case is folded to `0.0`.
#[inline(always)]
fn wrap_degrees(h: f32) -> f32 {
    let w = h.rem_euclid(360.0);
    if w >= 360.0 { 0.0 } else { w }
}

#[inline(always)]
fn to_u8(v: f32) -> u8 {
    (v * 255.0).round() as u8
}

/// A validated CSS hex color string (e.g. `#1a2b3c`).
///
/// The inner `String` is always a lowercase 7-character string of the form `#rrggbb`.
/// It can only be obtained via [`Color::css`], [`FromStr`](std::str::FromStr),
/// [`TryFrom<String>`] or deserialization (which goes through `TryFrom<String>`),
/// all of which enforce the invariant.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct CssColor(String);

impl Default for CssColor {
    fn default() -> Self {
        Self("#000000".into())
    }
}

impl CssColor {
    pub fn color(&self) -> Color {
        self.as_str()
            .parse()
            .expect("CssColor invariant guarantees validity")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<CssColor> for String {
    fn from(c: CssColor) -> String {
        c.0
    }
}

impl TryFrom<String> for CssColor {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        s.parse()
    }
}

impl std::str::FromStr for CssColor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.parse::<Color>().map(|c| c.css())
    }
}

impl std::fmt::Display for CssColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// D65 reference white. These are the row sums of the forward sRGB→XYZ matrix
// in `Color::lab` (Lindbloom's D65 matrix), so RGB (1, 1, 1) maps to exactly
// L*a*b* (100, 0, 0). The inverse matrix in `lab_to_linear` is the exact
// inverse of that forward matrix; with the commonly printed 7-digit inverse,
// neutral greys came back with channels differing by one 8-bit level.
const D65_XN: f64 = 0.95047;
const D65_YN: f64 = 1.0000001;
const D65_ZN: f64 = 1.08883;

// Exact CIE constants: ε = (6/29)³, κ = (29/3)³.
const LAB_EPS: f64 = 216.0 / 24389.0;
const LAB_KAPPA: f64 = 24389.0 / 27.0;

fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(c: f64) -> f64 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

fn lab_f(t: f64) -> f64 {
    if t > LAB_EPS {
        t.cbrt()
    } else {
        (LAB_KAPPA * t + 16.0) / 116.0
    }
}

fn lab_f_inv(t: f64) -> f64 {
    let t3 = t * t * t;
    if t3 > LAB_EPS {
        t3
    } else {
        (116.0 * t - 16.0) / LAB_KAPPA
    }
}

/// L*a*b* → unclamped linear sRGB.
fn lab_to_linear(l: f64, a: f64, b: f64) -> (f64, f64, f64) {
    let fy = (l + 16.0) / 116.0;
    let fx = a / 500.0 + fy;
    let fz = fy - b / 200.0;

    let x = D65_XN * lab_f_inv(fx);
    let y = D65_YN * lab_f_inv(fy);
    let z = D65_ZN * lab_f_inv(fz);

    (
        3.2404548360214083 * x - 1.5371388501025751 * y - 0.49853154686848089 * z,
        -0.96926638987565372 * x + 1.8760109288424913 * y + 0.041556082346673524 * z,
        0.055643419604213658 * x - 0.20402585426769815 * y + 1.0572251624579287 * z,
    )
}

fn lab_in_gamut(l: f64, a: f64, b: f64) -> bool {
    const EPS: f64 = 1e-6;
    let (r, g, bl) = lab_to_linear(l, a, b);
    [r, g, bl].iter().all(|&c| (-EPS..=1.0 + EPS).contains(&c))
}

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

    use std::io::Write;
    let _ = std::io::stdout().flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32, tol: f32) -> bool {
        (a - b).abs() <= tol
    }

    #[test]
    fn lab_reference_values() {
        let (l, a, b) = Color::from_hex(0xff0000).lab();
        assert!(close(l, 53.2408, 0.01) && close(a, 80.0925, 0.01) && close(b, 67.2032, 0.01));
        let (l, a, b) = Color::from_hex(0xffffff).lab();
        assert!(close(l, 100.0, 1e-3) && close(a, 0.0, 1e-3) && close(b, 0.0, 1e-3));
        let (l, a, b) = Color::from_hex(0x000000).lab();
        assert!(close(l, 0.0, 1e-4) && close(a, 0.0, 1e-4) && close(b, 0.0, 1e-4));
    }

    #[test]
    fn lab_roundtrip_8bit() {
        for r in (0..=255u16).step_by(5).chain([255]) {
            for g in (0..=255u16).step_by(5).chain([255]) {
                for b in (0..=255u16).step_by(5).chain([255]) {
                    let c = Color::from_rgb(r as u8, g as u8, b as u8);
                    let (l, a, bb) = c.lab();
                    assert_eq!(Color::from_lab(l, a, bb).rgb(), c.rgb());
                }
            }
        }
    }

    #[test]
    fn hsv_hsl_roundtrip_8bit() {
        for r in (0..=255u16).step_by(5).chain([255]) {
            for g in (0..=255u16).step_by(5).chain([255]) {
                for b in (0..=255u16).step_by(5).chain([255]) {
                    let c = Color::from_rgb(r as u8, g as u8, b as u8);
                    let (h, s, v) = c.hsv();
                    assert!(h >= 0.0 && h < 360.0);
                    assert_eq!(Color::from_hsv(h, s, v).rgb(), c.rgb());
                    let (h, s, l) = c.hsl();
                    assert!(h >= 0.0 && h < 360.0);
                    assert_eq!(Color::from_hsl(h, s, l).rgb(), c.rgb());
                }
            }
        }
    }

    #[test]
    fn hue_never_360() {
        assert_eq!(wrap_degrees(-1e-9), 0.0);
        assert_eq!(wrap_degrees(360.0), 0.0);
        assert_eq!(wrap_degrees(-90.0), 270.0);
    }

    #[test]
    fn rotate() {
        let red = Color::from_hex(0xff0000);
        assert_eq!(red.rotate_hue(120.0).hex(), 0x00ff00);
        assert_eq!(red.rotate_hue(-120.0).hex(), 0x0000ff);
        assert_eq!(red.rotate(120.0, 0.0).hex(), 0x00ff00);
    }

    #[test]
    fn nan_is_sanitised() {
        let c = Color::new(f32::NAN, 2.0, -1.0);
        assert_eq!(c, c);
        assert_eq!(c.hex(), 0x00ff00);
    }

    #[test]
    fn parsing() {
        assert!("##ffffff".parse::<Color>().is_err());
        assert!("#fff".parse::<Color>().is_err());
        assert!("#gggggg".parse::<Color>().is_err());
        assert_eq!("#FFaa00".parse::<Color>().unwrap().hex(), 0xffaa00);
        assert_eq!("ffaa00".parse::<CssColor>().unwrap().as_str(), "#ffaa00");
        assert!(CssColor::try_from("abc".to_string()).is_err());
    }

    #[test]
    fn wcag() {
        let (w, k) = (Color::from_hex(0xffffff), Color::from_hex(0));
        assert!(close(w.luminance(), 1.0, 1e-5));
        assert!(close(w.wcag_contrast_ratio(&k), 21.0, 1e-3));
        assert!(close(k.wcag_contrast_ratio(&w), 21.0, 1e-3));
    }

    #[test]
    fn blends() {
        let (w, k) = (Color::from_hex(0xffffff), Color::from_hex(0));
        assert_eq!(k.blend(&w, 0.5).hex(), 0x808080);
        assert_eq!(k.blend_linear(&w, 0.5).hex(), 0xbcbcbc);
        assert_eq!(k.blend_lab(&w, 0.5).hex(), 0x777777);
        let (a, b) = (Color::from_hex(0x123456), Color::from_hex(0xfedcba));
        for f in [0.0, 1.0] {
            let t = if f == 0.0 { a } else { b };
            assert_eq!(a.blend(&b, f).hex(), t.hex());
            assert_eq!(a.blend_linear(&b, f).hex(), t.hex());
            assert_eq!(a.blend_lab(&b, f).hex(), t.hex());
        }
    }

    #[test]
    fn neutral_greys_stay_neutral() {
        for n in 0..=255u8 {
            let (_, a, b) = Color::from_rgb(n, n, n).lab();
            assert!(a.abs() < 1e-3 && b.abs() < 1e-3, "{n}: {a} {b}");
        }
        let mut l = 0.0001_f32;
        while l < 100.0 {
            for m in [
                Color::from_lab(l, 0.0, 0.0),
                Color::from_lab_gamut_mapped(l, 0.0, 0.0),
            ] {
                let (r, g, b) = m.rgb();
                assert!(r == g && g == b, "L={l} -> {:?}", m.rgb());
            }
            l += 0.0173;
        }
    }

    #[test]
    fn known_hsl_hsv_values() {
        assert_eq!(Color::from_hsl(210.0, 50.0, 40.0).hex(), 0x336699);
        assert_eq!(Color::from_hsv(210.0, 200.0 / 3.0, 60.0).hex(), 0x336699);
        let (h, s, v) = Color::from_hex(0x336699).hsv();
        assert!(close(h, 210.0, 1e-3) && close(s, 66.6667, 1e-2) && close(v, 60.0, 1e-3));
        let (h, s, l) = Color::from_hex(0x336699).hsl();
        assert!(close(h, 210.0, 1e-3) && close(s, 50.0, 1e-2) && close(l, 40.0, 1e-3));
        assert_eq!(Color::from_hsv(-60.0, 100.0, 100.0).hex(), 0xff00ff);
        assert_eq!(Color::from_hsv(840.0, 100.0, 100.0).hex(), 0x00ff00);
    }

    #[test]
    fn lab_green_blue_reference() {
        let (l, a, b) = Color::from_hex(0x00ff00).lab();
        assert!(close(l, 87.7347, 0.02) && close(a, -86.1827, 0.03) && close(b, 83.1793, 0.03));
        let (l, a, b) = Color::from_hex(0x0000ff).lab();
        assert!(close(l, 32.3026, 0.02) && close(a, 79.1967, 0.03) && close(b, -107.8637, 0.03));
    }

    #[test]
    fn gamut_mapping_keeps_lightness_and_hue() {
        let (l, a, b) = (50.0_f32, 100.0_f32, 100.0_f32);
        let m = Color::from_lab_gamut_mapped(l, a, b);
        let (ml, ma, mb) = m.lab();
        assert!(close(ml, l, 1.0));
        let (h0, h1) = (b.atan2(a).to_degrees(), mb.atan2(ma).to_degrees());
        assert!(close(h0, h1, 1.5));
        assert!(ma.hypot(mb) < a.hypot(b));
    }
}
