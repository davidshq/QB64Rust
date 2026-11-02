# Design

## Context

See `proposal.md`, "Why". What the code does now:

- **The rejection rule.** `tools\legacy_tests\run_legacy_tests.py`'s `verification_compare` reads
  `<name>.out.txt`. When it is `(not compiled)` (`NOT_COMPILED`), the program is compiled with `-x -q -m <name>.bas`
  in a fresh folder, and `new_compiler_rejection(rc, output)` passes it when the last line is the error summary
  (`SUMMARY_RE`) with at least one unmarked error. The output goes to `results\<tag>-compile.txt`. A rejected
  program is never served from the pass cache.
- **What each side writes.** QB64pe's `<name>.compile.txt` is the message, `Caused by (or after):…`, then
  `LINE n:<source>`, where n is a line of the main file (for an error QB64pe finds in an included file, the line of
  the `$INCLUDE`, and the message says "in line M of <file> included" or "… auto-included"). The new compiler prints
  one line per error, `<file>:<line>:<column>: error: <message>`, with `not supported yet: ` before the message of a
  marked one (`qb64rust_base::Diagnostic::render`), then the summary. The errors are printed in the order the
  passes find them, not sorted by place (only `crates\driver\src\hover.rs` sorts them): the parser's come before
  the checker's.
- **The measurement of 2026-10-28** (`TODO.md` row 1) compared the first error printed by `qb64rust -z` with the
  first `LINE n:`: 2,384 agree, 28 differ (mostly block errors), 57 not compared (36 with no `LINE n:`, 10 with our
  first error in an included file, 11 that `-z` accepts).
- **The lists.** `crates\driver\tests\it\inputs.rs` (the tier-1 step `lists`) holds `pass.list` and `known.list`
  to the programs of `verification\` and `known.list`'s reasons to their vocabulary. The runner's `--list-only`
  (`read_by_runs`, the loop over `known.list` and `known.linux.list`) names the files a run reads, and
  `tools\check\test_check.py` holds each to be an input of its step.

## Goals / Non-Goals

**Goals:**

- A rejection passes only on QB64pe's line, or on the line `lines.list` records for it.
- The exceptions are a short list with a reason each, and a row that is no longer needed is reported.

**Non-Goals:**

- Comparing the message text, or the column.
- Mapping our error in an included file to the line of its `$INCLUDE` (D4).
- Changing where the compiler reports a block error: QB64pe's "where it noticed" is not a better place (D5).
- Re-judging a change of `lines.list` without a run (D7).

## Decisions

**D1. The pin sits in the rejection branch of `verification_compare`.** After `new_compiler_rejection` returns
`""`, two new functions read the lines: `recorded_line(compile_txt: bytes) -> int | None` (the first `LINE n:`, or
`None` when there is none or the message before it matches `in line \d+ of .* included`), and
`earliest_error_line(output: bytes, main: str) -> int | None` (the lowest line among the `: error: ` lines of the
file `main` whose message does not start with `not supported yet: `, or `None` when there is none). The program
passes when that line is the recorded one, or the one `lines.list` gives; otherwise it fails at the stage
`error line` with the detail `QB64pe names line 7, the earliest error is on line 3` (or `… no unmarked error in
<name>.bas`). Both functions take bytes and are tested on their own. The fixes-on run (`fixed=True`) is never a rejection, since its sidecar is a program's output, so it
is not touched. *Instead of:* a separate pass over stored results after the run; the compare already holds both
outputs.

**D2. The earliest unmarked error of the main file, not the first printed nor any** (user, 2026-11-02, after the
panel). QB64pe goes through the source in order and stops at its first error; we go on, so "any error on the line"
would pass a program with a wrong error before the right one. "First printed" would too when the wrong error comes
from a later pass (a checker error on line 3 printed after a parser error on line 7), and it would tie the result
to the order of the passes, which a refactor may change. The earliest by line catches both. The measurement of
2026-10-28 was of the first printed, so the number of exceptions may differ; the full run of task 4 measures the
rule as built. A marked error is skipped: the rejection rule already treats marked errors as no rejection.

**D3. The file is matched by name.** The compile runs in the program's own folder with the bare name `<name>.bas`,
so our output names the main file as `<name>.bas`; an included file is named by its path from there
(`v16_inc/x.bi`). The match is on the name as printed, slashes taken either way, and errors of other files are
left out of the pin. If the compiler ever prints the main file another way, the runner tests (task 2) see it first.

**D4. QB64pe's error in an included file is not compared.** Its `LINE n:` is then the `$INCLUDE` line; ours is a
line of the included file, and our output does not name the `$INCLUDE` that brought it. Mapping one to the other
needs the include chain in our output, which nothing else asks for. These programs (about 15 recordings carry "in
line … of … included") keep the rule they have now. A recording without that wording and our unmarked errors all
in included files is a failure, since QB64pe found the error in the main file.

**D5. Exceptions keep a line, not a skip.** `tests\verification\lines.list` has rows `<name><TAB><line><TAB><reason>`,
`#` comments, sorted by name, as `known.list`. The line replaces QB64pe's for that program, so it still pins: a block
error we name at its opening line must stay there. A program on `lines.list` whose earliest error is on QB64pe's own
line passes, with the detail `lines.list row not needed`, and the run's end lists such rows beside the known-list
programs that passed. Reasons are free text; the expected kinds are "the line the block opens" and "the inner
block". *Instead of:* a skip list (it would stop pinning the 28), or changing our block errors to QB64pe's line
(the opening line is the more useful one to a user, and it would be a compiler change for a test's sake).

**D6. A program whose wrong line is a bug goes to `known.list`, not `lines.list`.** When triage (task 4) finds our
earliest error is a wrong one (a spurious error before the real one, or the right error found for the wrong reason),
the program gets a `known.list` row `bug: <what>` and a known bug in `STATUS.md`. It is not fixed in this change.
`lines.list` is only for a place both compilers can defend.

**D7. `lines.list` is a run input, not a judge list.** Judging it needs our compile output, which `--rejudge` does
not have in its results. So `--list-only` names it (beside `known.list`), it stays out of
`steps.py`'s `JUDGE_LISTS`, and a change to it runs the verification step again. The step's inputs already take
`tests/verification/**`. The pass cache is unaffected: it never serves a rejected program. A run after an edit is
cheap (user, 2026-11-02, after the panel): the step runs with the pass cache, which serves every program that built,
and the rejected programs stop in the front end without a C++ build, so it is a few minutes of compiles in the
background. A change to the runner is not: the pass cache's key holds the runner's source, so every program that
builds is built and run again (about 23 min of the step on 2026-11-02). *Instead of:* teaching `--rejudge` to apply a judging rule from stored compile output, a new kind of
re-judge to maintain for minutes saved.

**D8. The tier-1 check.** `inputs.rs`, beside the check of `known.list`, reads `lines.list` and fails a row with no
tab-separated line or reason, a line that is not a positive number, a name that is not a program of
`verification\`, a program whose `.out.txt` is not `(not compiled)`, a program whose `.compile.txt` names no line
to compare (no `LINE n:`, `LINE 0:`, or an included file: the row would never be used), or a name given twice. The
runner reports such a row as not needed too (the review of the change, 2026-11-02).

**D9. `LINE 0:` names no line** (found in the triage, user, 2026-11-02). QB64pe writes `LINE 0:` after "Caused by
(or after):[INFORMATION UNAVAILABLE]" when it knows no line (an `$IF` without `$END IF`, a `DECLARE LIBRARY`
function not found in the library); `recorded_line` takes it as no line recorded.

## Measured

The first run of the rule (the step `verification`, `pass.list`, 2026-11-02): 2,516 rejected programs judged, 24
failed at `error line`. 6 were `LINE 0:` (D9). 9 were block errors, 7 an earlier follow-on error of a refused SUB or
FUNCTION header or of a refused line number, 1 a tab before `$IF` that QB64pe ignores (`v26_x40`), 1 a label refused
where it is defined, not where it is used (`v59_x41`): 18 rows of `lines.list`. 1 was a bug (`v45_x48`, D6). The
user ruled the follow-on errors rows of `lines.list`, not bugs (the panel; a `SOMEDAY.md` row to reduce them).

## Risks / Trade-offs

- [The full run finds more than the 28 measured: the compiler has changed since 2026-10-28, and D2 pins another
  error than the one measured] → triage each failure by D5 and D6, and stop and put a table of reasons with their
  counts to the user, before writing any row, at the first of: more than 40 exceptions, more than 10 bug rows (each
  takes a program off `pass.list`), or a reason other than the block-error kinds covering more than 3 programs
  (user, 2026-11-02, after the panel).
- [A program now on `pass.list` fails the pin] → it is triaged in this change (D5 or D6); `pass.list` and
  `known.list` stay exact.
- [The pin reads `<name>.compile.txt` on every platform, while the `(not compiled)` recording goes through
  `expected_file` and may be a `<name>.linux.out.txt`] → no `.linux.compile.txt` exists and nothing records one
  (the review of the change, 2026-11-02, the pragmatic engineer). The change that first records a Linux rejection
  takes the compile recording through `expected_file` too.
