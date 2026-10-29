## MODIFIED Requirements

### Requirement: Adapter start and backend
`qb64rust dap` SHALL run a Debug Adapter Protocol server over stdin and stdout. It SHALL start the `lldb-dap` of
the toolchain a build would use (on Windows `bin\lldb-dap.exe` of the toolchain folder, spec `compiler/toolchain`;
on Linux `bin/lldb-dap` of a toolchain folder given by option or variable, else the `lldb-dap-<n>` on `PATH` with the
highest `<n>` of at least 20, else `lldb-dap` on `PATH`) as its
backend and pass every message it does not rewrite to the other side unchanged but for sequence numbers. An
`lldb-dap-<n>` older than 20 SHALL NOT be started. When
`lldb-dap` cannot be started, the adapter SHALL answer `initialize` with an error naming the path it looked for, or,
with no toolchain found, the message of "No toolchain found". When the backend ends, the adapter SHALL send
`terminated` and end.

#### Scenario: Initialize
- **WHEN** a client sends `initialize`
- **THEN** the adapter answers with `lldb-dap`'s capabilities plus set next line (`supportsGotoTargetsRequest`),
  exception information, and one exception filter, "BASIC runtime errors"

#### Scenario: No lldb-dap
- **WHEN** `qb64rust dap --toolchain-root <dir>` names a folder without `bin\lldb-dap.exe` and a client sends
  `initialize`
- **THEN** the answer is an error whose message names `<dir>\bin\lldb-dap.exe`

#### Scenario: No toolchain
- **WHEN** `qb64rust dap` finds no toolchain and a client sends `initialize`
- **THEN** the answer is an error whose message names `qb64rust toolchain install`

#### Scenario: A versioned lldb-dap on Linux
- **WHEN** `qb64rust dap` runs on Ubuntu where only `/usr/bin/lldb-dap-20` exists, and a client sends `initialize`
- **THEN** the adapter starts `lldb-dap-20` and answers with its capabilities and the additions above

#### Scenario: A versioned lldb-dap before the plain one
- **WHEN** `qb64rust dap` runs on Linux with both `lldb-dap` (the distribution's default) and `lldb-dap-21` on
  `PATH`
- **THEN** the adapter starts `lldb-dap-21`

#### Scenario: Only an older lldb-dap
- **WHEN** `qb64rust dap` runs on Linux where only `lldb-dap-19` is on `PATH`, and a client sends `initialize`
- **THEN** the answer is an error that names the path of `lldb-dap-19`, its version, that 20 or later is needed, and
  the package that provides it on the distribution; no `lldb-dap` is started

#### Scenario: No lldb-dap on Linux
- **WHEN** `qb64rust dap` runs on Linux with no `lldb-dap` or `lldb-dap-<n>` on `PATH`, and a client sends
  `initialize`
- **THEN** the answer is an error that names `lldb-dap` and the package that provides it on the distribution
