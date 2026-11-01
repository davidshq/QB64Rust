# Design

## Context

See `proposal.md`, "Why". What is there today, and what the close of 2026-10-31 measured with nothing in
`crates\` changed:

- `tools\close\close.py` (1,230 lines) plans a close from a reach map (which files reach which step) and judges
  each step's log; since `0acbd3bb` a log of another commit is stale. `tools\hooks\guard_tool.py` refuses the
  close's runs while a change has open tasks, and `stop_continue.py` keeps a turn going while tasks are open.
- The runner `tools\legacy_tests\run_legacy_tests.py` has two caches: `--build-cache` (the driver's, keyed on
  the C++ fragments, libqb's copy and the toolchain, `crates\driver\src\build.rs`) and `--pass-cache`
  (`pass_key`, line 1409: the executable's bytes, the source, the expected files and sidecars). Every program
  is built before the pass cache is looked at, since the key needs the executable.
- **The miss of the close of 2026-10-31, diagnosed (task 1.1).** No part of the key differed. Of the 4,146
  results, 1,651 were served from the pass cache, 2,478 were programs the compiler rejects (no executable, so no
  key; 0.1 s each, 283 s summed) and 17 were new or changed (14 of them `v66`). The 18.2 minutes of wall time
  were the hits' builds: 8,372 s summed, about 5 s each, because `tools\clean\clean.py`, the close's first
  command, had pruned the build cache to 4 GB (`CACHE_LIMIT_GB`), and a full pass list needs about 6.2 GB (2,505
  entries). A second run of the list on the same build (`target\scratch\2026-10-31\ver-rerun.log`) took 2.2
  minutes: 1,667 served (794 s summed, 0.5 s each, nearly all of it the build-cache lookup), the same 2,478
  rejected programs, one miss, `v53_c_system`, whose C++ holds `_COMPILEDATE$` and changes with every build. Two
  more facts found on the way: an executable holds the absolute path of its build folder (`__FILE__` of
  `qbx.cpp` in the SIGSEGV message, and the CodeView GUID, a hash of the content), so the same program built in
  two folders gives two executables, and a build-cache entry made in a worktree or another results folder is not
  byte-equal to a rebuild here; built twice in one folder, it is byte-equal. The build cache's key was not at
  fault; its size limit was.
- `tools\callsite\callsite.py` compares 2,650 programs one after another (`main`, the loop at line 860): the old
  compiler's C++ from a per-program cache (`old_cpp_cached`), the new compiler's written each time (`new_cpp`,
  `qb64rust -z`). About 6 minutes.
- Tier 1 on a warm build is 101 s, of which 84 s are tests; cold, the debug build of 52 test binaries is most of
  about 7 minutes. `crates\*\tests\` holds 27 integration-test files, each its own binary.
- **The baseline (task 1.2)**, one run each, one after another, on a warm build with nothing changed
  (`target\scratch\2026-10-31\baseline\`): tier 1 warm 85 s (702 tests passed and 121 ignored, in 40 test
  binaries and 11 doc-test runs); tier 1 cold, the workspace's own crates cleaned (`cargo clean --profile dev
  -p` each, dependencies kept), 217 s, of which the build 102 s (a `--list-only` run beside it for its first two
  minutes, so the figure is high, not low); clippy warm 2 s; the slice list 25 s, the upstream list 48 s and the
  verification list 168 s, each with both caches warm; the call-site check 170 s (2,650 programs, the old
  compiler's cache warm); the CLI tests (`--ignored`) 100 s (96 tests). The release build was current (1 s).
- **The fold (task 5.1)**: 27 test files into 7 binaries (`crates\<c>\tests\it\main.rs`), with `git mv`. Tier 1
  ran the same tests after it: 704 passed and 121 ignored (the baseline's 702 and 121, and the two tests this
  change adds: `program_inputs_key_follows_the_program_not_libqb`, `external_sidecars_mark_programs`), in 20
  test binaries in place of 40. What changed besides the place: `mod support;` became `use crate::support;`
  (one `mod support;` in each `main.rs`, the folder moved under `it\`); insta names a snapshot after the module
  path, so the 71 snapshots of `assert_snapshot!` became `it__<module>__<name>.snap` under `tests\it\snapshots\`
  (their `source:` line too), while the frontend's 455, named by the test itself, kept their names; the
  commands named in the known lists' headers and in the documents (`--test it inputs::` for `--test inputs`).
- **nextest (task 5.2)**: `.config\nextest.toml` as D7. The workspace's tests but inputs.rs's: 686 in 15 s warm
  (`cargo nextest run`), against 85 s for all of tier 1 under `cargo test`. inputs.rs's 18 stay under `cargo test`
  (the `lists` step): they share one computed outcome of every input per process, so a process per test took
  85 s, against 33 s in one. Checked with a temporary test in `format` and a copy of the settings with a 3 s
  period: a test that hangs is reported slow, ended at twice the period and retried, the other 47 going on; a
  test that passes only on its retry is reported flaky and fails the run.
- **Tier 1 after the fold (task 5.3)**, measured as the baseline (`target\scratch\2026-10-31\after-fold\`): warm,
  `cargo test --workspace` 53 s (baseline 85 s); the quick layer's split, `nextest` 16 s and `lists` 34 s. Cold
  (the workspace's own crates cleaned): `cargo test --workspace` 128 s (baseline 217 s, which had a run beside it);
  `nextest` 91 s with the build, then `lists` 42 s. 704 passed in each.
- **The call-site check in parallel (task 4.1)**, the old compiler's cache warm: `--jobs 1` 138 s, `--jobs 8`
  42 s, and 32 s in a second pair (whose `--jobs 1`, 154 s, had a `bacon` run beside it); the reports of each pair
  identical but for the header line, 2,650 equal. The first pair found 4 `_LOG*` programs different: the new
  compiler names the source by its path, which now has the program's folder in it; the mask of the scratch
  folder (`WORK`) takes that folder too.
- **The quick layer (task 7.1, so far)**: the first `just quick` with no records ran every step in 266 s (and
  found a real fault: `cargo fmt` wanted the folded `main.rs` files sorted); with nothing changed it planned
  nothing and took 1 s; after a one-line edit of `crates\sema\src\lib.rs` it ran `fmt`, `build`, `clippy`,
  `nextest`, `lists`, `doc` and `slice`, nothing else, in 205 s (`lists` 52 s, `slice` 69 s, `build` 47 s,
  `nextest` 31 s). The verification list as a step, with the new pass cache filled, 98 s: 1,563 programs served
  without a build, the 107 `.external` ones and the 2,478 rejections run (`target\scratch\2026-10-31\
  ver-pc2-2.log`). `v29_a_sound_queue`, now `.external`, was the one failure of the first run (the
  second started it first, as the only program that had failed) and failed again there (its queue held 4 where 3
  is recorded), passing when the runner ran it again alone; under the old key it had been
  served from one old pass for weeks. Running what depends on a device shows what the cache hid.
- No crate has a compiled doc example: every crate's doc tests run 0 tests, the fenced blocks being `text`
  (the earlier line here said one had; it was wrong). The `doc` step stays for the day one is written;
  `insta` is the snapshot library; `difftest` and the recordings
  against `qb64pe.exe` are the oracles.
- Four CI workflows run on push and pull request (Rust, VS Code extension, license inventory, repo check; the
  Rust workflow's tier 2 job only by hand). This change sets all four to run only by hand (task 6.0). A
  worktree of the commit is the place where `check full` runs beside the working checkout.

## Goals / Non-Goals

**Goals:**
- A run never repeats work whose inputs are unchanged, and a commit changes no input.
- The quick layer under 5 minutes warm; a no-change `check quick` plans nothing.
- The cost held by tests, not by prose.
- Nothing refuses a command or holds a session.

**Non-Goals:**
- Rewriting the runner, the recorder, the screen-state check or the call-site check in Rust, or replacing them
  with `lit`, Bazel or a monorepo task runner: each knows this project's sidecars and oracles; a task runner's
  unit is a project, not one of 4,000 programs.
- sccache: it caches objects, not linked programs; the link against libqb is the cost the executable cache
  already saves (the second opinion of 2026-10-31).
- Changing what any suite checks, or any recording.
- Automatic CI on push: the user's standing decision; `check full` in a worktree of the merge commit is the
  "not rocket science" rule by hand.
- Merging the 4,000 programs into fewer binaries: each is a BASIC program whose build is part of its test.

## Decisions

**D1. The pass cache keys on what the build reads, not on the executable.** The key becomes: the program's
half of the driver's build key, which `qb64rust -z` prints with `--print-build-inputs` (the fragments, the
settings with the make flags, the resource and embedded files, the `DECLARE LIBRARY` files and headers: what
`build_key` in `crates\driver\src\build.rs` hashes before the stamps of libqb and the toolchain), a hash of the
contents of libqb's copy (`libqb\`, the files `build_key` stamps, by content, once per run) and the toolchain's
identity (the toolchain's executables by content, once per run), the runner's own hash, and what `pass_key` takes
today but the executable. The run then goes: write the C++ with `-z`, look up; on a hit neither build nor run.
The program's half comes from the driver, not from hashing the fragments in Python, because the source is not
the only input of the settings: under this key the compiler binary is not an input, and a compiler change that
moves a make flag, a resource or a library without changing a fragment must miss. Code: `build_key` split in
two (`program_inputs_key`, the stamps after it), the flag in `crates\driver\src\main.rs` and `compile.rs`;
`run_legacy_tests.py`, `pass_key` and its callers in the corpus, compile and verification suites, one
`build_inputs(...)` helper beside `pass_key`; the cache folder becomes `target\pass-cache-2` (the old one is
not read; `clean.py` removes it). *Why, measured (task 1.1):* the pass cache did not miss at the close of
2026-10-31; the hits' builds cost the time, on a build cache pruned to 4 GB. *Alternative:* keep the
executable as the key and raise `clean.py`'s limit to about 10 GB. Rejected: 6 GB more of disk that the user
cut on 2026-10-31, hits still cost a build-cache lookup (0.5 s each), and every prune brings the 18 minutes
back; an executable also holds its build folder's path, so it is a poor key across worktrees. A program the
compiler rejects writes no C++ and is not cached, as today (2,478 of the verification list, 0.1 s each).

**D2. Programs that cannot be cached carry an `.external` sidecar**, beside their `.bas`: one line, the reason.
The runner never remembers them and never caches a failure (today's `remembered.touch()` already runs only on a
pass; the sidecar is the new part). The `varies` reason of `known.list` is the same mark for verification
programs. Which programs: every `.screen` and `.keys` program, the sound programs (`s75`–`s76`, `s127`–`s129`),
`v39`, `v46`, the clipboard ones, the timer ones; the list is made in task 2.3 from the `varies` rows,
`known.list` and `tests\never_run.list`'s families, and checked by a test that every `.screen` program has one.
Code: `pass_key`'s callers; `inputs.rs` of the driver's tests for the check. *Alternative:* a list file.
Rejected: the sidecar stays with the program when it moves, as the other sidecars do.

**D3. `check` is a `justfile` over `tools\check\check.py`.** The `justfile` holds the recipes (`quick`,
`full`, `path`, `status`, `force`) and nothing else; `check.py` holds the fingerprints, the plan and the
report. The fingerprint of a step is the SHA-256 over the paths and bytes of its inputs; `tools\check\steps.py`
names each step's inputs as globs (`crates/**`, `Cargo.lock`, `libqb/**`, `tools/libqb/edits/**`,
`tests/corpus/slice.list`, `tests/corpus/slice/**`, …) and this list is the cache-key specification the review
asked for; `test_check.py` has a test that every file the runner reads (from a `--list-only` run's manifest) is
covered by some step's globs. Results go to `target\check\<step>.json` (fingerprint, result, started, seconds,
log path). A step whose fingerprint matches a passing result is reported `standing`; a failed result is never
`standing`; `--force` ignores them. Code: new `tools\check\` (about 300 lines with tests); `clean.py` keeps
`target\check\`. *Alternative:* keep `close.py` and change its staleness rule. Rejected: its plan logic is the
reach map plus the log judge plus the commit rule, 1,230 lines for a question the fingerprints answer in one
function; and the user asked for the workflow from nothing.

**D4. The quick layer's steps and their inputs.** `fmt` (crates), `clippy` (crates, Cargo.lock), `nextest`
(crates, Cargo.lock, tests\frontend, tests\corpus for the front-end checks, tests\callsite for `inputs.rs`),
`doc` (the one crate with a doc example), `tools` (`python -m unittest` over `tools\*\test_*.py`; inputs
`tools\**`), `slice` (the release build, libqb, the slice list and group, the runner). The release build is a
step of its own (`build`, inputs `crates\**`, `Cargo.lock`, `libqb\**`) that the program steps depend on. A
step with no changed input since its last pass is not planned. Code: `tools\check\steps.py`.

**D5. `check full` is the quick layer plus every other suite,** run in order of last duration, longest first,
in the background: upstream and verification pass lists, the call-site check, the CLI tests (`--ignored`),
qbasic, addprefix, the format runs, the differential list, the run without folding, the coverage check, the
Linux family in WSL, the sandbox family last and alone. It is meant for a worktree of the commit
(`git worktree add`); `check full` refuses to start in a checkout with uncommitted changes under `crates\`,
`libqb\` or `tests\`, naming the worktree command, so its results describe one commit. The merge into `main`
runs it with `--force` (no cache, as today's merge set). Code: `tools\check\check.py`, `full`.

**D6. The call-site check takes `--jobs`.** The new compiler's side runs in a thread pool, each program in its
own scratch folder (today one `work` folder); the old side keeps the per-program cache and, on a miss, the
clone's lock. Results are collected per program and printed in the programs' order. Code:
`tools\callsite\callsite.py`, `main` (the loop at line 860) and `new_cpp` (a folder per program).

**D7. One test binary per crate, nextest, bacon.** Each `crates\<c>\tests\*.rs` moves to
`crates\<c>\tests\it\<name>.rs` with a `tests\it\main.rs` of `mod` lines; `autotests = false` is not needed
(a folder is not a test target). `.config\nextest.toml`: `slow-timeout = { period = "120s", terminate-after =
2 }`, `retries = 1`, `flaky-result = "fail"`, a JUnit report under `target\nextest\`; the CLI tests that build
programs keep `#[ignore]` and run under `check full`. `cargo test --doc -p <that crate>` is the `doc` step.
`bacon.toml` gets a `nextest` job; it is offered, not required. Code: `Cargo.toml` of each crate unchanged;
the test files moved with `git mv`.

**D8. Failures first, then longest first.** The runner writes `target\check\times\<suite>.json` (test, last
seconds, last result) at the end of each run and orders the next run by it: failures, then by seconds
descending. The verification and corpus suites already order by nothing; the order of the report stays by name.
Code: `run_legacy_tests.py`, where the program list is built for the thread pool (`run_suite`); `nextest` does
this itself.

**D9. What goes, and what of the hooks stays.** `tools\close\` is deleted; `stop_continue.py` and the close
part of `guard_tool.py` (`CLOSE_RUNS`, `WORKING_RUN`, `open_tasks`) are deleted; the guard of human rule 1
(staging, stashing, a plain `rm`/`mv` of a tracked file, writes at the top of `target\`) stays, since it
enforces a human rule, not a test rule. `tools\testtime\` moves to `study\archive\testtime\` as the
measurement it was. CLAUDE.md working rule 6 becomes four lines (D10). `.claude\settings.json` loses the Stop
hook. *Why not keep `close.py` for merges:* `check full --force` is that set.

**D10. The rule.** Working rule 6, whole: "Run `cargo test -p <crate>` on what you touched while working. Run
`just quick` in the background before handing over files to stage, and say what ran, what stood and why. `just
full` runs in a worktree of the commit, started by the user, before a merge into `main`. Read a run's output
from its log; never start a run again to read it." Recorded in `DECISIONS.md` in task 6.2.

## Risks / Trade-offs

- [A cached pass is the riskiest cache: an input left out of a step's globs makes a stale pass stand] → the
  manifest test of D3 (every file a runner reads is covered), and the key of D1 lists every input by name; a
  `--force` run at each merge.
- [The C++ as key: a change in the compiler that changes no program's C++ but changes the driver's build
  (flags, the Makefile it writes) would be missed] → the build's inputs (the Makefile, `CXXFLAGS`, the
  toolchain identity) are in the key, D1; a test writes a program's C++, changes a flag, and expects a miss.
- [Folding 27 test files into 7 binaries loses `cargo test --test <name>`] → `cargo nextest run -p <crate> -E
  'test(<name>)'` and `check crates/<c>` cover it; the names of the modules are the old file names.
- [nextest and just are new tools to install] → `crates\README.md` names them with their `cargo install`
  lines; `check` prints the install line when one is missing, and falls back to `cargo test` for nextest.
- [The thread pool of the call-site check and the clone's lock] → a miss on the old side takes the lock once
  per program, as today; with the cache warm (it is, after one run) no lock is taken.
- [`check full` refusing a dirty checkout surprises] → the message names `git worktree add` and the reason.
- [The timing file makes runs non-deterministic in order] → the report is sorted by name; only the start order
  changes.

## Migration Plan

1. Build `tools\check\` beside `tools\close\` (groups 1–3), with the old still runnable.
2. Switch the runner's cache key and the call-site check (groups 2, 4): a one-time cold pass-list run fills
   the new cache (`target\pass-cache-2`), about 35 minutes once, in the background.
3. Fold the test binaries and add nextest (group 5): tier 1 is the first thing `just quick` runs, so a wrong
   fold shows at once.
4. Delete the old tooling and rewrite the rule (group 6). Rollback: `git revert` of group 6 brings `close.py`
   back; nothing of groups 1–5 depends on its absence.

## Open Questions

- ~~Whether `bacon` is worth a `bacon.toml`~~ Closed (task 5.4): a line in `crates\README.md`, no `bacon.toml`.
  bacon 3.26 has a `nextest` job built in, and the filter that leaves out inputs.rs's tests (85 s under nextest)
  goes on its command line; `bacon nextest --headless` with it ran the workspace's tests once on start, as the
  terminal mode does on each save. A file of the repository for one job and one filter does not earn its place.
