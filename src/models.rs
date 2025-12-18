#[allow(dead_code)]
use crate::color::Color;
use serde::{Deserialize, Serialize};

pub type BackgroundColors = [String; 5];
pub type ForegroundColors = [String; 4];
pub type SelectionColors = [String; 2];

const DEFAULT_BACKGROUND_COLOR: &str = "#000000";
const DEFAULT_FOREGROUND_COLOR: &str = "#ffffff";
const DEFAULT_SELECTION_COLOR: &str = "#2a2a2a";

const DEFAULT_SHADE_FACTOR: f32 = 0.15;
const DEFAULT_COMMENT_BLEND_FACTOR: f32 = 0.4;

const DEFAULT_DIFF_BLEND_CONFIG: DiffBlendConfig = DiffBlendConfig {
    add: 0.2,
    delete: 0.2,
    change: 0.2,
    text: 0.3,
};

const DEFAULT_BACKGROUND_SHADE_CONFIG: BackgroundShadeConfig =
    BackgroundShadeConfig([-4., 6., 12., 23.]);
const DEFAULT_FOREGROUND_SHADE_CONFIG: ForegroundShadeConfig =
    ForegroundShadeConfig([6., -23., -46.]);
const DEFAULT_SELECTION_SHADE_CONFIG: SelectionShadeConfig = SelectionShadeConfig([16.]);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DiffBlendConfig {
    pub add: f32,
    pub delete: f32,
    pub change: f32,
    pub text: f32,
}

impl Default for DiffBlendConfig {
    fn default() -> Self {
        DEFAULT_DIFF_BLEND_CONFIG
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BackgroundShadeConfig(pub [f32; 4]);

impl Default for BackgroundShadeConfig {
    fn default() -> Self {
        DEFAULT_BACKGROUND_SHADE_CONFIG
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ForegroundShadeConfig(pub [f32; 3]);

impl Default for ForegroundShadeConfig {
    fn default() -> Self {
        DEFAULT_FOREGROUND_SHADE_CONFIG
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SelectionShadeConfig(pub [f32; 1]);

impl Default for SelectionShadeConfig {
    fn default() -> Self {
        DEFAULT_SELECTION_SHADE_CONFIG
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermColors {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub orange: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pink: Option<String>,
}

impl Default for TermColors {
    fn default() -> Self {
        Self {
            black: "#000000".into(),
            red: "#ff0000".into(),
            green: "#00ff00".into(),
            yellow: "#ffff00".into(),
            blue: "#0000ff".into(),
            magenta: "#ff00ff".into(),
            cyan: "#00ffff".into(),
            white: "#ffffff".into(),
            orange: Some("#ff8700".into()),
            pink: Some("#ff5faf".into()),
        }
    }
}

impl TermColors {
    pub fn shade(&self, factor: Option<f32>) -> Result<Self, String> {
        let factor = factor.unwrap_or(DEFAULT_SHADE_FACTOR);
        Ok(Self {
            black: Color::from_hex_str(&self.black)?
                .shade(factor)
                .to_css(false),
            red: Color::from_hex_str(&self.red)?.shade(factor).to_css(false),
            green: Color::from_hex_str(&self.green)?
                .shade(factor)
                .to_css(false),
            yellow: Color::from_hex_str(&self.yellow)?
                .shade(factor)
                .to_css(false),
            blue: Color::from_hex_str(&self.blue)?.shade(factor).to_css(false),
            magenta: Color::from_hex_str(&self.magenta)?
                .shade(factor)
                .to_css(false),
            cyan: Color::from_hex_str(&self.cyan)?.shade(factor).to_css(false),
            white: Color::from_hex_str(&self.white)?
                .shade(factor)
                .to_css(false),
            orange: Some(
                Color::from_hex_str(&self.orange.as_ref().unwrap_or(&self.yellow))?
                    .shade(factor)
                    .to_css(false),
            ),
            pink: Some(
                Color::from_hex_str(&self.pink.as_ref().unwrap_or(&self.red))?
                    .shade(factor)
                    .to_css(false),
            ),
        })
    }

    pub fn validation(&self) -> Result<(), String> {
        let _ = Color::from_hex_str(&self.black)?;
        let _ = Color::from_hex_str(&self.red)?;
        let _ = Color::from_hex_str(&self.green)?;
        let _ = Color::from_hex_str(&self.yellow)?;
        let _ = Color::from_hex_str(&self.blue)?;
        let _ = Color::from_hex_str(&self.magenta)?;
        let _ = Color::from_hex_str(&self.cyan)?;
        let _ = Color::from_hex_str(&self.white)?;
        if let Some(c) = self.orange.as_ref() {
            let _ = Color::from_hex_str(c)?;
        }
        if let Some(c) = self.pink.as_ref() {
            let _ = Color::from_hex_str(c)?;
        }

        Ok(())
    }
}

macro_rules! color_enum {
    ($name:ident, $colors:ty, $default:expr) => {
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub enum $name {
            Color(String),
            Colors($colors),
            #[default]
            None,
        }

        impl $name {
            pub fn index(&self, i: usize) -> &str {
                match self {
                    Self::Colors(c) => &c[i],
                    _ => panic!(concat!(stringify!($name), " not prepared")),
                }
            }

            pub fn is_none(&self) -> bool {
                matches!(self, Self::None)
            }

            fn ensure_color(&mut self) {
                if matches!(self, Self::None) {
                    *self = Self::Color($default.into());
                }
            }
        }
    };
}

color_enum!(Background, BackgroundColors, DEFAULT_BACKGROUND_COLOR);
color_enum!(Foreground, ForegroundColors, DEFAULT_FOREGROUND_COLOR);
color_enum!(Selection, SelectionColors, DEFAULT_SELECTION_COLOR);

impl Background {
    pub fn prepare(&mut self, cfg: &Option<BackgroundShadeConfig>) -> Result<(), String> {
        self.ensure_color();

        let s = cfg.unwrap_or(DEFAULT_BACKGROUND_SHADE_CONFIG).0;
        match self {
            Self::Color(c) => {
                let base = Color::from_hex_str(c)?;

                *self = Self::Colors([
                    base.brighten(s[0]).to_css(false),
                    base.to_css(false),
                    base.brighten(s[1]).to_css(false),
                    base.brighten(s[2]).to_css(false),
                    base.brighten(s[3]).to_css(false),
                ]);
            }
            Self::Colors(cs) => {
                let find = cs
                    .iter()
                    .enumerate()
                    .find(|(_, v)| !v.is_empty())
                    .map(|(n, v)| (n, v.clone()))
                    .ok_or("Background base color not defined")?;
                let find_color = Color::from_hex_str(&find.1)?;
                let base_color = if cs[1].is_empty() {
                    let bc = if find.0 == 0 {
                        find_color.brighten(-s[0])
                    } else {
                        let bc = find_color.brighten(s[find.0 - 1]);
                        cs[0] = bc.brighten(s[0]).to_css(false);
                        bc
                    };
                    cs[1] = bc.to_css(false);
                    bc
                } else {
                    Color::from_hex_str(&cs[1])?
                };
                for (n, c) in cs.iter_mut().skip(2).enumerate() {
                    if c.is_empty() {
                        *c = base_color.brighten(s[n + 1]).to_css(false);
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn validation(&self) -> Result<(), String> {
        match self {
            Self::Color(c) => {
                let _ = Color::from_hex_str(c)?;
            }
            Self::Colors(cs) => {
                for c in cs {
                    let _ = Color::from_hex_str(c)?;
                }
            }
            _ => (),
        }

        Ok(())
    }
}

impl Foreground {
    pub fn prepare(&mut self, cfg: &Option<ForegroundShadeConfig>) -> Result<(), String> {
        self.ensure_color();

        let s = cfg.unwrap_or(DEFAULT_FOREGROUND_SHADE_CONFIG).0;
        match self {
            Self::Color(c) => {
                let base = Color::from_hex_str(c)?;

                *self = Self::Colors([
                    base.brighten(s[0]).to_css(false),
                    base.to_css(false),
                    base.brighten(s[1]).to_css(false),
                    base.brighten(s[2]).to_css(false),
                ]);
            }
            Self::Colors(cs) => {
                let find = cs
                    .iter()
                    .enumerate()
                    .find(|(_, v)| !v.is_empty())
                    .map(|(n, v)| (n, v.clone()))
                    .ok_or("Foreground base color not defined")?;
                let find_color = Color::from_hex_str(&find.1)?;
                let base_color = if cs[1].is_empty() {
                    let bc = if find.0 == 0 {
                        find_color.brighten(-s[0])
                    } else {
                        let bc = find_color.brighten(s[find.0 - 1]);
                        cs[0] = bc.brighten(s[0]).to_css(false);
                        bc
                    };
                    cs[1] = bc.to_css(false);
                    bc
                } else {
                    Color::from_hex_str(&cs[1])?
                };
                for (n, c) in cs.iter_mut().skip(2).enumerate() {
                    if c.is_empty() {
                        *c = base_color.brighten(s[n + 1]).to_css(false);
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn validation(&self) -> Result<(), String> {
        match self {
            Self::Color(c) => {
                let _ = Color::from_hex_str(c)?;
            }
            Self::Colors(cs) => {
                for c in cs {
                    let _ = Color::from_hex_str(c)?;
                }
            }
            _ => (),
        }

        Ok(())
    }
}

impl Selection {
    pub fn prepare(&mut self, cfg: &Option<SelectionShadeConfig>) -> Result<(), String> {
        self.ensure_color();

        if let Self::Color(c) = self {
            let base = Color::from_hex_str(c)?;
            let s = cfg.unwrap_or(DEFAULT_SELECTION_SHADE_CONFIG).0[0];

            *self = Self::Colors([base.to_css(false), base.brighten(s).to_css(false)]);
        }
        Ok(())
    }

    pub fn validation(&self) -> Result<(), String> {
        match self {
            Self::Color(c) => {
                let _ = Color::from_hex_str(c)?;
            }
            Self::Colors(cs) => {
                for c in cs {
                    let _ = Color::from_hex_str(c)?;
                }
            }
            _ => (),
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Theme {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub light: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub colors: Option<ThemeColors>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub config: Option<ThemeConfig>,
}

impl Theme {
    pub fn get_colors(&mut self) -> &ThemeColors {
        self.colors.get_or_insert(Default::default())
    }

    // pub fn get_config(&mut self) -> &ThemeConfig {
    //     self.config.get_or_insert(Default::default())
    // }

    pub fn is_light(&mut self) -> bool {
        *self.light.get_or_insert(Default::default())
    }

    pub fn prepare(&mut self) -> Result<(), String> {
        let config = self.config.get_or_insert(Default::default());
        self.colors
            .get_or_insert(Default::default())
            .prepare(&config)?;
        Ok(())
    }

    pub fn validation(&mut self) -> Result<(), String> {
        self.colors.get_or_insert(Default::default()).validation()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub shade_factor: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment_blend_factor: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub background_shade: Option<BackgroundShadeConfig>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub foreground_shade: Option<ForegroundShadeConfig>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub selection_shade: Option<SelectionShadeConfig>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub diff_blend: Option<DiffBlendConfig>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            shade_factor: Some(DEFAULT_SHADE_FACTOR),
            comment_blend_factor: Some(DEFAULT_COMMENT_BLEND_FACTOR),
            background_shade: Some(DEFAULT_BACKGROUND_SHADE_CONFIG),
            foreground_shade: Some(DEFAULT_FOREGROUND_SHADE_CONFIG),
            selection_shade: Some(DEFAULT_SELECTION_SHADE_CONFIG),
            diff_blend: Some(DEFAULT_DIFF_BLEND_CONFIG),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeColors {
    #[serde(default)]
    pub base: TermColors,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bright: Option<TermColors>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dim: Option<TermColors>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub variable: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status_line: Option<String>,

    #[serde(skip_serializing_if = "Background::is_none", default)]
    pub background: Background,

    #[serde(skip_serializing_if = "Foreground::is_none", default)]
    pub foreground: Foreground,

    #[serde(skip_serializing_if = "Selection::is_none", default)]
    pub selection: Selection,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub diff: Option<DiffColors>,
}

impl ThemeColors {
    pub fn prepare(&mut self, config: &ThemeConfig) -> Result<(), String> {
        self.background.prepare(&config.background_shade)?;
        self.foreground.prepare(&config.foreground_shade)?;
        self.selection.prepare(&config.selection_shade)?;

        let shade = config.shade_factor.unwrap_or(DEFAULT_SHADE_FACTOR);

        self.bright.get_or_insert(self.base.shade(Some(shade))?);
        self.dim.get_or_insert(self.base.shade(Some(-shade))?);

        let bg = Color::from_hex_str(self.background.index(1))?;
        let diff_cfg = config.diff_blend.unwrap_or_default();

        let diff = self.diff.get_or_insert_with(DiffColors::default);

        diff.add.get_or_insert(
            bg.blend(&Color::from_hex_str(&self.base.green)?, diff_cfg.add)
                .to_css(false),
        );
        diff.delete.get_or_insert(
            bg.blend(&Color::from_hex_str(&self.base.red)?, diff_cfg.delete)
                .to_css(false),
        );
        diff.change.get_or_insert(
            bg.blend(&Color::from_hex_str(&self.base.blue)?, diff_cfg.change)
                .to_css(false),
        );
        diff.text.get_or_insert(
            bg.blend(&Color::from_hex_str(&self.base.cyan)?, diff_cfg.text)
                .to_css(false),
        );

        self.comment.get_or_insert(
            bg.blend(
                &Color::from_hex_str(self.foreground.index(1))?,
                config
                    .comment_blend_factor
                    .unwrap_or(DEFAULT_COMMENT_BLEND_FACTOR),
            )
            .to_css(false),
        );

        self.variable.get_or_insert(self.foreground.index(1).into());

        self.status_line
            .get_or_insert(self.background.index(0).into());

        Ok(())
    }

    pub fn validation(&self) -> Result<(), String> {
        self.base.validation()?;
        if let Some(b) = self.bright.as_ref() {
            b.validation()?;
        }
        if let Some(d) = self.dim.as_ref() {
            d.validation()?;
        }
        if let Some(d) = self.diff.as_ref() {
            d.validation()?;
        }
        self.background.validation()?;
        self.foreground.validation()?;
        self.selection.validation()?;
        if let Some(c) = self.comment.as_ref() {
            let _ = Color::from_hex_str(c)?;
        }
        if let Some(c) = self.status_line.as_ref() {
            let _ = Color::from_hex_str(c)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffColors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl DiffColors {
    pub fn validation(&self) -> Result<(), String> {
        if let Some(c) = self.add.as_ref() {
            let _ = Color::from_hex_str(c)?;
        }
        if let Some(c) = self.delete.as_ref() {
            let _ = Color::from_hex_str(c)?;
        }
        if let Some(c) = self.change.as_ref() {
            let _ = Color::from_hex_str(c)?;
        }
        if let Some(c) = self.text.as_ref() {
            let _ = Color::from_hex_str(c)?;
        }

        Ok(())
    }
}

pub mod alacritty {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub colors: Option<Colors>,

        #[serde(flatten)]
        pub other: toml::Value,
    }

    impl Config {
        pub fn replace_colors_from_theme(&mut self, theme: &ThemeColors) {
            let new_colors = Colors::from_theme(theme);
            self.colors = Some(new_colors);
        }
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct Colors {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub primary: Option<PrimaryColors>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cursor: Option<CursorColors>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub vi_mode_cursor: Option<CursorColors>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub search: Option<SearchColors>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub footer_bar: Option<BarColors>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hints: Option<HintsColors>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub selection: Option<SelectionColors>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub normal: Option<AnsiColors>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bright: Option<AnsiColors>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dim: Option<AnsiColors>,

        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub indexed_colors: Vec<IndexedColor>,
    }

    impl Colors {
        pub fn from_theme(theme: &ThemeColors) -> Self {
            Self {
                primary: Some(PrimaryColors {
                    background: Some(theme.background.index(1).into()),
                    foreground: Some(theme.foreground.index(1).into()),
                    dim_foreground: Some(theme.foreground.index(2).into()),
                    bright_foreground: Some(theme.foreground.index(0).into()),
                }),
                cursor: Some(CursorColors {
                    text: Some(theme.background.index(1).into()),
                    cursor: Some(theme.foreground.index(2).into()),
                }),
                // vi_mode_cursor: Some(CursorColors {
                //     text: Some(theme.background.index(1).into()),
                //     cursor: Some(theme.foreground.index(3).into()),
                // }),
                selection: Some(SelectionColors {
                    text: Some(theme.foreground.index(1).into()),
                    background: Some(theme.selection.index(0).into()),
                }),
                normal: Some(AnsiColors::from(&theme.base)),
                bright: theme.bright.as_ref().map(AnsiColors::from),
                dim: theme.dim.as_ref().map(AnsiColors::from),
                indexed_colors: vec![
                    IndexedColor {
                        index: 16,
                        color: theme
                            .base
                            .orange
                            .clone()
                            .unwrap_or(theme.base.yellow.clone()),
                    },
                    IndexedColor {
                        index: 17,
                        color: theme.base.pink.clone().unwrap_or(theme.base.red.clone()),
                    },
                ],
                ..Default::default()
            }
        }
    }

    impl From<&TermColors> for AnsiColors {
        fn from(t: &TermColors) -> Self {
            Self {
                black: t.black.clone(),
                red: t.red.clone(),
                green: t.green.clone(),
                yellow: t.yellow.clone(),
                blue: t.blue.clone(),
                magenta: t.magenta.clone(),
                cyan: t.cyan.clone(),
                white: t.white.clone(),
            }
        }
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct PrimaryColors {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub background: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub foreground: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dim_foreground: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bright_foreground: Option<String>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct CursorColors {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cursor: Option<String>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct BarColors {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub foreground: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub background: Option<String>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct SelectionColors {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub background: Option<String>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct SearchColors {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub matches: Option<BarColors>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub focused_match: Option<BarColors>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct HintsColors {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start: Option<BarColors>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end: Option<BarColors>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct IndexedColor {
        pub index: u8,
        pub color: String,
    }
}
