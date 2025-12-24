use serde::{Deserialize, Serialize};

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
