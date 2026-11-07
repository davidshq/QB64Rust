# testing/check-workflow Specification

## Purpose
The one entry point for testing the project: its three layers, what makes a recorded result valid, the order the
runs go in, and the tests that keep the entry point's own cost from growing.

## Requirements

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
run those built-ins and the lints and Rust tests; a change to a Python tool outside `tools/check` and the folders
`check`'s tests read plans the Python tools' tests without `check`'s own. A change that makes any of these plan more
SHALL fail a test.

#### Scenario: A tool changed
- **WHEN** the working tree differs from the last passing check only in `tools/diagnostics/old_wording.py`
- **THEN** `check quick` plans `tools` and not `tools-check`

#### Scenario: Documents only
- **WHEN** the working tree differs from the last passing check only under `docs/`
- **THEN** `check quick` plans nothing and says so

#### Scenario: Recordings only
- **WHEN** the working tree adds verification programs and their recordings and changes the two lists
- **THEN** `check quick` plans the compare of the new programs and the list check, and nothing else

### Requirement: One Linux copy per source tree
The Linux steps SHALL run in a WSL copy that follows one Windows source tree, named from that tree's folder, so
that the checkout and a worktree each have their own. The copy SHALL be brought up to date from the files git
reports changed since the last sync, without a scan of the tree, and from every file compared by content when the
run is forced.

The Linux steps SHALL show nothing on the host's desktop and play nothing on its speakers. Each SHALL run on a
virtual display of its own (Xvfb), with WSLg's Wayland and PulseAudio servers out of reach.

#### Scenario: A window program in a Linux step
- **WHEN** a Linux step runs a program that opens a window or plays a sound
- **THEN** the window opens on the step's virtual display, the host's desktop shows nothing, and the host's
  speakers play nothing

#### Scenario: A worktree's run
- **WHEN** `check full` runs in the worktree `QB64Rust-full`
- **THEN** its Linux steps sync to and run in `~/qb64rust-full`, and the checkout's `~/qb64rust` is untouched

#### Scenario: Nothing changed
- **WHEN** the tree's copy was synced from it and no file changed
- **THEN** the sync gives rsync only the files `git status` reports (the tree's uncommitted edits), scans no
  other file, and finishes in seconds

#### Scenario: An edit undone
- **WHEN** a file was edited at the last sync and the edit is undone before this one
- **THEN** the file is copied again, as it was at the last sync's status list

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

### Requirement: A step has a place
Each step SHALL run on the host or in the suites VM. A step that builds and runs programs (a list or whole suite of
the runner, the call-site check, the coverage check, the CLI tests) SHALL be able to run in either. Every other
step SHALL run on the host. `--where host` or `--where vm` SHALL choose the place of every step that has a choice.

#### Scenario: One step in the VM
- **WHEN** `check verification --where vm` runs
- **THEN** the build runs on the host and the verification list runs in the suites VM

#### Scenario: A host-only step
- **WHEN** `check quick --where vm` runs
- **THEN** `fmt`, `build`, `clippy`, `clippy-linux`, `nextest`, `lists`, `doc`, `tools` and `tools-check` run on the
  host, and `slice` runs in the VM

### Requirement: The place by default
Without `--where`, `check quick` and a step or program named alone SHALL run on the host, and `check full` and
`check merge` SHALL run their steps that have a choice in the suites VM. When the suites VM cannot be used,
`full` and `merge` SHALL run those steps on the host and print one line naming why. With `--where vm` they
SHALL instead fail those steps with that reason.

#### Scenario: A full run on a machine with the VM
- **WHEN** `check full` runs where the suites VM exists
- **THEN** the steps that run programs run in the VM, and no window of theirs appears on the host's screen

#### Scenario: A full run without Hyper-V
- **WHEN** `check full` runs on a machine without Hyper-V
- **THEN** every step runs on the host, and the run's report says once that the suites VM could not be used and why

#### Scenario: The VM asked for but missing
- **WHEN** `check full --where vm` runs where the suites VM was never made
- **THEN** the steps that run programs fail, naming the script that makes the VM, and the host-only steps run

### Requirement: The place is part of a result
A step's record SHALL hold the place it ran in, and its fingerprint SHALL include it. A pass in one place SHALL NOT
stand for a run planned in the other.

#### Scenario: A host pass, then a VM run
- **WHEN** `slice` passed on the host and `check slice --where vm` runs with nothing changed
- **THEN** `slice` runs in the VM

#### Scenario: The status of a step
- **WHEN** `check status` lists a step whose last pass was in the VM
- **THEN** its line says the pass was in the VM

### Requirement: A full run on uncommitted changes
`check full --uncommitted` SHALL run where `check full` would refuse uncommitted changes under `crates/`, `libqb/`
or `tests/`, and its report SHALL say once that it ran on uncommitted changes. `check merge` SHALL refuse the
switch.

#### Scenario: A change tested whole before its commit
- **WHEN** `check full --uncommitted` runs in a tree with an uncommitted change under `crates/`
- **THEN** the run's steps are planned and run, and the report has one line saying the run is on uncommitted changes

#### Scenario: A merge with the switch
- **WHEN** `check merge --uncommitted` runs
- **THEN** it refuses, saying the switch is for `check full` only

### Requirement: The toolchain fallback is reported
When a `check` run has a step on the host that builds programs, and `qb64rust` there would build with the QB64pe
clone's toolchain rather than an installed one, the run's report SHALL say so in one line naming
`qb64rust toolchain install`. The run SHALL go on.

#### Scenario: A host without the installed toolchain
- **WHEN** `check quick` runs the slice list on a host where `qb64rust toolchain path` names the clone's toolchain
- **THEN** the report has one line saying the builds use the clone's toolchain and how to install the toolchain,
  and the slice list runs
