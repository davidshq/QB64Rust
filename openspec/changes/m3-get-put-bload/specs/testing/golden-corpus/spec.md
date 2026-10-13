# Spec Delta

## ADDED Requirements

### Requirement: Data files beside a program
A folder `<name>.files` beside a corpus program SHALL be copied, with everything in it, into the folder the
program is compiled and run in, before it is compiled, for a run and for a recording and with either compiler.
The files SHALL be part of what the pass cache compares: a program whose data file changed is run again. The
folder beside a program that does not run (one with an `.err` or `.norun` file) SHALL be an error in the test.
Git SHALL store the files byte for byte.

#### Scenario: A program that loads a file
- **WHEN** `g07_mountains.bas` runs `BLOAD "mtn.vga", VARPTR(mtn(1))` and `g07_mountains.files\MTN.VGA` exists
- **THEN** the program finds the file with either compiler and passes by the state of its screen

#### Scenario: A changed data file
- **WHEN** a program passed with `--pass-cache`, then a file of its `.files` folder changes
- **THEN** the next run with `--pass-cache` runs the program again

#### Scenario: Data files beside a program that does not run
- **WHEN** a program has an `.err` file and a `.files` folder
- **THEN** the runner reports a failure of the test that names the two
