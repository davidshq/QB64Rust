# Status and next steps

Updated 2026-11-02 (cut to its own rule by finding 2 of the ninth review, `study\archive\30` §2).

**This file holds the current state only**: where we are, what waits on the user, what comes next, the numbers of
the last full runs, known bugs. It holds no history. When a step is done, replace its line; do not add a paragraph
about it. **Nothing outside "Waiting on the user" asks the user anything.** Where everything else lives:

| Kind of information | Home |
|---|---|
| What a change did, task by task, and what was found on the way | the change's `tasks.md` and `design.md` under `openspec\changes\` (archived ones in `archive\`), and `git log` |
| A measured fact about the old compiler | the family's recorded output and its line in `verification\README.md`; the families up to `v45` also in `study\00` §5 (closed to new facts, `CLAUDE.md` working rule 7) |
| A decision | `DECISIONS.md`; a decided difference from QB64pe: `QB64PE-DIVERGENCES.md` |
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
added `TODO.md`. The open OpenSpec change is `known-output`, which is `TODO.md` row 1. The extension's last
release is tagged `vscode-v0.2.0`; the versions are raised to 0.3.0 (`9ab21d1`), not yet tagged.

## Waiting on the user

Oldest first. Each line names where the details are. A question is never accepted by default; one left open is
put again at the next merge into `main`. Not on this list: `SOMEDAY.md`'s "QB64pe behaviours to review", the
questionable behaviours each change kept QB64pe's way, which stay silent until the user asks for them (`CLAUDE.md`
working rule 7).

Nothing is waiting: names of more than 40 characters were answered on 2026-11-02 (`DECISIONS.md`).

## Next

1. **`known-output`** (`TODO.md` row 1) is built and archived (2026-11-02). 116 verification programs that differ
   from QB64pe on purpose are now on `pass.list`, each judged by its reviewed
   `tests\verification\diverged\<name>.out.txt`. The review is the table at the end of
   `openspec\changes\archive\2026-11-02-known-output\design.md`. The shorter timeout of the row is not built: it
   would save nothing now (design D7). The runner changed, so the next `just full` runs everything once.
2. **M7, Linux** (`ROADMAP.md`): the one change `m7-linux` (the system toolchain, `DECISIONS.md` 2026-10-28) is
   built and closed (2026-10-29). Its CI jobs (tasks 7.1–7.3) are started by hand once they are pushed; their
   results are read when they come. M8 next.
3. **`windows-vm`** (OpenSpec change, archived 2026-11-03): the sandbox family runs in a Hyper-V test VM by
   default, with nothing on the host's screen. The VM is made and the family passes there, 7 of 7, in 14 full runs in a row.
   It also passes under load, so its `just full` step no longer runs alone, and still passes in Windows Sandbox
   (`--where sandbox`). All tasks are done, and its spec is merged into `testing/desktop-sandbox`. Trying
   `create.ps1 -FromBase` is deferred to the VM's first expiry, about 2027-02-01 (`SOMEDAY.md`).
4. **`vm-suites`** (OpenSpec change, archived 2026-11-04, its specs merged into `testing/check-workflow` and
   the new `testing/suites-vm`): `check full` and `check merge` run the
   steps that build and run programs in a second Hyper-V VM, `qb64rust-suites`, made on 2026-11-03 from the new
   shared base disk `qb64rust-base`. `--where host` keeps them on the host; `check quick` stays there. In the VM
   the slice list passed 494 of 494, the CLI tests 120 of 120 and a whole `full` run passed, at 1.7 times the
   host's time. The VM found two dependencies on the host: a lock folder the runner never made, and
   `v59_d_conversions` reading the GPU driver's stencil bits. Both are fixed. `check full --uncommitted` runs on
   uncommitted changes. `qb64rust.exe` needs `VCRUNTIME140.dll`, which a clean Windows lacks (`SOMEDAY.md`). Archive
   at the merge into `main`, after `just merge` runs its whole suites in the VM.

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

## Numbers at the last full runs

The last merge into `main` only; earlier ones are in `git log` and their pull requests. **Pull request 23, branch
`linux-sync-per-tree`** (2026-11-02): `just merge` in the worktree at `7334117`, then at `df67128` the steps its
fix touched (`upstream-whole`, `upstream-list`, `linux-upstream`) and `counts.py`; `linux-verification`'s two
failures re-judged against the new `known.linux.list`, not run again.

| Run | Result |
|---|---|
| Tests (nextest), fmt, clippy, doc | 686 pass (140 skipped), clean |
| CLI tests that build | 120 pass |
| Full corpus | 514 pass, 7 known failures, none failing |
| `slice.list` judged from it / without folding | 494 of 494 / 494 of 494 |
| Full upstream | 401 pass, 3 known failures (`const/math_funcs` D-031, `noprompt-continue-fatal` D-052, `http/read_example`); `pass.list` 401 of 401 |
| Full verification | 4149 pass, 203 known failures, none failing; `pass.list` 4139 of 4139 |
| qbasic / differential / call-site check | 143 of 143 / 59 of 59 / 2650 of 2650 equal |
| Built-ins, coverage of Civil War Strategy's scripts, sandbox | pass; sandbox 7 of 7 |
| Linux: tier 1 / corpus / upstream / verification | 694 pass (116 ignored) / 492 pass, 9 known, 20 skipped / 399 pass, 5 known / 4128 pass, 207 known, 17 skipped |

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

- `STATIC a(5, 5)` in a SUB beside a shared array `a` of one dimension compiles; QB64pe rejects it ("Cannot change
  the number of elements an array has!", `verification\v66_c22`, on `known.list`).
- `$COLOR:0` and then `SUB Red`: the error "name already in use" is reported at the constant in the auto-included
  `internal\support\color\color0.bi`, not on the program's `SUB Red` (line 4), where QB64pe reports it
  (`verification\v45_x48`, on `known.list` since the change `pin-rejection-line`).

## Queued for later milestones

| When | Item |
|---|---|
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
