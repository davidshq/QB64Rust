# Status and next steps

Updated 2026-10-24 (cut to its own rule by finding 2 of the ninth review, `study\archive\30` §2).

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
23 screens as the old compiler's build; its scripts are whole games now, held to line coverage
(`cws-whole-game`). Upstream is at 401 of 404 (the three left are out of scope: `http/read_example`,
`opengl/test`, `const/math_funcs`); the verification families are a suite of the new compiler, 2959 programs on
`tests\verification\pass.list`. `main` is at the merge of `m4-finish` (pull request 14, `82afd7f`, 2026-10-24),
which holds M4's fourteen changes so far: through pull request 12 (`e0ac8c6`) `m4-checker-groups`,
`m4-devices-continue-paths`, `m4-names-arrays`, `m4-builtins-param-types`, `dry-cleanup`,
`m4-handlers-logging-checking`, `m4-upstream-rejections` and `m4-qb64pe-sources` (the full set at a merge ran on
`fe5e2de`, below), through pull request 13 (`fe596c3`) `m4-qb64pe-builtins`, `m4-line-numbers-whole-arrays`,
`m4-names-implicit-arrays` and `m4-memory-records`, and through pull request 14 `m4-statements-metacommands` and
`m4-builtins-remaining` (the full set at a merge ran on the tree of pull request 14, which stands for both, below).
Branch `m4-sub-gl` holds step 5, `m4-sub-gl`, archived. The extension's first release is tagged `vscode-v0.2.0`.

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

1. **Step 5 of M4's plan (`DECISIONS.md` 2026-10-22), `SUB _GL`, is built and archived** (`m4-sub-gl`, on branch
   `m4-sub-gl`; spec `language/opengl`). `opengl/test` passes (upstream 402 of
   404, the two left out of scope: `http/read_example`, `const/math_funcs`); the upstream demand report marks
   nothing "not supported yet".
2. **The forms steps 2 and 3 left marked are built and archived** (`m4-left-marked`, on branch `m4-sub-gl`):
   `tests\verification\known.list` holds no "not supported yet" reason; `v56_a5` and `v60_d6` take D-037, `v60_c3`
   D-004.
3. **Step 6, the coverage review, is built** (`m4-coverage-review`, on branch `m4-sub-gl`, waiting for the user's
   review before it is archived): the built-ins no test runs are `tests\never_run.list` (15, each with its
   reason), checked by `tools\coverage\builtin_runs.py` on the counts of the full suites run with `--coverage`.
   Then the full set at a merge into `main`, with those coverage runs for the first time; M4 is declared done by
   the user at that merge.

Still to watch: `s75_sound_wait` crashed once under load (`libqb-copy`'s design, "Risks").
`screen/g12_game.files` (a script of Civil War Strategy) times out (exit 124 after 30 s, "no prompt in time" at
line 97 of its keys file) in every run of the slice list or the full corpus at `--jobs 8` since 2026-10-21: in
the full set at the merge of pull request 12 the full corpus twice, the slice list and the slice list without
folding (2026-10-22). The game hangs in `SHELL "copy cws.his oldhist.ory"`: libqb starts `cmd /c copy …` through
`ShellExecuteEx` with a console of its own (`libqb\internal\c\libqb\src\shell.cpp`, `sub_shell`), and that
`cmd.exe` neither copies nor ends until the game is killed at the timeout (about 30 s, watched once a second); the
runner's retry, a moment later, still finds the script's folder in use and fails with `PermissionError`. The script passes run alone
(3.1 s), as the first of 61 programs at `--jobs 8`, with its nine sibling scripts at `--jobs 8`, and in the
coverage check; the failure comes only late in a long run, so it is taken as the test environment's, not the
compiler's. Not yet known: what the `cmd.exe` waits on.

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

**For the merge into `main` of `m4-finish`, and of pull request 13** (2026-10-24, the full set on the tree of
`91172cc` with 4b archived, which is the tree of a merge with `main` at `fe596c3`, in this checkout, no pass cache,
`--jobs 8 --build-cache`): tier 1 passes, fmt and clippy clean; the full corpus suite 510 pass, 5 known failures,
1 failure, `screen/c15_resize` (the desktop resizes the window, the old compiler's build too, "Known bugs");
`g12_game.files` passed; `slice.list` judged from it 490 of 491, and 490 of 491 with `QB64RUST_NO_FOLD=1` (the same
program); the full upstream suite 401 pass, 2 failures out of scope (`const/math_funcs`, D-031; `opengl/test`,
`SUB _GL`), 1 known failure, `pass.list` judged from it 401 of 401; the full verification suite 3771 pass, 70 known
failures, `pass.list` judged from it 3771 of 3771, none of `known.list` passing; the differential pass list 59 of
59; the call-site check 2312 of 2312 equal; the coverage check of Civil War Strategy's scripts passes.

**At the merge into `main` of pull request 12, again** (2026-10-22, the full set on `fe5e2de`, the tree of the
merge, in this checkout, no pass cache, `--jobs 8 --build-cache`): tier 1 408 of 408, fmt and clippy clean; the
full corpus suite 495 pass, 5 known failures, `218_fre` (only marked errors) and `g12_game.files` (the timeout of
"Still to watch"; the same in a second run with nothing beside it), `slice.list` judged from it 476 of 477, and 476
of 477 with `QB64RUST_NO_FOLD=1` (the same script); the full upstream suite 401 pass, 2 rejected or with only marked
errors or, for `const/math_funcs`, an output that differs (D-031), 1 known failure, `pass.list` judged from it 401
of 401; the full verification suite 2960 pass, 196 known failures, `pass.list` judged from it 2959 of 2959
(`v29_b_sound_wait` passed on `known.list` and stays there: SOUND's time varies); the differential pass list 59 of
59; the call-site check 2074 of 2074 equal; the coverage check of Civil War Strategy's scripts passes.

**At the merge into `main` of pull request 12** (2026-10-21, the full set on the tree of `6fcaab0`, which is the
merged tree, in a worktree, no pass cache, `--jobs 8 --build-cache`): the full corpus suite 495 pass, 5 known
failures and `218_fre` (only marked errors), `slice.list` judged from it 476 of 476, and 476 of 476 with
`QB64RUST_NO_FOLD=1`; the full upstream suite 379 pass, 24 rejected or with only marked errors or, for
`const/math_funcs` alone, an output that differs (D-031), 1 known failure, `pass.list` judged from it 378 of 378
(`arrays/45_err_solved_bug_Incorrect_nr_of_args` passed off the list and joined it: 379); the full verification
suite 2793 pass, 205 known failures, `pass.list` judged from it 2790 of 2790 (`v28_x89_on_strig_gosub_sub_name`
passed on `known.list` and moved: 2791; `v42_a_timer` passed only alone, timing-sensitive); the differential pass
list 59 of 59; the call-site check 2036 of 2036 equal; the coverage check of Civil War Strategy's scripts passes.

**At the merge into `main` of pull request 11** (2026-10-20, `c8ac126`): the full set was not run; the closing set
of `cpp-failure-errors`, the last change in it, on the merged compiler (`close.py --check` passes) stands for it:
tier 1 398 of 398; `slice.list` 469 of 469, upstream `pass.list` 355 of 355 and `tests\verification\pass.list`
2628 of 2628 (`--pass-cache`); the call-site check 1888 of 1888 equal; the 43 command-line tests that build pass;
the old compiler's wording 877 notes, all the recorded message.

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

- `tests\corpus\screen\c15_resize` (`$RESIZE:ON`, nobody resizes the window) was recorded with `_RESIZE`,
  `_RESIZEWIDTH` and `_RESIZEHEIGHT` 0 on every line. In the full set of 2026-10-24 the first run gave
  `-1  640  400` after the half-second wait, in the full run and alone, and the old compiler's own build of the
  program (in a scratch folder) gave the same: the machine's desktop now resizes the window once after it opens.
  Not the compiler's; the program's premise (a window nobody resized reads 0) holds on this machine no longer.

- `verification\v39_a_handles` recorded `_SNDGETPOS` and `_SNDLEN` of a raw sound at a sound device of 44100 Hz
  (`.1249886602163315`); at the close of `m4-builtins-remaining` (2026-10-24) the machine's device gave `.125`
  (48000 Hz), from the old compiler's build too (recorded again in a scratch folder), so the program moved to
  `tests\verification\known.list` as `varies`, beside `v39_b_new_mem`, which went the same way earlier. Not the
  compiler's.
- `tests\corpus\slice\s150_builtins_more` stores and reads the system clipboard, so it can fail when another run
  uses the clipboard at the same time (seen once in the no-fold run at the close of `m4-qb64pe-sources`: `LEN` of
  `"a" + CHR$(0) + "b"` read back as 3, not 1; it passed alone). Run the no-fold list by itself, not beside
  another slice run. At the close of `m4-line-numbers-whole-arrays` (2026-10-22) it failed on every run, also alone,
  and so did the old compiler's own `verification\v49_d_clipboard.exe`: libqb's `_CLIPBOARD$` gives its internal copy
  (all 3 bytes) whenever `OpenClipboard` fails, so any program of the machine that opens the clipboard after a
  change (a clipboard history) is enough. Not the compiler's.

## Queued for later milestones

| When | Item |
|---|---|
| Any time | A formatter that matches `-y` (out of M2's exit criterion, `DECISIONS.md` 2026-10-09) |
| M4 | `qb64pe.bas` reached through its own include files, smallest first (`study\archive\22` §4.2) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
