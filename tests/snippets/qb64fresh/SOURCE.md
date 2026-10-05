# Source of `tests/snippets/qb64fresh/`

The BASIC snippets of QB64Fresh's inline tests (`tests/integration_tests.rs` of QB64Fresh, the user's own MIT
code; commit `56ea996c66`, 2026-02-04). Only the BASIC text is taken, never the assertions (`CLAUDE.md` rule 5): each
snippet's verdict is that of `qb64pe.exe` (QB64pe at the commit in `tests/upstream/SOURCE.md`). A snippet it
rejects has an `.err` file with its output; one it accepts has none. Nothing is run.

Made by `tools/snippets/extract_qb64fresh_snippets.py` (rules in its header). Do not edit the files by hand.

| | Snippets |
|---|---|
| Raw strings with BASIC text (assigned to `source` or passed to `compile_to_c`) | 418 |
| After dropping duplicates | 406 |
| Rejected by `qb64pe.exe` (`.err`) | 64 |

The 31 other raw strings in the file are C text in assertions about QB64Fresh's generated code and are not taken.
