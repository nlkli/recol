# recol

A fast CLI utility for managing color themes and fonts across your terminal and Neovim.

![recol-demo-o](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-o.gif)

- Includes 500+ prebuilt color schemes from the iTerm2 Color Schemes repository: 
    https://github.com/mbadolato/iTerm2-Color-Schemes
- Neovim color configuration is derived from the nightfox.nvim theme collection: 
    https://github.com/EdenEast/nightfox.nvim
- Terminal color themes are currently supported for Alacritty only
- Font switching is implemented on macOS only
- Makes non-destructive changes to existing configuration, affecting colors only
- ~600 KB binary

**Note:** Unlike Alacritty, Neovim doesn't support hot reload. To apply the new theme, either restart Neovim or use a keybinding to reload your config:

```lua
vim.keymap.set("n", "<leader>R", ":source ~/.config/nvim/init.lua<CR>")
```

### Build from source

```sh
git clone https://github.com/nlkli/recol.git
cd recol
cargo build --release
cp target/release/recol /usr/local/bin/
```

### Help message

```text
recol — quickly change your terminal theme
https://github.com/nlkli/recol
500+ terminal color schemes:
https://github.com/mbadolato/iTerm2-Color-Schemes
Supported targets: alacritty, neovim.

  recol <TNAME> -f <FNAME> # set a specific theme and font (fuzzy match)
  recol -rdF               # random dark theme and random Nerd Font
  recol -rls               # show random light theme palette

Options:
  [ ], -t, --theme <NAME>
      Apply a theme by name (fuzzy matching)
  -r, --rand
      Apply a random theme
  -d, --dark
  -l, --light
      Filter to dark or light themes 
      (used with --rand, --theme or --theme-list)

  --alacritty_config <PATH>
      default: ~/.config/alacritty/alacritty.toml
  --nvim_config <PATH>
      default: ~/.config/nvim/init.lua

  -f, --font <NAME>
      Set font family by name (fuzzy matching)
  -F, --font-rand   
      Pick a random Nerd Font

  --theme-list  List available themes
  --font-list   List available Nerd Fonts

  -s, --show
    Show the theme color palette without applying it
      --show-json   Output theme as JSON
      --show-toml   Output theme as TOML
      --show-fmt    Output theme in rustfmt-style format

  -h, --help
  -V, --version
```
