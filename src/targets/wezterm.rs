use recol_lib as lib;
use std::io::{self, BufRead, Write};
use std::{fs, path::Path};

pub fn write_theme_into_config(path: impl AsRef<Path>, theme: &lib::Theme) -> io::Result<()> {
    let path = path.as_ref();

    // --- Parse existing config ---
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut preamble = Vec::new();
    let mut config_var = Vec::new();
    let mut mark_block = Vec::new();
    let mut in_mark_block = false;
    let mut in_config_var = false;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim_start();

        if trimmed.starts_with("-- recol:start") && !in_mark_block {
            in_mark_block = true;
            continue;
        }
        if trimmed.starts_with("-- recol:end") && in_mark_block {
            in_mark_block = false;
            continue;
        }
        if in_mark_block {
            mark_block.extend_from_slice(line.as_bytes());
            mark_block.push(b'\n');
            continue;
        }

        if trimmed.starts_with("return ") {
            // Flush any previously accumulated config var lines,
            // then rewrite this line as a local binding.
            if !config_var.is_empty() {
                preamble.extend_from_slice(&config_var);
                config_var.clear();
            }
            let rewritten = trimmed.replacen("return", "local config =", 1);
            in_config_var = true;
            config_var.extend_from_slice(rewritten.as_bytes());
            config_var.push(b'\n');
            continue;
        }

        if in_config_var {
            config_var.extend_from_slice(line.as_bytes());
            config_var.push(b'\n');
            continue;
        }

        preamble.extend_from_slice(line.as_bytes());
        preamble.push(b'\n');
    }

    // TODO: tab_bar

    // --- Build theme block ---
    let colors = theme.colors.clone().into_advanced(None);
    let theme_block = format!(
        r###"-- {theme_name}
config.colors = {{}}
config.colors.background = "{bg}"
config.colors.foreground = "{fg}"
config.colors.cursor_bg = "{cur_bg}"
config.colors.cursor_fg = "{cur_fg}"
config.colors.cursor_border = "{cur_bg}"
config.colors.selection_bg = "{sel_bg}"
config.colors.selection_fg = "{sel_fg}"
config.colors.ansi = {{
    "{black}",
    "{red}",
    "{green}",
    "{yellow}",
    "{blue}",
    "{magenta}",
    "{cyan}",
    "{white}",
}}
config.colors.brights = {{
    "{black_bright}",
    "{red_bright}",
    "{green_bright}",
    "{yellow_bright}",
    "{blue_bright}",
    "{magenta_bright}",
    "{cyan_bright}",
    "{white_bright}",
}}"###,
        theme_name = theme.name,
        bg = colors.bg[1],
        fg = colors.fg[1],
        cur_bg = colors.cursor.bg,
        cur_fg = colors.cursor.fg,
        sel_bg = colors.selection.bg,
        sel_fg = colors.selection.fg,
        black = colors.base.black,
        red = colors.base.red,
        green = colors.base.green,
        yellow = colors.base.yellow,
        blue = colors.base.blue,
        magenta = colors.base.magenta,
        cyan = colors.base.cyan,
        white = colors.base.white,
        black_bright = colors.bright.black,
        red_bright = colors.bright.red,
        green_bright = colors.bright.green,
        yellow_bright = colors.bright.yellow,
        blue_bright = colors.bright.blue,
        magenta_bright = colors.bright.magenta,
        cyan_bright = colors.bright.cyan,
        white_bright = colors.bright.white,
    );

    // --- Write output ---
    let mut file = fs::File::create(path)?;

    // Preamble (strip trailing newline added by the loop).
    preamble.pop();
    file.write_all(&preamble)?;

    if in_mark_block {
        // file.write_all(&mark_block)?;
    }

    // Config-var section, or a fresh empty table when absent.
    const PASSTHROUGH_CONFIG: &[u8] = b"local config = config\n";
    if config_var.is_empty() {
        writeln!(&mut file, "\nlocal config = {{}}")?;
    } else if config_var != PASSTHROUGH_CONFIG {
        writeln!(&mut file)?;
        file.write_all(&config_var)?;
    }

    // Inject new theme block.
    writeln!(&mut file, "-- recol:start")?;
    writeln!(&mut file, "{theme_block}")?;
    writeln!(&mut file, "-- recol:end")?;
    writeln!(&mut file, "\nreturn config")?;

    file.flush()
}
