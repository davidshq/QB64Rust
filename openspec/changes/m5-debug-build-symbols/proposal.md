# Proposal

## Why

M5, the debugger, takes Path A of `study\36` (user, `DECISIONS.md` 2026-10-24): the toolchain's `lldb-dap`
debugs the program, the compiler supplies the line table and a symbol file, a thin proxy speaks BASIC to the
editor. This is M5's first change: the two things the compiler itself must produce, and the measurement the
proxy's design depends on. Nothing of it needs the proxy, so it can be built and checked with `lldb` alone.

What exists (`study\36` §2, measured): the emitter writes a `#line` directive for every C++ line naming the
BASIC file and line; built with `-g`, a program of this compiler stops at a BASIC line under the clone's
`lldb` and its variables are readable through their C names. What is missing: a way to ask for such a build
(`-f:StripDebugSymbols` is accepted and ignored; nothing adds `-g`), and any record of how the BASIC names map
to the C names, storage and layouts the emitter chose. Open until measured (`study\36` §3.1 "Risks"): how `next`
steps through the `do{…}while(r)` statement wrapper, across a `RESUME` retry, and whether a `#line` directive
repeated per C++ line gives one stop per BASIC line.

## What Changes

- **A debug build.** `qb64rust --debug` (a compile line like any other) builds the program's own objects with
  `-g`, leaves the executable unstripped and never optimises it (`OptimizeCppProgram=true` is overridden with a
  note on the console). libqb's objects are untouched, as with `-O2` today. The build cache tells a debug build
  from a plain one.
- **`-f:StripDebugSymbols=false` does what it does in `qb64pe`**: the executable is not stripped
  (`STRIP_SYMBOLS=n`), nothing more; `true` (the default) strips, as today. It stops being ignored. This is
  narrower than the row of `DECISIONS.md` 2026-10-24 said ("honoured as the same" as `--debug`); the row is
  amended in this change: the upstream suite passes `false` on every program, and turning those 402 builds into
  `-g` builds would slow the suite and test a build nobody ships.
- **The debug symbol file.** A debug build writes `<exe>.debug.json` beside the executable: the program's
  files, procedures (BASIC name, kind, C name, header and `END` lines), variables (BASIC name, type as written,
  storage and procedure, shape and static bounds, the C name and how to reach the value through it), user types
  (members with type, offset, size, bounds), and the fixed layouts the emitter relies on (the array descriptor's
  slots, the `qbs` string's `chr` and `len`, the pointer size, `_FLOAT`'s size in an array). It is versioned.
  Without `--debug` nothing is written and the C++ is unchanged.
- **`--dump debug-info`** prints the file for a program without building it (as `--dump cpp` prints the C++),
  and a `tests\frontend` mode line `debug-info` snapshots it, so the file is checked in tier 1 without a toolchain.
- **The stepping measurement.** CLI tests (`--ignored`, needing the clone) build programs with `--debug` and drive
  the clone's `lldb` in batch mode: a breakpoint by `.bas` line, `next` over plain statements, over a one-line
  loop, over a `CALL`, through a `RESUME` retry, into a SUB, a variable of each storage read, a stop at libqb's
  `error()`. What they show is written into the design (its "Measured" section) and is the input of the next
  change, the DAP proxy.

Out of this change: `qb64rust dap` (the proxy), the extension's `debuggers` contribution, any reading of
`_BIT` elements or `_MEM` blocks by a debugger (the file describes them; the proxy decides how to show them).

## Capabilities

### New Capabilities

- `compiler/debug-info`: the debug symbol file a debug build writes beside the executable: what it holds, its
  version, and what a reader may rely on.

### Modified Capabilities

- `compiler/cli`: `--debug` on a compile line; `-f:StripDebugSymbols` no longer ignored; `--dump debug-info`.
- `compiler/pipeline`: the C++ build's flags for the program's own objects gain `-g` for a debug build, and the
  executable is left unstripped for a debug build or `StripDebugSymbols=false`; the build cache keys on both.
- `testing/compiler-tests`: the mode line `debug-info`.

## Impact

- `crates\codegen-cpp`: a `DebugInfo` produced beside the fragments from the names and layouts it already
  computes (`names.rs`, `decl.rs`); no change to the C++ written but one found by the measurement (task 3.3): a
  procedure of an included file has its header and `END` lines under its own file's name.
- `crates\driver`: the option and setting, `program.mk` and the make variables, the build cache's key, writing
  the file beside the executable, the dump and the test mode; CLI tests with `lldb`.
- `crates\README.md`: the build and run lines for `--debug` and the dump; `tests\frontend\README.md`'s mode list.
- The upstream and verification runs pass `-f:StripDebugSymbols=false` (the runner's compile line for the
  upstream suite): their executables become unstripped (larger on disk, same behaviour), and the build cache's
  key changes, so the first tier 2 run after this change rebuilds every program once.
- `DECISIONS.md`: the row of 2026-10-24 amended as above. `SOMEDAY.md`: nothing. `DIVERGENCES.md`: nothing (the
  program's behaviour does not change).
