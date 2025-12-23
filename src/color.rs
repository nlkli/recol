use std::io::Write;

pub fn print_palette(colors: &[Color]) {
    print!("\x1b[48;2;90;90;90m");
    for _ in colors {
        print!("    ");
    }
    println!("\x1b[0m");
    for _ in 0..2 {
        for c in colors {
            let (r, g, b) = c.to_rgb();
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

#[inline(always)]
fn clamp(v: f32, min: f32, max: f32) -> f32 {
    v.max(min).min(max)
}

#[inline(always)]
fn round_u8(v: f32) -> u8 {
    (v + 0.5).floor() as u8
}

#[macro_export]
macro_rules! color {
    ($e:expr) => {
        Color::from_css($e)
    };
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: clamp(r, 0.0, 1.0),
            g: clamp(g, 0.0, 1.0),
            b: clamp(b, 0.0, 1.0),
        }
    }

    pub fn is_valid_css_str(s: &str) -> bool {
        let hex = s.trim_start_matches('#');
        hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit())
    }

    pub fn from_css(s: &str) -> Self {
        let hex = s.trim_start_matches('#');
        assert!(hex.len() == 6, "Hex string must be 6 characters");
        let value = u32::from_str_radix(hex, 16).expect("Invalid hex color");
        Self::from_hex(value)
    }

    pub fn from_bytes(b: &[u8; 3]) -> Self {
        Self::new(
            b[0] as f32 / 255.0,
            b[1] as f32 / 255.0,
            b[2] as f32 / 255.0,
        )
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        )
    }

    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xff) as f32 / 255.0;
        let g = ((hex >> 8) & 0xff) as f32 / 255.0;
        let b = (hex & 0xff) as f32 / 255.0;
        Self::new(r, g, b)
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let h = h.rem_euclid(360.0);
        let s = clamp(s, 0.0, 100.0) / 100.0;
        let v = clamp(v, 0.0, 100.0) / 100.0;

        let f = |n: f32| {
            let k = (n + h / 60.0).rem_euclid(6.0);
            v - v * s * (k.min(4.0 - k).min(1.0)).max(0.0)
        };

        Self::new(f(5.0), f(3.0), f(1.0))
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let h = h.rem_euclid(360.0);
        let s = clamp(s, 0.0, 100.0) / 100.0;
        let l = clamp(l, 0.0, 100.0) / 100.0;

        let a = s * l.min(1.0 - l);

        let f = |n: f32| {
            let k = (n + h / 30.0).rem_euclid(12.0);
            l - a * (k - 3.0).min(9.0 - k).min(1.0).max(-1.0)
        };

        Self::new(f(0.0), f(8.0), f(4.0))
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            round_u8(self.r * 255.0),
            round_u8(self.g * 255.0),
            round_u8(self.b * 255.0),
        )
    }

    pub fn to_bytes(&self) -> [u8; 3] {
        let rgb = self.to_rgb();
        [rgb.0, rgb.1, rgb.2]
    }

    pub fn to_hex(&self) -> u32 {
        let r = round_u8(self.r * 255.0) as u32;
        let g = round_u8(self.g * 255.0) as u32;
        let b = round_u8(self.b * 255.0) as u32;
        (r << 16) | (g << 8) | b
    }

    pub fn to_css(&self) -> String {
        format!("#{:06x}", self.to_hex())
    }

    pub fn to_hsv(&self) -> (f32, f32, f32) {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max - min;

        let mut h = if delta == 0.0 {
            0.0
        } else if max == self.r {
            60.0 * ((self.g - self.b) / delta)
        } else if max == self.g {
            60.0 * ((self.b - self.r) / delta + 2.0)
        } else {
            60.0 * ((self.r - self.g) / delta + 4.0)
        };

        if h < 0.0 {
            h += 360.0;
        }

        let s = if max == 0.0 { 0.0 } else { delta / max };
        (h, s * 100.0, max * 100.0)
    }

    pub fn to_hsl(&self) -> (f32, f32, f32) {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let l = (max + min) / 2.0;

        let delta = max - min;
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        let h = self.to_hsv().0;
        (h, s * 100.0, l * 100.0)
    }

    pub fn blend(&self, other: &Color, f: f32) -> Color {
        Color::new(
            (other.r - self.r) * f + self.r,
            (other.g - self.g) * f + self.g,
            (other.b - self.b) * f + self.b,
        )
    }

    pub fn shade(&self, f: f32) -> Color {
        let t = if f < 0.0 { 0.0 } else { 1.0 };
        let p = f.abs();

        Color::new(
            (t - self.r) * p + self.r,
            (t - self.g) * p + self.g,
            (t - self.b) * p + self.b,
        )
    }

    pub fn brighten(&self, v: f32) -> Color {
        let (h, s, val) = self.to_hsv();
        Color::from_hsv(h, s, clamp(val + v, 0.0, 100.0))
    }

    pub fn lighten(&self, v: f32) -> Color {
        let (h, s, l) = self.to_hsl();
        Color::from_hsl(h, s, clamp(l + v, 0.0, 100.0))
    }

    pub fn saturate(&self, v: f32) -> Color {
        let (h, s, val) = self.to_hsv();
        Color::from_hsv(h, clamp(s + v, 0.0, 100.0), val)
    }

    pub fn rotate(&self, v: f32) -> Color {
        let (h, s, val) = self.to_hsv();
        Color::from_hsv((h + v).rem_euclid(360.0), s, val)
    }

    pub fn luminance(&self) -> f32 {
        let f = |c: f32| {
            if c > 0.04045 {
                ((c + 0.055) / 1.055).powf(2.4)
            } else {
                c / 12.92
            }
        };

        0.2126 * f(self.r) + 0.7152 * f(self.g) + 0.0722 * f(self.b)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_css())
    }
}
