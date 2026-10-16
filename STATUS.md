# Status and next steps

Updated 2026-10-16 (cut to its own rule by finding 2 of the ninth review, `study\30` §2).

**This file holds the current state only**: where we are, what waits on the user, what comes next, the numbers of
the last full runs, known bugs. It holds no history. When a step is done, replace its line; do not add a paragraph
about it. **Nothing outside "Waiting on the user" asks the user anything.** Where everything else lives:

| Kind of information | Home |
|---|---|
| What a change did, task by task, and what was found on the way | the change's `tasks.md` and `design.md` under `openspec\changes\` (archived ones in `archive\`), and `git log` |
| A measured fact about the old compiler | `study\00` §5 (one home per fact); the measurement programs: `verification\README.md` |
| A decision | `DECISIONS.md`; a decided difference from QB64pe: `DIVERGENCES.md` |
| What the compiler supports, how to build and test it | `crates\README.md`, the merged specs under `openspec\specs\`; per test set: `tests\*\README.md` |
| Milestones, done and not done | `ROADMAP.md` |
| History up to 2026-10-09 (what this file held before) | `study\progress-record.md` (closed) |

## Where we are

M0, M1 and M2 are complete (`ROADMAP.md`). **M3 is under way**: eleven of its steps are done, committed and
archived on branch `m3` (table below). Every program of the corpus that the old compiler accepts compiles, and
every one that can be compared passes; Civil War Strategy compiles with no diagnostic, builds, and played for two
turns from a keys file shows the same 23 screens as the old compiler's build. Upstream is at 310 of 399. The
twelfth step, `DECLARE LIBRARY` (OpenSpec change `m3-declare-library`), is under way: groups 1 to 3 are done (the
measurements `verification\v37_*`, the blocks and calls in `sema`, the IR and the C++). `main` is at the merge of
`m2-console-screenhide` (pull request 4, 2026-10-13; every workflow green there, seen 2026-10-13); `m3` has not
been merged.

## Waiting on the user

Oldest first. Each line names where the details are. The list of 2026-10-16 was answered item by item that day
(`DECISIONS.md` 2026-10-16); a question is never accepted by default, so one left open is put again at each
change's close. Separate from this list and standing: `SOMEDAY.md`'s "QB64pe behaviours to review", the
questionable behaviours each change kept QB64pe's way.

Nothing waiting.

**The `tier2` job on `m3`** (`study\30` §4; started by Claude on the user's word, 2026-10-16, run 38923356942 at
`acf8033`): the 236 console programs of the slice list before the screen programs passed; **the runner gives the
screen programs no window**, each waited out its 60 s limit, and the job ran into its 45-minute limit. The
design's fallback is applied in `rust.yml`: the job runs `slice.list` without its `screen/` lines (365 programs),
and the 66 screen programs run locally only. Still unseen on GitHub: whether `s75_sound_wait` and
`s76_sound_errors` run without a sound device, and the upstream and differential steps; the next run after the
push shows them (the user is not concerned with CI, working rule 9).

## Next

**Decided 2026-10-16 and not built yet** (`DECISIONS.md` 2026-10-16), in this order:

1. **`m3-declare-library`, reworked for the answer on D-023 and D-024**: a header included once per block and a
   name looked up only in the block's last library, as the old compiler does (task 3.2 changes; its `proposal.md`
   says how); then group 4, the lookup of the library files and the build in the driver.
2. **Three measurements, each then done QB64pe's way unless it misuses memory**: the unpack after a `GET` whose
   read raised an error (D-021); the dimension count in the bounds check of a file's `_DYNAMIC` member
   (`m3-typefields`); the dimension count of a dynamic member array's descriptor in a whole-array copy
   (`m3-whole-array-assign` tasks 3.1).
3. **D-022 narrowed**: QB64pe's copy where nothing is read past the source; nothing copied only where it would be.
4. **A run-time trace of divergences**: an environment variable makes the runtime print one line when a D-row's
   path is taken; the test runner sets it for every run and fails on a line it does not expect. Every D-row with
   a run-time path.
5. **Merge `m3` into `main`** at the close of `m3-declare-library` (a pull request for the user to merge), then
   every two or three changes.
6. **Its own change after `m3-declare-library`**: every diagnostic whose wording differs from the old compiler's
   carries the old wording as a note, from recorded output.

After the change, the upstream report's first lines are `_SNDOPEN` (6), `_ARRAYCOPY` (4, deferred),
`$VERSIONINFO` (4), the event handlers and switches (4 each), `_MIDISOUNDBANK` (4) and `LEN` of a whole array (4);
the plain copy of `internal\c` waits until libqb must change.

**M3's steps** (user, `DECISIONS.md` 2026-10-12, and one row per later step). Done, each archived under
`openspec\changes\archive\` with its `tasks.md` and `design.md`; its measurements in `study\00` §5; what stayed
marked in `SOMEDAY.md` ("Left marked by …"):

| Step | Change (archive folder) | What it brought |
|---|---|---|
| 1 | `2026-10-12-m3-window-output` | a program with a window; text-screen, drawing and pacing statements; screen functions |
| 2 | `2026-10-13-m3-get-put-bload` | graphics `GET`/`PUT`, `DEF SEG`, `BLOAD`/`BSAVE`, `TAB`, `SPC`, labels named like a SUB |
| 3 | `2026-10-13-m3-sound` | `SOUND`, `PLAY`, `BEEP`, `$EXEICON`, a template after `CALL`; the whole game builds |
| 4 | `2026-10-14-m3-console-files` | `PRINT USING`, random and binary files, `FIELD`, `LSET`, `RSET`, `_IIF`, `_MIN` … `_ROR` |
| 5 | `2026-10-14-m3-scripted-game` | Civil War Strategy's screens in a scripted game (`g12_game`) |
| 6 | `2026-10-14-m3-images-fonts-mouse` | images, colours, fonts, the mouse |
| 7 | `2026-10-14-m3-mem` | `_MEM` and its statements, `MKI$` … `_CV` |
| 8 | `2026-10-14-m3-asserts-debug-zlib` | `$ASSERTS`, `_ASSERT`, `$DEBUG`, checksums, compression, base64, `SYSTEM n`, the licence file |
| 9 | `2026-10-14-m3-redim-retain` | `REDIM _RETAIN`; D-018 |
| 10 | `2026-10-15-m3-typefields` | `$UNSTABLE:TYPEFIELDS`, member markers, `_DYNAMIC` member arrays; the arrays used after a marked declaration (`DECISIONS.md` 2026-10-14, item 1) are no longer named by the demand report |
| 11 | `2026-10-15-m3-whole-array-assign` | whole-array assignment `a() = b()` |
| 12 | `m3-declare-library` (open) | `DECLARE LIBRARY`: groups 1 to 3 done, group 4 next |

## Numbers at the last full runs

From 2026-10-15 (task 5.3 of `m3-whole-array-assign`, release build against the reference clone, each runner run
with `--jobs 8 --build-cache target\build-cache`): tier 1 (at the close of group 4, after the last change to the
compiler; only documents changed after it), the command-line tests that build, the runner's tests, the slice
list with and without folding, the upstream and differential pass lists, the two full suites and the demand
reports, each run once after the last change to the compiler. The call-site check is from
`m3-asserts-debug-zlib` (no row of the built-in table changed since; this change adds none). The rows of the real programs, of the old
compiler and of `--pass-cache` are older (sessions 56, 40 and 54).

| Yardstick | Result |
|---|---|
| `tests\corpus\slice.list` (tier 2) | **431 of 431** (66 of them screen programs, `g12_game` the scripted game; `s119_whole_array` and `s120_whole_array_members`, new with `m3-whole-array-assign`, each passing on its first run); 431 of 431 also with `QB64RUST_NO_FOLD=1` |
| Full corpus (456 programs) | 450 pass (the 431 of the slice list, 19 `.err` programs), none wrong at run time; **none rejected with "not supported yet"**; 1 `.err` program with only marked errors (`218` `FRE`); 5 known failures (`tests\corpus\README.md`) |
| Upstream | **310 of 399** (`tests\upstream\pass.list`: 310, all passing, run by itself; 16 programs left `deferred.list` with `m3-whole-array-assign`, so 399 are counted); the full suite: 404 programs, 310 pass, 93 rejected or with only marked errors (80 at the compile, 13 `.err` programs), 1 known failure, none wrong |
| Call-site check | 1677 of 1677 programs equal (`tools\callsite\README.md`; 46 of them for `m3-asserts-debug-zlib`: `_ASSERT` 18, the data-processing functions 28) |
| Real programs (`tests\programs`) | Civil War Strategy: **no diagnostic, and it builds** (3.6 s; the old compiler 6.9 s the same day). Its C++ compared once with the old compiler's, line by line: no call differs on 4,332 lines. **Played for two turns from a keys file (`g12_game`, a copy with listed edits): 23 states equal to the old compiler's**, 1.1–1.9 s a run (`tests\programs\README.md`) |
| Differential (`tests\differential\pass.list`) | 59 of 59 |
| Shrink-only lists | 0 false errors, 0 parse gaps, 28 only-marked rejections (the eight `.err` programs of whole-array assignment left), 5 clean programs on no pass list (`tests\upstream\README.md`); regenerated at the close |
| Tier 1 (`cargo test`) | 311 tests pass; the 27 command-line tests that build an executable (`--ignored`, run alone) pass too (2 new with `m3-whole-array-assign`: D-004 widened, D-022) |
| The runner's own tests (`python -m unittest discover -s tools/legacy_tests -p "test_*.py"`) | 73 pass, 1 s; those of `tools\testtime` 38 (not run with this change, which does not touch it) |
| `slice.list` while working, with `--pass-cache` | 24 s when no program's executable changed, 92 s when all are run (`--jobs 8 --build-cache`, 2026-10-12) |
| Screen programs (`tests\corpus\screen`, 66) | Old compiler: 66 recorded, each from two equal runs. New compiler: 66 pass; the 12 of `m3-images-fonts-mouse` (`i01`–`i08`, `f01`, `f02`, `u01`, `u02`) each on its first run (`u01` and `u02` were recorded again after the fix of the mouse steps' wait, task 4.1) |
| Demand (`qb64rust --report unsupported`; upstream with `--include-root tests\upstream\root`) | Corpus: **nothing held back** (436 without an `.err`: 434 without errors, and `s77_exeicon` and `g12_game`, whose icons only the runner puts beside them). Upstream (348 without an `.err`): 266 without errors, 71 held back, 11 with other errors (the ten that include the old compiler's sources, and `icon_relative/test`, whose icon lies in the clone); no whole-array assignment is named (the whole arrays it names are `LEN(a())`, 4 programs); first lines: `DECLARE LIBRARY` 9, `_SNDOPEN` 6, `_ARRAYCOPY` 4 |
| Tier 2 as CI runs it, locally | about 12 min at the last full build; about 2 min with `--jobs 8 --build-cache` when little C++ changed |
| Full corpus with the old compiler (2026-10-10, 336 programs) | 331 pass, 5 known failures, `--jobs 8`; the 305 before the screen programs took about 12 min with one job, 3 min 44 s with `--jobs 8` (`tools\legacy_tests\README.md`) |

What `deferred.list` still holds needs `_ARRAYCOPY` or, `t001`, a plain scalar beside an array typed by `DIM …
AS`.

## Known bugs

None known.

## Queued for later milestones

| When | Item |
|---|---|
| M3 | Plain copy of `..\QB64pe\internal\c` at the reference clone's commit, `16f629784e` (`DECISIONS.md` 2026-10-02 and 2026-10-09) |
| Any time | A formatter that matches `-y` (out of M2's exit criterion, `DECISIONS.md` 2026-10-09) |
| M4 | `qb64pe.bas` reached through its own include files, smallest first (`study\22` §4.2) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
