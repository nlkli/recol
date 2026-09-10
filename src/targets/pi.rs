use recol_lib as lib;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{io, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,

    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

pub fn apply_theme_to(path: impl AsRef<Path>, theme: &lib::Theme) -> io::Result<()> {
    let c = theme.colors.clone().into_advanced(None);

    let variant = if theme.is_light { &c.dim } else { &c.bright };

    let content = format!(
        r###"{{
  "$schema": "https://raw.githubusercontent.com/earendil-works/pi/main/packages/coding-agent/src/modes/interactive/theme/theme-schema.json",
  "name": "{theme_name}",
  "colors": {{
    "accent": "{blue_variant}",
    "border": "{fg3}",
    "borderAccent": "{blue_base}",
    "borderMuted": "{bg3}",
    "success": "{green_base}",
    "error": "{red_base}",
    "warning": "{yellow_base}",
    "muted": "{fg2}",
    "dim": "{fg3}",
    "text": "{fg1}",
    "thinkingText": "{comment}",

    "selectedBg": "{sel0}",
    "scrollbarThumb": "{sel0}",
    "searchMatchBg": "{sel0}",
    "searchMatchText": "{fg1}",
    "userMessageBg": "{bg2}",
    "userMessageText": "{fg1}",
    "customMessageBg": "{bg3}",
    "customMessageText": "{fg1}",
    "customMessageLabel": "{blue_variant}",
    "toolPendingBg": "{bg0}",
    "toolSuccessBg": "{diff_add}",
    "toolErrorBg": "{diff_delete}",
    "toolTitle": "{blue_variant}",
    "toolOutput": "{fg2}",

    "mdHeading": "{blue_variant}",
    "mdLink": "{magenta_base}",
    "mdLinkUrl": "{orange_variant}",
    "mdCode": "{cyan_base}",
    "mdCodeBlock": "{pink_base}",
    "mdCodeBlockBorder": "{fg3}",
    "mdQuote": "{fg2}",
    "mdQuoteBorder": "{bg3}",
    "mdHr": "{bg4}",
    "mdListBullet": "{cyan_variant}",

    "toolDiffAdded": "{green_base}",
    "toolDiffRemoved": "{red_base}",
    "toolDiffContext": "{comment}",

    "syntaxComment": "{comment}",
    "syntaxKeyword": "{magenta_base}",
    "syntaxFunction": "{blue_variant}",
    "syntaxVariable": "{fg1}",
    "syntaxString": "{green_base}",
    "syntaxNumber": "{orange_base}",
    "syntaxType": "{yellow_base}",
    "syntaxOperator": "{fg2}",
    "syntaxPunctuation": "{fg2}",

    "thinkingOff": "{bg4}",
    "thinkingMinimal": "{comment}",
    "thinkingLow": "{blue_base}",
    "thinkingMedium": "{cyan_base}",
    "thinkingHigh": "{magenta_base}",
    "thinkingXhigh": "{orange_base}",
    "thinkingMax": "{red_base}",

    "bashMode": "{orange_variant}"
  }}
}}
"###,
        theme_name = theme.name,
        blue_variant = variant.blue,
        fg3 = c.fg[3],
        blue_base = c.base.blue,
        bg3 = c.bg[3],
        green_base = c.base.green,
        red_base = c.base.red,
        yellow_base = c.base.yellow,
        fg2 = c.fg[2],
        fg1 = c.fg[1],
        comment = c.comment,
        sel0 = c.alt_selection[0],
        bg2 = c.bg[2],
        bg0 = c.bg[0],
        diff_add = c.diff.add,
        diff_delete = c.diff.delete,
        magenta_base = c.base.magenta,
        orange_variant = variant.orange,
        cyan_base = c.base.cyan,
        pink_base = c.base.pink,
        bg4 = c.bg[4],
        cyan_variant = variant.cyan,
        orange_base = c.base.orange,
    );

    let settings_path = path.as_ref().join("settings.json");
    if let Ok(settings) = std::fs::read_to_string(&settings_path) {
        let mut settings = serde_json::from_str::<Settings>(&settings)?;
        settings.theme = theme.name.clone();
        std::fs::write(&settings_path, serde_json::to_vec_pretty(&settings)?)?;
    }

    let themes_path = path.as_ref().join("themes");
    std::fs::create_dir_all(&themes_path)?;

    std::fs::write(themes_path.join("recol.json"), content)
}
