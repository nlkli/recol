use crate::collection::Theme;
use std::{
    fs,
    io::{self, BufRead, BufWriter, Write},
    path::Path,
};

enum ConfigLine {
    KeyValue((String, String)),
    Palette((isize, String)),
    Comment(String),
    Empty,
}

fn read_config(path: impl AsRef<Path>) -> io::Result<Vec<ConfigLine>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.is_empty() {
            lines.push(ConfigLine::Empty);
            continue;
        }
        if line.starts_with("#") {
            lines.push(ConfigLine::Comment(line));
            continue;
        }
        if let Some((k, v)) = line.split_once("=") {
            let k = k.trim().to_string();
            let v = v.trim().to_string();
            match k.as_str() {
                "palette" => {
                    if let Some((n, c)) = v.split_once("=") {
                        if let Ok(n) = n.parse::<isize>() {
                            lines.push(ConfigLine::Palette((n, c.into())));
                        }
                    }
                }
                _ => lines.push(ConfigLine::KeyValue((k, v))),
            }
        }
    }

    Ok(lines)
}

fn write_config(path: impl AsRef<Path>, lines: &[ConfigLine]) -> io::Result<()> {
    let file = fs::File::create(path)?;
    let mut writer = BufWriter::new(file);

    for line in lines {
        match line {
            ConfigLine::KeyValue((k, v)) => writeln!(writer, "{k} = {v}")?,
            ConfigLine::Palette((n, c)) => writeln!(writer, "palette = {n}={c}")?,
            ConfigLine::Comment(c) => writeln!(writer, "{}", c)?,
            ConfigLine::Empty => writeln!(writer, "")?,
        }
    }

    writer.flush()?;
    Ok(())
}

#[inline(always)]
fn replace_or_add_palette(lines: &mut Vec<ConfigLine>, n: isize, c: String) {
    if let Some(ConfigLine::Palette((_, pc))) = lines.iter_mut().rev().find(|e| {
        if let ConfigLine::Palette((pn, _)) = e {
            return *pn == n;
        }
        false
    }) {
        *pc = c;
    } else {
        lines.push(ConfigLine::Palette((n, c)));
    };
}

#[inline(always)]
fn replace_or_add_key_value(lines: &mut Vec<ConfigLine>, k: &str, v: String) {
    if let Some(ConfigLine::KeyValue((_, rv))) = lines.iter_mut().rev().find(|e| {
        if let ConfigLine::KeyValue((rk, _)) = e {
            return rk == k;
        }
        false
    }) {
        *rv = v.into();
    } else {
        lines.push(ConfigLine::KeyValue((k.into(), v.into())));
    };
}

pub fn write_theme_into_config(path: impl AsRef<Path>, theme: &mut Theme) -> io::Result<()> {
    let mut lines = read_config(&path)?;

    replace_or_add_palette(&mut lines, 0, theme.colors.base.black.clone());
    replace_or_add_palette(&mut lines, 1, theme.colors.base.red.clone());
    replace_or_add_palette(&mut lines, 2, theme.colors.base.green.clone());
    replace_or_add_palette(&mut lines, 3, theme.colors.base.yellow.clone());
    replace_or_add_palette(&mut lines, 4, theme.colors.base.blue.clone());
    replace_or_add_palette(&mut lines, 5, theme.colors.base.magenta.clone());
    replace_or_add_palette(&mut lines, 6, theme.colors.base.cyan.clone());
    replace_or_add_palette(&mut lines, 7, theme.colors.base.white.clone());

    replace_or_add_palette(&mut lines, 8, theme.colors.bright.black.clone());
    replace_or_add_palette(&mut lines, 9, theme.colors.bright.red.clone());
    replace_or_add_palette(&mut lines, 10, theme.colors.bright.green.clone());
    replace_or_add_palette(&mut lines, 11, theme.colors.bright.yellow.clone());
    replace_or_add_palette(&mut lines, 12, theme.colors.bright.blue.clone());
    replace_or_add_palette(&mut lines, 13, theme.colors.bright.magenta.clone());
    replace_or_add_palette(&mut lines, 14, theme.colors.bright.cyan.clone());
    replace_or_add_palette(&mut lines, 15, theme.colors.bright.white.clone());

    replace_or_add_palette(&mut lines, 16, theme.colors.base.orange.clone());
    replace_or_add_palette(&mut lines, 17, theme.colors.base.pink.clone());

    replace_or_add_key_value(&mut lines, "background", theme.colors.background[1].clone());
    replace_or_add_key_value(&mut lines, "foreground", theme.colors.foreground[1].clone());
    replace_or_add_key_value(&mut lines, "cursor-color", theme.colors.cursor.bg.clone());
    replace_or_add_key_value(&mut lines, "cursor-text", theme.colors.cursor.fg.clone());
    replace_or_add_key_value(
        &mut lines,
        "selection-background",
        theme.colors.selection.bg.clone(),
    );
    replace_or_add_key_value(
        &mut lines,
        "selection-foreground",
        theme.colors.selection.fg.clone(),
    );

    write_config(path, &lines)
}

pub fn set_font_into_config(path: impl AsRef<Path>, font: String) -> io::Result<()> {
    let mut lines = read_config(&path)?;

    replace_or_add_key_value(&mut lines, "font-family", font);

    write_config(path, &lines)
}
