// Safety! single thread :)
#![allow(static_mut_refs)]

use std::{
    env, fs,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

const APP_STATE_DIRNAME: &str = "recol";
const THEME_HISTORY_FILE: &str = "theme.history";
const FONT_HISTORY_FILE: &str = "font.history";

pub const THEME_HISTORY_CAP: usize = 128;
pub const FONT_HISTORY_CAP: usize = 16;

/// Returns the lazily initialized app state directory.
fn state_dir() -> &'static PathBuf {
    static mut STATE_DIR: Option<PathBuf> = None;

    unsafe {
        STATE_DIR.get_or_insert_with(|| {
            let default_state_dir = || {
                if let Ok(state_dir) = env::var("RECOL_STATE_DIR") {
                    let path = PathBuf::from(state_dir);
                    if path.is_dir() {
                        return path;
                    }
                }

                let base = env::var_os("XDG_STATE_HOME")
                    .map(PathBuf::from)
                    .or_else(|| {
                        env::home_dir().map(|home| PathBuf::from(home).join(".local/state"))
                    })
                    .unwrap_or_else(env::temp_dir);

                base.join(APP_STATE_DIRNAME)
            };

            let state_dir = default_state_dir();
            let _ = fs::create_dir_all(&state_dir);
            state_dir
        })
    }
}

static mut THEME_HISTORY_BUFF: Vec<String> = Vec::new();
static mut FONT_HISTORY_BUFF: Vec<String> = Vec::new();

/// Reads up to `limit` non-empty lines from a history file.
/// Existing entries in `buff` are cleared before reading.
fn read_history(file_name: &str, limit: usize, buff: &mut Vec<String>) {
    let Ok(file) = fs::File::open(state_dir().join(file_name)) else {
        return;
    };
    buff.clear();
    BufReader::new(file)
        .lines()
        .take(limit)
        .map(|l| l.unwrap_or_default())
        .filter(|l| !l.is_empty())
        .for_each(|l| buff.push(l));
}

/// Prepends `entry` to a history file, keeping at most `cap` entries.
/// Skips the write if `entry` is already the most recent entry.
fn append_history(file_name: &str, entry: &str, cap: usize, buff: &mut Vec<String>) {
    read_history(file_name, cap, buff);
    if buff.first().map(String::as_str) == Some(entry) {
        return;
    }
    let dir = state_dir();
    let Ok(file) = fs::File::create(dir.join(file_name)) else {
        return;
    };
    let mut writer = BufWriter::new(file);
    let _ = writeln!(writer, "{}", entry);
    for line in buff {
        let _ = writeln!(writer, "{}", line);
    }
    let _ = writer.flush();
}

#[inline]
pub fn read_theme_history(limit: usize) -> &'static [String] {
    unsafe {
        read_history(THEME_HISTORY_FILE, limit, &mut THEME_HISTORY_BUFF);
        &THEME_HISTORY_BUFF
    }
}

#[inline]
pub fn append_theme_history(theme_name: &str) {
    unsafe {
        append_history(
            THEME_HISTORY_FILE,
            theme_name,
            THEME_HISTORY_CAP,
            &mut THEME_HISTORY_BUFF,
        );
    }
}

#[inline]
pub fn read_font_history(limit: usize) -> &'static [String] {
    unsafe {
        read_history(FONT_HISTORY_FILE, limit, &mut FONT_HISTORY_BUFF);
        &FONT_HISTORY_BUFF
    }
}

#[inline]
pub fn append_font_history(font_name: &str) {
    unsafe {
        append_history(
            FONT_HISTORY_FILE,
            font_name,
            FONT_HISTORY_CAP,
            &mut FONT_HISTORY_BUFF,
        );
    }
}

// ---------------------------------------------------------------------------
// Favorites
//
// Unlike history, favorites are a hand-curated list: entries are toggled
// explicitly and never evicted by use. They live in the config dir rather than
// the state dir so they can be version-controlled with the user's dotfiles.

const APP_CONFIG_DIRNAME: &str = "recol";
const FAVORITES_FILE: &str = "favorites";

pub const FAVORITES_CAP: usize = 512;

static mut FAVORITES_BUFF: Vec<String> = Vec::new();

/// Returns the lazily initialized app config directory.
fn config_dir() -> &'static PathBuf {
    static mut CONFIG_DIR: Option<PathBuf> = None;

    unsafe {
        CONFIG_DIR.get_or_insert_with(|| {
            if let Ok(dir) = env::var("RECOL_CONFIG_DIR") {
                let path = PathBuf::from(dir);
                if path.is_dir() {
                    return path;
                }
            }

            let base = env::var_os("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .or_else(|| env::home_dir().map(|home| PathBuf::from(home).join(".config")))
                .unwrap_or_else(env::temp_dir);

            let config_dir = base.join(APP_CONFIG_DIRNAME);
            let _ = fs::create_dir_all(&config_dir);
            config_dir
        })
    }
}

/// Splits the favorites file into its leading comment block and its entries,
/// so a rewrite keeps any header the user put there by hand.
fn read_favorites_file() -> (Vec<String>, Vec<String>) {
    let mut preamble = Vec::new();
    let mut entries = Vec::new();

    let Ok(file) = fs::File::open(config_dir().join(FAVORITES_FILE)) else {
        return (preamble, entries);
    };
    for line in BufReader::new(file).lines().map_while(Result::ok) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            if entries.is_empty() {
                preamble.push(line);
            }
            continue;
        }
        if entries.len() < FAVORITES_CAP {
            entries.push(trimmed.to_string());
        }
    }
    (preamble, entries)
}

fn write_favorites_file(preamble: &[String], entries: &[String]) {
    let Ok(file) = fs::File::create(config_dir().join(FAVORITES_FILE)) else {
        return;
    };
    let mut writer = BufWriter::new(file);
    for line in preamble.iter().chain(entries.iter()) {
        let _ = writeln!(writer, "{}", line);
    }
    let _ = writer.flush();
}

#[inline]
pub fn read_theme_favorites() -> &'static [String] {
    unsafe {
        let (_, entries) = read_favorites_file();
        FAVORITES_BUFF = entries;
        &FAVORITES_BUFF
    }
}

/// Adds `theme_name` to the favorites, or removes it when already present.
/// Returns `true` when the theme is a favorite afterwards.
pub fn toggle_theme_favorite(theme_name: &str) -> bool {
    let (preamble, mut entries) = read_favorites_file();
    let favorited = match entries.iter().position(|n| n == theme_name) {
        Some(idx) => {
            entries.remove(idx);
            false
        }
        None => {
            if entries.len() >= FAVORITES_CAP {
                return false;
            }
            entries.push(theme_name.to_string());
            true
        }
    };
    write_favorites_file(&preamble, &entries);
    unsafe {
        FAVORITES_BUFF = entries;
    }
    favorited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_adds_then_removes() {
        let dir = env::temp_dir().join(format!("recol-fav-test-{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        unsafe { env::set_var("RECOL_CONFIG_DIR", &dir) };
        let _ = fs::write(dir.join("favorites"), "# my favorites\n");

        assert!(toggle_theme_favorite("Aura"), "first toggle favorites it");
        assert_eq!(read_theme_favorites(), &["Aura".to_string()]);

        assert!(toggle_theme_favorite("Nord"));
        assert_eq!(read_theme_favorites().len(), 2);

        assert!(!toggle_theme_favorite("Aura"), "second toggle unfavorites");
        assert_eq!(read_theme_favorites(), &["Nord".to_string()]);

        let text = fs::read_to_string(dir.join("favorites")).unwrap();
        assert!(text.starts_with("# my favorites"), "header is preserved: {text:?}");

        let _ = fs::remove_dir_all(&dir);
    }
}
