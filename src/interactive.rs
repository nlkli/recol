use crate::cli::Args;
use crossterm::{cursor, event, execute, style, terminal as term};
use recol_lib as lib;
use std::{
    fmt::Debug,
    io::{self, Write},
};

struct TerminalGuard;

impl TerminalGuard {
    fn new() -> std::io::Result<Self> {
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
    Noraml,
    Input,
}

// #[derive(Debug, Clone, Default)]
// enum ThemeMode {
//     #[default]
//     None,
//     Dark,
//     Light,
// }

type Point = (u16, u16);

#[inline]
fn color_text(text: &str, c: &lib::CssColor) -> String {
    let rgb = c.color().rgb();
    format!("\x1b[38;2;{};{};{}m{}\x1b[0m", rgb.0, rgb.1, rgb.2, text)
}

#[derive(Debug, Clone, Default)]
struct State {
    size: Point,
    mode: Mode,
    // theme_mode: ThemeMode,
    input_buf: String,
    cursor: Point,
    list: Vec<lib::LazyTheme>,
    list_offset: usize,
    list_index: usize,
    dbg: String,
    last_char: char,
    scrolloff: usize,
}

impl State {
    fn scroll_list_up(&mut self, n: usize) {
        for _ in 0..n {
            if self.list_index == 0 {
                break;
            }

            let can_scroll = self.list_offset > 0;
            let limit = if can_scroll {
                self.scrolloff as usize
            } else {
                0
            };

            if (self.cursor.1 as usize) > limit {
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

            let limit = if can_scroll {
                max_cursor_row.saturating_sub(self.scrolloff as usize)
            } else {
                max_cursor_row
            };

            if (self.cursor.1 as usize) < limit {
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

    fn filter_list_by_input(&mut self) {
        self.list.clear();
        if self.input_buf.is_empty() {
            lib::Collection::new().for_each(|t| self.list.push(t));
            return;
        }
        lib::Collection::new()
            .filtered(&[lib::ThemeFilter::StartWith(&self.input_buf)])
            .for_each(|t| self.list.push(t));
        lib::Collection::new()
            .filtered(&[lib::ThemeFilter::StartWithLower(&self.input_buf)])
            .for_each(|t| {
                if !self.list.contains(&t) {
                    self.list.push(t)
                }
            });
        lib::Collection::new()
            .filtered(&[lib::ThemeFilter::Contains(&self.input_buf)])
            .for_each(|t| {
                if !self.list.contains(&t) {
                    self.list.push(t)
                }
            });
        lib::Collection::new()
            .filtered(&[lib::ThemeFilter::ContainsLower(&self.input_buf)])
            .for_each(|t| {
                if !self.list.contains(&t) {
                    self.list.push(t)
                }
            });
        if self.list.is_empty() {
            self.list
                .extend_from_slice(&lib::Collection::new().fuzzy_search_top_n(
                    &self.input_buf,
                    &[],
                    10,
                    None,
                ));
        }
    }
}

struct PartsBuf(Vec<(String, lib::CssColor)>);

impl PartsBuf {
    fn size(&self) -> usize {
        let mut res = 0;
        for i in self.0.iter() {
            res += i.0.len()
        }
        res
    }

    fn push_str(&mut self, s: &str) {
        if let Some((l, _)) = self.0.last_mut() {
            l.push_str(s);
        }
    }

    fn truncate(&mut self, mut n: usize) {
        // for (s, _) in self.0.iter_mut().rev() {
        //     let t = s.len() - 1
        // }
    }

}

fn gen_preview(t: &lib::Theme, row_lenght: usize) -> Vec<String> {
    let c = t.colors.clone().into_advanced(None);
    let preview_parts = vec![
        ("// recol", &c.comment),
        ("fn ", &c.base.magenta),
        ("main", &c.base.blue),
        ("() -> ", &c.fg[1]),
        ("Result", &c.base.yellow),
        ("<(), ", &c.fg[1]),
        ("Box", &c.base.yellow),
        ("<", &c.fg[1]),
        ("dyn ", &c.base.magenta),
        ("std", &c.base.cyan),
        ("::", &c.base.blue),
        ("error", &c.base.cyan),
        ("::", &c.base.blue),
        ("Error", &c.base.red),
        (">> {", &c.fg[1]),
        ("    let mut stdout = std::io::stdout();", &t.colors.fg),
        (&t.name, &c.fg[1]),
        ("", &t.colors.fg),
        ("hello", &c.fg[1]),
        ("hello", &c.fg[1]),
        ("hello", &c.fg[1]),
        ("hello", &c.fg[1]),
        ("hello", &c.fg[1]),
    ];
    let mut lines = Vec::new();
    for (line, color) in preview_parts.into_iter() {
        let mut s = format!(" {line}");
        let n = s.len();
        if n < row_lenght {
            s.push_str(&" ".repeat(row_lenght - n));
        } else if n >= row_lenght {
            s.truncate(row_lenght - 1);
            s.push(' ');
        }
        lines.push(color_text(&s, color));
    }
    lines
}

fn draw_screen(s: &State) -> io::Result<()> {
    const MIN_SIZE: Point = (12, 4);

    let mut stdout = io::stdout();

    execute!(
        stdout,
        term::Clear(term::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    if s.size.0 < MIN_SIZE.0 || s.size.1 < MIN_SIZE.1 {
        // TODO: print window to small
        return Ok(());
    }

    let mut row_lenght = s.size.0 as usize;
    if s.size.0 >= MIN_SIZE.0 * 2 + 2 {
        row_lenght = row_lenght / 2 - 2;
    }
    let preview_lenght = s.size.0.saturating_sub(row_lenght as u16) as usize + 1;
    if preview_lenght > 0 {
        let t = s.list[s.list_index].into_theme();
        let mut preview_lines = gen_preview(&t, preview_lenght).into_iter();
        let bg = t.colors.bg.color().rgb();
        for y in 0..s.size.1.saturating_sub(1) {
            let line = preview_lines.next().unwrap_or_default();
            execute!(
                stdout,
                cursor::MoveTo(row_lenght.saturating_sub(1) as u16, y),
                style::SetBackgroundColor(style::Color::Rgb {
                    r: bg.0,
                    g: bg.1,
                    b: bg.2
                }),
                style::Print(&line),
            )?;
            if line.is_empty() {
                execute!(stdout, style::Print(" ".repeat(preview_lenght)),)?;
            }
        }
        execute!(stdout, cursor::MoveTo(0, 0), style::ResetColor)?;
    }

    for (n, theme) in s.list.iter().skip(s.list_offset).enumerate() {
        let mut row = format!(
            " {}  {}",
            if theme.is_light { "☀" } else { "⏾" },
            theme.name
        );
        while row.len() < row_lenght {
            row.push(' ');
        }
        while row.len() > row_lenght {
            row.pop();
        }
        if n + s.list_offset == s.list_index {
            execute!(
                stdout,
                style::SetAttribute(style::Attribute::Reverse),
                style::Print(row),
                style::SetAttribute(style::Attribute::NoReverse),
                cursor::MoveDown(1),
                cursor::MoveToColumn(0)
            )?;
        } else {
            execute!(
                stdout,
                style::Print(row),
                cursor::MoveDown(1),
                cursor::MoveToColumn(0)
            )?;
        }
        if n >= s.size.1.saturating_sub(2) as usize {
            break;
        }
    }

    if !s.dbg.is_empty() {
        execute!(
            stdout,
            cursor::MoveTo(s.size.0 - s.dbg.len() as u16, 0),
            style::Print(s.dbg.clone())
        )?;
        write!(stdout, "{}", s.dbg)?;
        execute!(stdout, cursor::MoveTo(0, 0))?;
    }

    if s.mode == Mode::Input || !s.input_buf.is_empty() {
        execute!(stdout, cursor::MoveTo(0, s.size.1), cursor::Show)?;
        write!(stdout, ": {}", s.input_buf)?;
    }

    if s.mode == Mode::Noraml {
        execute!(stdout, cursor::MoveTo(s.cursor.0, s.cursor.1), cursor::Hide)?;
    }

    stdout.flush()?;

    Ok(())
}

pub fn run(_args: &Args) -> io::Result<()> {
    let _terminal_guard = TerminalGuard::new();

    let mut s = State::default();
    s.size = term::size()?;
    s.list = lib::Collection::new().collect::<Vec<_>>();
    s.scrolloff = 6;

    draw_screen(&s)?;

    loop {
        match event::read()? {
            event::Event::Key(key) => match key.code {
                event::KeyCode::Enter | event::KeyCode::Esc if s.mode == Mode::Input => {
                    s.mode = Mode::Noraml;
                }
                event::KeyCode::Backspace if s.mode == Mode::Input => {
                    s.input_buf.pop();
                    if !s.input_buf.is_empty() {
                        s.filter_list_by_input();
                    }
                }
                event::KeyCode::Char(c) if s.mode == Mode::Input => match c {
                    'c' if key.modifiers.contains(event::KeyModifiers::CONTROL) => break,
                    _ => {
                        s.input_buf.push(c);
                        s.filter_list_by_input();
                    }
                },
                event::KeyCode::Char(c) if s.mode == Mode::Noraml => {
                    match c {
                        'q' => break,
                        'c' if key.modifiers.contains(event::KeyModifiers::CONTROL) => break,
                        '/' | ':' if s.mode == Mode::Noraml => {
                            s.mode = Mode::Input;
                            s.input_buf.clear();
                            s.list_offset = 0;
                            s.list_index = 0;
                            s.cursor.1 = 0;
                            s.filter_list_by_input();
                        }
                        'j' if s.mode == Mode::Noraml => {
                            s.scroll_list_down(1);
                        }
                        'k' if s.mode == Mode::Noraml => {
                            s.scroll_list_up(1);
                        }
                        'g' if s.last_char == 'g' && s.mode == Mode::Noraml => {
                            s.list_offset = 0;
                            s.list_index = 0;
                            s.cursor.1 = 0;
                        }
                        'G' if s.mode == Mode::Noraml => {
                            s.scroll_list_down(s.list.len());
                        }
                        'd' if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                            let half = (s.size.1 as usize / 2).max(1);
                            s.scroll_list_down(half);
                        }
                        'u' if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                            let half = (s.size.1 as usize / 2).max(1);
                            s.scroll_list_up(half);
                        }
                        _ => {}
                    }
                    s.last_char = c;
                }
                _ => {}
            },

            event::Event::Resize(x, y) => s.size = (x, y),

            _ => {}
        }

        draw_screen(&s)?;
    }

    Ok(())
}
