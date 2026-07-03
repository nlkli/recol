use crate::{cli::Args, targets, tmpstore};
use crossterm::{cursor, event, execute, style, terminal as term};
use recol_lib as lib;
use std::{
    fmt::Debug,
    io::{self, Write},
};

const DEFAULT_SCROLLOFF: usize = 6;

/// RAII guard that enables raw mode and the alternate screen on creation,
/// and restores the terminal on drop.
struct TerminalGuard;

impl TerminalGuard {
    fn new() -> io::Result<Self> {
        term::enable_raw_mode()?;
        execute!(io::stdout(), term::EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), term::LeaveAlternateScreen, cursor::Show);
        let _ = term::disable_raw_mode();
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
enum Mode {
    #[default]
    Normal,
    Input,
    Help,
}

type Point = (u16, u16);

#[derive(Debug, Clone, Default)]
struct State {
    /// Terminal dimensions `(cols, rows)`.
    size: Point,
    mode: Mode,
    input_buf: String,
    /// Visual cursor position within the list column.
    cursor: Point,
    list: Vec<lib::LazyTheme>,
    /// Index of the first visible list entry.
    list_offset: usize,
    /// Absolute index of the selected entry in `list`.
    list_index: usize,
    /// Minimum number of visible rows to keep above/below the selection.
    scrolloff: usize,
    current_theme: Option<String>,
    // last_char: Option<char>,
}

impl State {
    fn scroll_list_up(&mut self, n: usize) {
        for _ in 0..n {
            if self.list_index == 0 {
                break;
            }

            let can_scroll = self.list_offset > 0;
            let top_limit = if can_scroll { self.scrolloff } else { 0 };

            if (self.cursor.1 as usize) > top_limit {
                self.cursor.1 -= 1;
                self.list_index -= 1;
            } else if can_scroll {
                self.list_offset -= 1;
                self.list_index -= 1;
            } else {
                break;
            }
        }
    }

    fn scroll_list_down(&mut self, n: usize) {
        for _ in 0..n {
            if self.list_index + 1 >= self.list.len() {
                break;
            }

            let visible_rows = self.size.1.saturating_sub(1) as usize;
            let remaining = self.list.len().saturating_sub(self.list_offset);
            let can_scroll = remaining > visible_rows;

            let max_cursor_row = visible_rows
                .saturating_sub(1)
                .min(remaining.saturating_sub(1));

            let bottom_limit = if can_scroll {
                max_cursor_row.saturating_sub(self.scrolloff)
            } else {
                max_cursor_row
            };

            if (self.cursor.1 as usize) < bottom_limit {
                self.cursor.1 += 1;
                self.list_index += 1;
            } else if can_scroll {
                self.list_offset += 1;
                self.list_index += 1;
            } else {
                break;
            }
        }
    }

    /// Rebuild `list` filtered by the current `input_buf`.
    /// Entries are ordered: exact-prefix > lowercase-prefix > contains > lowercase-contains > fuzzy.
    fn filter_list_by_input(&mut self) {
        self.list.clear();
        if self.input_buf.is_empty() {
            lib::Collection::new().for_each(|t| self.list.push(t));
            return;
        }

        let query = &self.input_buf;

        let filters: &[(&[lib::ThemeFilter], bool)] = &[
            (&[lib::ThemeFilter::StartWith(query)], false),
            (&[lib::ThemeFilter::StartWithLower(query)], true),
            (&[lib::ThemeFilter::Contains(query)], true),
            (&[lib::ThemeFilter::ContainsLower(query)], true),
        ];

        for (filter_set, dedup) in filters {
            lib::Collection::new().filtered(filter_set).for_each(|t| {
                if !dedup || !self.list.contains(&t) {
                    self.list.push(t);
                }
            });
        }

        if self.list.is_empty() {
            self.list
                .extend_from_slice(&lib::Collection::new().fuzzy_search_top_n(
                    query,
                    &[],
                    10,
                    None,
                ));
        }
    }

    fn reset_pos(&mut self) {
        self.list_offset = 0;
        self.list_index = 0;
        self.cursor.1 = 0;
    }
}

/// Wrap `text` in an ANSI truecolor foreground escape using `color`.
#[inline]
fn color_text(text: &str, color: &lib::CssColor) -> String {
    let (r, g, b) = color.color().rgb();
    format!("\x1b[38;2;{r};{g};{b}m{text}")
}

/// A buffer of `(text, color)` pairs that assembles into a fixed-width colored string.
struct PartBuf<'a>(Vec<(String, &'a lib::CssColor)>);

macro_rules! part_buf {
    ($(($text:expr, $color:expr)),* $(,)?) => {
        PartBuf(vec![
            $(
                (($text).to_string(), $color),
            )*
        ])
    };
}

impl<'a> PartBuf<'a> {
    /// Pad or truncate so the visible character count equals `width`, then colorize.
    fn assemble(mut self, width: usize) -> String {
        // Use char counts so we don't split multibyte codepoints.
        let total_chars: usize = self
            .0
            .iter()
            .map(|(s, _)| s.chars().count())
            .collect::<Vec<_>>()
            .iter()
            .sum();

        if total_chars < width {
            if let Some((last, _)) = self.0.last_mut() {
                last.push_str(&" ".repeat(width - total_chars));
            }
        } else if total_chars > width {
            let mut to_remove = total_chars - width;
            while to_remove > 0 {
                let Some((s, _)) = self.0.last_mut() else {
                    break;
                };
                let char_len = s.chars().count();
                if char_len > to_remove {
                    // Truncate at a char boundary.
                    let keep = char_len - to_remove;
                    *s = s.chars().take(keep).collect();
                    break;
                }
                to_remove -= char_len;
                self.0.pop();
            }
        }

        self.0.into_iter().map(|(s, c)| color_text(&s, c)).collect()
    }
}

fn gen_preview(theme: &lib::Theme, col_width: usize) -> Vec<String> {
    let c = theme.colors.clone().into_advanced(None);

    vec![
        part_buf![("// Press ?/h for help", &c.comment)],
        part_buf![
            ("use ", &c.base.magenta),
            ("std", &c.base.cyan),
            ("::", &c.base.blue),
            ("io", &c.base.cyan),
            ("::", &c.base.blue),
            ("Write", &c.base.red),
            (";", &c.fg[1])
        ],
        part_buf![
            ("fn ", &c.base.magenta),
            ("main", &c.base.blue),
            ("() ", &c.fg[1]),
            ("-> ", &c.fg[2]),
            ("Result", &c.base.yellow),
            ("<", &c.fg[2]),
            ("i32", &c.base.cyan),
            (", ", &c.fg[1]),
            ("Box", &c.base.yellow),
            ("<", &c.fg[2]),
            ("dyn ", &c.base.magenta),
            ("std", &c.base.cyan),
            ("::", &c.base.blue),
            ("error", &c.base.cyan),
            ("::", &c.base.blue),
            ("Error", &c.base.red),
            (">>", &c.fg[2]),
            (" {", &c.fg[1])
        ],
        part_buf![
            ("    let ", &c.base.magenta),
            ("mut ", &c.base.yellow),
            ("stdout = ", &c.fg[1]),
            ("std", &c.base.cyan),
            ("::", &c.base.blue),
            ("io", &c.base.cyan),
            ("::", &c.base.blue),
            ("stdout", &c.base.blue),
            ("();", &c.fg[1]),
        ],
        part_buf![
            ("    let ", &c.base.magenta),
            ("theme = ", &c.fg[1]),
            ("recol", &c.base.cyan),
            ("::", &c.base.blue),
            ("current", &c.base.cyan),
            ("();", &c.fg[1]),
        ],
        part_buf![
            ("    write!", &c.base.pink),
            ("(stdout, ", &c.fg[1]),
            (r#""Name: "#, &c.base.green),
            ("{}", &c.base.cyan),
            ("\\n", &c.base.yellow),
            (r#"""#, &c.base.green),
            (", theme", &c.fg[1]),
            (".", &c.fg[2]),
            ("name", &c.base.blue),
            (")", &c.fg[1]),
            ("?", &c.fg[2]),
            (";", &c.fg[1]),
        ],
        part_buf![
            ("    write!", &c.base.pink),
            ("(stdout, ", &c.fg[1]),
            (r#""Palette:"#, &c.base.green),
            ("\\n", &c.base.yellow),
            ("{}", &c.base.cyan),
            ("\\n", &c.base.yellow),
            (r#"""#, &c.base.green),
            (", theme", &c.fg[1]),
            (".", &c.fg[2]),
            ("palette", &c.base.blue),
            (")", &c.fg[1]),
            ("?", &c.fg[2]),
            (";", &c.fg[1]),
        ],
        part_buf![
            ("    Ok", &c.dim.yellow),
            ("(", &c.fg[1]),
            ("42", &c.base.orange),
            (")", &c.fg[1]),
        ],
        part_buf![("}", &c.fg[1])],
        part_buf![("Name: ", &c.fg[1]), (&theme.name, &c.cursor.bg)],
        part_buf![("Palette:", &c.fg[1])],
        part_buf![
            ("  [0]", &c.base.black),
            ("[0]", &c.base.red),
            ("[0]", &c.base.green),
            ("[0]", &c.base.yellow),
            ("[0]", &c.base.blue),
            ("[0]", &c.base.magenta),
            ("[0]", &c.base.cyan),
            ("[0]", &c.base.white),
            ("[0]", &c.base.orange),
            ("[0]", &c.base.pink),
        ],
        part_buf![
            ("  [0]", &c.bright.black),
            ("[0]", &c.bright.red),
            ("[0]", &c.bright.green),
            ("[0]", &c.bright.yellow),
            ("[0]", &c.bright.blue),
            ("[0]", &c.bright.magenta),
            ("[0]", &c.bright.cyan),
            ("[0]", &c.bright.white),
            ("[0]", &c.bright.orange),
            ("[0]", &c.bright.pink),
        ],
        part_buf![
            ("  [0]", &c.dim.black),
            ("[0]", &c.dim.red),
            ("[0]", &c.dim.green),
            ("[0]", &c.dim.yellow),
            ("[0]", &c.dim.blue),
            ("[0]", &c.dim.magenta),
            ("[0]", &c.dim.cyan),
            ("[0]", &c.dim.white),
            ("[0]", &c.dim.orange),
            ("[0]", &c.dim.pink),
        ],
        part_buf![("Selection:", &c.fg[1])],
        part_buf![("  [0]", &c.selection.bg), ("[0]", &c.selection.fg)],
        part_buf![("Cursor:", &c.fg[1])],
        part_buf![("  [0]", &c.cursor.bg), ("[0]", &c.cursor.fg)],
        part_buf![("Background:", &c.fg[1])],
        part_buf![
            ("  [0]", &c.bg[0]),
            ("[0]", &c.bg[1]),
            ("[0]", &c.bg[2]),
            ("[0]", &c.bg[3]),
            ("[0]", &c.bg[4]),
        ],
        part_buf![("Foreground:", &c.fg[1])],
        part_buf![
            ("  [0]", &c.fg[0]),
            ("[0]", &c.fg[1]),
            ("[0]", &c.fg[2]),
            ("[0]", &c.fg[3]),
        ],
        part_buf![("Diff:", &c.fg[1])],
        part_buf![
            ("  [0]", &c.diff.add),
            ("[0]", &c.diff.delete),
            ("[0]", &c.diff.change),
            ("[0]", &c.diff.text),
        ],
    ]
    .into_iter()
    .map(|p| p.assemble(col_width))
    .collect()
}

fn draw_screen(s: &State) -> io::Result<()> {
    const MIN_SIZE: Point = (15, 7);

    let mut stdout = io::stdout();

    execute!(
        stdout,
        term::Clear(term::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    if s.size.0 < MIN_SIZE.0 || s.size.1 < MIN_SIZE.1 {
        let msg = format!(
            "Window too small ({}x{}), need {}x{}",
            s.size.0, s.size.1, MIN_SIZE.0, MIN_SIZE.1
        );

        let col = (s.size.0.saturating_sub(msg.len() as u16)) / 2;
        let row = s.size.1 / 2;

        execute!(
            stdout,
            cursor::MoveTo(col, row),
            style::SetForegroundColor(style::Color::Red),
            style::Print(msg),
            style::ResetColor,
        )?;
        stdout.flush()?;
        return Ok(());
    }

    if s.mode == Mode::Help {
        let sections: &[(&str, &[(&str, &str)])] = &[
            (
                "NAVIGATION",
                &[
                    ("↑ / k / -", "Move selection up"),
                    ("↓ / j / +", "Move selection down"),
                    ("g", "Jump to first theme"),
                    ("G", "Jump to last theme"),
                    ("Ctrl+u", "Scroll up half a page"),
                    ("Ctrl+d", "Scroll down half a page"),
                ],
            ),
            (
                "FILTER & SEARCH",
                &[
                    ("/ : i", "Enter filter/search mode"),
                    ("Esc / Enter", "Leave filter mode"),
                    ("Backspace", "Delete last filter character"),
                    ("f", "Filter by selected theme (match 1st word)"),
                ],
            ),
            (
                "LIST ACTIONS",
                &[
                    ("d / l", "Keep only dark themes"),
                    ("s", "Shuffle the list"),
                    ("r", "Reverse the list"),
                    ("Space", "Reset list (show all themes)"),
                ],
            ),
            (
                "GENERAL",
                &[
                    ("Enter", "Apply selected theme"),
                    ("? / h", "Toggle this help screen"),
                    ("q / Ctrl+c", "Quit"),
                ],
            ),
        ];

        let width = s.size.0 as usize;
        let title = " RECOL —  HELP ";
        let pad_left = width.saturating_sub(title.len()) / 2;
        let pad_right = width.saturating_sub(pad_left + title.len());

        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            style::SetAttribute(style::Attribute::Bold),
            style::SetForegroundColor(style::Color::Black),
            style::SetBackgroundColor(style::Color::Cyan),
            style::Print(" ".repeat(pad_left)),
            style::Print(title),
            style::Print(" ".repeat(pad_right)),
            style::ResetColor,
            style::SetAttribute(style::Attribute::Reset),
        )?;

        let mut row: u16 = 2;
        let max_row = s.size.1.saturating_sub(1);

        for (heading, entries) in sections {
            if row >= max_row {
                break;
            }
            execute!(
                stdout,
                cursor::MoveTo(2, row),
                style::SetAttribute(style::Attribute::Bold),
                style::SetForegroundColor(style::Color::Yellow),
                style::Print(*heading),
                style::ResetColor,
                style::SetAttribute(style::Attribute::Reset),
            )?;
            row += 1;

            for (key, desc) in *entries {
                if row >= max_row {
                    break;
                }
                execute!(
                    stdout,
                    cursor::MoveTo(4, row),
                    style::SetForegroundColor(style::Color::Green),
                    style::Print(format!("{:<14}", key)),
                    style::ResetColor,
                    style::Print(*desc),
                )?;
                row += 1;
            }
            row += 1;
        }

        execute!(
            stdout,
            cursor::MoveTo(0, s.size.1),
            style::SetForegroundColor(style::Color::DarkGrey),
            style::Print("Press any key to return · q to quit"),
            style::ResetColor,
        )?;

        stdout.flush()?;
        return Ok(());
    }

    let mut list_col_width = s.size.0 as usize;
    if s.size.0 >= MIN_SIZE.0 * 2 + 4 {
        list_col_width = list_col_width / 2 - 4;
    }

    let preview_col_width = s.size.0.saturating_sub(list_col_width as u16) as usize + 1;

    if preview_col_width > 0 && !s.list.is_empty() {
        let selected_theme = s.list[s.list_index].into_theme();
        let mut preview_lines = gen_preview(&selected_theme, preview_col_width).into_iter();
        let bg = selected_theme.colors.bg.color().rgb();

        for row in 0..s.size.1.saturating_sub(1) {
            let line = preview_lines.next().unwrap_or_default();
            execute!(
                stdout,
                cursor::MoveTo(list_col_width.saturating_sub(1) as u16, row),
                style::SetBackgroundColor(style::Color::Rgb {
                    r: bg.0,
                    g: bg.1,
                    b: bg.2
                }),
                style::Print(&line),
            )?;
            if line.is_empty() {
                execute!(stdout, style::Print(" ".repeat(preview_col_width)))?;
            }
        }
        execute!(stdout, cursor::MoveTo(0, 0), style::ResetColor)?;
    }

    for (row_idx, theme) in s.list.iter().skip(s.list_offset).enumerate() {
        let mut row_text = format!(
            " {}  {}",
            if theme.is_light { "☀" } else { "⏾" },
            theme.name
        );
        while row_text.chars().count() < list_col_width - 2 {
            row_text.push(' ');
        }
        while row_text.chars().count() > list_col_width - 2 {
            row_text.pop();
        }

        if s.current_theme
            .as_ref()
            .map(|n| n == theme.name)
            .unwrap_or(false)
        {
            execute!(stdout, style::SetForegroundColor(style::Color::Cyan))?;
        }
        let is_selected = row_idx + s.list_offset == s.list_index;
        if is_selected {
            execute!(
                stdout,
                style::SetAttribute(style::Attribute::Reverse),
                style::Print(row_text),
                style::SetAttribute(style::Attribute::NoReverse),
            )?;
        } else {
            execute!(stdout, style::Print(row_text),)?;
        }
        execute!(
            stdout,
            cursor::MoveDown(1),
            cursor::MoveToColumn(0),
            style::ResetColor
        )?;

        if row_idx >= s.size.1.saturating_sub(2) as usize {
            break;
        }
    }

    if s.mode == Mode::Input || !s.input_buf.is_empty() {
        execute!(stdout, cursor::MoveTo(0, s.size.1), cursor::Show)?;
        write!(stdout, ": {}", s.input_buf)?;
    }

    if s.mode == Mode::Normal {
        let status = format!("{}/{}/{}", s.list_offset, s.list_index, s.list.len());
        if status.len() < s.size.0 as usize - 8 {
            execute!(
                stdout,
                cursor::MoveTo(s.size.0 / 2 - status.len() as u16 / 2 - 4, s.size.1),
                style::Print(status),
            )?;
        }
        execute!(
            stdout,
            cursor::MoveTo(s.size.0.saturating_sub(3), s.size.1),
            style::SetForegroundColor(style::Color::DarkGrey),
            style::Print("h/?"),
            style::ResetColor,
        )?;
        execute!(stdout, cursor::MoveTo(s.cursor.0, s.cursor.1), cursor::Hide)?;
    }

    stdout.flush()
}

pub fn run(args: &Args) -> io::Result<()> {
    let _terminal_guard = TerminalGuard::new();

    let mut s = State {
        size: term::size()?,
        list: lib::Collection::new().collect(),
        scrolloff: DEFAULT_SCROLLOFF,
        current_theme: tmpstore::read_theme_history(1).into_iter().next(),
        ..Default::default()
    };

    if args.init_input {
        s.mode = Mode::Input
    } else if args.init_help {
        s.mode = Mode::Help
    }

    draw_screen(&s)?;

    loop {
        match event::read()? {
            event::Event::Key(key) => {
                let ctrl = key.modifiers.contains(event::KeyModifiers::CONTROL);
                match (key.code, &s.mode) {
                    // Normal mode
                    (event::KeyCode::Enter, Mode::Normal) => {
                        if !s.list.is_empty() {
                            continue;
                        }

                        let Some(theme) = s.list.get(s.list_index).map(|v| v.into_theme()) else {
                            continue;
                        };

                        if s.current_theme
                            .as_ref()
                            .map(|n| n != &theme.name)
                            .unwrap_or(true)
                        {
                            if targets::apply_theme(args, &theme).is_ok() {
                                tmpstore::append_theme_history(&theme.name);
                                s.current_theme.replace(theme.name);
                                if args.quit_on_select {
                                    break;
                                }
                            };
                        }
                    }
                    (event::KeyCode::Up, Mode::Normal) => s.scroll_list_up(1),
                    (event::KeyCode::Down, Mode::Normal) => s.scroll_list_down(1),

                    (event::KeyCode::Char('q'), Mode::Normal) => break,

                    (event::KeyCode::Char('/' | ':' | 'i'), Mode::Normal) => {
                        s.mode = Mode::Input;
                        s.input_buf.clear();
                        s.reset_pos();
                        s.filter_list_by_input();
                    }
                    (event::KeyCode::Char('?' | 'h'), Mode::Normal) => {
                        s.mode = Mode::Help;
                    }
                    (event::KeyCode::Char('j' | '+'), Mode::Normal) => s.scroll_list_down(1),
                    (event::KeyCode::Char('k' | '-'), Mode::Normal) => s.scroll_list_up(1),
                    (event::KeyCode::Char('g'), Mode::Normal) => {
                        s.reset_pos();
                    }
                    (event::KeyCode::Char('G'), Mode::Normal) => {
                        s.scroll_list_down(s.list.len());
                    }
                    (event::KeyCode::Char('d'), Mode::Normal) if ctrl => {
                        let half = (s.size.1 as usize / 2).max(1);
                        s.scroll_list_down(half);
                    }
                    (event::KeyCode::Char('u'), Mode::Normal) if ctrl => {
                        let half = (s.size.1 as usize / 2).max(1);
                        s.scroll_list_up(half);
                    }
                    (event::KeyCode::Char('d'), Mode::Normal) => {
                        s.list = s.list.into_iter().filter(|t| !t.is_light).collect();
                        if s.list.is_empty() {
                            s.list = lib::Collection::new().filter(|t| !t.is_light).collect();
                        }
                        if s.list_index >= s.list.len() {
                            s.list_index = s.list.len() - 1;
                        }
                    }
                    (event::KeyCode::Char('l'), Mode::Normal) => {
                        s.list = s.list.into_iter().filter(|t| t.is_light).collect();
                        if s.list.is_empty() {
                            s.list = lib::Collection::new().filter(|t| t.is_light).collect();
                        }
                        if s.list_index >= s.list.len() {
                            s.list_index = s.list.len() - 1;
                        }
                    }
                    (event::KeyCode::Char('s'), Mode::Normal) => {
                        fastrand::shuffle(&mut s.list);
                    }
                    (event::KeyCode::Char('r'), Mode::Normal) => {
                        s.list.reverse();
                    }
                    (event::KeyCode::Char('f'), Mode::Normal) => {
                        if s.list.is_empty() {
                            continue;
                        }

                        if let Some(theme) = s.list.get(s.list_index) {
                            s.input_buf = theme.name.splitn(2, " ").next().unwrap_or("").into();
                            s.reset_pos();
                            s.filter_list_by_input();
                        };
                    }
                    (event::KeyCode::Char(' '), Mode::Normal) => {
                        s.list = lib::Collection::new().collect();
                    }

                    // Input mode
                    (event::KeyCode::Enter | event::KeyCode::Esc, Mode::Input) => {
                        s.mode = Mode::Normal;
                    }
                    (event::KeyCode::Backspace, Mode::Input) => {
                        s.input_buf.pop();
                        s.filter_list_by_input();
                    }
                    (event::KeyCode::Char(c), Mode::Input) => {
                        s.input_buf.push(c);
                        s.filter_list_by_input();
                    }

                    // Help mode
                    (event::KeyCode::Char('q'), Mode::Help) => break,
                    (
                        event::KeyCode::Char(_)
                        | event::KeyCode::Esc
                        | event::KeyCode::Enter
                        | event::KeyCode::Backspace,
                        Mode::Help,
                    ) => {
                        s.mode = Mode::Normal;
                    }

                    (event::KeyCode::Char('c'), _) if ctrl => break,

                    _ => {}
                }

                // if let event::KeyCode::Char(c) = key.code {
                //     s.last_char.replace(c);
                // }
            }

            event::Event::Resize(cols, rows) => s.size = (cols, rows),

            _ => {}
        }

        if s.size.1 as usize <= DEFAULT_SCROLLOFF * 2 + 2 {
            s.scrolloff = DEFAULT_SCROLLOFF / 2 + 1;
        } else {
            s.scrolloff = DEFAULT_SCROLLOFF;
        }

        draw_screen(&s)?;
    }

    Ok(())
}
