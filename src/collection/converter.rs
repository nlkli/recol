//! Alacritty color scheme converter.
//!
//! This module converts color definitions from Alacritty `.toml` configuration
//! files into the internal [`Theme`] and [`ColorScheme`] formats.
//!
//! It is intended to build a collection of themes from the Alacritty color
//! scheme repository:
//! https://github.com/mbadolato/iTerm2-Color-Schemes/tree/master/alacritty
//!
//! Each Alacritty config is parsed, validated, and mapped to a fixed color order
//! expected by the theme system. Missing or invalid required fields result in
//! an error.
//!
//! The module supports:
//! - converting a single Alacritty config into a `Theme`
//! - writing a theme directly to an output writer
//! - processing a directory of Alacritty configs to build a theme collection

use crate::collection::theme::{COLOR_SCHEME_NC, ColorScheme, Theme};
use crate::utils::{io_other_error, missing_field};
use crate::{color, color::Color, models, require_field};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

/// Reads an Alacritty `.toml` config and writes the theme bytes to the writer.
pub fn write_theme_from_alacritty<P: AsRef<Path>, W: Write>(
    path: P,
    name: String,
    w: W,
) -> io::Result<()> {
    theme_from_alacritty(path, name)?.write_bytes(w)
}

/// Parses an Alacritty color config and builds a `Theme`.
/// Fails if any required color field is missing or invalid.
pub fn theme_from_alacritty<P: AsRef<Path>>(path: P, name: String) -> io::Result<Theme> {
    let content = fs::read_to_string(path)?;
    let config = toml::from_str::<models::alacritty::Config>(&content).map_err(io_other_error)?;

    let mut colors = Vec::with_capacity(COLOR_SCHEME_NC);

    let colors_cfg = require_field!(config, "colors", colors)?;
    let primary = require_field!(colors_cfg, "colors.primary", primary)?;

    colors.push(color!(require_field!(
        primary,
        "colors.primary.background",
        background
    )?));
    colors.push(color!(require_field!(
        primary,
        "colors.primary.foreground",
        foreground
    )?));

    let selection = require_field!(colors_cfg, "colors.selection", selection)?;

    colors.push(color!(require_field!(
        selection,
        "colors.selection.background",
        background
    )?));
    colors.push(color!(require_field!(
        selection,
        "colors.selection.text",
        text
    )?));

    let cursor = require_field!(colors_cfg, "colors.cursor", cursor)?;

    colors.push(color!(require_field!(
        cursor,
        "colors.cursor.cursor",
        cursor
    )?));
    colors.push(color!(require_field!(cursor, "colors.cursor.text", text)?));

    let normal = require_field!(colors_cfg, "colors.normal", normal)?;

    colors.push(color!(require_field!(
        normal,
        "colors.normal.black",
        black
    )?));
    colors.push(color!(require_field!(normal, "colors.normal.red", red)?));
    colors.push(color!(require_field!(
        normal,
        "colors.normal.green",
        green
    )?));
    colors.push(color!(require_field!(
        normal,
        "colors.normal.yellow",
        yellow
    )?));
    colors.push(color!(require_field!(normal, "colors.normal.blue", blue)?));
    colors.push(color!(require_field!(
        normal,
        "colors.normal.magenta",
        magenta
    )?));
    colors.push(color!(require_field!(normal, "colors.normal.cyan", cyan)?));
    colors.push(color!(require_field!(
        normal,
        "colors.normal.white",
        white
    )?));

    let bright = require_field!(colors_cfg, "colors.bright", normal)?;

    colors.push(color!(require_field!(
        bright,
        "colors.bright.black",
        black
    )?));
    colors.push(color!(require_field!(bright, "colors.bright.red", red)?));
    colors.push(color!(require_field!(
        bright,
        "colors.bright.green",
        green
    )?));
    colors.push(color!(require_field!(
        bright,
        "colors.bright.yellow",
        yellow
    )?));
    colors.push(color!(require_field!(bright, "colors.bright.blue", blue)?));
    colors.push(color!(require_field!(
        bright,
        "colors.bright.magenta",
        magenta
    )?));
    colors.push(color!(require_field!(bright, "colors.bright.cyan", cyan)?));
    colors.push(color!(require_field!(
        bright,
        "colors.bright.white",
        white
    )?));

    let scheme = ColorScheme::from_color_slice(crate::utils::as_array_ref(&colors));
    Ok(Theme::new(name, scheme))
}

/// Reads all Alacritty `.toml` files in a directory and writes each theme.
pub fn write_themes_from_alacritty_dir<P: AsRef<Path>, W: Write>(
    dir: P,
    mut w: W,
) -> io::Result<()> {
    const FILET_EXT: &str = ".toml";
    let dir = fs::read_dir(dir)?;
    for entry in dir {
        let entry = entry?;
        let file_name = entry.file_name();
        if let Ok(file_name) = file_name.into_string() {
            let theme_name = file_name.trim_end_matches(FILET_EXT).to_string();
            let path = entry.path();
            write_theme_from_alacritty(path, theme_name, &mut w)?;
        }
    }
    Ok(())
}
