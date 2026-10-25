# 36. M5, the debugger: the path forward, and the eleventh review (2026-10-24)

Asked by the user, after M4 was declared done: research the best path forward for M5 (`ROADMAP.md`: "Debug
symbol file and a DAP adapter"), and have a panel (software architect, pragmatic engineer, compiler/languages
engineer and others) review what has been done so far and what remains, noting what needs to change. No nits.

The panel is role-played inline (`CLAUDE.md` working rule 6): the pragmatic engineer (PRAG), the software
architect (ARCH), the compiler/languages engineer (COMP), the QB64 engineer (QB64), the Rust engineer (RUST), the
test engineer (TEST), and two extra roles because M5 is a tool for an editor and sits on the runtime: the
developer-tools engineer (DX: debuggers, DAP, VS Code) and the runtime engineer (RT: libqb's threads and error
path, which a debugger stops and reads). Effort is counted in Claude's time (working rule 8).

Evidence read: `STATUS.md`, `ROADMAP.md`, `DECISIONS.md` (the rows on `$DEBUG`, the language server, the
dependency policy), `DIVERGENCES.md` D-017, `SOMEDAY.md`, `study\04` Part F (the old debugger: instrumentation,
protocol, state machine, §8 hazards) and Part G, `study\06` (the DAP mapping), `study\07` sessions 6 and 8,
`crates\README.md`, the spec `language/asserts-and-debug`, the emitter (`codegen-cpp\src\procs.rs`, `coverage.rs`,
`names.rs`, `decl.rs`), the driver's build (`driver\src\build.rs`), the language server (`lsp\src\server.rs`),
the extension's manifest, the copy's `Makefile`, and the clone's toolchain folder. Run (§2): one probe program
built by the new compiler with `-g`, debugged with the clone's `lldb`; one probe of the clone's `lldb-dap`
capabilities.

## 1. Verdict in one paragraph

The plan of 2026-10-03 ("a debug symbol file and a DAP adapter", `study\07` session 8) was written for a
toolchain that no longer exists: QB64pe's Windows toolchain is now LLVM 23 (clang, lld) and **ships `lldb.exe`,
`lldb-dap.exe` and `lldb-server.exe`**, and this compiler already writes a `#line` directive for every C++ line
naming the BASIC file and line. Built with `-g`, a program of the new compiler can be debugged by BASIC line
with lldb today, with no change to the compiler (measured, §2). So M5 is not "write a debugger": it is **a debug
build mode, a symbol file that maps BASIC names to the C names and layouts the emitter chose, and a thin DAP
proxy in front of `lldb-dap` that shows the program in BASIC terms**. That is a fraction of the work of the old
design (an agent inside the program, a wire protocol, a full adapter), and it is better than the old debugger
on nine of its listed hazards (§3.3). The panel recommends this path (§4), with the symbol file kept as the
roadmap says, and finds nothing astray in M0–M4 (§5); it recommends one change of order before M6 (§5.3).

## 2. What exists today (measured 2026-10-24)

1. **The toolchain has a debugger.** `..\QB64pe\internal\c\c_compiler\bin\` holds `lldb.exe`, `lldb-dap.exe`,
   `lldb-server.exe`, `lldb-mi.exe` and `liblldb.dll` (lldb version 23.1.2), with `libpython3.14.dll`: `lldb -b
   -o "script print('python ok')"` prints `python ok`, so lldb's Python scripting works there (data formatters
   and synthetic children are possible). No `gdb`.
2. **The emitter writes line tables already.** Every C++ line of a body is preceded by `#line <n> "<file>"`
   with the BASIC file's path (`codegen-cpp\src\names.rs`, `lines_in`); included files get their own path. This
   exists for the C++ compiler's error messages and for `evnt`'s arguments; with `-g` it becomes DWARF line
   information that names the `.bas` file.
3. **A debug build works with no compiler change.** The probe program below (a `DIM SHARED LONG`, a `TYPE`
   with a fixed string, a static `SINGLE` array, a `STRING`, a `FOR` loop, a `SUB` with a `STATIC`) was built
   with `qb64rust -x --keep-build`, then its `qbx.cpp` was rebuilt through the copy's `Makefile` with `-g`
   added to the program's own flags and `STRIP_SYMBOLS=n` (libqb's objects were not rebuilt, so the objects'
   cache and the clone are untouched; the executable grew from about 2 MB to 3 MB). Then:

   ```
   lldb -b -o "breakpoint set -f dbg_probe.bas -l 11" -o run -o "p *__LONG_TOTAL" -o "p *__SINGLE_I"
        -o "p (char*)__STRING_S->chr" -o "p ((float*)__ARRAY_SINGLE_A[0])[0]" -o "bt 3" -o continue ...
   ```

   gives `Breakpoint 1: where = program.exe\`QBMAIN(void*) + 1084 at dbg_probe.bas:11:1`; the process stops
   on thread 12 (libqb runs the program on its own thread, `QBMAIN`; the window and timer threads keep running)
   with the BASIC source shown around line 11 (`total = total + i`); `*__LONG_TOTAL` is `0`, then `1` after
   `continue` (the loop stops once per pass, as the old debugger's line granularity did); `*__SINGLE_I` is
   `1`, the string's bytes are `"hi"`, the array's first element `1.5`; a breakpoint in the SUB resolves to
   `SUB_SHOW(int*) + 97 at dbg_probe.bas:18:1`. The stack is `QBMAIN` ← `stdcall_thread_wrapper`: the C
   procedure names are the emitter's (`SUB_SHOW`), to be shown as `SUB show` by the symbol file.
4. **`lldb-dap` 23 advertises** (an `initialize` request answered over stdio): conditional, hit-count and log
   breakpoints, function breakpoints, `breakpointLocations`, **data breakpoints with a byte count**
   (hardware watchpoints), `setVariable`, `readMemory` and `writeMemory`, `evaluate` for hovers, completions,
   value formatting (hex), stepping granularity, `exceptionInfo`, modules and disassembly. **Not advertised:**
   `gotoTargets`/`goto` (set next line), `restartFrame`, `stepBack`, `restart`. lldb itself has `thread jump
   --line N`, reachable through `evaluate` in its command mode, so set next line can be built by the proxy.
5. **What the compiler knows and does not write down yet.** `sema` has every variable with its BASIC name,
   type, storage (`Main`, `Static`, `Local`, `Param`, `Result` of a procedure) and shape, and a symbol table
   with each name's definition and references; `codegen-cpp` has the C name of each (`__LONG_TOTAL`,
   `_SUB_SHOW_INTEGER_CALLS`, `_SUB_SHOW_LONG_N`), the member offsets and size of every user type
   (`decl.rs`, `layout`), and the array descriptor's layout (data pointer at word 0, base and size per dimension
   at 4 and 5, …). None of it leaves the compiler; the `.sym` file of a build is `nm`'s list for `DECLARE
   LIBRARY`, not this.
6. **The build has no debug switch.** `-f:StripDebugSymbols=...` is accepted and ignored (`crates\README.md`);
   `OptimizeCppProgram=true` adds `-O2`; the Makefile strips with `-s` unless `STRIP_SYMBOLS=n`; the program's
   own flags go in `program.mk`, which the driver writes per build, so `-g` has a place to go that leaves
   libqb's objects alone (as `-O2` does today, design D1 of `libqb-object-flags`).
7. **The old debugger** (`study\04` Part F): an 880-line BASIC SUB compiled into the program, called before
   every statement of the main file, a TCP client of the IDE with a hand-made protocol (frame length off by
   four on both sides, reversed string comparisons, most commands ignored while running), variable metadata only
   in the IDE's memory, memory read by two dereferences with hard-wired struct layouts, line granularity, no
   pause of a one-line loop, no locals of a caller, nothing in included files or under `$CHECKING:OFF`, no stop
   at a runtime error, a stall of 10 s when a `$DEBUG` build runs with the port variable set and no IDE.
   `DIVERGENCES.md` D-017: this compiler writes none of it; a program built here cannot be debugged from the
   QB64pe IDE.

## 3. The paths

### 3.1 Path A: native debug information, `lldb-dap`, a symbol file and a thin proxy (recommended)

- **Compiler.** A debug build (`--debug`, and `-f:StripDebugSymbols=false` honoured as the same thing) adds
  `-g` to the program's flags in `program.mk`, passes `STRIP_SYMBOLS=n`, and ignores `OptimizeCppProgram` with
  a note (stepping optimised code misleads; the default is already `-O0`). Nothing else in the C++ changes; the
  build cache keys on the flags as it does for `-O2`.
- **Symbol file** (the roadmap's item, kept): written beside the executable by the driver from what `sema` and
  `codegen-cpp` know, versioned, one JSON document: each procedure (BASIC name as declared, kind, C name, file
  and line range of header and `END`), each variable (BASIC name with suffix, type as the user would write it,
  scope, C name, storage: pointer variable or C local or parameter, shape with static bounds, `conventional`),
  each user type (members with name, type, offset, size; the type's size), the fixed layouts the proxy needs to
  read an array (descriptor words) and a string (`qbs`: `chr`, `len`), and the include files. DWARF carries
  the line table, so the file does not.
- **Adapter.** `qb64rust dap`: a DAP server to VS Code that starts `lldb-dap` from the toolchain folder as its
  backend and forwards every message, rewriting the few that must speak BASIC: `launch` (build the program in
  debug mode first, or take a built one; set `QB64PE_NOPROMPT` as the launch config says; the console program's
  terminal), `scopes`/`variables` (the frame's procedure from the symbol file: its locals, parameters and
  statics by BASIC name, read through `evaluate`/`readMemory` of their C names, strings by `chr`/`len`, arrays
  as children by index within the bounds the descriptor holds at that moment, members by offset; a "Main
  module" scope for `Main` storage; the runtime's own globals hidden), `evaluate` (a BASIC name, element or
  member, translated to the C expression; display formats DEC/HEX/BIN/OCT by a suffix as the old watch list had
  them), `setVariable` (the reverse: numbers written by type, a variable string through `qbs_set` is a function
  call in the debuggee, which lldb's `evaluate` can make, as the old debugger did with `set_qbs_size`),
  `setDataBreakpoints` (the address and byte size of a place, to the backend as they are), `stackTrace`
  (C names to BASIC names; frames of libqb and the thread wrapper folded away), `goto`/`gotoTargets` (lldb's
  `thread jump`, within the current procedure, with the same hazard as the old one), and the stop at a runtime
  error (a function breakpoint on libqb's `error(int32)` with a condition on the error number; the proxy reports
  `ERR` and the line from the frame below it; the old debugger only notified, §2 item 7). Everything else
  (breakpoints by file and line, conditions, hit counts, log points, step in/over/out, pause of a running
  program, run to cursor, the call stack, hover, completions, the Debug Console, attach) is `lldb-dap`'s as it
  is.
- **Extension.** A `debuggers` contribution of type `qb64rust` (`program`, `args`, `stopOnEntry`, `console`,
  `noPrompt`), a configuration provider that builds in debug mode before launch, a "Debug" command beside Build
  and Run. The adapter runs as `qb64rust dap`, found as the language server is.
- **What it gives that the old one did not** (QB64, DX): a one-line loop or a tight loop can be paused; the
  locals of every frame, not the top one only; variables of included files, and breakpoints in them; code under
  `$CHECKING:OFF`; a stop at the error with the variables readable; conditional and hit-count breakpoints and
  log points; no cost at run time (no hook call per statement, no socket poll), so the debugged program runs at
  full speed; no reserved names (`vwatch`), no port, no stall; attach to a running debug build.
- **What it gives up or changes** (QB64): hardware watchpoints are limited by the CPU (x86-64: four of up to
  eight bytes; a watch of a whole string or a large element falls back to a conditional breakpoint per line,
  slower; to be said in the design and `DIVERGENCES.md` need not grow: D-017 already covers the debugger's
  absence); "skip lines" stays out (`SOMEDAY.md`); set next line is lldb's jump (same hazard as the old one's
  raw `goto`); the window keeps painting while the program thread is stopped (an improvement in practice; the old
  one blocked the main thread, so the window went stale) but a `_DISPLAY`-paced program will show its last frame.
- **Risks** (COMP, RT): lldb on Windows reads DWARF from a mingw-targeted clang build — the probe shows it
  resolves lines, variables and the stack; what has not been tried is `next` over a `do{…}while(r)` statement
  wrapper (does one `next` stop once per BASIC line, or at the `evnt` check too?), stepping across a `RESUME`
  retry, and whether `#line` directives repeated per C++ line give clean `is_stmt` rows. Those are the first
  tasks of the change, measured before the adapter is written; if `next` stops twice per line the proxy can
  step until the line changes (lldb-dap's `stepInTargets` are not needed). The error stop depends on libqb's
  `error` keeping its name and a frame with line information below it: the program's `evnt` call is in the
  program's own object, so it does.
- **Effort** (Claude's time): the debug build and the symbol file about half a day with tests; the proxy's
  passthrough and `launch` a few hours; the variables, evaluate, stack and error stop a day or two with the
  tests of §4.3; the extension's contribution and an integration test half a day. The cost that stays: one
  crate of moderate size (the proxy), a JSON schema to keep, and a dependence on the toolchain's `lldb-dap`
  (which comes with QB64pe, as `c++.exe` does).

### 3.2 Path B: the plan as written, with the project's own agent

A per-statement hook (as the coverage counter is placed today), a C++ agent linked into the program as
`screen_state.cpp` is, a versioned protocol over a pipe or TCP to a Rust adapter, memory read by the agent from
descriptors the adapter sends, the symbol file for the adapter. Full control and no dependence on lldb; works
with any toolchain. Against it: it rebuilds what lldb does (stop a thread, read memory, walk a stack, set a
hardware watchpoint, evaluate an expression in the debuggee, attach), each a correctness risk of its own
(`study\04` §8 items 1–3 are exactly where the old one went wrong); the hook costs every statement of every
debug build; a running program can be paused only at a hook; a crash inside the agent takes the program down.
Perhaps five to ten times the code of Path A, all of it to maintain, for less (RUST, PRAG).

### 3.3 Path C: `lldb-dap` alone, no proxy, a Python formatter script

Ship a `launch.json` snippet for the `lldb-dap` extension with an `initCommands` script that pretty-prints
`qbs*` and arrays. Nearly free, and a useful first step inside Path A (the probe of §2 is one `lldb` command
away from it), but the user sees `__LONG_TOTAL` and `SUB_SHOW`, every runtime global, and no BASIC expressions;
it needs the `lldb-dap` VS Code extension installed. Not the product; the first task of Path A, to measure the
stepping questions (§3.1 risks) with.

### 3.4 How the old hazards fare (`study\04` §8)

| Hazard | A | B |
|---|---|---|
| 1 debugger inside the program, sharing heap, errors, timers, keys | gone | partly (a C++ agent shares less than a BASIC one) |
| 2 hard-wired names and layouts in three places | one place, the symbol file, written by the code that chooses them | same, if disciplined |
| 3 types by name strings | typed symbol file | same |
| 4 metadata only in the IDE | symbol file; attach possible | same |
| 5 per-line cost, no throttle | none | present |
| 6 line granularity; one-line loops unpausable | stop anywhere | present |
| 7 nothing in includes or `$CHECKING:OFF` | covered | buildable |
| 8 locals of the top frame only | every frame | buildable at a cost |
| 14 error stop is notification only | a real stop | buildable |
| 15–19 protocol defects | no protocol of ours but DAP | a new protocol to get right |

## 4. Recommendation and the shape of the first change

### 4.1 Recommendation

Take Path A. Keep the roadmap's two nouns, read anew: the **debug symbol file** is the bridge between the
compiler's knowledge and any debugger; the **DAP adapter** is a proxy over the toolchain's `lldb-dap`, not a
debugger. Measure the three stepping questions first (one task, Path C's script), design the symbol file and the
proxy after them.

Proposed `ROADMAP.md` text for M5 (to replace the one line, once the user decides; `ROADMAP.md` is otherwise
edited at the merge cadence, working rule 11):

```
## M5. Debugger

Done when: a program built in debug mode by the new compiler is debugged in VS Code by BASIC line, with
breakpoints, stepping, pause, the call stack, the variables of every frame by BASIC name (scalars, strings,
arrays, user types), send value, data breakpoints and a stop at a runtime error; checked by tests that drive
the adapter (`study\36`).

- [ ] Debug build (`--debug`, `-f:StripDebugSymbols`): `-g`, unstripped, never optimised; the stepping questions
      measured with the toolchain's `lldb`
- [ ] The debug symbol file beside the executable: procedures, variables, user types, the descriptor layouts
- [ ] `qb64rust dap`: the DAP proxy over the toolchain's `lldb-dap` (variables, evaluate, set value, stack,
      data breakpoints, set next line, the error stop); its protocol tests
- [ ] The extension's `debuggers` contribution and launch configuration; a debug build before launch
```

### 4.2 Design points settled by the panel

- **ARCH:** the proxy is its own crate, `crates\dap`, started by `qb64rust dap` as `lsp` is by `qb64rust lsp`;
  it depends on `base` and `serde_json` and reads the symbol file; it does not depend on `sema` or
  `codegen-cpp` (the layering of `crates\README.md`). The symbol file is produced where the names are chosen:
  `codegen-cpp` returns a `DebugInfo` beside the fragments, the driver writes it. If a BASIC expression
  evaluator is wanted later (parse the watch expression with `syntax`, type it with `sema` against the
  program, emit the C expression with the emitter's own `place.rs`/`value.rs`, so a watch has exactly the
  compiled program's semantics), it comes as a function the driver hands the proxy, as `Check` is handed to the
  language server (design D2 of `extension-diagnostics`). Not in the first change.
- **RUST:** DAP's framing is LSP's (`Content-Length`, a JSON body) with a different envelope (`seq`, `type`,
  `command`/`event`); the proxy forwards `serde_json::Value` and types only the messages it rewrites. A protocol
  crate is allowed by the dependency policy (2026-10-07); the `dap` crate on crates.io is small and typed, worth
  taking for the request and response structs if it matches DAP 1.6x; otherwise a hundred lines of structs. No
  crate takes source as `str` (none is involved).
- **COMP:** the symbol file is a product of the compiler and gets a spec (`compiler/debug-info`) with a version
  field and snapshot tests over `tests\frontend` programs in tier 1 (deterministic, no toolchain). A
  procedure's parameters are C pointers to the caller's places (`int32*_SUB_SHOW_LONG_N`): a by-value argument
  gets a temporary; the file says for each parameter whether its C name is a pointer, so the proxy reads it in
  one more step. `FUNCTION` results, `STATIC`s (C globals named by procedure), `SHARED` views of main-module
  variables, `_BIT` elements (read with the runtime's bit helpers, as the old debugger did, or shown packed with
  a note in the first change), `_MEM` blocks (shown as their struct) and `_OFFSET`s each need a line in the
  design; `_FLOAT` is 16 bytes in arrays and `long double` in C.
- **RT:** libqb starts the program on its own thread (`QBMAIN`); stopping it leaves the window thread painting
  and the timer thread counting, and `ON TIMER` handlers run at the program's `evnt` so they do not fire while
  stopped; `_DISPLAY`-paced programs show their last frame; `SLEEP`/`INPUT` are waits in the program thread
  and can be paused like anything else. The error stop: libqb's `error(int32)` is called by the program's
  own code and by libqb's; a function breakpoint with a condition `new_error == 0 && code != 0` stops at the
  raise, before `evnt` services it, with the raising statement one frame below. Errors 300–315 (`_ASSERT`,
  `_MEM`) go through the same function. `QB64PE_NOPROMPT` decides what happens after a continue.
- **DX:** the launch configuration builds first through the extension's own build path with the debug flag
  (the run queue and the output channel already exist); the adapter's path is resolved as the server's;
  `console` defaults to the integrated terminal for a `$CONSOLE` program, else `internalConsole`; the
  "Variables" view has three scopes: Locals (parameters, locals and statics of the frame's procedure), Main
  module, and, when the frame is libqb's or the thread wrapper's, nothing but a note. Hover uses `evaluate` with
  the BASIC name under the cursor. The old IDE's HEX/BIN/OCT display formats become `evaluate` suffixes (`,h`,
  `,b`, `,o`) and the DAP `format.hex` flag.
- **PRAG:** no `$DEBUG` dependence. A program is debugged because it was built in debug mode, whatever
  metacommands it has; `$DEBUG` keeps meaning what D-017 says. That makes the old IDE's "$DEBUG features only
  work from the IDE" warning literally true for this compiler too, which is fine.

### 4.3 Tests

- Tier 1: the symbol file's snapshots (`tests\frontend`, a mode line `debug-info`); the proxy's rewrites with a
  stand-in backend (a fake `lldb-dap` of recorded responses, in-process over a pair of pipes, as `lsp\tests\
  server.rs` does with its connection), one scenario per DAP request rewritten; the launch configuration and
  the adapter's discovery in the extension's unit tests.
- Tier 1 by hand (`--ignored`, needs the clone): CLI tests that build a program in debug mode and drive
  `lldb -b` with a script (breakpoint by `.bas` line, `next` once per line, a variable of each kind read, the
  error stop), asserting on its output; and the proxy end to end against the real `lldb-dap` with a built
  program (one scenario: launch, stop on entry, breakpoint, variables of each scope, set value, continue to the
  error stop, stack, exit).
- The extension's integration tests: `vscode.debug.startDebugging` on a fixture program, a breakpoint hit, a
  variable read, as the compiler tests there run when the binary is given.
- The screen programs are not debugged by a test; the keys file and the debugger both drive the program and
  would fight.

## 5. The eleventh review: what has been done, and what remains

### 5.1 What has been done (M0–M4), the panel's reading

22 days (2026-10-02 to 2026-10-24), 183 commits, 65 archived OpenSpec changes, about 38,000 lines of Rust in
nine crates (the count of `crates\*\src\*.rs`; `study\35` counted tests and generated tables too). Tier 1 439
tests; the full corpus 512 of 517 compared programs pass and the five left are known failures of the recorded
environment; upstream 402 of 404, the two left out of scope by decision; 3,890 verification programs pass; the
call-site check 2,649 of 2,649; Civil War Strategy plays whole scripted games held to line coverage; `qb64pe.bas`
compiles clean and builds. The extension is released as a `.vsix` with the compiler inside.

- **ARCH:** the layering has held for 22 days under pressure (one crate per stage, dependencies downward, `lsp`
  never on `sema`, the IR ABI-neutral with the C++ emitter holding every libqb name); the three registers
  (`DECISIONS.md`, `DIVERGENCES.md`, `SOMEDAY.md`) and the measured-first rule gave every behaviour a
  provenance. The debugger drops into this without a new seam: the symbol file is one more product of
  `codegen-cpp` written by the driver, the proxy one more server in the binary. Nothing to change.
- **COMP:** the decision that pays off most now is one taken for another reason: `#line` directives on every
  emitted line (for C++ diagnostics and `evnt`) are the whole line table of the debugger. The flat IR body with
  labels makes set next line well defined (a jump within one C function, as the old one). The one soft spot
  for debugging is that a BASIC statement is a `do{…}while(r)` with an `evnt` check at its end, so `next` may
  need the proxy's help; measured first.
- **QB64:** parity is real where it was measured: the corpus, the upstream tests and 3,890 verification programs
  are the old compiler's own output. What a user sees is what matters next, and today a user of this compiler
  still installs QB64pe: for the toolchain, and for the formatter (`-y`), which the extension still runs
  through `qb64pe.exe`. The debugger adds a third reason to have QB64pe installed (its `lldb-dap`), which is
  fine as long as the toolchain is QB64pe's; it would be a line in a bundled-toolchain item (`SOMEDAY.md`), not
  a change now.
- **RUST:** nothing to change. The lints (no wildcard enum arms in `sema`/`ir`/`codegen-cpp`, no `str` from
  bytes, checked casts) are the reason M4's 14 changes added forms without regressions; a `dap` crate takes the
  same rules. The dependency policy already allows a DAP crate.
- **TEST:** the closing set is mechanical now (`close.py --plan`/`--check`), the full set runs at merges, every
  run is on record. Two things stand out, neither a nit: (1) `g12_game.files`'s `SHELL "copy …"` hang under
  `--jobs 8` late in long runs has been "the environment's" for three days with cause unknown ("Not yet known:
  what the `cmd.exe` waits on", `STATUS.md`); it costs a 30 s timeout, a retry and a `PermissionError` per full
  run and it shadows a class of failure (a program that hangs). Worth one hour to find (attach the toolchain's
  `lldb` to the hung `cmd.exe`'s parent, or run the script under Process Monitor once): the same tool M5 brings.
  (2) `screen/c15_resize` fails because the desktop now resizes new windows; its premise is gone; the recording
  should move to a `varies` or be rerecorded with a `_RESIZE` check that tolerates one resize, at the next
  merge. Both for the merge of `m4-sub-gl`'s "paperwork", not for M5.
- **PRAG:** the thing nobody can do yet is debug a program, and the second thing is format one without QB64pe.
  M5 as recommended is small; the formatter is the other half of "someone would choose this over QB64pe"
  (`study\35` §3 item 1), and `ROADMAP.md` lists it under "Not tied to a milestone yet". See §5.3.
- **DX:** the language server is thin by decision (parser only live; `sema` on a check), and `sema`'s symbol
  table already has definitions and references of every name, so hover (a name's type and declaration), find
  references and rename are a day's work each, with `Check`'s pattern (the driver hands the function). Not M5,
  but the same order of cost as M5 and the same user.
- **RT:** M6 ("the ABI owned by the new compiler; error model, string heap and threading redesigned") is
  the only milestone whose size is unmeasured and whose benefit to a user is indirect. With the debugger native,
  nothing in M5 constrains M6: a second backend keeps `#line` and gets DWARF for free; the symbol file's
  "fixed layouts" section is per backend by design (its version field says which).

### 5.2 What needs to change

1. **`ROADMAP.md` M5** (one line today) becomes the four items of §4.1, with its exit criterion. The user's
   decision; then a `DECISIONS.md` row naming this study.
2. **`study\06`, "Debug adapter"**: its closing paragraph ("An alternative is native debug info plus an existing
   native debugger, but BASIC-level variable display then needs a custom layer anyway") is now the plan, and the
   custom layer is the proxy; a pointer to this study is added at its top (done, 2026-10-24). The old mapping
   table stays right: every row is a DAP request `lldb-dap` serves, but set next line (the proxy's) and skip
   lines (out).
3. **`DIVERGENCES.md` D-017** stays as it is; no new row: the new debugger is not a difference from the old
   compiler's program outside its IDE. The design of the first change says in a paragraph where the new debugger
   differs from the old IDE's (watchpoint limits, no skip lines) as `SOMEDAY.md` "QB64pe behaviours to review"
   rows if any are questionable; none is a divergence of the program.
4. **`-f:StripDebugSymbols`** stops being ignored: it is the debug switch QB64pe users know (`crates\README.md`,
   spec `compiler/cli`), equal to `--debug`.
5. **`STATUS.md`**: "Next" item 2 names this study and the decision it waits for (done, 2026-10-24).

### 5.3 What remains, and one change of order

M5 (as above), then M6. The panel recommends inserting, between them or inside M5 as a second half, **"M5b:
the editor without QB64pe"**: the formatter that matches `-y` (its corpus is recorded: `baselines\` format
tests 24 of 24, the old compiler's `-y` on every corpus program can be recorded by the runner), and the
language-server features `sema`'s symbol table already pays for (hover, references, rename, maybe completion of
names and built-ins from the table). Reason, by rule 8: each is a day or two of Claude's time, no cost that
stays beyond the code, and together with the debugger they are what a user compares against the QB64pe IDE;
M6 is the largest piece of work left and the one with the least visible result, so it should come when the
product in front of it is whole. This is scope the user has not asked for, so it is a recommendation to decide,
not a plan. If declined, M6 follows M5 as the roadmap says.

## 6. What the user decides

1. **Path A for M5** (§4.1), and its roadmap text; or Path B, or Path C only.
2. **The order after M5** (§5.3): M5b (formatter and the language-server features) before M6, or M6 next.
3. Whether the two test-environment items of §5.1 TEST (the `SHELL` hang's cause; `c15_resize`) are taken up
   at the merge of `m4-sub-gl`, or left as recorded.

The first change of M5 can be proposed (`openspec propose`) as soon as item 1 is decided; its first task is the
stepping measurement of §3.1 "Risks", with Path C's script, before the design of the symbol file and the proxy
is final.
