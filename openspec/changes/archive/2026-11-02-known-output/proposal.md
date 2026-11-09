# Proposal: check what the new compiler prints for a known divergence

## Why

124 verification programs are on `tests\verification\known.list` because the new compiler differs from QB64pe on
purpose, by a row `D-0nn` of `DIVERGENCES-QB64PE.md`. The runner never checks what our compiler prints for them. A
regression in any of these 124 programs goes unseen, whether in the divergent line itself or in the lines around
it that match QB64pe. This is `TODO.md` row 1.

## What Changes

- A new sidecar `tests\verification\diverged\<name>.out.txt` holds the output our compiler should give for a
  program that diverges on purpose. When it exists, the compare judges the program by it in place of QB64pe's
  recording. The rules of the compare stay the same: the folder mask, `exit=`, normalization, the D-012 exit code,
  the log and the divergence trace.
- An optional `diverged\<name>.log.txt` stands in for QB64pe's recorded log in the same way.
- A program with a `diverged\` sidecar moves from `known.list` to `pass.list`. Its file
  `divergences\<name>.divergences` must name the row or rows it takes, so the reason it diverges stays on record.
- A `diverged\` sidecar that holds `(not compiled)` is for a program QB64pe builds and we reject on purpose. Such
  a program needs a row in `lines.list`, which gives the line the rejection must stand on. QB64pe names no line
  for a program it built.
- Sidecars are written for the D programs that have no CLI test of their own: 116 of the 124. Not covered: the 6
  programs a CLI test already names, and the 2 `D-050` programs, whose crash depends on the heap. Each sidecar is
  checked against QB64pe's recording. Every line that differs must be explained by the program's D row. A
  program whose output differs in any other way stays on `known.list`, as a bug.
- Tier 1 checks the sidecars: each names a program of the suite on `pass.list`, with a `.divergences` file that
  names rows of `DIVERGENCES-QB64PE.md`.
- Not built: judging a program recorded as a timeout after 10 s instead of 60. The row expected this to save
  about 17% of a full run's program time. That was true on 2026-10-27. Since D-048 and D-036 were built, none
  of the 13 programs recorded as timeouts runs to a timeout with our compiler. In the last full run
  (2026-11-02) each ended in about 4.5 s, so there is nothing left to save.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `testing/verification-suite`: a new requirement for a program judged by its `diverged\` sidecar. The
  requirement "Pass list and known differences" changes: a program on `pass.list` may now be judged by its
  sidecar.

## Impact

- `tools\legacy_tests\run_legacy_tests.py`: the compare picks the sidecar, a fingerprint and the pass cache read
  it, and a diverged rejection's line comes from `lines.list`. Its tests are in `test_run_legacy_tests.py`.
- `crates\driver\tests\it\inputs.rs`: the tier-1 check of the lists.
- `tools\legacy_tests\counts.py`: the membership block counts the programs judged by a sidecar.
- `tests\verification\`: about 116 new `diverged\` files, the matching `.divergences` files, rows that move
  from `known.list` to `pass.list`, and a few `lines.list` rows. `README.md` there documents the folder.
- The compiler is not changed.
