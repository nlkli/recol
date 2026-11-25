use crate::utils;
use recol_lib as lib;
use std::io::{Read, Write};
use std::{
    collections::HashSet,
    fs, io,
    os::unix::{
        fs::{FileTypeExt, MetadataExt},
        net::UnixStream,
    },
    path::{Path, PathBuf},
    time::Duration,
};

/// `<appname>.<pid>.<n>`; `nvim*` also matches NVIM_APPNAME variants.
fn is_nvim_socket_name(name: &str) -> bool {
    let mut it = name.split('.');
    match (it.next(), it.next(), it.next(), it.next()) {
        (Some(app), Some(pid), Some(n), None) => {
            app.starts_with("nvim") && pid.parse::<u32>().is_ok() && n.parse::<u32>().is_ok()
        }
        _ => false,
    }
}

fn candidate_roots() -> Vec<PathBuf> {
    let mut roots = Vec::with_capacity(4);
    if let Some(d) = std::env::var_os("XDG_RUNTIME_DIR") {
        roots.push(PathBuf::from(d));
    }
    // SAFETY: getuid has no preconditions and cannot fail
    let uid = unsafe { libc::getuid() };
    roots.push(PathBuf::from(format!("/run/user/{uid}")));
    roots.push(std::env::temp_dir()); // $TMPDIR on macOS
    roots.push(PathBuf::from("/tmp"));
    roots.retain(|p| p.is_dir());
    roots
}

/// Root children: only `nvim*` dirs. Below that (`nvim.<user>/<random>`): any dir, max 2 levels.
fn scan(
    dir: &Path,
    level: u8,
    inside_nvim: bool,
    seen: &mut HashSet<(u64, u64)>,
    out: &mut Vec<PathBuf>,
) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let Ok(ft) = e.file_type() else { continue };
        let name = e.file_name();
        let Some(name) = name.to_str() else { continue };

        if ft.is_socket() {
            if is_nvim_socket_name(name) {
                // dedupe: /tmp vs /private/tmp symlinks, overlapping roots
                if let Ok(m) = e.metadata() {
                    if seen.insert((m.dev(), m.ino())) {
                        out.push(e.path());
                    }
                }
            }
        } else if ft.is_dir() && level < 2 && (inside_nvim || name.starts_with("nvim")) {
            scan(&e.path(), level + 1, true, seen, out);
        }
    }
}

pub fn find_nvim_sockets() -> Vec<PathBuf> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for root in candidate_roots() {
        scan(&root, 0, false, &mut seen, &mut out);
    }
    out
}

fn encode_str(buf: &mut Vec<u8>, s: &str) {
    let n = s.len();
    if n < 32 {
        buf.push(0xa0 | n as u8);
    } else if n < 256 {
        buf.extend([0xd9, n as u8]);
    } else if n < 65536 {
        buf.push(0xda);
        buf.extend((n as u16).to_be_bytes());
    } else {
        buf.push(0xdb);
        buf.extend((n as u32).to_be_bytes());
    }
    buf.extend(s.as_bytes());
}

fn send(path: &Path, cmd: &str) -> Result<(), String> {
    let mut s = UnixStream::connect(path).map_err(|e| e.to_string())?;
    let t = Some(Duration::from_millis(300));
    s.set_read_timeout(t).map_err(|e| e.to_string())?;
    s.set_write_timeout(t).map_err(|e| e.to_string())?;

    // [0 (request), msgid 1, "nvim_command", [cmd]]
    let mut req = vec![0x94, 0x00, 0x01];
    encode_str(&mut req, "nvim_command");
    req.push(0x91);
    encode_str(&mut req, cmd);
    s.write_all(&req).map_err(|e| e.to_string())?;

    // response: [1, 1, error, result]; error == nil (0xc0) means success
    let mut buf = [0u8; 1024];
    let n = s.read(&mut buf).map_err(|e| e.to_string())?;
    match buf[..n] {
        [0x94, 0x01, 0x01, 0xc0, ..] => Ok(()),
        [0x94, 0x01, 0x01, ..] => Err(String::from_utf8_lossy(&buf[4..n]).into_owned()),
        _ => Err("unexpected response".into()),
    }
}

pub fn broadcast(sockets: Vec<PathBuf>, cmd: &str) {
    std::thread::scope(|s| {
        for path in &sockets {
            s.spawn(move || {
                let _ = send(path, cmd);
                // if let Err(e) = send(path, cmd) {
                //     eprintln!("{}: {e}", path.display());
                // }
            });
        }
    });
}

pub fn apply_theme_to(path: impl AsRef<Path>, theme: &lib::ThemeEx) -> io::Result<()> {
    let c = theme
        .advanced
        .clone()
        .unwrap_or(theme.colors.clone().into_advanced(None));

    let content = format!(
        r###"
-- {theme_name}
local function applyRecolTheme()
    vim.cmd("highlight clear")
    if vim.fn.has("syntax_on") then vim.cmd("syntax reset") end
    local P = {{
        black   = {{ "{black_base}", "{black_bright}" }},
        red     = {{ "{red_base}", "{red_bright}" }},
        green   = {{ "{green_base}", "{green_bright}" }},
        yellow  = {{ "{yellow_base}", "{yellow_bright}" }},
        blue    = {{ "{blue_base}", "{blue_bright}" }},
        magenta = {{ "{magenta_base}", "{magenta_bright}" }},
        cyan    = {{ "{cyan_base}", "{cyan_bright}" }},
        white   = {{ "{white_base}", "{white_bright}" }},
        orange  = {{ "{orange_base}", "{orange_bright}" }},
        pink    = {{ "{pink_base}", "{pink_bright}" }},
        bg = {{ "{bg0}", "{bg1}", "{bg2}", "{bg3}", "{bg4}" }},
        fg = {{ "{fg0}", "{fg1}", "{fg2}", "{fg3}" }},
        sel = {{ "{sel0}", "{sel1}" }},
        cur = {{ "{cur_bg}", "{cur_fg}" }},
        comment = "{comment}",
        diff = {{
            add = "{diff_add}",
            del = "{diff_delete}",
            change = "{diff_change}",
            txt = "{diff_text}",
        }}
    }}
    for group, opts in pairs({{
        Normal             = {{ bg = P.bg[2], fg = P.fg[2] }},
        NormalNC           = "Normal",
        NormalFloat        = {{ bg = P.bg[1], fg = P.fg[2] }},
        FloatBorder        = {{ bg = P.bg[1], fg = P.fg[4] }},
        FloatTitle         = {{ bg = P.bg[1], fg = P.blue[2], bold = true }},
        FloatFooter        = {{ bg = P.bg[1], fg = P.fg[4] }},
        FloatShadow        = {{ bg = P.black[1], blend = 80 }},
        FloatShadowThrough = {{ bg = P.black[1], blend = 100 }},
        Directory          = {{ fg = P.blue[1] }},
        EndOfBuffer        = {{ fg = P.bg[2] }},
        NonText            = {{ fg = P.comment }},

        Cursor        = {{ bg = P.cur[1], fg = P.cur[2] }},
        lCursor       = {{ bg = P.cyan[1], fg = P.cur[2] }},
        CursorIM      = {{ bg = P.magenta[1], fg = P.cur[2] }},
        TermCursor    = {{ bg = P.cur[1], fg = P.cur[2] }},
        CursorLine    = {{ bg = P.bg[3] }},
        CursorColumn  = "CursorColumn",
        Visual        = {{ bg = P.sel[1] }},
        VisualNOS     = {{ bg = P.sel[2] }},
        MatchParen    = {{ fg = P.orange[1], bg = P.bg[4], bold = true }},
        MCursor       = {{ bg = P.pink[1], fg = P.cur[2] }},
        MCursorVisual = {{ bg = P.sel[2] }},

        LineNr         = {{ fg = P.fg[4] }},
        LineNrAbove    = "LineNr",
        LineNrBelow    = "LineNr",
        CursorLineNr   = {{ fg = P.cur[1], bg = P.bg[3], bold = true }},
        SignColumn     = {{ bg = P.bg[2], fg = P.fg[4] }},
        FoldColumn     = {{ bg = P.bg[2], fg = P.fg[4] }},
        CursorLineSign = {{ bg = P.bg[3] }},
        CursorLineFold = {{ bg = P.bg[3], fg = P.fg[3] }},

        WinSeparator = {{ fg = P.bg[5] }},
        WinBar       = {{ fg = P.fg[2], bg = P.bg[2] }},
        WinBarNC     = {{ fg = P.fg[4], bg = P.bg[1] }},
        TabLine      = {{ fg = P.fg[3], bg = P.bg[3] }},
        TabLineSel   = {{ fg = P.cur[1], bg = P.bg[2], bold = true }},
        TabLineFill  = {{ bg = P.bg[1] }},

        Search     = {{ bg = P.diff.txt, fg = P.fg[1] }},
        CurSearch  = {{ bg = P.yellow[1], fg = P.cur[2], bold = true }},
        IncSearch  = {{ bg = P.orange[1], fg = P.cur[2], bold = true }},
        Substitute = {{ bg = P.red[1], fg = P.fg[1], bold = true }},

        Pmenu                = {{ bg = P.bg[3], fg = P.fg[2] }},
        PmenuSel             = {{ bg = P.bg[5], fg = P.fg[1], bold = true }},
        PmenuMatch           = {{ fg = P.blue[2], bold = true }},
        PmenuMatchSel        = {{ fg = P.yellow[2], bold = true }},
        PmenuKind            = {{ fg = P.magenta[1] }},
        PmenuKindSel         = {{ fg = P.magenta[2], bold = true }},
        PmenuExtra           = {{ fg = P.fg[4] }},
        PmenuExtraSel        = {{ fg = P.fg[3] }},
        PmenuBorder          = {{ bg = P.bg[3], fg = P.fg[4] }},
        PmenuSbar            = {{ bg = P.bg[4] }},
        PmenuThumb           = {{ bg = P.fg[4] }},
        PmenuShadow          = {{ bg = P.black[1], blend = 80 }},
        PmenuShadowThrough   = {{ bg = P.black[1], blend = 100 }},
        ComplMatchIns        = {{ fg = P.fg[3] }},
        PreInsert            = {{ fg = P.fg[3], bg = P.bg[4] }},
        ComplHint            = {{ fg = P.comment, italic = true }},
        ComplHintMore        = {{ fg = P.fg[4], italic = true }},
        WildMenu             = {{ bg = P.bg[5], fg = P.fg[1], bold = true }},
        SnippetTabstop       = {{ bg = P.bg[4] }},
        SnippetTabstopActive = {{ bg = P.diff.change, fg = P.fg[1] }},

        SpellBad   = {{ undercurl = true, sp = P.red[1] }},
        SpellCap   = {{ undercurl = true, sp = P.yellow[1] }},
        SpellLocal = {{ underdotted = true, sp = P.cyan[1] }},
        SpellRare  = {{ underdotted = true, sp = P.magenta[1] }},

        MsgArea      = {{ bg = P.bg[2], fg = P.fg[2] }},
        MsgSeparator = {{ bg = P.bg[3], fg = P.bg[5] }},
        OkMsg        = {{ fg = P.green[1] }},
        Question     = {{ fg = P.blue[1], bold = true }},
        MoreMsg      = {{ fg = P.cyan[1], bold = true }},
        ModeMsg      = {{ fg = P.yellow[1], bold = true }},
        WarningMsg   = {{ fg = P.yellow[1] }},
        ErrorMsg     = {{ fg = P.red[1], bold = true }},
        StdoutMsg    = {{ fg = P.fg[2] }},
        StderrMsg    = {{ fg = P.red[2] }},

        DiffAdd     = {{ bg = P.diff.add }},
        DiffChange  = {{ bg = P.diff.change }},
        DiffDelete  = {{ bg = P.diff.del }},
        DiffText    = {{ bg = P.diff.txt, bold = true }},
        DiffTextAdd = {{ bg = P.diff.add, bold = true }},

        Folded       = {{ bg = P.bg[3], fg = P.fg[3], italic = true }},
        QuickFixLine = {{ bg = P.sel[2], fg = P.yellow[2], bold = true }},

        StatusLine       = {{ bg = P.bg[3], fg = P.fg[2] }},
        StatusLineNC     = {{ bg = P.bg[1], fg = P.fg[4] }},
        StatusLineTerm   = {{ bg = P.bg[3], fg = P.green[1] }},
        StatusLineTermNC = {{ bg = P.bg[1], fg = P.fg[4] }},
        StatusLineRemote = {{ bg = P.bg[3], fg = P.orange[1], bold = true }},
        Title            = {{ fg = P.blue[1], bold = true }},

        Menu      = {{ bg = P.bg[3], fg = P.fg[2] }},
        Scrollbar = {{ bg = P.bg[3], fg = P.fg[4] }},
        Tooltip   = {{ bg = P.bg[3], fg = P.fg[2] }},

        ColorColumn = {{ bg = P.bg[3] }},
        SpecialKey  = {{ fg = P.cyan[1] }},
        Whitespace  = {{ fg = P.bg[5] }},

        Comment        = {{ fg = P.comment }},
        SpecialComment = {{ fg = P.cyan[2] }},
        Constant       = {{ fg = P.orange[1] }},
        String         = {{ fg = P.green[1] }},
        Character      = {{ fg = P.green[2] }},
        Number         = "Constant",
        Float          = "Constant",
        Boolean        = "Constant",
        Regexp         = {{ fg = P.cyan[2] }},
        Identifier     = {{ fg = P.fg[2] }},
        Function       = {{ fg = P.blue[1] }},
        Statement      = {{ fg = P.magenta[1] }},
        Conditional    = {{ fg = P.magenta[1] }},
        Repeat         = {{ fg = P.magenta[1] }},
        Label          = {{ fg = P.magenta[2] }},
        Keyword        = {{ fg = P.magenta[1] }},
        Exception      = {{ fg = P.pink[1] }},
        Operator       = {{ fg = P.fg[3] }},
        PreProc        = {{ fg = P.pink[1] }},
        Include        = {{ fg = P.red[1] }},
        Define         = {{ fg = P.pink[1] }},
        Macro          = {{ fg = P.pink[2] }},
        PreCondit      = {{ fg = P.pink[1] }},
        Type           = {{ fg = P.yellow[1] }},
        StorageClass   = {{ fg = P.magenta[2] }},
        Structure      = {{ fg = P.yellow[1] }},
        Typedef        = {{ fg = P.yellow[1] }},
        Special        = {{ fg = P.cyan[1] }},
        SpecialChar    = {{ fg = P.yellow[2], bold = true }},
        Tag            = {{ fg = P.blue[2] }},
        Delimiter      = {{ fg = P.fg[3] }},
        Debug          = {{ fg = P.orange[1] }},
        Underlined     = {{ fg = P.blue[1], underline = true }},
        Bold           = {{ bold = true }},
        Italic         = {{ italic = true }},
        BoldItalic     = {{ bold = true, italic = true }},
        Dimmed         = {{ fg = P.fg[4] }},
        Ignore         = {{ fg = P.bg[2] }},
        Error          = {{ fg = P.red[1] }},
        Todo           = {{ bg = P.yellow[1], fg = P.black[1], bold = true }},
        Added          = {{ fg = P.green[1] }},
        Changed        = {{ fg = P.blue[1] }},
        Removed        = {{ fg = P.red[1] }},

        healthError   = {{ fg = P.red[1] }},
        healthSuccess = {{ fg = P.green[1] }},
        healthWarning = {{ fg = P.yellow[1] }},

        DiagnosticError = {{ fg = P.red[1] }},
        DiagnosticWarn  = {{ fg = P.yellow[1] }},
        DiagnosticInfo  = {{ fg = P.blue[1] }},
        DiagnosticHint  = {{ fg = P.cyan[1] }},
        DiagnosticOk    = {{ fg = P.green[1] }},

        ["@variable"]                   = {{ fg = P.fg[2] }},
        ["@variable.builtin"]           = {{ fg = P.red[1] }},
        ["@variable.parameter"]         = {{ fg = P.fg[2] }},
        ["@variable.parameter.builtin"] = {{ fg = P.red[1] }},
        ["@variable.member"]            = {{ fg = P.blue[2] }},
        ["@constant"]                   = "Constant",
        ["@constant.builtin"]           = "Constant",
        ["@constant.macro"]             = {{ fg = P.pink[2] }},
        ["@module"]                     = {{ fg = P.cyan[2] }},
        ["@module.builtin"]             = {{ fg = P.cyan[1] }},
        ["@label"]                      = {{ fg = P.magenta[2] }},
        ["@string"]                     = {{ fg = P.green[1] }},
        ["@string.documentation"]       = {{ fg = P.green[2] }},
        ["@string.regexp"]              = "Regexp",
        ["@string.escape"]              = "SpecialChar",
        ["@string.special"]             = "Special",
        ["@string.special.symbol"]      = {{ fg = P.magenta[1] }},
        ["@string.special.url"]         = {{ fg = P.blue[1], underline = true }},
        ["@string.special.path"]        = {{ fg = P.blue[2] }},
        ["@character.special"]          = {{ fg = P.cyan[2] }},
        ["@character"]                  = "Character",
        ["@boolean"]                    = "Boolean",
        ["@number"]                     = "Number",
        ["@number.float"]               = "Float",
        ["@type"]                       = "Type",
        ["@type.builtin"]               = {{ fg = P.cyan[2] }},
        ["@type.definition"]            = "Typedef",
        ["@attribute"]                  = {{ fg = P.pink[2] }},
        ["@attribute.builtin"]          = {{ fg = P.pink[1] }},
        ["@property"]                   = {{ fg = P.blue[2] }},
        ["@function"]                   = "Function",
        ["@function.builtin"]           = {{ fg = P.red[1] }},
        ["@function.call"]              = "Function",
        ["@function.macro"]             = {{ fg = P.red[1] }},
        ["@function.method"]            = "Function",
        ["@function.method.call"]       = "Function",
        ["@constructor"]                = {{ fg = P.cyan[1] }},
        ["@operator"]                   = "Operator",

        ["@keyword"]                     = "Keyword",
        ["@keyword.coroutine"]           = "Keyword",
        ["@keyword.function"]            = "Keyword",
        ["@keyword.operator"]            = "Operator",
        ["@keyword.import"]              = "Include",
        ["@keyword.type"]                = "Keyword",
        ["@keyword.modifier"]            = "Keyword",
        ["@keyword.repeat"]              = "Repeat",
        ["@keyword.return"]              = {{ fg = P.red[1] }},
        ["@keyword.debug"]               = "Debug",
        ["@keyword.exception"]           = "Exception",
        ["@keyword.conditional"]         = "Conditional",
        ["@keyword.conditional.ternary"] = {{ fg = P.cyan[1] }},
        ["@keyword.directive"]           = "PreProc",
        ["@keyword.directive.define"]    = "Define",

        ["@punctuation.delimiter"] = "Delimiter",
        ["@punctuation.bracket"]   = {{ fg = P.fg[3] }},
        ["@punctuation.special"]   = {{ fg = P.cyan[2] }},

        ["@comment"]               = "Comment",
        ["@comment.documentation"] = {{ fg = P.comment }},
        ["@comment.error"]         = {{ bg = P.red[1], fg = P.black[1], bold = true }},
        ["@comment.warning"]       = {{ bg = P.yellow[1], fg = P.black[1], bold = true }},
        ["@comment.todo"]          = "Todo",
        ["@comment.note"]          = {{ bg = P.blue[1], fg = P.black[1], bold = true }},

        ["@markup.strong"]         = {{ fg = P.red[1], bold = true }},
        ["@markup.italic"]         = {{ italic = true }},
        ["@markup.strikethrough"]  = {{ strikethrough = true }},
        ["@markup.underline"]      = {{ underline = true }},
        ["@markup.heading"]        = {{ fg = P.blue[1], bold = true }},
        ["@markup.quote"]          = {{ fg = P.fg[3] }},
        ["@markup.math"]           = {{ fg = P.cyan[2] }},
        ["@markup.link"]           = {{ fg = P.magenta[1] }},
        ["@markup.link.label"]     = {{ fg = P.blue[2] }},
        ["@markup.link.url"]       = {{ fg = P.magenta[2], underline = true }},
        ["@markup.raw"]            = {{ fg = P.green[2], bg = P.bg[3] }},
        ["@markup.raw.block"]      = {{ fg = P.fg[2] }},
        ["@markup.list"]           = {{ fg = P.cyan[2] }},
        ["@markup.list.checked"]   = {{ fg = P.green[1] }},
        ["@markup.list.unchecked"] = {{ fg = P.fg[4] }},

        ["@diff.plus"]     = {{ fg = P.green[1] }},
        ["@diff.minus"]    = {{ fg = P.red[1] }},
        ["@diff.delta"]    = {{ fg = P.blue[1] }},
        ["@tag"]           = {{ fg = P.red[1] }},
        ["@tag.builtin"]   = {{ fg = P.red[1] }},
        ["@tag.attribute"] = {{ fg = P.orange[2] }},
        ["@tag.delimiter"] = {{ fg = P.fg[3] }},

        ["@constructor.lua"]      = {{ fg = P.fg[3] }},
        ["@field.rust"]           = {{ fg = P.fg[3] }},
        ["@label.json"]           = "@property",
        ["@variable.member.yaml"] = "@property",

        ["@lsp.type.comment"]      = {{}},
        ["@lsp.type.keyword"]      = {{}},
        ["@lsp.type.modifier"]     = {{}},
        ["@lsp.type.number"]       = {{}},
        ["@lsp.type.operator"]     = {{}},
        ["@lsp.type.regexp"]       = {{}},
        ["@lsp.type.string"]       = {{}},
        ["@lsp.type.variable"]     = {{}},
        ["@lsp.type.namespace.go"] = {{}},

        ["@lsp.type.interface"]                  = {{ fg = P.red[1] }},
        ["@lsp.type.boolean"]                    = "@boolean",
        ["@lsp.type.builtinConstant"]            = "@constant.builtin",
        ["@lsp.type.generic"]                    = "@type",
        ["@lsp.type.builtinType"]                = "@type.builtin",
        ["@lsp.type.enum"]                       = "@type",
        ["@lsp.type.enumMember"]                 = "@constant",
        ["@lsp.type.escapeSequence"]             = "@string.escape",
        ["@lsp.type.formatSpecifier"]            = "@punctuation.special",
        ["@lsp.type.namespace"]                  = "@module",
        ["@lsp.type.parameter"]                  = "@parameter",
        ["@lsp.type.property"]                   = "@property",
        ["@lsp.type.selfKeyword"]                = "@variable.builtin",
        ["@lsp.type.typeAlias"]                  = "@type.definition",
        ["@lsp.type.unresolvedReference"]        = "@error",
        ["@lsp.typemod.function.defaultLibrary"] = "@function.builtin",
        ["@lsp.typemod.function.builtin"]        = "@function.builtin",

        ["@lsp.type.selfParameter.python"] = "@variable.builtin",
        ["@lsp.type.clsParameter.python"]  = "@variable.builtin",

        ["@lsp.type.table.toml"]          = "@property",
        ["@lsp.type.key.toml"]            = "@property",
        ["@lsp.type.boolean.toml"]        = "Boolean",
        ["@lsp.type.offsetDateTime.toml"] = "Special",
        ["@lsp.type.localDateTime.toml"]  = "Special",
        ["@lsp.type.localDate.toml"]      = "Special",
        ["@lsp.type.localTime.toml"]      = "Special",
    }}) do
        if type(opts) == "string" then
            opts = {{ link = opts }}
        end
        vim.api.nvim_set_hl(0, group, opts)
    end
end
applyRecolTheme()"###,
        theme_name = theme.name,
        black_base = c.base.black,
        black_bright = c.bright.black,
        red_base = c.base.red,
        red_bright = c.bright.red,
        green_base = c.base.green,
        green_bright = c.bright.green,
        yellow_base = c.base.yellow,
        yellow_bright = c.bright.yellow,
        blue_base = c.base.blue,
        blue_bright = c.bright.blue,
        magenta_base = c.base.magenta,
        magenta_bright = c.bright.magenta,
        cyan_base = c.base.cyan,
        cyan_bright = c.bright.cyan,
        white_base = c.base.white,
        white_bright = c.bright.white,
        orange_base = c.base.orange,
        orange_bright = c.bright.orange,
        pink_base = c.base.pink,
        pink_bright = c.bright.pink,
        comment = c.comment,
        bg0 = c.bg[0],
        bg1 = c.bg[1],
        bg2 = c.bg[2],
        bg3 = c.bg[3],
        bg4 = c.bg[4],
        fg0 = c.fg[0],
        fg1 = c.fg[1],
        fg2 = c.fg[2],
        fg3 = c.fg[3],
        sel0 = c.alt_selection[0],
        sel1 = c.alt_selection[1],
        cur_bg = c.cursor.bg,
        cur_fg = c.cursor.fg,
        diff_add = c.diff.add,
        diff_delete = c.diff.delete,
        diff_change = c.diff.change,
        diff_text = c.diff.text,
    );

    utils::inject_content_between_markers(
        &path,
        content.as_bytes(),
        ("-- recol:start", "-- recol:end"),
    )?;

    // reload config
    broadcast(
        find_nvim_sockets(),
        &format!("source {}", path.as_ref().to_string_lossy()),
    );

    Ok(())
}
