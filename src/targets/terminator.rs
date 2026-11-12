//! Terminator's ConfigObj configuration has nested [profiles] / [[profile]] sections.
//! Update only the color settings of each profile, leaving layouts, keybindings,
//! comments outside replaced color entries, and unrelated settings untouched.
//! https://gnome-terminator.readthedocs.io/en/latest/advancedusage.html

use recol_lib as lib;
use std::{fs, io, path::Path, process::Command};

const COLOR_KEYS: &[&str] = &[
    "use_theme_colors",
    "foreground_color",
    "background_color",
    "palette",
    "color_scheme",        // Legacy schemes may override explicit colors.
    "cursor_color",        // Terminator 1.x (including Ubuntu 20.04).
    "cursor_color_default", // Terminator 2.x.
    "cursor_bg_color",      // Terminator 2.x.
    "cursor_fg_color",      // Terminator 2.x.
];

/// Parse a ConfigObj section heading, including headings with trailing comments.
/// All nesting depths are recognized so nested content is never treated as a
/// profile's own color settings.
fn section_depth(line: &str) -> Option<(usize, &str)> {
    let trimmed = line.trim();
    let depth = trimmed.bytes().take_while(|&byte| byte == b'[').count();
    if depth == 0 {
        return None;
    }

    let closing = depth + trimmed.get(depth..)?.find(']')?;
    let name = trimmed.get(depth..closing)?;
    if name.is_empty() || name.contains('[') || name.contains(']') {
        return None;
    }

    let suffix = trimmed.get(closing..)?;
    if !suffix
        .as_bytes()
        .get(..depth)?
        .iter()
        .all(|&byte| byte == b']')
    {
        return None;
    }
    let rest = suffix.get(depth..)?.trim_start();
    if !rest.is_empty() && !rest.starts_with('#') {
        return None;
    }
    Some((depth, name))
}

fn is_color_key(line: &str) -> bool {
    let trimmed = line.trim_start();
    // A comment such as "# palette = ..." should be preserved.
    if trimmed.starts_with('#') {
        return false;
    }
    trimmed
        .split_once('=')
        .is_some_and(|(key, _)| COLOR_KEYS.contains(&key.trim()))
}

fn theme_lines(theme: &lib::ThemeEx) -> Vec<String> {
    let c = &theme.colors;
    let palette = c
        .as_colors_array()
        .into_iter()
        .skip(6)
        .map(|color| color.to_string())
        .collect::<Vec<_>>()
        .join(":");

    vec![
        "    use_theme_colors = False".to_owned(),
        format!("    foreground_color = \"{}\"", c.fg),
        format!("    background_color = \"{}\"", c.bg),
        format!("    palette = \"{palette}\""),
        // Ubuntu 20.04 uses `cursor_color`. Newer Terminator versions use
        // cursor_color_default plus cursor_bg_color / cursor_fg_color.
        format!("    cursor_color = \"{}\"", c.cursor.bg),
        "    cursor_color_default = False".to_owned(),
        format!("    cursor_bg_color = \"{}\"", c.cursor.bg),
        format!("    cursor_fg_color = \"{}\"", c.cursor.fg),
    ]
}

fn insert_default_profile(output: &mut Vec<String>, color_lines: &[String]) {
    output.push("  [[default]]".to_owned());
    output.extend_from_slice(color_lines);
}

fn update_config(contents: &str, theme: &lib::ThemeEx) -> String {
    let newline = if contents.contains("\r\n") { "\r\n" } else { "\n" };
    let mut output = Vec::<String>::new();
    let colors = theme_lines(theme);
    let mut in_profiles = false;
    let mut in_profile = false;
    let mut found_profiles = false;
    let mut found_any_profile = false;

    for line in contents.lines() {
        let line = line.trim_end_matches('\r');
        if let Some((depth, section)) = section_depth(line) {
            if in_profile {
                // Finish this profile before entering another section.
                output.extend_from_slice(&colors);
                in_profile = false;
            }
            if depth == 1 {
                if in_profiles && !found_any_profile {
                    insert_default_profile(&mut output, &colors);
                }
                in_profiles = section == "profiles";
                if in_profiles {
                    found_profiles = true;
                    found_any_profile = false;
                }
            } else if depth == 2 && in_profiles {
                in_profile = true;
                found_any_profile = true;
            } else if depth > 2 {
                // Do not alter color-like keys in nested configuration sections.
                in_profile = false;
            }
        }
        if in_profile && is_color_key(line) {
            // Re-emit all color keys once at the end of the profile, avoiding
            // duplicates while leaving the surrounding comments intact.
            continue;
        }
        output.push(line.to_owned());
    }

    if in_profile {
        output.extend_from_slice(&colors);
    }
    if in_profiles && !found_any_profile {
        insert_default_profile(&mut output, &colors);
    }
    if !found_profiles {
        output.push("[profiles]".to_owned());
        insert_default_profile(&mut output, &colors);
    }

    let mut result = output.join(newline);
    if !result.is_empty() {
        result.push_str(newline);
    }
    result
}

/// Update Terminator's color settings and report whether the file changed.
/// Returning `false` avoids unnecessary D-Bus reloads on repeated selections.
pub fn apply_theme_to(path: impl AsRef<Path>, theme: &lib::ThemeEx) -> io::Result<bool> {
    let config = fs::read_to_string(&path)?;
    let updated = update_config(&config, theme);
    if updated == config {
        return Ok(false);
    }
    fs::write(path, updated)?;
    Ok(true)
}

/// Ask an already-running Terminator to reload its configuration via D-Bus.
/// The service's bus name is often suffixed with an MD5 hash of the X display;
/// enumerating registered bus names works for both suffixed and plain variants.
/// This is best-effort: a missing D-Bus session, dbus-send or Terminator is not an error.
pub fn reload_running() {
    let Ok(names) = Command::new("dbus-send")
        .args([
            "--session",
            "--print-reply",
            "--reply-timeout=1000",
            "--dest=org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus.ListNames",
        ])
        .output()
    else {
        return;
    };
    if !names.status.success() {
        return;
    }

    for name in String::from_utf8_lossy(&names.stdout).split('"') {
        let Some(suffix) = name.strip_prefix("net.tenshu.Terminator2") else {
            continue;
        };
        if !suffix.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            continue;
        }
        let destination = format!("--dest={name}");
        let method = format!("{name}.reload_configuration");
        let _ = Command::new("dbus-send")
            .args([
                "--session",
                "--type=method_call",
                destination.as_str(),
                "/net/tenshu/Terminator2",
                method.as_str(),
            ])
            .output();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> lib::ThemeEx {
        let colors = std::array::from_fn(|i| lib::Color::from_rgb(i as u8, 0, 0));
        lib::Theme::new("test", false, lib::ColorScheme::from_color_slice(&colors)).ex()
    }

    #[test]
    fn updates_multiple_profiles_without_modifying_other_sections() {
        let config = "[global_config]\n  focus = system\n[profiles]\n  [[default]]\n    font = Mono 12\n    foreground_color = \"#ffffff\"\n    palette = \"old\"\n    color_scheme = solarized_dark\n    # palette = leave this comment alone\n  [[work]]\n    use_theme_colors = True\n    background_color = \"#ffffff\"\n    scrollback_lines = 1000\n[layouts]\n  [[default]]\n    [[[window0]]]\n      background_color = \"keep layout untouched\"\n[plugins]\n";
        let result = update_config(config, &theme());
        assert!(result.contains("  focus = system"));
        assert!(result.contains("    font = Mono 12"));
        assert!(result.contains("    scrollback_lines = 1000"));
        assert!(result.contains("    # palette = leave this comment alone"));
        assert!(!result.contains("color_scheme = solarized_dark"));
        assert!(result.contains("      background_color = \"keep layout untouched\""));
        assert_eq!(result.matches("    use_theme_colors = False").count(), 2);
        assert_eq!(result.matches("    foreground_color = \"#010000\"").count(), 2);
        assert_eq!(result.matches("    background_color = \"#000000\"").count(), 2);
        assert_eq!(result.matches("    cursor_color = \"#040000\"").count(), 2);
        assert_eq!(result.matches("    cursor_fg_color = \"#050000\"").count(), 2);
        let ansi = (6..22).map(|i| format!("#{i:02x}0000")).collect::<Vec<_>>().join(":");
        assert_eq!(result.matches(&format!("    palette = \"{ansi}\"")).count(), 2);
        assert_eq!(update_config(&result, &theme()), result);
    }

    #[test]
    fn creates_default_profile_if_missing() {
        let result = update_config("[global_config]\n  focus = system\n", &theme());
        assert!(result.contains("[profiles]\n  [[default]]\n"));
        assert!(result.contains("    palette = \""));

        let result = update_config("[profiles]\n[layouts]\n", &theme());
        assert!(result.contains("[profiles]\n  [[default]]\n"));
        assert!(result.ends_with("[layouts]\n"));
    }

    #[test]
    fn preserves_crlf_and_existing_comments() {
        let original = "[profiles]\r\n  [[default]]\r\n    # user comment\r\n    palette = \"old\"\r\n";
        let output = update_config(original, &theme());
        assert!(output.contains("    # user comment\r\n"));
        assert!(!output.replace("\r\n", "").contains('\n'));
        assert_eq!(update_config(&output, &theme()), output);
    }

    #[test]
    fn recognizes_headers_with_comments_and_nested_sections() {
        assert_eq!(
            section_depth("[profiles] # configured profiles"),
            Some((1, "profiles"))
        );
        assert_eq!(
            section_depth("  [[work]]  # development"),
            Some((2, "work"))
        );
        assert_eq!(section_depth("    [[[child]]]"), Some((3, "child")));
        assert_eq!(section_depth("[[[[nested]]]]"), Some((4, "nested")));
        assert_eq!(section_depth("# [profiles]"), None);
        assert_eq!(section_depth("[profiles] invalid"), None);
        assert_eq!(section_depth("[[broken]"), None);

        let original = "[profiles] # configured profiles\n  [[work]] # development\n    # palette = preserve this comment\n    font = Mono 13\n[layouts] # saved layouts\n  [[default]]\n    [[[terminal0]]]\n      palette = \"leave untouched\"\n";
        let result = update_config(original, &theme());
        assert_eq!(result.matches("    foreground_color = ").count(), 1);
        assert!(result.contains("    # palette = preserve this comment"));
        assert!(result.contains("      palette = \"leave untouched\""));
        assert_eq!(update_config(&result, &theme()), result);
    }

    #[test]
    fn updates_only_profile_level_color_keys() {
        let original = "[profiles]\n  [[default]]\n    foreground_color = \"old\"\n    [[[nested]]]\n      foreground_color = \"nested unchanged\"\n  [[work]]\n    background_color = \"old\"\n";
        let result = update_config(original, &theme());
        assert!(result.contains("      foreground_color = \"nested unchanged\""));
        assert_eq!(result.matches("    foreground_color = \"#010000\"").count(), 2);
        assert_eq!(result.matches("    background_color = \"#000000\"").count(), 2);
        assert_eq!(update_config(&result, &theme()), result);
    }

    #[test]
    fn empty_config_is_populated_once() {
        let result = update_config("", &theme());
        assert!(result.starts_with("[profiles]\n  [[default]]\n"));
        assert_eq!(update_config(&result, &theme()), result);
    }

    #[test]
    fn file_application_does_not_touch_unrelated_entries() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config");
        fs::write(&path, "[keybindings]\n  split_horiz = <Ctrl>o\n").unwrap();
        assert!(apply_theme_to(&path, &theme()).unwrap());
        let output = fs::read_to_string(&path).unwrap();
        assert!(!apply_theme_to(&path, &theme()).unwrap());
        assert!(output.contains("  split_horiz = <Ctrl>o"));
        assert!(output.contains("[profiles]\n  [[default]]"));
    }
}
