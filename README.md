# recol

**Switch your terminal, editor, and application color schemes from one command — no manual config editing.** Pick from 600+ prebuilt schemes with instant fuzzy search and apply them across multiple supported targets.

![recol-demo-interactive-mode-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-interactive-mode.gif)

- **600+ color schemes** from [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes)
- **Targets support:** [Ghostty](https://ghostty.org), [Kitty](https://sw.kovidgoyal.net/kitty/), [Alacritty](https://alacritty.org), [WezTerm](https://wezterm.org), [Neovim](https://neovim.io), [Vim](https://www.vim.org), [Pi](https://github.com/earendil-works/pi)
- **Neovim theme integration** based on [Nightfox.nvim](https://github.com/EdenEast/nightfox.nvim)
- **Non-destructive** — only color values are modified, nothing else in your config

### Terminal support notes

- **Ghostty**: now reloads automatically via `SIGUSR2` (should work on most systems); if not, use the manual shortcut (`Ctrl + Shift + ,` on Linux or `Cmd + Shift + ,` on macOS).
- **Kitty 0.47.0+** automatically reloads config changes by default. Older versions, or configurations with auto reload disabled, require `Ctrl + Shift + F5` (`Ctrl + Cmd + ,` on macOS). See [Kitty's auto reload documentation](https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.auto_reload_config).
- **Alacritty**, **WezTerm** support hot configuration reload. Changes are applied immediately without restarting the terminal.

### Neovim integration

![recol-nvim-integration-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-nvim-integration.gif)

Neovim doesn't support hot theme reload — the config needs to be re-sourced after switching. The integration below handles this automatically after every `recol` run.

Implementation: [`recol.lua`](https://github.com/nlkli/recol/blob/main/recol.lua)

**Install:**

```sh
curl -Ls https://raw.githubusercontent.com/nlkli/recol/main/recol.lua >> ~/.config/nvim/init.lua
```

- `:Recol <args>` — runs `recol` with the given arguments, then reloads your config
- `:Recol -i` / `:RecolOpen` — opens `recol` in a floating terminal window

### Build From Source

```sh
git clone https://github.com/nlkli/recol
cd recol
cargo build --release
./target/release/recol --help
```

### Homebrew

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

```sh
recol lOnDoNsOhOnIgHt67       # fuzzy match - applies closest theme by name
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

**Requirements:** [ffmpeg](https://ffmpeg.org) must be installed and available on `PATH`.

This feature requires no additional Cargo/Rust dependencies. recol simply invokes the ffmpeg binary already installed on the system and uses its palettegen functionality to extract colors.

### Adding Support for New Targets

![recol-demo-pi-agent-target](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-pi-agent-target.gif)

`recol` ships with a limited set of built-in targets (Ghostty, Kitty, Alacritty, WezTerm, Neovim, Vim, Pi). You can extend it to apply themes to any application that lets you tweak its config file — window managers, browsers, file managers, text editors, and more.

Each target is a small module under `src/targets/`:

1. **Locate the config file** — add a `Target` arm in `existing_default_config_path()` (`src/targets/mod.rs`). Find the correct path(s), honouring `XDG_CONFIG_HOME`; return `None` if the target isn't configured.
2. **Apply the theme non-destructively** — implement `apply_theme_to(path, theme)` in a new `src/targets/<name>.rs`. Edit the config in place (replace or insert the color values), preserving everything else, and match an existing target's config format. Optionally implement `set_font_on` for font changes.
3. **Register the target** in `src/targets/mod.rs`:
   - `mod <name>;`
   - Add a variant to the `Target` enum
   - Add it to `ALL_TARGETS`
   - Add arms to `Display` (the shown name) and `FromStr` (the CLI value, e.g. `vscode`)
   - Add arms to `Target::apply_theme_to` and `existing_default_config_path`

```rust
// src/targets/mod.rs — example registration
mod vscode;
// ...
pub enum Target { /* ..., Vscode */ }
pub const ALL_TARGETS: [Target; 8] = [ /* ..., Target::Vscode */ ];
// Display, FromStr, apply_theme_to, existing_default_config_path — add one arm each
```

### Demo & Screenshots

![recol-demo-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo.gif)

![recol-demo-media-to-theme](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-media-to-theme.jpg)

![recol-demo-img-2](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-2.png)

### SCC

```text
───────────────────────────────────────────────────────────────────────────────
Language            Files       Lines    Blanks  Comments       Code Complexity
───────────────────────────────────────────────────────────────────────────────
Rust                   24       6,051       590       490      4,971        633
TOML                    2          49         6         0         43          0
License                 1          21         4         0         17          0
Markdown                1         226        54         0        172          0
Shell                   1          15         4         7          4          0
───────────────────────────────────────────────────────────────────────────────
Total                  29       6,362       658       497      5,207        633
───────────────────────────────────────────────────────────────────────────────
Estimated Cost to Develop (organic) $152,777
Estimated Schedule Effort (organic) 6.74 months
Estimated People Required (organic) 2.02
───────────────────────────────────────────────────────────────────────────────
Processed 217,355 bytes, 0.217 megabytes (SI)
───────────────────────────────────────────────────────────────────────────────
```

😉👉⭐️

![star-history](https://api.star-history.com/svg?repos=nlkli/recol)

[LICENSE](LICENSE)
