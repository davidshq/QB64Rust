# Proposal

## Why

The parser reads a small subset of QB64: of the 1,280 BASIC files tier 1 sees (corpus, upstream, snippets, and
from the clone `qbasic_testcases` and the old compiler's sources), only 491 parse without an `Error` node
(measured 2026-10-04 with `--dump tree`). The most common gaps are `END …` (437 files), `IF` (436), `FOR`/`NEXT`
(248), `TYPE` (239), array `DIM` (221), `REDIM`, `DO`/`LOOP`, `SELECT CASE`, the graphics and file statements, and
`DATA`. 181 programs the old compiler accepts still get a real error (`tests\known_false_errors.list`); most come
from member access after an index (`a(0).x`, 114 of the generic parse errors) and from follow-on errors after an
unsupported declaration (`TYPE`, `CONST`, `DEFSTR`, fixed-length strings: 24 "cannot store a string in a number
variable").

The tier-2 run also found **wrong code**: a comment metacommand (`'$INCLUDE: 'x.bm'`) is ignored as a comment,
so 4 upstream programs build and print the wrong output.

This is step 2 of the order of work (`study\22` §5, `STATUS.md`): parse the whole language, so that the
language server (step 3) is useful on real files, and remove the single-file assumption in `sema` before more
code depends on it.

## What Changes

- **Wrong code fixed first:** a comment metacommand (`'$INCLUDE`, `REM $DYNAMIC`, ..., by the old compiler's rule)
  gets a "not supported yet" error until it is implemented in this change.
- **Panic hook** in the driver: a panic prints "internal compiler error" with the location and exits with a
  distinct code (`study\21` item 6).
- **Tier 2 `.err` meaning (decided 2026-10-04):** for a compiler other than `qb64pe`, an `.err` program passes
  when the compile fails, writes no executable and reports at least one error not marked "not supported yet";
  the old message text is not compared. The 56 upstream `.err` programs stay in the 279.
- **Every statement of the language parses** into its own node: blocks (`IF`, `FOR`, `DO`, `WHILE`, `SELECT
  CASE`, `TYPE`, `DECLARE LIBRARY`, `DEF FN`), line numbers and numeric labels, `GOTO`/`GOSUB`/`RETURN`/`ON …`,
  declarations (`DIM` with arrays and every `AS` form, `REDIM`, `CONST`, `DEFxxx`, `_DEFINE`, `COMMON`,
  `OPTION`, `ERASE`), `DATA`/`READ`/`RESTORE`, the statements with their own syntax (`PRINT USING`, `PRINT #`,
  `INPUT`, `LINE INPUT`, `WRITE`, `OPEN`, `CLOSE`, `GET`/`PUT`, `LSET`, `SWAP`, `MID$ =`, ...) and the 98
  built-in statements whose syntax is a `specialformat` template (`LINE (a, b)-(c, d), , BF`).
- **Expressions:** member access after an index (`a(1).b.c(2)`), omitted arguments (`f(a, , b)`), and the
  remaining word forms.
- **Metacommands:** `$IF`/`$ELSEIF`/`$ELSE`/`$END IF` with `$LET` and the predefined names, evaluated while
  parsing; `$INCLUDE` and `'$INCLUDE` with **one tree per file** and `$INCLUDEONCE`; every other metacommand
  parsed into name and argument.
- **`sema`**: keyed by (`FileId`, offset) and reading text through the `SourceMap`; every new node kind it does
  not compile yet gets one "not supported yet" error at its statement (the same verdict as today, one stage
  later); **no follow-on errors**: a name or letter range whose declaration was not supported makes later errors
  that depend on it "not supported yet" too.
- **A third shrink-only list**, `tests\known_parse_gaps.list`: accepted programs whose parse still reports a
  diagnostic. Done when it and `tests\known_false_errors.list` are empty.

## Capabilities

### New Capabilities
None.

### Modified Capabilities
- `compiler/pipeline`: the parser covers the whole language; block structure and its recovery; comment
  metacommands are never ignored; the preprocessor and included files; no follow-on errors after an
  unsupported construct; the lossless tree holds for every file of a program.
- `compiler/cli`: an internal compiler error is reported as such, with its own exit code.
- `testing/upstream-tests`: the parse-gap list; the meaning of an `.err` program in tier 2 for the new compiler.

## Impact

- `crates\syntax` (lexer, many new node kinds, parser modules per statement family, `ast.rs` accessors,
  preprocessor), `crates\builtins` (the `specialformat` templates parsed at build time into grammar values; new
  dependency `syntax -> builtins`), `crates\sema` (`FileId` keys, `SourceMap`, marking of new nodes, follow-on
  suppression), `crates\driver` (include loading, panic hook), `crates\driver\tests\inputs.rs` (third list),
  `tools\legacy_tests\run_legacy_tests.py` (`.err` meaning), `tests\frontend\` (new `parse-ok` and `check-fail`
  tests), `verification\v16_*` (measurements), `crates\README.md`, `tests\upstream\README.md`, `STATUS.md`.
- Generated code changes only where a program now compiles that did not (included files, `$IF`); no supported
  construct changes its output.
- Tier-1 time: more parsing per file, same files; the one-minute budget of `crates\README.md` still applies.
