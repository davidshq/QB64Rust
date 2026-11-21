## MODIFIED Requirements

### Requirement: The suites VM is made from the base disk
A script SHALL make the VM `qb64rust-suites` with a Windows disk that is a differencing disk on the kept base disk
of the desktop VM's script, and SHALL save a checkpoint of it with its test account logged on and idle. It SHALL
put Python and Node.js, each from a pinned release checked by SHA-256, in the VM. Making it SHALL write no host
path into the repo.

#### Scenario: The suites VM is made
- **WHEN** the script makes the suites VM from an existing base disk, from a shell without administrator rights
- **THEN** the VM exists with its checkpoint saved, an activated license, Python and Node.js on the PATH, and its
  Windows disk is a child of the base disk

#### Scenario: No base disk
- **WHEN** the script is asked for the suites VM and there is no base disk
- **THEN** it stops, saying to make the base with the ISO first, and makes nothing

#### Scenario: A base disk with a child
- **WHEN** the script is asked to write a new base disk while the suites VM's disk is a child of the old one
- **THEN** it stops and says to remove the suites VM first, since a new base would break it

### Requirement: What persists between runs
Every run SHALL start from the suites VM's checkpoint. A cache disk, attached after the restore, SHALL keep only
the tree's copy, the QB64pe clone without its git folder, the community clones without their git folders, the pass
caches of the steps run there, the copies of the game ports' repos and of the folders beside them that their runs
named, and the pinned Node.js of a VM made without it. It SHALL keep no build cache. Nothing on the cache disk
SHALL be copied to the host's caches, or from them.

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

## ADDED Requirements

### Requirement: A game port's run in the suites VM
A script SHALL run one command of a game's repo, which is beside this repo, in the suites VM. It SHALL start and
stop the VM as `check` does, copy in this repo, the game, and the other folders beside them that the run names,
put Node.js on the command's PATH, and copy the named outputs back to the game's folder, also after a failure.
When the VM has no installed Node.js, the script SHALL put the pinned release on the cache disk, checked by
SHA-256 on the host.

#### Scenario: Node is not installed in the VM
- **WHEN** a game's run starts in a suites VM made without Node.js
- **THEN** the pinned Node.js is unpacked on the cache disk, once, and the command finds `node` on its PATH

#### Scenario: Node is installed in the VM
- **WHEN** a game's run starts in a suites VM made with Node.js
- **THEN** the command uses the installed one, and nothing of Node is copied

#### Scenario: The command fails
- **WHEN** the game's command ends with a non-zero exit code
- **THEN** the script ends with that code, the outputs are copied back, and the VM is turned off

#### Scenario: A folder that is not beside this repo
- **WHEN** the game, or a folder the run names, is not beside this repo, or is this repo, the QB64pe clone or
  the community clones
- **THEN** the script refuses before the VM is started
