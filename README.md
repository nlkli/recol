# recol

A fast CLI utility for managing color themes and fonts across your terminal and [Neovim](https://neovim.io).

![recol-demo-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo.gif)

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

### Help message

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
  -f, --font <NAME>
      Set font family by name (fuzzy matching)
  -F, --font-rand
      Pick a random Nerd Font
  --theme-list  List available themes
  --font-list   List available Nerd Fonts
  -s, --show
      Show the theme color palette without applying it
  -j, --json    Output theme as JSON
  -h, --help
  -V, --version
```

---

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

### Tree

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
    ├── main.rs
    ├── targets
    │   ├── alacritty.rs
    │   ├── ghostty.rs
    │   ├── mod.rs
    │   ├── nvim.rs
    │   └── wezterm.rs
    ├── tmpstore.rs
    └── utils.rs

5 directories, 25 files
```

### SCC

```text
───────────────────────────────────────────────────────────────────────────────
Language            Files       Lines    Blanks  Comments       Code Complexity
───────────────────────────────────────────────────────────────────────────────
Rust                   17       2,967       321       310      2,336        225
TOML                    2          37         4         0         33          1
License                 1          21         4         0         17          0
Markdown                1         161        38         0        123          0
Shell                   1           5         1         1          3          2
───────────────────────────────────────────────────────────────────────────────
Total                  22       3,191       368       311      2,512        228
───────────────────────────────────────────────────────────────────────────────
Estimated Cost to Develop (organic) $71,058
Estimated Schedule Effort (organic) 5.04 months
Estimated People Required (organic) 1.25
───────────────────────────────────────────────────────────────────────────────
Processed 107152 bytes, 0.107 megabytes (SI)
───────────────────────────────────────────────────────────────────────────────
```

😉👉⭐️
