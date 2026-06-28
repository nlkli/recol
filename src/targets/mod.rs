use crate::cli::Args;
use std::path::PathBuf;

use recol_lib as lib;

mod alacritty;
mod ghostty;
mod nvim;
mod wezterm;

pub enum Target {
    Alacritty,
    Ghostty,
    Nvim,
    Wezterm,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[inline(always)]
fn home_dir() -> std::path::PathBuf {
    std::env::home_dir().unwrap()
}

pub fn config_path(target: Target) -> Option<PathBuf> {
    let prefix = match std::env::var("XDG_CONFIG_HOME").ok() {
        Some(p) => PathBuf::from(p),
        None => home_dir().join(".config"),
    };
    match target {
        Target::Ghostty => {
            let path = prefix.join("ghostty/config.ghostty");
            if path.is_file() {
                return Some(path);
            }
            let path = prefix.join("ghostty/config");
            if path.is_file() {
                return Some(path);
            }
            #[cfg(target_os = "macos")]
            {
                let path = home_dir()
                    .join("Library/Application Support/com.mitchellh.ghostty/config.ghostty");
                if path.is_file() {
                    return Some(path);
                }
                let path =
                    home_dir().join("Library/Application Support/com.mitchellh.ghostty/config");
                if path.is_file() {
                    return Some(path);
                }
            }
            None
        }
        Target::Alacritty => {
            let path = prefix.join("alacritty/alacritty.toml");
            if path.is_file() {
                return Some(path);
            }
            let path = prefix.join("alacritty.toml");
            if path.is_file() {
                return Some(path);
            }
            let path = home_dir().join(".alacritty.toml");
            if path.is_file() {
                return Some(path);
            }
            let path = PathBuf::from("/etc/alacritty/alacritty.toml");
            if path.is_file() {
                return Some(path);
            }
            None
        }
        Target::Wezterm => {
            if let Ok(var) = std::env::var("WEZTERM_CONFIG_FILE") {
                let path = PathBuf::from(var);
                if path.is_file() {
                    return Some(path);
                }
            }
            let path = prefix.join("wezterm/wezterm.lua");
            if path.is_file() {
                return Some(path);
            }
            let path = home_dir().join(".wezterm.lua");
            if path.is_file() {
                return Some(path);
            }
            None
        }
        Target::Nvim => {
            let path = prefix.join("nvim/init.lua");
            if path.is_file() {
                return Some(path);
            }
            None
        }
    }
}

pub fn apply_theme(_args: &Args, t: &lib::Theme) -> Result<()> {
    if let Some(path) = config_path(Target::Ghostty) {
        ghostty::write_theme_into_config(&path, t)?;
    }
    if let Some(path) = config_path(Target::Alacritty) {
        alacritty::write_theme_into_config(&path, t)?;
    }
    if let Some(path) = config_path(Target::Wezterm) {
        wezterm::write_theme_into_config(&path, t)?;
    }
    // if let Some(ref path) = args.nvim_config {
    //     let path = std::path::PathBuf::from(path);
    //     if path.exists() && path.is_file() {
    //         nvim::write_theme_into_config(&path, t)?;
    //     }
    // } else {
    //     if let Some(path) = config_path(Target::Nvim) {
    //         nvim::write_theme_into_config(&path, t)?;
    //     }
    // }
    if let Some(path) = config_path(Target::Nvim) {
        nvim::write_theme_into_config(&path, t)?;
    }

    Ok(())
}

pub fn apply_font(name: &str) -> Result<()> {
    if let Some(path) = config_path(Target::Ghostty) {
        ghostty::set_font_into_config(&path, name.into())?;
    }
    if let Some(path) = config_path(Target::Alacritty) {
        alacritty::set_font_into_config(&path, name.into())?;
    }
    // TODO
    if let Some(_path) = config_path(Target::Wezterm) {
        // wezterm::set_font_into_config(&path, name.into())?;
    }

    Ok(())
}
