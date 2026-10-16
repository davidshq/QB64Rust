# Tasks

Order: group 1 first (the measurements), then 2 to 5 in any order, then 6. Each of groups 2 to 5 ends with
`cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test` (tier 1) green and
with its tests, lists and `crates\README.md` lines in place, so the change can stop after any group.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools or
a script file; `python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done.
**Test cadence** (`CLAUDE.md` working rule 9): while working, tier 1 and only the programs the work touches
(`--glob`, `--category`, or a list with `--pass-cache target\pass-cache`), each run's output sent to a log under
`target\`; tier 2, the run without folding and the full suites run once, in group 6. A new measured fact goes
into a `v33` paragraph of `study\00` §5 in the task that measures it. A behaviour of the old compiler that looks
questionable is implemented its way and listed in `SOMEDAY.md` ("QB64pe behaviours to review").

## 1. Measurements before code

- [x] 1.1 Measure every form with `qb64pe -z` and `qb64pe.exe`, outside the repository (design D1–D5):
  `$ASSERTS`, `$ASSERTS:CONSOLE` and `$DEBUG` in each form (case, blanks, a tab, a word after, in a `$IF` branch
  not taken, in an included file, in a SUB, twice); `_ASSERT` with one and two arguments, after `CALL`, with a
  number as the message, with none; the six functions with each argument type, an empty string, bad input, a
  level out of range, without `$`; `SYSTEM n` and `END n` with each numeric type and a string; what `$DEBUG`
  changes that a program can see outside the IDE (start time, `_SOCKETS_`, error messages, `ERL`). Keep them as
  `verification\v33_*` (design D7), recorded with `--suite verification`. Verify: the `v33` paragraph of
  `study\00` §5 has the facts; each scenario of the specs names values the old compiler gives, corrected if not;
  the reference clone's `git status --porcelain` is empty before and after.
- [x] 1.2 Measure at run time: errors 314 and 315 under `ON ERROR` (trapped or not), their messages and exit
  codes; the process's exit code for `SYSTEM` 3, 0, -1, 256, 2.7, `2^40` and `END 7`, with and without a window;
  how the runner records an exit code (`exit=` in the recorded output). Verify: `v33_a`, `v33_c` recorded;
  design D3 and D5 say what was found.
- [x] 1.3 Record `v33*` in `CLAUDE.md`'s layout table, and tell the user of anything `$DEBUG` changes outside
  the IDE (design D2) or any form that needs more than the design describes. Verify: the row names the programs;
  `check_repo.py --untracked` is clean.

## 2. $ASSERTS, _ASSERT and $DEBUG

- [x] 2.1 `$ASSERTS`, `$ASSERTS:CONSOLE` and `$DEBUG` accepted in `sema`'s `meta` with the measured forms
  (design D1); `asserts` in the IR's settings; `int32 asserts=1;`; `$ASSERTS:CONSOLE` a console; the `$DEBUG`
  warning with `-w`. Verify: `check-ok` and `check-fail` front-end tests of the forms; a `cpp` snapshot shows
  `asserts=1`; the `$DEBUG` warning in a `cli.rs` test.
- [x] 2.2 `_ASSERT` as a statement row (design D3), with call-site programs per form. Verify: the call-site check
  passes for them; a slice program (asserts on, off, trapped, a message with a console) passes; the four upstream
  programs (`precomp-flags/asserts`, `assertscon`, `debug`, `auto_include/debug`) pass and join `pass.list`.
- [x] 2.3 `DIVERGENCES.md` entry for `$DEBUG` (design D2; the decision's row in `DECISIONS.md` is written).
  Verify: the entry names the user's decision and what a program built here cannot do.

## 3. Checksums, compression and base64

- [x] 3.1 The six rows and `RuntimePart::DataProcessing` with `DEP_ZLIB=y` in the old compiler's place (design
  D4). Verify: call-site programs per function and argument type equal; a unit test of `make_flags` order; a slice
  program of values passes.
  *Found*: a built-in that needs `$` named without it (`_DEFLATE("x")`) is "not supported yet", as every such
  name is today (`_TRIM(...)`), not the old compiler's "Invalid array name"; the spec's scenario was narrowed to
  that (user, 2026-10-14) and the form listed in `SOMEDAY.md`. The INTEGER slot of `_ASSERT` (task 2.2) is
  `Slot::Integer`, new. `_INFLATE$` with a size larger than the data leaves the bytes past it unset, so the slice
  program prints only their count.
- [x] 3.2 Upstream: `console_only/libz`, `image/pcx_test`, `image/webp_test` pass and join `pass.list`
  (`libz/test` waits on group 5). Verify: each run alone passes.

## 4. END and SYSTEM with an exit code

- [x] 4.1 The exit code on `StmtKind::End` and `System`, checked and converted as measured; the emitter's two
  lines (design D5). Verify: an `ir` test, `check-fail` for a string, a call-site or `cpp` snapshot of the lines,
  a slice program whose recorded output ends with its exit code;
  `arrays/t_qb64pe_refactor_preserve_inline_varstrings` passes and joins `pass.list`.

## 5. The licence file

- [x] 5.1 `-f:GenerateLicenseFile=true|false` (design D6): the make variables, the file moved with the
  executable, the build cache keyed by it and keeping the file; the `cli.rs` test of an unknown setting names
  `MaxCompilerProcesses`. Verify: `cli.rs` tests (written when asked, absent when not, cached and copied);
  `libz/test`, `audio_out/test` and `image/test` pass and join `pass.list`, leaving
  `tests\known_clean_not_passing.list`; `basic/test` checked and its line there updated if it still cannot pass.

## 6. Close

- [x] 6.1 Closing runs, once (working rule 9): tier 1 with the command-line tests that build, the runner's tests,
  the call-site check, the slice list with and without folding, the upstream and differential pass lists, the two
  full suites, the demand reports, the shrink-only lists regenerated. Verify: numbers in `STATUS.md`; nothing on a
  pass list fails.
- [x] 6.2 Documents: `STATUS.md`, `ROADMAP.md`, `crates\README.md`, `tests\upstream\README.md`, `study\00` §5,
  `SOMEDAY.md` (what stays marked), `CLAUDE.md`'s layout table; `python tools\testtime\test_time.py`. Verify:
  `check_repo.py --untracked` is clean; the files to stage and a commit message are given to the user.
