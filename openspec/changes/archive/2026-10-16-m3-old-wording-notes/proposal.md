# Proposal

## Why

Decided 2026-10-16 (`DECISIONS.md`, item (6)): "its own change after `m3-declare-library`: every diagnostic whose
wording differs from the old compiler's carries the old wording as a note, taken from recorded output". This
compiler's messages are its own ("`_MEM` with empty parentheses"); a user who knows QB64pe, or looks a message up
on its wiki or forum, knows its wording ("Expected (...)"). Built on the user's word after the trade-offs were put
(a fuzzy pairing, a table to regenerate; "continue doing it", 2026-10-16), in the narrow form: a note only where
the recorded pairing is exact.

## What Changes

- **A table from recorded output**, `crates\base\src\old_wording.tsv`, written by
  `tools\diagnostics\old_wording.py`: every program the old compiler rejected with its message recorded
  (`verification\*.compile.txt`, the `.err` files of the corpus and of `tests\upstream`) is compiled by this
  compiler; its first real error on the line the old message names is paired with that message. A pattern (names
  and numbers as placeholders, keywords kept) is kept only when every pair agrees, the wording differs, and the old
  text names no file and holds nothing of one program. 1,336 pairs, 643 patterns, 431 kept.
- **A note after the error**: the CLI prints `<file>:<line>:<column>: note: QB64pe: <wording>` after a real error
  the table holds; the language server adds the line `QB64pe: <wording>` to the diagnostic's message. A "not
  supported yet" error and a warning get none.
- **A check**: `old_wording.py --check` compiles the recorded rejections again and compares every note with the
  recorded message (696 notes, all equal).

Out of scope: notes for errors no recorded program reaches (no guess is made); the wording of messages themselves.

## Capabilities

### Modified Capabilities

- `compiler/cli`: requirement "Diagnostics output" (the note line).
- `editor/language-server`: requirement "Syntax diagnostics" (the note in the message).

## Impact

- `crates\base` (`old_wording.rs`, the table and its word list; `Diagnostic::render`, `old_wording`),
  `crates\lsp\src\analysis.rs`, `tools\diagnostics\`.
- Every `check-fail` snapshot with an error the table holds (95, only note lines added; 7 again when the forms of a name were made exact).
- Documents: `CLAUDE.md`'s layout table (`tools\diagnostics\`), `crates\README.md`, `DECISIONS.md`, `STATUS.md`.
