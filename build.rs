fn main() {
    // Build the embedded color schemes binary when enabled.
    if std::env::var("RECOL_BUILD_COLORSCHEMES_BIN").is_ok() {
        let mut output = std::fs::File::create("./recol-lib/src/colorschemes.bin")
            .expect("Failed to create colorschemes.bin");

        // RECOL_GHOSSTY_THEMES_DIR overrides the default themes directory.
        build_colorschemes_bin(
            std::env::var("RECOL_GHOSSTY_THEMES_DIR").unwrap_or_else(|_| "./colorschemes".into()),
            &mut output,
            // Return `true` to exclude a theme.
            |theme| ["theme_to_exclude"].contains(&theme.name.as_str()),
            // normalize_bw
            true,
            // tmux_fix
            Some(TmuxFix {
                target_contrast: 2.3,
            }),
            // normalize_anomalies
            Some(AnomalyNormalization {
                z_threshold: 1.0,
                alpha: 0.666,
            }),
        )
        .expect("Failed to build colorschemes.bin");
    }
}

use recol_lib as lib;
use std::{io::Write, path::Path};

// Makes `black` readable on `green` (tmux's default status bar).
struct TmuxFix {
    // Minimum WCAG contrast ratio between `green` and `black`.
    target_contrast: f32,
}

// Pulls the lightness of palette colors that are outliers on both
// contrast and L* toward the median.
struct AnomalyNormalization {
    // Outlier threshold (lower = more colors are fixed).
    z_threshold: f32,
    // Pull strength toward the median (0.0 = none, 1.0 = full).
    alpha: f32,
}

// Builds `colorschemes.bin` from a directory of Ghostty theme files.
// Layout (big-endian): theme count (u16), theme offsets (u32 each), themes.
//
// - `filter_mut`: called on each parsed theme; `true` excludes it.
// - `normalize_bw`: orders black/white by lightness (light themes: black lighter
//   than white, dark themes: the opposite).
// - `tmux_fix`, `normalize_anomalies`: applied only when `Some`.
fn build_colorschemes_bin(
    dir_path: impl AsRef<Path>,
    mut out: impl Write,
    mut filter_mut: impl FnMut(&mut lib::Theme) -> bool,
    normalize_bw: bool,
    tmux_fix: Option<TmuxFix>,
    normalize_anomalies: Option<AnomalyNormalization>,
) -> std::io::Result<()> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir_path)? {
        let path = entry?.path();
        let Some(stem) = path.file_stem() else {
            continue;
        };
        files.push((stem.to_string_lossy().into_owned(), path));
    }
    // Sorted by name for a deterministic output.
    files.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut count = 0u16;
    let mut offset = 0u32;
    let mut offsets_bytes = Vec::new();
    let mut themes_bytes = Vec::new();

    for (name, path) in files {
        let mut theme = parse_ghostty_theme(&path, &name)?;
        if filter_mut(&mut theme) {
            continue;
        }

        if normalize_bw {
            normalize_black_white(&mut theme);
        }
        if let Some(fix) = &tmux_fix {
            apply_tmux_fix(&mut theme, fix.target_contrast);
        }
        if let Some(cfg) = &normalize_anomalies {
            apply_anomaly_normalization(&mut theme, cfg);
        }

        let bytes = theme.bytes();
        themes_bytes.extend_from_slice(&bytes);
        offsets_bytes.extend_from_slice(&offset.to_be_bytes());
        offset += bytes.len() as u32;
        count += 1;
    }

    out.write_all(&count.to_be_bytes())?;
    out.write_all(&offsets_bytes)?;
    out.write_all(&themes_bytes)?;
    out.flush()
}

// Parses a Ghostty theme file (`key = value` lines).
// Panics on malformed colors: themes are build-time input.
fn parse_ghostty_theme(path: impl AsRef<Path>, name: &str) -> std::io::Result<lib::Theme> {
    // Palette entries 0-15 are stored at colors[6..22].
    const PALETTE_OFFSET: usize = 6;

    let content = std::fs::read_to_string(path)?;
    let mut colors = [lib::Color::default(); lib::COLOR_SCHEME_NC];

    for line in content.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let (key, value) = (key.trim(), value.trim());

        let slot = match key {
            "background" => 0,
            "foreground" => 1,
            "selection-background" => 2,
            "selection-foreground" => 3,
            "cursor-color" => 4,
            "cursor-text" => 5,
            "palette" => {
                // Format: `palette = <index>=<#rrggbb>`
                let (index, color) = value
                    .split_once('=')
                    .expect("palette entry must be `<index>=<color>`");
                let index: usize = index.trim().parse().expect("palette index");
                let color: lib::Color = color.trim().parse().expect("palette color");
                if index < 16 {
                    colors[PALETTE_OFFSET + index] = color;
                }
                continue;
            }
            _ => continue,
        };
        colors[slot] = value
            .parse()
            .unwrap_or_else(|e| panic!("invalid `{key}` color: {e:?}"));
    }

    let scheme = lib::ColorScheme::from_color_slice(&colors);
    let is_light = scheme.bg.color().hsl().2 > 50.0;
    Ok(lib::Theme::new(name, is_light, scheme))
}

// Returns (black, white), swapped if their lightness order doesn't match the theme type.
fn ordered_black_white(
    is_light: bool,
    black: lib::Color,
    white: lib::Color,
) -> (lib::Color, lib::Color) {
    let (black_l, white_l) = (black.lab().0, white.lab().0);
    let needs_swap = if is_light {
        black_l < white_l
    } else {
        black_l > white_l
    };
    if needs_swap {
        (white, black)
    } else {
        (black, white)
    }
}

// Applies `ordered_black_white` to the base and bright palettes.
fn normalize_black_white(theme: &mut lib::Theme) {
    let is_light = theme.is_light;

    let base = &mut theme.colors.base;
    let (black, white) = ordered_black_white(is_light, base.black.color(), base.white.color());
    base.black = black.css();
    base.white = white.css();

    let bright = &mut theme.colors.bright;
    let (black, white) = ordered_black_white(is_light, bright.black.color(), bright.white.color());
    bright.black = black.css();
    bright.white = white.css();
}

// Shifts `black` (darker for dark colors, lighter for light ones) until its contrast
// with `green` reaches `target_contrast`, up to MAX_STEPS. `bright.black` is shifted
// by the same total amount.
fn apply_tmux_fix(theme: &mut lib::Theme, target_contrast: f32) {
    const MAX_STEPS: u32 = 99;

    let green = theme.colors.base.green.color();
    let mut black = theme.colors.base.black.color();
    let step = if black.lab().0 < 50. { -1. } else { 1. };

    let mut steps = 0u32;
    while green.wcag_contrast_ratio(&black) < target_contrast && steps < MAX_STEPS {
        black = black.brighten(step);
        steps += 1;
    }

    if steps > 0 {
        theme.colors.base.black = black.css();
        let bright_black = theme.colors.bright.black.color();
        theme.colors.bright.black = bright_black.brighten(step * steps as f32).css();
    }
}

// Runs `normalize_lightness_of_dual_outliers` on the listed fields of a color group
// (e.g. `theme.colors.base`) and writes the results back.
macro_rules! normalize_group {
    ($group:expr, [$($field:ident),+], $z:expr, $alpha:expr, $bg:expr) => {{
        let group = &mut $group;
        let mut palette = [$(group.$field.color()),+];
        normalize_lightness_of_dual_outliers($z, $alpha, $bg, &mut palette);
        let [$($field),+] = palette;
        $(group.$field = $field.css();)+
    }};
}

// Normalizes outliers in the base and bright palettes in two passes:
// 1. the six chromatic colors;
// 2. the same colors plus `white`, with a looser threshold and weaker pull.
fn apply_anomaly_normalization(theme: &mut lib::Theme, cfg: &AnomalyNormalization) {
    let z = cfg.z_threshold.abs();
    let alpha = cfg.alpha.clamp(0., 1.);
    let bg = theme.colors.bg.color();

    normalize_group!(
        theme.colors.base,
        [red, green, yellow, blue, magenta, cyan],
        z,
        alpha,
        &bg
    );
    normalize_group!(
        theme.colors.bright,
        [red, green, yellow, blue, magenta, cyan],
        z,
        alpha,
        &bg
    );

    let (z, alpha) = (z * 1.1, alpha * 0.5);
    normalize_group!(
        theme.colors.base,
        [red, green, yellow, blue, magenta, cyan, white],
        z,
        alpha,
        &bg
    );
    normalize_group!(
        theme.colors.bright,
        [red, green, yellow, blue, magenta, cyan, white],
        z,
        alpha,
        &bg
    );
}

// Pulls the L* of palette colors toward the median L* by `alpha` (a*, b* are kept).
// A color is changed only if it is an outlier (|z| > `z_threshold`) on BOTH
// WCAG contrast against `base` and L*.
fn normalize_lightness_of_dual_outliers(
    z_threshold: f32,
    alpha: f32,
    base: &lib::Color,
    palette: &mut [lib::Color],
) {
    fn median(v: &mut [f32]) -> f32 {
        if v.is_empty() {
            return 0.0;
        }
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = v.len();
        if n % 2 == 1 {
            v[n / 2]
        } else {
            (v[n / 2 - 1] + v[n / 2]) * 0.5
        }
    }

    // Returns (median, z-scores), where z = 0.6745 * (x - median) / (1.4826 * MAD).
    // Both constants are applied, so scores are ~1.48x smaller than the classic
    // Iglewicz-Hoaglin score (0.6745 * (x - median) / MAD). Zero MAD gives z = 0.
    fn robust_z(values: &[f32]) -> (f32, Vec<f32>) {
        let med = median(&mut values.to_vec());
        let mad = median(&mut values.iter().map(|v| (v - med).abs()).collect::<Vec<_>>()) * 1.4826;
        let z = values
            .iter()
            .map(|v| {
                if mad > 1e-6 {
                    0.6745 * (v - med) / mad
                } else {
                    0.0
                }
            })
            .collect();
        (med, z)
    }

    let contrasts: Vec<f32> = palette
        .iter()
        .map(|c| base.wcag_contrast_ratio(c))
        .collect();
    let lightness: Vec<f32> = palette.iter().map(|c| c.lab().0).collect();

    let (_, contrast_z) = robust_z(&contrasts);
    let (l_med, lightness_z) = robust_z(&lightness);

    for (i, color) in palette.iter_mut().enumerate() {
        if contrast_z[i].abs() <= z_threshold || lightness_z[i].abs() <= z_threshold {
            continue;
        }
        let l_new = lightness[i] + alpha * (l_med - lightness[i]);
        let (_, a, b) = color.lab();
        *color = lib::Color::from_lab(l_new, a, b);
    }
}
