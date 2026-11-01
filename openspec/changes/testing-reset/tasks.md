# Tasks

Standing rules: CLAUDE.md. This change replaces working rule 6 (task 6.2); until then, its runs are the
change's own `just quick`, and the old close is not run.

## 1. Measure before building

- [x] 1.1 Find why the pass cache missed 2,614 of 4,139 verification programs at the close of 2026-10-31 with
      nothing in `crates\` changed: run the pass list twice with `--pass-cache` on one build, log the key's parts
      for ten missed programs (`target\scratch\<date>\miss.log`), name the part that differed in `design.md`
      ("Context"), and fix the build cache's key if the part is its (`crates\driver\src\build.rs`); verified by
      a second run of the list in which every program not marked `varies` is served from the cache.
- [x] 1.2 Record the baseline in `design.md` ("Context"): wall time of tier 1 warm and cold, clippy, the slice
      list, the upstream and verification lists with the cache warm, the call-site check, the CLI tests, from one
      run each with the output in `target\scratch\<date>\`; these are the numbers task 7.1 compares against.

## 2. The pass cache keyed on the C++

- [x] 2.1 `qb64rust -z --print-build-inputs` prints the program's half of `build_key` (design D1), tested in
      the driver: the same for two builds of one program, another value for a changed fragment, make flag,
      resource and library; `build_inputs(...)` in `run_legacy_tests.py` beside `pass_key` adds libqb's copy
      by content and the toolchain's executables (both hashed once per run); `pass_key` takes it in place of
      the executable's bytes, the cache folder becomes `target\pass-cache-2`; verified by
      `test_run_legacy_tests.py`: the same build inputs from two different executables is one key, a changed
      program half, a changed libqb file and a changed toolchain file are three other keys.
- [x] 2.2 On a hit the program is neither built nor run, in the corpus, compile and verification suites; the
      message becomes "not run: passed before with this C++ and these expected files"; verified by a pass-list
      run of `tests\corpus\slice.list` twice on one build, the second with every program but the `.external`
      ones served from the cache and no C++ build started (the build cache's log shows no entry made).
- [x] 2.3 The `.external` sidecar: the runner never remembers a program that has one; one is added to every
      `.screen` and `.keys` program, the sound, timer and clipboard programs, and the `varies` programs; a test
      in `crates\driver\tests\it\inputs.rs` fails when a `.screen` program lacks one; `tests\corpus\README.md`
      and `tools\legacy_tests\README.md` describe the sidecar; verified by the test and a slice run in which
      those programs run twice.
- [x] 2.4 Failures first, then longest first: the runner writes `target\check\times\<suite>.json` at the end of
      a run and orders the next run's start by it; verified by a unit test of the ordering function and a slice
      run after a forced failure in which that program starts first.
- [x] 2.5 `clean.py` removes `target\pass-cache` (the old folder) and keeps `target\pass-cache-2` and
      `target\check\`; verified by `test_clean.py`.

## 3. The check tool

- [x] 3.1 `tools\check\steps.py`: each step with its command, its input globs and the steps it depends on, for
      `fmt`, `clippy`, `build`, `nextest`, `doc`, `tools`, `slice`, `upstream`, `verification`, `callsite`,
      `cli`, `qbasic`, `addprefix`, `format`, `format-corpus`, `differential`, `nofold`, `coverage`,
      `linux-sync`, `linux-tier1`, `linux-slice`, `sandbox`; verified by `test_check.py`: every step's globs
      match at least one tracked file, and the manifest test (every file a `--list-only` run of each suite reads
      is covered by its step's globs).
- [x] 3.2 `tools\check\check.py`: the fingerprint of a step (SHA-256 over the paths and bytes of its inputs),
      `target\check\<step>.json` (fingerprint, result, started, seconds, log), `quick`, `full`, `status`,
      `--force`, a step whose fingerprint matches a passing result reported `standing`, a failed result never
      standing, runs over a minute started in the background with the log's path printed; verified by
      `test_check.py` with a fake repository: standing after a no-change rerun, rerun after an input change,
      rerun after a failure, everything rerun with `--force`.
- [x] 3.3 `check <path-or-name>`: a crate path runs that crate's tests, a suite folder or list runs that suite,
      a program name runs that program in its suite (found by name across `tests\corpus\*`, `tests\upstream`,
      `verification\`); verified by `test_check.py` and by `just check v66_a01_varptr_dynamic_owner`.
- [x] 3.4 The cost tests: `test_check.py` asserts the plan for a documents-only tree (nothing), a tree with new
      verification programs and changed lists (their compare and the list check), a tree changed under
      `crates\sema` (the quick layer), a libqb edit naming built-ins (the programs that run them, from the
      coverage counts, plus lints and Rust tests); verified by the tests passing and by one deliberate
      widening of a glob failing them.
- [ ] 3.5 `check full` refuses a checkout with uncommitted changes under `crates\`, `libqb\` or `tests\`, naming
      `git worktree add`; runs the steps longest-first by the times file; the sandbox family last and alone;
      verified by `test_check.py` and one real `just full` in a worktree with its report kept in
      `target\scratch\<date>\full.log`.
- [x] 3.6 The `justfile`: `quick`, `full`, `check <target>`, `status`, `force`, each one line calling
      `check.py`; `crates\README.md` "Building and testing" names `just` and `cargo-nextest` with their install
      lines, and `check` prints the install line for a missing tool; verified by `just --list` and by running
      `just quick` without `nextest` on the path (falls back to `cargo test`, says so).

## 4. The call-site check in parallel

- [x] 4.1 `callsite.py --jobs`: the new compiler's side in a thread pool with a folder per program, the old side
      from its cache or under the clone's lock, the report in the programs' order; verified by `--self-test`
      and by two runs, `--jobs 1` and `--jobs 8`, with identical reports and the time of the second in
      `design.md` ("Context").

## 5. Tier 1 made cheap

- [x] 5.1 Each crate's `tests\*.rs` moved with `git mv` to `tests\it\<name>.rs` under a `tests\it\main.rs` of
      `mod` lines (`dap`, `driver`, `lsp`, `sema`, `syntax`, `format`, `difftest`), test names and bodies
      unchanged; verified by `cargo test --workspace` reporting the same count of tests as before the move
      (the count written in `design.md`) and by `ls target\debug\deps\*.exe` showing one test binary per crate.
- [x] 5.2 `.config\nextest.toml` (slow-timeout with terminate-after, one retry, `flaky-result = "fail"`, a
      JUnit report under `target\nextest\`); the `nextest` step of `check` uses it, the `doc` step runs `cargo
      test --doc` for the crate with the doc example; verified by `cargo nextest run --workspace` passing and
      a deliberately hanging test ended at the timeout.
- [x] 5.3 Tier 1's time after the fold, warm and cold, written in `design.md` ("Context") beside the baseline.
- [x] 5.4 Try `bacon nextest` for one session; keep a `bacon.toml` if it earns it, else a line in
      `crates\README.md`; `design.md`'s open question closed either way.

## 6. The old tooling out, the rule in

- [x] 6.0 The four CI workflows (`rust.yml`, `vscode-extension.yml`, `licenses.yml`, `repo-check.yml`) set to
      `on: workflow_dispatch` only, `release-vsix.yml` unchanged; verified by `grep -n "push:\|pull_request:"
      .github/workflows/*.yml` naming only `release-vsix.yml`.
- [ ] 6.1 `tools\close\`, `tools\hooks\stop_continue.py`, and the close part of `guard_tool.py`
      (`CLOSE_RUNS`, `WORKING_RUN`, `open_tasks` and their tests) removed; `tools\testtime\` moved with `git mv`
      to `study\archive\testtime\`; `.claude\settings.json` loses the Stop hook (the user applies this edit);
      verified by `test_hooks.py` passing on the guard that stays and `python -m unittest discover -s
      tools/hooks` finding no reference to the close.
- [x] 6.2 CLAUDE.md working rule 6 replaced by the four lines of design D10; `tools\close\README.md` gone, its
      cadence section replaced by `tools\check\README.md` (the three layers, the fingerprints, the sidecar, the
      cost tests); `docs\LAYOUT.md` rows for `tools\check\`, `justfile`, `.config\nextest.toml`, the moved
      folders; a `DECISIONS.md` row dated with the rule and this change's name; verified by `check_repo.py`
      and by `grep -rn "close.py" --include=*.md` finding only the archive and `DECISIONS.md`.
- [x] 6.3 `STATUS.md` "Next" updated: this change built, the baseline and the new times in one line, and the
      main checkout's close of 2026-10-31 noted as the last of its kind.

## 7. Integration

- [ ] 7.1 The numbers: `just quick` on a warm build with nothing changed plans nothing; after a one-line edit
      in `crates\sema` it runs fmt, clippy, nextest and the slice list in under 5 minutes; `just full` in a
      worktree passes every step; all three times written in `design.md` ("Context") against the baseline of
      1.2.
- [x] 7.2 `openspec validate testing-reset` passes.
