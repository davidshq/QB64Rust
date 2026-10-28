# Spec Delta

## ADDED Requirements

### Requirement: The qbasic programs build
Every program of QB64pe's `qbasic_testcases` that the old compiler builds SHALL build with the new compiler, read
from the clone, compiled only, as QB64pe's `qbasic_tests.sh` judges them. The suite SHALL run with the new compiler
in the closing runs of a change and at each merge into `main`, and SHALL accept `--jobs`.

#### Scenario: The suite with the new compiler
- **WHEN** `run_legacy_tests.py --suite qbasic --qb64 target/release/qb64rust.exe --jobs 8` runs with the clone
  present
- **THEN** all 143 programs pass

#### Scenario: A rejection
- **WHEN** one program gets an error from the new compiler, "not supported yet" or not
- **THEN** the run fails and names it
