# Upstream tests

`compile_tests/` is a copy of the text files of QB64pe's `tests/compile_tests` (provenance and licence:
`SOURCE.md`, `LICENSE-QB64pe.txt`; made by `tools/upstream/copy_upstream_tests.py`, never edited by hand).

## Tier 1

`crates/driver/tests/inputs.rs` puts every file of the copy through the new compiler's front end on every
`cargo test`, together with the corpus, the snippets and (when the reference clone is present) QB64pe's
`qbasic_testcases` and its compiler sources: no panic, exact round trip, and the two shrink-only lists
`tests/known_false_errors.list` and `tests/known_unsupported_rejections.list` (spec `testing/upstream-tests`).

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

After a change, regenerate both lists with `QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test inputs`
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
