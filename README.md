# recol

A fast CLI utility for managing color themes and fonts across your terminal and [Neovim](https://neovim.io).

![recol-demo-interactive-mode-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-interactive-mode.gif)

* 550+ prebuilt color schemes from the iTerm2 Color Schemes repository:
  [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes)
* Neovim theme integration based on the Nightfox theme collection:
  [Nightfox.nvim](https://github.com/EdenEast/nightfox.nvim)
* Terminal support:

  * [Ghostty](https://ghostty.org)
  * [Alacritty](https://alacritty.org/index.html)
  * [WezTerm](https://wezterm.org/index.html)
* Font switching support (macOS only)
* Non-destructive configuration updates (only colors/fonts are modified)
* Minimal dependency footprint: [Cargo.toml](Cargo.toml)

### Terminal support notes

- **Ghostty** requires a manual reload (e.g. `Ctrl + Shift + ,` on Linux or `Cmd + Shift + ,` on macOS).
- **Alacritty**, **WezTerm** supports hot configuration reload. Changes are applied immediately without restarting the terminal.

### Neovim integration

**Neovim** does not support hot theme reloading. To apply the new theme, either restart Neovim or use a keybinding to reload your config:

```lua
vim.keymap.set("n", "<leader>R", ":source ~/.config/nvim/init.lua<CR>")
```

Add a simple command to apply themes from within Neovim:

```lua
if vim.fn.executable("recol") == 1 then
    vim.api.nvim_create_user_command("Recol", function(opts)
        vim.cmd("!recol " .. opts.args)
        vim.cmd("source ~/.config/nvim/init.lua")
    end, { nargs = "*" })
end
```

### Full Neovim Integration (interactive mode)

![recol-nvim-integration-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-nvim-integration.gif)

- `:RecolOpen` to start Recol in interactive floating mode inside Neovim.
- `:Recol <args>` to run Recol directly from Neovim.

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
        local is_interactive_mode = vim.tbl_contains(vim.split(opts.args, "%s+", { trimempty = true }), "-i")

        if is_interactive_mode then
            launch_interactive_mode()
        end
        vim.cmd("!recol " .. opts.args)
        vim.cmd("source ~/.config/nvim/init.lua")
    end, { nargs = "*" })

    vim.api.nvim_create_user_command("RecolOpen", function()
        launch_interactive_mode()
    end, { nargs = 0 })
end
```

### Build from source

```sh
git clone https://github.com/nlkli/recol
cd recol
cargo build --release
cp target/release/recol /usr/local/bin/
```

### Fetch and rebuild color schemes

To download the latest themes from [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes) and recompile the binary blob:

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

### Interactive Mode Keybindings

```text
Normal Mode
Enter       apply theme
j / ↓       down
k / ↑       up
gg / G      top/bottom
Ctrl+d/u    half page down/up
/ or :      search
q / Ctrl+c  quit

Input Mode
Enter / Esc back
Backspace   delete
Ctrl+c      quit
typing      filter
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

![recol-demo-img-9](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-9.jpg)

![recol-demo-img-10](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-10.jpg)

![recol-demo-img-11](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-11.jpg)

![recol-demo-img-12](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-img-12.jpg)

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
Rust                   18       3,728       403       321      3,004        318
TOML                    2          46         5         0         41          1
License                 1          21         4         0         17          0
Markdown                1         283        60         0        223          0
Shell                   1           5         1         1          3          2
───────────────────────────────────────────────────────────────────────────────
Total                  23       4,083       473       322      3,288        321
───────────────────────────────────────────────────────────────────────────────
Estimated Cost to Develop (organic) $94,270
Estimated Schedule Effort (organic) 5.61 months
Estimated People Required (organic) 1.49
───────────────────────────────────────────────────────────────────────────────
Processed 137686 bytes, 0.138 megabytes (SI)
───────────────────────────────────────────────────────────────────────────────
```

😉👉⭐️

![star-history](https://api.star-history.com/svg?repos=nlkli/recol)
