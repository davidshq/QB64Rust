# Spec Delta

## Purpose

Runs the built-ins that act on the desktop, the clipboard or the console in a throwaway Windows Sandbox, so they are
tested by what they do without touching the user's desktop, and compares each result with QB64pe's.

## ADDED Requirements

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
SHALL be written into the repo: the sandbox's configuration is written at run time.

#### Scenario: The clipboard program runs
- **WHEN** the program for `_CLIPBOARDIMAGE` writes an image to the clipboard in the sandbox
- **THEN** the host's clipboard holds what it held before the run

### Requirement: A run that cannot start
The runner SHALL refuse to run, with a message saying why and a nonzero exit, when Windows Sandbox is not installed
or a sandbox is already running. A refused run SHALL NOT count as a pass.

#### Scenario: Windows Sandbox not installed
- **WHEN** the runner is started on a machine without Windows Sandbox
- **THEN** it exits nonzero with a message naming the Windows feature to turn on, and builds nothing

#### Scenario: A sandbox already running
- **WHEN** a sandbox is open when the runner starts
- **THEN** it exits nonzero, saying a sandbox is running, and leaves that sandbox open

### Requirement: Timeouts
Each program SHALL run in the sandbox with a time limit; a program over it SHALL be ended and fail. The whole run
SHALL have a time limit on the host; past it the runner SHALL close the sandbox and fail, reporting the programs that
finished.

#### Scenario: A program that hangs
- **WHEN** a program does not end within its limit
- **THEN** it is ended, it fails as timed out, and the programs after it still run

#### Scenario: The sandbox does not shut down
- **WHEN** the sandbox has not shut down within the run's limit
- **THEN** the runner closes it and fails, naming the programs with results and those without

### Requirement: The family reaches its built-ins
The runner SHALL build `qb64rust`'s executables with `--coverage` and fail when a built-in that
`tests\never_run.list` gives the reason `sandbox` ran in no program of the family.

#### Scenario: A program that no longer calls its built-in
- **WHEN** an edit leaves the call of `_SCREENPRINT` in its program unreached
- **THEN** the run fails, naming `_SCREENPRINT` as listed for the sandbox but not run there
