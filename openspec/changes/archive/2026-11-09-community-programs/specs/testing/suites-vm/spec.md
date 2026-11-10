## MODIFIED Requirements

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
