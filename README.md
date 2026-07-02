# recol

**Switch your terminal and Neovim color theme from one command - no manual config editing.**

Pick from 570+ prebuilt schemes with instant fuzzy search - from your shell or an interactive picker.

![recol-demo-interactive-mode-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-interactive-mode.gif)

- **570+ color schemes** from [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes)
- **Neovim theme integration** based on [Nightfox.nvim](https://github.com/EdenEast/nightfox.nvim)
- **Terminal support:** [Ghostty](https://ghostty.org), [Alacritty](https://alacritty.org), [WezTerm](https://wezterm.org)
- **Font switching** (macOS only)
- **Non-destructive** — only color/font values are modified, nothing else in your config
- **Minimal dependencies** — see [Cargo.toml](Cargo.toml)

### Terminal support notes

- **Ghostty** requires a manual reload (e.g. `Ctrl + Shift + ,` on Linux or `Cmd + Shift + ,` on macOS).
- **Alacritty**, **WezTerm** supports hot configuration reload. Changes are applied immediately without restarting the terminal.

## Neovim integration
 
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
 
`:RecolOpen` launches `recol` in a floating window; `:Recol <args>` runs it directly.
 
![recol-nvim-integration-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-nvim-integration.gif)
 
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
        local is_interactive_mode = vim.tbl_contains(args, "-i") or vim.tbl_contains(args, "--interactive")
 
        if is_interactive_mode then
            launch_interactive_mode()
            return
        end
        vim.cmd("!recol " .. opts.args)
        vim.cmd("source ~/.config/nvim/init.lua")
    end, { nargs = "*" })
 
    vim.api.nvim_create_user_command("RecolOpen", function()
        launch_interactive_mode()
    end, { nargs = 0 })
end
```

## Install
 
**Cargo:**
```sh
cargo install --git https://github.com/nlkli/recol --branch main --force
```
 
**From source:**
```sh
git clone https://github.com/nlkli/recol
cd recol
cargo build --release
cp target/release/recol /usr/local/bin/
```

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
550+ color schemes:
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
  ↑ / k / -    Move selection up
  ↓ / j / +    Move selection down
  g            Jump to first theme
  G            Jump to last theme
  Ctrl+u       Scroll up half a page
  Ctrl+d       Scroll down half a page

FILTER & SEARCH
  / : i        Enter filter/search mode
  Esc / Enter  Leave filter mode
  Backspace    Delete last filter character
  f            Filter by selected theme (match 1st word)

LIST ACTIONS
  d / l        Keep only dark themes
  s            Shuffle the list
  r            Reverse the list
  Space        Reset list (show all themes)

GENERAL
  Enter        Apply selected theme
  ? / h        Toggle this help screen
  q / Ctrl+c   Quit

Press any key to return · q to quit
```

### Demo

![recol-demo-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo.gif)

### Screenshots

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
    ├── targets
    │   ├── alacritty.rs
    │   ├── ghostty.rs
    │   ├── mod.rs
    │   ├── nvim.rs
    │   └── wezterm.rs
    ├── tmpstore.rs
    └── utils.rs

5 directories, 26 files
```

### SCC

```text
───────────────────────────────────────────────────────────────────────────────
Language            Files       Lines    Blanks  Comments       Code Complexity
───────────────────────────────────────────────────────────────────────────────
Rust                   18       3,878       417       322      3,139        322
TOML                    2          46         5         0         41          1
License                 1          21         4         0         17          0
Markdown                1         297        62         0        235          0
Shell                   1           8         2         1          5          0
───────────────────────────────────────────────────────────────────────────────
Total                  23       4,250       490       323      3,437        323
───────────────────────────────────────────────────────────────────────────────
Estimated Cost to Develop (organic) $98,760
Estimated Schedule Effort (organic) 5.71 months
Estimated People Required (organic) 1.54
───────────────────────────────────────────────────────────────────────────────
Processed 143335 bytes, 0.143 megabytes (SI)
───────────────────────────────────────────────────────────────────────────────
```

😉👉⭐️

![star-history](https://api.star-history.com/svg?repos=nlkli/recol)
