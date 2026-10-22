# Proposal

## Why

M4's exit criterion is met in its words: the corpus and the upstream tests in reach match (upstream 401 of 404, the
three left out of scope), and `qb64pe.bas` compiles with no diagnostic since `m4-qb64pe-builtins`. What
`ROADMAP.md` still lists open for M4 is the rest of the language the old compiler accepts. The user asked for a
proposal for it (2026-10-22) and chose a plan of steps with this change as its first (design, "The steps of
the rest of M4"); `SUB _GL` is a step of its own, later.

The measure of what is left is `tests\verification\known.list`: 140 programs marked "not supported yet". The
demand report over them (`qb64rust --report unsupported`, 2026-10-22) puts three forms first:

- **Line numbers**: 21 programs, 65 uses. A number at the start of a line is parsed (`ast::LineNumber`,
  `ImplicitGoto`), and every use of it is marked: as a label, after `GOTO`, `GOSUB`, `RETURN`, `RESTORE`,
  `RESUME`, `ON ERROR GOTO`, `ON … GOTO/GOSUB`, an event's `GOSUB`, and `IF c THEN 10`. `ERL` is always 0. Line
  numbers are the oldest form of BASIC there is, and a real program from the 1980s has them on every line.
- **Unsigned literals**: 9 programs, 14 uses. An integer written without a suffix above 9223372036854775807
  (`18446744073709551615`) is marked; the same number with `~&&` compiles. The old compiler gives such a literal
  the suffix `~&&` (`qb64pe.bas` 23937).
- **A whole array as a value**: 14 programs. `PRINT a()`, `PRINT USING "##"; a()`, `_IIF(0, a(), b())`, `READ
  a()`, `INPUT #1, a()`, `SWAP x(), y()`, `FIELD #1, 4 AS e()`, `LSET e() = "x"` and `b%() = a%()` (suffixes on
  both sides) compile in the old compiler and use one element of the array. The user left these for later on
  2026-10-15 and 2026-10-20 (`DECISIONS.md`) and now rules that they are built as QB64pe has them (user,
  2026-10-22), each a row in `SOMEDAY.md`'s "QB64pe behaviours to review".

## What Changes

- **Line numbers are labels**, as in the old compiler: a number at the start of a line (with a decimal point, a
  type suffix, glued to the statement, beyond the LONG range) names the line, in the body it stands in, and is a
  target wherever a label is: `GOTO`, `GOSUB`, `RETURN n`, `RESTORE n`, `RESUME n`, `ON ERROR GOTO n`,
  `ON ERROR GOTO _NEWHANDLER n`, `ON n GOTO/GOSUB`, an event's `GOSUB n`, `IF c THEN n [ELSE m]` and
  `IF c GOTO n`. The old compiler's errors are kept: "Duplicate label", "Label '99' not defined".
- **`ERL`** gives the number of the last numbered line passed before the error, as libqb's `last_line` does,
  and no longer only 0.
- **An integer literal without a suffix above the `_INTEGER64` range** up to 18446744073709551615 is
  `_UNSIGNED _INTEGER64`, as if written with `~&&`. A larger one stays "Overflow".
- **A whole array `a()` as a value** compiles where the old compiler takes it, and is the element it takes
  (measured first, design D4): in any expression (`v54` found the old compiler takes it as an operand, a
  built-in's argument and a condition too), in `READ`, `INPUT`, `INPUT #`, `LINE INPUT #`, `SWAP`, `FIELD`,
  `LSET`/`RSET`, and the whole-array assignment with a type suffix on both names. Where the old compiler rejects
  one (`x = a()`, `a() = b() + 1`), the error stays.
- Every program of `tests\verification\known.list` whose first mark is one of these, and that the new compiler
  then passes, moves to `pass.list`; the `SOMEDAY.md` rows of the three forms are closed.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/control-flow`: a requirement for line numbers (where they stand, what they name, their errors), and
  "ON GOTO and ON GOSUB" takes line numbers as targets.
- `language/error-handling`: "ERROR, ERR and ERL": `ERL` gives the last line number; `ON ERROR GOTO n` and
  `RESUME n` take line numbers.
- `language/data-read`: "RESTORE" takes a line number.
- `language/events`: "Events the old compiler does not have": a line number as a handler compiles.
- `language/numeric-semantics`: "Integer literal typing": an unsuffixed literal above the `_INTEGER64` range is
  `_UNSIGNED _INTEGER64`.
- `language/arrays-and-types`: a requirement for a whole array used as a value, and "Whole-array assignment
  checks": suffixes on both names compile.

## Impact

- `crates\sema`: `check\flow.rs` (labels, jump targets, `RESTORE`, `RESUME`), `check\events.rs`,
  `check\mod.rs` (the statement dispatch), `literal.rs`, `check\places.rs` and `check\whole.rs` (whole arrays),
  the statements that take a place (`READ`, `INPUT #`, `SWAP`, `FIELD`, `LSET`).
- `crates\ir` and `crates\codegen-cpp`: a label that is a line number writes `last_line=n;` after it; the
  element a whole array stands for.
- `crates\lsp`: go-to-definition already finds line numbers (`definition.rs`); nothing to change but its tests.
- Tests: a new verification family `v54` (design D5), front-end tests, slice programs where no verification
  program runs the form; `tests\verification\known.list` shrinks by about 44 programs.
- `SOMEDAY.md` (rows closed, rows added for the kept quirks), `DECISIONS.md` (the ruling and the plan).
