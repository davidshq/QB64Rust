# Proposal

## Why

M5 takes Path A of `study\36` (user, `DECISIONS.md` 2026-10-24). Its first change, `m5-debug-build-symbols`
(archived 2026-10-25), gave a debug build, the symbol file `<exe>.debug.json` and the measured facts of how the
clone's `lldb` steps a program of this compiler (its design, "Measured"). What a user still cannot do is debug in
VS Code in BASIC terms: the toolchain's `lldb-dap` would show `__LONG_TOTAL`, `SUB_SHOW`, every libqb global and
thread, and would stop in `data1.txt`, past a file's end and at the rest of a `CALL`. This change is ROADMAP M5's
third item: `qb64rust dap`, a thin proxy in front of `lldb-dap` that rewrites the few messages that must speak
BASIC and passes everything else through.

One risk comes first: `debug_next_steps_by_line` hung once under the load of the whole `--ignored` run (2026-10-25,
`STATUS.md`): the clone's `lldb` never came back from a `next` over a `PRINT`. A `next` that hangs over a call into
libqb would hang the proxy's users too, so the change starts by finding out why.

## What Changes

- **The `next` hang, looked at first.** Reproduced under load and its cause found or bounded (what `lldb` waits
  on); the proxy's design takes the result (a fix, a bound, or a step limit with a message).
- **`qb64rust dap`**: a Debug Adapter Protocol server over stdin and stdout that starts the clone's `lldb-dap`
  (`internal\c\c_compiler\bin\lldb-dap.exe`, found as a build finds the clone) and forwards every message both
  ways, but for these:
  - `launch`: takes a debug build's executable (`program`), reads its symbol file (an executable without one is
    refused with a message naming `--debug`), passes `args`, `cwd`, `stopOnEntry`, `console` and
    `QB64PE_NOPROMPT` (`noPrompt`) to `lldb-dap`.
  - **Threads and the stack**: one thread, the program's (`QBMAIN`'s); libqb's own threads hidden; frames shown by
    BASIC name (`SUB show`, `FUNCTION twice&`, the main module), the frames of libqb, the thread wrapper and
    `evnt` folded away, so a handler's frame sits above the line that raised the error.
  - **Stepping**: `next`, `stepIn`, `stepOut` stop only on a line of a file the symbol file lists, past a
    procedure's header line and not past a file's end; after a return, the rest of a `CALL` is stepped past (the
    rules of the measurement).
  - **Variables**: scopes "Locals" (the frame's parameters, locals, statics and result) and "Main module"; each
    variable by its BASIC name and type, its value read from the debuggee's memory through the symbol file's
    `c_name`, `reach` and layouts; strings, arrays (elements by index within the bounds the descriptor holds at
    that moment) and user types (members by offset) expand.
  - **Evaluate**: a BASIC name, an element with constant subscripts or a member, for hover, watch and the Debug
    Console, with `,h` / `,b` / `,o` display suffixes and DAP's hex format; anything else typed in the Debug
    Console goes to `lldb` as it is.
  - **Set value**: numbers by type, `STRING * n` in place, and a `STRING` through libqb's own string routine if
    the measurement shows `lldb` can call it (else left out and recorded).
  - **Data breakpoints**: a variable or element of up to 8 bytes as a hardware watchpoint.
  - **Set next line** (`gotoTargets`/`goto`): a line of the current procedure, by `lldb`'s `thread jump`.
  - **Stop on a runtime error**: an exception filter "BASIC runtime errors"; a stop shows the error's number at
    the raising line.
- **Tests**: tier 1 drives the proxy against a stand-in backend of scripted replies (one scenario per rewritten
  request); ignored CLI tests drive the real `lldb-dap` through the proxy on debug builds.

Out of this change: the extension's `debuggers` contribution and launch configuration (ROADMAP M5's fourth
item, the next change); building the program from `launch` (the extension builds, then launches the executable);
a BASIC expression evaluator (operators, function calls in a watch); "skip lines" (`SOMEDAY.md`); showing values
formatted exactly as `PRINT` would.

## Capabilities

### New Capabilities

- `editor/debug-adapter`: the debug adapter `qb64rust dap`: what it starts, what it shows and how it steps, which
  requests it answers in BASIC terms, and what it passes to `lldb-dap` unchanged.

### Modified Capabilities

- `compiler/cli`: the subcommand `qb64rust dap` (as `qb64rust lsp`), with `--qb64pe-root`.

## Impact

- New crate `crates\dap` (`qb64rust-dap`): the proxy; depends on `qb64rust-base` and `serde_json` only, not on
  `sema` or `codegen-cpp` (it reads the symbol file). No new external dependency.
- `crates\driver`: `main.rs` dispatches `dap`, finds the clone and hands the proxy `lldb-dap`'s path; CLI tests
  that drive the proxy end to end, beside the `lldb` tests of `m5-debug-build-symbols`.
- `crates\README.md`: the crate map, the run line for `qb64rust dap`, the tests.
- `DECISIONS.md`: what the hang's investigation decides, and set value of a `STRING` if left out. `SOMEDAY.md`:
  rows for what stays out (value formatting as `PRINT`, expressions). `DIVERGENCES.md`: nothing (a program's
  behaviour does not change; D-017 covers the old IDE's debugger).
- The `--ignored` CLI run grows by the proxy's end-to-end tests (seconds each, one at a time as the `lldb` tests).
