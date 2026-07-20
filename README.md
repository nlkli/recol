# recol

**Switch your terminal and Neovim color theme from one command - no manual config editing.** Pick from 590+ prebuilt schemes with instant fuzzy search - from your shell or an interactive picker.

![recol-demo-interactive-mode-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-interactive-mode.gif)

- **590+ color schemes** from [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes)
- **Neovim theme integration** based on [Nightfox.nvim](https://github.com/EdenEast/nightfox.nvim)
- **Terminal support:** [Ghostty](https://ghostty.org), [Alacritty](https://alacritty.org), [WezTerm](https://wezterm.org)
- **Font switching** (macOS only)
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

### Help Message

```text
CLI utility for changing the color scheme
https://github.com/nlkli/recol
590+ color schemes:
https://github.com/mbadolato/iTerm2-Color-Schemes

Supported targets: alacritty, ghostty, wezterm, neovim.

Usage: recol [OPTIONS] [THEME_NAME]

Options:
  -t, --theme <NAME>
      Apply a theme by name (fuzzy matching)
  -r, --rand
      Apply a random theme
  -d, --dark
  -l, --light
  -c, --contains <STR>
      Filter themes by dark, light or name substring
      (used with --rand, --theme or --theme-list)
  -a, --adjust <SPEC|PATH>
      Apply color adjustments (see --adjust help) [env: RECOL_ADJUST]
      Format: "group.adjustment=value,..."
  -i, --interactive
      Browse and apply themes interactively
  -f, --font <NAME>
      Set font family by name (fuzzy matching)
  -F, --font-rand
      Pick a random Nerd Font
  -T, --target <TARGET>
      Apply for specific target
  --theme-list  List available themes
  --font-list   List available Nerd Fonts
  -s, --show
      Show the theme color palette without applying it
  -j, --json    Output theme/list as JSON
  -h, --help
  -V, --version
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

FILTER & SEARCH
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

Adjust theme colors with `--adjust "group.adjustment=value,..."`. Supports lightness, contrast, saturation, hue, exposure, gamma, temperature, tint, black/white points and more. Apply to UI elements, specific colors, or the full ANSI palette using short group names (e.g. pal, bg, red).

In interactive mode you can change adjustments live and see the preview update instantly.

Help Message:

```text
Color adjustments: --adjust "group.adjustment=value,..."  [env: RECOL_ADJUST]
  Apply one or more transformations to theme colors.
Quick start:
  --adjust "brightness=-10"       Darken whole theme slightly
  --adjust "saturation=20"        Boost all colors
  --adjust "pal.hue=180"          Rotate ANSI palette hues
  --adjust "ui.exposure=-15,text.contrast=10"
  --adjust "sel-bg.brightness=10,cur-bg.sat=30"
  --adjust "temperature=20,tint=-10"
  --adjust "pal.gamma=0.8,bb.fade=-20"
  --adjust "pal.saturation-cap=30"  Tame overly vivid ANSI colors
  --adjust "preset.txt"           Load adjustments from file
  --adjust "_"                    Reset all adjustments
Groups (optional, defaults to All):
  u/ui         UI colors (fg, bg)
  b/bg         Backgrounds (base, sel, cursor)
  f/fg         Foregrounds (base, sel, cursor)
  s/sel        Selection colors
  c/cur        Cursor colors
  bb/base-bg   Base background only
  bf/base-fg   Base foreground only
  sb/sel-bg    Selection background
  sf/sel-fg    Selection foreground
  cb/cur-bg    Cursor background
  cf/cur-fg    Cursor foreground
  p/pal        All ANSI palette colors
  t/text       Foregrounds + palette
  black,red,green,yellow,blue,magenta,cyan,white,orange,pink
  Named ANSI colors (normal + bright)
Color/Tone adjustments:
  b/br/brightness=N     HSL lightness shift, toward white/black (-100..100)
  e/exposure=N         Linear-light scale (-100..100)
  like a camera: highlights move more than shadows
  c/contrast=N         HSL contrast around midpoint (-100..100)
  g/gamma=N            Gamma correction (0.25..4.0, 1.0 = unchanged)
  f/fade=N             Blend toward black/white (-100..100)
  negative shades darker, positive whitens
  i/invert=1           Flip light/dark (value ignored)
  s/sat/saturation=N   HSV saturation, toward full/gray (-100..100, -100 = grayscale)
  sc/saturation-cap=N  Cap the most saturated colors only (0..100)
  v/vib/vibrance=N     Saturation that spares already-vivid colors (-100..100)
  h/hue=N              Hue rotation, degrees (-180..180)
  t/temp/temperature=N Blue ↔ Orange white balance (-100..100)
  ti/tint=N            Green ↔ Magenta white balance (-100..100)
```

### Demo & Screenshots

![recol-demo-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo.gif)

![recol-demo-img-1](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-1.jpg)

![recol-demo-img-2](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-2.jpg)

![recol-demo-img-3](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-3.jpg)

![recol-demo-img-4](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-4.jpg)

![recol-demo-img-5](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-5.jpg)

![recol-demo-img-6](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-6.jpg)

![recol-demo-img-7](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-7.jpg)

![recol-demo-img-8](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-8.jpg)

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
    │   └── wezterm.rs
    └── utils.rs

5 directories, 27 files
```

### SCC

```text
───────────────────────────────────────────────────────────────────────────────
Language            Files       Lines    Blanks  Comments       Code Complexity
───────────────────────────────────────────────────────────────────────────────
Rust                   19       4,717       489       385      3,843        393
TOML                    2          46         5         0         41          1
License                 1          21         4         0         17          0
Markdown                1         363        66         0        297          0
Plain Text              1         205        63         0        142          0
Shell                   1           8         2         1          5          0
───────────────────────────────────────────────────────────────────────────────
Total                  25       5,360       629       386      4,345        394
───────────────────────────────────────────────────────────────────────────────
Estimated Cost to Develop (organic) $126,323
Estimated Schedule Effort (organic) 6.27 months
Estimated People Required (organic) 1.79
───────────────────────────────────────────────────────────────────────────────
Processed 191144 bytes, 0.191 megabytes (SI)
───────────────────────────────────────────────────────────────────────────────
```

😉👉⭐️

![star-history](https://api.star-history.com/svg?repos=nlkli/recol)

[LICENSE](LICENSE)
