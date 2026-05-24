use crate::collection::Theme;
use std::{
    fs,
    io::{self, BufRead, BufWriter, Write},
    path::Path,
};

enum ConfigRow {
    KeyValue((String, String)),
    Palette((isize, String)),
    Comment(String),
    Empty,
}

fn read_config(path: impl AsRef<Path>) -> io::Result<Vec<ConfigRow>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut rows = Vec::new();
    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.len() == 0 {
            rows.push(ConfigRow::Empty);
            continue;
        }
        if line.starts_with("#") {
            rows.push(ConfigRow::Comment(line));
            continue;
        }
        if let Some((k, v)) = line.split_once("=") {
            let k = k.trim().to_string();
            let v = v.trim().to_string();
            match k.as_str() {
                "palette" => {
                    if let Some((n, c)) = v.split_once("=") {
                        if let Ok(n) = n.parse::<isize>() {
                            rows.push(ConfigRow::Palette((n, c.into())));
                        }
                    }
                }
                _ => rows.push(ConfigRow::KeyValue((k, v))),
            }
        }
    }

    Ok(rows)
}

fn write_config(path: impl AsRef<Path>, rows: &[ConfigRow]) -> io::Result<()> {
    let file = fs::File::create(path)?;
    let mut writer = BufWriter::new(file);

    for row in rows {
        match row {
            ConfigRow::KeyValue((k, v)) => writeln!(writer, "{k} = {v}")?,
            ConfigRow::Palette((n, c)) => writeln!(writer, "palette = {n}={c}")?,
            ConfigRow::Comment(c) => writeln!(writer, "{}", c)?,
            ConfigRow::Empty => writeln!(writer, "")?,
        }
    }

    writer.flush()?;
    Ok(())
}

#[inline(always)]
fn replace_or_add_palette(rows: &mut Vec<ConfigRow>, n: isize, c: String) {
    if let Some(ConfigRow::Palette((_, pc))) = rows.iter_mut().rev().find(|e| {
        if let ConfigRow::Palette((pn, _)) = e {
            return *pn == n;
        }
        false
    }) {
        *pc = c;
    } else {
        rows.push(ConfigRow::Palette((n, c)));
    };
}

#[inline(always)]
fn replace_or_add_key_value(rows: &mut Vec<ConfigRow>, k: &str, v: String) {
    if let Some(ConfigRow::KeyValue((_, rv))) = rows.iter_mut().rev().find(|e| {
        if let ConfigRow::KeyValue((rk, _)) = e {
            return rk == k;
        }
        false
    }) {
        *rv = v.into();
    } else {
        rows.push(ConfigRow::KeyValue((k.into(), v.into())));
    };
}

pub fn write_theme_into_config(path: impl AsRef<Path>, theme: &mut Theme) -> io::Result<()> {
    let mut rows = read_config(&path)?;

    replace_or_add_palette(&mut rows, 0, theme.colors.base.black.clone());
    replace_or_add_palette(&mut rows, 1, theme.colors.base.red.clone());
    replace_or_add_palette(&mut rows, 2, theme.colors.base.green.clone());
    replace_or_add_palette(&mut rows, 3, theme.colors.base.yellow.clone());
    replace_or_add_palette(&mut rows, 4, theme.colors.base.blue.clone());
    replace_or_add_palette(&mut rows, 5, theme.colors.base.magenta.clone());
    replace_or_add_palette(&mut rows, 6, theme.colors.base.cyan.clone());
    replace_or_add_palette(&mut rows, 7, theme.colors.base.white.clone());

    replace_or_add_palette(&mut rows, 8, theme.colors.bright.black.clone());
    replace_or_add_palette(&mut rows, 9, theme.colors.bright.red.clone());
    replace_or_add_palette(&mut rows, 10, theme.colors.bright.green.clone());
    replace_or_add_palette(&mut rows, 11, theme.colors.bright.yellow.clone());
    replace_or_add_palette(&mut rows, 12, theme.colors.bright.blue.clone());
    replace_or_add_palette(&mut rows, 13, theme.colors.bright.magenta.clone());
    replace_or_add_palette(&mut rows, 14, theme.colors.bright.cyan.clone());
    replace_or_add_palette(&mut rows, 15, theme.colors.bright.white.clone());

    replace_or_add_key_value(&mut rows, "background", theme.colors.background[1].clone());
    replace_or_add_key_value(&mut rows, "foreground", theme.colors.foreground[1].clone());
    replace_or_add_key_value(&mut rows, "cursor-color", theme.colors.cursor.bg.clone());
    replace_or_add_key_value(&mut rows, "cursor-text", theme.colors.cursor.fg.clone());
    replace_or_add_key_value(
        &mut rows,
        "selection-background",
        theme.colors.selection.bg.clone(),
    );
    replace_or_add_key_value(
        &mut rows,
        "selection-foreground",
        theme.colors.selection.fg.clone(),
    );

    write_config(path, &rows)
}

pub fn set_font_into_config(path: impl AsRef<Path>, font: String) -> io::Result<()> {
    let mut rows = read_config(&path)?;

    replace_or_add_key_value(&mut rows, "font-family", font);

    write_config(path, &rows)
}
