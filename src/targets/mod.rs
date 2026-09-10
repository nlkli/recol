use std::{
    fmt, fs,
    path::{Path, PathBuf},
    time,
};

use recol_lib as lib;

mod alacritty;
mod ghostty;
mod nvim;
mod pi;
mod vim;
mod wezterm;

pub const ALL_TARGETS: [Target; 6] = [
    Target::Ghostty,
    Target::Alacritty,
    Target::Wezterm,
    Target::Nvim,
    Target::Vim,
    Target::Pi,
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
    Pi,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Target::None => "none",
                Target::Ghostty => "ghostty",
                Target::Alacritty => "alacritty",
                Target::Wezterm => "wezterm",
                Target::Nvim => "neovim",
                Target::Vim => "vim",
                Target::Pi => "pi",
            }
        )
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
            "pi" => Ok(Self::Pi),
            _ => Err(()),
        }
    }
}

#[inline(always)]
fn home_dir() -> std::path::PathBuf {
    std::env::home_dir().unwrap()
}

impl Target {
    pub fn apply_theme_to(
        &self,
        config_path: impl AsRef<Path>,
        theme: &lib::Theme,
    ) -> crate::Result<()> {
        match self {
            Target::Ghostty => ghostty::apply_theme_to(&config_path, theme)?,
            Target::Alacritty => alacritty::apply_theme_to(&config_path, theme)?,
            Target::Wezterm => wezterm::apply_theme_to(&config_path, theme)?,
            Target::Nvim => nvim::apply_theme_to(&config_path, theme)?,
            Target::Vim => vim::apply_theme_to(&config_path, theme)?,
            Target::Pi => pi::apply_theme_to(&config_path, theme)?,
            Target::None => {}
        }
        Ok(())
    }

    pub fn set_font_on(
        &self,
        config_path: impl AsRef<Path>,
        font_name: impl Into<String>,
    ) -> crate::Result<()> {
        match self {
            Target::Ghostty => ghostty::set_font_on(&config_path, font_name.into())?,
            Target::Alacritty => alacritty::set_font_on(&config_path, font_name.into())?,
            Target::Wezterm => {} // wezterm::set_font_on(&path, font_name.into())?,
            _ => {}
        }
        Ok(())
    }

    /// Ghostty: https://ghostty.org/docs/config#file-location
    /// Alacritty: https://alacritty.org/config-alacritty.html#location
    /// WezTerm: https://wezterm.org/config/files.html#configuration-files
    pub fn existing_default_config_path(&self) -> Option<PathBuf> {
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
            Target::Pi => {
                if let Ok(pi_dir) = std::env::var("PI_CODING_AGENT_DIR") {
                    let path = PathBuf::from(pi_dir);
                    if path.is_dir() {
                        return Some(path);
                    }
                }
                let path = home_dir().join(".pi/agent");
                if path.is_dir() {
                    return Some(path);
                }
                None
            }
            Target::None => None,
        }
    }

    pub fn create_backup(&self, config_path: impl AsRef<Path>) -> crate::Result<Option<PathBuf>> {
        // TODO: the path to the pi configuration is the directory. skip?
        if config_path.as_ref().is_dir() {
            return Ok(None);
        }
        let backup_path = std::env::temp_dir().join(format!(
            "recol.{}.{}.backup",
            self,
            time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)?
                .as_millis()
        ));
        fs::copy(config_path, &backup_path)?;
        Ok(Some(backup_path))
    }
}

/// Returns the selected targets, or all targets when none are specified.
#[inline]
pub fn with_specific_or_for_all(specific: &[Target]) -> impl Iterator<Item = &Target> {
    if specific.is_empty() {
        ALL_TARGETS.iter()
    } else {
        specific.iter()
    }
}

/// Returns targets that have an existing default configuration file.
pub fn with_existing_default_config_path<'a>(
    targets: impl Iterator<Item = &'a Target>,
) -> impl Iterator<Item = (&'a Target, PathBuf)> {
    targets.filter_map(|target| {
        target
            .existing_default_config_path()
            .map(|path| (target, path))
    })
}

fn with_backup<'a>(
    targets: impl Iterator<Item = (&'a Target, PathBuf)>,
) -> crate::Result<Vec<(&'a Target, PathBuf, Option<PathBuf>)>> {
    let mut backups = Vec::with_capacity(ALL_TARGETS.len());
    for (target, config_path) in targets {
        backups.push((
            target,
            config_path.clone(),
            target.create_backup(config_path)?,
        ));
    }
    Ok(backups)
}

/// Applies a theme to all selected configurations and restores backups on failure.
pub fn apply_theme<'a>(
    targets: impl Iterator<Item = (&'a Target, PathBuf)>,
    theme: &lib::Theme,
) -> crate::Result<()> {
    // Create all backups before changing anything.
    let backups = with_backup(targets)?;
    let mut error = None;
    for (target, config_path, _) in backups.iter() {
        if let Err(e) = target.apply_theme_to(config_path, theme) {
            error.replace(e);
            break;
        }
    }
    if let Some(e) = error {
        backups.iter().for_each(|(_, dst, src)| {
            if let Some(src) = src {
                let _ = fs::copy(src, dst);
                let _ = fs::remove_file(src);
            }
        });
        return crate::Result::Err(e);
    }
    backups.into_iter().for_each(|(_, _, src)| {
        if let Some(src) = src {
            let _ = fs::remove_file(src);
        }
    });

    Ok(())
}

pub fn set_font<'a>(
    targets: impl Iterator<Item = (&'a Target, PathBuf)>,
    font_name: &str,
) -> crate::Result<()> {
    for (target, config_path) in targets {
        target.set_font_on(config_path, font_name)?;
    }
    Ok(())
}
