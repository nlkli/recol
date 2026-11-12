use std::{
    fmt, fs,
    path::{Path, PathBuf},
    time,
};

use recol_lib as lib;

mod alacritty;
mod ghostty;
mod kitty;
mod nvim;
mod pi;
mod terminator;
mod vim;
mod wezterm;

pub const ALL_TARGETS: &[Target] = &[
    Target::Ghostty,
    Target::Kitty,
    Target::Terminator,
    Target::Alacritty,
    Target::Wezterm,
    Target::Nvim,
    Target::Vim,
    Target::Pi,
];

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Target {
    Ghostty,
    Kitty,
    Terminator,
    Alacritty,
    Wezterm,
    Nvim,
    Vim,
    Pi,

    #[default]
    None,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Target::None => "none",
                Target::Ghostty => "ghostty",
                Target::Kitty => "kitty",
                Target::Terminator => "terminator",
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
            "k" | "kt" | "kitty" => Ok(Self::Kitty),
            "terminator" => Ok(Self::Terminator),
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
        theme: &lib::ThemeEx,
    ) -> crate::Result<()> {
        Ok(match self {
            Target::Ghostty => ghostty::apply_theme_to(&config_path, theme)?,
            Target::Kitty => kitty::apply_theme_to(&config_path, theme)?,
            Target::Terminator => {
                let changed = terminator::apply_theme_to(&config_path, theme)?;
                if changed
                    && self.existing_default_config_path().as_deref() == Some(config_path.as_ref())
                {
                    terminator::reload_running();
                }
            }
            Target::Alacritty => alacritty::apply_theme_to(&config_path, theme)?,
            Target::Wezterm => wezterm::apply_theme_to(&config_path, theme)?,
            Target::Nvim => nvim::apply_theme_to(&config_path, theme)?,
            Target::Vim => vim::apply_theme_to(&config_path, theme)?,
            Target::Pi => pi::apply_theme_to(&config_path, theme)?,
            Target::None => (),
        })
    }

    pub fn set_font_on(
        &self,
        config_path: impl AsRef<Path>,
        font_name: impl Into<String>,
    ) -> crate::Result<()> {
        Ok(match self {
            Target::Ghostty => ghostty::set_font_on(&config_path, font_name.into())?,
            Target::Alacritty => alacritty::set_font_on(&config_path, font_name.into())?,
            // Not implemented yet
            // Target::Wezterm => wezterm::set_font_on(&path, font_name.into())?,
            // Target::Kitty => kitty::set_font_on(&path, font_name.into())?,
            _ => (),
        })
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
            Target::Kitty => {
                let path = prefix.join("kitty/kitty.conf");
                path.is_file().then_some(path)
            }
            Target::Terminator => {
                let path = prefix.join("terminator/config");
                path.is_file().then_some(path)
            }
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
                let appname = std::env::var("NVIM_APPNAME").ok();
                // Respect user's nvim config, e.g. ~/.config/customvim/init.lua
                let path = prefix
                    .join(appname.as_deref().unwrap_or("nvim"))
                    .join("init.lua");
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
        // TODO: the path to the Pi configuration is the directory. skip?
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
    theme: &lib::ThemeEx,
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

pub fn generate_theme_into_dir(
    path: impl AsRef<Path>,
    theme: &lib::ThemeEx,
    targets: &[Target],
) -> crate::Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        std::fs::create_dir(path)?;
    } else {
        if !path.is_dir() {
            return Err(std::io::Error::other(format!("not a directory: {path:?}")).into());
        }
    }
    for target in with_specific_or_for_all(targets) {
        let target_path_name = match target {
            Target::Ghostty => "config.ghostty",
            Target::Kitty => "kitty.conf",
            Target::Terminator => "terminator.conf",
            Target::Alacritty => "alacritty.toml",
            Target::Wezterm => "wezterm.lua",
            Target::Nvim => "init.lua",
            Target::Vim => "vimrc",
            Target::Pi => "pi_agent",
            Target::None => "",
        };
        if target_path_name.is_empty() {
            continue;
        }
        let config_path = path.join(target_path_name);
        if *target != Target::Pi {
            let _f = std::fs::File::create(&config_path)?;
            drop(_f);
        }
        target.apply_theme_to(config_path, theme)?;
    }
    Ok(())
}

#[cfg(test)]
mod target_tests {
    use super::*;

    #[test]
    fn terminator_is_registered_and_selectable() {
        assert!(ALL_TARGETS.contains(&Target::Terminator));
        assert_eq!("terminator".parse::<Target>(), Ok(Target::Terminator));
        assert_eq!("TERMINATOR".parse::<Target>(), Ok(Target::Terminator));
        assert!("t".parse::<Target>().is_err());
        assert!("tm".parse::<Target>().is_err());
        assert_eq!(Target::Terminator.to_string(), "terminator");
    }
}
