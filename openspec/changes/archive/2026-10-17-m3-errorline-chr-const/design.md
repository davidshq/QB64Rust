# Design

## Context

Motivation, scope and what was measured: `proposal.md` (`verification\v43_*`, 2026-10-17). Where this compiler
stood:

- **`sema`** `check\constants.rs` walks a `CONST` value over the old evaluator's arithmetic (`consteval.rs`); a
  call of a function of the evaluator's list (`FUNCTIONS`) was "not supported yet". `check\auto.rs` reads the
  expressions of the auto-included file in three shapes and marked any with a function the same way.
- **Lines**: `Checker::line` gave a span's physical line; a statement's line is its first token's. The lexer
  makes `_` before a line end a `LineContinuation` token (trivia) that eats the line end.
- **Event checks**: the emitter writes `evnt(line)` in the main file and `evnt(line, line, "file")` in an included
  one (`names.rs` `evnt_args`); the old compiler's first argument there is the main file's line, which feeds only
  `_ERRORLINE` in libqb (`error_handle.cpp` `error_set_line`).
- **Includes**: each inclusion is a tree of its own; `ParsedProgram::includes` maps an include statement (tree,
  offset) to the tree it brought in, and `Checker::main_line` already walked that up for `$VERSIONINFO`.

## Goals / Non-Goals

**Goals:** `CHR$` in a `CONST` and in the auto-included constants as the old evaluator computes it; the four
error functions with the old compiler's values in the main file, in included files and on joined lines.

**Non-Goals:** the other functions of the evaluator; numeric line labels; `$ERRORLOCATION` (libqb's other
tracker, which no program here asks for).

## Decisions

### D1. A joined line is numbered as its last line

The checker collects, once, the lines of every file that end with a `LineContinuation` token
(`continued: HashSet<(FileId, u32)>`). `Checker::line` then moves down while the line is in the set. Every use of
`line` follows: statements, block headers and ends, labels, procedure lines, the message that names a later
`CONST`'s line. All of them are the old compiler's `linenumber`, which counts a joined line once, at its last
line (measured, `v43_d`, `v43_e`). The `#line` directives follow too (they only map C++ messages).

*Alternative considered*: numbering only the event checks so. Rejected: two numberings of one statement would
disagree in the dumps and in the line-keyed checks of `flow.rs`, for no gain.

### D2. The main file's line of an included statement travels beside its line

`sema::Stmt` and `sema::Label` get `include_line: Option<u32>`: `None` in the main file, else `main_line` of the
node (the line of the outermost `$INCLUDE`, by D1's numbering). `ir::Stmt` and `ir::Label` get the same field; the
lowerer keeps the value of the `sema` statement it lowers and gives it to every IR statement and label it makes
from it, a block's included (a block cannot cross a file: a `$IF` or block closed in another file is "not
supported yet"). `evnt_args(file, line, include_line)` writes `evnt(main, line, "file")`. The IR dump shows
`in main line N` after the line.

*Alternative considered*: a table from (file, line) to the main line. Rejected: a file included twice from two
lines has two main lines for the same (file, line).

### D3. `CHR$` in `consteval`

`consteval::chr(code)`: a string is an error; an integer must be 0–255; a float within -1 to 256 is rounded
half to even by the evaluator's existing `round_to_int` (which is "not supported yet" for an inexact value next
to a half), then must be 0–255; any other float is an error (a NaN "not supported yet"). In `constants.rs` a call
of `CHR$` with exactly one argument computes the argument, reads it back as a parenthesised group (the old
evaluator turns each argument into a single element), then calls `chr`; any other number of arguments is an
error, and `CHR$` without parentheses is one too. `auto.rs` reads a term as an operand or a call of one operand,
and computes `CHR$` with the same `chr`; `EXP` stays "not supported yet".

### D4. A built-in function's name in a `CONST` is an error

A bare name in a `CONST` that is no constant, no variable and no function of the evaluator's list was "not
supported yet" (it might be a constant this compiler does not know). A name of a built-in function cannot be a
constant (it is reserved), so it is an error (`v43_x40`: `CONST c = _ERRORLINE`).

### D5. Tests

- Slice programs: `s135_chr_const` (`v43_c` and the auto-included constants), `s136_errorline` (`v43_a` without
  its numeric line labels and its fatal `ERROR 300`, with `v43_d` and `v43_x44`), `s137_errorline_include`
  (`v43_b`; its included files in `s137_errorline_include.files\inc\`). Tier 1 looks a corpus program's includes
  up in its `.files` folder, where the runner puts them beside it (`crates\driver\tests\inputs.rs`
  `beside_files`).
- Front end: `const_chr_errors` and `errorline_errors` (`check-fail`, one form of `v43_x*` each),
  `errorline_typed`, `errorline_cpp` (the event checks of a joined line, an included file, a nested one and a
  label in it). Three tests used `CONST c$ = CHR$(65)` as their "not supported yet" declaration; they use
  `CONST c = EXP(1)`, and `mem_follow_on` `_E` for `_CHR_CR`.
- Call-site programs: `func_errorline`, `func_inclerrorline`, `func_inclerrorfile`, `errormessage_bare`,
  `_long`, `_double`, `_literal`.

## Risks / Trade-offs

- D1 changes the line of every statement on a joined line, in every program. Only programs with a continuation
  change; the slice list, the upstream pass list and Civil War Strategy run at the close show whether any
  recorded output named such a line.
- `v43_x46` (a FUNCTION in a single-line `IF` continued over two lines) works in both compilers; it is kept as a
  record of the form.
