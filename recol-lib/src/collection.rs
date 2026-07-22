//! Binary color scheme collection — zero-allocation, zero-copy access.
//!
//! # Binary layout
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │ Header                                                  │
//! │   count        : u16 BE   — number of themes           │
//! ├─────────────────────────────────────────────────────────┤
//! │ Offset table  (count × 4 bytes)                        │
//! │   offsets      : [u32 BE; count]                       │
//! │                  each value is the byte offset of the  │
//! │                  theme relative to the themes section   │
//! ├─────────────────────────────────────────────────────────┤
//! │ Themes section  (variable length)                      │
//! │   Per theme:                                           │
//! │     name_len   : u8       — length of the name in bytes│
//! │     name       : [u8; name_len]  — UTF-8              │
//! │     is_light   : u8       — 0 = dark, non-zero = light │
//! │     colors     : [u8; COLOR_SCHEME_SIZE]               │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! Themes are sorted alphabetically by name at build time.
//! All byte sequences are guaranteed valid; panics on corruption are intentional.

use crate::{COLOR_SCHEME_NC, COLOR_SCHEME_SIZE, Color, ColorScheme, Theme};
use std::{
    io::{BufRead, BufReader, Write},
    path::Path,
};

// TODO: remove offst table
/// Source: <https://github.com/mbadolato/iTerm2-Color-Schemes/tree/master/ghostty>
pub const COLOR_SCHEMES: &[u8] = include_bytes!("colorschemes.bin");

/// Number of themes stored in the binary.
#[inline]
fn theme_count() -> usize {
    u16::from_be_bytes([COLOR_SCHEMES[0], COLOR_SCHEMES[1]]) as usize
}

/// Byte offset where the themes section begins (after header + offset table).
#[inline]
fn themes_section_start() -> usize {
    2 + theme_count() * 4
}

/// Byte offset of theme `i` within the themes section.
#[inline]
fn theme_section_offset(i: usize) -> usize {
    let b = &COLOR_SCHEMES[2 + i * 4..];
    u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize
}

/// Raw bytes of theme `i` (starting at its first field).
#[inline]
fn theme_raw(i: usize) -> &'static [u8] {
    &COLOR_SCHEMES[themes_section_start() + theme_section_offset(i)..]
}

/// A theme whose color data is not yet decoded — only name and light/dark flag
/// are held as direct references into the embedded binary.
#[derive(Debug, Clone, Copy)]
pub struct LazyTheme {
    pub name: &'static str,
    pub is_light: bool,
    color_scheme_bytes: &'static [u8],
}

impl PartialEq for LazyTheme {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for LazyTheme {}

impl PartialOrd for LazyTheme {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LazyTheme {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(other.name)
    }
}

impl LazyTheme {
    /// Decode a `LazyTheme` from raw bytes (see binary layout in module docs).
    fn from_bytes(b: &'static [u8]) -> Self {
        let name_len = b[0] as usize;
        // SAFETY: all names are written as valid UTF-8 at build time.
        let name = unsafe { std::str::from_utf8_unchecked(&b[1..1 + name_len]) };
        let is_light = b[1 + name_len] != 0;
        let colors_start = 1 + name_len + 1;
        let color_scheme_bytes = &b[colors_start..colors_start + COLOR_SCHEME_SIZE];
        Self {
            name,
            is_light,
            color_scheme_bytes,
        }
    }

    /// Decode the full [`Theme`], including its color data.
    pub fn into_theme(self) -> Theme {
        Theme::new(
            self.name,
            self.is_light,
            ColorScheme::try_from_bytes(self.color_scheme_bytes)
                .expect("color scheme bytes are always valid"),
        )
    }
}

impl From<LazyTheme> for Theme {
    fn from(lazy: LazyTheme) -> Theme {
        lazy.into_theme()
    }
}

/// Predicate used to filter themes inside a [`Collection`].
#[derive(Default, Clone, Copy, Debug)]
pub enum ThemeFilter<'a> {
    /// Accept every theme (default).
    #[default]
    None,
    /// Only light themes.
    Light,
    /// Only dark themes.
    Dark,
    /// Themes whose name contains the given substring.
    Contains(&'a str),
    /// Theme whose name start with
    StartWith(&'a str),

    ContainsLower(&'a str),
    StartWithLower(&'a str),

    /// Arbitrary predicate.
    Custom(fn(&LazyTheme) -> bool),
}

impl<'a> ThemeFilter<'a> {
    #[inline]
    pub fn matches(&self, t: &LazyTheme) -> bool {
        match self {
            Self::None => true,
            Self::Light => t.is_light,
            Self::Dark => !t.is_light,
            Self::Contains(s) => t.name.contains(s),
            Self::StartWith(s) => t.name.starts_with(s),
            Self::ContainsLower(s) => t.name.to_lowercase().contains(s),
            Self::StartWithLower(s) => t.name.to_lowercase().starts_with(s),
            Self::Custom(f) => f(t),
        }
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

/// Lazy, zero-allocation iterator over the embedded theme collection.
///
/// Call [`Collection::new`] to start from the beginning, then use the
/// standard [`Iterator`] API or the convenience methods below.
#[derive(Clone, Copy)]
pub struct Collection {
    total: usize,
    index: usize,
}

impl Collection {
    pub fn new() -> Self {
        Self {
            index: 0,
            total: theme_count(),
        }
    }

    /// Reset the internal cursor so the collection can be iterated again.
    #[inline]
    pub fn reset(&mut self) {
        self.index = 0;
    }

    /// Look up a theme by its exact position in the sorted list.
    pub fn by_index(i: usize) -> Option<LazyTheme> {
        (i < theme_count()).then(|| LazyTheme::from_bytes(theme_raw(i)))
    }

    /// Look up a theme by its exact name.
    pub fn by_name(&mut self, name: &str) -> Option<LazyTheme> {
        self.reset();
        self.find(|t| t.name == name)
    }

    /// Pick a uniformly random theme among those matching `filters`
    /// (reservoir sampling — single pass, no allocation).
    pub fn random(&mut self, filters: &[ThemeFilter<'_>]) -> Option<LazyTheme> {
        self.reset();
        let mut chosen = None;
        let mut seen = 0usize;
        for theme in self.filtered(filters) {
            seen += 1;
            if fastrand::usize(..seen) == 0 {
                chosen = Some(theme);
            }
        }
        chosen
    }

    /// Collect the names of all themes matching `filters`.
    pub fn name_list(&mut self, filters: &[ThemeFilter<'_>]) -> Vec<&'static str> {
        self.reset();
        self.filtered(filters).map(|t| t.name).collect()
    }

    /// Find the best fuzzy match for `query` among themes matching `filters`.
    pub fn fuzzy_search(&mut self, query: &str, filters: &[ThemeFilter<'_>], min_score: Option<f64>) -> Option<LazyTheme> {
        let candidates = self.name_list(filters);
        crate::fuzzy::search(query, &candidates, min_score).and_then(|name| self.by_name(name))
    }

    /// Find the best tip n fuzzy match for `query` among themes matching `filters`.
    pub fn fuzzy_search_top_n(
        &mut self,
        query: &str,
        filters: &[ThemeFilter<'_>],
        limit: usize,
        min_score: Option<f64>,
    ) -> Vec<LazyTheme> {
        let candidates = self.name_list(filters);
        crate::fuzzy::search_top_n(query, &candidates, limit, min_score)
            .into_iter()
            .map(|name| self.by_name(name).unwrap())
            .collect()
    }

    /// Iterator adapter that applies all filters in `filters`.
    pub fn filtered<'a>(
        &'a mut self,
        filters: &'a [ThemeFilter<'_>],
    ) -> impl Iterator<Item = LazyTheme> + 'a {
        self.filter(move |t| filters.iter().all(|f| f.matches(t)))
    }
}

impl Default for Collection {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Collection {
    type Item = LazyTheme;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.total {
            return None;
        }
        let item = LazyTheme::from_bytes(theme_raw(self.index));
        self.index += 1;
        Some(item)
    }
}

/// Build `colorschemes.bin` from a directory of Ghostty theme files.
///
/// `filter_by_name` lets callers exclude files by name (e.g. hidden files).
/// Files are sorted alphabetically by name before parsing, so offsets are
/// computed in a single forward pass — no reordering or recomputation needed.
pub fn build_colorschemes_bin(
    dir_path: impl AsRef<Path>,
    mut out: impl Write,
    filter_by_name: fn(&str) -> bool,
) -> std::io::Result<()> {
    let mut files = Vec::new();

    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        let Some(file_name) = path.file_name() else {
            continue;
        };
        files.push((file_name.to_string_lossy().to_string(), path));
    }

    files.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut count = 0u16;
    let mut offset = 0u32;
    let mut offsets_bytes = Vec::new();
    let mut theme_list_bytes = Vec::new();

    for (name, path) in files.into_iter() {
        if !filter_by_name(&name) {
            continue;
        }
        let bytes = parse_ghostty_theme(&path, &name)?.bytes();
        theme_list_bytes.extend_from_slice(&bytes);
        offsets_bytes.extend_from_slice(&offset.to_be_bytes());
        offset += bytes.len() as u32;
        count += 1;
    }

    out.write_all(&count.to_be_bytes())?;
    out.write_all(&offsets_bytes)?;
    out.write_all(&theme_list_bytes)?;
    out.flush()?;

    Ok(())
}

fn parse_ghostty_theme(path: impl AsRef<Path>, name: &str) -> std::io::Result<Theme> {
    let reader = BufReader::new(std::fs::File::open(path)?);
    let mut colors = [Color::default(); COLOR_SCHEME_NC];

    for line in reader.lines() {
        let line = line?;
        let mut kv = line.splitn(2, '=');
        let (Some(key), Some(value)) = (kv.next(), kv.next()) else {
            continue;
        };
        let (key, value) = (key.trim(), value.trim());

        match key {
            "background" => colors[0] = value.parse().expect("background color"),
            "foreground" => colors[1] = value.parse().expect("foreground color"),
            "selection-background" => colors[2] = value.parse().expect("selection-background"),
            "selection-foreground" => colors[3] = value.parse().expect("selection-foreground"),
            "cursor-color" => colors[4] = value.parse().expect("cursor-color"),
            "cursor-text" => colors[5] = value.parse().expect("cursor-text"),
            "palette" => {
                // Format: `palette = <index>=<#rrggbb>`
                let mut parts = value.splitn(2, '=');
                let index: usize = parts
                    .next()
                    .and_then(|s| s.trim().parse().ok())
                    .expect("palette index");
                let color: Color = parts
                    .next()
                    .and_then(|s| s.trim().parse().ok())
                    .expect("palette color");
                // Palette entries 0–15 map to colors[6..22].
                if index < 16 {
                    colors[6 + index] = color;
                }
            }
            _ => {}
        }
    }

    let scheme = ColorScheme::from_color_slice(&colors);
    let is_light = scheme.bg.color().hsl().2 > 50.0;
    Ok(Theme::new(name, is_light, scheme))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_themes_decode_without_panic() {
        let mut decoded = 0;
        for lazy in Collection::new() {
            let _ = lazy.into_theme().colors.into_advanced(None);
            decoded += 1;
        }
        assert_eq!(decoded, theme_count());
    }
}
