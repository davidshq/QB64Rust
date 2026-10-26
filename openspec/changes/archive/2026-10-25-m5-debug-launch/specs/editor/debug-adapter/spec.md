# Spec Delta

## MODIFIED Requirements

### Requirement: Launch
`launch` SHALL take the executable of a debug build (`program`) and read its symbol file `<program>.debug.json`;
it SHALL refuse an executable without one, an executable modified later than its symbol file (built again by
another build, which left the symbol file of an earlier one), or one with a symbol file of a version it does not
read, with an error message that says to build with `--debug`. It SHALL pass `args`, `cwd` (by default the
executable's folder), `stopOnEntry` and `console` to `lldb-dap`, and set `QB64PE_NOPROMPT` in the program's
environment when `noPrompt` is true. With `stopOnEntry` the program SHALL stop at its first BASIC line, not inside
libqb's start.

#### Scenario: Not a debug build
- **WHEN** `launch` names an executable built without `--debug`
- **THEN** the answer is an error that names the missing `<program>.debug.json` and `--debug`, and no process
  is started

#### Scenario: Built again by QB64pe
- **WHEN** `launch` names an executable that QB64pe built after a debug build of the same program left its
  `<program>.debug.json`
- **THEN** the answer is an error that says the executable is newer than its debug symbol file and names
  `--debug`, and no process is started

#### Scenario: Stop on entry
- **WHEN** `launch` has `stopOnEntry: true` for a program whose first statement is on line 3
- **THEN** the first `stopped` event is on line 3 of the main file, in the main module's frame
