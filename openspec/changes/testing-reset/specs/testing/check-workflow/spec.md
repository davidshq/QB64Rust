# Spec Delta

## Purpose
The one entry point for testing the project: its three layers, what makes a recorded result valid, the order the
runs go in, and the tests that keep the entry point's own cost from growing.

## ADDED Requirements

### Requirement: Three layers behind one command
The project SHALL have one command, `check`, with three forms. `check quick` SHALL run the formatting check,
the lints, the workspace's Rust tests, the tests of the Python tools and the slice list. `check full` SHALL run
every suite the project has, the Linux and sandbox families included. `check <path-or-name>` SHALL run only the
tests of the named crate, folder, suite, list or program, with one syntax for every suite.

#### Scenario: The quick layer
- **WHEN** `check quick` runs on a warm build with nothing changed since the last run
- **THEN** it finishes in under 5 minutes and prints each step with its result and its time

#### Scenario: One program by name
- **WHEN** `check v66_a01_varptr_dynamic_owner` runs
- **THEN** only that verification program is compared, and nothing else runs

#### Scenario: One crate
- **WHEN** `check crates/sema` runs
- **THEN** only that crate's tests run

### Requirement: The merge into main
`check merge` SHALL run, without any result standing and without the pass cache, every step of `check full` but
the pass lists that its whole suites stand for, and the steps of the merge: the known lists and the old wording
table regenerated before the build, the wording held to its recordings, the corpus, compile and verification
suites run whole with their coverage records written, each pass list held to its whole suite's results, the
built-ins' list held to the records, the demand reports, the count blocks (failing when one is left as it is) and
the three suites whole on Linux. Like `check full`, it SHALL refuse a checkout with uncommitted changes under
`crates/`, `libqb/` or `tests/`.

#### Scenario: The coverage records are of the merge
- **WHEN** `check merge` passes
- **THEN** `target/coverage/merge` holds the records of the three whole suites of that commit, which `check
  quick`'s libqb narrowing reads

### Requirement: A result is valid by its inputs
Each step SHALL record, with its result, a fingerprint of every input the step reads: the compiler binary, libqb's
sources and edits, the toolchain, the runner, the lists and the files of the suite. The result SHALL stand while
every input's fingerprint matches, whatever commit, branch or working-tree state the repository is in. A failed
result SHALL never stand for a later run. `check --force` SHALL run every step again.

#### Scenario: A commit changes nothing
- **WHEN** every step of `check quick` passed, then the user commits, and `check quick` runs again
- **THEN** no step runs again, and each is reported as standing with the time of its last run

#### Scenario: An input changed
- **WHEN** a file under `crates/sema` changes after `check quick` passed
- **THEN** the next `check quick` runs the lints and the Rust tests again, and the slice list, and reports which
  input changed

#### Scenario: A failure never stands
- **WHEN** a step failed and `check quick` runs again with nothing changed
- **THEN** the step runs again

### Requirement: Programs that cannot be cached
A program whose result depends on the clock, a device, the network, sound or the desktop SHALL be marked so, and
SHALL run on every `check` that reaches it, never served from a cache.

#### Scenario: A sound program
- **WHEN** a slice program marked as depending on a sound device passed, and the slice list runs again with nothing
  changed
- **THEN** the program runs again

### Requirement: Failures first, then longest first
Every runner of the project SHALL keep a file of each test's last duration and result, and SHALL run the tests
that failed last time first, then the rest by their last duration, longest first.

#### Scenario: A failure is seen early
- **WHEN** a slice program failed on the last run and the list runs again
- **THEN** it is the first program started

### Requirement: Nothing waits on a run
`check` SHALL start a run that takes longer than a minute in the background with its output in a log file under
`target/check/`, and SHALL print where the log is. No hook, gate or tool of the project SHALL refuse a command or
keep a session going because a run has not happened.

#### Scenario: A long run
- **WHEN** `check full` starts
- **THEN** the command returns at once with the log's path, and the runs go on in the background

### Requirement: The cost of a check is held by tests
The tests of `check` SHALL assert the set of steps it plans for standard changes: a change of documents alone plans
no step; a change of a suite's list or recordings plans that suite's new or changed programs only; a change under
one crate plans the quick layer and nothing else; an edit of libqb naming its built-ins plans the programs that
run those built-ins and the lints and Rust tests. A change that makes any of these plan more SHALL fail a test.

#### Scenario: Documents only
- **WHEN** the working tree differs from the last passing check only under `docs/`
- **THEN** `check quick` plans nothing and says so

#### Scenario: Recordings only
- **WHEN** the working tree adds verification programs and their recordings and changes the two lists
- **THEN** `check quick` plans the compare of the new programs and the list check, and nothing else
