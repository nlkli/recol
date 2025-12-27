use crate::{
    collection::{AnsiColors as CsAnsiColors, ColorScheme, Theme},
    utils,
};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

pub fn write_theme_into_config(path: impl AsRef<Path>, theme: &mut Theme) -> io::Result<()> {
    let content = fs::read_to_string(&path)?;
    let mut config =
        toml::from_str::<Config>(&content).map_err(|_| utils::io_other_error("serde fail"))?;
    config
        .colors
        .replace(Colors::from_color_scheme(&mut theme.colors));
    let content =
        toml::to_string::<Config>(&config).map_err(|_| utils::io_other_error("serde fail"))?;

    fs::write(&path, &content)?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub colors: Option<Colors>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub font: Option<Font>,

    #[serde(flatten)]
    pub other: toml::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Colors {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub primary: Option<PrimaryColors>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cursor: Option<CursorColors>,

    // #[serde(skip_serializing_if = "Option::is_none", default)]
    // pub vi_mode_cursor: Option<CursorColors>,

    // #[serde(skip_serializing_if = "Option::is_none", default)]
    // pub search: Option<SearchColors>,

    // #[serde(skip_serializing_if = "Option::is_none", default)]
    // pub footer_bar: Option<BarColors>,

    // #[serde(skip_serializing_if = "Option::is_none", default)]
    // pub hints: Option<HintsColors>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub selection: Option<SelectionColors>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub normal: Option<AnsiColors>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bright: Option<AnsiColors>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dim: Option<AnsiColors>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub indexed_colors: Vec<IndexedColor>,
}

impl Colors {
    pub fn from_color_scheme(cs: &mut ColorScheme) -> Self {
        Self {
            primary: Some(PrimaryColors {
                background: Some(cs.background[1].clone()),
                foreground: Some(cs.foreground[1].clone()),
                dim_foreground: Some(cs.foreground[2].clone()),
                bright_foreground: Some(cs.foreground[0].clone()),
            }),
            cursor: Some(CursorColors {
                cursor: Some(cs.cursor.bg.clone()),
                text: Some(cs.cursor.fg.clone()),
            }),
            selection: Some(SelectionColors {
                background: Some(cs.selection.bg.clone()),
                text: Some(cs.selection.fg.clone()),
            }),
            normal: Some(AnsiColors::from_color_scheme_ansi(&cs.base)),
            bright: Some(AnsiColors::from_color_scheme_ansi(&cs.bright)),
            dim: Some(AnsiColors::from_color_scheme_ansi(cs.dim(None))),
            indexed_colors: vec![
                IndexedColor {
                    index: 16,
                    color: cs.base.orange.clone(),
                },
                IndexedColor {
                    index: 17,
                    color: cs.base.pink.clone(),
                },
            ],
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrimaryColors {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub background: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub foreground: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dim_foreground: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bright_foreground: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CursorColors {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SelectionColors {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub background: Option<String>,
}

// #[derive(Debug, Clone, Default, Serialize, Deserialize)]
// pub struct BarColors {
//     #[serde(skip_serializing_if = "Option::is_none", default)]
//     pub foreground: Option<String>,
//
//     #[serde(skip_serializing_if = "Option::is_none", default)]
//     pub background: Option<String>,
// }

// #[derive(Debug, Clone, Default, Serialize, Deserialize)]
// pub struct SearchColors {
//     #[serde(skip_serializing_if = "Option::is_none", default)]
//     pub matches: Option<BarColors>,
//
//     #[serde(skip_serializing_if = "Option::is_none", default)]
//     pub focused_match: Option<BarColors>,
// }

// #[derive(Debug, Clone, Default, Serialize, Deserialize)]
// pub struct HintsColors {
//     #[serde(skip_serializing_if = "Option::is_none", default)]
//     pub start: Option<BarColors>,
//     #[serde(skip_serializing_if = "Option::is_none", default)]
//     pub end: Option<BarColors>,
// }

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnsiColors {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub black: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub red: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub green: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub yellow: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub blue: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub magenta: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cyan: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub white: Option<String>,
}

impl AnsiColors {
    pub fn from_color_scheme_ansi(ansi: &CsAnsiColors) -> Self {
        Self {
            black: Some(ansi.black.clone()),
            red: Some(ansi.red.clone()),
            green: Some(ansi.green.clone()),
            yellow: Some(ansi.yellow.clone()),
            blue: Some(ansi.blue.clone()),
            magenta: Some(ansi.magenta.clone()),
            cyan: Some(ansi.cyan.clone()),
            white: Some(ansi.white.clone()),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexedColor {
    pub index: u8,
    pub color: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Font {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub normal: Option<FontInner>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FontInner {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub family: Option<String>,
}
