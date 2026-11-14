# Proposal

## Why

Format Document on an include file fails. On `turn.bm` of Civil War Strategy the editor says "The program was
not formatted: turn.bm, line 6: `CALL clrbot`: no SUB of this name". The language server treats every open file
as a program of its own, and `turn.bm` is a part of `CWSTRAT.BAS`, which defines that SUB. Checked alone,
`turn.bm` gets 108 errors, all of them false, and the check on save shows them. Formatting `CWSTRAT.BAS` does
not help, because it lays out the main file only. So no `.bi` or `.bm` file can be formatted today, and
go to definition, hover and rename in one see nothing defined outside it.

Other tools do not have this problem. Formatters such as clang-format, rustfmt, gofmt and Black work on one
file's syntax and never refuse over a missing name. Language servers for languages with textual includes (clangd,
typst-lsp, LaTeX Workshop) find the file that includes the open one and answer from that program.

This is the first change under the decision of 2026-11-11 (`DECISIONS.md`): compatibility with QB64pe 4.7.0 is
kept, and the tools may do better where QB64pe gives nothing. `qb64pe -y` gives nothing for a program with an
error, and the QB64pe IDE treats an open `.bm` as a program too.

## What Changes

**Part 1: formatting does not need a complete program.**

- Only a syntax error stops formatting. An error of the later stages (a SUB that is not defined, a type error)
  does not. Indentation, spacing and keyword case come from the syntax alone.
- In a program with such an error, and in an include file formatted alone, the letter case of a name changes
  only when a statement declares it (`SUB`, `FUNCTION`, `DIM`, `CONST`, a parameter, a `TYPE`, a label). Every
  other name stays as typed. So formatting `turn.bm` alone never disagrees with formatting it inside
  `CWSTRAT.BAS`; it only does less.
- A program without errors is formatted exactly as today, as `qb64pe -y` formats it.
- `qb64rust -y` follows the same rules: it writes the output file for a program whose only errors are not syntax
  errors. This differs from `qb64pe -y`, which writes nothing (a new `DIVERGENCES-QB64PE.md` row).
- The server no longer shows "The program was not formatted" for such an error. It still does for a syntax
  error.

**Part 2: an include file belongs to its program.**

- For every open file, the server finds the program it belongs to. The extension does not decide it: a `.bas`
  file that another program includes is a part of that program. The order is:
  1. the `main` entry of the `[project]` table in the nearest `qb64rust.toml`;
  2. an open `.bas` document whose program includes the file;
  3. a `.bas` file in the document's folder, or a folder above it, whose program includes the file.
- A `.bas` file that nothing includes is its own program and is checked in full, as today. So is the file that
  `main` names.
- The parser's diagnostics, the check, hover, go to definition, references, rename, completion and formatting
  for that document then come from the program. Formatting it gives the declared letter case of every name.
- The editor shows which program was chosen, in the language status area.
- A `.bi` or `.bm` document with no program found is handled alone. Its check reports the parser's errors only, not the false
  errors of the later stages, and its formatting follows part 1.

**Not in this change** (each a `SOMEDAY.md` row): Build and Run from an include file building its program;
`qb64rust -y` formatting an include file inside a named program; a command to choose among several programs;
modules in place of textual `$INCLUDE`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `compiler/format`: "Program with an error" stops only on a syntax error. A new requirement says which names
  change their letter case in a program with an error or an include file formatted alone.
- `editor/language-server`: "Format failure" covers a syntax error only. New requirements: the program of an
  included file, the answers that come from it, a file with no program, the `main` entry of `qb64rust.toml`, and
  the notification that tells the client which program was chosen.
- `editor/formatting`: a new requirement that an include file can be formatted, with or without its program.

## Impact

- `crates\sema`: the symbol table records which symbols a statement declares.
- `crates\format`: the name's letter case under the new rule.
- `crates\driver`: `-y` and the server's format hook stop only on a syntax error; the format hook takes the file
  to lay out; reading `[project]` `main` from `qb64rust.toml`.
- `crates\lsp`: the lookup of a document's program, programs whose main file is not open, requests answered from
  the program, the new notification.
- `vscode`: a language status item for the program; the `README.md` and `CHANGELOG.md`.
- `docs`: `DIVERGENCES-QB64PE.md` (one row), `MANUAL.md` (the `[project]` table), `SOMEDAY.md` (four rows),
  `DECISIONS.md`.
- `compiler/fixes` is not changed: its settings file already ignores tables other than `[fixes]`.
