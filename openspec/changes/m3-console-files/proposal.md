# Proposal

## Why

This is step 4 of M3's order of work (`DECISIONS.md` 2026-10-12): the console programs of the corpus that still
wait. Twelve programs of `tests\corpus` are rejected with "not supported yet" and nothing else
(`qb64rust --report unsupported tests\corpus`, 2026-10-13):

| Programs | Uses | Not supported yet | Corpus program |
|---|---|---|---|
| 2 | 2 | `GET` on a file | `175_get_put`, `177_open_binary` |
| 1 | 1 | `PUT` on a file | `175_get_put` |
| 1 | 5 | `_IIF` | `243_iif` |
| 1 | 4 | `PRINT USING` | `186_print_using` |
| 1 | 3 | `_CLAMP` | `108_clamp` |
| 1 | 3 | `_MIN`, `_MAX` | `78_min_max` |
| 1 | 4 | `_SHL`, `_SHR` | `159_bitwise_shl_shr` |
| 1 | 4 | `_ROL`, `_ROR` | `160_bitwise_rol_ror` |
| 1 | 2 | `LOCK`, `UNLOCK` | `179_file_lock` |
| 1 | 1 | `EXIT SELECT` | `151_exit_select` |
| 1 | 1 | `CHAIN` | `238_chain` |
| 1 | 1 | `LPRINT` | `239_lprint` |

After this change no program of the corpus that the old compiler accepts is held back.

Measured before this proposal (`verification\v30_*`, 100 programs and the table of result types; the facts are in
the `v30` paragraph of `study\00` §5):

- **Everything that happens when these programs run is libqb's**, which both compilers link. The new compiler has
  to write the same calls; what a format string prints, what a record holds and which error a file raises then
  follow.
- **`PRINT USING`** is a statement of its own shape: the format is copied once, then one call per value, chosen
  by the value's type, with a jump out after each when an error is pending.
- **`GET` and `PUT` on a file** pass a variable's bytes (its place and size, as graphics `GET` does), a
  variable-length string through entry points of its own, and with no variable they read and write the buffer
  that `FIELD` names.
- **`_IIF`** evaluates only the part chosen. **`_IIF`, `_MIN`, `_MAX` and `_CLAMP` share one rule for the type of
  their result, and it is not the rule of the arithmetic operators** (a SINGLE beside an `_INTEGER64` gives
  SINGLE; of two integers of one width the unsigned one wins).
- **`_ROL` and `_ROR`** turn within the width of their first argument's type; `_SHL` and `_SHR` are plain calls.
- **`EXIT SELECT`** is a jump to the end of the innermost `SELECT` that has a `CASE` open.
- **`CHAIN` and `LPRINT` cannot be compared at run time**: `CHAIN` starts `qb64pe.exe -c` beside the program
  through `cmd`, and `LPRINT` prints a page.

Two things were the user's to decide (2026-10-13, `DECISIONS.md`):

1. **`CHAIN` and `LPRINT` are compiled**, checked by the call-site check; they are never run by a test.
2. **`FIELD`, `LSET` and `RSET` belong to this step.** No corpus program names them, but `GET` and `PUT` with no
   variable are useful only with them. They are not measured yet: that is this change's first task group.

## What Changes

- **`PRINT USING`** to the console and to a file, in every form the old compiler accepts: any string expression
  as the format, values of every numeric type and strings, `TAB` and `SPC` among the values, items before
  `USING`, a trailing `;` or `,`. Its rejected forms are rejected.
- **`GET` and `PUT` on a file**, `FOR RANDOM` and `FOR BINARY`: with a numeric variable, a fixed-length string, a
  variable-length string, a `TYPE` variable or member, an array element, a whole array; with and without a
  position; with no variable. The name's graphics form stays as it is.
- **`FIELD`, `LSET`, `RSET`**: the record buffer of a random file named by string variables, and the two
  statements that fill such a variable without changing its length.
- **`LOCK` and `UNLOCK`** as statement rows, with a record, a range or neither.
- **`EXIT SELECT`**, and `EXIT CASE` in `SELECT EVERYCASE` and in a plain `SELECT`.
- **`_IIF`, `_MIN`, `_MAX`, `_CLAMP`** with the old compiler's result type for every pair of numeric types;
  **`_SHL`, `_SHR`, `_ROL`, `_ROR`**. None of the eight in a `CONST` expression, as in the old compiler.
- **`CHAIN` and `LPRINT`** (with `USING` too), compiled and never run: each joins the list of built-ins no test
  runs, with a call-site program per form.
- **Tests and lists**: `verification\v30_*` (and the new programs for `FIELD`, `LSET`, `RSET`); a call-site
  program per plain form; front-end tests; slice programs recorded with `qb64pe.exe`; the twelve corpus programs
  on `tests\corpus\slice.list` where they can be compared; the shrink-only lists regenerated.

## Capabilities

### New Capabilities

- `language/print-using`: `PRINT USING` and `LPRINT USING`: the forms that compile, which runtime entry each
  value goes to, what is printed when a value raises an error, as the old compiler's program behaves.
- `language/record-files`: files read and written by position: `GET` and `PUT` on a random or binary file,
  `FIELD`, `LSET`, `RSET`, `LOCK` and `UNLOCK`: the forms that compile, what a variable of each kind reads and
  writes, the position, the run-time errors.

### Modified Capabilities

- `language/builtin-functions`: `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_SHL`, `_SHR`, `_ROL`, `_ROR` join the list;
  new requirements for the choice that evaluates one part, the result type of the four, the shifts and
  rotations, and the eight in a `CONST`.
- `language/builtin-statements`: `GET` and `PUT` on a file, `FIELD`, `LSET`, `RSET`, `LOCK`, `UNLOCK`, `PRINT
  USING`, `LPRINT` and `CHAIN` join the list (the scenarios of an unsupported statement and of the file form of
  `PUT` change); a requirement for `CHAIN` and `LPRINT`.
- `language/control-flow`: `EXIT SELECT` and `EXIT CASE`.
- `testing/compiler-tests`: `CHAIN` and `LPRINT` among the built-ins that no test runs.

## Impact

- Changed: `crates\sema` (`builtins.rs`: rows and new rules; `check\io.rs`: `PRINT USING`, `LPRINT`;
  `check\builtins.rs`: the file form of `GET` and `PUT`; `check\proc.rs` and `check\blocks.rs`: `EXIT SELECT`;
  `check\mod.rs`: `FIELD`, `LSET`, `RSET`, `LPRINT` leave the "not supported yet" branch; `check\constants.rs`:
  the eight functions in a `CONST`), `crates\ir` (the new statements and the lazy choice; `validate.rs`,
  `dump.rs`), `crates\codegen-cpp` (`io.rs`, `builtins.rs`, `stmt.rs`), `crates\syntax` only where a measured
  form does not parse today, `crates\driver\tests` (`cli.rs`: the test that uses `LPRINT` as a statement not
  compiled yet takes another), `tests\corpus\slice.list`, the shrink-only lists; new `tests\callsite\*.bas`,
  `tests\frontend\*.bas`, `tests\corpus\slice\*`, `verification\v30_*`.
- No change to libqb or the reference clone, to the build, or to `tools\`.
- Not in this change: `MKI$`, `CVI` and their family (no corpus program names them; they come by demand);
  `OPEN … FOR RANDOM` itself (compiled since `m2-builtin-statements`); `_MEM`; `VARPTR$`; `RUN` and `STOP`;
  `COMMON` across a `CHAIN` (QB64pe passes nothing more than libqb's `sub_chain` does); what `LPRINT` puts on a
  page; the image built-ins.
- `STATUS.md`, `ROADMAP.md`, `crates\README.md`, `tests\corpus\README.md`, `study\00` §5, `SOMEDAY.md` (what
  stays marked; questionable behaviours), `DIVERGENCES-QB45.md` where a kept behaviour differs from QuickBASIC
  4.5, and `CLAUDE.md`'s layout table updated when done.
