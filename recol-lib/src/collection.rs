use crate::{COLOR_SCHEME_SIZE, ColorScheme, Theme};

/// File layout:
/// 1. `count: u16` — number of themes
/// 2. `offsets: [u32; count]` — offsets of each theme relative to the themes section
/// 3. `themes: Theme bytes...` — each theme:
///    - `[NAME_SIZE u8]`
///    - `[NAME utf8 string]`
///    - `[IS_LIGHT u8]`
///    - `[COLOR_SCHEME bytes]`
///
pub const COLOR_SCHEMES: &[u8] = include_bytes!("colorschemes.bin");
const CS: &[u8] = COLOR_SCHEMES;

#[inline(always)]
fn count() -> usize {
    u16::from_be_bytes([CS[0], CS[1]]) as usize
}

#[inline(always)]
fn start_themes() -> usize {
    2 + count() * 4
}

#[inline(always)]
fn offset_by_index(i: usize) -> usize {
    let b = &CS[2 + i * 4..];
    u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize
}

#[inline(always)]
fn start_theme_bytes_by_index(i: usize) -> &'static [u8] {
    &CS[start_themes() + offset_by_index(i)..]
}

#[derive(Clone, Copy)]
pub struct LazyTheme {
    name: &'static str,
    is_light: bool,
    color_scheme_bytes: &'static [u8],
}

impl Into<Theme> for LazyTheme {
    fn into(self) -> Theme {
        Theme::new(
            self.name,
            self.is_light,
            ColorScheme::try_from_bytes(self.color_scheme_bytes).expect("it's ok"),
        )
    }
}

impl LazyTheme {
    fn from_bytes(b: &'static [u8]) -> Self {
        let name_size = b[0] as usize;
        let name = str::from_utf8(&b[1..1 + name_size]).expect("must be valid utf8");
        let is_light = b[1 + name_size] != 0;
        let color_scheme_bytes = &b[1 + name_size + 1..1 + name_size + 1 + COLOR_SCHEME_SIZE];
        Self {
            name,
            is_light,
            color_scheme_bytes,
        }
    }

    pub fn into_theme(self) -> Theme {
        self.into()
    }
}

#[derive(Clone)]
pub struct LazyThemeIter {
    total: usize,
    index: usize,
}

impl LazyThemeIter {
    pub fn new() -> Self {
        Self {
            index: 0,
            total: count(),
        }
    }
}

impl Iterator for LazyThemeIter {
    type Item = LazyTheme;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.total {
            return None;
        }
        let item = LazyTheme::from_bytes(start_theme_bytes_by_index(self.index));
        self.index += 1;
        Some(item)
    }
}

pub struct Collection {}

impl Collection {
    pub fn iter() -> LazyThemeIter {
        LazyThemeIter::new()
    }
}
