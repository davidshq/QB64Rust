# Proposal

## Why

2,533 verification programs are recorded `(not compiled)`. The compare passes each one when the new compiler rejects
it with any error not marked "not supported yet", wherever that error stands, so a program can pass for the wrong
reason on the wrong line (`TODO.md` row 1, the test engineer's review of the verification suite, 2026-10-27). QB64pe's
recording names the line (`LINE n:` in `<name>.compile.txt`), and a measurement of 2026-10-28 found our first error
on that line in 2,384 of the 2,412 programs where both name a line of the main file (98.8%). Pinning is therefore
possible at the cost of about 28 recorded exceptions.

## What Changes

- A program recorded `(not compiled)` passes only when the earliest line of the program's own file holding an error
  of the new compiler not marked "not supported yet" is the line of the recording's first `LINE n:`, whatever order
  the errors are printed in. Otherwise it fails at the new stage "error line", naming both lines.
- The pin is not checked when the recording names no line (a C++ build failure, `$VERSIONINFO`, `_BIT` parameters;
  about 36 programs) or when QB64pe's message places the error in an included file ("… in line N of <file>
  included"), where its `LINE n:` is the `$INCLUDE` line and ours is a line of the other file.
- A new list, `tests\verification\lines.list`, holds the exceptions: a program, the line our rejection is pinned
  to instead, and the reason (mostly block errors, where we name the line the block opens and QB64pe the line where
  it noticed). An exception still pins: the program fails if our earliest error moves to any third line. A row whose
  program would pass without it is reported, as a `known.list` row that passes is.
- The message text is still not compared.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `testing/verification-suite`: the rule for a program recorded `(not compiled)` adds the line of the earliest error;
  a new requirement for the exceptions list; the runner tests cover the new rule.

## Impact

- `tools\legacy_tests\run_legacy_tests.py` (`verification_compare`, the rejection rule) and its tests in
  `test_run_legacy_tests.py`; `tools\legacy_tests\README.md`, "Verification programs".
- `tools\check\steps.py`: the step `verification` reads the new list through `tests/verification/**` already; the
  list is not a judge-only list (`JUDGE_LISTS`), since judging it needs the compile's output.
- `tests\verification\lines.list` (new), `tests\verification\README.md`.
- Programs that fail the pin for a reason that is a bug in the compiler move to `known.list` as "bug: …" until fixed.
- No compiler code changes. The first full run of the verification suite after this change runs every rejected
  program again (they are never served from the pass cache).
