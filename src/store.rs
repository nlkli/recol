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

/// Resolves the app's state directory per the XDG Base Directory spec:
/// `$XDG_STATE_HOME/recol`, falling back to `$HOME/.local/state/recol`,
/// and finally to the system temp dir if `$HOME` is unavailable.
fn store_dir() -> PathBuf {
    let base = env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| env::home_dir().map(|home| PathBuf::from(home).join(".local/state")))
        .unwrap_or_else(env::temp_dir);

    base.join(APP_STATE_DIRNAME)
}

/// Initializes the storage directory. Safe to call multiple times.
#[inline(always)]
pub fn init() {
    let _ = fs::create_dir_all(store_dir());
}

/// Reads up to `limit` non-empty lines from `file_name` in the store dir.
fn read_history(file_name: &str, limit: usize) -> Vec<String> {
    let Ok(file) = fs::File::open(store_dir().join(file_name)) else {
        return Vec::new();
    };
    BufReader::new(file)
        .lines()
        .take(limit)
        .map(|l| l.unwrap_or_default())
        .filter(|l| !l.is_empty())
        .collect()
}

/// Prepends `entry` to `file_name`, keeping at most `cap` previous entries.
/// Skips writing if `entry` is identical to the most recent (top) entry.
fn append_history(file_name: &str, entry: &str, cap: usize) {
    let previous = read_history(file_name, cap);

    if previous.first().map(String::as_str) == Some(entry) {
        return;
    }

    let dir = store_dir();
    if fs::create_dir_all(&dir).is_err() {
        return;
    }

    let Ok(file) = fs::File::create(dir.join(file_name)) else {
        return;
    };
    let mut writer = BufWriter::new(file);
    let _ = writeln!(writer, "{}", entry);
    for line in previous {
        let _ = writeln!(writer, "{}", line);
    }
    let _ = writer.flush();
}

pub fn read_theme_history(limit: usize) -> Vec<String> {
    read_history(THEME_HISTORY_FILE, limit)
}

pub fn append_theme_history(theme_name: &str) {
    append_history(THEME_HISTORY_FILE, theme_name, THEME_HISTORY_CAP);
}

pub fn read_font_history(limit: usize) -> Vec<String> {
    read_history(FONT_HISTORY_FILE, limit)
}

pub fn append_font_history(font_name: &str) {
    append_history(FONT_HISTORY_FILE, font_name, FONT_HISTORY_CAP);
}
