<p align="center">
  <img src="https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-logo.png" alt="Logo" height="60">
</p>

<p align="center"><strong>CLI utility for changing the color scheme</strong></p>

![recol-demo-interactive-mode-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-interactive-mode.gif)

**recol** is a fast and lightweight command-line color scheme manager. Browse, preview, apply, adjust, and generate color schemes from images - all from one tool.

Designed for a consistent color experience across your development environment.

- **888+ built-in color schemes** from [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes) - embedded directly into the binary, carefully curated, color-normalized, and optimized for consistent appearance across supported tools.
- **7 built-in targets:** [Ghostty](https://ghostty.org), [Kitty](https://sw.kovidgoyal.net/kitty/), [Alacritty](https://alacritty.org), [WezTerm](https://wezterm.org), [Neovim](https://neovim.io), [Vim](https://www.vim.org) and [Pi](https://github.com/earendil-works/pi)
- **Interactive browser with live previews**
- **Color adjustments with brightness, contrast, saturation, hue, and more**
- **Image-to-theme generation from images and video frames**
- **Hot reload support where available**

### Terminal support notes

- **Ghostty** - auto-reloads via `SIGUSR2`; otherwise use `Ctrl + Shift + ,` (Linux) / `Cmd + Shift + ,` (macOS).
- **Kitty 0.47.0+** - auto-reloads by default. Older versions: `Ctrl + Shift + F5` (`Ctrl + Cmd + ,` on macOS). See [docs](https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.auto_reload_config).
- **Alacritty**, **WezTerm** - hot reload, no restart needed.

### Neovim integration

![recol-nvim-integration-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-nvim-integration.gif)

Neovim does not automatically reload colorschemes when its configuration changes. The included integration handles this by re-sourcing your configuration after applying a theme. See [`recol.lua`](https://github.com/nlkli/recol/blob/main/recol.lua).

**Install in `init.lua`:**

```sh
curl -Ls https://raw.githubusercontent.com/nlkli/recol/main/recol.lua >> ~/.config/nvim/init.lua
```

- `:Recol <args>` - runs `recol` with the given arguments, then reloads your config
- `:Recol -i` / `:RecolOpen` - opens `recol` in a floating terminal window

### Build From Source

```sh
git clone https://github.com/nlkli/recol
cd recol
cargo build --release
./target/release/recol --help
```

### Homebrew

[Formula](https://github.com/nlkli/homebrew-tap/blob/main/Formula/recol.rb)

```sh
brew install nlkli/tap/recol
```

### Cargo

```sh
cargo install --git https://github.com/nlkli/recol --branch main --force
```

### Build Colorschemes Collection

Build your own color schemes collection right into the binary:

```sh
./fetch.sh   # downloads themes into ./colorschemes
RECOL_BUILD_COLORSCHEMES_BIN=1 cargo build --release
```

Add your own themes to `./colorschemes`, or exclude unwanted ones in `build.rs`.

### Usage Examples

By default, recol applies the selected theme to all supported targets for which a configuration file can be found. To apply a theme to a specific target, use the `--target` flag, or use `--output` to generate themes in a specified directory without applying them.

```sh
recol lOnDoNsOhOnIgHt67       # fuzzy match - applies closest theme by name
recol NordWave -o ./output    # generate themes into the directory
recol -rd --contains Gruvbox  # random dark theme with "Gruvbox" in name
recol dracula --dark --show   # preview palette without applying
recol --list -l --json        # list light themes as JSON
recol terafox --target nvim   # apply theme for specific target
recol                         # print current theme name (add --show or --json for more)

# change a theme color via JSON and apply the modified theme
echo GruberDarker69 | recol -j | jq '.colors.fg[1] = "#ff0000"' | recol
recol --list | tail | recol -rdj | jq '.colors.bg[1] = "#000000"' | recol

echo "ubuntu\nVague" | recol -i # interactive mode with initial themes

# start interactive mode with your favourite themes
recol --list -dc "Catppuccin" >> favourites
cat favourites | recol -i
```

### Shell completion

Bash and zsh. Completes flags, targets, theme names and `--adjust` specs. See [`completion.sh`](https://github.com/nlkli/recol/blob/main/completion.sh).

**Install** (zsh: the line must come after `compinit`):

```sh
f=~/.${SHELL##*/}rc; echo 'eval "$(recol --completions)"' >> "$f"
```

### Color Adjustments

![recol-demo-adjust-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-adjust.gif)

Adjust theme colors with `--adjust "group.adjustment=value,..."`. Supports brightness, contrast, saturation, hue, exposure, gamma, temperature, tint, normalize and more. Apply to UI elements, specific colors, or the full ANSI palette using short group names (e.g. pal, bg, red).

```sh
recol --adjust help
```

In interactive mode you can change adjustments live and see the preview update instantly.

### Generate a theme from a media file

![recol-demo-media-to-theme](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-media-to-theme.gif)

`recol` can derive a color scheme from any image/video frame. Pass a media file with `--media` and recol builds a theme automatically:

```sh
recol --media ~/Pictures/Sunset.png            # generate and apply a theme from an image
recol -m ~/Videos/X.gif --json                 # generate a theme and output it as JSON
recol -m ~/Photo/Tree.png -a t.s=9,bb.b=-12    # generate and apply with color adjust
```

**Requirement:** [FFmpeg](https://ffmpeg.org) must be installed and available on `PATH`.

This feature uses FFmpeg's built-in palettegen functionality and requires no additional Rust dependencies.

### Adding Support for New Targets

![recol-demo-pi-agent-target](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-pi-agent-target.gif)

`recol` ships with a limited set of built-in targets (Ghostty, Kitty, Alacritty, WezTerm, Neovim, Vim, Pi). You can extend it to apply themes to any application that lets you tweak its config file - window managers, browsers, file managers, text editors, and more.

Each target is a small module under `src/targets/`:

1. **Locate the config file** - add a `Target` arm in `existing_default_config_path()` (`src/targets/mod.rs`). Find the correct path(s), honouring `XDG_CONFIG_HOME`; return `None` if the target isn't configured.
2. **Apply the theme non-destructively** - implement `apply_theme_to(path, theme)` in a new `src/targets/<name>.rs`. Edit the config in place (replace or insert the color values), preserving everything else, and match an existing target's config format.
3. **Register the target** in `src/targets/mod.rs`:

```rust
// src/targets/mod.rs - example registration
mod vscode;
// ...
pub enum Target { /* ..., Vscode */ }
pub const ALL_TARGETS: &[Target] = &[ /* ..., Target::Vscode */ ];
// Display, FromStr, apply_theme_to, existing_default_config_path - add one arm each
```

### Demo & Screenshots

![recol-demo-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo.gif)

![recol-demo-media-to-theme](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-media-to-theme.jpg)

![recol-demo-img-2](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-2.png)

<sub>All demos and screenshots are from an older version. I'm too lazy to update them.</sub>

[LICENSE](LICENSE)
