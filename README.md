# recol

A fast CLI utility for managing color themes and fonts across your terminal and [Neovim](https://neovim.io).

![recol-demo-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo.gif)

* 550+ prebuilt color schemes from the iTerm2 Color Schemes repository:
  [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes?utm_source=chatgpt.com)
* Neovim theme integration based on the Nightfox theme collection:
  [Nightfox.nvim](https://github.com/EdenEast/nightfox.nvim?utm_source=chatgpt.com)
* Terminal support:

  * [Alacritty](https://alacritty.org/index.html)
  * [Ghostty](https://ghostty.org)
* Font switching support (macOS only)
* Non-destructive configuration updates (only colors/fonts are modified)

### Terminal support notes

- **Alacritty** supports hot configuration reload. Changes are applied immediately without restarting the terminal.
- **Ghostty** requires a manual reload (e.g. `Ctrl + Shift + ,` on Linux or `Cmd + Shift + ,` on macOS).

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

### Help message

```text
recol — quickly change your terminal theme
https://github.com/nlkli/recol
500+ terminal color schemes:
https://github.com/mbadolato/iTerm2-Color-Schemes
Supported targets: alacritty, ghostty, neovim.

recol <TNAME> -f <FNAME> # set a specific theme and font (fuzzy match)
recol -rdF               # random dark theme and random Nerd Font
recol -rls               # show random light theme palette

Options:
  <TNAME>, -t, --theme <NAME>
      Apply a theme by name (fuzzy matching)
  -r, --rand
      Apply a random theme
  -d, --dark
  -l, --light
      Filter to dark or light themes 
      (used with --rand, --theme or --theme-list)

  --nvim-config <PATH>
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

😉👉⭐️
