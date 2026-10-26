# Design

## Context

For the motivation see `proposal.md` ("Why"), `study\36` §3.1 and §4.2; for the behaviour, the delta specs. How the
parts stand (2026-10-25, branch `m5` after `m5-debug-build-symbols`):

- **The symbol file** (spec `compiler/debug-info`, `codegen-cpp\src\debug.rs`): `version` 1; `files` (index, path
  as the `#line` directives name it, hash); `procedures` (name, kind, `c_name`, `file`, `line`, `end_file`,
  `end_line`, parameters, result); `variables` (name in upper case, type as BASIC spells it, storage with the
  procedure's index, shape, `conventional`, `c_name`, `reach`: `pointer`, `descriptor`, `string`, `bytes`,
  `param-pointer`); `types` (size, members with offset, size, bounds, `dynamic`); `layouts` (pointer size, the
  array descriptor's slots, `qbs` `chr` at 0 and `len` at 8, `_FLOAT` 32 bytes in an array). It has no line count
  per file and no record of `SHARED`. Snapshots of five programs: `tests\frontend\debug_info_*.bas`, written by
  `driver\tests\frontend.rs` under `crates\driver\tests\snapshots\`.
- **The measured facts** (design of `m5-debug-build-symbols`, "Measured"): `next` stops once per BASIC line, at
  column 1, and never at the `evnt` check; `step` into a procedure stops at its header line (twice), in
  `is_error_pending()` (`error_handle.h`), in a procedure's data fragment (`data1.txt`) for an included file, then
  at the first statement; `finish` and a `next` off a procedure's end land on the caller's line at a column other
  than 1 (the rest of the `CALL`); after a `RESUME` the stops are past the file's end (line 12 of a 9-line file)
  and in `evnt` in `qbx.cpp`; a handler's frame is `QBMAIN` above `evnt` above the raising `QBMAIN`; the main
  module's variables are C globals seen from every frame; `breakpoint set -n error` resolves to libqb's `error(int)`
  (no debug information), its code is `(int)$ecx`, and frame 1 is the raising line.
- **The hang** (`STATUS.md`, "Still to watch"): once, under the load of the whole `--ignored` run, the clone's
  `lldb` did not come back from a `next` over a `PRINT` in `debug_next_steps_by_line`; alone the test passes in 6 s.
  Not understood. `lldb`'s default run mode for a step over a range runs only the stepping thread while it
  instruction-steps and all threads while it runs to a return address; a `PRINT` of a `$CONSOLE:ONLY` program takes
  libqb locks another thread may hold. A hypothesis, nothing more.
- **The language server is the model of a server in the binary**: `crates\lsp` (`run_stdio(root, check)`),
  started by `driver\src\main.rs` `lsp()` when the first argument is exactly `lsp`, on a thread with
  `with_stack`; its tests drive `serve` in process over a pair of connections (`lsp\tests\server.rs`).
- **The clone** is found by `driver\src\build.rs` `find_root` (`--qb64pe-root`, `QB64RUST_QB64PE_ROOT`, the
  default next to the repository); `lldb-dap.exe` 23.1.2 is in its `internal\c\c_compiler\bin`, beside `lldb.exe`
  and `liblldb.dll`. Its `initialize` answer is in `study\36` §2 item 4.
- **The error texts** are libqb's: `libqb\internal\c\libqb\src\error_handle.cpp`, a `switch` from the number to
  the text (`case 5: return "Illegal function call";`).

## Goals / Non-Goals

**Goals:**

- A proxy whose rewrites are each a small function over a symbol file and a few backend round trips, testable
  in tier 1 against a scripted backend, and whose passthrough is byte-for-byte `lldb-dap`'s but for `seq`.
- Every fact the proxy depends on that was not measured by `m5-debug-build-symbols` (reading memory, writing it,
  data breakpoints, `thread jump`, calling `qbs_set`, stop on entry, pause) measured against the real `lldb-dap`
  by an ignored CLI test before the rewrite that uses it is relied on.

**Non-Goals:**

- Matching the old IDE's debugger message for message (D-017: there is none to match in a program of this
  compiler). Display formats are this design's own.
- A BASIC expression evaluator, `_BIT` values, `_MEM` blocks' fields (shown as "not shown yet"; `SOMEDAY.md` rows).
- More than one program per session, attach, restart, or debugging a program that is not a `--debug` build.
- Any change to the compiler, the symbol file or libqb's copy. If a fact needs one (a line count per file, the
  `SHARED` list), it is a version 2 of the file in a later change; this design works around both (D5, D7).

## Decisions

### D1. The hang is looked at first, and the result decides one setting

Task 1 reproduces it: `debug_next_steps_by_line` run in a loop beside a load like the closing set's (the slice
list at `--jobs 8`), with `lldb`'s `log enable lldb step thread process -f <log>` added to the commands when an
environment variable of the test asks for it, and the stuck processes' stacks taken when the deadline passes (the
clone's `lldb` attached to the stuck program after the stuck `lldb` is killed, `thread backtrace all`). What
follows from what is found:

- A thread deadlock while only the stepping thread runs → the proxy steps with all threads running
  (`singleThread: false`, and `settings set target.process.thread.step-out-avoid-nodebug true` as needed, passed
  in `launch`'s `initCommands`), and the CLI test says the same.
- Something else with a remedy on our side → that remedy, in this change if it is the proxy's or the test's, a
  row in `SOMEDAY.md` otherwise.
- Not reproduced in a bounded try (an hour of machine time) → recorded in `STATUS.md` as before, and the proxy
  goes ahead: a user can always `pause` a step that does not end (the spec's requirement "One thread, the
  program's"), so a hang costs a click, not a session.

No step watchdog: a `next` over `INPUT` or `SLEEP 20` is a long step by design, and a timer cannot tell it from a
hang.

*Amended by "Measured" (task 1.2):* the hang was none of the branches above: the stepping thread did not run alone
(`stop others = 0` in the log), and the cause is `lldb`'s own Windows process plugin losing the stop that ends a
step. The remedy is the backend's: `lldb` and `lldb-dap` run with `LLDB_USE_LLDB_SERVER=1` (debugging through
`lldb-server`), the proxy's `LLDB_ENV`, used by the proxy's backend and by the CLI tests' `lldb` alike.

### D2. A crate `dap`, one thread of state, two reader threads, blocking internal requests

`crates\dap` (`qb64rust-dap`), depending on `qb64rust-base` and `serde_json`. Not the `dap` crate of crates.io:
the proxy forwards `serde_json::Value` and reads a handful of fields of the messages it rewrites, so typed
request structs would be code to keep in step with DAP for nothing.

- `wire.rs`: the framing (`Content-Length: n\r\n\r\n` and a JSON body), `read_message(&mut impl BufRead)` and
  `write_message(&mut impl Write, &Value)`.
- `proxy.rs`: `Proxy<C: Write, B: Write>` owns all state. Two threads read the client and the backend and send
  each message into one `mpsc` channel tagged with its side; the main loop takes them in order. The proxy numbers
  every message it writes, per side, and keeps a map from the `seq` of each request it sends the backend to its
  origin: a client request (answered with the client's `seq` as `request_seq`) or an internal one. A reverse
  request of `lldb-dap` (`runInTerminal`) is mapped the same way in the other direction.
- An internal request (a `readMemory`, an `evaluate`, a `stackTrace` the proxy needs for a rewrite) is
  **blocking**: `Proxy::call(command, args) -> Result<Value, String>` writes it and takes messages from the
  channel until its response arrives; the other messages that arrive meanwhile are queued and handled after the
  current one, in order. Client requests are handled one at a time; a passthrough request does not wait for its
  response. *Alternative:* continuations per request (an `async` state machine). Rejected: no async runtime in the
  workspace, and VS Code's concurrent requests (`threads`, `stackTrace`, `scopes` after a stop) are answered fast
  enough in order; the blocking calls are a few round trips each over local pipes.
- `run_stdio(lldb_dap: PathBuf) -> Result<(), String>` starts the backend (`std::process::Command`, stdin and
  stdout piped, stderr inherited so that `lldb-dap`'s own complaints reach the editor's output) and runs the loop
  over stdin and stdout. `serve(client_in, client_out, backend_in, backend_out)` is the testable core.

### D3. The symbol file is read once, at `launch`, into plain structs

`symbols.rs`: `Symbols::read(path) -> Result<Symbols, String>` with `serde_json::Value` lookups into `File`,
`Procedure`, `Variable`, `Type`, `Layouts` (no `serde` derive; the workspace has none). A `version` other than 1, or
a missing field, is an error naming the field; `launch` turns it into its spec'd refusal. Indices: the procedure
of a C name (`c_name` up to `(`, as `lldb` names the frame `SUB_SHOW(int*)`), the variables of a procedure and of
the main module by name, and the listed file of a path (D5). A tier 1 test reads every `debug_info_*` snapshot of
`crates\driver\tests\snapshots\` and parses it with `Symbols`, so a change of the emitter's file is a test
failure here in the same run (the snapshots are the file's text).

### D4. Launch: the proxy builds `lldb-dap`'s `launch`; stop on entry is its own

`launch.rs`: `program` must be a file with `<program>.debug.json` beside it (else the spec'd error, and no
backend `launch`). The backend's `launch` gets `program`, `args`, `cwd` (default the executable's folder), `env`
with `QB64PE_NOPROMPT=y` when `noPrompt`, `runInTerminal: true` when `console` is `integratedTerminal` or
`externalTerminal`, `stopOnEntry: false`, and the `initCommands` of D1 if any. `stopOnEntry` is done by the proxy:
an internal function breakpoint on `QBMAIN` (merged with the client's, D9), which on its hit is removed and
followed by one step under D6's rules to the first BASIC line, reported as `stopped` with reason `entry`.
`lldb-dap`'s own stop on entry stops in the loader, before libqb has started the program's thread.

*Amended by "Measured":* the breakpoint on `QBMAIN` is set in every session, not only with `stopOnEntry`: its hit
is how the proxy learns the program's thread (D11). Without `stopOnEntry` the proxy continues from it. Its stop is in
`qbx.cpp`, and the way to the first BASIC line is about twenty `next`s (D6's rules step on through them), the last
landing on line 1 of the main file whatever is there; on entry a line with no statement (blank, a comment, a
metacommand) is stepped past too.

### D5. Which frames are BASIC code, and the stack

`stack.rs`: a frame is BASIC code when its `source.path` is a file of `files` (compared as normalised absolute
paths without case; a path in `files` that is relative is matched by suffix against the frame's, since the main
file's `#line` path is as given to the compiler) and its line is at most the file's line count. The count is not in
the symbol file: the proxy reads the file once (the `files` path, resolved against the executable's folder when
relative) and counts its lines; a file it cannot read has no count, and only the first test applies to it.

`stackTrace` (the program's thread only): the proxy asks the backend for all frames (at most 1000), keeps the BASIC
ones, names each by its procedure (`c_name` → `SUB <name>` / `FUNCTION <name>`, `QBMAIN` → "Main module"), keeps
`lldb-dap`'s frame `id`s (so `scopes`, `evaluate` and `variables` name the right frame) and pages the result by the
client's `startFrame` and `levels` itself. A handler's frame above the raising line falls out of this with no
special case: `evnt` and the frames between are not BASIC code.

### D6. Stepping: forward, then look, then step on or report

`step.rs`: a client `next`, `stepIn` or `stepOut` is answered at once and sent to the backend as an internal
request; the proxy remembers the kind and the frame depth and procedure it started from. When the backend's
`stopped` event with reason `step` comes, the proxy takes the top frame (an internal `stackTrace` of 2 levels) and
decides:

| Where the stop is | Then |
|---|---|
| a frame that is not BASIC code (libqb, `is_error_pending`, `data1.txt`, `evnt`, past a file's end) | `stepOut` |
| a procedure's header line (`procedures[].line` in its `file`) | `stepIn` (on to the first statement) |
| the caller's frame after a return (depth less than at the start), at a column other than 1 | `next` |
| anything else | report: the `stopped` event goes to the client, on the program's thread |

*Amended by "Measured":* the first row is two: a frame of a function that is no BASIC code is stepped out of; a
frame of the main module or a procedure on a line of no listed file (`qbx.cpp`, `maindata.txt`, `data1.txt`) or
past a file's end is stepped on with `next` (a `stepOut` there would leave the BASIC function). A header line is
stepped on with `next`, not `stepIn`, which would enter `is_error_pending` again. The depth is the backend's
`totalFrames` of the program's thread. The rules are in `step.rs`'s table.

*Amended after review (2026-10-25):* the third row also covers a stop after a `stepOut` of code that is no BASIC
code, at any depth: `stepIn` on `s$ = STR$(z)` enters libqb's inline `qbs_cleanup` (`qbs.h`), and its `stepOut`
comes back to the same line at column 5, at the depth the step began at, which the row's depth test missed
(`dap_step_in_on_lines_without_a_call`). A client's `stepIn` goes on with `stepIn` there, so that a call later in
the line is still entered; `next` and `stepOut` go on with `next`.

At most 200 steps on one client step; past that, the proxy reports the stop where it is with a `output` event
saying the step was cut short (a bound on a rule's mistake, not on a long statement: each step on is a
short `lldb` step). A breakpoint, a pause, an exception or an exit that arrives during the steps ends them and is
reported as itself.

### D7. Variables: read memory, format in the proxy

`vars.rs`. The proxy holds a table of `variablesReference`s for the current stop (cleared on every `continued`
and step): a scope (frame, Locals or Main module) or a value (frame, address, BASIC type, shape). Reading:

1. The C name's value is the address the `reach` starts from (every reach's C name is a pointer variable):
   `evaluate` of `(unsigned long long)<c_name>` in the frame. An error (a local before its allocation) or a null
   is "not allocated yet".
2. `readMemory` from that address: the value's bytes for `pointer`, `param-pointer` and `bytes`; the `qbs`'s first
   16 bytes for `string` (`chr`, `len` by `layouts.qbs`), then `len` bytes from `chr` (at most 4096, then "…");
   the descriptor's slots for `descriptor` (`layouts.array`), then elements from the data pointer by index.
3. Formatting by type (`format.rs`): integers exact (signed or `_UNSIGNED` by the type text), `SINGLE` and `DOUBLE`
   by Rust's shortest round-trip text, `_FLOAT` (80-bit extended in 16 bytes) converted to `f64` for display,
   `_OFFSET` in hex; strings quoted, with bytes outside 32–126 written as BASIC would build them
   (`"a" + CHR$(0) + "b"`); an array as `(1 TO 3) AS SINGLE` with children; a user type as its name with members
   as children, each at its offset; `_BIT` and `_MEM` as "not shown yet".

The main module scope reads its variables from whichever frame is asked (they are C globals). Locals of a frame
further down are read with that frame's id. *Alternative:* C expressions per value evaluated by `lldb`, as the
measurement did. Rejected: a round trip per value and `lldb`'s C formatting (`int32_t`, `'\0'`); the bytes and
the symbol file's types are enough, and `_FLOAT`, fixed strings and `_UNSIGNED` come out right.

### D8. Evaluate, set value, data breakpoints, set next line

- **Evaluate** (`eval.rs`): a parser of `name[suffix] [(int, …)] {.member [(int, …)]} [,h|,b|,o]`; the name is
  looked up in the frame's procedure's variables, then the main module's (a suffix narrows by type); the result is
  D7's reading of that place. In `repl` context, text that does not parse or names nothing goes to the backend
  unchanged (with `lldb-dap`'s command prefix it is an `lldb` command).
- **Set value** (`setVariable`, `setExpression`): the text parsed as a number of the type (range checked) and
  written with `writeMemory`; a `STRING * n` padded or cut and written in place; a `STRING` by an `evaluate` that
  calls libqb's `qbs_set` on the variable's `qbs` and a new temporary (`qbs_new_txt_len`), both cast to their C
  types, as `lldb` must call a function without debug information. Measured first (task 3); if `lldb` cannot make
  the call, setting a `STRING` is refused with a message, and a row goes to `SOMEDAY.md`.
- **Data breakpoints**: `dataBreakpointInfo` is answered by the proxy for its own `variablesReference`s and for
  evaluate names: `dataId` `<hex address>/<size>`, the form `lldb-dap` gives its own (measured in task 3), size at
  most 8; `setDataBreakpoints` goes to the backend with those ids.
- **Set next line**: `gotoTargets` answered by the proxy (one target per asked line inside the current frame's
  procedure; the main module's range is every line of the main file outside any procedure's lines); `goto` sends
  `` `thread jump --file <path> --line <n> `` as a `repl` `evaluate` and then a `stopped` event of reason `goto`.

### D9. Breakpoints of the proxy's own, kept apart from the client's

`setFunctionBreakpoints` replaces the whole set in DAP. The proxy keeps the client's last list and its own (the
`QBMAIN` of D4, the `error` of D10) and always sends the union; the response to the client lists only the client's
breakpoints, and `breakpoint` events about the proxy's own are dropped. A `stopped` event whose
`hitBreakpointIds` name only the proxy's own is the proxy's to handle.

### D10. The error stop

The filter `qbError` ("BASIC runtime errors") in `initialize`'s `exceptionBreakpointFilters`, default off.
`setExceptionBreakpoints` with it sets the proxy's function breakpoint `error` with condition `(int)$ecx != 0`
(D9) and is answered by the proxy; without it the breakpoint goes. On its hit the proxy reads the code
(`evaluate` `(int)$ecx` in frame 0), keeps it for `exceptionInfo`, and sends `stopped` with reason `exception`,
`description` `Error <n>: <text>`. The top BASIC frame (D5) is the raising line. The texts are `errors.rs`, a table
written from libqb's `error_handle.cpp`; a tier 1 test reads that file in `libqb\` and checks that every `case n:
return "…"` there is in the table with the same text.

### D11. Threads

The program's thread is the one whose stack has the `QBMAIN` frame, found by an internal `threads` and
`stackTrace` at the first stop and kept for the session. `threads` answers it alone, named "Program" (before the
first stop, `lldb-dap`'s answer with every name replaced is passed: nothing can be inspected before a stop). Every
`stopped` event is sent with that thread's id and `allThreadsStopped: true`.

*Amended by "Measured":* `lldb-dap`'s `threads` need not list the program's thread at all, so it cannot be found
from there. It is the thread of the hit of the proxy's breakpoint on `QBMAIN` (D4), set in every session. Before
that hit `threads` passes through unchanged, and `thread` events are dropped throughout. A pause stops the process on
a thread of its own, reported by `lldb-dap` as an `exception`; the proxy reports it as `pause` on the program's
thread.

### D12. The subcommand

`driver\src\main.rs`: `dap(args)` beside `lsp`, reached when the first argument is exactly `dap`; the only option
`--qb64pe-root <dir>`; it calls `build::find_root`, and `qb64rust_dap::run_stdio(root.join("internal/c/c_compiler/
bin/lldb-dap.exe"))` on a thread of `with_stack`. When the clone is not found the server still starts and
answers `initialize` with the error (the spec), so the editor shows it rather than a dead adapter.

### D13. Where each piece lives

| Piece | Where |
|---|---|
| Framing | `dap\src\wire.rs` |
| The loop, `seq` maps, internal calls, passthrough | `dap\src\proxy.rs` (`serve`, `Proxy::call`) |
| The symbol file | `dap\src\symbols.rs` |
| `launch`, stop on entry | `dap\src\launch.rs` |
| BASIC frames, `stackTrace`, threads | `dap\src\stack.rs` |
| Stepping rules | `dap\src\step.rs` |
| `scopes`, `variables`, reading memory | `dap\src\vars.rs`; formatting `dap\src\format.rs` |
| `evaluate`, set value, data breakpoints, set next line | `dap\src\eval.rs` |
| Own breakpoints, the error stop, the error texts | `dap\src\breakpoints.rs`, `dap\src\errors.rs` |
| `qb64rust dap` | `driver\src\main.rs` `dap` |
| Tier 1 tests (scripted backend) | `dap\tests\proxy.rs` (passthrough), `session.rs` (launch, breakpoints, stack, steps, error stop), `values.rs` (variables, evaluate, set value, data breakpoints, set next line), `symbols.rs`; the stand-in `Fake` in `dap\tests\support`; the fixture in `dap\tests\fixtures\` |
| End-to-end tests (real `lldb-dap`, `--ignored`) | `driver\tests\cli.rs` `dap_*`, and `lldb_dap_measured_*` (the clone's `lldb-dap` alone); a small DAP client in `driver\tests\support\dap.rs` |
| Documentation | `crates\README.md` (crate map, `qb64rust dap`, the tests) |

## Risks / Trade-offs

- [The hang is `lldb`'s and not fixable here] → D1's last branch: recorded; `pause` ends a step that does not;
  the end-to-end tests run one at a time with a deadline, as the `lldb` tests do.
- [`lldb-dap` 23 behaves differently from the measurement through `lldb`] (reading memory at a raw address, the
  `dataId` form, `thread jump` through `repl`, the `QBMAIN` breakpoint's first stop) → each is an ignored CLI
  test written before the rewrite that uses it (task 3); a difference changes the rewrite, not the spec.
- [Blocking internal calls make the proxy slow on a large stack or a large array] → the stack is one call;
  arrays are read in pages of at most 1000 elements per level, one `readMemory` per page.
- [A symbol file older than its executable] → impossible by `compiler/debug-info` (every build removes the old
  file); the proxy trusts it.
- [The main file's path in `#line` is relative when built with a relative path] → D5's suffix match; the
  extension (next change) builds with absolute paths.
- [`_FLOAT` displayed through `f64`] → a display rounding only; said in `SOMEDAY.md` with `_BIT` and `_MEM`.

## Measured

From the hang's reproduction (task 1.1, 2026-10-25) and the ignored CLI tests of task 3.1 (`lldb_dap_measured_memory`,
`lldb_dap_measured_stops`: the clone's `lldb-dap` 23.1.2 alone, no proxy). Each line: what was found, then what the
proxy takes from it.

- **The hang of a `next`** (task 1.1): `debug_next_steps_by_line` in a loop beside the slice list at `--jobs 8`
  without caches hung in 4 of 16 runs, each time on the `next` over the `PRINT` of line 7. The step log shows every
  thread resumed (`stop others = 0`): the stepping thread did not run alone, so D1's first branch does not apply. The
  minidumps taken at 40 s show the program's thread a few bytes past the step's resume address, in the program's
  own code, not in libqb (the process frozen on a debug event), and no thread of `lldb` in `WaitForDebugEvent`: its
  debugger thread had taken the event and waited for the process to handle it, while the process side waited for an
  event and took the program for running. A race in `lldb`'s Windows process plugin between a resume and the very
  next stop, which load makes likely; not ours to fix. *Proxy and tests:* `LLDB_USE_LLDB_SERVER=1`, so that `lldb`
  debugs through `lldb-server` (gdb-remote) instead. Under the same load, 222 runs never hung; in 3 of them, each
  at the `next` over the one-line `FOR` loop of line 5, `lldb-server` left within a second (`exited with status = -1
  lost connection`, then `lldb-server exiting...`; no crash in Windows' Application log). A packet timeout of 30 s
  (`plugin.process.gdb-remote.packet-timeout`) did not change it (2 in 180 with it, 1 in 42 without), so it is not
  set. With `lldb-server`'s own log on (`LLDB_DEBUGSERVER_LOG_FILE`, `LLDB_SERVER_LOG_CHANNELS`), and no load, it
  happened in every one of 35 runs, which shows the cause: after a single step (`vCont;s`) the process reports a
  stop while the stepping thread is still marked running (`NativeThreadWindows`: "in state running cannot answer
  stop reason"), so `lldb-server` sends an error (`$E2a`) as the stop reply and `lldb` drops the connection; the
  log's slowness widens the window. A second race of the same kind, in `lldb`'s code, not ours. That failure ends the session at once, where the plugin's froze it, so the proxy takes
  `lldb-server`: `LLDB_ENV`, which `support::lldb` uses too; `QB64RUST_LLDB_LOG` names a file for `lldb`'s step
  log. Every behaviour below was measured again through `lldb-server` and is the same.
- **A function breakpoint on `QBMAIN`** stops in `qbx.cpp` (line 1563, the C++ before the program's first line);
  `next` from there passes through `qbx.cpp`, `maindata.txt`, `mainerr.txt` and `main0.txt` (22 steps) and lands on
  line 1 of the main file. *Proxy:* D4 amended (stepped by D6's rules, a line with no statement passed on entry).
- **Reading and writing**: `evaluate` of `(unsigned long long)__LONG_TOTAL` (context `watch`) gives the address in
  decimal; `readMemory` at `0x…` gives the bytes in base64 (`BwAAAA==` for 7); `writeMemory` changes them.
  *Proxy:* D7 as designed.
- **Data breakpoints**: `dataBreakpointInfo` of `lldb-dap`'s own variable gives `dataId` `<hex address>/<size>`
  without `0x` (`74CB1FF800/8`); a `dataId` of that form made by the proxy stops with reason `data breakpoint` at
  the line that writes the value (column 5 of it), not the line after. *Proxy:* D8 as designed; the spec's
  scenario "Watch a variable" corrected to the writing line.
- **`qbs_set`**: libqb has no debug information, so `qbs_set` alone is "unknown type"; called through function
  pointers of its C type, `((void*(*)(void*,void*))&qbs_set)((void*)__STRING_S, ((void*(*)(const char*,int))
  &qbs_new_txt_len)("xyz", 3))`, it sets the string (`__STRING_S->len` 3). *Proxy:* setting a `STRING` is built.
- **`thread jump`**: `` `thread jump --line 4`` as a `repl` `evaluate` moves the next line (with a warning that the
  line has several locations; the first is taken), and `next` then runs line 4. *Proxy:* D8 as designed.
- **Pause**: a `pause` of `DO: x = x + 1: LOOP` stops the process on a new thread (`DbgUiRemoteBreakin`), reason
  `exception`; the program's thread is then at the loop's line. `lldb-dap`'s `threads` answer listed four threads,
  none of them the program's. *Proxy:* D11 amended.
- **Output**: the program's output comes as `output` events of category `stdout`.

## Open Questions

- Whether `lldb-dap`'s `disassemble` and `modules` views should be hidden: left as they are (a user who opens
  them sees the C++, which is true).
