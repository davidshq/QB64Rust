# Proposal

## Why

M5b, "the editor without QB64pe", comes between M5 and M6 (user, `DECISIONS.md` 2026-10-25; `ROADMAP.md` M5b).
Its first item is a formatter that matches `qb64pe -y`. That formatter was taken out of M2's exit criterion on
2026-10-09, and the extension still formats by running the old compiler. Formatting is now the only part of the
editor that needs `qb64pe.exe` itself, not just its toolchain. While it does, Format Document writes a temporary
file beside the document, cannot see unsaved included files, and follows whatever the user's QB64pe `config.ini`
says. This change is the first of M5b's two changes. The language-server features (hover, references, rename,
completion) are the second, `m5b-language-features`, proposed after this one.

Measured on 2026-10-25 with the clone's `qb64pe.exe`: `-y` is more than spacing and keyword case. It writes a
variable or procedure name with the spelling of its first declaration everywhere, including a call written before
the `FUNCTION`. It writes a built-in with the old table's spelling (`Len`, `_RGB32`). It writes `?` as `Print`.
It writes nothing when the program has a compile error, because the output is written only after both passes.
So the formatter has to run the whole front end and read `sema`'s symbol table. It cannot work from the tree
alone.

## What Changes

- **`qb64rust -y <file.bas> -o <out.bas>`** writes the file laid out as `qb64pe -y` lays it out, with the same
  `-f:` layout settings: `AutoIndent`, `AutoIndentSize` (1 to 64), `IndentSubs`, `AutoLayout`,
  `KeywordCapitals` and `KeywordLowercase`. Where a setting is not given, the default is QB64pe's own built-in
  default (indent on at 4, SUBs indented, layout on, keywords in CamelCase). The new compiler reads no
  `config.ini`. As with `qb64pe`, `-o` is required, and a program with an error is reported as a compile
  reports it and writes no output (exit status 1).
- **The layout rules of the old compiler**, reproduced:
  - each statement's spacing, the `sp`/`sp2` decisions of QB64pe's handlers;
  - keyword case from the old table, including `SCase2$`'s exceptions such as `_AndAlso`;
  - names written as first declared;
  - indentation by control depth, with `ELSE`/`CASE` one level out, `TYPE` and `DECLARE LIBRARY` bodies, and
    `IndentSubs`;
  - metacommands and `$IF` blocks;
  - comments after a statement, labels and line numbers;
  - `?` written as `Print`, and a missing closing quote added;
  - left as written: a line joined by `_`, and the lines between `'$FORMAT:OFF` and `'$FORMAT:ON`;
  - with `AutoLayout` off, only the keyword case and indentation applied over the user's own spacing.
- **The oracle**: the old compiler's `-y` output, recorded with the default settings given explicitly, for every
  program of the golden corpus, the upstream copy, the snippets and Civil War Strategy that the old compiler
  accepts, kept in `tests\format\`. The runner records it with `qb64pe` and compares any other compiler, as it
  already does for the verification suite. Tier 1 formats each recorded program in process and compares, and
  checks that formatting does not change the program: the formatted text gives the same typed tree, and
  formatting it again changes nothing. QB64pe's own `format_tests` (24 cases, all six settings) pass with
  `qb64rust` through the runner's `--suite format`.
- **The language server formats**: `textDocument/formatting` and `textDocument/rangeFormatting`. The driver hands
  the formatter to the server as it hands `Check`, so `qb64rust-lsp` still does not depend on `sema`. It formats
  the open text, takes unsaved included files from open documents, and writes no temporary file. A range format
  formats the whole program and returns only the edits to the lines in the range.
- **The extension formats through the server** when the server runs. Otherwise it runs `qb64rust -y`, and runs
  `qb64pe -y` only when `qb64rust.buildWith` is `qb64pe`. New settings `qb64rust.format.autoIndent`,
  `indentSize`, `indentSubs`, `autoLayout` and `keywordCase` (`camel`, `upper`, `lower`) are passed to whichever
  compiler formats, so the result no longer depends on QB64pe's `config.ini`. Their defaults are QB64pe's built-in
  defaults.

Out of this change:
- formatting on type, which the old IDE does as the cursor leaves a line;
- the `?` expansion as the user types;
- the language-server features of `m5b-language-features`;
- recording `-y` over the verification families (about 4,000 programs, 5 MB, mostly the same statement forms
  again). `design.md` says why, and they can be added later.

## Capabilities

### New Capabilities

- `compiler/format`: the `-y` mode of `qb64rust`, covering:
  - the command line and its layout settings, and their defaults;
  - what the output is held to (the old compiler's `-y` output);
  - that formatting never changes the program;
  - the lines left as written;
  - failure on a program with an error.
- `testing/format-oracle`: the recorded `-y` outputs in `tests\format\`, covering:
  - which programs and settings are recorded, how they are recorded and compared;
  - what tier 1 checks (equal to the record, the same typed tree, idempotent);
  - QB64pe's `format_tests` run with the new compiler.

### Modified Capabilities

- `editor/formatting`: formatting is by the new compiler. It goes through the language server when that runs,
  else through `qb64rust -y`, and goes to `qb64pe -y` only when the build is QB64pe's. Range formatting, and
  layout settings that are the extension's own. The purpose no longer says "by delegating to the compiler's
  formatting mode" of QB64pe alone.
- `editor/language-server`: "Server start" advertises formatting and range formatting; new requirements
  "Formatting" and "Range formatting".
- `compiler/cli`: the "qb64pe-compatible compile" settings list stays for builds; `-y` and its layout settings are
  accepted (specified in `compiler/format`), and a layout setting given to a build is ignored as `qb64pe` ignores
  it, not an error.

## Impact

- A new crate `crates\format` (`qb64rust-format`): the layout of a parsed and checked program. It depends on
  `syntax`, `builtins` and `sema` (for the spelling of names), and the `driver` uses it. The crate map in
  `crates\README.md` gets a row and the graph gets an edge.
- `crates\driver`: `-y`, the layout settings, and a format function handed to the server beside
  `check_for_server`.
- `crates\lsp`: the two formatting requests, the capabilities, and a `Format` hook like `Check`.
- `crates\sema`: the declared spelling of each symbol, if the symbol table does not already keep it.
- `tools\legacy_tests\`: a suite that records and compares `-y` output (`tests\format\`), with its tests.
  `--suite format` already runs QB64pe's `format_tests` with any compiler.
- `tests\format\` (new): the recorded outputs and a `README.md`.
- `vscode\`: `format.ts` goes through the server or `qb64rust -y`; `compiler\qb64rust.ts` gets `format`; the new
  settings in `package.json` and `config.ts`; tests; `README.md` and `CHANGELOG.md` (unreleased).
- `DIVERGENCES.md`: a row for each place where the formatter deliberately does not copy `-y`, for example where
  QB64pe's layout changes what a program means, if the oracle finds one.
- No change to code generation or libqb. `UPSTREAM.md` gets a row for any `-y` bug the oracle finds.
