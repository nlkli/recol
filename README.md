# recol

**Switch your terminal and Neovim color theme from one command - no manual config editing.** Pick from 590+ prebuilt schemes with instant fuzzy search - from your shell or an interactive picker.

![recol-demo-interactive-mode-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-interactive-mode.gif)

- **590+ color schemes** from [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes)
- **Targets support:** [Ghostty](https://ghostty.org), [Alacritty](https://alacritty.org), [WezTerm](https://wezterm.org), [Neovim](https://neovim.io), [Vim](https://www.vim.org)
- **Neovim theme integration** based on [Nightfox.nvim](https://github.com/EdenEast/nightfox.nvim)
- **Non-destructive** — only color/font values are modified, nothing else in your config
- **Minimal dependencies** — see [Cargo.toml](Cargo.toml)

### Terminal support notes

- **Ghostty** requires a manual reload (e.g. `Ctrl + Shift + ,` on Linux or `Cmd + Shift + ,` on macOS).
- **Alacritty**, **WezTerm** supports hot configuration reload. Changes are applied immediately without restarting the terminal.

### Neovim integration
 
Neovim doesn't support hot theme reload, so add a keybinding or command to re-source your config after switching:
 
```lua
vim.keymap.set("n", "<leader>R", ":source ~/.config/nvim/init.lua<CR>")
```
 
Run `recol` directly from Neovim:
 
```lua
if vim.fn.executable("recol") == 1 then
    vim.api.nvim_create_user_command("Recol", function(opts)
        vim.cmd("!recol " .. opts.args)
        vim.cmd("source ~/.config/nvim/init.lua")
    end, { nargs = "*" })
end
```
 
### Interactive mode inside Neovim
 
![recol-nvim-integration-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-nvim-integration.gif)

`:RecolOpen` launches `recol` in a floating window; `:Recol <args>` runs it directly.
 
```lua
if vim.fn.executable("recol") == 1 then
    local launch_interactive_mode = function()
        local width = math.floor(vim.o.columns * 0.75)
        local height = math.floor(vim.o.lines * 0.75)
        local buf = vim.api.nvim_create_buf(false, true)
        local win = vim.api.nvim_open_win(buf, true, {
            relative = "editor",
            width = width,
            height = height,
            row = math.floor((vim.o.lines - height - 3) / 2),
            col = math.floor((vim.o.columns - width) / 2),
            border = "rounded",
            title = " Recol ",
            title_pos = "center",
        })
        vim.bo[buf].bufhidden = "wipe"
        vim.fn.termopen({ "recol", "-i", "--quit-on-select" }, {
            on_exit = function()
                vim.schedule(function()
                    if vim.api.nvim_win_is_valid(win) then
                        vim.api.nvim_win_close(win, true)
                    end
                    vim.cmd.source("~/.config/nvim/init.lua")
                end)
            end,
        })
        vim.cmd.startinsert()
    end
 
    vim.api.nvim_create_user_command("Recol", function(opts)
        local args = vim.split(opts.args, "%s+", { trimempty = true })
        local is_interactive_mode = vim.tbl_contains(args, "-i") or 
            vim.tbl_contains(args, "--interactive")
        if is_interactive_mode then
            launch_interactive_mode()
            return
        end
        vim.cmd("!recol " .. opts.args)
        vim.cmd.source("~/.config/nvim/init.lua")
    end, { nargs = "*" })
 
    vim.api.nvim_create_user_command("RecolOpen", function()
        launch_interactive_mode()
    end, { nargs = 0 })
end
```

### Build From Source

```sh
git clone https://github.com/nlkli/recol
cd recol
cargo build --release
cp target/release/recol /usr/local/bin/
```

### Cargo Install
 
```sh
cargo install --git https://github.com/nlkli/recol --branch main --force
```

### Pre-built binaries

Download the latest release binary for your platform from the [Releases](https://github.com/nlkli/recol/releases) page.

### Fetch and rebuild color schemes

Fetch the latest themes from [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes) and rebuild the embedded binary:

```sh
RECOL_FETCH_GHOSSTY_THEMES=1 \
RECOL_BUILD_COLORSCHEMES_BIN=1 \
cargo build --release
```

### Custom color schemes

To build with your own themes, point `RECOL_GHOSSTY_THEMES_DIR` to your themes directory:

```sh
RECOL_GHOSSTY_THEMES_DIR=/path/to/your/themes \
RECOL_BUILD_COLORSCHEMES_BIN=1 \
cargo build --release
```

Themes use the [Ghostty config format](https://github.com/mbadolato/iTerm2-Color-Schemes/blob/master/ghostty/0x96f) (no file extension). The filename becomes the theme name.

To add your themes to the default collection, place them in `./colorschemes` (run `./fetch.sh` first to populate it). Filter unwanted themes in `build.rs`:

```rust
recol_lib::build_colorschemes_bin(
    ...,
    |name| !["theme_to_exclude"].contains(&name),
)
```

### Help Message

```text
CLI utility for changing the color scheme
https://github.com/nlkli/recol

Supported targets:
alacritty, ghostty, wezterm, neovim, vim.

Usage: recol [OPTIONS] [THEME_NAME]

Options:
  -t, --theme <NAME>
      Apply a theme by name (fuzzy matching)
  -r, --rand
      Apply a random theme
  -d, --dark; -l, --light
  -c, --contains <STR>
      Filter themes by dark, light or name substring
      (used with --rand, --theme or --theme-list)
  -a, --adjust <SPEC|PATH> [env: RECOL_ADJUST]
      Apply color adjustments (see --adjust help)
      Format: "group.adjustment=value,..."
  -i, --interactive
      Browse and apply themes interactively
  -f, --font <NAME>
      Set font family by name (fuzzy matching)
  -F, --font-rand
      Pick a random Nerd Font
  -T, --target <Name>
      Apply for specific target (see --target list)
  -L, --theme-list  List available themes
  --font-list       List available Nerd Fonts
  -s, --show
      Show the theme color palette without applying it
  -j, --json  Output theme/list as JSON
  -h, --help; -V, --version; --logo
```

### Usage Examples

```sh
recol londonsohonight         # fuzzy match - applies closest theme by name
recol -rd --contains Gruvbox  # random dark theme with "Gruvbox" in name
recol --theme-list -l --json  # list light themes as JSON
recol dracula --dark --show   # preview palette without applying
recol -t tokyo --json         # print tokyo theme as JSON
recol terafox --target nvim   # apply theme for specific target
recol                         # print current theme name (add --show or --json for more)
```

### TUI Keybindings

```text
NAVIGATION
  ↑ / k / -      Move selection up
  ↓ / j / +      Move selection down
  g / G          Jump to first / last
  Ctrl+ u / d    Half page up / down

INPUT & FILTER
  / : i          Enter input mode
  a              Enter adjust input mode
  Backspace      Delete last character
  Esc / Enter    Exit filter mode
  f              Filter by first word (family)

LIST ACTIONS
  s / r          Shuffle / Reverse order
  d / l          Dark / Light only
  h              Recently applied (history)
  Space          Reset filters (show all)

GENERAL
  Enter          Apply theme
  ? / H          Open this help
  q / Ctrl+c     Quit

CLI ARGS
  --quit-on-select
  --init-input
  --init-help
```

### Color Adjustments

![recol-demo-adjust-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-adjust.gif)

Adjust theme colors with `--adjust "group.adjustment=value,..."`. Supports brightness, contrast, saturation, hue, exposure, gamma, temperature, tint, normalize and more. Apply to UI elements, specific colors, or the full ANSI palette using short group names (e.g. pal, bg, red).

In interactive mode you can change adjustments live and see the preview update instantly.

Help Message:

```text
Color adjustments: --adjust "group.adjustment=value,..."  [env: RECOL_ADJUST]
  Apply one or more transformations to theme colors.

Quick start:
  --adjust "brightness=-10"      Darken entire theme
  --adjust "saturation=20"       Boost all colors evenly
  --adjust "temperature=20,tint=-10"   Warmer + slight green tint
  --adjust "pal.hue=180"         Rotate ANSI palette hues
  --adjust "sel.invert,cur.hue=90"  Invert selection, green cursor
  --adjust "pal.normalize=50,pal.vibrance=-20"  Unify palette & desaturate
  --adjust "preset.txt"          Load adjustments from file
  --adjust "_"                   Reset all adjustments

Groups (optional, defaults to All):
  u/ui            All UI (fg + bg + sel + cur)
  b/bg            All backgrounds (base, sel, cursor)
  f/fg            All foregrounds (base, sel, cursor)
  s/sel           Selection (bg + fg)
  c/cur           Cursor (bg + fg)
  bb/base-bg      Base background
  bf/base-fg      Base foreground
  sb/sel-bg       Selection background
  sf/sel-fg       Selection foreground
  cb/cur-bg       Cursor background
  cf/cur-fg       Cursor foreground
  p/pal           All 16 ANSI colors
  t/text          All foregrounds + palette
  black/red/green/yellow/blue/magenta/cyan/white
  Standard ANSI (normal + bright)
  orange/pink     Extra named colors

Adjustments (all values -100..100 unless noted):
  b/brightness=N        HSL lightness shift
  e/exposure=N          Photographic exposure (linear light scale)
  c/contrast=N          Contrast around midpoint
  g/gamma=N             Gamma curve (midtones, preserves black/white)
  f/fade=N              Blend RGB toward black (-) or white (+)
  i/invert              Flip lightness around midpoint (value ignored)
  s/sat/saturation=N    Uniform saturation
  v/vib/vibrance=N      Smart saturation (protects vivid, boosts muted)
  h/hue=N               Hue shift (maps to -180°..180°)
  t/temp/temperature=N  Blue↔Yellow axis (negative = cooler)
  ti/tint=N             Green↔Magenta axis (negative = greener)
  n/norm/normalize=N    Pull lightness toward group average
  nb/norm-both=N        Pull lightness + chroma toward average
  nc/norm-chroma=N      Pull chroma toward group average
```

### Demo & Screenshots

![recol-demo-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo.gif)

![recol-demo-img-1](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-1.png)

![recol-demo-img-2](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-2.png)

![recol-demo-img-3](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-3.png)

![recol-demo-img-4](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-4.png)

### Project Tree

```text
.
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── fetch.sh
├── LICENSE
├── README.md
├── recol-lib
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       ├── adjustments.rs
│       ├── collection.rs
│       ├── color.rs
│       ├── colorschemes.bin
│       ├── error.rs
│       ├── fuzzy.rs
│       ├── lib.rs
│       └── theme.rs
└── src
    ├── cli.rs
    ├── font.rs
    ├── interactive.rs
    ├── main.rs
    ├── store.rs
    ├── targets
    │   ├── alacritty.rs
    │   ├── ghostty.rs
    │   ├── mod.rs
    │   ├── nvim.rs
    │   ├── vim.rs
    │   └── wezterm.rs
    └── utils.rs

5 directories, 28 files
```

### SCC

```text
───────────────────────────────────────────────────────────────────────────────
Language            Files       Lines    Blanks  Comments       Code Complexity
───────────────────────────────────────────────────────────────────────────────
Rust                   20       5,106       512       386      4,208        405
TOML                    2          46         5         0         41          1
License                 1          21         4         0         17          0
Markdown                1         370        71         0        299          0
Shell                   1           8         2         1          5          0
───────────────────────────────────────────────────────────────────────────────
Total                  25       5,551       594       387      4,570        406
───────────────────────────────────────────────────────────────────────────────
Estimated Cost to Develop (organic) $133,200
Estimated Schedule Effort (organic) 6.39 months
Estimated People Required (organic) 1.85
───────────────────────────────────────────────────────────────────────────────
Processed 188835 bytes, 0.189 megabytes (SI)
───────────────────────────────────────────────────────────────────────────────
```

😉👉⭐️

![star-history](https://api.star-history.com/svg?repos=nlkli/recol)

[LICENSE](LICENSE)
