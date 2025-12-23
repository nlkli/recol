use serde::{Serialize, Deserialize};
use crate::utils::as_array_ref;
use crate::color::Color;
use crate::color;

const ANSI_NC: usize = 8;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ColorScheme {
    normal: Option<AnsiColors>,
    bright: Option<AnsiColors>,
    dim: Option<AnsiColors>,
}

impl ColorScheme {

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
}

impl Default for AnsiColors {
    fn default() -> Self {
        Self { 
            black: Default::default(), 
            red: Default::default(), 
            green: Default::default(), 
            yellow: Default::default(), 
            blue: Default::default(), 
            magenta: Default::default(), 
            cyan: Default::default(), 
            white: Default::default()
        }
    }
}

impl AnsiColors {
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
        }
    }


    pub fn from_bytes(b: &[u8; ANSI_NC * 3]) -> Self {
        let mut colors = b.chunks_exact(3).map(|s| Color::from_bytes(as_array_ref(s)));
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

