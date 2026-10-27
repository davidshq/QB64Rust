# Spec Delta

## MODIFIED Requirements

### Requirement: The adapter
The extension SHALL start the debug adapter as `qb64rust dap --toolchain-root <folder>`, from the `qb64rust`
binary the language server and builds use (spec `editor/packaging`), with the toolchain folder a build uses. When
no toolchain is found, the launch SHALL offer it as a build does (spec `editor/packaging`, "Toolchain offer") and
start no session until it is installed.

#### Scenario: Bundled binary
- **WHEN** `qb64rust.path` is empty and the extension carries its binary
- **THEN** the debug session's adapter is the carried binary run with `dap` and the toolchain folder

#### Scenario: Toolchain without lldb-dap
- **WHEN** the toolchain folder has no `bin\lldb-dap.exe`
- **THEN** the session ends with the adapter's error naming that path, shown to the user

#### Scenario: QB64pe without lldb-dap
- **WHEN** the toolchain used is the QB64pe folder's and it has no `internal\c\c_compiler\bin\lldb-dap.exe`
- **THEN** the session ends with the adapter's error naming that path, shown to the user

#### Scenario: Debugging without QB64pe
- **WHEN** QB64pe is not found, the toolchain is installed, and the user starts debugging `hello.bas` with a
  breakpoint on line 3
- **THEN** the program is built with `--debug` and stops on line 3
