# Proposal

## Why

Step 6 of M4's plan (`DECISIONS.md` 2026-10-22) is the coverage review: the features checked only by their
emitted call or by programs compiled and never run, with runnable tests where they can be had
(`openspec\changes\archive\2026-10-22-m4-line-numbers-whole-arrays\design.md`, "6."). Today nothing measures
that. Tier 1's "Built-in coverage" asks only that each built-in be *written* in a slice program
(`crates\driver\tests\inputs.rs`), and a slice program may hold it in code it never reaches; the call-site check
compares the call and runs nothing. The 40 names of `sema`'s `NEVER_RUN` are the ones known to be compiled only,
but no tool says whether they are all of them. It is the last step of M4; the user declares M4 done at the merge
into `main` after it.

## What Changes

- **A measure of which built-ins the test runs reach.** A coverage build's record (`codegen-cpp`, `coverage.rs`)
  names the built-ins each counted statement calls (a run whose effect cannot be read still counts as run), so a run of a pass list with `--coverage` says, for every
  built-in, whether any statement calling it ran. The runner's `--coverage`, corpus-only today, is taken by the
  verification and upstream suites too.
- **A report and a check of it** (`tools\coverage\builtin_runs.py`): merges the counts of the slice list, the
  upstream pass list and `tests\verification\pass.list` run with `--coverage`, and lists each built-in as run or
  not run. `--check` fails on a built-in that no run reaches and that is not listed, and on a listed one that ran,
  so the list only shrinks.
- **The list moves out of `sema`**: `NEVER_RUN` becomes `tests\never_run.list`, one built-in per line with a
  reason from a fixed set (the desktop, the clipboard or a console the program shares; a printer; the machine's
  keyboard state or bell; a person must answer; another compiler), read by tier 1's check and by `builtin_runs.py`.
- **Runnable tests where they can be had**: for every built-in the first report finds not run that is not
  inherently unrunnable, a program that runs it, in the home its effect can be compared in (a `verification`
  family recorded with `qb64pe.exe` for a value, a screen program for a window's state). Each of the 40 names of
  `NEVER_RUN` is looked at again under the same rule: where a property can be compared though the value cannot
  (`_DESKTOPWIDTH > 0`, `_TITLE$` after `_TITLE`), the name is run and leaves the list.
- **Its place in the cadence**: at a merge into `main` the three full suites run with `--coverage` and
  `builtin_runs.py --check` reads their counts (the merge set of `CLAUDE.md` working rule 9 and
  `tools\legacy_tests\README.md`); a change's closing set does not take it. This changes working rule 9's merge
  set (user, 2026-10-24, at this proposal).

Out of it: running the executable built from `qb64pe.bas` against the suite (`SOMEDAY.md`); line coverage of the
compiler's own Rust code; the OpenGL routines (no rows of the built-in tables; held by the call-site check and
`Coverage of the OpenGL table`); a differential program's built-ins (the differential pass list is
arithmetic only).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `testing/coverage`: new requirements "Built-ins named in the counts", "Built-ins reached by the test runs",
  "The list of built-ins no test runs" and "The check of the built-ins reached"; "Coverage of a set of runs"
  reads both versions of the counts format.
- `testing/compiler-tests`: requirement "Built-in coverage" reads the explicit list from `tests\never_run.list`,
  with a reason per name.
- `language/images`: requirement "Image built-ins that no test runs" (`_SCREENIMAGE` and `_DEPTHBUFFER` are run).
- `language/mouse`: requirement "Mouse built-ins that no test runs" (`_MOUSESHOW`, `_MOUSEHIDE` and `_MOUSECURSOR`
  are run).

## Impact

- `codegen-cpp` (`coverage.rs`: the record names each entry's built-ins; the record's version goes to 2),
  `sema` (`builtins.rs`: `NEVER_RUN` removed), `driver` (`tests\inputs.rs`: the check reads the list).
- `tools\coverage\` (`coverage.py` reads version 1 and 2; new `builtin_runs.py` with its tests),
  `tools\legacy_tests\run_legacy_tests.py` (`--coverage` for `--suite verification` and `compile`) and its
  `README.md` (the merge set), `tests\never_run.list` (new).
- New programs for the built-ins found not run: two screen programs, `tests\corpus\screen\w01_window_builtins`
  and `w02_depthbuffer`, recorded with `qb64pe.exe` (the first report found no name beyond the 40, so no
  `verification` family was needed; design D5, D6).
- `CLAUDE.md` working rule 9's merge set names the new check; `DECISIONS.md` one row.
- No new dependency.
