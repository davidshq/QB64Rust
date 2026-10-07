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

The set columns count parse gaps. After group 6 no first gap is a block any more; the most frequent first gaps are
arrays (93), `REDIM` (144), `SCREEN` (47), `DEFINT` and the other `DEFxxx` (65), `CONST` (41) and statements whose
arguments the parser cannot read yet (`OPEN … FOR`, `LINE INPUT`, `NAME … AS`; 37 `Error` nodes).

After a change, regenerate the three lists with `QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test inputs`
and review the diff: entries may only go away (a new entry is a regression to fix, unless it is a program new to
the inputs).

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
