use crate::utils;
use recol_lib::{self as lib, CssColor};
use std::{io, path::Path};

pub fn write_theme_to_config(path: impl AsRef<Path>, theme: &lib::Theme) -> io::Result<()> {
    let c = theme.colors.clone().into_advanced(None);

    let pick = |bright: &CssColor, dim: &CssColor| -> CssColor {
        if theme.is_light {
            dim.clone()
        } else {
            bright.clone()
        }
    };

    let syn_comment = c.comment;
    let syn_const = pick(&c.bright.orange, &c.dim.orange);
    let syn_string = c.base.green.clone();
    let syn_number = c.base.orange.clone();
    let syn_ident = c.base.cyan.clone();
    let syn_func = pick(&c.bright.blue, &c.dim.blue);
    let syn_statement = c.base.magenta.clone();
    let syn_conditional = pick(&c.bright.magenta, &c.dim.magenta);
    let syn_operator = c.fg[2].clone();
    let syn_keyword = c.base.magenta.clone();
    let syn_preproc = pick(&c.bright.pink, &c.dim.pink);
    let syn_type = c.base.yellow.clone();
    let syn_builtin2 = pick(&c.bright.orange, &c.dim.orange);

    let diag_error = c.base.red.clone();
    let diag_warn = c.base.yellow.clone();
    let diag_info = c.base.blue.clone();
    let diag_hint = c.base.green.clone();

    let git_add = c.base.green.clone();
    let git_removed = c.base.red.clone();
    let git_changed = c.base.blue.clone();

    let background = if theme.is_light { "light" } else { "dark" };

    let content = format!(
        r#"
hi clear
if exists("syntax_on")
  syntax reset
endif
let g:colors_name = "{theme_name}"
set background={background}

hi Normal        guifg={fg1} guibg={bg1}
hi NormalNC      guifg={fg1} guibg={bg1}
hi Terminal      guifg={fg1} guibg={bg1}
hi ColorColumn   guibg={bg2}
hi Conceal       guifg={bg4}
hi Cursor        guifg={cur_fg} guibg={cur_bg}
hi lCursor       guifg={cur_fg} guibg={cur_bg}
hi CursorIM      guifg={cur_fg} guibg={cur_bg}
hi CursorColumn  guibg={bg3}
hi CursorLine    guibg={bg3}
hi Directory     guifg={syn_func}
hi EndOfBuffer   guifg={bg1}
hi ErrorMsg      guifg={diag_error}
hi VertSplit     guifg={bg0}
hi WinSeparator  guifg={bg0}
hi Folded        guifg={fg3} guibg={bg2}
hi FoldColumn    guifg={fg3}
hi SignColumn    guifg={fg3}
hi Substitute    guifg={bg1} guibg={diag_error}
hi LineNr        guifg={fg3}
hi CursorLineNr  guifg={diag_warn} gui=bold
hi MatchParen    guifg={diag_warn} gui=bold
hi ModeMsg       guifg={diag_warn} gui=bold
hi MoreMsg       guifg={diag_info} gui=bold
hi Question      guifg={diag_info} gui=bold
hi NonText       guifg={bg4}
hi SpecialKey    guifg={bg4}
hi Pmenu         guifg={fg1} guibg={sel0}
hi PmenuSel      guifg={fg1} guibg={sel1}
hi PmenuSbar     guibg={sel0}
hi PmenuThumb    guibg={sel1}
hi QuickFixLine  guibg={bg3}
hi Search        guifg={fg1} guibg={sel1}
hi IncSearch     guifg={bg1} guibg={diag_hint}
hi CurSearch     guifg={bg1} guibg={diag_hint}
hi StatusLine       guifg={fg2} guibg={bg0}
hi StatusLineNC     guifg={fg3} guibg={bg0}
hi StatusLineTerm   guifg={fg2} guibg={bg0}
hi StatusLineTermNC guifg={fg3} guibg={bg0}
hi TabLine       guifg={fg2} guibg={bg2}
hi TabLineFill   guibg={bg0}
hi TabLineSel    guifg={bg1} guibg={fg3}
hi Title         guifg={syn_func} gui=bold
hi Visual        guibg={sel0}
hi VisualNOS     guibg={sel0}
hi WarningMsg    guifg={diag_warn}
hi Whitespace    guifg={bg3}
hi WildMenu      guifg={fg1} guibg={sel0}
hi WinBar        guifg={fg3} guibg={bg1} gui=bold
hi WinBarNC      guifg={fg3} guibg={bg1} gui=bold
hi Menu          guifg={fg1} guibg={bg1}
hi Scrollbar     guibg={bg1}
hi Tooltip       guifg={fg1} guibg={bg0}

hi SpellBad   gui=undercurl guisp={diag_error}
hi SpellCap   gui=undercurl guisp={diag_warn}
hi SpellLocal gui=undercurl guisp={diag_info}
hi SpellRare  gui=undercurl guisp={diag_info}

hi DiffAdd    guibg={diff_add}
hi DiffChange guibg={diff_change}
hi DiffDelete guibg={diff_delete}
hi DiffText   guibg={diff_text}

hi Comment        guifg={syn_comment}
hi Constant       guifg={syn_const}
hi String         guifg={syn_string}
hi Character      guifg={syn_string}
hi Number         guifg={syn_number}
hi Boolean        guifg={syn_number}
hi Float          guifg={syn_number}
hi Identifier     guifg={syn_ident}
hi Function       guifg={syn_func}
hi Statement      guifg={syn_statement}
hi Conditional    guifg={syn_conditional}
hi Repeat         guifg={syn_conditional}
hi Label          guifg={syn_conditional}
hi Operator       guifg={syn_operator}
hi Keyword        guifg={syn_keyword}
hi Exception      guifg={syn_keyword}
hi PreProc        guifg={syn_preproc}
hi Include        guifg={syn_preproc}
hi Define         guifg={syn_preproc}
hi Macro          guifg={syn_preproc}
hi PreCondit      guifg={syn_preproc}
hi Type           guifg={syn_type}
hi StorageClass   guifg={syn_type}
hi Structure      guifg={syn_type}
hi Typedef        guifg={syn_type}
hi Special        guifg={syn_func}
hi SpecialChar    guifg={syn_func}
hi Tag            guifg={syn_func}
hi Delimiter      guifg={syn_func}
hi SpecialComment guifg={syn_func}
hi Debug          guifg={syn_func}
hi Underlined     guifg={syn_func} gui=underline
hi Ignore         guifg={bg2}
hi Error          guifg={diag_error}
hi Todo           guifg={bg1} guibg={diag_info}

hi qfLineNr      guifg={fg3}
hi qfFileName    guifg={syn_func}

hi diffAdded     guifg={git_add}
hi diffRemoved   guifg={git_removed}
hi diffChanged   guifg={git_changed}
hi diffOldFile   guifg={diag_warn}
hi diffNewFile   guifg={diag_hint}
hi diffFile      guifg={diag_info}
hi diffLine      guifg={syn_builtin2}
hi diffIndexLine guifg={syn_preproc}"#,
        theme_name = theme.name,
        background = background,
        bg0 = c.bg[0],
        bg1 = c.bg[1],
        bg2 = c.bg[2],
        bg3 = c.bg[3],
        bg4 = c.bg[4],
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
        syn_comment = syn_comment,
        syn_const = syn_const,
        syn_string = syn_string,
        syn_number = syn_number,
        syn_ident = syn_ident,
        syn_func = syn_func,
        syn_statement = syn_statement,
        syn_conditional = syn_conditional,
        syn_operator = syn_operator,
        syn_keyword = syn_keyword,
        syn_preproc = syn_preproc,
        syn_type = syn_type,
        syn_builtin2 = syn_builtin2,
        diag_error = diag_error,
        diag_warn = diag_warn,
        diag_info = diag_info,
        diag_hint = diag_hint,
        git_add = git_add,
        git_removed = git_removed,
        git_changed = git_changed,
    );

    utils::write_content_inside_text_block(
        path,
        content.as_bytes(),
        ("\" recol:start", "\" recol:end"),
    )?;

    Ok(())
}
