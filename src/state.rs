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
