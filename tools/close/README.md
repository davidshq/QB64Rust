# The closing runs of a change

`close.py` holds the closing set of `CLAUDE.md` working rule 6 as a plan and a check. It runs nothing: each test
run is started by its own command, so that `tools\testtime` sees it. When each run happens, at a change's close and
at a merge into `main`: "The test cadence" below.

```
python tools/close/close.py --plan                  # the commands, in order, each with its log under target/close/
python tools/close/close.py --check                 # each step: ran on the current build, and passed?
python tools/close/close.py --check --known <test>  # a failing test that is known and not the change's
python tools/close/close.py --check --not-repeated slice,upstream=<why>   # older than an edit that cannot affect them
```

**The plan** (`--plan`) prints, for the working tree as it stands, Bash commands (Git Bash or the Bash tool, not
PowerShell): the shrink-only lists and the release build first ("Order" below: what changes the compiler or what
tier 1 reads before the first long run); tier 1 as CI runs it, `--workspace
--locked`, with `--no-fail-fast` (without it `cargo test` stops at the first failing test binary and the tests
after it never run), fmt and clippy; the slice list and the upstream pass list with `--pass-cache` and
`--build-cache`, side by side; QB64pe's converter test (`--suite addprefix`, a few seconds); QB64pe's qbasic programs (`--suite qbasic --jobs 8`,
compiled only, every one to build); the run without folding (with `--pass-cache` since builds are deterministic,
`DECISIONS.md` 2026-10-27: only the programs whose C++ folding changes run), only when a file of `sema`'s constant
evaluation, literals or operators changed against `HEAD` (`--nofold` asks for it when a numeric conversion elsewhere changed); the
coverage check of Civil War Strategy's scripts (`tools\coverage\coverage.py --check`), only when the game's copy or
its scripts (`tests\corpus\screen\g12_game.*`), its list of unreached lines, `tools\screen\cut_game.py`,
`screen_state.cpp`, `tools\coverage\`, `crates\codegen-cpp\`, `crates\ir\` (which statements are counted) or
`crates\driver\` (the record's files) changed (`--coverage` asks for it anyway); QB64pe's format tests
(`--suite format`) and the format oracle (`--suite format-corpus`, `tests\format\`) with the new compiler, only when
`crates\format\`, `tests\format\`, `tools\format\`, the layout settings (`base`'s `layout.rs`), `crates\syntax\`,
`crates\builtins\`, `sema`'s symbols (`symbols.rs`, `check\places.rs`, `check\decl.rs`) or the driver's `-y`
(`main.rs`, `lib.rs`) changed (`--format` asks for them anyway); the
call-site check and the run without folding with `--wait` (the call-site check needs the clone alone); the
command-line tests that build, alone. Then `--check`.

**The check** (`--check`) reads each step's log and names it:

- `MISSING`: no log;
- `STALE`: the log is older than the release build or than a changed input (a file under `crates\`, `tests\` or the
  test tools, changed against `HEAD`, staged or not; documents and the test-time record do not count); for the
  build, also when the executable itself is older than a changed input (a build that stopped, another profile);
  for the wording table and the shrink-only lists, when the log is newer than the build or older than a changed
  input other than the files those two write (a log left by an earlier change); for a run that compiles with a
  copy of the release build (the slice and upstream lists, the run without folding, the call-site check;
  `tools\legacy_tests\README.md`, "The compiler a run tests"), when the build its first line names by hash is not
  the current one, or when it started before a changed input that does not reach it through the build (a file
  under `tests\` or the test tools; one under `crates\` reaches it only through the build, which is judged
  itself). Such a run may end after a new build and still count, as long as the bytes are the same; a log
  without that line is judged by its time, as before. A passing step older than an edit that cannot affect it
  is accepted with `--not-repeated STEP[,STEP...]=REASON`: the step points to the reason by number and the reason
  is printed once, with the steps it covers (rule 6: "say which runs were not repeated and why"); a failing step
  never is;
- `FAIL`: the log shows no pass (a failed test not given with `--known`, a failure the log does not name, a test
  binary that stopped without a result (a crash), a runner result with a `FAIL`, a
  call-site program that differs or was not compared, a clippy warning, a fmt difference, a note that is not the
  recorded message, a run refused for the clone's lock);
- `UNLISTED`: a new or changed program under `tests\corpus\` that is on no list (`slice.list`) and has no `.err`
  or `.norun`, so no closing run reaches it.

Each `ok` line shows when its log was written. The exit code is 0 when every step is `ok` and nothing is
`UNLISTED`.

**Not in it:** the change's new programs before they are on a list (`--glob` while working), and its
`verification` family: the runner's `--suite verification` records the old compiler only, so the new compiler's
side of a family is seen through the slice programs and front-end tests made from it. The full set at a merge
into `main` is not a change's close; a `--plan --merge` for it is not built yet (left for a next small step by
the panel, `study\archive\33`).

## The test cadence

`CLAUDE.md` working rule 6 says when to test; the details are here (moved from the rule on 2026-10-28,
`DECISIONS.md`).

**A change's closing set** (`study\archive\32` §4 item 1, user, 2026-10-17; thinned by the user on 2026-10-28,
`study\38` §5.3 item 2): what can show the change's fault.

- tier 1 with fmt and clippy, and the shrink-only lists before the build;
- the command-line tests that build (`--ignored`);
- the change's own new programs, and the programs of its `verification` family put in
  `tests\verification\pass.list` or `known.list` (`tests\verification\README.md`);
- the slice list, the upstream pass list and `tests\verification\pass.list`, with `--pass-cache target\pass-cache`
  and `--build-cache target\build-cache`;
- QB64pe's converter test (`--suite addprefix`) and QB64pe's qbasic programs, all to build (`--suite qbasic
  --jobs 8`);
- the call-site check;
- only when `close.py` finds the change touched them: the run without folding (`QB64RUST_NO_FOLD=1`, with the pass
  cache), the coverage check of Civil War Strategy's scripts, the format runs.

**Not in a change's close since 2026-10-28**, and run at a merge into `main` instead: the old compiler's wording
table (`tools\diagnostics\old_wording.py`) and its `--check`, the two demand reports (`qb64rust --report
unsupported`), the counts (`tools\legacy_tests\counts.py`). The record of test runs (`tools\testtime\test_time.py`)
is written when the user asks. *Why:* nothing is "not supported yet" any more, the counts and the wording table
change only with recordings, and the test-time record's finding (runs were repeated) is made (`study\38` §5.3).

**At each merge into `main`**, every two or three changes (`study\archive\32` §4 item 2): the full set, no pass
cache: the three full suites (corpus, upstream, verification), run with `--coverage`; QB64pe's converter test
(`--suite addprefix`) and the qbasic programs (`--suite qbasic`); the slice list, the upstream pass list and
`tests\verification\pass.list` judged from them (`--check-list`, `tools\legacy_tests\README.md`); the check of the
built-ins they reach (`tools\coverage\builtin_runs.py --check` on their counts, `tests\never_run.list`; user,
2026-10-24); the run without folding; the differential pass list; the call-site check; the coverage check of Civil
War Strategy's scripts; and the reports left out of a change's close (the wording table first, since it changes the
compiler, then the demand reports and the counts). A failure there is found among the merged changes with the
build cache. The pass cache is left out because it cannot see a result that depends on time or load. The runs may
go in a worktree of the merge commit beside this repo while the next change goes on here
(`tools\legacy_tests\README.md`, "The runs at a merge into `main` in a worktree").

**Order.** Whatever changes the compiler (the wording table, generated tables) comes before the first long run, so
that no run is thrown away for a new build. Runs of the new compiler may run side by side, and beside a recording
with `qb64pe.exe`, which has the clone alone: they build against `libqb\` and take the clone's lock only to run
programs in its own test folders (`tools\legacy_tests\README.md`, "Locks").

**Work goes on during a run** (`study\archive\34`): each run of the new compiler tests a copy of it named by hash,
so start a long run in the background and go on building, running tier 1 and editing; read its log when it ends.
Only a recording with `qb64pe.exe` waits for the clone; batch recordings before the closing runs.

**CI** runs tier 1 on a push. Its `tier2` job runs only when started by hand: the weekly run was stopped on
2026-10-28 (the runner gives screen programs no window, so it ran into its limit, `study\38` §5.3 item 5).

Its own tests: `python -m unittest discover -s tools/close -p "test_*.py"` (`test_close.py`; also run by CI).
