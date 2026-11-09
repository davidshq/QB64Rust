# Status and next steps

Updated 2026-11-07, for the merge of `after-pr25` (cut to its own rule by finding 2 of the ninth review,
`study\archive\30` §2).

**This file holds the current state only**: where we are, what waits on the user, what comes next, the numbers of
the last full runs, known bugs. It holds no history. When a step is done, replace its line; do not add a paragraph
about it. **Nothing outside "Waiting on the user" asks the user anything.** Where everything else lives:

| Kind of information | Home |
|---|---|
| What a change did, task by task, and what was found on the way | the change's `tasks.md` and `design.md` under `openspec\changes\` (archived ones in `archive\`), and `git log` |
| A measured fact about the old compiler | the family's recorded output and its line in `verification\README.md`; the families up to `v45` also in `study\00` §5 (closed to new facts, `CLAUDE.md` working rule 7) |
| A decision | `DECISIONS.md`; a decided difference from QB64pe: `DIVERGENCES-QB64PE.md` |
| What the compiler supports | the merged specs under `openspec\specs\` (an index by area: `crates\README.md`, "What the compiler supports"); how a change built it: its `design.md` |
| How to build and test it | `crates\README.md`; per test set: `tests\*\README.md` |
| Milestones, done and not done | `ROADMAP.md` (open), `ROADMAP-COMPLETED.md` (done) |
| History up to 2026-10-09 (what this file held before) | `study\archive\progress-record.md` (closed) |

## Where we are

M0 to M5 are complete (`ROADMAP-COMPLETED.md`; M3 declared done at the merge of pull request 9, user, 2026-10-18,
tenth review `study\35` §3 item 1; M4 at the merge of branch `m4-sub-gl`, user, 2026-10-24, `DECISIONS.md`, what
its list left open moved to `SOMEDAY.md`; M5, the debugger, at the merge of pull request 17, user, 2026-10-26;
M5b, the formatter and the language-server features, at the merge of pull request 18, 2026-10-26). M6, rewritten
as "the runtime without QB64pe" (`ROADMAP.md`, `DECISIONS.md` 2026-10-28), is done, its check on a machine with no
QB64pe postponed to `SOMEDAY.md` (`DECISIONS.md` 2026-10-28): `m6-toolchain`, `m6-compiler-download`,
`fixes-switch` and `libqb-fixes` are archived. Twenty-two M3 steps are built and archived (`ROADMAP-COMPLETED.md`,
M3), the last `m3-constant-evaluation` (`4f647a9`); `test-parallel-runs` and `test-while-building` (the test
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
(`fa91454`, 2026-10-26) merged branch `mb5`: `m5b-format-settings` and `m5b-language-features`. Pull request 19
(`fa6de03`, 2026-10-28) merged branch `more-stuff`: `qbasic-testcases`, `m6-toolchain`, `m6-compiler-download`,
`fixes-switch`, `libqb-fixes`, the QB 4.5 oracle (`tools\qb45`) and the purpose, on `libqb-fixes`' closing runs
and the full corpus, without the rest of the full set (`DECISIONS-SMALL.md` 2026-10-28). Pull request 21
(`012ed21`, 2026-10-29) merged branch `docs-and-cuts`, the answers to `study\38`'s last questions (`DECISIONS.md`
2026-10-28); pull request 22 (`bfd2aba`, 2026-11-01) merged branch `misc-updates`: `m7-linux`,
`basic-unit-tests`, `desktop-sandbox` and `testing-reset`. Pull request 23 (`d6d341e`, 2026-11-02) merged
branch `linux-sync-per-tree`. It holds four changes, all archived: `linux-sync-per-tree`, `check-rejudge`,
`pin-rejection-line`, and the 40-character limit on names, which was built without an OpenSpec change. It also
added `TODO.md`. Pull request 24 (`1038e38`, 2026-11-04) merged branch `more-updates`: `known-output`,
`windows-vm`, `vm-suites` and `_BIT` parameters. Pull request 25 (`3bb0d2c`, 2026-11-06) merged branch
`we-like-changes`: `value-trace`, `lints`, `test --coverage`, `toolchain install --from-zip`, the PDS 7.1
oracle, QB64pe's `-c`, `-s`, `-e` and `-p`, and fixes to the Linux steps and `tools\check`. M6 moved to
`ROADMAP-COMPLETED.md` with it. Pull request 26 (`4b1066a5`, 2026-11-07) merged branch `after-pr25`:
`eval-order-left-to-right`, the end of M7 (Linux), `TODO.md` rows 4, 6 and 7, a bare `RETURN` before `ELSE`,
the fixes for CI after pull request 25, a fix to `tools\screen` for a crash when a key is pressed during exit, and
the QB 4.5 rows Q-021 and Q-022. M7, Linux, moved to `ROADMAP-COMPLETED.md` with it. No OpenSpec change is
open. Pull request 27 (`f1a64ac0`, 2026-11-07) merged branch `after-pr26` without a `just merge` (user's choice:
one build flag and documents since pull request 26's run; CI's Rust, extension and release dry run passed). The
extension's last release is `vscode-v0.3.0` (2026-11-07), on that merge commit: the first with Linux, and the
first whose `qb64rust.exe` needs no `VCRUNTIME140.dll` (`DECISIONS.md` 2026-11-07).

## Waiting on the user

Oldest first. Each line names where the details are. A question is never accepted by default; one left open is
put again at the next merge into `main`. Not on this list: `SOMEDAY.md`'s "QB64pe behaviours to review", the
questionable behaviours each change kept QB64pe's way, which stay silent until the user asks for them (`CLAUDE.md`
working rule 7).

Nothing is waiting. The game ports' open questions are kept in the games' own repos (`CLAUDE.md` working rule 2).

## Next

1. **Choose the next milestone.** `ROADMAP.md` has no open milestone. macOS, which was M8, is postponed to
   `SOMEDAY.md` (`DECISIONS.md` 2026-11-07), so no platform milestone is next. The candidates are in
   `SOMEDAY.md`: deferred QB64pe features, a strict QuickBASIC 4.5 mode, popular PDS 7.1 features, and the VS
   Code features still missing.

Still to watch: the CLI tests in the suites VM. They have now failed there at two merges in a row, a different
test each run, and each failed test passed in the next run:
- Pull request 25: four debugger tests and `unit_tests::a_timeout`, twice, in 945 s against 192 s at pull
  request 24's. All five passed on the host at `540aacb0` in 31 s.
- Pull request 26: `trace_record_when_the_window_is_closed` in the merge run ("the program did not end after its
  window was closed", 417 s). Run alone, it passed and `debug_next_over_call_and_into_sub` failed instead, with 2
  stops of the 4 it waits for (1050 s, of which 468 s copying the tree in). That one is the `lldb-server` race
  below.

The next step is to find out why the VM runs `cli` so much slower than the host.
`s75_sound_wait` crashed once under load (`libqb-copy`'s design, "Risks").
`lldb` and `lldb-dap` debug through `lldb-server` since `m5-dap-proxy` (`DECISIONS.md` 2026-10-25): its race after a
single step ends about one session in seventy at once ("lost connection"; 3 in 222 runs of
`debug_next_steps_by_line` under load), so a debugger CLI test can fail so now and then; run it again alone. The
cause is `lldb-server`'s (`openspec\changes\archive\2026-10-25-m5-dap-proxy\design.md`, "Measured").
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

## Numbers at the last full runs

The last merge into `main` only; earlier ones are in `git log` and their pull requests. **Pull request 26, branch
`after-pr25`** (2026-11-07): `just merge` in the worktree at `dc731652`, whose tree is the merge commit's but
for the commit of its count blocks, the fix to `tools\screen\screen_state.cpp` and the QB 4.5 recordings
(`q45_*`), with the whole suites in the suites VM. 36 steps passed and one failed: `cli`, one test in the VM
("Still to watch" under "Next"). Linux corpus: `screen/u01_mouse_queue` passed. It stays on
`tests\corpus\known.linux.list` as `varies`, since it passed 1 run in 2 under WSLg.

| Run | Result |
|---|---|
| Tests (nextest), fmt, clippy, doc | 720 pass (153 skipped), clean |
| CLI tests that build | 130 pass, 1 failed in the VM; it passed when run again, and another failed |
| Full corpus | 516 pass, 7 known failures, none failing |
| `slice.list` judged from it / without folding | 496 of 496 / 496 of 496 |
| Full upstream | 401 pass, 3 known failures (`const/math_funcs` D-031, `noprompt-continue-fatal` D-052, `http/read_example`); `pass.list` 401 of 401 |
| Full verification | 4321 pass, 79 known failures, none failing; `pass.list` 4312 of 4312 |
| qbasic / differential / call-site check | 143 of 143 / 65 of 65 / 2650 of 2650 equal |
| Built-ins, coverage of Civil War Strategy's scripts, sandbox | pass (375 of 390 built-ins run); 10 of 10; 8 of 8 |
| Linux: tier 1 / corpus / upstream / verification | 728 pass (126 ignored) / 491 pass, 12 known, 20 skipped / 400 pass, 4 known / 4290 pass, 93 known, 17 skipped |

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
- `verification\v49_d_clipboard` can read `LEN(_CLIPBOARD$)` of `"a" + CHR$(0) + "b"` as 3, not the recorded 1,
  on the host. libqb's read (`libqb\internal\c\parts\os\clipboard\clipboard.cpp`) gives its own copy of all 3
  bytes whenever `OpenClipboard` fails. Windows' clipboard history (`TextInputHost`) may hold the clipboard open
  just after the write, and the old compiler's build does the same. Not the compiler's. Since 2026-11-07 its
  sidecar starts with `clipboard:`, so the runner skips it on a Windows host and it runs in the suites VM and CI,
  where nothing else holds the clipboard. `v31_x01_accepted` and upstream `clipboard/clipboard_test` are marked
  the same way. Its cases also run in the sandbox family as `tests\sandbox\clipboard_text`.
  `slice/s150_builtins_more` no longer uses the clipboard.

- `STATIC a(5, 5)` in a SUB beside a shared array `a` of one dimension compiles; QB64pe rejects it ("Cannot change
  the number of elements an array has!", `verification\v66_c22`, on `known.list`).
- `$COLOR:0` and then `SUB Red`: the error "name already in use" is reported at the constant in the auto-included
  `internal\support\color\color0.bi`, not on the program's `SUB Red` (line 4), where QB64pe reports it
  (`verification\v45_x48`, on `known.list` since the change `pin-rejection-line`).

## Queued for later milestones

| When | Item |
|---|---|
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
