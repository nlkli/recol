use crate::cli::Args;
use std::{fmt, path::PathBuf};

use recol_lib as lib;

mod alacritty;
mod ghostty;
mod nvim;
mod vim;
mod wezterm;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const ALL_TARGETS: [Target; 5] = [
    Target::Ghostty,
    Target::Alacritty,
    Target::Wezterm,
    Target::Nvim,
    Target::Vim,
];

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Target {
    #[default]
    None,
    Ghostty,
    Alacritty,
    Wezterm,
    Nvim,
    Vim,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Target::None => "none",
            Target::Ghostty => "ghostty",
            Target::Alacritty => "alacritty",
            Target::Wezterm => "wezterm",
            Target::Nvim => "neovim",
            Target::Vim => "vim",
        };
        write!(f, "{s}")
    }
}

impl std::str::FromStr for Target {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "g" | "gt" | "ghostty" => Ok(Self::Ghostty),
            "a" | "at" | "alacritty" => Ok(Self::Alacritty),
            "w" | "wt" | "wezterm" => Ok(Self::Wezterm),
            "n" | "nv" | "nvi" | "nvim" | "neovim" => Ok(Self::Nvim),
            "v" | "vi" | "vim" => Ok(Self::Vim),
            _ => Err(()),
        }
    }
}

#[inline(always)]
fn home_dir() -> std::path::PathBuf {
    std::env::home_dir().unwrap()
}

impl Target {
    pub fn apply_theme(&self, t: &lib::Theme) -> Result<()> {
        if let Some(path) = self.config_path() {
            match self {
                Target::Ghostty => ghostty::write_theme_to_config(&path, t)?,
                Target::Alacritty => alacritty::write_theme_to_config(&path, t)?,
                Target::Wezterm => wezterm::write_theme_to_config(&path, t)?,
                Target::Nvim => nvim::write_theme_to_config(&path, t)?,
                Target::Vim => vim::write_theme_to_config(&path, t)?,
                Target::None => {}
            }
        }
        Ok(())
    }

    pub fn set_font(&self, font_name: impl Into<String>) -> Result<()> {
        if let Some(path) = self.config_path() {
            match self {
                Target::Ghostty => ghostty::set_font_to_config(&path, font_name.into())?,
                Target::Alacritty => alacritty::set_font_to_config(&path, font_name.into())?,
                Target::Wezterm => {} // wezterm::set_font_to_config(&path, t)?,
                _ => {}
            }
        }
        Ok(())
    }

    pub fn config_path(&self) -> Option<PathBuf> {
        let prefix = match std::env::var("XDG_CONFIG_HOME").ok() {
            Some(p) => PathBuf::from(p),
            None => home_dir().join(".config"),
        };
        match self {
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
            Target::Vim => {
                let path = home_dir().join(".vimrc");
                if path.is_file() {
                    return Some(path);
                }
                let path = home_dir().join(".vim/vimrc");
                if path.is_file() {
                    return Some(path);
                }
                let path = PathBuf::from("/etc/vimrc");
                if path.is_file() {
                    return Some(path);
                }
                if let Ok(vim_dir) = std::env::var("VIM") {
                    let path = PathBuf::from(vim_dir).join("vimrc");
                    if path.is_file() {
                        return Some(path);
                    }
                }
                None
            }
            Target::None => None,
        }
    }
}

pub fn apply_theme(args: &Args, theme: &lib::Theme) -> Result<()> {
    for target in if args.targets.is_empty() {
        &ALL_TARGETS
    } else {
        args.targets.as_slice()
    } {
        target.apply_theme(&theme)?;
    }

    Ok(())
}

pub fn set_font(args: &Args, font_name: &str) -> Result<()> {
    for target in if args.targets.is_empty() {
        &ALL_TARGETS
    } else {
        args.targets.as_slice()
    } {
        target.set_font(font_name)?;
    }

    Ok(())
}
