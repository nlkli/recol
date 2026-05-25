use std::{
    fs,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

const TMP_STORE_DIR: &str = "/tmp/recol.tmpstore";
const THEME_HISTORY_CAP: usize = 64;
const FONT_HISTORY_CAP: usize = 16;

#[inline(always)]
pub fn init() {
    let _ = fs::create_dir(TMP_STORE_DIR);
}

pub fn read_theme_history(limit: usize) -> Vec<String> {
    if let Ok(file) = fs::File::open(PathBuf::from(TMP_STORE_DIR).join("theme.history")) {
        let reader = BufReader::new(file);
        return reader
            .lines()
            .take(limit)
            .map(|l| l.ok().unwrap_or_default())
            .filter(|l| !l.is_empty())
            .collect();
    }
    Vec::default()
}

pub fn append_theme_history(theme_name: &str) {
    let lines = read_theme_history(THEME_HISTORY_CAP);
    if let Ok(file) = fs::File::create(PathBuf::from(TMP_STORE_DIR).join("theme.history")) {
        let mut writer = BufWriter::new(file);
        let _ = writeln!(&mut writer, "{}", theme_name);
        for line in lines {
            let _ = writeln!(&mut writer, "{}", line);
        }
    }
}

pub fn read_font_history(limit: usize) -> Vec<String> {
    if let Ok(file) = fs::File::open(PathBuf::from(TMP_STORE_DIR).join("font.history")) {
        let reader = BufReader::new(file);
        return reader
            .lines()
            .take(limit)
            .map(|l| l.ok().unwrap_or_default())
            .filter(|l| !l.is_empty())
            .collect();
    }
    Vec::default()
}

pub fn append_font_history(font_name: &str) {
    let lines = read_font_history(FONT_HISTORY_CAP);
    if let Ok(file) = fs::File::create(PathBuf::from(TMP_STORE_DIR).join("font.history")) {
        let mut writer = BufWriter::new(file);
        let _ = writeln!(&mut writer, "{}", font_name);
        for line in lines {
            let _ = writeln!(&mut writer, "{}", line);
        }
    }
}
