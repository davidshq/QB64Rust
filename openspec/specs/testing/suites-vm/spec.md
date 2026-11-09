# testing/suites-vm Specification

## Purpose
Runs the Windows steps that build and run programs in a second Hyper-V test VM, apart from the desktop family's,
so that they put nothing on the host's screen and pass on a clean Windows, not because of what the host has.

## Requirements

### Requirement: The suites VM is made from the base disk
A script SHALL make the VM `qb64rust-suites` with a Windows disk that is a differencing disk on the kept base disk
of the desktop VM's script, and SHALL save a checkpoint of it with its test account logged on and idle. It SHALL
put Python, from a pinned release checked by SHA-256, in the VM. Making it SHALL write no host path into the repo.

#### Scenario: The suites VM is made
- **WHEN** the script makes the suites VM from an existing base disk, from a shell without administrator rights
- **THEN** the VM exists with its checkpoint saved, an activated license and Python, and its Windows disk is a
  child of the base disk

#### Scenario: No base disk
- **WHEN** the script is asked for the suites VM and there is no base disk
- **THEN** it stops, saying to make the base with the ISO first, and makes nothing

#### Scenario: A base disk with a child
- **WHEN** the script is asked to write a new base disk while the suites VM's disk is a child of the old one
- **THEN** it stops and says to remove the suites VM first, since a new base would break it

### Requirement: The suites VM builds with the installed toolchain
The suites VM SHALL get its C++ toolchain through `qb64rust toolchain install --from-zip`, run in the VM while it
is made, with the release's zip downloaded on the host and copied in. A
step in the VM SHALL fail without running its programs when `qb64rust` there would build with another toolchain,
such as the QB64pe clone's.

#### Scenario: The toolchain is installed in the VM
- **WHEN** the suites VM has been made
- **THEN** `qb64rust toolchain path` in the VM names the folder under the test account's `%LOCALAPPDATA%` and says
  it is installed

#### Scenario: The VM would use the clone's toolchain
- **WHEN** a step starts in the VM and `qb64rust toolchain path` there names the clone's toolchain
- **THEN** the step fails, naming the toolchain it found and saying to make the VM again, and builds nothing

### Requirement: Each test VM has its own credential
Each test VM's test account SHALL have its own random password, kept on the host in a file of its own for that
VM, encrypted for the user. Making one VM SHALL NOT replace the credential of another.

#### Scenario: The suites VM is made beside the desktop VM
- **WHEN** the suites VM is made while the desktop VM exists
- **THEN** a run of the desktop family still opens its session in the desktop VM

### Requirement: The suites VM has no network
The suites VM SHALL have no network adapter during a run. Files SHALL go in and out only through the hypervisor.
Its clipboard SHALL NOT be shared with the host's.

#### Scenario: A network program runs in the suites VM
- **WHEN** `s141_tcp_loopback` runs in the suites VM
- **THEN** it connects through `localhost` and passes against its recording, and the VM has no network adapter

### Requirement: What persists between runs
Every run SHALL start from the suites VM's checkpoint. A cache disk, attached after the restore, SHALL keep only
the tree's copy, the QB64pe clone without its git folder, the community clones without their git folders, and the
pass caches of the steps run there. It SHALL keep no build cache. Nothing on the cache disk SHALL be copied to the
host's caches, or from them.

#### Scenario: A file left by a program
- **WHEN** a program of one run writes a file on the VM's Windows disk
- **THEN** the next run does not find it

#### Scenario: A pass served in the VM
- **WHEN** a slice program passed in the VM, and the next VM run has the same build inputs
- **THEN** the program is served from the VM's pass cache, not run, and the host's pass cache is unchanged

#### Scenario: The clone changed on the host
- **WHEN** the host's `..\QB64pe` is at another commit than the clone on the cache disk
- **THEN** the run replaces the VM's clone before any step runs

#### Scenario: The community clones changed on the host
- **WHEN** a community clone on the host is at another commit than its copy on the cache disk
- **THEN** a run with the community step replaces the VM's copy of the community clones before that step runs

### Requirement: The tree is at the host's path in the VM
The tree's copy SHALL be at the same absolute path in the VM as the tree on the host, with the clone at `..\QB64pe`
beside it, so that any path built into a host-built executable is right in the VM. The path SHALL be set at run
time and never written into the repo.

#### Scenario: The CLI tests in the VM
- **WHEN** the CLI test binary built on the host runs in the VM
- **THEN** the files it reads by the paths built into it are found

### Requirement: A step in the suites VM
A step SHALL run in the test account's logged-on session, with the `qb64rust.exe` built on the host. Its log SHALL
be copied to the host's log of the step while it runs, and its results files and coverage records when it ends,
also when it fails or times out. Steps that run on the host after it SHALL read those copied files.

#### Scenario: A whole suite in the VM
- **WHEN** `verification-whole` runs in the VM during `check merge`
- **THEN** its results file and coverage record are on the host when `verification-list` and `builtins` run

#### Scenario: A step that hangs
- **WHEN** a step in the VM does not end within its limit
- **THEN** the step fails as timed out, its log so far is on the host, and the VM is turned off at the end of the run

### Requirement: The VM is used once per run
A `check` run with steps in the suites VM SHALL restore the VM once, before its first VM step, and SHALL turn it
off after its last, whatever their results. It SHALL refuse to take over the suites VM while it is running.

#### Scenario: Three VM steps
- **WHEN** `check full` runs `slice`, `verification` and `cli` in the VM
- **THEN** the VM is restored once, all three run in it, and it is off when the run ends

#### Scenario: The VM is in use
- **WHEN** the suites VM is running when a `check` run with VM steps starts
- **THEN** the run's VM steps fail, saying the VM is running, and the VM is left as it is
