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
            r: clamp_unit(r),
            g: clamp_unit(g),
            b: clamp_unit(b),
        }
    }

    /// Creates a color from 8-bit sRGB components.
    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    /// Creates a color from a packed 24-bit hex value (`0xRRGGBB`).
    /// The upper 8 bits are ignored.
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
    /// - `v` – value (brightness) in `[0, 100]`
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

        let amplitude = s * l.min(1.0 - l);

        let channel = |n: f32| {
            let k = (n + h / 30.0).rem_euclid(12.0);
            l - amplitude * (k - 3.0).min(9.0 - k).clamp(-1.0, 1.0)
        };

        Self::new(channel(0.0), channel(8.0), channel(4.0))
    }

    /// Creates a color from CIE L*a*b* (D65).
    ///
    /// Colors outside the sRGB gamut are clipped per channel, which can shift
    /// the hue. Use [`Color::from_lab_gamut_mapped`] to preserve lightness and
    /// hue instead.
    pub fn from_lab(l: f32, a: f32, b: f32) -> Self {
        Self::from_lab64(l as f64, a as f64, b as f64)
    }

    /// Creates a color from CIE L*a*b* (D65). Out-of-gamut colors are mapped
    /// into sRGB by reducing chroma at constant lightness and hue, rather than
    /// by clipping channels.
    pub fn from_lab_gamut_mapped(l: f32, a: f32, b: f32) -> Self {
        Self::from_lab64_gamut_mapped(l as f64, a as f64, b as f64)
    }

    /// Creates a color from raw bytes.
    ///
    /// Returns an error if the slice is not a valid encoding
    /// (see the `TryFrom<&[u8]>` implementation).
    #[inline]
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self> {
        Self::try_from(bytes)
    }

    /// Parses a CSS hex color string (`#rrggbb` or `rrggbb`).
    #[inline]
    pub fn try_from_css(css: &str) -> Result<Self> {
        css.parse()
    }

    /// Returns the color as 8-bit sRGB `(r, g, b)`.
    #[inline]
    pub fn rgb(&self) -> (u8, u8, u8) {
        (
            unit_to_byte(self.r),
            unit_to_byte(self.g),
            unit_to_byte(self.b),
        )
    }

    /// Returns the color as 8-bit sRGB `[r, g, b]`.
    #[inline]
    pub fn bytes(&self) -> [u8; 3] {
        let (r, g, b) = self.rgb();
        [r, g, b]
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
    pub fn linear(&self) -> (f32, f32, f32) {
        let (r, g, b) = self.linear64();
        (r as f32, g as f32, b as f32)
    }

    /// Returns `(hue °, saturation %, value %)`; hue in `[0°, 360°)`.
    pub fn hsv(&self) -> (f32, f32, f32) {
        let (hue, min, max) = self.hue_and_extremes();
        let saturation = if max == 0.0 { 0.0 } else { (max - min) / max };
        (hue, saturation * 100.0, max * 100.0)
    }

    /// Returns `(hue °, saturation %, lightness %)`; hue in `[0°, 360°)`.
    pub fn hsl(&self) -> (f32, f32, f32) {
        let (hue, min, max) = self.hue_and_extremes();
        let lightness = (max + min) / 2.0;
        let delta = max - min;
        let saturation = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * lightness - 1.0).abs())
        };
        (hue, saturation * 100.0, lightness * 100.0)
    }

    /// Returns CIE L*a*b* (D65 white point).
    pub fn lab(&self) -> (f32, f32, f32) {
        let (l, a, b) = self.lab64();
        (l as f32, a as f32, b as f32)
    }

    /// Returns CIE LCh(ab) as `(L, C, h°)`, with `h` in `[0°, 360°)`.
    /// Approximate hues: red ≈ 40°, yellow ≈ 102°, green ≈ 136°, blue ≈ 306°.
    /// Hue is `0.0` for exact neutrals.
    pub fn lch(&self) -> (f32, f32, f32) {
        let (l, a, b) = self.lab64();
        let chroma = a.hypot(b);
        let hue = if chroma < 1e-9 {
            0.0
        } else {
            wrap_degrees(b.atan2(a).to_degrees() as f32)
        };
        (l as f32, chroma as f32, hue)
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

    /// WCAG 2.x contrast ratio between two colors, in `[1.0, 21.0]`.
    pub fn wcag_contrast_ratio(&self, other: &Color) -> f32 {
        let (a, b) = (self.luminance(), other.luminance());
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }

    /// Euclidean distance in CIE L*a*b* (ΔE76). Fast but overstates
    /// differences between saturated colors. Prefer [`Color::delta_e2000`].
    pub fn delta_e76(&self, other: &Color) -> f32 {
        let (l1, a1, b1) = self.lab64();
        let (l2, a2, b2) = other.lab64();
        ((l1 - l2).powi(2) + (a1 - a2).powi(2) + (b1 - b2).powi(2)).sqrt() as f32
    }

    /// Perceptual color difference (CIEDE2000). Symmetric.
    /// Rough guide: `< 1` invisible, `1–2` barely visible, `2–10` clearly
    /// visible, `> 10` different colors. Black vs white is `100`.
    pub fn delta_e2000(&self, other: &Color) -> f32 {
        ciede2000(self.lab64(), other.lab64()) as f32
    }

    /// Overall color similarity in `(0.0, 1.0]`; `1.0` means identical.
    ///
    /// `half_similarity_delta_e` is the ΔE2000 at which the result is exactly
    /// `0.5` (try `15.0`; smaller = stricter).
    pub fn similarity(&self, other: &Color, half_similarity_delta_e: f32) -> f32 {
        let d = self.delta_e2000(other) / half_similarity_delta_e.max(f32::EPSILON);
        1.0 / (1.0 + d * d)
    }

    /// Shortest angular distance between Lab hues, in degrees `[0, 180]`.
    /// Meaningless for near-neutral colors (see [`Color::hue_similarity`]).
    pub fn hue_distance(&self, other: &Color) -> f32 {
        angular_distance(self.lch().2, other.lch().2)
    }

    /// How close this color's *hue* is to `other`'s, in `[0.0, 1.0]`,
    /// ignoring lightness. Use it to ask "how red / green is this color?":
    /// `c.hue_similarity(&Color::from_rgb(255, 0, 0), 30.0)`.
    ///
    /// - `half_similarity_degrees`: hue difference at which the score halves
    ///   (try `25–40`).
    /// - Grays score near `0.0` because they have no hue; both colors must
    ///   be chromatic, so pass a saturated `other`.
    pub fn hue_similarity(&self, other: &Color, half_similarity_degrees: f32) -> f32 {
        // Chroma at which a color counts as "clearly chromatic"
        // (weight ≈ 0.63). Neutrals (C ≈ 0) have no meaningful hue.
        const CHROMA_SCALE: f32 = 20.0;

        let (_, chroma_a, hue_a) = self.lch();
        let (_, chroma_b, hue_b) = other.lch();

        let d = angular_distance(hue_a, hue_b) / half_similarity_degrees.max(f32::EPSILON);
        let hue_score = 1.0 / (1.0 + d * d);

        let chromatic_weight = |chroma: f32| 1.0 - (-chroma / CHROMA_SCALE).exp();
        hue_score * chromatic_weight(chroma_a).min(chromatic_weight(chroma_b))
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
        let (l1, a1, b1) = self.lab64();
        let (l2, a2, b2) = other.lab64();
        let f = f as f64;
        Color::from_lab64_gamut_mapped(l1 + (l2 - l1) * f, a1 + (a2 - a1) * f, b1 + (b2 - b1) * f)
    }

    /// Lightens (`amount > 0`) or darkens (`amount < 0`) the color by blending
    /// toward white or black (in gamma-encoded sRGB) by the proportion
    /// `|amount|` in `[0.0, 1.0]`.
    pub fn shade(&self, amount: f32) -> Color {
        let target = if amount >= 0.0 { 1.0 } else { 0.0 };
        self.blend(&Color::new(target, target, target), amount.abs())
    }

    /// Adjusts HSV *value* by `delta` percentage points (result clamped to `[0, 100]`).
    pub fn adjust_value(&self, delta: f32) -> Color {
        let (h, s, v) = self.hsv();
        Color::from_hsv(h, s, v + delta)
    }

    /// Adjusts HSV *saturation* by `delta` percentage points (result clamped to `[0, 100]`).
    pub fn adjust_saturation(&self, delta: f32) -> Color {
        let (h, s, v) = self.hsv();
        Color::from_hsv(h, s + delta, v)
    }

    /// Adjusts HSL *lightness* by `delta` percentage points (result clamped to `[0, 100]`).
    pub fn adjust_lightness(&self, delta: f32) -> Color {
        let (h, s, l) = self.hsl();
        Color::from_hsl(h, s, l + delta)
    }

    /// Rotates the hue by `degrees` (any value; wrapped to `[0°, 360°)`).
    pub fn rotate_hue(&self, degrees: f32) -> Color {
        let (h, s, v) = self.hsv();
        Color::from_hsv(h + degrees, s, v)
    }

    #[deprecated(note = "use `adjust_value`")]
    pub fn brighten(&self, v: f32) -> Color {
        self.adjust_value(v)
    }

    #[deprecated(note = "use `adjust_saturation`")]
    pub fn saturate(&self, v: f32) -> Color {
        self.adjust_saturation(v)
    }

    #[deprecated(note = "use `adjust_lightness`")]
    pub fn lighten(&self, v: f32) -> Color {
        self.adjust_lightness(v)
    }
}

impl Color {
    /// Channels as linear-light `f64`.
    fn linear64(&self) -> (f64, f64, f64) {
        (
            srgb_to_linear(self.r as f64),
            srgb_to_linear(self.g as f64),
            srgb_to_linear(self.b as f64),
        )
    }

    /// Creates a color from linear-light `f64`. Out-of-range and `NaN` values
    /// are handled by the clamping in [`Color::new`].
    fn from_linear64(r: f64, g: f64, b: f64) -> Self {
        Self::new(
            linear_to_srgb(r) as f32,
            linear_to_srgb(g) as f32,
            linear_to_srgb(b) as f32,
        )
    }

    /// CIE L*a*b* (D65) in `f64`; avoids `f32` rounding.
    fn lab64(&self) -> (f64, f64, f64) {
        let (r, g, b) = self.linear64();

        let x = (0.4124564 * r + 0.3575761 * g + 0.1804375 * b) / D65_WHITE_X;
        let y = (0.2126729 * r + 0.7151522 * g + 0.0721750 * b) / D65_WHITE_Y;
        let z = (0.0193339 * r + 0.1191920 * g + 0.9503041 * b) / D65_WHITE_Z;

        let (fx, fy, fz) = (
            lab_nonlinearity(x),
            lab_nonlinearity(y),
            lab_nonlinearity(z),
        );
        (116.0 * fy - 16.0, 500.0 * (fx - fy), 200.0 * (fy - fz))
    }

    /// CIE L*a*b* (D65) → color, clipping out-of-gamut channels.
    fn from_lab64(l: f64, a: f64, b: f64) -> Self {
        let (r, g, bl) = lab_to_linear(l, a, b);
        Self::from_linear64(r, g, bl)
    }

    /// CIE L*a*b* (D65) → color, mapping out-of-gamut values into sRGB by
    /// bisecting chroma at constant lightness and hue.
    fn from_lab64_gamut_mapped(l: f64, a: f64, b: f64) -> Self {
        if !(l.is_finite() && a.is_finite() && b.is_finite()) {
            return Self::from_lab64(l, a, b);
        }
        if l <= 0.0 {
            return Self::new(0.0, 0.0, 0.0);
        }
        if l >= 100.0 {
            return Self::new(1.0, 1.0, 1.0);
        }
        if lab_in_gamut(l, a, b) {
            return Self::from_lab64(l, a, b);
        }

        // Here 0 < L < 100 and the color is out of gamut, so chroma > 0
        // (the neutral axis is always inside the gamut).
        let chroma = a.hypot(b);
        let (cos_h, sin_h) = (a / chroma, b / chroma);

        // 32 halvings narrow the chroma range below 1e-7 (max chroma ≈ 230).
        let (mut inside, mut outside) = (0.0_f64, chroma);
        for _ in 0..32 {
            let mid = 0.5 * (inside + outside);
            if lab_in_gamut(l, mid * cos_h, mid * sin_h) {
                inside = mid;
            } else {
                outside = mid;
            }
        }
        Self::from_lab64(l, inside * cos_h, inside * sin_h)
    }

    /// Returns `(hue °, min channel, max channel)`. The hue is the HSV/HSL hue
    /// (`0.0` for grays), wrapped to `[0°, 360°)`.
    fn hue_and_extremes(&self) -> (f32, f32, f32) {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max - min;

        let hue = if delta == 0.0 {
            0.0
        } else if max == self.r {
            60.0 * (self.g - self.b) / delta
        } else if max == self.g {
            60.0 * ((self.b - self.r) / delta + 2.0)
        } else {
            60.0 * ((self.r - self.g) / delta + 4.0)
        };

        (wrap_degrees(hue), min, max)
    }
}

/// Clamps to `[0.0, 1.0]`, mapping `NaN` to `0.0`.
#[inline(always)]
fn clamp_unit(v: f32) -> f32 {
    if v.is_nan() { 0.0 } else { v.clamp(0.0, 1.0) }
}

/// Converts a `[0.0, 1.0]` channel to the nearest 8-bit value.
#[inline(always)]
fn unit_to_byte(v: f32) -> u8 {
    (v * 255.0).round() as u8
}

/// Wraps an angle to `[0.0, 360.0)`. `rem_euclid` can return exactly `360.0`
/// for tiny negative inputs due to rounding; that case is folded to `0.0`.
#[inline(always)]
fn wrap_degrees(degrees: f32) -> f32 {
    let wrapped = degrees.rem_euclid(360.0);
    if wrapped >= 360.0 { 0.0 } else { wrapped }
}

/// Shortest distance between two angles in degrees, in `[0, 180]`.
#[inline(always)]
fn angular_distance(a: f32, b: f32) -> f32 {
    let d = (a - b).abs();
    d.min(360.0 - d)
}

fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Inverse of [`srgb_to_linear`]. Not clamped: values below `0.0` or above
/// `1.0` map outside `[0.0, 1.0]` and are clamped later by [`Color::new`].
fn linear_to_srgb(c: f64) -> f64 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

// D65 reference white. These are the row sums of the forward sRGB→XYZ matrix
// in `Color::lab64` (Lindbloom's D65 matrix), so RGB (1, 1, 1) maps to exactly
// L*a*b* (100, 0, 0). The inverse matrix in `lab_to_linear` is the exact
// inverse of that forward matrix; with the commonly printed 7-digit inverse,
// neutral greys came back with channels differing by one 8-bit level.
const D65_WHITE_X: f64 = 0.95047;
const D65_WHITE_Y: f64 = 1.0000001;
const D65_WHITE_Z: f64 = 1.08883;

// Exact CIE constants: ε = (6/29)³, κ = (29/3)³.
const LAB_EPSILON: f64 = 216.0 / 24389.0;
const LAB_KAPPA: f64 = 24389.0 / 27.0;

/// CIE L*a*b* companding function `f(t)` applied to a white-normalized XYZ
/// component.
fn lab_nonlinearity(t: f64) -> f64 {
    if t > LAB_EPSILON {
        t.cbrt()
    } else {
        (LAB_KAPPA * t + 16.0) / 116.0
    }
}

/// Inverse of [`lab_nonlinearity`].
fn lab_nonlinearity_inverse(t: f64) -> f64 {
    let cubed = t * t * t;
    if cubed > LAB_EPSILON {
        cubed
    } else {
        (116.0 * t - 16.0) / LAB_KAPPA
    }
}

/// L*a*b* → unclamped linear sRGB.
fn lab_to_linear(l: f64, a: f64, b: f64) -> (f64, f64, f64) {
    let fy = (l + 16.0) / 116.0;
    let fx = a / 500.0 + fy;
    let fz = fy - b / 200.0;

    let x = D65_WHITE_X * lab_nonlinearity_inverse(fx);
    let y = D65_WHITE_Y * lab_nonlinearity_inverse(fy);
    let z = D65_WHITE_Z * lab_nonlinearity_inverse(fz);

    (
        3.2404548360214083 * x - 1.5371388501025751 * y - 0.49853154686848089 * z,
        -0.96926638987565372 * x + 1.8760109288424913 * y + 0.041556082346673524 * z,
        0.055643419604213658 * x - 0.20402585426769815 * y + 1.0572251624579287 * z,
    )
}

/// Whether L*a*b* lies inside the sRGB gamut (with a small tolerance for
/// floating-point error on the gamut boundary).
fn lab_in_gamut(l: f64, a: f64, b: f64) -> bool {
    const TOLERANCE: f64 = 1e-6;
    let (r, g, bl) = lab_to_linear(l, a, b);
    [r, g, bl]
        .iter()
        .all(|c| (-TOLERANCE..=1.0 + TOLERANCE).contains(c))
}

/// CIEDE2000 (kL = kC = kH = 1) between two CIE L*a*b* colors.
///
/// Reference: Sharma, Wu, Dalal (2005). Variable names follow the paper:
/// a trailing `p` is the prime (′), `bar` is the mean of the pair.
fn ciede2000((l1, a1, b1): (f64, f64, f64), (l2, a2, b2): (f64, f64, f64)) -> f64 {
    use std::f64::consts::PI;
    let pow25_7 = 25f64.powi(7);

    // Chroma-dependent a* correction (G).
    let c_bar7 = (0.5 * (a1.hypot(b1) + a2.hypot(b2))).powi(7);
    let g = 0.5 * (1.0 - (c_bar7 / (c_bar7 + pow25_7)).sqrt());

    let (a1p, a2p) = ((1.0 + g) * a1, (1.0 + g) * a2);
    let (c1p, c2p) = (a1p.hypot(b1), a2p.hypot(b2));

    let hue = |b: f64, ap: f64| {
        if b == 0.0 && ap == 0.0 {
            0.0
        } else {
            b.atan2(ap).to_degrees().rem_euclid(360.0)
        }
    };
    let (h1p, h2p) = (hue(b1, a1p), hue(b2, a2p));

    // Differences ΔL′, ΔC′, ΔH′.
    let dl = l2 - l1;
    let dc = c2p - c1p;
    let achromatic = c1p * c2p == 0.0;

    let dh_degrees = if achromatic {
        0.0
    } else {
        let d = h2p - h1p;
        if d > 180.0 {
            d - 360.0
        } else if d < -180.0 {
            d + 360.0
        } else {
            d
        }
    };
    let dh = 2.0 * (c1p * c2p).sqrt() * (dh_degrees.to_radians() / 2.0).sin();

    // Means L̄′, C̄′, H̄′.
    let l_bar = 0.5 * (l1 + l2);
    let c_barp = 0.5 * (c1p + c2p);
    let h_barp = if achromatic {
        h1p + h2p
    } else if (h1p - h2p).abs() <= 180.0 {
        0.5 * (h1p + h2p)
    } else if h1p + h2p < 360.0 {
        0.5 * (h1p + h2p + 360.0)
    } else {
        0.5 * (h1p + h2p - 360.0)
    };

    // Weighting functions and rotation term.
    let t = 1.0 - 0.17 * (h_barp - 30.0).to_radians().cos()
        + 0.24 * (2.0 * h_barp).to_radians().cos()
        + 0.32 * (3.0 * h_barp + 6.0).to_radians().cos()
        - 0.20 * (4.0 * h_barp - 63.0).to_radians().cos();

    let delta_theta = 30.0 * (-((h_barp - 275.0) / 25.0).powi(2)).exp();
    let c_barp7 = c_barp.powi(7);
    let rc = 2.0 * (c_barp7 / (c_barp7 + pow25_7)).sqrt();
    let l50 = (l_bar - 50.0).powi(2);
    let sl = 1.0 + 0.015 * l50 / (20.0 + l50).sqrt();
    let sc = 1.0 + 0.045 * c_barp;
    let sh = 1.0 + 0.015 * c_barp * t;
    let rt = -(2.0 * delta_theta * PI / 180.0).sin() * rc;

    let (tl, tc, th) = (dl / sl, dc / sc, dh / sh);
    (tl * tl + tc * tc + th * th + rt * tc * th).max(0.0).sqrt()
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
    //     pub const ALICE_BLUE: &str = "#f0f8ff";
    //     pub const ANTIQUE_WHITE: &str = "#faebd7";
    //     pub const AQUA: &str = "#00ffff";
    //     pub const AQUAMARINE: &str = "#7fffd4";
    //     pub const AZURE: &str = "#f0ffff";
    //     pub const BEIGE: &str = "#f5f5dc";
    //     pub const BISQUE: &str = "#ffe4c4";
    //     pub const BLACK: &str = "#000000";
    //     pub const BLANCHED_ALMOND: &str = "#ffebcd";
    //     pub const BLUE: &str = "#0000ff";
    //     pub const BLUE_VIOLET: &str = "#8a2be2";
    //     pub const BROWN: &str = "#a52a2a";
    //     pub const BURLYWOOD: &str = "#deb887";
    //     pub const CADET_BLUE: &str = "#5f9ea0";
    //     pub const CHARTREUSE: &str = "#7fff00";
    //     pub const CHOCOLATE: &str = "#d2691e";
    //     pub const CORAL: &str = "#ff7f50";
    //     pub const CORNFLOWER_BLUE: &str = "#6495ed";
    //     pub const CORNSILK: &str = "#fff8dc";
    //     pub const CRIMSON: &str = "#dc143c";
    //     pub const CYAN: &str = "#00ffff";
    //     pub const DARK_BLUE: &str = "#00008b";
    //     pub const DARK_CYAN: &str = "#008b8b";
    //     pub const DARK_GOLDENROD: &str = "#b8860b";
    //     pub const DARK_GRAY: &str = "#a9a9a9";
    //     pub const DARK_GREEN: &str = "#006400";
    //     pub const DARK_KHAKI: &str = "#bdb76b";
    //     pub const DARK_MAGENTA: &str = "#8b008b";
    //     pub const DARK_OLIVE_GREEN: &str = "#556b2f";
    //     pub const DARK_ORANGE: &str = "#ff8c00";
    //     pub const DARK_ORCHID: &str = "#9932cc";
    //     pub const DARK_RED: &str = "#8b0000";
    //     pub const DARK_SALMON: &str = "#e9967a";
    //     pub const DARK_SEA_GREEN: &str = "#8fbc8f";
    //     pub const DARK_SLATE_BLUE: &str = "#483d8b";
    //     pub const DARK_SLATE_GRAY: &str = "#2f4f4f";
    //     pub const DARK_TURQUOISE: &str = "#00ced1";
    //     pub const DARK_VIOLET: &str = "#9400d3";
    //     pub const DEEP_PINK: &str = "#ff1493";
    //     pub const DEEP_SKY_BLUE: &str = "#00bfff";
    //     pub const DIM_GRAY: &str = "#696969";
    //     pub const DODGER_BLUE: &str = "#1e90ff";
    //     pub const FIREBRICK: &str = "#b22222";
    //     pub const FLORAL_WHITE: &str = "#fffaf0";
    //     pub const FOREST_GREEN: &str = "#228b22";
    //     pub const FUCHSIA: &str = "#ff00ff";
    //     pub const GAINSBORO: &str = "#dcdcdc";
    //     pub const GHOST_WHITE: &str = "#f8f8ff";
    //     pub const GOLD: &str = "#ffd700";
    //     pub const GOLDENROD: &str = "#daa520";
    //     pub const GRAY: &str = "#808080";
    //     pub const GREEN: &str = "#008000";
    //     pub const GREEN_YELLOW: &str = "#adff2f";
    //     pub const HONEYDEW: &str = "#f0fff0";
    //     pub const HOT_PINK: &str = "#ff69b4";
    //     pub const INDIAN_RED: &str = "#cd5c5c";
    //     pub const INDIGO: &str = "#4b0082";
    //     pub const IVORY: &str = "#fffff0";
    //     pub const KHAKI: &str = "#f0e68c";
    //     pub const LAVENDER: &str = "#e6e6fa";
    //     pub const LAVENDER_BLUSH: &str = "#fff0f5";
    //     pub const LAWN_GREEN: &str = "#7cfc00";
    //     pub const LEMON_CHIFFON: &str = "#fffacd";
    //     pub const LIGHT_BLUE: &str = "#add8e6";
    //     pub const LIGHT_CORAL: &str = "#f08080";
    //     pub const LIGHT_CYAN: &str = "#e0ffff";
    //     pub const LIGHT_GOLDENROD_YELLOW: &str = "#fafad2";
    //     pub const LIGHT_GRAY: &str = "#d3d3d3";
    //     pub const LIGHT_GREEN: &str = "#90ee90";
    //     pub const LIGHT_PINK: &str = "#ffb6c1";
    //     pub const LIGHT_SALMON: &str = "#ffa07a";
    //     pub const LIGHT_SEA_GREEN: &str = "#20b2aa";
    //     pub const LIGHT_SKY_BLUE: &str = "#87cefa";
    //     pub const LIGHT_SLATE_GRAY: &str = "#778899";
    //     pub const LIGHT_STEEL_BLUE: &str = "#b0c4de";
    //     pub const LIGHT_YELLOW: &str = "#ffffe0";
    //     pub const LIME: &str = "#00ff00";
    //     pub const LIME_GREEN: &str = "#32cd32";
    //     pub const LINEN: &str = "#faf0e6";
    //     pub const MAGENTA: &str = "#ff00ff";
    //     pub const MAROON: &str = "#800000";
    //     pub const MEDIUM_AQUAMARINE: &str = "#66cdaa";
    //     pub const MEDIUM_BLUE: &str = "#0000cd";
    //     pub const MEDIUM_ORCHID: &str = "#ba55d3";
    //     pub const MEDIUM_PURPLE: &str = "#9370db";
    //     pub const MEDIUM_SEA_GREEN: &str = "#3cb371";
    //     pub const MEDIUM_SLATE_BLUE: &str = "#7b68ee";
    //     pub const MEDIUM_SPRING_GREEN: &str = "#00fa9a";
    //     pub const MEDIUM_TURQUOISE: &str = "#48d1cc";
    //     pub const MEDIUM_VIOLET_RED: &str = "#c71585";
    //     pub const MIDNIGHT_BLUE: &str = "#191970";
    //     pub const MINT_CREAM: &str = "#f5fffa";
    //     pub const MISTY_ROSE: &str = "#ffe4e1";
    //     pub const MOCCASIN: &str = "#ffe4b5";
    //     pub const NAVAJO_WHITE: &str = "#ffdead";
    //     pub const NAVY: &str = "#000080";
    //     pub const OLD_LACE: &str = "#fdf5e6";
    //     pub const OLIVE: &str = "#808000";
    //     pub const OLIVE_DRAB: &str = "#6b8e23";
    //     pub const ORANGE: &str = "#ffa500";
    //     pub const ORANGE_RED: &str = "#ff4500";
    //     pub const ORCHID: &str = "#da70d6";
    //     pub const PALE_GOLDENROD: &str = "#eee8aa";
    //     pub const PALE_GREEN: &str = "#98fb98";
    //     pub const PALE_TURQUOISE: &str = "#afeeee";
    //     pub const PALE_VIOLET_RED: &str = "#db7093";
    //     pub const PAPAYA_WHIP: &str = "#ffefd5";
    //     pub const PEACH_PUFF: &str = "#ffdab9";
    //     pub const PERU: &str = "#cd853f";
    //     pub const PINK: &str = "#ffc0cb";
    //     pub const PLUM: &str = "#dda0dd";
    //     pub const POWDER_BLUE: &str = "#b0e0e6";
    //     pub const PURPLE: &str = "#800080";
    //     pub const REBECCA_PURPLE: &str = "#663399";
    //     pub const RED: &str = "#ff0000";
    //     pub const ROSY_BROWN: &str = "#bc8f8f";
    //     pub const ROYAL_BLUE: &str = "#4169e1";
    //     pub const SADDLE_BROWN: &str = "#8b4513";
    //     pub const SALMON: &str = "#fa8072";
    //     pub const SANDY_BROWN: &str = "#f4a460";
    //     pub const SEA_GREEN: &str = "#2e8b57";
    //     pub const SEASHELL: &str = "#fff5ee";
    //     pub const SIENNA: &str = "#a0522d";
    //     pub const SILVER: &str = "#c0c0c0";
    //     pub const SKY_BLUE: &str = "#87ceeb";
    //     pub const SLATE_BLUE: &str = "#6a5acd";
    //     pub const SLATE_GRAY: &str = "#708090";
    //     pub const SNOW: &str = "#fffafa";
    //     pub const SPRING_GREEN: &str = "#00ff7f";
    //     pub const STEEL_BLUE: &str = "#4682b4";
    //     pub const TAN: &str = "#d2b48c";
    //     pub const TEAL: &str = "#008080";
    //     pub const THISTLE: &str = "#d8bfd8";
    //     pub const TOMATO: &str = "#ff6347";
    //     pub const TURQUOISE: &str = "#40e0d0";
    //     pub const VIOLET: &str = "#ee82ee";
    //     pub const WHEAT: &str = "#f5deb3";
    //     pub const WHITE: &str = "#ffffff";
    //     pub const WHITE_SMOKE: &str = "#f5f5f5";
    //     pub const YELLOW: &str = "#ffff00";
    //     pub const YELLOW_GREEN: &str = "#9acd32";
    //
    //     pub fn from_const(value: &'static str) -> Self {
    //         Self(value.to_owned())
    //     }
    //
    //     pub fn from_name(name: &str) -> Option<Self> {
    //         let normalized: String = name
    //             .chars()
    //             .filter(|c| !c.is_whitespace())
    //             .flat_map(char::to_lowercase)
    //             .collect();
    //
    //         let hex: &'static str = match normalized.as_str() {
    //             "aliceblue" => Self::ALICE_BLUE,
    //             "antiquewhite" => Self::ANTIQUE_WHITE,
    //             "aqua" => Self::AQUA,
    //             "aquamarine" => Self::AQUAMARINE,
    //             "azure" => Self::AZURE,
    //             "beige" => Self::BEIGE,
    //             "bisque" => Self::BISQUE,
    //             "black" => Self::BLACK,
    //             "blanchedalmond" => Self::BLANCHED_ALMOND,
    //             "blue" => Self::BLUE,
    //             "blueviolet" => Self::BLUE_VIOLET,
    //             "brown" => Self::BROWN,
    //             "burlywood" => Self::BURLYWOOD,
    //             "cadetblue" => Self::CADET_BLUE,
    //             "chartreuse" => Self::CHARTREUSE,
    //             "chocolate" => Self::CHOCOLATE,
    //             "coral" => Self::CORAL,
    //             "cornflowerblue" => Self::CORNFLOWER_BLUE,
    //             "cornsilk" => Self::CORNSILK,
    //             "crimson" => Self::CRIMSON,
    //             "cyan" => Self::CYAN,
    //             "darkblue" => Self::DARK_BLUE,
    //             "darkcyan" => Self::DARK_CYAN,
    //             "darkgoldenrod" => Self::DARK_GOLDENROD,
    //             "darkgray" | "darkgrey" => Self::DARK_GRAY,
    //             "darkgreen" => Self::DARK_GREEN,
    //             "darkkhaki" => Self::DARK_KHAKI,
    //             "darkmagenta" => Self::DARK_MAGENTA,
    //             "darkolivegreen" => Self::DARK_OLIVE_GREEN,
    //             "darkorange" => Self::DARK_ORANGE,
    //             "darkorchid" => Self::DARK_ORCHID,
    //             "darkred" => Self::DARK_RED,
    //             "darksalmon" => Self::DARK_SALMON,
    //             "darkseagreen" => Self::DARK_SEA_GREEN,
    //             "darkslateblue" => Self::DARK_SLATE_BLUE,
    //             "darkslategray" | "darkslategrey" => Self::DARK_SLATE_GRAY,
    //             "darkturquoise" => Self::DARK_TURQUOISE,
    //             "darkviolet" => Self::DARK_VIOLET,
    //             "deeppink" => Self::DEEP_PINK,
    //             "deepskyblue" => Self::DEEP_SKY_BLUE,
    //             "dimgray" | "dimgrey" => Self::DIM_GRAY,
    //             "dodgerblue" => Self::DODGER_BLUE,
    //             "firebrick" => Self::FIREBRICK,
    //             "floralwhite" => Self::FLORAL_WHITE,
    //             "forestgreen" => Self::FOREST_GREEN,
    //             "fuchsia" => Self::FUCHSIA,
    //             "gainsboro" => Self::GAINSBORO,
    //             "ghostwhite" => Self::GHOST_WHITE,
    //             "gold" => Self::GOLD,
    //             "goldenrod" => Self::GOLDENROD,
    //             "gray" | "grey" => Self::GRAY,
    //             "green" => Self::GREEN,
    //             "greenyellow" => Self::GREEN_YELLOW,
    //             "honeydew" => Self::HONEYDEW,
    //             "hotpink" => Self::HOT_PINK,
    //             "indianred" => Self::INDIAN_RED,
    //             "indigo" => Self::INDIGO,
    //             "ivory" => Self::IVORY,
    //             "khaki" => Self::KHAKI,
    //             "lavender" => Self::LAVENDER,
    //             "lavenderblush" => Self::LAVENDER_BLUSH,
    //             "lawngreen" => Self::LAWN_GREEN,
    //             "lemonchiffon" => Self::LEMON_CHIFFON,
    //             "lightblue" => Self::LIGHT_BLUE,
    //             "lightcoral" => Self::LIGHT_CORAL,
    //             "lightcyan" => Self::LIGHT_CYAN,
    //             "lightgoldenrodyellow" => Self::LIGHT_GOLDENROD_YELLOW,
    //             "lightgray" | "lightgrey" => Self::LIGHT_GRAY,
    //             "lightgreen" => Self::LIGHT_GREEN,
    //             "lightpink" => Self::LIGHT_PINK,
    //             "lightsalmon" => Self::LIGHT_SALMON,
    //             "lightseagreen" => Self::LIGHT_SEA_GREEN,
    //             "lightskyblue" => Self::LIGHT_SKY_BLUE,
    //             "lightslategray" | "lightslategrey" => Self::LIGHT_SLATE_GRAY,
    //             "lightsteelblue" => Self::LIGHT_STEEL_BLUE,
    //             "lightyellow" => Self::LIGHT_YELLOW,
    //             "lime" => Self::LIME,
    //             "limegreen" => Self::LIME_GREEN,
    //             "linen" => Self::LINEN,
    //             "magenta" => Self::MAGENTA,
    //             "maroon" => Self::MAROON,
    //             "mediumaquamarine" => Self::MEDIUM_AQUAMARINE,
    //             "mediumblue" => Self::MEDIUM_BLUE,
    //             "mediumorchid" => Self::MEDIUM_ORCHID,
    //             "mediumpurple" => Self::MEDIUM_PURPLE,
    //             "mediumseagreen" => Self::MEDIUM_SEA_GREEN,
    //             "mediumslateblue" => Self::MEDIUM_SLATE_BLUE,
    //             "mediumspringgreen" => Self::MEDIUM_SPRING_GREEN,
    //             "mediumturquoise" => Self::MEDIUM_TURQUOISE,
    //             "mediumvioletred" => Self::MEDIUM_VIOLET_RED,
    //             "midnightblue" => Self::MIDNIGHT_BLUE,
    //             "mintcream" => Self::MINT_CREAM,
    //             "mistyrose" => Self::MISTY_ROSE,
    //             "moccasin" => Self::MOCCASIN,
    //             "navajowhite" => Self::NAVAJO_WHITE,
    //             "navy" => Self::NAVY,
    //             "oldlace" => Self::OLD_LACE,
    //             "olive" => Self::OLIVE,
    //             "olivedrab" => Self::OLIVE_DRAB,
    //             "orange" => Self::ORANGE,
    //             "orangered" => Self::ORANGE_RED,
    //             "orchid" => Self::ORCHID,
    //             "palegoldenrod" => Self::PALE_GOLDENROD,
    //             "palegreen" => Self::PALE_GREEN,
    //             "paleturquoise" => Self::PALE_TURQUOISE,
    //             "palevioletred" => Self::PALE_VIOLET_RED,
    //             "papayawhip" => Self::PAPAYA_WHIP,
    //             "peachpuff" => Self::PEACH_PUFF,
    //             "peru" => Self::PERU,
    //             "pink" => Self::PINK,
    //             "plum" => Self::PLUM,
    //             "powderblue" => Self::POWDER_BLUE,
    //             "purple" => Self::PURPLE,
    //             "rebeccapurple" => Self::REBECCA_PURPLE,
    //             "red" => Self::RED,
    //             "rosybrown" => Self::ROSY_BROWN,
    //             "royalblue" => Self::ROYAL_BLUE,
    //             "saddlebrown" => Self::SADDLE_BROWN,
    //             "salmon" => Self::SALMON,
    //             "sandybrown" => Self::SANDY_BROWN,
    //             "seagreen" => Self::SEA_GREEN,
    //             "seashell" => Self::SEASHELL,
    //             "sienna" => Self::SIENNA,
    //             "silver" => Self::SILVER,
    //             "skyblue" => Self::SKY_BLUE,
    //             "slateblue" => Self::SLATE_BLUE,
    //             "slategray" | "slategrey" => Self::SLATE_GRAY,
    //             "snow" => Self::SNOW,
    //             "springgreen" => Self::SPRING_GREEN,
    //             "steelblue" => Self::STEEL_BLUE,
    //             "tan" => Self::TAN,
    //             "teal" => Self::TEAL,
    //             "thistle" => Self::THISTLE,
    //             "tomato" => Self::TOMATO,
    //             "turquoise" => Self::TURQUOISE,
    //             "violet" => Self::VIOLET,
    //             "wheat" => Self::WHEAT,
    //             "white" => Self::WHITE,
    //             "whitesmoke" => Self::WHITE_SMOKE,
    //             "yellow" => Self::YELLOW,
    //             "yellowgreen" => Self::YELLOW_GREEN,
    //             _ => return None,
    //         };
    //
    //         Some(Self::from_const(hex))
    //     }

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
