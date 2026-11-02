# Proposal

## Why

Testing takes a fifth of all working time (37 hours of 181 by `tools\testtime\REPORT.md`, 2026-10-02 to
2026-10-28), most of it waiting for runs that repeat work already done: the verification pass list ran 65 times
for 21 hours, the slice list 262 times, and tier 1's wall time is almost all compiling (84 s of tests inside about
7 minutes). Each fix so far (the closing set, the reach map, the hooks, and on 2026-10-31 the rule that a log of
another commit is stale) added a gate on top without making a run cheaper or stopping a repeat; the last one made
every commit invalidate every log, so a batch of recordings and documents cost a 40-minute close. The user asked
for the workflow rebuilt from nothing, at a pace that lets work go on while tests run (`study\41`).

## What Changes

- **One entry point, three layers.** `check` (a `justfile`) with `check quick` (fmt, clippy, the workspace
  tests, the tool tests, the slice list; under 5 minutes warm, in the background), `check full` (everything, in a
  worktree of the commit, started by the user) and `check <path-or-name>` (one way to narrow, for every suite).
- **A run is valid by its inputs, never by the commit.** Each result records a fingerprint of every input it
  depends on; it stands while they match. Committing, rebasing and switching branches invalidate nothing. A
  failure is never cached; programs that depend on time, devices or the desktop are tagged and never cached;
  `--force` reruns anything.
- **The pass cache keyed on what the program's build reads** (the C++ the compiler writes and the build's
  settings, as the driver's build key takes them), libqb's sources, the C++ toolchain, the runner and the
  sidecars, not on the executable's bytes. Today's key is the executable, so every program is built before the
  cache is looked at: at the close of 2026-10-31 the verification pass list took 18.2 minutes with only 17 of
  its programs new or changed, because `clean.py` had pruned the build cache to 4 GB and the 1,651 programs
  served from the pass cache were rebuilt first (about 5 s each); with the build cache warm the same list took
  2.2 minutes (`design.md`, "Context"). With the build's inputs as the key a hit needs no build and no build
  cache, which can stay small. **BREAKING** for the cache's folder: the old `target\pass-cache` entries are not
  read.
- **The call-site check runs its programs in parallel** (today one after another, about 6 minutes for 2,650
  programs); the old compiler's side is already cached per program.
- **Tier 1 made cheap:** each crate's integration tests in one binary, `cargo nextest` as the runner (with
  `cargo test --doc` beside it for the one doc example), `bacon nextest` offered for a warm build while editing.
- **Failures first, then longest first** in every runner, from a timing file.
- **The check tool's cost is held by tests:** a docs-only change plans nothing; a list change plans its suite's new
  programs; a `sema` change plans `quick` and nothing else; a libqb edit plans the programs of its built-ins.
- **Removed:** `tools\close\` (1,230 lines plus 1,244 of tests), the close gate and the Stop hook of
  `tools\hooks\` (the guard of human rule 1 stays), the `.head` files, CLAUDE.md working rule 6 cut to four
  lines, the cadence section of `tools\close\README.md`. CI stays started by hand; `check full` in a worktree is
  the "not rocket science" rule done by hand.
- **Kept as they are:** the runner (`tools\legacy_tests`), the recordings and the oracle (`qb64pe.exe`), the
  screen-state and call-site checks, `insta`, `difftest`, the `varies` quarantine. sccache was weighed and left
  out: it caches objects, and a program's cost is its link against libqb, which the executable cache already
  saves.

## Capabilities

### New Capabilities
- `testing/check-workflow`: the `check` entry point, its three layers, the validity of a result by its inputs, the
  ordering of runs, and the tests that hold the tool's cost.

### Modified Capabilities
- `testing/verification-suite`: "Selecting and caching the compare": a program passed before is not run again
  while its generated C++, libqb, the toolchain, the runner and its sidecars are unchanged (today: its executable
  and the compiler binary).
- `testing/golden-corpus`: the same key for `--pass-cache` of the corpus and compile suites; `varies` and screen,
  sound and sandbox programs are never cached; failures are never cached.
- `testing/call-site-check`: the programs are compared in parallel, `--jobs` as the runner's.
- `testing/compiler-tests`: tier 1 runs under `cargo nextest` with one integration-test binary per crate; the
  tier 2 slice run is the `quick` layer's program run.

## Impact

- `tools\legacy_tests\run_legacy_tests.py`: the pass-cache key and the tag of uncacheable programs; a timing file
  and failure-first order. `tools\callsite\callsite.py`: `--jobs`.
- New: `justfile`, `tools\check\check.py` (the fingerprints, the plan, its cost tests), `.config\nextest.toml`.
- `crates\*\tests\`: 27 test files folded into one `tests\it\main.rs` per crate (`dap` 4, `driver` 12, `lsp` 3,
  `sema` 3, `syntax` 3, `format` 1, `difftest` 1). No test changes; only where it is linked from.
- Deleted: `tools\close\`, `tools\hooks\stop_continue.py`, the close part of `tools\hooks\guard_tool.py`,
  `tools\testtime\` moved to `study\archive\` as the measurement it was.
- Documents: CLAUDE.md working rule 6, `tools\close\README.md` (gone), `tools\legacy_tests\README.md`,
  `crates\README.md` test commands, `docs\LAYOUT.md`, `.claude\settings.json` (the Stop hook removed), the four
  four CI workflows (Rust, VS Code extension, license inventory, repo check) set to run only by hand
  (`workflow_dispatch`).
- New developer tools to install: `cargo-nextest`, `just`; `bacon` optional.
