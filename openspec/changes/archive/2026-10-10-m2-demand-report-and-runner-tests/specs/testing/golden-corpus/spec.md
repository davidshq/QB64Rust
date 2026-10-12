# Spec Delta

## ADDED Requirements

### Requirement: The runner's rules are tested
The rules by which `run_legacy_tests.py` decides that a corpus program passes, and what `--record` writes, SHALL be
covered by tests of the runner itself (`tools\legacy_tests\test_run_legacy_tests.py`) that need no compiler, no
reference clone and no C++ build, and that run in CI with tier 1: the normalisation of program output and of
`.err` text, the rule for a rejection by a compiler other than `qb64pe`, `.normalize` files, the `--list` and
known-failures files, and for one corpus program every outcome (equal and different output, each kind of `.err`
result, `.norun`, `.stdin`, a known failure without an expected file, a failed or timed-out run, and `--record`
with equal runs, with differing runs and with a rejection). A change to one of these rules SHALL come with a
change to its test.

#### Scenario: A broken comparison is noticed
- **WHEN** the runner is changed so that a bare CR in a program's output no longer counts, or so that the two runs
  of `--record` need not be equal
- **THEN** `python -m unittest discover -s tools/legacy_tests -p "test_*.py"` fails

#### Scenario: Runs without a compiler
- **WHEN** the tests run on a machine with Python and neither `qb64pe` nor `qb64rust`
- **THEN** they pass
