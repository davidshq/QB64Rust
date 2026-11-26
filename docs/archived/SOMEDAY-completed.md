# Someday: the parts that were built

The rows of `docs\SOMEDAY.md`, or the parts of them, that were built, under the headings they stood under there
(`CLAUDE.md` working rule 7). What is still open is in `docs\SOMEDAY.md`.

## VS Code parity left

| Priority | Group | Feature | What was built | Source |
|---|---|---|---|---|
| P3 | f1-help | Contextual F1 help: help text of our own, the hover's part | The hover on a built-in gives, after its arguments, the first sentence of its page of the project's own language reference; a keyword with a page gives that sentence alone (2026-11-26, `DECISIONS.md`). The sentences are a table that `tools\reference\reference.py render` writes from the pages, `crates\lsp\src\help_table.rs`. The rest of the row, help by F1 and text checked against the recordings, stays in `SOMEDAY.md` | `study\archive\06` "Help system"; the panel, 2026-10-29 and 2026-11-08 |
