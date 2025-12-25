//! Alacritty color scheme converter (build-time).
//!
//! This module is used during development to generate a binary collection
//! of color themes (`colorschemes.bin`) from Alacritty `.toml` configuration
//! files.
//!
//! The input themes are sourced from the Alacritty color scheme repository:
//! https://github.com/mbadolato/iTerm2-Color-Schemes/tree/master/alacritty
//!
//! Each `.toml` file is parsed, validated, converted into the internal
//! [`Theme`] representation, and serialized into a compact binary format.
//!
//! The resulting `colorschemes.bin` file is embedded into the final build
//! and used at runtime. This module itself is not required after the binary
//! asset has been generated.

use crate::collection::theme::{COLOR_SCHEME_NC, ColorScheme, Theme};
use crate::utils::{io_other_error, missing_field};
use crate::{color, color::Color, models, require_field};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

/// Generates the `colorschemes.bin` file from a directory of Alacritty themes.
///
/// This function is intended to be used at build time only. It reads all
/// Alacritty `.toml` color schemes from the given directory, converts them
/// into the internal `Theme` format, and serializes them into a single
/// binary file.
///
/// File layout:
/// - `u16` theme count
/// - `u32[]` offset table (relative to the start of the theme data section)
/// - serialized `Theme` entries
///
/// The resulting file is written to `src/collection/colorschemes.bin` and
/// embedded into the final build using `include_bytes!`.
pub fn create_colorshemes_bin<P: AsRef<Path>>(dir: P) -> io::Result<()> {
    let mut colorschemes_bin = fs::File::create("src/collection/colorschemes.bin")?;
    let mut buf = Vec::new();
    let (count, offsets) = build_theme_bundle(dir, &mut buf)?;
    colorschemes_bin.write_all(&count.to_be_bytes())?;
    for offset in offsets.iter() {
        colorschemes_bin.write_all(&offset.to_be_bytes())?;
    }
    colorschemes_bin.write_all(&buf)?;

    Ok(())
}

/// Builds a binary theme bundle from a directory of Alacritty configs.
///
/// Returns:
/// - number of themes
/// - offset table (byte offsets of each theme)
fn build_theme_bundle<P: AsRef<Path>, W: Write>(
    dir: P,
    mut w: W,
) -> io::Result<(u16, Vec<u32>)> {
    const TOML_EXT: &str = ".toml";

    let mut count = 0u16;
    let mut offset = 0u32;
    let mut offsets = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        let Some(name) = path
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(|n| n.strip_suffix(TOML_EXT))
        else {
            continue;
        };

        offsets.push(offset);
        let theme = parse_alacritty_theme(&path, name.to_string())?;
        let size = theme.write_bytes(&mut w)?;
        offset += size as u32;
        count += 1;
    }
    Ok((count, offsets))
}

/// Parses a single Alacritty `.toml` file into a `Theme`.
fn parse_alacritty_theme<P: AsRef<Path>>(
    path: P,
    name: String,
) -> io::Result<Theme> {
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

