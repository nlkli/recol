use std::{io, fs, path::Path};

// ANSI Table
// 0  black
// 1  red
// 2  green
// 3  yellow
// 4  blue
// 5  magenta
// 6  cyan
// 7  white

#[derive(Deserialize, Debug)]
pub struct AnsiColors {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String,
}

#[derive(Deserialize, Debug)]
pub struct CursorColors {
    cursor: String,
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct PrimaryColors {
    background: String,
    foreground: String,
}

#[derive(Daserialize, Debug)]
pub struct SelectionColors {
    background: String,
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct Colors {
    primary: Primary,
    normal: TermColors,
    bright: TermColors,
    cursor: Cursor,
    selection: Selection,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    colors: Colors,
}

pub fn alacritty_colors_to_bytes() {

}

// [HEADER_SIZE u32] [HEADER] [PAYLOAD]
// HEADER -> [ [NAME_SIZE u8] [NAME] [PROPERTY] .. ]

// https://github.com/mbadolato/iTerm2-Color-Schemes/tree/master/alacritty
pub fn from_alacritty_colors_config<P: AsRef<Path>>(src_dir_path: P, dst_bin_file: P) -> Result<(), io::Error> {
    const FILET_EXT: &str = ".toml";
    let dir = fs::read_dir(src_dir_path)?;
    for entry in dir {
        let file_name = entry?.file_name();
        if let Ok(file_name) = file_name.into_string() {
            let _theme_name = file_name.trim_end_matches(FILET_EXT);
        }
    }
    Ok(())
}
