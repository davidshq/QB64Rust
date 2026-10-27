# Status and next steps

Updated 2026-10-27 (cut to its own rule by finding 2 of the ninth review, `study\archive\30` §2).

**This file holds the current state only**: where we are, what waits on the user, what comes next, the numbers of
the last full runs, known bugs. It holds no history. When a step is done, replace its line; do not add a paragraph
about it. **Nothing outside "Waiting on the user" asks the user anything.** Where everything else lives:

| Kind of information | Home |
|---|---|
| What a change did, task by task, and what was found on the way | the change's `tasks.md` and `design.md` under `openspec\changes\` (archived ones in `archive\`), and `git log` |
| A measured fact about the old compiler | the family's recorded output and its line in `verification\README.md`; the families up to `v45` also in `study\00` §5 (closed to new facts, `CLAUDE.md` working rule 11) |
| A decision | `DECISIONS.md`; a decided difference from QB64pe: `QB64PE-DIVERGENCES.md` |
| What the compiler supports | the merged specs under `openspec\specs\` (an index by area: `crates\README.md`, "What the compiler supports"); how a change built it: its `design.md` |
| How to build and test it | `crates\README.md`; per test set: `tests\*\README.md` |
| Milestones, done and not done | `ROADMAP.md` (open), `ROADMAP-COMPLETED.md` (done) |
| History up to 2026-10-09 (what this file held before) | `study\archive\progress-record.md` (closed) |

## Where we are

M0 to M5 are complete (`ROADMAP-COMPLETED.md`; M3 declared done at the merge of pull request 9, user, 2026-10-18,
tenth review `study\35` §3 item 1; M4 at the merge of branch `m4-sub-gl`, user, 2026-10-24, `DECISIONS.md`, what
its list left open moved to `SOMEDAY.md`; M5, the debugger, at the merge of pull request 17, user, 2026-10-26;
M5b, the formatter and the language-server features, at the merge of pull request 18, 2026-10-26). M6 has begun:
`m6-toolchain` is archived, `m6-compiler-download` built (waits for review). Twenty-two M3 steps are built and archived
(table below), the last `m3-constant-evaluation` (`4f647a9`); `test-parallel-runs` and `test-while-building` (the test
cadence of `study\archive\32` and `study\archive\34`, not M3 steps, `4a8d06e` and `5d3730f`) are archived too.
Every program of the corpus that the old compiler accepts compiles, and every one that can be compared passes;
Civil War Strategy compiles with no diagnostic, builds, and played for two turns from a keys file shows the same
23 screens as the old compiler's build; its scripts are whole games now, held to line coverage
(`cws-whole-game`). Upstream is at 402 of 404 (the two left are out of scope: `http/read_example`,
`const/math_funcs`); the verification families are a suite of the new compiler, 3890 programs on
`tests\verification\pass.list`. M4's first fourteen changes went into `main` through pull requests 12 to 14
(`ROADMAP-COMPLETED.md`, M4). Pull request 15 (`311c0bb`, 2026-10-25) merged branch `m4-sub-gl`, M4's last three changes (`m4-sub-gl`,
`m4-left-marked`, `m4-coverage-review`). Pull request 17 (`a81180a`, 2026-10-26) merged branch `m5`: M5's three
changes (`m5-debug-build-symbols`, `m5-dap-proxy`, `m5-debug-launch`) and `m5b-formatter`. Pull request 18
(`fa91454`, 2026-10-26) merged branch `mb5`: `m5b-format-settings` and `m5b-language-features`. Branch
`more-stuff` holds since `qbasic-testcases` (all 143 of QB64pe's qbasic programs build), `m6-toolchain` (both
archived), the QB 4.5 oracle (`tools\qb45`), study 39 and the purpose; it is not yet in `main`.
The extension's first release is tagged `vscode-v0.2.0`.

## Waiting on the user

Oldest first. Each line names where the details are. A question is never accepted by default; one left open is
put again at the next merge into `main`. Not on this list: `SOMEDAY.md`'s "QB64pe behaviours to review", the
questionable behaviours each change kept QB64pe's way, which stay silent until the user asks for them (`CLAUDE.md`
working rule 11).

1. **The project's direction** (`study\38` §7, 2026-10-26): the purpose is decided, a QB64 for anyone
   (`ROADMAP.md` "Purpose", `DECISIONS.md` 2026-10-26); still open: the release work of §5.2 (usable
   without QB64pe is decided and built: `m6-toolchain`, `DECISIONS.md` 2026-10-26; the compiler as its own
   download is decided and built, `m6-compiler-download`, first published at the next `vscode-v*` tag; still open: the `UPSTREAM.md` issues; the
   Marketplace, yes or no); the cuts
   of §5.3 (the policy on QB64pe releases is decided: frozen at 4.7.0, `DECISIONS.md` 2026-10-26; the 165 rows stay, as candidates
   for fixes behind a switch; still open: a thinner close, merged documents, the weekly CI job); the direction after that (§5.4,
   P2's row unless the user chooses otherwise). It answers `study\37` §8 (what M6 becomes) too.

**The `tier2` job on `m3`** (`study\archive\30` §4; started by Claude on the user's word, 2026-10-16, run 38923356942 at
`acf8033`): the 236 console programs of the slice list before the screen programs passed; **the runner gives the
screen programs no window**, each waited out its 60 s limit, and the job ran into its 45-minute limit. The
design's fallback is applied in `rust.yml`: the job runs `slice.list` without its `screen/` lines (365 programs),
and the 66 screen programs run locally only. Still unseen on GitHub: whether `s75_sound_wait` and
`s76_sound_errors` run without a sound device, and the upstream and differential steps; the next run after the
push shows them (the user is not concerned with CI, working rule 9).

## Next

1. The change `m6-compiler-download` (`openspec\changes\m6-compiler-download`) is built and its closing runs
   pass (`close.py --check`, 2026-10-27): the compiler as a zip on the `vscode-v*` release beside the `.vsix`,
   libqb found beside the executable, `--version` and `--help`, `MANUAL.md`, the workspace version 0.2.0. It waits
   for the user's review, then `/opsx:archive`. The first release with the zip is the user's next `vscode-v*` tag,
   with `package.json` and `Cargo.toml` raised together.
2. M6, runtime modernisation (`ROADMAP.md`): a study first, `study\37` (what M6 is for, where it starts and where
   it stops, with the review that comes with this merge), then the user's decision on its scope.
3. The fixes switch and the crash-class rulings (`study\39`, rows 1–19, user, 2026-10-26): `qb64rust.toml` and
   the flags, the always-on fixes and the switched ones, each with its measurement under QB64pe first. Not started;
   a change (or two: the switch with the codegen fixes, then the libqb edits) proposed when it starts.

Still to watch: `s75_sound_wait` crashed once under load (`libqb-copy`'s design, "Risks").
`lldb` and `lldb-dap` debug through `lldb-server` since `m5-dap-proxy` (`DECISIONS.md` 2026-10-25): its race after a
single step ends about one session in seventy at once ("lost connection"; 3 in 222 runs of
`debug_next_steps_by_line` under load), so a debugger CLI test can fail so now and then; run it again alone. The
cause is `lldb-server`'s (`openspec\changes\archive\2026-10-25-m5-dap-proxy\design.md`, "Measured").
`slice/s150_builtins_more` and `verification\v49_d_clipboard` read `LEN(_CLIPBOARD$)` of `"a" + CHR$(0) + "b"` as
3 where 1 is recorded, in the closing runs of `m5-debug-build-symbols` (2026-10-25; the committed `HEAD`'s build
too): libqb's read (`libqb\internal\c\parts\os\clipboard\clipboard.cpp`) replaces its own copy of the 3 bytes with
the system clipboard cut at the NUL only when `OpenClipboard` succeeds, and Windows' clipboard watcher
(`TextInputHost`, clipboard history) may hold it open just after the write. Alone, 1 of 6 runs gave 3; under the
load of three runs side by side, every run. The recordings (1) stay; turning clipboard history off while testing
should end the race.
`screen/g12_game.files` (a script of Civil War Strategy) times out (exit 124 after 30 s, "no prompt in time" at
line 97 of its keys file) in every run of the slice list or the full corpus at `--jobs 8` since 2026-10-21: in
the full set at the merge of pull request 12 the full corpus twice, the slice list and the slice list without
folding (2026-10-22). The game hangs in `SHELL "copy cws.his oldhist.ory"`: libqb starts `cmd /c copy …` through
`ShellExecuteEx` with a console of its own (`libqb\internal\c\libqb\src\shell.cpp`, `sub_shell`), and that
`cmd.exe` neither copies nor ends until the game is killed at the timeout (about 30 s, watched once a second); the
runner's retry, a moment later, still finds the script's folder in use and fails with `PermissionError`. The script passes run alone
(3.1 s), as the first of 61 programs at `--jobs 8`, with its nine sibling scripts at `--jobs 8`, and in the
coverage check; the failure comes only late in a long run, so it is taken as the test environment's, not the
compiler's. Likely cause (2026-10-25, not yet seen in a hang): the game has no console, so libqb's
`ShellExecuteEx` (`SW_SHOW`) gives `cmd.exe` a new one, and with the machine's default terminal on "Let Windows
decide" Windows hands each new console to Windows Terminal over COM (a new `OpenConsole.exe` and
`WindowsTerminal.exe -Embedding` per call, and a window): a `SHELL "copy …"` took 0.67 s on average and up to
1.1 s that way, 0.10 s with Windows Console Host as the default terminal (5 calls, setting put back after). The
`cmd.exe` is taken to wait on that handover under load. With Console Host as the default terminal (Settings, For
developers, Terminal), two full corpus runs at `--jobs 8` passed (513 pass, 5 known failures; `g12_game.files`
8.6 s and 3.8 s; 2026-10-25, the setting put back after). The hang was never every run, so two passes do not prove
it. Left as noted (user, `DECISIONS.md` 2026-10-25): no setting is required; if the script times out late in a
long run, it is this, and it passes alone.

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

**For the merge into `main` of branch `mb5`** (2026-10-26, the full set on the tree of `de49cfb`, which is the
merged tree, in this checkout, no pass cache, `--jobs 8 --build-cache`; the three full suites with `--coverage`):
tier 1 624 tests pass (79 ignored), fmt and clippy clean, after `stale_index_job_skipped` was made to wait for its
first index job (it failed once under the suites' load: the job had not started when the text changed, so it was
skipped too); the full corpus suite 516 pass, 5 known failures, none failing; `slice.list` judged from it 496 of
496, and 496 of 496 with `QB64RUST_NO_FOLD=1`; the full upstream suite 402 pass, 1 failure out of scope
(`const/math_funcs`, D-031), 1 known failure, `pass.list` judged from it 402 of 402; the full verification suite
3892 pass, 63 known failures, 1 failure, `v59_g_readback` (its OpenGL viewport read 640 x 400 where 512 x 320 is
recorded, alone too, and so did the old compiler's own build of it three times: the machine's; a `.normalize`
sidecar now masks the viewport, and the program passes with it run alone), so `pass.list` judged from the run
3889 of 3890 (`v29_a_sound_queue` failed at `--jobs 8` and passed alone, timing-sensitive); the built-ins no run
reaches are those of `tests\never_run.list` (`builtin_runs.py --check`); the differential pass list 59 of 59; the
call-site check 2649 of 2649 equal; the coverage check of Civil War Strategy's scripts passes; the 77 command-line
tests that build pass.

**For the merge into `main` of branch `m5`** (pull request 17, 2026-10-25, the full set on the tree of
`961b4cc`, which is the merged tree, in this checkout, no pass cache, `--jobs 8 --build-cache`; the three full
suites with `--coverage`): tier 1 596 tests pass (79 ignored), fmt and clippy clean; the full corpus suite 513
pass, 5 known failures, none failing (`screen/c15_resize` and `g12_game.files` passed); `slice.list` judged from
it 493 of 493, and 493 of 493 with `QB64RUST_NO_FOLD=1`; the full upstream suite 402 pass, 1 failure out of scope
(`const/math_funcs`, D-031), 1 known failure, `pass.list` judged from it 402 of 402; the full verification suite
3889 pass, 66 known failures, 1 failure, `v42_a_timer` (timing-sensitive: it failed at `--jobs 8` and in the
runner's retry with other runs beside it, and passed run alone with nothing beside it), so `pass.list` judged
from the run 3889 of 3890; the built-ins no run reaches are those of `tests\never_run.list`
(`builtin_runs.py --check`); the differential pass list 59 of 59; the call-site check 2649 of 2649 equal; the
coverage check of Civil War Strategy's scripts passes.


## Known bugs

- `verification\v32_d33_free_zero_size` frees a block twice (`_MEMFREE` of a `_MEMNEW(0)` block, as libqb does),
  and whether that crashes (exit 127 recorded) depends on the heap: at the close of `m6-compiler-download`
  (2026-10-27) it printed `after` once at `--jobs 8` and in the runner's retry, and passed in the next run; the old
  compiler's own `v32_d33_free_zero_size.exe` printed `after` and exited 0 in 1 of 3 runs alone, so did the new
  build. Not the compiler's; it stays on `pass.list` until it fails again.

- `verification\v39_a_handles` recorded `_SNDGETPOS` and `_SNDLEN` of a raw sound at a sound device of 44100 Hz
  (`.1249886602163315`); at the close of `m4-builtins-remaining` (2026-10-24) the machine's device gave `.125`
  (48000 Hz), from the old compiler's build too (recorded again in a scratch folder), so the program moved to
  `tests\verification\known.list` as `varies`, beside `v39_b_new_mem`, which went the same way earlier. Not the
  compiler's. `verification\v46_a_play_v0` went the same way at the full set for the merge of `m4-sub-gl`
  (2026-10-24): `PLAY(0)` read while the background queue drains gives 40 or 41 at line 193, the old build's too.
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
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
