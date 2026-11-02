# Spec Delta

## ADDED Requirements

### Requirement: Stored results judged again
`run_legacy_tests.py --rejudge --suite <suite>` SHALL read the suite's last results, apply the known lists as they
are now (with `--platform linux`, the Linux lists too), rewrite each program's status (a failure on a list is a
known failure, a known failure off every list is a failure), the summary and the results file, and print the
summary and exit code a run would. It SHALL build and run no program. It SHALL refuse, with a message and exit
code 2, results with no run record of this runner, of another compiler build, or of a program whose files changed
after the run started. A program the lists now skip SHALL be reported as skipped. The results file SHALL record
that it was judged again and when, and `--check-list` and the count blocks SHALL read it as the run it came from.

#### Scenario: A failure made known
- **WHEN** a full run of the verification suite on Linux failed only on `v24_x93_large_bytes`, which is then added
  to `known.linux.list`, and `--rejudge --suite verification --platform linux` runs
- **THEN** that program is a known failure, the summary has no failure, the exit code is 0, and no program ran

#### Scenario: A known failure taken off the lists
- **WHEN** a program's row is removed from `known.list` and its stored result is a known failure
- **THEN** the re-judge reports it as a failure and exits 1

#### Scenario: A changed program
- **WHEN** a program's `.out.txt` changed after the run started
- **THEN** the re-judge refuses and names the program
