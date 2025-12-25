#[cfg(debug_assertions)]
pub mod converter;

pub mod theme;
use rand::seq::IteratorRandom;
use std::io;

use crate::utils::{as_array_ref, fuzzy_search};
use theme::{COLOR_SCHEME_SIZE, ColorScheme, Theme};

/// All Alacritty color schemes embedded as a binary bundle.
///
/// `colorschemes.bin` is generated at build time from `.toml` themes
/// (https://github.com/mbadolato/iTerm2-Color-Schemes/tree/master/alacritty).
///
/// File layout:
/// 1. `count: u16` — number of themes
/// 2. `offsets: [u32; count]` — offsets of each theme relative to the themes section
/// 3. `themes: Theme bytes...` — each theme:
///    - `[NAME_SIZE u8]`
///    - `[NAME utf8 string]`
///    - `[IS_LIGHT u8]`
///    - `[COLOR_SCHEME bytes]`
///
/// Included at compile time with `include_bytes!` for runtime deserialization.
pub const COLOR_SCHEMES: &[u8] = include_bytes!("colorschemes.bin");

/// Lightweight view of a theme without fully deserializing colors.
#[derive(Clone)]
pub struct LazyTheme<'a> {
    name: &'a str,
    is_light: bool,
    color_scheme_bytes: &'a [u8],
}

impl<'a> LazyTheme<'a> {
    fn from_bytes(b: &'a [u8]) -> io::Result<Self> {
        let name_size = b[0] as usize;
        let required = 1 + name_size + 1 + COLOR_SCHEME_SIZE;
        if b.len() < required {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid length"));
        }
        let name = str::from_utf8(&b[1..1 + name_size])
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid utf8"))?;
        let is_light = b[1 + name_size] != 0;
        let color_scheme_bytes = &b[2 + name_size..required];

        Ok(Self {
            name,
            is_light,
            color_scheme_bytes,
        })
    }

    pub fn into_theme(&self) -> Theme {
        Theme {
            name: self.name.to_string(),
            is_light: self.is_light,
            colors: ColorScheme::from_bytes(as_array_ref(self.color_scheme_bytes)),
        }
    }
}

#[derive(Clone)]
pub struct LazyThemeIter<'a> {
    store: &'a Collection<'a>,
    index: usize,
}

impl<'a> LazyThemeIter<'a> {
    pub fn new(store: &'a Collection<'a>) -> Self {
        Self { store, index: 0 }
    }
}

impl<'a> Iterator for LazyThemeIter<'a> {
    type Item = LazyTheme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let store = self.store;
        if self.index >= store.offsets.len() {
            return None;
        }

        let start = store.themes_start + store.offsets[self.index] as usize;
        let b = &store.bytes[start..];

        if b.len() < 2 {
            return None;
        }

        let item = LazyTheme::from_bytes(b).ok()?;

        self.index += 1;

        Some(item)
    }
}

/// Collection of embedded themes with lazy access.
#[derive(Clone)]
pub struct Collection<'a> {
    bytes: &'a [u8],
    offsets: Vec<u32>,
    count: u16,
    themes_start: usize,
}

impl<'a> Collection<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        let count = u16::from_be_bytes([bytes[0], bytes[1]]);
        let offsets_start = 2;
        let offsets_end = offsets_start + (count as usize) * 4;
        let offsets = &bytes[offsets_start..offsets_end];
        let themes_start = offsets_end;

        let offsets: Vec<u32> = offsets
            .chunks_exact(4)
            .map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
            .collect();

        Self {
            bytes,
            offsets,
            count,
            themes_start,
        }
    }

    pub fn get(&'a self, index: usize) -> Option<LazyTheme<'a>> {
        if index >= self.count as usize {
            return None;
        }
        let start = self.themes_start + self.offsets[index] as usize;
        LazyTheme::from_bytes(&self.bytes[start..]).ok()
    }

    pub fn iter(&'a self) -> LazyThemeIter<'a> {
        LazyThemeIter::new(self)
    }

    pub fn rand(&'a self) -> Option<LazyTheme<'a>> {
        let i = rand::random_range(0..self.count as usize);
        self.get(i)
    }

    pub fn rand_dark(&'a self) -> Option<LazyTheme<'a>> {
        self.iter().filter(|t| !t.is_light).choose(&mut rand::rng())
    }

    pub fn rand_light(&'a self) -> Option<LazyTheme<'a>> {
        self.iter().filter(|t| t.is_light).choose(&mut rand::rng())
    }

    pub fn by_name(&'a self, name: &str) -> Option<LazyTheme<'a>> {
        self.iter().find(|t| t.name == name)
    }

    pub fn name_list(&'a self) -> Vec<&'a str> {
        let mut list = Vec::with_capacity(self.count as usize);
        self.iter().for_each(|t| list.push(t.name));
        list
    }

    pub fn fuzzy_search(&'a self, query: &str) -> Option<&'a str> {
        let names = self.name_list();
        fuzzy_search(&names, query)
    }
}
