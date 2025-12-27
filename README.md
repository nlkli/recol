# recol

A fast CLI utility for managing color themes and fonts across your terminal and Neovim.

- Includes 425+ prebuilt color schemes from the iTerm2 Color Schemes repository: 
    https://github.com/mbadolato/iTerm2-Color-Schemes
- Neovim color configuration is derived from the nightfox.nvim theme collection: 
    https://github.com/EdenEast/nightfox.nvim
- Terminal color themes are currently supported for Alacritty only
- Font switching is implemented on macOS only
- Makes non-destructive changes to existing configuration, affecting colors only
- ~600 KB binary

### Build from source

```sh
git clone https://github.com/nlkli/recol.git
cd recol
cargo build --release
cp target/release/recol /usr/local/bin/
```

### Help message

```text
Quickly change your terminal theme.
Over 425 terminal colorschemes.
https://github.com/mbadolato/iTerm2-Color-Schemes
Supported targets: Alacritty, Neovim.

  recol <TNAME> -f <FNAME> # set a specific theme and font (fuzzy match)
  recol -rdF               # random dark theme and random Nerd Font
  recol -rls               # show random light theme palette

Options:
  [ ], -t, --theme <NAME>
          Apply a theme by name (fuzzy matching)
  -r, --rand
          Apply a random theme
  -d, --dark
    Filter to dark themes (used with --rand, --theme or --theme-list)
  -l, --light   Filter to light themes

  -f, --font <NAME>
          Set font family by name (fuzzy matching)
      -F, --font-rand   Pick a random Nerd Font

  --theme-list  List available themes
  --font-list   List available Nerd Fonts

  -s, --show
    Show the theme color palette without applying it
      --show-toml   Output theme as TOML
      --show-fmt    Output theme in rustfmt-style format

  -h, --help    Print help
  -V, --version Print version
```
