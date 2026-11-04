# Spec Delta

## ADDED Requirements

### Requirement: A step has a place
Each step SHALL run on the host or in the suites VM. A step that builds and runs programs (a list or whole suite of
the runner, the call-site check, the coverage check, the CLI tests) SHALL be able to run in either. Every other
step SHALL run on the host. `--where host` or `--where vm` SHALL choose the place of every step that has a choice.

#### Scenario: One step in the VM
- **WHEN** `check verification --where vm` runs
- **THEN** the build runs on the host and the verification list runs in the suites VM

#### Scenario: A host-only step
- **WHEN** `check quick --where vm` runs
- **THEN** `fmt`, `build`, `clippy`, `nextest`, `lists`, `doc` and `tools` run on the host, and `slice` runs in
  the VM

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
