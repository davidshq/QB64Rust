# Spec Delta

## MODIFIED Requirements

### Requirement: Adapter start and backend
`qb64rust dap` SHALL run a Debug Adapter Protocol server over stdin and stdout. It SHALL start the `lldb-dap` of
the toolchain a build would use (`bin\lldb-dap.exe` of the toolchain folder, spec `compiler/toolchain`) as its
backend and pass every message it does not rewrite to the other side unchanged but for sequence numbers. When
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
