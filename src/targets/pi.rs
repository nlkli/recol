use recol_lib as lib;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{io, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,

    #[serde(flatten)]
    pub any: HashMap<String, serde_json::Value>,
}

pub fn write_theme_to_config(path: impl AsRef<Path>, theme: &lib::Theme) -> io::Result<()> {
    let c = theme.colors.clone().into_advanced(None);

    let variant = if theme.is_light { &c.dim } else { &c.bright };

    let content = format!(
        r###"{{
  "$schema": "https://raw.githubusercontent.com/earendil-works/pi/main/packages/coding-agent/src/modes/interactive/theme/theme-schema.json",
  "name": "{theme_name}",
  "vars": {{
    "black_base": "{black_base}",
    "black_bright": "{black_bright}",
    "black_dim": "{black_dim}",
    "red_base": "{red_base}",
    "red_bright": "{red_bright}",
    "red_dim": "{red_dim}",
    "red_variant": "{red_variant}",
    "green_base": "{green_base}",
    "green_bright": "{green_bright}",
    "green_dim": "{green_dim}",
    "yellow_base": "{yellow_base}",
    "yellow_bright": "{yellow_bright}",
    "yellow_dim": "{yellow_dim}",
    "yellow_variant": "{yellow_variant}",
    "blue_base": "{blue_base}",
    "blue_bright": "{blue_bright}",
    "blue_dim": "{blue_dim}",
    "blue_variant": "{blue_variant}",
    "magenta_base": "{magenta_base}",
    "magenta_bright": "{magenta_bright}",
    "magenta_dim": "{magenta_dim}",
    "magenta_variant": "{magenta_variant}",
    "cyan_base": "{cyan_base}",
    "cyan_bright": "{cyan_bright}",
    "cyan_dim": "{cyan_dim}",
    "cyan_variant": "{cyan_variant}",
    "white_base": "{white_base}",
    "white_bright": "{white_bright}",
    "white_dim": "{white_dim}",
    "orange_base": "{orange_base}",
    "orange_bright": "{orange_bright}",
    "orange_dim": "{orange_dim}",
    "orange_variant": "{orange_variant}",
    "pink_base": "{pink_base}",
    "pink_bright": "{pink_bright}",
    "pink_dim": "{pink_dim}",
    "pink_variant": "{pink_variant}",
    "bg0": "{bg0}",
    "bg1": "{bg1}",
    "bg2": "{bg2}",
    "bg3": "{bg3}",
    "bg4": "{bg4}",
    "fg0": "{fg0}",
    "fg1": "{fg1}",
    "fg2": "{fg2}",
    "fg3": "{fg3}",
    "sel0": "{sel0}",
    "sel1": "{sel1}",
    "cur_bg": "{cur_bg}",
    "cur_fg": "{cur_fg}",
    "comment": "{comment}",
    "diff_add": "{diff_add}",
    "diff_delete": "{diff_delete}",
    "diff_change": "{diff_change}",
    "diff_text": "{diff_text}"
  }},
  "colors": {{
    "accent": "blue_variant",
    "border": "fg3",
    "borderAccent": "blue_base",
    "borderMuted": "bg3",
    "success": "green_base",
    "error": "red_base",
    "warning": "yellow_base",
    "muted": "fg2",
    "dim": "fg3",
    "text": "fg1",
    "thinkingText": "comment",

    "selectedBg": "sel0",
    "scrollbarThumb": "sel0",
    "searchMatchBg": "sel0",
    "searchMatchText": "fg1",
    "userMessageBg": "bg2",
    "userMessageText": "fg1",
    "customMessageBg": "bg3",
    "customMessageText": "fg1",
    "customMessageLabel": "blue_variant",
    "toolPendingBg": "bg0",
    "toolSuccessBg": "diff_add",
    "toolErrorBg": "diff_delete",
    "toolTitle": "blue_variant",
    "toolOutput": "fg2",

    "mdHeading": "blue_variant",
    "mdLink": "magenta_base",
    "mdLinkUrl": "orange_variant",
    "mdCode": "cyan_base",
    "mdCodeBlock": "pink_base",
    "mdCodeBlockBorder": "fg3",
    "mdQuote": "fg2",
    "mdQuoteBorder": "bg3",
    "mdHr": "bg4",
    "mdListBullet": "cyan_variant",

    "toolDiffAdded": "green_base",
    "toolDiffRemoved": "red_base",
    "toolDiffContext": "comment",

    "syntaxComment": "comment",
    "syntaxKeyword": "magenta_base",
    "syntaxFunction": "blue_variant",
    "syntaxVariable": "fg1",
    "syntaxString": "green_base",
    "syntaxNumber": "orange_base",
    "syntaxType": "yellow_base",
    "syntaxOperator": "fg2",
    "syntaxPunctuation": "fg2",

    "thinkingOff": "bg4",
    "thinkingMinimal": "comment",
    "thinkingLow": "blue_base",
    "thinkingMedium": "cyan_base",
    "thinkingHigh": "magenta_base",
    "thinkingXhigh": "orange_base",
    "thinkingMax": "red_base",

    "bashMode": "orange_variant"
  }}
}}
"###,
        theme_name = theme.name,
        black_base = c.base.black,
        black_bright = c.bright.black,
        black_dim = c.dim.black,
        red_base = c.base.red,
        red_bright = c.bright.red,
        red_dim = c.dim.red,
        red_variant = variant.red,
        green_base = c.base.green,
        green_bright = c.bright.green,
        green_dim = c.dim.green,
        yellow_base = c.base.yellow,
        yellow_bright = c.bright.yellow,
        yellow_dim = c.dim.yellow,
        yellow_variant = variant.yellow,
        blue_base = c.base.blue,
        blue_bright = c.bright.blue,
        blue_dim = c.dim.blue,
        blue_variant = variant.blue,
        magenta_base = c.base.magenta,
        magenta_bright = c.bright.magenta,
        magenta_dim = c.dim.magenta,
        magenta_variant = variant.magenta,
        cyan_base = c.base.cyan,
        cyan_bright = c.bright.cyan,
        cyan_dim = c.dim.cyan,
        cyan_variant = variant.cyan,
        white_base = c.base.white,
        white_bright = c.bright.white,
        white_dim = c.dim.white,
        orange_base = c.base.orange,
        orange_bright = c.bright.orange,
        orange_dim = c.dim.orange,
        orange_variant = variant.orange,
        pink_base = c.base.pink,
        pink_bright = c.bright.pink,
        pink_dim = c.dim.pink,
        pink_variant = variant.pink,
        comment = c.comment,
        bg0 = c.bg[0],
        bg1 = c.bg[1],
        bg2 = c.bg[2],
        bg3 = c.bg[3],
        bg4 = c.bg[4],
        fg0 = c.fg[0],
        fg1 = c.fg[1],
        fg2 = c.fg[2],
        fg3 = c.fg[3],
        sel0 = c.alt_selection[0],
        sel1 = c.alt_selection[1],
        cur_bg = c.cursor.bg,
        cur_fg = c.cursor.fg,
        diff_add = c.diff.add,
        diff_delete = c.diff.delete,
        diff_change = c.diff.change,
        diff_text = c.diff.text,
    );

    let themes_path = path.as_ref().join("themes");
    std::fs::create_dir_all(&themes_path)?;

    let settings_path = path.as_ref().join("settings.json");
    if let Ok(settings) = std::fs::read_to_string(&settings_path) {
        let mut settings = serde_json::from_str::<Settings>(&settings)?;
        settings.theme = theme.name.clone();
        std::fs::write(&settings_path, serde_json::to_vec_pretty(&settings)?)?;
    }

    std::fs::write(themes_path.join("recol.json"), content)
}
