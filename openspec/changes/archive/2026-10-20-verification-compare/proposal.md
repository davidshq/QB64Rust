# Proposal: verification-compare

## Why

`verification\` holds 2,842 programs in 45 families, each recorded with `qb64pe.exe` to design a change. It is the
largest body of evidence about the old compiler in this repo, but the runner's `--suite verification` only
records: the new compiler meets a family only through the slice programs and front-end tests that each change
wrote by hand from it. A family's program that no slice program copied can regress without any test seeing
it. The tenth review calls this the largest correctness gap it found (`study\35` §3 item 2, approved,
`DECISIONS.md` 2026-10-18). It was to come after the three changes of `ROADMAP.md` "After M3", which are done.

## What Changes

- `--suite verification` run with `qb64rust` **compares** instead of recording: each program is compiled and run
  under the conditions it was recorded in, and the result is checked against its `<name>.compile.txt` and
  `<name>.out.txt`. Run with `qb64pe` it still records, unchanged.
  - A program the old compiler rejected (`(not compiled)`, 1,710 of them, 31 of them where its C++ build failed)
    passes when the new compiler rejects it with at least one real error, the rule the corpus uses for `.err`.
    The message text is not compared.
  - A program the old compiler built and ran (about 1,130) passes when the new compiler builds it and its run
    gives the same output and the same `exit=` line. The program's folder is masked as `<verification>` on both
    sides. The compiler's own output is not compared for an accepted program.
  - A recorded timeout (7 programs) passes when the new build also times out with the same output.
- Each program is compiled and run in a fresh folder of its own under `target\legacy-tests\`. The folder holds a
  copy of the program and of the family's support files (`v*_inc\`, `v44_files\`, the libraries
  `v37_lib\build_libs.py` builds), so parallel runs (`--jobs`) cannot see each other's files. `verification\`
  itself is never written by a compare.
- `--list`, `--glob`, program names, `--jobs`, `--build-cache`, `--pass-cache` and `--check-list` work for the
  compare as they do for the corpus. Windows go on the hidden desktop, without the key presses: as when recorded,
  a program waiting at `END` times out.
- A new folder `tests\verification\` holds what the new compiler is held to, so that `verification\` keeps the old
  compiler's measurements only:
  - `pass.list`: the programs the new compiler passes. It becomes part of tier 2 and is run at a change's close
    with `--pass-cache`.
  - `known.list`: every other program with its reason: a row of `DIVERGENCES.md` (D-026 among them), "not
    supported yet", an output that varies from run to run, or a bug of the new compiler.
  - `divergences\<name>.divergences` and `normalize\<name>.normalize`: sidecars for divergences and for
    normalisation, as for the upstream suite.
- The first full run sorts every program into one of the two lists. A bug found is fixed in this change when the
  fix is local to one form. Otherwise it stays in `known.list` as a bug and is named in `STATUS.md`'s known
  bugs.
- The closing set (`CLAUDE.md` working rule 9, `tools\close\close.py`) runs `tests\verification\pass.list` in
  place of the hand runs of a family's accepted programs. The merge set judges it from a full run.
  `tools\legacy_tests\counts.py` writes its counts.

Not in this change: the CI `tier2` job (already at its 45-minute limit; one row in `SOMEDAY.md`), and comparing
the wording of rejections (the corpus does not compare it either; `tools\diagnostics` holds the old wording).

## Capabilities

### New Capabilities
- `testing/verification-suite`: the verification programs, recorded by the old compiler and compared against the
  new one, with the pass list and the list of known differences that hold the new compiler to them.

### Modified Capabilities
None. The corpus and upstream suites keep their rules. The compare reuses their rejection rule, divergence trace,
caches and `--check-list`, without changing them.

## Impact

- `tools\legacy_tests\run_legacy_tests.py`: the compare path of `--suite verification`, `--list` and
  `--check-list` for it, a result key per program. Tests of each rule are added to `test_run_legacy_tests.py`.
- `tools\legacy_tests\press_any_key.py`: a way to run on the hidden desktop without the key presses.
- `tools\legacy_tests\counts.py`, `tools\close\close.py` (a closing step, with its test), `CLAUDE.md` working rule
  9 and its layout table, `tools\legacy_tests\README.md`, `verification\README.md` (one line naming the compare).
- New: `tests\verification\` (`README.md`, `pass.list`, `known.list`, sidecars).
- The compiler changes only for bugs that the first run finds and that are fixed here.
- Machine time: a full compare builds about 1,130 programs. With `--pass-cache` at a close, only those whose
  executable changed are run.
