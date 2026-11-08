use crate::utils;
use recol_lib as lib;
use std::{io, path::Path};

pub fn apply_theme_to(path: impl AsRef<Path>, theme: &lib::ThemeEx) -> io::Result<()> {
    let c = theme
        .advanced
        .clone()
        .unwrap_or(theme.colors.clone().into_advanced(None));

    let content = format!(
        r###"
if !exists("s:recol_cache")
    let s:recol_cache = {{}}
endif

function! s:RecolCube(v) abort
    return a:v < 48 ? 0 : (a:v < 115 ? 1 : (a:v - 35) / 40)
endfunction

function! s:RecolCterm(hex) abort
    if a:hex ==# ""
        return "NONE"
    endif
    if has_key(s:recol_cache, a:hex)
        return s:recol_cache[a:hex]
    endif
    let l:r = str2nr(a:hex[1:2], 16)
    let l:g = str2nr(a:hex[3:4], 16)
    let l:b = str2nr(a:hex[5:6], 16)
    let l:lv = [0, 95, 135, 175, 215, 255]
    let l:ir = s:RecolCube(l:r)
    let l:ig = s:RecolCube(l:g)
    let l:ib = s:RecolCube(l:b)
    let l:dr = l:r - l:lv[l:ir]
    let l:dg = l:g - l:lv[l:ig]
    let l:db = l:b - l:lv[l:ib]
    let l:dcube = l:dr * l:dr + l:dg * l:dg + l:db * l:db
    let l:avg = (l:r + l:g + l:b) / 3
    let l:gi = l:avg < 3 ? 0 : (l:avg > 238 ? 23 : (l:avg - 3) / 10)
    let l:gv = 8 + l:gi * 10
    let l:dgray = (l:r - l:gv) * (l:r - l:gv) + (l:g - l:gv) * (l:g - l:gv) + (l:b - l:gv) * (l:b - l:gv)
    let l:idx = l:dgray < l:dcube ? 232 + l:gi : 16 + 36 * l:ir + 6 * l:ig + l:ib
    let s:recol_cache[a:hex] = string(l:idx)
    return s:recol_cache[a:hex]
endfunction

function! s:RecolHi(group, fg, bg, ...) abort
    let l:attr = (a:0 >= 1 && a:1 !=# "") ? a:1 : "NONE"
    let l:sp = a:0 >= 2 ? a:2 : ""
    let l:cattr = substitute(l:attr, "under\(curl\|dotted\|double\|dashed\)", "underline", "g")
    let l:cmd = "highlight " . a:group
    let l:cmd .= " guifg=" . (a:fg ==# "" ? "NONE" : a:fg)
    let l:cmd .= " guibg=" . (a:bg ==# "" ? "NONE" : a:bg)
    let l:cmd .= " ctermfg=" . s:RecolCterm(a:fg)
    let l:cmd .= " ctermbg=" . s:RecolCterm(a:bg)
    let l:cmd .= " gui=" . l:attr . " cterm=" . l:cattr
    if l:sp !=# ""
        let l:cmd .= " guisp=" . l:sp
    endif
    execute l:cmd
endfunction

function! s:RecolApply() abort
    if exists("g:colors_name")
        unlet g:colors_name
    endif
    if &background !=# "dark"
        set background=dark
    endif
    highlight clear
    if exists("syntax_on")
        syntax reset
    endif

    if !has("gui_running") && has("termguicolors")
                \ && ($COLORTERM ==# "truecolor" || $COLORTERM ==# "24bit")
        if empty(&t_8f)
            let &t_8f = "\<Esc>[38;2;%lu;%lu;%lum"
            let &t_8b = "\<Esc>[48;2;%lu;%lu;%lum"
        endif
        set termguicolors
    endif

    let P = {{}}
    let P.black   = ["", "{black_base}", "{black_bright}"]
    let P.red     = ["", "{red_base}", "{red_bright}"]
    let P.green   = ["", "{green_base}", "{green_bright}"]
    let P.yellow  = ["", "{yellow_base}", "{yellow_bright}"]
    let P.blue    = ["", "{blue_base}", "{blue_bright}"]
    let P.magenta = ["", "{magenta_base}", "{magenta_bright}"]
    let P.cyan    = ["", "{cyan_base}", "{cyan_bright}"]
    let P.white   = ["", "{white_base}", "{white_bright}"]
    let P.orange  = ["", "{orange_base}", "{orange_bright}"]
    let P.pink    = ["", "{pink_base}", "{pink_bright}"]
    let P.bg      = ["", "{bg0}", "{bg1}", "{bg2}", "{bg3}", "{bg4}"]
    let P.fg      = ["", "{fg0}", "{fg1}", "{fg2}", "{fg3}"]
    let P.sel     = ["", "{sel0}", "{sel1}"]
    let P.cur     = ["", "{cur_bg}", "{cur_fg}"]
    let P.comment = "{comment}"
    let P.diff = {{}}
    let P.diff["add"]    = "{diff_add}"
    let P.diff["del"]    = "{diff_delete}"
    let P.diff["change"] = "{diff_change}"
    let P.diff["txt"]    = "{diff_text}"

    call s:RecolHi("Normal",       P.fg[2], P.bg[2])
    call s:RecolHi("Terminal",     P.fg[2], P.bg[2])
    call s:RecolHi("Directory",    P.blue[1], "")
    call s:RecolHi("EndOfBuffer",  P.bg[2], "")
    call s:RecolHi("NonText",      P.comment, "")
    call s:RecolHi("Conceal",      P.fg[4], "")

    call s:RecolHi("Cursor",       P.cur[2], P.cur[1])
    call s:RecolHi("lCursor",      P.cur[2], P.cyan[1])
    call s:RecolHi("CursorIM",     P.cur[2], P.magenta[1])
    call s:RecolHi("CursorLine",   "", P.bg[3])
    call s:RecolHi("CursorColumn", "", P.bg[3])
    call s:RecolHi("Visual",       "", P.sel[1])
    call s:RecolHi("VisualNOS",    "", P.sel[2])
    call s:RecolHi("MatchParen",   P.orange[1], P.bg[4], "bold")

    call s:RecolHi("LineNr",         P.fg[4], "")
    highlight! link LineNrAbove LineNr
    highlight! link LineNrBelow LineNr
    call s:RecolHi("CursorLineNr",   P.cur[1], P.bg[3], "bold")
    call s:RecolHi("SignColumn",     P.fg[4], P.bg[2])
    call s:RecolHi("FoldColumn",     P.fg[4], P.bg[2])
    call s:RecolHi("CursorLineSign", "", P.bg[3])
    call s:RecolHi("CursorLineFold", P.fg[3], P.bg[3])

    call s:RecolHi("VertSplit",    P.bg[5], P.bg[2])
    call s:RecolHi("WinBar",       P.fg[2], P.bg[2])
    call s:RecolHi("WinBarNC",     P.fg[4], P.bg[1])
    call s:RecolHi("TabLine",      P.fg[3], P.bg[3])
    call s:RecolHi("TabLineSel",   P.cur[1], P.bg[2], "bold")
    call s:RecolHi("TabLineFill",  "", P.bg[1])

    call s:RecolHi("Search",       P.fg[1], P.diff.txt)
    call s:RecolHi("CurSearch",    P.cur[2], P.yellow[1], "bold")
    call s:RecolHi("IncSearch",    P.cur[2], P.orange[1], "bold")

    call s:RecolHi("Pmenu",         P.fg[2], P.bg[3])
    call s:RecolHi("PmenuSel",      P.fg[1], P.bg[5], "bold")
    call s:RecolHi("PmenuMatch",    P.blue[2], P.bg[3], "bold")
    call s:RecolHi("PmenuMatchSel", P.yellow[2], P.bg[5], "bold")
    call s:RecolHi("PmenuKind",     P.magenta[1], P.bg[3])
    call s:RecolHi("PmenuKindSel",  P.magenta[2], P.bg[5], "bold")
    call s:RecolHi("PmenuExtra",    P.fg[4], P.bg[3])
    call s:RecolHi("PmenuExtraSel", P.fg[3], P.bg[5])
    call s:RecolHi("PmenuSbar",     "", P.bg[4])
    call s:RecolHi("PmenuThumb",    "", P.fg[4])
    call s:RecolHi("ComplMatchIns", P.fg[3], "")
    call s:RecolHi("WildMenu",      P.fg[1], P.bg[5], "bold")

    call s:RecolHi("SpellBad",   "", "", "undercurl", P.red[1])
    call s:RecolHi("SpellCap",   "", "", "undercurl", P.yellow[1])
    call s:RecolHi("SpellLocal", "", "", "undercurl", P.cyan[1])
    call s:RecolHi("SpellRare",  "", "", "undercurl", P.magenta[1])

    call s:RecolHi("Question",   P.blue[1], "", "bold")
    call s:RecolHi("MoreMsg",    P.cyan[1], "", "bold")
    call s:RecolHi("ModeMsg",    P.yellow[1], "", "bold")
    call s:RecolHi("WarningMsg", P.yellow[1], "")
    call s:RecolHi("ErrorMsg",   P.red[1], "", "bold")

    call s:RecolHi("DiffAdd",     "", P.diff.add)
    call s:RecolHi("DiffChange",  "", P.diff.change)
    call s:RecolHi("DiffDelete",  P.diff.del, P.diff.del)
    call s:RecolHi("DiffText",    "", P.diff.txt, "bold")
    call s:RecolHi("DiffTextAdd", "", P.diff.add, "bold")

    call s:RecolHi("Folded",       P.fg[3], P.bg[3], "italic")
    call s:RecolHi("QuickFixLine", P.yellow[2], P.sel[2], "bold")

    call s:RecolHi("StatusLine",       P.fg[2], P.bg[3])
    call s:RecolHi("StatusLineNC",     P.fg[4], P.bg[1])
    call s:RecolHi("StatusLineTerm",   P.green[1], P.bg[3])
    call s:RecolHi("StatusLineTermNC", P.fg[4], P.bg[1])
    call s:RecolHi("Title",            P.blue[1], "", "bold")

    call s:RecolHi("Menu",        P.fg[2], P.bg[3])
    call s:RecolHi("Scrollbar",   P.fg[4], P.bg[3])
    call s:RecolHi("Tooltip",     P.fg[2], P.bg[3])
    call s:RecolHi("ToolbarLine", P.fg[2], P.bg[3])
    call s:RecolHi("ToolbarButton", P.fg[1], P.bg[5], "bold")

    call s:RecolHi("ColorColumn", "", P.bg[3])
    call s:RecolHi("SpecialKey",  P.cyan[1], "")
    call s:RecolHi("Whitespace",  P.bg[5], "")

    call s:RecolHi("Comment",        P.comment, "")
    call s:RecolHi("SpecialComment", P.cyan[2], "")
    call s:RecolHi("Constant",       P.orange[1], "")
    call s:RecolHi("String",         P.green[1], "")
    call s:RecolHi("Character",      P.green[2], "")
    highlight! link Number  Constant
    highlight! link Float   Constant
    highlight! link Boolean Constant
    call s:RecolHi("Identifier",     P.fg[2], "")
    call s:RecolHi("Function",       P.blue[1], "")
    call s:RecolHi("Statement",      P.magenta[1], "")
    call s:RecolHi("Conditional",    P.magenta[1], "")
    call s:RecolHi("Repeat",         P.magenta[1], "")
    call s:RecolHi("Label",          P.magenta[2], "")
    call s:RecolHi("Keyword",        P.magenta[1], "")
    call s:RecolHi("Exception",      P.pink[1], "")
    call s:RecolHi("Operator",       P.fg[3], "")
    call s:RecolHi("PreProc",        P.pink[1], "")
    call s:RecolHi("Include",        P.red[1], "")
    call s:RecolHi("Define",         P.pink[1], "")
    call s:RecolHi("Macro",          P.pink[2], "")
    call s:RecolHi("PreCondit",      P.pink[1], "")
    call s:RecolHi("Type",           P.yellow[1], "")
    call s:RecolHi("StorageClass",   P.magenta[2], "")
    call s:RecolHi("Structure",      P.yellow[1], "")
    call s:RecolHi("Typedef",        P.yellow[1], "")
    call s:RecolHi("Special",        P.cyan[1], "")
    call s:RecolHi("SpecialChar",    P.yellow[2], "", "bold")
    call s:RecolHi("Tag",            P.blue[2], "")
    call s:RecolHi("Delimiter",      P.fg[3], "")
    call s:RecolHi("Debug",          P.orange[1], "")
    call s:RecolHi("Underlined",     P.blue[1], "", "underline")
    call s:RecolHi("Ignore",         P.bg[2], "")
    call s:RecolHi("Error",          P.red[1], "")
    call s:RecolHi("Todo",           P.black[1], P.yellow[1], "bold")
    call s:RecolHi("Added",          P.green[1], "")
    call s:RecolHi("Changed",        P.blue[1], "")
    call s:RecolHi("Removed",        P.red[1], "")

    highlight! link diffAdded   Added
    highlight! link diffRemoved Removed
    highlight! link diffChanged Changed
    highlight! link diffFile    Title
    highlight! link diffNewFile Title
    highlight! link diffOldFile Title
    highlight! link diffLine    Special

    call s:RecolHi("htmlTag",        P.fg[3], "")
    call s:RecolHi("htmlEndTag",     P.fg[3], "")
    call s:RecolHi("htmlTagName",    P.red[1], "")
    call s:RecolHi("htmlSpecialTagName", P.red[1], "")
    call s:RecolHi("htmlArg",        P.orange[2], "")
    call s:RecolHi("xmlTag",         P.fg[3], "")
    call s:RecolHi("xmlEndTag",      P.fg[3], "")
    call s:RecolHi("xmlTagName",     P.red[1], "")
    call s:RecolHi("xmlAttrib",      P.orange[2], "")

    for l:n in range(1, 6)
        call s:RecolHi("markdownH" . l:n, P.blue[1], "", "bold")
    endfor
    call s:RecolHi("markdownHeadingDelimiter", P.blue[1], "", "bold")
    call s:RecolHi("markdownBold",        P.red[1], "", "bold")
    call s:RecolHi("markdownItalic",      "", "", "italic")
    call s:RecolHi("markdownBoldItalic",  P.red[1], "", "bold,italic")
    call s:RecolHi("markdownCode",        P.green[2], P.bg[3])
    call s:RecolHi("markdownCodeBlock",   P.fg[2], "")
    call s:RecolHi("markdownCodeDelimiter", P.green[2], "")
    call s:RecolHi("markdownLinkText",    P.magenta[1], "")
    call s:RecolHi("markdownUrl",         P.magenta[2], "", "underline")
    call s:RecolHi("markdownLinkDelimiter",     P.fg[3], "")
    call s:RecolHi("markdownLinkTextDelimiter", P.fg[3], "")
    call s:RecolHi("markdownListMarker",  P.cyan[2], "")
    call s:RecolHi("markdownBlockquote",  P.fg[3], "")
    call s:RecolHi("markdownIdDeclaration", P.blue[2], "")

    call s:RecolHi("jsonKeyword",          P.blue[2], "")
    call s:RecolHi("yamlBlockMappingKey",  P.blue[2], "")
    call s:RecolHi("yamlFlowMappingKey",   P.blue[2], "")
    call s:RecolHi("tomlKey",              P.blue[2], "")
    call s:RecolHi("tomlTable",            P.blue[2], "")

    call s:RecolHi("DiagnosticError", P.red[1], "")
    call s:RecolHi("DiagnosticWarn",  P.yellow[1], "")
    call s:RecolHi("DiagnosticInfo",  P.blue[1], "")
    call s:RecolHi("DiagnosticHint",  P.cyan[1], "")
    call s:RecolHi("DiagnosticOk",    P.green[1], "")
    call s:RecolHi("DiagnosticUnderlineError", "", "", "undercurl", P.red[1])
    call s:RecolHi("DiagnosticUnderlineWarn",  "", "", "undercurl", P.yellow[1])
    call s:RecolHi("DiagnosticUnderlineInfo",  "", "", "undercurl", P.blue[1])
    call s:RecolHi("DiagnosticUnderlineHint",  "", "", "undercurl", P.cyan[1])

    highlight! link ALEError                 DiagnosticUnderlineError
    highlight! link ALEWarning               DiagnosticUnderlineWarn
    highlight! link ALEInfo                  DiagnosticUnderlineInfo
    highlight! link ALEErrorSign             DiagnosticError
    highlight! link ALEWarningSign           DiagnosticWarn
    highlight! link ALEInfoSign              DiagnosticInfo
    highlight! link ALEVirtualTextError      DiagnosticError
    highlight! link ALEVirtualTextWarning    DiagnosticWarn
    highlight! link ALEVirtualTextInfo       DiagnosticInfo
    highlight! link LspErrorHighlight        DiagnosticUnderlineError
    highlight! link LspWarningHighlight      DiagnosticUnderlineWarn
    highlight! link LspInformationHighlight  DiagnosticUnderlineInfo
    highlight! link LspHintHighlight         DiagnosticUnderlineHint
    highlight! link LspErrorText             DiagnosticError
    highlight! link LspWarningText           DiagnosticWarn
    highlight! link LspInformationText       DiagnosticInfo
    highlight! link LspHintText              DiagnosticHint

    let g:colors_name = "{theme_name}"
endfunction
call s:RecolApply()"###,
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
        path,
        content.as_bytes(),
        ("\" recol:start", "\" recol:end"),
    )?;

    Ok(())
}
