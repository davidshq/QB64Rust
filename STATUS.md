# Status and next steps

Updated 2026-10-19 (cut to its own rule by finding 2 of the ninth review, `study\archive\30` §2).

**This file holds the current state only**: where we are, what waits on the user, what comes next, the numbers of
the last full runs, known bugs. It holds no history. When a step is done, replace its line; do not add a paragraph
about it. **Nothing outside "Waiting on the user" asks the user anything.** Where everything else lives:

| Kind of information | Home |
|---|---|
| What a change did, task by task, and what was found on the way | the change's `tasks.md` and `design.md` under `openspec\changes\` (archived ones in `archive\`), and `git log` |
| A measured fact about the old compiler | the family's recorded output and its line in `verification\README.md`; the families up to `v45` also in `study\00` §5 (closed to new facts, `CLAUDE.md` working rule 11) |
| A decision | `DECISIONS.md`; a decided difference from QB64pe: `DIVERGENCES.md` |
| What the compiler supports | the merged specs under `openspec\specs\` (an index by area: `crates\README.md`, "What the compiler supports"); how a change built it: its `design.md` |
| How to build and test it | `crates\README.md`; per test set: `tests\*\README.md` |
| Milestones, done and not done | `ROADMAP.md` |
| History up to 2026-10-09 (what this file held before) | `study\archive\progress-record.md` (closed) |

## Where we are

M0, M1, M2 and M3 are complete (`ROADMAP.md`; M3 declared done at the merge of pull request 9, user, 2026-10-18,
tenth review `study\35` §3 item 1; the areas it left are M4's). Twenty-two M3 steps are built and archived
(table below), the last `m3-constant-evaluation` (`4f647a9`); `test-parallel-runs` and `test-while-building` (the test
cadence of `study\archive\32` and `study\archive\34`, not M3 steps, `4a8d06e` and `5d3730f`) are archived too.
Every program of the corpus that the old compiler accepts compiles, and every one that can be compared passes;
Civil War Strategy compiles with no diagnostic, builds, and played for two turns from a keys file shows the same
23 screens as the old compiler's build. Upstream is at 355 of 399. `main` is at the merge of `newer-changes` (pull
request 10, `5db547a`, 2026-10-19), which holds every change up to `libqb-object-flags` and `const-rulings`; the
extension's first release is tagged `vscode-v0.2.0` there.

## Waiting on the user

Oldest first. Each line names where the details are. A question is never accepted by default; one left open is
put again at the next merge into `main`. Not on this list: `SOMEDAY.md`'s "QB64pe behaviours to review", the
questionable behaviours each change kept QB64pe's way, which stay silent until the user asks for them (`CLAUDE.md`
working rule 11).

Nothing.

**The `tier2` job on `m3`** (`study\archive\30` §4; started by Claude on the user's word, 2026-10-16, run 38923356942 at
`acf8033`): the 236 console programs of the slice list before the screen programs passed; **the runner gives the
screen programs no window**, each waited out its 60 s limit, and the job ran into its 45-minute limit. The
design's fallback is applied in `rust.yml`: the job runs `slice.list` without its `screen/` lines (365 programs),
and the 66 screen programs run locally only. Still unseen on GitHub: whether `s75_sound_wait` and
`s76_sound_errors` run without a sound device, and the upstream and differential steps; the next run after the
push shows them (the user is not concerned with CI, working rule 9).

## Next

1. M4's parity work; the upstream report's first lines are `_DEVICES`, the `$EXEICON` relative names, blanks
   around `.` and `_CONTINUE` (2 programs each). The verification families are a suite of the new compiler since
   `verification-compare` (`tests\verification\`); its `known.list` holds more such lines.

Still to watch: `s75_sound_wait` crashed once under load (`libqb-copy`'s design, "Risks").

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
| 18 | `2026-10-17-m3-arraycopy` | `_ARRAYCOPY` in its six forms, member arrays included; D-004 widened, D-025 |
| 19 | `2026-10-17-m3-events` | `ON TIMER`, `ON KEY`, the switches, `_FREETIMER`, `KEY ON`/`OFF`/`LIST`, `KEY n, text$`; `STRIG` left for device input |
| 20 | `2026-10-17-m3-errorline-chr-const` | `CHR$` in a `CONST`, `_ERRORLINE`, `_INCLERRORLINE`, `_INCLERRORFILE$`, `_ERRORMESSAGE$`; the line of a joined line |
| 21 | `2026-10-17-m3-network-embed` | the network functions and `_SOCKETS_`, `$EMBED` and `_EMBEDDED$`, the URL functions, `afterlastline.bm` |
| 22 | `2026-10-18-m3-constant-evaluation` | the `CONST` evaluator in full: `_FLOAT` in software, its lanes and typing by text, every function (math functions certified, else D-026), suffixed constants, the auto-included constants, `$COLOR`; the dependency `astro-float-num` |

## Numbers at the last full runs

**At the merge into `main` of pull request 10** (2026-10-19, the full set on the tree of `5db547a`, no pass cache,
`--jobs 8 --build-cache`): the full corpus suite 477 pass, 5 known failures and `218_fre` (only marked errors),
`slice.list` judged from it 458 of 458, and 458 of 458 with `QB64RUST_NO_FOLD=1`; the full upstream suite 355
pass, 48 rejected or with only marked errors or, for `const/math_funcs` alone, an output that differs (D-031), 1
known failure, `pass.list` judged from it 355 of 355; the differential pass list 59 of 59; the call-site check
1888 of 1888 equal. Tier 1 and fmt and clippy passed in CI on the merge commit.

**At the merge into `main` of pull request 9** (2026-10-18, `61ae7be`; the compiler is the one of the change's
close, the commits after it touched comments and documents only): the differential pass list 59 of 59. The full
suites were left to the next merge (user, `DECISIONS.md` 2026-10-18); the close's runs below stand for the rest.

**At the close of `libqb-object-flags` and `const-rulings`** (2026-10-19, one closing set for both,
`close.py --check` passes): tier 1 388 of 388; `slice.list` 458 of 458 (`--pass-cache`), also without folding;
upstream `pass.list` 355 of 355 (`const/math_funcs` left it, D-031); the call-site check 1888 of 1888 equal; the
41 command-line tests that build pass. Demand: corpus 460 of 464 without errors (the same four as before); upstream
312 of 348 without errors, 24 held back.

**At the close of `m3-constant-evaluation`** (2026-10-18, `close.py --check` passes, again after the review's fix
of the D-026 warning): tier 1 364 of 364;
`slice.list` 458 of 458 (`--pass-cache`), also without folding; upstream `pass.list` 356 of 356; the call-site
check 1884 of 1884 equal; the 39 command-line tests that build pass. Demand: corpus 460 of 464 without errors (the
other four have files only the runner puts beside them); upstream 312 of 348 without errors, 24 held back. Of the
75 function constants of `verification\v45_b_funcs`, 72 equal the old compiler's to 19 digits and 3 take D-026
with its warning (`crates\driver\tests\consts_v45.rs`).

**At the merge into `main` of pull request 8** (2026-10-17, the full set at a merge on `5d3730f`, no pass
cache, `--jobs 8 --build-cache`): tier 1 349 of 349; the full corpus suite 472 pass, 5 known failures and
`218_fre` (only marked errors), `slice.list` judged from it (`--check-list`) 453 of 453, and 453 of 453 with
`QB64RUST_NO_FOLD=1`; the full upstream suite 347 pass, 56 rejected or with only marked errors, 1 known failure,
`pass.list` judged from it 347 of 347; the differential pass list 59 of 59; the call-site check 1884 of 1884
equal.

**At the close of `m3-network-embed`** (2026-10-17, `close.py --check` passes): `slice.list` 453 of 453
(`--pass-cache`, which skipped none: the regenerated wording table changed every executable); upstream `pass.list`
347 of 347; the call-site check 1884 of 1884 equal; the 39 command-line tests that build pass; tier 1 346 of 346.
Demand: corpus 455 of 459 without errors (the other four, `s77_exeicon`, `g12_game`, `s137_errorline_include`
and `s138_embed`, have files only the runner puts beside them); upstream 303 of 348 without errors, 33 held back.

**At the close of `m3-errorline-chr-const`** (2026-10-17, the lighter close, working rule 9; the table below is
still the last full set): `slice.list` 449 of 449 (`--pass-cache`, which skipped none: the last close ran without
it), also without folding; upstream `pass.list` 339 of 339; the call-site check 1871 of 1871 equal; the 37
command-line tests that build pass; tier 1 339 of 339 (`--workspace --locked --no-fail-fast`), after the files
of `verification\v40_rc` that the `.gitattributes` edit made CRLF were checked out again (they had LF in the
working tree, and `resource_files_equal_the_old_compilers` failed). `tools\close\close.py --check` passes.
Demand: corpus 452 of 455 without errors (the other three, `s77_exeicon`, `g12_game` and `s137_errorline_include`,
have files only the runner puts beside them); upstream 295 of 348 without errors, 42 held back.

From 2026-10-17 (the close of `m3-events`, release build against the reference clone, each runner run with
`--jobs 8 --build-cache target\build-cache`, none with `--pass-cache`): tier 1, the 37 command-line tests that
build, the two full suites (the slice list and the upstream pass list judged from them with `--check-list`), the
slice list without folding, the differential pass list, the call-site check, the demand reports, the shrink-only
lists and the old compiler's wording, all on one build. Not run: the runner's tests and those of `tools\testtime`
(untouched). The rows of the real programs, of the old compiler and of `--pass-cache` are older.

| Yardstick | Result |
|---|---|
| `tests\corpus\slice.list` (tier 2) | **446 of 446**, judged from the full corpus run (67 of them screen programs, `g12_game` the scripted game; `s134_events_timer` and `k01_on_key`, new with `m3-events`, and `s133_arraycopy_dynamic_dims` passing on their first full run; five take a row of `DIVERGENCES.md` on purpose and list it in a `.divergences` sidecar); with `QB64RUST_NO_FOLD=1` 445 of 446, `u02_mouse_text` (the mouse program a real cursor can disturb) failing once under load and passing alone |
| Full corpus (471 programs) | 465 pass (the 446 of the slice list, 19 `.err` programs), none wrong at run time; **none rejected with "not supported yet"**; 1 `.err` program with only marked errors (`218` `FRE`); 5 known failures (`tests\corpus\README.md`) |
| Upstream | **336 of 399** (`tests\upstream\pass.list`: 336, all passing in the full run; the four `timer/` programs joined with `m3-events`); the full suite: 404 programs, 336 pass, 67 rejected or with only marked errors, 1 known failure, none wrong; no program of either suite takes a row of `DIVERGENCES.md` it does not list |
| Call-site check | 1864 of 1864 programs equal (26 `event_*` new) (`tools\callsite\README.md`) |
| Real programs (`tests\programs`) | Civil War Strategy: **no diagnostic, and it builds** (3.6 s; the old compiler 6.9 s the same day). Its C++ compared once with the old compiler's, line by line: no call differs on 4,332 lines. **Played for two turns from a keys file (`g12_game`, a copy with listed edits): 23 states equal to the old compiler's**, 1.1–1.9 s a run (`tests\programs\README.md`) |
| Differential (`tests\differential\pass.list`) | 59 of 59 |
| Shrink-only lists | 0 false errors, 0 parse gaps, 28 only-marked rejections, 5 clean programs on no pass list (`tests\upstream\README.md`); regenerated at the close of `m3-events`, unchanged by it |
| Tier 1 (`cargo test`) | 338 tests pass (new with `m3-events`: two parser tests, a unit test of the IR's validation; the front-end tests `events_errors`, `_ok`, `_typed`, `_ir`, `_cpp` run within one test); the 37 command-line tests that build an executable (`--ignored`, run alone) pass too (new: `event_handlers_run`) |
| The runner's own tests (`python -m unittest discover -s tools/legacy_tests -p "test_*.py"`) | 80 pass, 1 s; those of `tools\testtime` 38 (not run: untouched) |
| `slice.list` while working, with `--pass-cache` | 24 s when no program's executable changed, 92 s when all are run (`--jobs 8 --build-cache`, 2026-10-12) |
| Screen programs (`tests\corpus\screen`, 67) | Old compiler: 67 recorded, each from two equal runs. New compiler: 67 pass; `k01_on_key` (`m3-events`) on its first run |
| Demand (`qb64rust --report unsupported`; upstream with `--include-root tests\upstream\root`) | Corpus and `tests\programs`: **nothing held back** (452 without an `.err`: 450 without errors, and `s77_exeicon` and `g12_game`, whose icons only the runner puts beside them). Upstream (348 without an `.err`): 292 without errors, 45 held back, 11 with other errors (the ten that include the old compiler's sources, and `icon_relative/test`, whose icon lies in the clone); first lines: `_OPENCLIENT`, `CHR$` in a `CONST`, `_ERRORLINE` 3 each; the event switches and handlers no longer named |
| The old compiler's wording (`tools\diagnostics\old_wording.py`) | 1,525 recorded rejections paired on the same line; 522 messages with a note (19 new with `m3-events`); `--check`: 819 notes on recorded rejections, all the recorded message |
| Tier 2 as CI runs it, locally | about 12 min at the last full build; about 2 min with `--jobs 8 --build-cache` when little C++ changed |
| Full corpus with the old compiler (2026-10-10, 336 programs) | 331 pass, 5 known failures, `--jobs 8`; the 305 before the screen programs took about 12 min with one job, 3 min 44 s with `--jobs 8` (`tools\legacy_tests\README.md`) |

`deferred.list` holds `t001` alone: a plain scalar beside an array typed by `DIM … AS`.

## Known bugs

- **A program whose C++ build fails in both compilers gets no error before the C++** (found by
  `verification-compare`, user 2026-10-19): an `$EXEICON` file that is no icon (`v29_e16_not_an_icon`), and five
  `DECLARE LIBRARY` forms whose C++ fails (`v37_x04`, `x17`, `x23`, `x30`, `x31`). The new compiler should reject
  each with an error that says why; `tests\verification\known.list` names them.

## Queued for later milestones

| When | Item |
|---|---|
| Any time | A formatter that matches `-y` (out of M2's exit criterion, `DECISIONS.md` 2026-10-09) |
| M4 | `qb64pe.bas` reached through its own include files, smallest first (`study\archive\22` §4.2) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
