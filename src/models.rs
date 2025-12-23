use crate::color;
use crate::color::Color;
use crate::utils::as_array_ref;
use serde::{Deserialize, Serialize};

// # Colors (iTerm2 Default)
//
// [colors.bright]
// black = '#686868'
// blue = '#6871ff'
// cyan = '#60fdff'
// green = '#5ffa68'
// magenta = '#ff77ff'
// red = '#ff6e67'
// white = '#ffffff'
// yellow = '#fffc67'
//
// [colors.cursor]
// cursor = '#e5e5e5'
// text = '#000000'
//
// [colors.normal]
// black = '#000000'
// blue = '#2225c4'
// cyan = '#00c5c7'
// green = '#00c200'
// magenta = '#ca30c7'
// red = '#c91b00'
// white = '#ffffff'
// yellow = '#c7c400'
//
// [colors.primary]
// background = '#000000'
// foreground = '#ffffff'
//
// [colors.selection]
// background = '#c1deff'
// text = '#000000'

pub const ANSI_NC: usize = 8;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ColorScheme {
    bg: Option<String>,
    fg: Option<String>,
    normal: Option<AnsiColors>,
    bright: Option<AnsiColors>,
    dim: Option<AnsiColors>,
}

impl ColorScheme {
    pub fn complete(&mut self) {
        const SHADE_F: f32 = 0.15;

        if self.normal.is_none() || self.bright.is_none() || self.dim.is_none() {
            let mut normal = self.normal().clone();
            self.bright
                .get_or_insert(normal.for_each(|c| Color::shade(c, SHADE_F)));
            self.dim
                .get_or_insert(normal.for_each(|c| Color::shade(c, -SHADE_F)));
        }
        if let Some(normal) = self.normal.as_mut() {
            normal.complete();
        }
        if let Some(bright) = self.bright.as_mut() {
            bright.complete();
        }
        if let Some(dim) = self.dim.as_mut() {
            dim.complete();
        }
    }
    pub fn bg(&mut self) -> &str {
        self.bg.get_or_insert("#000000".into())
    }

    pub fn fg(&mut self) -> &str {
        self.fg.get_or_insert("#ffffff".into())
    }

    pub fn normal(&mut self) -> &AnsiColors {
        self.normal.get_or_insert(Default::default())
    }

    pub fn bright(&mut self) -> &AnsiColors {
        self.bright.get_or_insert(Default::default())
    }

    pub fn dim(&mut self) -> &AnsiColors {
        self.dim.get_or_insert(Default::default())
    }
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

    orange: Option<String>,
    pink: Option<String>,
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

            orange: None,
            pink: None,
        }
    }
}

impl AnsiColors {
    pub fn complete(&mut self) {
        self.orange
            .get_or_insert(color!(&self.red).blend(&color!(&self.yellow), 0.5).to_css());
        self.pink
            .get_or_insert(color!(&self.red).blend(&color!(&self.white), 0.5).to_css());
    }

    pub fn from_colors_slice(c: &[Color; ANSI_NC]) -> Self {
        Self {
            black: c[0].to_css(),
            red: c[1].to_css(),
            green: c[2].to_css(),
            yellow: c[3].to_css(),
            blue: c[4].to_css(),
            magenta: c[5].to_css(),
            cyan: c[6].to_css(),
            white: c[7].to_css(),
            ..Default::default()
        }
    }

    pub fn from_bytes(b: &[u8; ANSI_NC * 3]) -> Self {
        let mut colors = b
            .chunks_exact(3)
            .map(|s| Color::from_bytes(as_array_ref(s)));
        unsafe {
            Self {
                black: colors.next().unwrap_unchecked().to_css(),
                red: colors.next().unwrap_unchecked().to_css(),
                green: colors.next().unwrap_unchecked().to_css(),
                yellow: colors.next().unwrap_unchecked().to_css(),
                blue: colors.next().unwrap_unchecked().to_css(),
                magenta: colors.next().unwrap_unchecked().to_css(),
                cyan: colors.next().unwrap_unchecked().to_css(),
                white: colors.next().unwrap_unchecked().to_css(),
                ..Default::default()
            }
        }
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
        let mut buf = Vec::with_capacity(ANSI_NC * 3);
        for c in self.to_colors_array() {
            buf.extend_from_slice(&c.to_bytes());
        }
        buf
    }

    pub fn for_each(&mut self, f: fn(&Color) -> Color) -> Self {
        let mut colors = self.to_colors_array();
        colors.iter_mut().for_each(|c| *c = f(c));
        Self::from_colors_slice(&colors)
    }
}

#[derive(Deserialize, Debug)]
pub struct CursorColors {
    cursor: String,
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct PrimaryColors {
    background: String,
    foreground: String,
}

#[derive(Daserialize, Debug)]
pub struct SelectionColors {
    background: String,
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct Colors {
    primary: Primary,
    normal: TermColors,
    bright: TermColors,
    cursor: Cursor,
    selection: Selection,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    colors: Colors,
}
