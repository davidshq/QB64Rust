# Upstream tests

`compile_tests/` is a copy of the text files of QB64pe's `tests/compile_tests` (provenance and licence:
`SOURCE.md`, `LICENSE-QB64pe.txt`; made by `tools/upstream/copy_upstream_tests.py`, never edited by hand).

## Tier 1

`crates/driver/tests/inputs.rs` puts every file of the copy through the new compiler's front end on every
`cargo test`, together with the corpus, the snippets and (when the reference clone is present) QB64pe's
`qbasic_testcases` and its compiler sources: no panic, exact round trip, and the two shrink-only lists
`tests/known_false_errors.list` and `tests/known_unsupported_rejections.list`, and the third list below (spec
`testing/upstream-tests`).

Baseline, 2026-10-04 (after the "not supported yet" marker, before parser breadth):

| Set | Files | Programs accepted by `qb64pe` with a real error (`known_false_errors.list`) | Programs rejected by `qb64pe` with only marked errors (`known_unsupported_rejections.list`) |
|---|---|---|---|
| `corpus/` | 275 `.bas` (20 rejected) | 5 | 7 |
| `upstream/` | 404 `.bas` (56 rejected), 12 includes | 158 | 54 |
| `snippets/` | 406 `.bas` (64 rejected) | 9 | 21 |
| `qbasic/` (clone) | 143 `.bas` | 9 | – |
| `qb64pe-source/` (clone) | `qb64pe.bas` and 50 includes | 0 (stops at 100 errors, all marked) | – |
| **All** | | **181** | **82** |

Every file goes through without a panic and round-trips exactly. The most frequent first false errors are
generic parse errors (`expected ...`, 114 of 181) and type errors from constructs read the wrong way (24 "cannot
store a string in a number variable"). 22 are "names starting with `_` are reserved" for names the compiler does
not know yet: the constants of QB64pe's auto-included BASIC files (`_TRUE`, `_FALSE`, `_E`, `_LOG_TRACE`; the
known gap of `tools\builtins\builtins.json`), `SUB _GL`, and `_CLIPBOARD$ = ...`; they should become "not
supported yet". Parser breadth works the list down.

**Third list, `tests/known_parse_gaps.list`** (2026-10-04, change `m2-parser-breadth` D1): programs the old
compiler accepts, and every file of its own sources, whose parse reports a diagnostic or leaves an `Error` node;
the comment is the first parser diagnostic. Parser breadth is done when this list and `known_false_errors.list`
are empty. Baseline: **673 of 1,139 files** (466 parse cleanly).

| Set | Files checked | In `known_parse_gaps.list` |
|---|---|---|
| `corpus/` | 255 accepted `.bas` | 109 |
| `upstream/` | 348 accepted `.bas` | 268 |
| `snippets/` | 342 accepted `.bas` | 111 |
| `qbasic/` (clone) | 143 `.bas` | 141 |
| `qb64pe-source/` (clone) | `qb64pe.bas` and 50 includes | 44 |
| **All** | **1,139** | **673** |

(The design's estimate of about 790 counted 1,280 files, include files of the other sets among them; the list
covers what the spec names.)

Progress (number of entries per set):

| Date, step | `corpus/` | `upstream/` | `snippets/` | `qbasic/` | `qb64pe-source/` | Parse gaps | False errors | Only marked |
|---|---|---|---|---|---|---|---|---|
| 2026-10-04 baseline | 109 | 268 | 111 | 141 | 44 | 673 | 181 | 82 |
| 2026-10-05 group 5 (member access, `DATA`, line numbers) | 94 | 267 | 104 | 141 | 44 | 650 | 66 | 84 |
| 2026-10-05 group 6 (blocks) | 47 | 236 | 68 | 137 | 39 | 527 | 59 | 81 |
| 2026-10-06 `CONST` and `OPTION` parsed (`m2-control-flow-slice` 4.1) | 42 | 224 | 62 | 137 | 35 | 500 | 59 | 77 |
| 2026-10-07 `m2-control-flow-slice` done (constants, `OPTION _EXPLICIT`, blocks in `sema`) | 42 | 224 | 62 | 137 | 35 | 500 | 43 | 72 |
| 2026-10-07 `m2-arrays-and-types` done (static arrays, `TYPE`) | – | – | – | – | – | 461 | 43 | 71 |
| 2026-10-07 `m2-parser-breadth` groups 7 and 8 (statements, templates, `$IF`, `$INCLUDE`, follow-on rule) | 0 | 0 | 0 | 0 | 0 | **0** | **0** | 66 |

The set columns count parse gaps. After group 6 no first gap is a block any more; the most frequent first gaps are
arrays (93), `REDIM` (144), `SCREEN` (47), `DEFINT` and the other `DEFxxx` (65), `CONST` (41) and statements whose
arguments the parser cannot read yet (`OPEN … FOR`, `LINE INPUT`, `NAME … AS`; 37 `Error` nodes).

After groups 7 and 8 (2026-10-07) **both `known_parse_gaps.list` and `known_false_errors.list` are empty**: every
program the old compiler accepts parses without a diagnostic and gets no real error, with `qb64pe.bas` read
through all its included files and every clone set read. Included files are followed now: tier 1 uses
`tests/upstream/root` as the compiler root for the repo's sets (it mirrors `compile_tests/extra`, which two tests
include relative to the compiler's folder), and the `upstream/qb64pe/` programs, which include the old compiler's
sources by a relative path, count as a clone-dependent set. `known_unsupported_rejections.list` went from 71 to
66: six programs now get a real error for the old compiler's reason, and one joined
(`snippets/qb64fresh/constants__builtin_chr_str_constants_registered`: its real error, `_STR_CR`, follows a name of
the auto-included files on the same line, which is now marked; named in task 8.2).

After a change, regenerate the three lists with `QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test inputs`
and review the diff: entries may only go away (a new entry is a regression to fix, unless it is a program new to
the inputs).

**Fourth list, `tests/known_clean_not_passing.list`** (2026-10-07, `study\26` §3; spec `testing/compiler-tests`,
"Clean programs are listed"): every corpus or upstream program the old compiler accepts that the new compiler
compiles without an error must be on `tests/corpus/slice.list`, on `pass.list` or here, with the reason it does not
pass tier 2. It is kept by hand (not regenerated) and shrink-only; today it holds the five corpus `KFAIL` programs.
A program that becomes clean fails tier 1 until it has been run in tier 2 and listed.

## Tier 2 and upstream progress

```
cargo build --release
python tools\legacy_tests\run_legacy_tests.py --suite compile --qb64 target\release\qb64rust.exe --list tests\upstream\pass.list
```

runs the programs of `pass.list` end to end from the clone's `tests\compile_tests` (with its binary assets),
with the suite's settings (`-f:OptimizeCppProgram=true -f:StripDebugSymbols=false -q -m -x`); CI runs the same
list from this copy against the QB64pe release (`rust.yml`, job `tier2`).

- `pass.list`: the upstream programs the new compiler passes. A ratchet: add a program when it passes, never
  remove one.
- `deferred.list`: the 125 programs that need a feature deferred to `SOMEDAY.md` (`$UNSTABLE:TYPEFIELDS` 109,
  `REDIM _RETAIN` 28, whole-array assignment 21, `_ARRAYCOPY` 4; a program may use several), found by pattern and
  checked by eye.

Progress is the pass list out of the programs not deferred; `cargo test` checks that no program is in both lists
and prints the line (`cargo test -p qb64rust-driver --test inputs upstream_progress -- --nocapture`).

**Upstream progress, 2026-10-04: 10 of 279.** The first full run of the suite with `qb64rust` (404 programs, not a
pass criterion, 29 s of test time, since most programs stop in the front end; the clone unchanged): 10 pass; 4 build but print the wrong output
(they use `'$INCLUDE`, which the front end ignores as a comment: to be fixed, see `STATUS.md`); the 56 `.err`
programs fail on the message text, since the runner compares it with the old compiler's; the rest are rejected
with a diagnostic, 1 known failure (`http/read_example`).

**`.err` programs, 2026-10-04 (`m2-parser-breadth` tasks 2.1, 2.3).** For a compiler other than `qb64pe` the
runner now passes an `.err` program when the compile fails, writes no executable and the summary line counts at
least one error not marked "not supported yet"; the message text is not compared. Of the 56: 54 fail with only
marked errors (the same 54 as the `upstream/` entries of `known_unsupported_rejections.list`), 2 pass
(`arrays/42_` and `43_err_solved_bug_Incorrect_nr_of_args`) but are **not added to `pass.list`**: their only real
error is the parse gap `expected , or )` at `f(10).a(5)`, not the old compiler's reason (wrong argument count to
`UBOUND`/`LBOUND`), so they would leave the ratchet once member access parses. **Progress stays 10 of 279.**

**Upstream progress, 2026-10-06: 11 of 279** (`m2-control-flow-slice` tasks 3.1, 3.2, the operators). Full run:
11 pass (`noprompt/noprompt-continue-fatal`, a fatal `1 \ 0`, added to `pass.list`); none builds and prints the
wrong output; the 56 `.err` programs all get only "not supported yet" errors; the rest are rejected with a
diagnostic, 1 known failure.

**Upstream progress, 2026-10-06: 15 of 279** (`m2-control-flow-slice` task 4.1, the `CONST` parser). Four
`const/` `.err` programs now get a real parse error for the old compiler's reason (`1 ASDF 2`, `NOT` or `OR`
without an operand) and pass; added to `pass.list`. The rest of `const/` waits for the evaluator (task 4.2).
`ROOT`, an operator of the old constant evaluator only, is "not supported yet" inside a `CONST` value.

**Upstream progress, 2026-10-06: 20 of 279** (`m2-control-flow-slice` task 4.2, the constant evaluator). Full run:
20 pass; added to `pass.list`: `const/comma` and `const/const_sub` (output), `const/not_string` and the two
`const/type_mismatch_string_*` (`.err`, now a real error for the old compiler's reason); none builds and prints
the wrong output. `const/undefined_argument*` stay "not supported yet": a name that is neither a constant nor a
variable may be a constant of an auto-included file (`$COLOR`), which the compiler does not have yet.

**Upstream progress, 2026-10-07: 23 of 279** (`m2-control-flow-slice` task 8.1, the blocks through to C++). Full
run (404 programs, 41 s): 23 pass; added to `pass.list`: `source_ordering/goto_gosub` and the two
`arrays/t659_*` (`CALL` and a `SUB` call with comparisons as arguments, `IF` in the `SUB`; both `.output`
programs); none builds and prints the wrong output; 1 known failure. Over the change 13 programs joined, against
the 12 its proposal expected. No program outside `deferred.list` is blocked only by a construct of the change any
more; the most frequent single blockers are `VAL` (5), a malformed whole-array assignment (4 `.err` programs,
`arrays/t025`, `t027`, `t037`, `t040`; the rest of `t024`–`t041` is in `deferred.list`), comment `$INCLUDE` (4)
and `REDIM` (3).

**Upstream progress, 2026-10-07: 24 of 279** (`m2-arrays-and-types`, static arrays and `TYPE`). Full run (404
programs, 36 s): 24 pass; added to `pass.list`: `arrays/t659_array_assignment_control`; none builds and prints
the wrong output; 1 known failure. Of the 11 programs outside `deferred.list` that got only array or `TYPE`
diagnostics before the change, the other 10 need array parameters (`array_arg_dimensions_transitive`,
`comma_array_parameter`), whole arrays `x()` (6 `.err` programs, `t025`–`t046`, whose old message is about the
whole array; they stay "not supported yet" until whole arrays exist) or `_MEM` (`types/dim_array`,
`types/static_array`). The suite's array programs mostly need what the slice left out (`REDIM`, dynamic arrays,
array parameters).

**Upstream progress, 2026-10-07: 34 of 279** (`m2-parser-breadth` groups 7 and 8). Full run with the release
build against the QB64pe 4.7.0 release (404 programs, 48 s): 34 pass; added to `pass.list`: the four
`include_once`/`include_paths` programs (comment `$INCLUDE` followed, `--include-root tests/upstream/root`), the
two `declare_library_external` programs (their `DECLARE LIBRARY` is in a `$IF` branch not taken on Windows; also
found by `study\26` §3) and the four `types/*_no_length` `.err` programs (now the old compiler's reason, no size
after `*`). None builds and prints the wrong output: an earlier run of the change found
`precomp-flags/consoleonly` doing so (`$IF _CONSOLE_` false), now marked "not supported yet" (`study\26` §2).
`auto_include/no-debug` passed by luck before the fix (its `_DEBUG_` is 0) and is marked now. 1 known failure.
The tier-2 runner passes `--include-root tests/upstream/root` to the new compiler on its own.

**Upstream progress, 2026-10-08: 42 of 279** (`m2-core-builtins`: 43 built-in functions, `SELECT CASE`, `ON …
GOTO/GOSUB`). Full run with the release build against the reference clone (404 programs, 64 s): 44 pass; none
builds and prints the wrong output; 1 known failure. Added to `pass.list` (42 pass there against the QB64pe 4.7.0
release, as the CI job runs them): `basic/val_default_regression`, `val_typed_floating_precision`,
`val_typed_nondecimal_precision`, `val_typed_tostr_overload`, `console_only/print_console_null_char`,
`func_tostr/mmLiteral`, `func_tostr/mmVariable`, `print/auto_semicolon_insertion`. The other two that pass against
the clone, `basic/val_default_large_integer_decimal` and `val_typed_large_integer_decimal`, expect the `VAL` of
libqb after 4.7.0: against the release they print other digits, with the release's own `qb64pe.exe` too, so they
are in `tests\known_clean_not_passing.list` until CI moves to a newer release. The proposal counted 11 upstream
programs carrying only marks the change removes; 10 compile and pass against the clone.

**Upstream progress, 2026-10-09: 43 of 279** (`m2-numeric-types` task group 4: declarations and literals of the
new numeric types). `const/expression` now compiles cleanly (its `CONST const__unsignedint = 2~&& * 5~&&` takes the
held values of `~&&` literals) and passes against the reference clone; added to `pass.list`. `const/offset`, which
the old compiler rejects, now gets its real error (`234%&`: an `_OFFSET` suffix after a number) and left
`known_unsupported_rejections.list`. The full run comes with task 10.1 of the change.

**Upstream progress, 2026-10-09: 45 of 279** (`m2-numeric-types` done: task groups 8 and 9 brought
`const/hex_literals`, task 10.1 `const/offset`). Full run with the release build against the reference clone (404
programs, task 10.1): 47 pass, the 45 of `pass.list` and the two `VAL` programs of
`tests\known_clean_not_passing.list`; 312 are rejected with a diagnostic (296 with an `.output`, 16 compile-only),
44 `.err` programs get only "not supported yet" errors; 1 known failure. None builds and prints the wrong output, no
compiler crash. Added to `pass.list`: `const/offset`, an `.err` program that gets its real error since task group
4.

**Upstream progress, 2026-10-09 (later): 47 of 279.** No compiler change: the reference clone (`16f629784e`) was
decided to be the one reference for the old compiler, in CI too (`DECISIONS.md`, `study\28` §2 and §9). The two
`VAL` programs that pass against the clone and not against the 4.7.0 release, `basic/val_default_large_integer_decimal`
and `val_typed_large_integer_decimal`, were run again (both pass) and moved from
`tests\known_clean_not_passing.list` to `pass.list`. Where this file says a program passes "against the 4.7.0
release, as the CI job runs them", that was CI before this date; its `tier2` job now builds against QB64pe's
sources at the clone's commit.
