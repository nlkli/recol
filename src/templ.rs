use crate::models;

pub fn nvim(theme: &mut models::Theme) -> String {
    let is_light = theme.is_light();
    let c = theme.get_colors();
    const HEAD: &str = r###"
local function themeSyncExe()
    vim.cmd("highlight clear")
    if vim.fn.has("syntax_on") then vim.cmd("syntax reset") end
"###;
    let palette = format!(
        r###"
    local P = {{
        black   = {{ base = "{black_base}",   bright = "{black_bright}",   dim = "{black_dim}" }},
        red     = {{ base = "{red_base}",     bright = "{red_bright}",     dim = "{red_dim}" }},
        green   = {{ base = "{green_base}",   bright = "{green_bright}",   dim = "{green_dim}" }},
        yellow  = {{ base = "{yellow_base}",  bright = "{yellow_bright}",  dim = "{yellow_dim}" }},
        blue    = {{ base = "{blue_base}",    bright = "{blue_bright}",    dim = "{blue_dim}" }},
        magenta = {{ base = "{magenta_base}", bright = "{magenta_bright}", dim = "{magenta_dim}" }},
        cyan    = {{ base = "{cyan_base}",    bright = "{cyan_bright}",    dim = "{cyan_dim}" }},
        white   = {{ base = "{white_base}",   bright = "{white_bright}",   dim = "{white_dim}" }},
        orange  = {{ base = "{orange_base}",  bright = "{orange_bright}",  dim = "{orange_dim}" }},
        pink    = {{ base = "{pink_base}",    bright = "{pink_bright}",    dim = "{pink_dim}" }},
        comment = "{comment}",
        status_line = "{status_line}",
        bg0     = "{bg0}", -- Dark bg (status line and float)
        bg1     = "{bg1}", -- Default bg
        bg2     = "{bg2}", -- Lighter bg (colorcolm folds)
        bg3     = "{bg3}", -- Lighter bg (cursor line)
        bg4     = "{bg4}", -- Conceal, border fg
        fg0     = "{fg0}", -- Lighter fg
        fg1     = "{fg1}", -- Default fg
        fg2     = "{fg2}", -- Darker fg (status line)
        fg3     = "{fg3}", -- Darker fg (line numbers, fold colums)
        sel0    = "{sel0}", -- Popup bg, visual selection bg
        sel1    = "{sel1}", -- Popup sel bg, search bg
        diff = {{
            add = "{diff_add}",
            delete = "{diff_delete}",
            change = "{diff_change}",
            text = "{diff_text}",
        }}
    }}
"###,
        black_base = c.base.black,
        black_bright = c.bright.as_ref().unwrap().black,
        black_dim = c.dim.as_ref().unwrap().black,
        red_base = c.base.red,
        red_bright = c.bright.as_ref().unwrap().red,
        red_dim = c.dim.as_ref().unwrap().red,
        green_base = c.base.green,
        green_bright = c.bright.as_ref().unwrap().green,
        green_dim = c.dim.as_ref().unwrap().green,
        yellow_base = c.base.yellow,
        yellow_bright = c.bright.as_ref().unwrap().yellow,
        yellow_dim = c.dim.as_ref().unwrap().yellow,
        blue_base = c.base.blue,
        blue_bright = c.bright.as_ref().unwrap().blue,
        blue_dim = c.dim.as_ref().unwrap().blue,
        magenta_base = c.base.magenta,
        magenta_bright = c.bright.as_ref().unwrap().magenta,
        magenta_dim = c.dim.as_ref().unwrap().magenta,
        cyan_base = c.base.cyan,
        cyan_bright = c.bright.as_ref().unwrap().cyan,
        cyan_dim = c.dim.as_ref().unwrap().cyan,
        white_base = c.base.white,
        white_bright = c.bright.as_ref().unwrap().white,
        white_dim = c.dim.as_ref().unwrap().white,
        orange_base = c.base.orange.as_ref().unwrap_or(&c.base.yellow),
        orange_bright = c
            .bright
            .as_ref()
            .unwrap()
            .orange
            .as_ref()
            .unwrap_or(&c.base.yellow),
        orange_dim = c
            .dim
            .as_ref()
            .unwrap()
            .orange
            .as_ref()
            .unwrap_or(&c.base.yellow),
        pink_base = c.base.pink.as_ref().unwrap_or(&c.base.red),
        pink_bright = c
            .bright
            .as_ref()
            .unwrap()
            .pink
            .as_ref()
            .unwrap_or(&c.base.red),
        pink_dim = c.dim.as_ref().unwrap().pink.as_ref().unwrap_or(&c.base.red),
        comment = c.comment.as_ref().unwrap(),
        status_line = c.status_line.as_ref().unwrap(),
        bg0 = c.background.index(0),
        bg1 = c.background.index(1),
        bg2 = c.background.index(2),
        bg3 = c.background.index(3),
        bg4 = c.background.index(4),
        fg0 = c.foreground.index(0),
        fg1 = c.foreground.index(1),
        fg2 = c.foreground.index(2),
        fg3 = c.foreground.index(3),
        sel0 = c.selection.index(0),
        sel1 = c.selection.index(1),
        diff_add = c.diff.as_ref().unwrap().add.as_ref().unwrap(),
        diff_delete = c.diff.as_ref().unwrap().delete.as_ref().unwrap(),
        diff_change = c.diff.as_ref().unwrap().change.as_ref().unwrap(),
        diff_text = c.diff.as_ref().unwrap().text.as_ref().unwrap(),
    );

    let spec = format!(
        r###"
    local spec = {{}}
    spec.diag = {{
        error = P.red.base,
        warn  = P.yellow.base,
        info  = P.blue.base,
        hint  = P.green.base,
        ok    = P.green.base,
    }}
    spec.git = {{
        add      = P.green.base,
        removed  = P.red.base,
        changed  = P.yellow.base,
        conflict = P.orange.base,
        ignored  = P.comment,
    }}
    local syn = {{
        bracket     = P.fg2,           -- Brackets and Punctuation
        builtin0    = P.red.base,      -- Builtin variable
        builtin1    = P.cyan.{shade},    -- Builtin type
        builtin2    = P.orange.{shade},  -- Builtin const
        builtin3    = P.red.{shade},     -- Not used
        comment     = P.comment,       -- Comment
        conditional = P.magenta.{shade}, -- Conditional and loop
        const       = P.orange.{shade},  -- Constants, imports and booleans
        dep         = P.fg3,           -- Deprecated
        field       = P.blue.base,     -- Field
        func        = P.blue.{shade},    -- Functions and Titles
        ident       = P.cyan.base,     -- Identifiers
        keyword     = P.magenta.base,  -- Keywords
        number      = P.orange.base,   -- Numbers
        operator    = P.fg2,           -- Operators
        preproc     = P.pink.{shade},    -- PreProc
        regex       = P.yellow.{shade},  -- Regex
        statement   = P.magenta.base,  -- Statements
        string      = P.green.base,    -- Strings
        type        = P.yellow.base,   -- Types
        variable    = "{variable_color}",    -- Variables
    }}
    local trans = false
    local inactive = false
    local inv = {{
        match_paren = false,
        visual = false,
        search = false,
    }}
    local stl = {{
        comments = "NONE",
        conditionals = "NONE",
        constants = "NONE",
        functions = "NONE",
        keywords = "NONE",
        numbers = "NONE",
        operators = "NONE",
        preprocs = "NONE",
        strings = "NONE",
        types = "NONE",
        variables = "NONE",
    }}
"###,
        // variable_color = if theme.is_light() {"black"} else {"white"},
        variable_color = c.variable.as_ref().unwrap(),
        shade = if is_light { "dim" } else { "bright" },
    );
    const TAIL: &str = r###"
    for group, opts in pairs({
        ColorColumn  = { bg = P.bg2 },                                                                       -- used for the columns set with 'colorcolumn'
        Conceal      = { fg = P.bg4 },                                                                       -- placeholder characters substituted for concealed text (see 'conceallevel')
        Cursor       = { fg = P.bg1, bg = P.fg1 },                                                           -- character under the cursor
        lCursor      = { link = "Cursor" },                                                                  -- the character under the cursor when |language-mapping| is used (see 'guicursor')
        CursorIM     = { link = "Cursor" },                                                                  -- like Cursor, but used when in IME mode |CursorIM|
        CursorColumn = { link = "CursorLine" },                                                              -- Screen-column at the cursor, when 'cursorcolumn' is set.
        CursorLine   = { bg = P.bg3 },                                                                       -- Screen-line at the cursor, when 'cursorline' is set.  Low-priority if foreground (ctermfg OR guifg) is not set.
        Directory    = { fg = syn.func },                                                            -- directory names (and other special names in listings)
        DiffAdd      = { bg = P.diff.add },                                                               -- diff mode: Added line |diff.txt|
        DiffChange   = { bg = P.diff.change },                                                            -- diff mode: Changed line |diff.txt|
        DiffDelete   = { bg = P.diff.delete },                                                            -- diff mode: Deleted line |diff.txt|
        DiffText     = { bg = P.diff.text },                                                              -- diff mode: Changed text within a changed line |diff.txt|
        EndOfBuffer  = { fg = P.bg1 },                                                                       -- filler lines (~) after the end of the buffer.  By default, this is highlighted like |hl-NonText|.
        ErrorMsg     = { fg = spec.diag.error },                                                             -- error messages on the command line
        WinSeparator = { fg = P.bg0 },                                                                       -- the column separating vertically split windows
        VertSplit    = { link = "WinSeparator" },                                                            -- the column separating vertically split windows
        Folded       = { fg = P.fg3, bg = P.bg2 },                                                           -- line used for closed folds
        FoldColumn   = { fg = P.fg3 },                                                                       -- 'foldcolumn'
        SignColumn   = { fg = P.fg3 },                                                                       -- column where |signs| are displayed
        SignColumnSB = { link = "SignColumn" },                                                              -- column where |signs| are displayed
        Substitute   = { fg = P.bg1, bg = spec.diag.error },                                                 -- |:substitute| replacement text highlighting
        LineNr       = { fg = P.fg3 },                                                                       -- Line number for ":number" and ":#" commands, and when 'number' or 'relativenumber' option is set.
        CursorLineNr = { fg = spec.diag.warn, style = "bold" },                                              -- Like LineNr when 'cursorline' or 'relativenumber' is set for the cursor line.
        MatchParen   = { fg = spec.diag.warn, style = inv.match_paren and "reverse,bold" or "bold" },        -- The character under the cursor or just before it, if it is a paired bracket, and its match. |pi_paren.txt|
        ModeMsg      = { fg = spec.diag.warn, style = "bold" },                                              -- 'showmode' message (e.g., "-- INSERT -- ")
        MoreMsg      = { fg = spec.diag.info, style = "bold" },                                              -- |more-prompt|
        NonText      = { fg = P.bg4 },                                                                       -- '@' at the end of the window, characters from 'showbreak' and other characters that do not really exist in the text (e.g., ">" displayed when a double-wide character doesn't fit at the end of the line). See also |hl-EndOfBuffer|.
        Normal       = { fg = P.fg1, bg = trans and "NONE" or P.bg1 },                                       -- normal text
        NormalNC     = { fg = P.fg1, bg = (inactive and P.bg0) or (trans and "NONE") or P.bg1 },             -- normal text in non-current windows
        NormalFloat  = { fg = P.fg1, bg = P.bg0 },                                                           -- Normal text in floating windows.
        FloatBorder  = { fg = P.fg3 },                                                                       -- TODO
        Pmenu        = { fg = P.fg1, bg = P.sel0 },                                                          -- Popup menu: normal item.
        PmenuSel     = { bg = P.sel1 },                                                                      -- Popup menu: selected item.
        PmenuSbar    = { link = "Pmenu" },                                                                   -- Popup menu: scrollbar.
        PmenuThumb   = { bg = P.sel1 },                                                                      -- Popup menu: Thumb of the scrollbar.
        Question     = { link = "MoreMsg" },                                                                 -- |hit-enter| prompt and yes/no questions
        QuickFixLine = { link = "CursorLine" },                                                              -- Current |quickfix| item in the quickfix window. Combined with |hl-CursorLine| when the cursor is there.
        Search       = inv.search and { style = "reverse" } or { fg = P.fg1, bg = P.sel1 },                  -- Last search pattern highlighting (see 'hlsearch').  Also used for similar items that need to stand out.
        IncSearch    = inv.search and { style = "reverse" } or { fg = P.bg1, bg = spec.diag.hint },          -- 'incsearch' highlighting; also used for the text replaced with ":s///c"
        CurSearch    = { link = "IncSearch" },                                                               -- Search result under cursor (available since neovim >0.7.0 (https://github.com/neovim/neovim/commit/b16afe4d556af7c3e86b311cfffd1c68a5eed71f)).
        SpecialKey   = { link = "NonText" },                                                                 -- Unprintable characters: text displayed differently from what it really is.  But not 'listchars' whitespace. |hl-Whitespace|
        SpellBad     = { sp = spec.diag.error, style = "undercurl" },                                        -- Word that is not recognized by the spellchecker. |spell| Combined with the highlighting used otherwise.
        SpellCap     = { sp = spec.diag.warn, style = "undercurl" },                                         -- Word that should start with a capital. |spell| Combined with the highlighting used otherwise.
        SpellLocal   = { sp = spec.diag.info, style = "undercurl" },                                         -- Word that is recognized by the spellchecker as one that is used in another region. |spell| Combined with the highlighting used otherwise.
        SpellRare    = { sp = spec.diag.info, style = "undercurl" },                                         -- Word that is recognized by the spellchecker as one that is hardly ever used.  |spell| Combined with the highlighting used otherwise.
        StatusLine   = { fg = P.fg2, bg = P.status_line },                                                           -- status line of current window
        StatusLineNC = { fg = P.fg3, bg = P.status_line },                                                           -- status lines of not-current windows Note: if this is equal to "StatusLine" Vim will use "^^^" in the status line of the current window.
        TabLine      = { fg = P.fg2, bg = P.bg2 },                                                           -- tab pages line, not active tab page label
        TabLineFill  = { bg = P.bg0 },                                                                       -- tab pages line, where there are no labels
        TabLineSel   = { fg = P.bg1, bg = P.fg3 },                                                           -- tab pages line, active tab page label
        Title        = { fg = syn.func, style = "bold" },                                            -- titles for output from ":set all", ":autocmd" etc.
        Visual       = inv.visual and { style = "reverse" } or { bg = P.sel0 },                              -- Visual mode selection
        VisualNOS    = inv.visual and { style = "reverse" } or { link = "visual" },                          -- Visual mode selection when vim is "Not Owning the Selection".
        WarningMsg   = { fg = spec.diag.warn },                                                              -- warning messages
        Whitespace   = { fg = P.bg3 },                                                                       -- "nbsp", "space", "tab" and "trail" in 'listchars'
        WildMenu     = { link = "Pmenu" },                                                                   -- current match in 'wildmenu' completion
        WinBar       = { fg = P.fg3, bg = trans and "NONE" or P.bg1, style = "bold" },                       -- Window bar of current window.
        WinBarNC     = { fg = P.fg3, bg = trans and "NONE" or inactive and P.bg0 or P.bg1, style = "bold" }, --Window bar of not-current windows.

        Comment        = { fg = syn.comment, style = stl.comments },         -- any comment
        Constant       = { fg = syn.const, style = stl.constants },          -- (preferred) any constant
        String         = { fg = syn.string, style = stl.strings },           -- a string constant: "this is a string"
        Character      = { link = "String" },                                -- a character constant: 'c', '\n'
        Number         = { fg = syn.number, style = stl.numbers },           -- a number constant: 234, 0xff
        Float          = { link = "Number" },                                -- a floating point constant: 2.3e10
        Boolean        = { link = "Number" },                                -- a boolean constant: TRUE, false
        Identifier     = { fg = syn.ident, style = stl.variables },          -- (preferred) any variable name
        Function       = { fg = syn.func, style = stl.functions },           -- function name (also: methods for classes)
        Statement      = { fg = syn.keyword, style = stl.keywords },         -- (preferred) any statement
        Conditional    = { fg = syn.conditional, style = stl.conditionals }, -- if, then, else, endif, switch, etc.
        Repeat         = { link = "Conditional" },                           -- for, do, while, etc.
        Label          = { link = "Conditional" },                           -- case, default, etc.
        Operator       = { fg = syn.operator, style = stl.operators },       -- "sizeof", "+", "*", etc.
        Keyword        = { fg = syn.keyword, style = stl.keywords },         -- any other keyword
        Exception      = { link = "Keyword" },                               -- try, catch, throw
        PreProc        = { fg = syn.preproc, style = stl.preprocs },         -- (preferred) generic Preprocessor
        Include        = { link = "PreProc" },                               -- preprocessor #include
        Define         = { link = "PreProc" },                               -- preprocessor #define
        Macro          = { link = "PreProc" },                               -- same as Define
        PreCondit      = { link = "PreProc" },                               -- preprocessor #if, #else, #endif, etc.
        Type           = { fg = syn.type, style = stl.types },               -- (preferred) int, long, char, etc.
        StorageClass   = { link = "Type" },                                  -- static, register, volatile, etc.
        Structure      = { link = "Type" },                                  -- struct, union, enum, etc.
        Typedef        = { link = "Type" },                                  -- A typedef
        Special        = { fg = syn.func },                                  -- (preferred) any special symbol
        SpecialChar    = { link = "Special" },                               -- special character in a constant
        Tag            = { link = "Special" },                               -- you can use CTRL-] on this
        Delimiter      = { link = "Special" },                               -- character that needs attention
        SpecialComment = { link = "Special" },                               -- special things inside a comment
        Debug          = { link = "Special" },                               -- debugging statements
        Underlined     = { style = "underline" },                            -- (preferred) text that stands out, HTML links
        Bold           = { style = "bold" },
        Italic         = { style = "italic" },
        Error          = { fg = spec.diag.error },            -- (preferred) any erroneous construct
        Todo           = { fg = P.bg1, bg = spec.diag.info }, -- (preferred) anything that needs extra attention; mostly the keywords TODO FIXME and XXX
        qfLineNr       = { link = "lineNr" },
        qfFileName     = { link = "Directory" },
        diffAdded      = { fg = spec.git.add },         -- Added lines ("^+.*" | "^>.*")
        diffRemoved    = { fg = spec.git.removed },     -- Removed lines ("^-.*" | "^<.*")
        diffChanged    = { fg = spec.git.changed },     -- Changed lines ("^! .*")
        diffOldFile    = { fg = spec.diag.warn },       -- Old file that is being diff against
        diffNewFile    = { fg = spec.diag.hint },       -- New file that is being compared to the old file
        diffFile       = { fg = spec.diag.info },       -- The filename of the diff ("diff --git a/readme.md b/readme.md")
        diffLine       = { fg = syn.builtin2 }, -- Line information ("@@ -169,6 +169,9 @@")
        diffIndexLine  = { fg = syn.preproc },  -- Index line of diff ("index bf3763d..94f0f62 100644")

        DiagnosticError          = { fg = spec.diag.error },
        DiagnosticWarn           = { fg = spec.diag.warn },
        DiagnosticInfo           = { fg = spec.diag.info },
        DiagnosticHint           = { fg = spec.diag.hint },
        DiagnosticOk             = { fg = spec.diag.ok },
        DiagnosticSignError      = { link = "DiagnosticError" },
        DiagnosticSignWarn       = { link = "DiagnosticWarn" },
        DiagnosticSignInfo       = { link = "DiagnosticInfo" },
        DiagnosticSignHint       = { link = "DiagnosticHint" },
        DiagnosticSignOk         = { link = "DiagnosticOk" },
        DiagnosticUnderlineError = { style = "undercurl", sp = spec.diag.error },
        DiagnosticUnderlineWarn  = { style = "undercurl", sp = spec.diag.warn },
        DiagnosticUnderlineInfo  = { style = "undercurl", sp = spec.diag.info },
        DiagnosticUnderlineHint  = { style = "undercurl", sp = spec.diag.hint },
        DiagnosticUnderlineOk    = { style = "undercurl", sp = spec.diag.ok },

        ["@variable"] = { fg = syn.variable, style = stl.variables },             -- various variable names
        ["@variable.builtin"] = { fg = syn.builtin0, style = stl.variables },     -- built-in variable names (e.g. `this`)
        ["@variable.parameter"] = { fg = syn.builtin1, style = stl.variables },   -- parameters of a function
        ["@variable.member"] = { fg = syn.field },                                -- object and struct fields
        ["@constant"] = { link = "Constant" },                                    -- constant identifiers
        ["@constant.builtin"] = { fg = syn.builtin2, style = stl.keywords },      -- built-in constant values
        ["@constant.macro"] = { link = "Macro" },                                 -- constants defined by the preprocessor
        ["@module"] = { fg = syn.builtin1 },                                      -- modules or namespaces
        ["@label"] = { link = "Label" },                                          -- GOTO and other labels (e.g. `label:` in C), including heredoc labels
        ["@string"] = { link = "String" },                                        -- string literals
        ["@string.regexp"] = { fg = syn.regex, style = stl.strings },             -- regular expressions
        ["@string.escape"] = { fg = syn.regex, style = "bold" },                  -- escape sequences
        ["@string.special"] = { link = "Special" },                               -- other special strings (e.g. dates)
        ["@string.special.url"] = { fg = syn.const, style = "italic,underline" }, -- URIs (e.g. hyperlinks)
        ["@character"] = { link = "Character" },                                  -- character literals
        ["@character.special"] = { link = "SpecialChar" },                        -- special characters (e.g. wildcards)
        ["@boolean"] = { link = "Boolean" },                                      -- boolean literals
        ["@number"] = { link = "Number" },                                        -- numeric literals
        ["@number.float"] = { link = "Float" },                                   -- floating-point number literals
        ["@type"] = { link = "Type" },                                            -- type or class definitions and annotations
        ["@type.builtin"] = { fg = syn.builtin1, style = stl.types },             -- built-in types
        ["@attribute"] = { link = "Constant" },                                   -- attribute annotations (e.g. Python decorators)
        ["@property"] = { fg = syn.field },                                       -- the key in key/value pairs
        ["@function"] = { link = "Function" },                                    -- function definitions
        ["@function.builtin"] = { fg = syn.builtin0, style = stl.functions },     -- built-in functions
        ["@function.macro"] = { fg = syn.builtin0, style = stl.functions },       -- preprocessor macros
        ["@constructor"] = { fg = syn.ident },                                    -- constructor calls and definitions
        ["@operator"] = { link = "Operator" },                                    -- symbolic operators (e.g. `+` / `*`)
        ["@keyword"] = { link = "Keyword" },                                      -- keywords not fitting into specific categories
        ["@keyword.function"] = { fg = syn.keyword, style = stl.functions },      -- keywords that define a function (e.g. `func` in Go, `def` in Python)
        ["@keyword.operator"] = { fg = syn.operator, style = stl.operators },     -- operators that are English words (e.g. `and` / `or`)
        ["@keyword.import"] = { link = "Include" },                               -- keywords for including modules (e.g. `import` / `from` in Python)
        ["@keyword.storage"] = { link = "StorageClass" },                         -- modifiers that affect storage in memory or life-time
        ["@keyword.repeat"] = { link = "Repeat" },                                -- keywords related to loops (e.g. `for` / `while`)
        ["@keyword.return"] = { fg = syn.builtin0, style = stl.keywords },        -- keywords like `return` and `yield`
        ["@keyword.exception"] = { link = "Exception" },                          -- keywords related to exceptions (e.g. `throw` / `catch`)
        ["@keyword.conditional"] = { link = "Conditional" },                      -- keywords related to conditionals (e.g. `if` / `else`)
        ["@keyword.conditional.ternary"] = { link = "Conditional" },              -- ternary operator (e.g. `?` / `:`)
        ["@punctuation.delimiter"] = { fg = syn.bracket },                        -- delimiters (e.g. `;` / `.` / `,`)
        ["@punctuation.bracket"] = { fg = syn.bracket },                          -- brackets (e.g. `()` / `{}` / `[]`)
        ["@punctuation.special"] = { fg = syn.builtin1, style = stl.operators },  -- special symbols (e.g. `{}` in string interpolation)
        ["@comment"] = { link = "Comment" },                                      -- line and block comments
        ["@comment.error"] = { fg = P.bg1, bg = spec.diag.error },                -- error-type comments (e.g. `ERROR`, `FIXME`, `DEPRECATED:`)
        ["@comment.warning"] = { fg = P.bg1, bg = spec.diag.warn },               -- warning-type comments (e.g. `WARNING:`, `FIX:`, `HACK:`)
        ["@comment.todo"] = { fg = P.bg1, bg = spec.diag.hint },                  -- todo-type comments (e.g. `TODO:`, `WIP:`, `FIXME:`)
        ["@comment.note"] = { fg = P.bg1, bg = spec.diag.info },                  -- note-type comments (e.g. `NOTE:`, `INFO:`, `XXX`)
        ["@markup"] = { fg = P.fg1 },                                             -- For strings considerated text in a markup language.
        ["@markup.strong"] = { fg = P.red.base, style = "bold" },                 -- bold text
        ["@markup.italic"] = { link = "Italic" },                                 -- italic text
        ["@markup.strikethrough"] = { fg = P.fg1, style = "strikethrough" },      -- struck-through text
        ["@markup.underline"] = { link = "Underline" },                           -- underlined text (only for literal underline markup!)
        ["@markup.heading"] = { link = "Title" },                                 -- headings, titles (including markers)
        ["@markup.quote"] = { fg = P.fg2 },                                       -- block quotes
        ["@markup.math"] = { fg = syn.func },                                     -- math environments (e.g. `$ ... $` in LaTeX)
        ["@markup.link"] = { fg = syn.keyword, style = "bold" },                  -- text references, footnotes, citations, etc.
        ["@markup.link.label"] = { link = "Special" },                            -- link, reference descriptions
        ["@markup.link.url"] = { fg = syn.const, style = "italic,underline" },    -- URL-style links
        ["@markup.raw"] = { fg = syn.ident, style = "italic" },                   -- literal or verbatim text (e.g. inline code)
        ["@markup.raw.block"] = { fg = P.pink.base },                             -- literal or verbatim text as a stand-alone block (use priority 90 for blocks with injections)
        ["@markup.list"] = { fg = syn.builtin1, style = stl.operators },          -- list markers
        ["@markup.list.checked"] = { fg = P.green.base },                         -- checked todo-style list markers
        ["@markup.list.unchecked"] = { fg = P.yellow.base },                      -- unchecked todo-style list markers
        ["@diff.plus"] = { link = "diffAdded" },                                  -- added text (for diff files)
        ["@diff.minus"] = { link = "diffRemoved" },                               -- deleted text (for diff files)
        ["@diff.delta"] = { link = "diffChanged" },                               -- changed text (for diff files)
        ["@tag"] = { fg = syn.keyword },                                          -- XML-style tag names (and similar)
        ["@tag.attribute"] = { fg = syn.func, style = "italic" },                 -- XML-style tag attributes
        ["@tag.delimiter"] = { fg = syn.builtin1 },                               -- XML-style tag delimiters
        ["@label.json"] = { fg = syn.func },                                      -- For labels: label: in C and :label: in Lua.
        ["@constructor.lua"] = { fg = P.fg2 },                                    -- Lua's constructor is { }
        ["@field.rust"] = { fg = P.fg2 },
        ["@variable.member.yaml"] = { fg = syn.func },                            -- For fields.

        ["@lsp.type.boolean"] = { link = "@boolean" },
        ["@lsp.type.builtinType"] = { link = "@type.builtin" },
        ["@lsp.type.comment"] = { link = "@comment" },
        ["@lsp.type.enum"] = { link = "@type" },
        ["@lsp.type.enumMember"] = { link = "@constant" },
        ["@lsp.type.escapeSequence"] = { link = "@string.escape" },
        ["@lsp.type.formatSpecifier"] = { link = "@punctuation.special" },
        ["@lsp.type.interface"] = { fg = syn.builtin3 },
        ["@lsp.type.keyword"] = { link = "@keyword" },
        ["@lsp.type.namespace"] = { link = "@module" },
        ["@lsp.type.number"] = { link = "@number" },
        ["@lsp.type.operator"] = { link = "@operator" },
        ["@lsp.type.parameter"] = { link = "@parameter" },
        ["@lsp.type.property"] = { link = "@property" },
        ["@lsp.type.selfKeyword"] = { link = "@variable.builtin" },
        ["@lsp.type.typeAlias"] = { link = "@type.definition" },
        ["@lsp.type.unresolvedReference"] = { link = "@error" },
    }) do
        if opts.style and opts.style ~= "NONE" then
            for token in opts.style:gmatch("[^,%s]+") do
                opts[token] = true
            end
        end
        opts.style = nil
        vim.api.nvim_set_hl(0, group, opts)
    end
end
themeSyncExe()
"###;

    format!("{HEAD}{palette}{spec}{TAIL}")
}
