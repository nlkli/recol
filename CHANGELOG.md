# Changelog

All notable changes to recol are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to Semantic Versioning.

## [Unreleased]

### Added
- **Generate a theme from your desktop wallpaper** — `recol -m W` now derives a theme directly from your current wallpaper, without selecting a file. Wallpaper detection is cross-platform: macOS (via System Events) and Linux/Unix (GNOME, KDE Plasma, XFCE, MATE, Cinnamon, LXQt). Note that on Linux the detection is best-effort and untested on some desktop environments.
- **Automatic Ghostty reload** — applying a theme to Ghostty now triggers a live config reload by sending `SIGUSR2`, so the terminal updates immediately on macOS and Linux, with no manual hot-reload shortcut needed.

### Improved
- **Interactive mode** — while browsing themes with `-i`, you can now print the selected theme as JSON (`-j`) or preview its palette (`-s`) without leaving the interactive session.
- **Media feature** — `recol -m` checks for `ffmpeg` up front and prints a warning instead of failing midway when it isn't installed.

### Documentation
- Added a **Homebrew** install option (`brew install nlkli/tap/recol`), with Cargo and pre-built release binaries kept as alternatives.
- Updated the **terminal support notes**: Ghostty now hot-reloads automatically, while Kitty and WezTerm/Alacritty behavior is documented.
- Tightened and reorganized the **CLI help** message.

### Note
- **Removed `CHANGELOG.md`** from the project history — version notes are now tracked in this file.
