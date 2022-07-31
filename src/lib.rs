/*
                             O X O C A R B O N
       _..._         _..._         _..._         _..._         _..._
     .:::::::.     .::::. `.     .::::  `.     .::'   `.     .'     `.
    :::::::::::   :::::::.  :   ::::::    :   :::       :   :         :
    :::::::::::   ::::::::  :   ::::::    :   :::       :   :         :
    `:::::::::'   `::::::' .'   `:::::   .'   `::.     .'   `.       .'
      `':::''       `'::'-'       `'::.-'       `':..-'       `-...-'

  Colorscheme name:    oxocarbon: oxidized carbon
  Description:         Neovim Colorscheme inspired from the IBM Carbon Palette in rust with nvim-oxi
  Author:              https://github.com/shaunsingh

  Losely based off of IBM Carbon Palette
  https://www.ibm.com/brand/experience-guides/developer/brand/color/)

*/

use nvim_oxi::{self as oxi, api, opts::*};

#[oxi::module]
fn oxocarbon() -> oxi::Result<String> {
    api::set_option("termguicolors", true)?;

    /*
    Decides pallete based on value of vim background.
    Each palette consists of 18 colors:
      - 16 base colors (based on the base16 standard)
      - 1 blend color (used for contrast for floating menus and such),
      - 1 transparent color (used for well, transparency)
    */
    let oxocarbon: [&str; 18] = match api::get_option::<String>("background").unwrap().as_str() {
        "dark" => [
            "#161616", "#262626", "#393939", "#525252", "#dde1e6", "#f2f4f8", "#ffffff", "#08bdba",
            "#3ddbd9", "#78a9ff", "#ee5396", "#33b1ff", "#ff7eb6", "#42be65", "#be95ff", "#82cfff",
            "#131313", "",
        ],
        "light" => [
            "#FFFFFF", "#FAFAFA", "#ECEFF1", "#161616", "#37474F", "#90A4AE", "#525252", "#08bdba",
            "#ff7eb6", "#ee5396", "#FF6F00", "#0f62fe", "#673AB7", "#42be65", "#be95ff", "#FFAB91",
            "#FAFAFA", "",
        ],
        _ => panic!("Error: Background not set"),
    };

    for x in 0..15 {
        api::set_var("terminal_color_{x}", oxocarbon[x].to_string())?;
    }

    macro_rules! highlight {
        ($hlname:expr, $fgbase:expr, $bgbase:expr) => {
            api::set_hl(
                0,
                stringify!($hlname),
                Some(
                    &SetHighlightOpts::builder()
                        .fg(oxocarbon[$fgbase])
                        .bg(oxocarbon[$bgbase])
                        .build(),
                ),
            )?;
        };
    }

    // editor
    highlight!(ColorColumn, 17, 1);
    highlight!(Cursor, 0, 4);
    highlight!(CursorLine, 17, 1);
    highlight!(CursorColumn, 17, 1);
    highlight!(CursorLineNr, 4, 17);
    highlight!(Error, 4, 11);
    highlight!(LineNr, 3, 0);
    highlight!(NonText, 2, 17);
    highlight!(Normal, 4, 0);
    highlight!(Pmenu, 4, 1);
    highlight!(PmenuSbar, 4, 1);
    highlight!(PmenuSel, 8, 2);
    highlight!(PmenuThumb, 8, 2);
    highlight!(SpecialKey, 3, 17);
    highlight!(Visual, 17, 2);
    highlight!(VisualNOS, 17, 2);
    api::set_hl(
        0,
        "MatchParen",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[17])
                .bg(oxocarbon[2])
                .underline(true)
                .build(),
        ),
    )?;

    // diagnostics
    highlight!(DiagnosticWarn, 8, 17);
    highlight!(DiagnosticError, 10, 17);
    highlight!(DiagnosticInfo, 4, 17);
    highlight!(DiagnosticHint, 4, 17);
    api::set_hl(
        0,
        "DiagnosticUnderlineWarn",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[8])
                .bg(oxocarbon[17])
                .undercurl(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "DiagnosticUnderlineError",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[10])
                .bg(oxocarbon[17])
                .undercurl(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "DiagnosticUnderlineInfo",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[4])
                .bg(oxocarbon[17])
                .undercurl(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "DiagnosticUnderlineHint",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[4])
                .bg(oxocarbon[17])
                .undercurl(true)
                .build(),
        ),
    )?;

    // lsp
    highlight!(LspReferenceText, 17, 3);
    highlight!(LspReferenceread, 17, 3);
    highlight!(LspReferenceWrite, 17, 3);
    highlight!(LspSignatureActiveParameter, 8, 17);

    // gutter
    highlight!(Folded, 3, 1);
    highlight!(FoldColumn, 3, 0);
    highlight!(SignColumn, 1, 0);

    // navigation
    highlight!(Directory, 8, 17);

    // prompts
    highlight!(EndOfBuffer, 1, 17);
    highlight!(ErrorMsg, 4, 11);
    highlight!(ModeMsg, 4, 17);
    highlight!(MoreMsg, 8, 17);
    highlight!(Question, 4, 17);
    highlight!(WarningMsg, 0, 13);
    highlight!(WildMenu, 8, 1);

    // search
    highlight!(IncSearch, 6, 10);
    highlight!(Search, 1, 8);

    // tabs
    highlight!(TabLine, 4, 1);
    highlight!(TabLineFill, 4, 1);
    highlight!(TabLineSel, 8, 3);

    // window
    highlight!(Title, 4, 17);
    highlight!(VertSplit, 1, 0);

    // regular syntax
    highlight!(Boolean, 9, 17);
    highlight!(Character, 14, 17);
    highlight!(Comment, 3, 17);
    highlight!(Conceal, 17, 17);
    highlight!(Conditional, 9, 17);
    highlight!(Constant, 4, 17);
    highlight!(Decorator, 12, 17);
    highlight!(Define, 9, 17);
    highlight!(Delimeter, 6, 17);
    highlight!(Exception, 9, 17);
    highlight!(Float, 15, 17);
    highlight!(Function, 8, 17);
    highlight!(Identifier, 4, 17);
    highlight!(Include, 9, 17);
    highlight!(Keyword, 9, 17);
    highlight!(Label, 9, 17);
    highlight!(Number, 15, 17);
    highlight!(Operator, 9, 17);
    highlight!(PreProc, 9, 17);
    highlight!(Repeat, 9, 17);
    highlight!(Special, 4, 17);
    highlight!(SpecialChar, 4, 17);
    highlight!(SpecialComment, 8, 17);
    highlight!(Statement, 9, 17);
    highlight!(StorageClass, 9, 17);
    highlight!(String, 14, 17);
    highlight!(Structure, 9, 17);
    highlight!(Tag, 4, 17);
    highlight!(Todo, 13, 17);
    highlight!(Type, 9, 17);
    highlight!(Typedef, 9, 17);

    // treesitter
    highlight!(TSAnnotation, 12, 17);
    highlight!(TSAttribute, 15, 17);
    highlight!(TSBoolean, 9, 17);
    highlight!(TSCharacter, 14, 17);
    highlight!(TSConstructor, 9, 17);
    highlight!(TSConditional, 9, 17);
    highlight!(TSConstant, 14, 17);
    highlight!(TSConstBuiltin, 7, 17);
    highlight!(TSConstMacro, 7, 17);
    highlight!(TSError, 11, 17);
    highlight!(TSException, 15, 17);
    highlight!(TSField, 4, 17);
    highlight!(TSFloat, 15, 17);
    highlight!(TSFuncBuiltin, 12, 17);
    highlight!(TSFuncMacro, 7, 17);
    highlight!(TSInclude, 9, 17);
    highlight!(TSKeyword, 9, 17);
    highlight!(TSKeywordFunction, 8, 17);
    highlight!(TSKeywordOperator, 8, 17);
    highlight!(TSLabel, 15, 17);
    highlight!(TSMethod, 7, 17);
    highlight!(TSNamespace, 4, 17);
    highlight!(TSNumber, 15, 17);
    highlight!(TSOperator, 9, 17);
    highlight!(TSParameter, 4, 17);
    highlight!(TSParameterReference, 4, 17);
    highlight!(TSProperty, 10, 17);
    highlight!(TSPunctDelimiter, 8, 17);
    highlight!(TSPunctBracket, 8, 17);
    highlight!(TSPunctSpecial, 8, 17);
    highlight!(TSRepeat, 9, 17);
    highlight!(TSString, 14, 17);
    highlight!(TSStringRegex, 7, 17);
    highlight!(TSStringEscape, 15, 17);
    highlight!(TSTag, 4, 17);
    highlight!(TSTagDelimiter, 15, 17);
    highlight!(TSText, 4, 17);
    highlight!(TSTitle, 10, 17);
    highlight!(TSLiteral, 4, 17);
    highlight!(TSType, 9, 17);
    highlight!(TSTypeBuiltin, 4, 17);
    highlight!(TSVariable, 4, 17);
    highlight!(TSVariableBuiltin, 4, 17);
    highlight!(TreesitterContext, 17, 1);
    api::set_hl(
        0,
        "TSStrong",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[17])
                .bg(oxocarbon[17])
                .bold(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "TSComment",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[3])
                .bg(oxocarbon[17])
                .italic(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "TSFunction",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[12])
                .bg(oxocarbon[17])
                .bold(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "TSSymbol",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[15])
                .bg(oxocarbon[17])
                .bold(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "TSEmphasis",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[10])
                .bg(oxocarbon[17])
                .bold(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "TSUnderline",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[4])
                .bg(oxocarbon[17])
                .underline(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "TSStrike",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[4])
                .bg(oxocarbon[17])
                .strikethrough(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "TSURI",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[14])
                .bg(oxocarbon[17])
                .underline(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "TSCurrentScope",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[17])
                .bg(oxocarbon[17])
                .bold(true)
                .build(),
        ),
    )?;

    // neovim
    highlight!(NvimInternalError, 0, 8);
    highlight!(NormalFloat, 5, 16);
    highlight!(FloatBorder, 16, 16);
    highlight!(NormalNC, 5, 0);
    highlight!(TermCursor, 0, 4);
    highlight!(TermCursorNC, 0, 4);

    // statusline/winbar
    highlight!(StatusLine, 3, 0);
    highlight!(StatusReplace, 0, 8);
    highlight!(StatusInsert, 0, 12);
    highlight!(StatusVisual, 0, 14);
    highlight!(StatusTerminal, 0, 11);
    api::set_hl(
        0,
        "WinBar",
        Some(
            &SetHighlightOpts::builder()
                .fg("#a2a9b0")
                .bg(oxocarbon[0])
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "StatusPosition",
        Some(
            &SetHighlightOpts::builder()
                .fg("#a2a9b0")
                .bg(oxocarbon[0])
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "StatusNormal",
        Some(
            &SetHighlightOpts::builder()
                .fg("#a2a9b0")
                .bg(oxocarbon[0])
                .underline(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "StatusCommand",
        Some(
            &SetHighlightOpts::builder()
                .fg("#a2a9b0")
                .bg(oxocarbon[0])
                .underline(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "StatusLineDiagnosticWarn",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[14])
                .bg(oxocarbon[0])
                .bold(true)
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "StatusLineDiagnosticError",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[8])
                .bg(oxocarbon[0])
                .bold(true)
                .build(),
        ),
    )?;

    // telescope
    highlight!(TelescopeBorder, 16, 16);
    highlight!(TelescopePromptBorder, 2, 2);
    highlight!(TelescopePromptNormal, 5, 2);
    highlight!(TelescopePromptPrefix, 8, 2);
    highlight!(TelescopeNormal, 17, 16);
    highlight!(TelescopePreviewTitle, 2, 11);
    highlight!(TelescopePromptTitle, 2, 8);
    highlight!(TelescopeResultsTitle, 16, 16);
    highlight!(TelescopeSelection, 17, 2);
    highlight!(TelescopePreviewLine, 17, 1);

    // notify
    highlight!(NotifyERRORBorder, 8, 17);
    highlight!(NotifyWARNBorder, 15, 17);
    highlight!(NotifyINFOBorder, 5, 17);
    highlight!(NotifyDEBUGBorder, 13, 17);
    highlight!(NotifyTRACEBorder, 13, 17);
    highlight!(NotifyERRORIcon, 8, 17);
    highlight!(NotifyWARNIcon, 15, 17);
    highlight!(NotifyINFOIcon, 5, 17);
    highlight!(NotifyDEBUGIcon, 13, 17);
    highlight!(NotifyTRACEIcon, 13, 17);
    highlight!(NotifyERRORTitle, 8, 17);
    highlight!(NotifyWARNTitle, 15, 17);
    highlight!(NotifyINFOTitle, 5, 17);
    highlight!(NotifyDEBUGTitle, 13, 17);
    highlight!(NotifyTRACETitle, 13, 17);

    // cmp
    highlight!(CmpItemAbbrMatchFuzzy, 4, 17);
    highlight!(CmpItemKindInterface, 11, 17);
    highlight!(CmpItemKindText, 8, 17);
    highlight!(CmpItemKindVariable, 13, 17);
    highlight!(CmpItemKindProperty, 10, 17);
    highlight!(CmpItemKindKeyword, 9, 17);
    highlight!(CmpItemKindUnit, 14, 17);
    highlight!(CmpItemKindFunction, 12, 17);
    highlight!(CmpItemKindMethod, 7, 17);
    api::set_hl(
        0,
        "CmpItemAbbr",
        Some(
            &SetHighlightOpts::builder()
                .fg("#adadad")
                .bg(oxocarbon[17])
                .build(),
        ),
    )?;
    api::set_hl(
        0,
        "CmpItemAbbrMatch",
        Some(
            &SetHighlightOpts::builder()
                .fg(oxocarbon[5])
                .bg(oxocarbon[17])
                .bold(true)
                .build(),
        ),
    )?;

    // nvimtree
    highlight!(NvimTreeImageFile, 12, 17);
    highlight!(NvimTreeFolderIcon, 12, 17);
    highlight!(NvimTreeWinSeperator, 0, 0);
    highlight!(NvimTreeFolderName, 9, 17);
    highlight!(NvimTreeIndentMarker, 2, 17);
    highlight!(NvimTreeEmptyFolderName, 15, 17);
    highlight!(NvimTreeOpenedFolderName, 15, 17);
    highlight!(NvimTreeNormal, 4, 16);

    // neogit
    highlight!(NeogitBranch, 10, 17);
    highlight!(NeogitRemote, 9, 17);
    highlight!(NeogitDiffAddHighlight, 13, 2);
    highlight!(NeogitDiffDeleteHighlight, 9, 2);
    highlight!(NeogitDiffContextHighlight, 4, 1);
    highlight!(NeogitHunkHeader, 4, 2);
    highlight!(NeogitHunkHeaderHighlight, 4, 3);

    // gitsigns
    highlight!(GitSignsAdd, 8, 17);
    highlight!(GitSignsChange, 9, 17);
    highlight!(GitSignsDelete, 14, 17);

    // parinfer
    highlight!(Trailhighlight, 3, 17);

    // hydra
    highlight!(HydraRed, 12, 17);
    highlight!(HydraBlue, 9, 17);
    highlight!(HydraAmaranth, 10, 17);
    highlight!(HydraTeal, 8, 17);
    highlight!(HydraPink, 14, 17);
    highlight!(HydraHint, 17, 16);

    Ok("Oxocarbon Loaded".to_string())
}