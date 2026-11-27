# Proposal

## Why

A runtime error names only the line where it happened: `Line: 212 (in army.bm)`, then `Illegal function call`.
It does not say which procedure the line is in, or who called it. On 2026-11-21 a game port hit an error in a
SUB that many places call, and the message could not say which call led there. The debugger shows the call
stack, but only inside a debug session, not when a program fails under a test script or on a player's machine.

## What Changes

- A **call-trace build** keeps a list of the procedures that are running: one frame per `SUB` or `FUNCTION`
  call, with the file and line of the call.
- When such a program reports a runtime error, the message gets a `Call stack:` block after the error's text.
  It lists the procedure the error is in, each caller with the line of its call, and each pending `GOSUB` with
  its line. The first two lines of the message stay as they are.
- The block is in every form of the report: the dialog, the text on stderr under `QB64PE_NOPROMPT`, and the
  report of a critical error (such as error 256, out of stack space).
- A new option `--call-trace` turns it on, and `--no-call-trace` turns it off.
- A new key `call_trace = true` in the `[build]` table of `qb64rust.toml` turns it on for a project.
- A `--debug` build has it on unless it is turned off. So a debug build's C++ is no longer the same as the
  normal build's, and its runtime error message is longer.
- A build without it writes the same C++ as today.
- Not in this change: a function that lets the program read its call stack (user, 2026-11-21: "leave for
  later"), and the trace in builds that did not ask for it.

## Capabilities

### New Capabilities
- `compiler/call-trace`: what a call-trace build records, and the call stack it adds to a runtime error's
  message.

### Modified Capabilities
- `compiler/cli`: a new requirement for the `--call-trace` and `--no-call-trace` options and the `[build]`
  key; "A debug build is the same program" now excepts the call trace; "A bad build table" names the new key.

## Impact

- `crates\codegen-cpp`: a new module `call_trace.rs` and its C++ block `call_trace_rt.h`; `procs.rs` (a
  procedure's start and end), the statement loop (statements that hold a call), `events.rs` (a SUB or label
  run by an event), `stmt.rs` (the table of `GOSUB` lines).
- `crates\driver`: `main.rs` (the options), `settings.rs` (the `[build]` key), `compile.rs` and `lib.rs`
  (passing it down).
- libqb: edits to `internal\c\libqb\src\error_handle.cpp` and its header, a hook that the error report calls
  for more text. It is empty unless the program sets it. `UPSTREAM.md` gets no entry: it lists QB64pe's bugs.
- Documents: `DIVERGENCES-QB64PE.md` (the longer message of a call-trace build), `docs\MANUAL.md` (the
  options), `docs\reference\chapters\error-handling.md` (the message).
- Run time: a call-trace build pays a small cost per call, to be measured. Other builds pay nothing.
