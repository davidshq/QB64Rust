# Proposal

## Why

The eighth review (`study\29` §4, §5) found two gaps in the tooling, both to close before the screen-state check
(step 9a) adds to the runner:

- What the input sets still need was counted by hand twice (`study\28` §3, `study\29` §3), with `grep` over a dump
  that stops at 100 errors. The next changes (step 11, then M3's drawing statements) are chosen by that demand.
- `tools\legacy_tests\run_legacy_tests.py` (1,455 lines) judges tier 2 and records every expected output, and has
  no test. A change to its comparison or recording rules can change what "pass" means without anything noticing,
  and step 9a is such a change.

## What Changes

- `qb64rust --report unsupported <file or folder>... [--include-root <dir>]`: every program through the front end
  with the error cap lifted; one line per "not supported yet" message with the number of programs it occurs in,
  the number it alone holds back, and its uses. Run by hand; no test depends on its numbers.
- `tools\legacy_tests\test_run_legacy_tests.py` (standard library `unittest`): the runner's pure rules, and every
  outcome of one corpus program with the compiler and the program replaced by a stand-in. Run in CI's `test` job.

## Not in this change

- The locks and the private copies of `qb64pe` stay untested here: they need the clone, and the full runs cover
  them.
- CI's `tier2` job keeps building `qb64pe.exe` from the reference commit. `study\29` §6 proposed the release's
  `qb64pe.exe` instead, on the claim that the 16 commits between them change libqb only. That claim was wrong
  (`source\qb64pe.bas` differs in `evaluatefunc`'s typed `VAL`), so the proposal is withdrawn (`study\29` §8).

## Impact

- `crates\base` (the cap can be lifted), `crates\driver` (`report.rs`, the option, tests).
- `tools\legacy_tests` (a test file, the README), `.github\workflows\rust.yml` (one step).
- Specs: `compiler/cli` (the report), `testing/golden-corpus` (the runner's rules are tested).
