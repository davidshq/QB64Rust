# 19. How often each test layer runs

Written 2026-10-03, after the first full golden-corpus runs (`tests\corpus\README.md`). Decided with the user the
same day (`CLAUDE.md` decisions table). Goal: conformance testing must not slow down day-to-day work on the new
compiler.

## 1. What a full run costs today

Measured with the old compiler (`baselines\`):

| Suite | Programs | Time | Where the time goes |
|---|---|---|---|
| Golden corpus (`--suite corpus`) | 262 | about 11 min | about 2.5 s per program, almost all of it qb64pe generating and compiling C++ and linking; running the program takes milliseconds |
| Golden corpus, `-O2` (`--cpp-opt`) | 262 | about 11 min | the same, with the optimiser on |
| Legacy suites (`--suite all` before the corpus existed) | 546 + 24 format | about 37 min | the same per program, plus the qbasic programs |

Programs run one at a time because the old compiler builds in one shared folder (`internal\temp`, `study\05`).
Known failures that hang (no expected file) are compiled but not run (2026-10-03); before that they added 60 s
each.

## 2. Runs with the old compiler

After the corpus is recorded, the old compiler is run rarely:

- **New corpus programs:** `--record` with `--glob` over the new files only.
- **New QB64pe version as reference:** one full check run and one `-O2` run; new baseline files.
- **A question about old behaviour:** a one-off program in `verification\`, or a `--glob` slice of the corpus.

## 3. Runs with the new compiler: four tiers

Each tier is cheaper and runs more often than the next.

| Tier | When | What | Target time |
|---|---|---|---|
| 1. `cargo test` | every edit | unit and snapshot tests; the corpus **front end only**: every corpus program is parsed and checked in-process, and the `.err` programs' diagnostics are compared (no C++, no executable) | seconds |
| 2. Corpus slice | before a commit, on demand | `--suite corpus --glob` over the programs of the feature being changed (`*print*`, `*redim*`) | under a minute |
| 3. Full corpus, parallel | CI on each push and pull request | all corpus programs, several at a time | a few minutes |
| 4. Everything | nightly and before a release | full corpus, the `-O2` comparison, the legacy suites | under an hour, unattended |

Nobody has to run tier 3 or 4 by hand during normal work. A tier-3 failure in CI is fixed like any other failing
test.

## 4. What makes the new compiler's runs fast

- **Front-end checks without C++.** For the 20 `.err` programs, and for any check that the front end accepts a
  program, the new compiler needs no C++ step. These take milliseconds and run in tier 1. The same code path serves
  the language server (`study\15`: it reports only errors the old compiler also reports).
- **Parallel runs.** The new compiler has no shared build folder, so the runner may run programs in parallel when
  the compiler under test is not qb64pe (option to add to `run_legacy_tests.py`; the old compiler stays
  sequential). With 8–16 workers a full corpus run should take 1–2 minutes; measure at the vertical slice.
- **Result cache (later).** Skip a program whose `.bas`, expected file, `.normalize` and compiler build are
  unchanged since it last passed. Most commits then rerun only what they can affect.
- **Prebuilt runtime.** The per-program cost should be compiling the generated C++ and linking against a libqb built
  once. Measure at the vertical slice (`study\15` §2) before optimising further.

## 5. Consequences for M2

1. The runner gains a parallel option for compilers other than qb64pe (when the new compiler first produces
   executables).
2. The Rust test harness reads `tests\corpus` directly for tier 1: one test per program, front end only, comparing
   `.err` programs' diagnostics with the expected file once the new error messages and the old texts are mapped
   (`CLAUDE.md`: new error messages, optional old texts).
3. CI (tier 3) needs a Windows runner with the new compiler's C++ toolchain; nightly (tier 4) also needs the QB64pe
   release for the legacy suites. Planned with the CI work, not now.
4. The result cache waits until the full corpus run is slow enough to matter.
