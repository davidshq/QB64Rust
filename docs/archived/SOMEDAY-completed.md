# Someday: the parts that were built

The rows of `docs\SOMEDAY.md`, or the parts of them, that were built, under the headings they stood under there
(`CLAUDE.md` working rule 7). What is still open is in `docs\SOMEDAY.md`.

## Deferred QB64pe features

Built 2026-11-26 (user: for each form that should be an error, show the correct message). Each was "not supported
yet" and is now a compile error. The programs named pass and are on `tests\verification\pass.list`.

| Priority | Group | Form | What was built | Source |
|---|---|---|---|---|
| P4 | varptr | `VARPTR` of a member of a `TYPE` type | "`VARPTR` of a member whose type is a `TYPE`". QB64pe gives an internal error | `verification\v66_a14` |
| P4 | varptr | `VARPTR$` of a member of an element (`VARPTR$(a(1).name)`) | "`VARPTR$` of a member of an array element: of an element it takes only an array of strings", where QB64pe says "VARPTR$ only accepts variable-length string arrays" | `verification\v66_a15` |
| P5 | varptr, mark-to-error | `_OFFSET` of a variable-length string member of a `TYPE`, or of a whole array of variable-length strings | "`_OFFSET` of a `STRING` member: a variable-length string has no fixed place" and "`_OFFSET` of a whole array of variable-length strings". QB64pe: "UDT must have fixed size", "Cannot pass array of variable-length strings". A member of an element stays marked | `verification\v66_b07`, `b08` |
| P4 | const-eval | A string argument to a color function in a `CONST` (`_RGB32("a")`) | "argument 1 of `_RGB32` needs a number", the error outside a `CONST`. QB64pe reads the string as 0 (`DIVERGENCES-QB64PE.md` D-078) | `verification\v66_d03` |
| P2 | unknown-type | A name that is no type in an `AS` clause: `REDIM` of a member array, `_UNSIGNED` before a name in `SHARED`, a parameter or `_DEFINE` | The errors of a name that is no type in each place, also after `_UNSIGNED`. A member array's `REDIM` is "Undefined type" in QB64pe for a fixed member array too (`v75_a1`, measured for this). `_UNSIGNED` and `* n` together stay marked in these places | `verification\v66_c08`–`c11`, `v75_a1` |
| P2 | fixedstr-dollar | `REDIM`, `STATIC` or `COMMON` of `a$(…)` beside a `STRING * n` array `a` | The error `DIM` gives: "`a$`: an array of variable-length strings beside the `STRING * n` one" (QB64pe: "Type mismatch") | `verification\v66_c12`–`c14` |
| P2 | mark-to-error | `x() = 1`, the whole array of a name that is no array as a place; a SUB written with a type suffix (`s$ 1` beside a `SUB s`); a function of the auto-included files as a place (`_ENCODEURL$(1) = "a"`) | "`x() =`: a store needs an element of the array, with its index"; "`s$` is no statement: the name of a SUB is written without a type suffix"; "`_ENCODEURL$(...) =`: `_ENCODEURL$` is a function of the language, which cannot be stored to". `READ x()` and `CALL s$(1)` stay marked: not measured | `verification\v66_c17`, `c19`, `c20` |
| P2 | mark-to-error | A statement named with one leading underscore that the table does not hold | "`_NOSUCHSTATEMENT` is no statement and no SUB" (QB64pe: "Syntax error"). After `CALL` it stays marked: not measured | `verification\v66_c24` |

## VS Code parity left

| Priority | Group | Feature | What was built | Source |
|---|---|---|---|---|
| P3 | f1-help | Contextual F1 help: help text of our own, the hover's part | The hover on a built-in gives, after its arguments, the first sentence of its page of the project's own language reference; a keyword with a page gives that sentence alone (2026-11-26, `DECISIONS.md`). The sentences are a table that `tools\reference\reference.py render` writes from the pages, `crates\lsp\src\help_table.rs`. The rest of the row, help by F1 and text checked against the recordings, stays in `SOMEDAY.md` | `study\archive\06` "Help system"; the panel, 2026-10-29 and 2026-11-08 |

## Other ideas raised during the study

| Priority | Group | Idea | What was built | Source |
|---|---|---|---|---|
|  |  | **A number as the variable of `FOR`**: `FOR 5 = 1 TO 2: NEXT` gave "internal error: incomplete syntax tree" beside the parser's error | Only "expected a variable after `FOR`" now: a `FOR` header that stops early keeps the rest of the statement inside it, so `sema` skips it (2026-11-26). QB64pe: "Unsupported variable used in FOR statement" (`v75_a2`, measured for this) | `verification\v75_a2`; batch 4 of the language reference (2026-11-11) |
