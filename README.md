# recol

**Switch your terminal, editor, and application color schemes from one command — no manual config editing.** Pick from 600+ prebuilt schemes with instant fuzzy search and apply them across multiple supported targets.

![recol-demo-interactive-mode-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-interactive-mode.gif)

- **600+ color schemes** from [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes)
- **Targets support:** [Ghostty](https://ghostty.org), [Kitty](https://sw.kovidgoyal.net/kitty/), [Alacritty](https://alacritty.org), [WezTerm](https://wezterm.org), [Neovim](https://neovim.io), [Vim](https://www.vim.org), [Pi](https://github.com/earendil-works/pi)
- **Neovim theme integration** based on [Nightfox.nvim](https://github.com/EdenEast/nightfox.nvim)
- **Non-destructive** — only color/font values are modified, nothing else in your config

### Terminal support notes

- **Ghostty** requires a manual reload (e.g. `Ctrl + Shift + ,` on Linux or `Cmd + Shift + ,` on macOS).
- **Kitty** requires a manual reload (`Ctrl + Shift + F5` by default, or `Ctrl + Cmd + ,` on macOS; see [Kitty documentation](https://sw.kovidgoyal.net/kitty/conf/#shortcut-kitty.Reload-config)).
- **Alacritty**, **WezTerm** supports hot configuration reload. Changes are applied immediately without restarting the terminal.

### Neovim integration
 
Neovim doesn't support hot theme reload, so add a keybinding or command to re-source your config after switching:
 
```lua
vim.keymap.set("n", "<leader>R", ":source ~/.config/nvim/init.lua<CR>") -- or :restart<CR> nvim v0.13
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
            width = width, height = height,
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
            return launch_interactive_mode()
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

Run `./fetch.sh` to download the latest themes from [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes) to `./colorschemes`. Add custom themes there or filter unwanted themes in `build.rs`.

Build the embedded color schemes binary:

```sh
RECOL_BUILD_COLORSCHEMES_BIN=1 cargo build --release
```

For a custom themes directory, use `RECOL_GHOSSTY_THEMES_DIR`:

```sh
RECOL_GHOSSTY_THEMES_DIR=/path/to/themes \
RECOL_BUILD_COLORSCHEMES_BIN=1 \
cargo build --release
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

### Color Adjustments

![recol-demo-adjust-gif](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-adjust.gif)

Adjust theme colors with `--adjust "group.adjustment=value,..."`. Supports brightness, contrast, saturation, hue, exposure, gamma, temperature, tint, normalize and more. Apply to UI elements, specific colors, or the full ANSI palette using short group names (e.g. pal, bg, red).

```sh
recol --adjust help
```

In interactive mode you can change adjustments live and see the preview update instantly.

### Generate a theme from a media file (beta)

![recol-demo-media-to-theme](https://github.com/nlkli/assetsrepo/blob/main/recol.demo/recol-demo-media-to-theme.gif)

`recol` can derive a color scheme from any image/video frame. Pass a media file with `--media` and recol builds a theme automatically:

```sh
recol --media ~/Pictures/Sunset.png            # generate and apply a theme from an image
recol -m ~/Videos/X.gif --json                 # generate a theme and output it as JSON
recol -m ~/Photo/Landscape.jpg --palettegen 12 # print the generated palette
recol -m ~/Photo/Logo.svg --palettegen 24 -s   # show extracted colors
recol -m ~/Photo/Cat.jpg --palettegen 250 -j   # print the generated palette as JSON
recol -m ~/Photo/Tree.png -a t.e=9,bb.b=-12    # generate and apply with color adjust
```

**Requirements:** [ffmpeg](https://ffmpeg.org) must be installed and available on `PATH`.

This feature requires no additional Cargo/Rust dependencies. recol simply invokes the ffmpeg binary already installed on the system and uses its palettegen functionality to extract colors.

**How it works:**

1. `recol` invokes the system-installed `ffmpeg` and uses `palettegen=max_colors=N` to extract the dominant palette from the media into a PPM file.
2. `recol` reads the unique colors from the generated PPM and maps them to the 16 ANSI slots.
3. The theme auto-detects light vs. dark and aligns the derived colors to a shared luminance for visual consistency.

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
Rust                   24       5,919       575       417      4,927        624
TOML                    2          49         6         0         43          1
License                 1          21         4         0         17          0
Markdown                1         231        55         0        176          0
Shell                   1          15         4         7          4          0
───────────────────────────────────────────────────────────────────────────────
Total                  29       6,235       644       424      5,167        625
───────────────────────────────────────────────────────────────────────────────
Estimated Cost to Develop (organic) $151,545
Estimated Schedule Effort (organic) 6.71 months
Estimated People Required (organic) 2.00
───────────────────────────────────────────────────────────────────────────────
Processed 212,351 bytes, 0.212 megabytes (SI)
───────────────────────────────────────────────────────────────────────────────
```

😉👉⭐️

![star-history](https://api.star-history.com/svg?repos=nlkli/recol)

[LICENSE](LICENSE)
