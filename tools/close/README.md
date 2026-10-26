# The closing runs of a change

`close.py` holds the closing set of `CLAUDE.md` working rule 9 as a plan and a check. It runs nothing: each test
run is started by its own command, so that `tools\testtime` sees it (rule 9, "Every test run is on record").

```
python tools/close/close.py --plan                  # the commands, in order, each with its log under target/close/
python tools/close/close.py --check                 # each step: ran on the current build, and passed?
python tools/close/close.py --check --known <test>  # a failing test that is known and not the change's
python tools/close/close.py --check --not-repeated slice,upstream=<why>   # older than an edit that cannot affect them
```

**The plan** (`--plan`) prints, for the working tree as it stands, Bash commands (Git Bash or the Bash tool, not
PowerShell): the old compiler's wording table, the shrink-only lists and the release build first (rule 9, "Order":
what changes the compiler or what tier 1 reads before the first long run); tier 1 as CI runs it, `--workspace
--locked`, with `--no-fail-fast` (without it `cargo test` stops at the first failing test binary and the tests
after it never run), fmt, clippy and the wording check; the slice list and the upstream pass list with `--pass-cache` and
`--build-cache`, side by side; the run without folding, only when a file of `sema`'s constant evaluation, literals
or operators changed against `HEAD` (`--nofold` asks for it when a numeric conversion elsewhere changed); the
coverage check of Civil War Strategy's scripts (`tools\coverage\coverage.py --check`), only when the game's copy or
its scripts (`tests\corpus\screen\g12_game.*`), its list of unreached lines, `tools\screen\cut_game.py`,
`screen_state.cpp`, `tools\coverage\`, `crates\codegen-cpp\`, `crates\ir\` (which statements are counted) or
`crates\driver\` (the record's files) changed (`--coverage` asks for it anyway); QB64pe's format tests
(`--suite format`) and the format oracle (`--suite format-corpus`, `tests\format\`) with the new compiler, only when
`crates\format\`, `tests\format\`, `tools\format\`, the layout settings (`base`'s `layout.rs`), `crates\syntax\`,
`crates\builtins\`, `sema`'s symbols (`symbols.rs`, `check\places.rs`, `check\decl.rs`) or the driver's `-y`
(`main.rs`, `lib.rs`) changed (`--format` asks for them anyway); the
call-site check and the run without folding with `--wait` (the call-site check needs the clone alone); the
command-line tests that build, alone; the two demand reports; the counts. Then `--check`, then
`tools\testtime\test_time.py`.

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
  is printed once, with the steps it covers (rule 9: "say which runs were not repeated and why"); a failing step
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
into `main` (rule 9) is not a change's close; a `--plan --merge` for it is not built yet (left for a next small
step by the panel, `study\archive\33`).

Its own tests: `python -m unittest discover -s tools/close -p "test_*.py"` (`test_close.py`; also run by CI).
