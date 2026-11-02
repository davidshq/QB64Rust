# Spec Delta

## ADDED Requirements

### Requirement: A run judged again when only the known lists changed
A whole-suite step whose last run, passed or failed, differs from now only in the lists that mark failures as known
(`tools/legacy_tests/known_failures.txt`, a suite's `known.list` or `known.linux.list`) SHALL be planned as a
re-judge of that run's results, not as a run: no program is built or run, and the step's result is the verdict of
the stored results under the lists as they are now. A change of any other input of the step SHALL plan a run.

#### Scenario: A failure made known
- **WHEN** the whole upstream suite failed on two programs, and a row for each is added to `known_failures.txt`
  with nothing else changed
- **THEN** the next `check` plans the step as a re-judge, which passes in seconds without running a program

#### Scenario: Another input changed too
- **WHEN** a known list and a file under `crates/` changed since the step's last run
- **THEN** the step is planned as a run

## MODIFIED Requirements

### Requirement: The merge into main
`check merge` SHALL run, without the pass cache, every step of `check full` but the pass lists that its whole
suites stand for, and the steps of the merge: the known lists and the old wording table regenerated before the
build, the wording held to its recordings, the corpus, compile and verification suites run whole with their
coverage records written, each pass list held to its whole suite's results, the built-ins' list held to the
records, the demand reports, the count blocks (failing when one is left as it is) and the three suites whole on
Linux, in a copy brought up to date by content at the tree's first merge and under `--force`, and from the files
git reports changed once an earlier merge's sync of that tree passed. A step's result SHALL stand for a later `check merge` only when it
passed in an earlier `check merge` and its fingerprint is unchanged; a whole suite of an earlier `check merge`
whose inputs differ only in the known lists SHALL be re-judged. Like `check full`, it SHALL refuse a checkout with
uncommitted changes under `crates/`, `libqb/` or `tests/`.

#### Scenario: The coverage records are of the merge
- **WHEN** `check merge` passes
- **THEN** `target/coverage/merge` holds the records of the three whole suites of that commit, which `check
  quick`'s libqb narrowing reads

#### Scenario: The exact copy
- **WHEN** `check --force` runs the Linux sync, or `check merge` runs it in a tree with no passed sync of an earlier
  merge
- **THEN** every file of the copy is compared by content, whatever its size and time

#### Scenario: The copy after an earlier merge
- **WHEN** `check merge` runs the Linux sync in a tree whose earlier merge's sync passed
- **THEN** the sync copies the files git reports changed since, in seconds

#### Scenario: A merge again after a fix to its lists
- **WHEN** `check merge` failed on a whole suite, the fix adds rows to a known list and is committed, and `check
  merge` runs in the worktree at the new commit
- **THEN** the steps that passed in the first `check merge` with unchanged fingerprints stand, the failed suite is
  re-judged, and the count blocks are written again
