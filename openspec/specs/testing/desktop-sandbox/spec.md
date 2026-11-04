# testing/desktop-sandbox Specification

## Purpose

Runs the built-ins that act on the desktop, the clipboard or the console in a throwaway Windows Sandbox, so they are
tested by what they do without touching the user's desktop, and compares each result with QB64pe's.

## Requirements

### Requirement: The sandbox family
`tests\sandbox\` SHALL hold one or more programs for each built-in that `tests\never_run.list` gives the reason
`sandbox`. A program SHALL write, to the file `out.txt` in its own folder (not to standard output: design D2), the
effect of the call read back by the program itself where the built-in has one that can be read, and otherwise that
the call raised no error and the program went on.

#### Scenario: A built-in with an effect to read back
- **WHEN** the program for `_SCREENMOVE` moves its window to 100, 120
- **THEN** its output holds the values of `_SCREENX` and `_SCREENY` read after the move

#### Scenario: A built-in with nothing to read back
- **WHEN** the program for `_NOTIFYPOPUP` shows a notification
- **THEN** its output holds `ERR` after the call and a line printed after it

### Requirement: Recordings from QB64pe
Each program of the family SHALL have a recording of its output from QB64pe's build of it run in the sandbox. The
runner SHALL record with `qb64pe.exe` when asked, and SHALL otherwise judge `qb64rust`'s build of each program by its
recording.

#### Scenario: Output that differs
- **WHEN** `qb64rust`'s build of a program prints a value its recording does not hold
- **THEN** the run fails, naming the program and showing the difference

#### Scenario: A program with no recording
- **WHEN** a program of the family has no recording
- **THEN** the run fails, naming the program

### Requirement: The sandbox is closed to the host
The runner SHALL start the sandbox with the clipboard, networking, the printer, and audio and video input not shared
with the host, and SHALL share with it only a folder of the run's executables and results. No path of the host
SHALL be written into the repo: the sandbox's configuration is written at run time. The test VM SHALL have no
network adapter during a run; it is online only while it is made, to activate Windows. The runner SHALL exchange
files with it only through the hypervisor: the executables in and the results out. The test VM's clipboard SHALL
NOT be shared with the host's.

#### Scenario: The clipboard program runs
- **WHEN** the program for `_CLIPBOARDIMAGE` writes an image to the clipboard in the sandbox
- **THEN** the host's clipboard holds what it held before the run

#### Scenario: The clipboard program runs in the VM
- **WHEN** the program for `_CLIPBOARDIMAGE` writes an image to the clipboard in the test VM
- **THEN** the host's clipboard holds what it held before the run

### Requirement: A run that cannot start
The runner SHALL refuse to run, with a message saying why and a nonzero exit, when Windows Sandbox is not installed
or a sandbox is already running. A refused run SHALL NOT count as a pass. In the test VM it SHALL refuse in the same
way when Hyper-V's tools cannot be used, the VM or its checkpoint does not exist, the VM is running when the run
starts, or the VM's evaluation license has expired. The message SHALL name what to do.

#### Scenario: Windows Sandbox not installed
- **WHEN** the runner is started on a machine without Windows Sandbox
- **THEN** it exits nonzero with a message naming the Windows feature to turn on, and builds nothing

#### Scenario: A sandbox already running
- **WHEN** a sandbox is open when the runner starts
- **THEN** it exits nonzero, saying a sandbox is running, and leaves that sandbox open

#### Scenario: No test VM
- **WHEN** the runner is started for the VM on a machine where the VM was never made
- **THEN** it exits nonzero, naming the script that makes it and the choice to run in Windows Sandbox instead

#### Scenario: The VM is in use
- **WHEN** the test VM is running when the runner starts
- **THEN** it exits nonzero, saying the VM is running, and leaves the VM as it is

#### Scenario: The evaluation license has expired
- **WHEN** the VM's evaluation period has ended
- **THEN** the runner exits nonzero, saying to make the VM again with the script

### Requirement: Timeouts
Each program SHALL run in the sandbox with a time limit; a program over it SHALL be ended and fail. The whole run
SHALL have a time limit on the host; past it the runner SHALL close the sandbox and fail, reporting the programs that
finished. The same limits SHALL hold in the test VM; past the run's limit the runner SHALL turn the VM off and fail
in the same way.

#### Scenario: A program that hangs
- **WHEN** a program does not end within its limit
- **THEN** it is ended, it fails as timed out, and the programs after it still run

#### Scenario: The sandbox does not shut down
- **WHEN** the sandbox has not shut down within the run's limit
- **THEN** the runner closes it and fails, naming the programs with results and those without

#### Scenario: The VM's run does not finish
- **WHEN** the run in the test VM has not finished within the run's limit
- **THEN** the runner turns the VM off and fails, naming the programs with results and those without

### Requirement: The family reaches its built-ins
The runner SHALL build `qb64rust`'s executables with `--coverage` and fail when a built-in that
`tests\never_run.list` gives the reason `sandbox` ran in no program of the family.

#### Scenario: A program that no longer calls its built-in
- **WHEN** an edit leaves the call of `_SCREENPRINT` in its program unreached
- **THEN** the run fails, naming `_SCREENPRINT` as listed for the sandbox but not run there

### Requirement: Where the family runs
The runner SHALL run the family in the test VM by default, and in Windows Sandbox when asked. Building, judging,
recording and the coverage check SHALL be the same in both. A run in the test VM SHALL put no window on the host's
screen and SHALL leave the host's display scale as it found it.

#### Scenario: A run with no choice given
- **WHEN** the runner is started without saying where to run
- **THEN** the programs run in the test VM, and the host's screen shows no window of the run

#### Scenario: A run in Windows Sandbox
- **WHEN** the runner is asked to run in Windows Sandbox
- **THEN** the programs run there as before this change, judged against the same recordings

#### Scenario: A recording run in the VM
- **WHEN** a recording run happens in the test VM
- **THEN** the host's display scale is not changed

### Requirement: The test VM is made once
A script SHALL make the test VM from a Windows 11 Enterprise evaluation ISO that the user supplies, without
showing Windows setup. When the VM is ready, the script SHALL save a checkpoint of it with its test account logged
on and idle. Making the VM SHALL write no path of the host into the repo. The script SHALL keep a copy of the
disk from before its first boot, and SHALL make the VM again from that copy without administrator rights.

#### Scenario: The VM is made again
- **WHEN** the VM has been removed and the script is run from the kept disk, from a shell without administrator
  rights
- **THEN** a VM exists with its checkpoint saved and an activated license

#### Scenario: The VM is made
- **WHEN** the script is run once, as administrator, with the ISO's path
- **THEN** a VM exists with its checkpoint saved, and nothing under the repo's tracked files names a host path

#### Scenario: The VM already exists
- **WHEN** the script is run while a VM of that name exists
- **THEN** it stops without changing that VM and says how to remove it first

### Requirement: Each run starts from the checkpoint
Every run in the test VM SHALL start from the saved checkpoint, so that no run sees files, settings or processes
left by an earlier run. After the run the VM SHALL be turned off.

#### Scenario: A program left a file behind
- **WHEN** a program of one run leaves a file in the VM
- **THEN** the next run does not find that file

#### Scenario: A run ends
- **WHEN** a run in the VM ends, passed or failed
- **THEN** the VM is off
