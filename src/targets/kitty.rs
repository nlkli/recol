use recol_lib as lib;
use std::{fs, io, path::Path};

pub fn apply_theme_to(path: impl AsRef<Path>, theme: &lib::Theme) -> io::Result<()> {
    // Match ColorScheme::as_colors_array(): UI colors, then ANSI colors 0–15.
    let keys: Vec<String> = [
        "background",
        "foreground",
        "selection_background",
        "selection_foreground",
        "cursor",
        "cursor_text_color",
    ]
    .into_iter()
    .map(str::to_owned)
    .chain((0..16).map(|i| format!("color{i}")))
    .collect();

    let config = fs::read_to_string(&path)?;
    let mut output = String::new();
    for line in config.split_inclusive('\n') {
        let key = line.split_whitespace().next().unwrap_or_default();
        if !keys.iter().any(|k| k == key) {
            output.push_str(line);
        }
    }
    if !output.is_empty() && !output.ends_with('\n') {
        output.push('\n');
    }
    // Append after includes so an included theme cannot override these colors.
    for (key, color) in keys.iter().zip(theme.colors.as_colors_array()) {
        output.push_str(&format!("{key} {color}\n"));
    }
    fs::write(path, output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_all_colors_after_includes_and_preserves_other_settings() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("kitty.conf");
        let preserved =
            "# foreground red\r\n  font_size 12\r\n\ncolor16 #abcdef\ninclude theme.conf\n";
        fs::write(
            &path,
            format!("foreground red\n\tcolor0 black\n{preserved}foreground blue"),
        )
        .unwrap();
        let colors = std::array::from_fn(|i| lib::Color::from_rgb(i as u8, 0, 0));
        let theme = lib::Theme::new("test", false, lib::ColorScheme::from_color_slice(&colors));
        apply_theme_to(&path, &theme).unwrap();
        let output = fs::read_to_string(&path).unwrap();
        let mut expected = preserved.to_owned();
        expected.push_str("background #000000\nforeground #010000\nselection_background #020000\nselection_foreground #030000\ncursor #040000\ncursor_text_color #050000\n");
        for i in 0..16 {
            expected.push_str(&format!("color{i} #{:02x}0000\n", i + 6));
        }
        assert_eq!(output, expected);
        apply_theme_to(&path, &theme).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), output);
    }
}
