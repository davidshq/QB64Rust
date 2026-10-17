# Status and next steps

Updated 2026-10-17 (cut to its own rule by finding 2 of the ninth review, `study\30` §2).

**This file holds the current state only**: where we are, what waits on the user, what comes next, the numbers of
the last full runs, known bugs. It holds no history. When a step is done, replace its line; do not add a paragraph
about it. **Nothing outside "Waiting on the user" asks the user anything.** Where everything else lives:

| Kind of information | Home |
|---|---|
| What a change did, task by task, and what was found on the way | the change's `tasks.md` and `design.md` under `openspec\changes\` (archived ones in `archive\`), and `git log` |
| A measured fact about the old compiler | `study\00` §5 (one home per fact); the measurement programs: `verification\README.md` |
| A decision | `DECISIONS.md`; a decided difference from QB64pe: `DIVERGENCES.md` |
| What the compiler supports | the merged specs under `openspec\specs\` (an index by area: `crates\README.md`, "What the compiler supports"); how a change built it: its `design.md` |
| How to build and test it | `crates\README.md`; per test set: `tests\*\README.md` |
| Milestones, done and not done | `ROADMAP.md` |
| History up to 2026-10-09 (what this file held before) | `study\progress-record.md` (closed) |

## Where we are

M0, M1 and M2 are complete (`ROADMAP.md`). **M3 is under way**: seventeen of its steps are built (table below);
the seventeenth, `m3-versioninfo`, is archived, not committed yet. Every
program of the corpus that the old compiler accepts compiles, and every one that can be compared passes; Civil
War Strategy compiles with no diagnostic, builds, and played for two turns from a keys file shows the same 23
screens as the old compiler's build. Upstream is at 328 of 399. `main` is at the merge of `m3` (pull request 5,
2026-10-16).

## Waiting on the user

Oldest first. Each line names where the details are. The list of 2026-10-16 was answered item by item that day
(`DECISIONS.md` 2026-10-16); a question is never accepted by default, so one left open is put again at each
change's close. Separate from this list and standing: `SOMEDAY.md`'s "QB64pe behaviours to review", the
questionable behaviours each change kept QB64pe's way.

1. **Merge `changes` into `main`**: the three changes of `447bb05`, then `m3-sound-handles` (`b1a818d`) and
   `m3-versioninfo` once committed, since the last merge (`DECISIONS.md` 2026-10-16, "every two or three
   changes").
2. **The next M3 step**: `m3-arraycopy` (user, 2026-10-16), measured first (`verification\v41_*`).

**The `tier2` job on `m3`** (`study\30` §4; started by Claude on the user's word, 2026-10-16, run 38923356942 at
`acf8033`): the 236 console programs of the slice list before the screen programs passed; **the runner gives the
screen programs no window**, each waited out its 60 s limit, and the job ran into its 45-minute limit. The
design's fallback is applied in `rust.yml`: the job runs `slice.list` without its `screen/` lines (365 programs),
and the 66 screen programs run locally only. Still unseen on GitHub: whether `s75_sound_wait` and
`s76_sound_errors` run without a sound device, and the upstream and differential steps; the next run after the
push shows them (the user is not concerned with CI, working rule 9).

## Next

`m3` goes to `main` every two or three changes; five wait now. Then `m3-arraycopy` (user, `DECISIONS.md`
2026-10-16), measured first; the upstream report's other first lines are the event handlers and switches (4 each); the plain copy of `internal\c` waits until libqb
must change.

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
| 12 | `2026-10-16-m3-declare-library` | `DECLARE LIBRARY` in its four forms on Windows; D-023 and D-024 not built |
| 13 | `2026-10-16-m3-measured-copies` | what owner types copy after an error done QB64pe's way where no memory is misused (`v38`); D-019, D-021, D-022 narrowed |
| 14 | `2026-10-16-m3-divergence-trace` | the run-time trace of divergences (`QB64RUST_DIVERGENCES`), checked by the runner and the CLI tests |
| 15 | `2026-10-16-m3-old-wording-notes` | the old compiler's wording as a note after an error, from recorded output |
| 16 | `2026-10-16-m3-sound-handles` | `_SNDOPEN`, the `_SND*` family, `_MIDISOUNDBANK`, `_WAVE`: every name of the audio part; `_MEMSOUND` run |
| 17 | `2026-10-17-m3-versioninfo` | `$VERSIONINFO`: the version resource and manifest byte for byte; a compile error where the old build fails or its executable cannot start |

## Numbers at the last full runs

From 2026-10-16 (the close of `m3-versioninfo`, release build against the reference clone, each runner run with
`--jobs 8 --build-cache target\build-cache`, none with `--pass-cache`): tier 1, the 33 command-line tests that
build, the slice list without folding, the differential pass list, the two full suites (the slice list and the
upstream pass list judged from them with `--check-list`), the call-site check, the demand reports, the shrink-only
lists and the old compiler's wording. The table of the old wording was regenerated after the full suites, so the
release build they ran differs from the last one by that table alone (notes after errors; `--check` and tier 1 ran
on the last). Not run again: the runner's tests and those of `tools\testtime` (untouched). The rows of the real
programs, of the old compiler and of `--pass-cache` are older.

| Yardstick | Result |
|---|---|
| `tests\corpus\slice.list` (tier 2) | **441 of 441**, judged from the full corpus run (66 of them screen programs, `g12_game` the scripted game; `s130_versioninfo`, new with `m3-versioninfo`, passing on its first run; five take a row of `DIVERGENCES.md` on purpose and list it in a `.divergences` sidecar); with `QB64RUST_NO_FOLD=1` 440 of 441 at the first run, `s127_sound_handles` ending with an access violation after its last line (the intermittent crash recorded in `m3-sound-handles`'s design, also seen with the old compiler's build), then passing when run again |
| Full corpus (466 programs) | 460 pass (the 441 of the slice list, 19 `.err` programs), none wrong at run time; **none rejected with "not supported yet"**; 1 `.err` program with only marked errors (`218` `FRE`); 5 known failures (`tests\corpus\README.md`) |
| Upstream | **328 of 399** (`tests\upstream\pass.list`: 328, all passing, judged from the full run; `versioninfo`, `versioninfo2`, `versioninfo3` joined with `m3-versioninfo`); the full suite: 404 programs, 328 pass, 75 rejected or with only marked errors, 1 known failure, none wrong; no program of either suite takes a row of `DIVERGENCES.md` it does not list |
| Call-site check | 1808 of 1808 programs equal (`tools\callsite\README.md`) |
| Real programs (`tests\programs`) | Civil War Strategy: **no diagnostic, and it builds** (3.6 s; the old compiler 6.9 s the same day). Its C++ compared once with the old compiler's, line by line: no call differs on 4,332 lines. **Played for two turns from a keys file (`g12_game`, a copy with listed edits): 23 states equal to the old compiler's**, 1.1–1.9 s a run (`tests\programs\README.md`) |
| Differential (`tests\differential\pass.list`) | 59 of 59 |
| Shrink-only lists | 0 false errors, 0 parse gaps, 28 only-marked rejections, 5 clean programs on no pass list (`tests\upstream\README.md`); regenerated at the close of `m3-versioninfo`, unchanged by it |
| Tier 1 (`cargo test`) | 333 tests pass (new with `m3-versioninfo`: the front-end tests `versioninfo_errors`, `_ok`, `_ir`, the driver test over `verification\v40_rc\`, a CLI test of the warning, a unit test of the cache key); the 33 command-line tests that build an executable (`--ignored`, run alone) pass too |
| The runner's own tests (`python -m unittest discover -s tools/legacy_tests -p "test_*.py"`) | 80 pass, 1 s; those of `tools\testtime` 38 (not run: untouched) |
| `slice.list` while working, with `--pass-cache` | 24 s when no program's executable changed, 92 s when all are run (`--jobs 8 --build-cache`, 2026-10-12) |
| Screen programs (`tests\corpus\screen`, 66) | Old compiler: 66 recorded, each from two equal runs. New compiler: 66 pass; the 12 of `m3-images-fonts-mouse` (`i01`–`i08`, `f01`, `f02`, `u01`, `u02`) each on its first run (`u01` and `u02` were recorded again after the fix of the mouse steps' wait, task 4.1) |
| Demand (`qb64rust --report unsupported`; upstream with `--include-root tests\upstream\root`) | Corpus and `tests\programs`: **nothing held back** (447 without an `.err`: 445 without errors, and `s77_exeicon` and `g12_game`, whose icons only the runner puts beside them). Upstream (348 without an `.err`): 284 without errors, 53 held back, 11 with other errors (the ten that include the old compiler's sources, and `icon_relative/test`, whose icon lies in the clone); no sound built-in is named; first lines: `_ARRAYCOPY` 4, the event switches and handlers 4 each; `$VERSIONINFO` no longer named |
| The old compiler's wording (`tools\diagnostics\old_wording.py`) | 1,426 recorded rejections paired on the same line; 466 messages with a note; `--check`: 743 notes on recorded rejections, all the recorded message |
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
