# Design

## Context

For the motivation see `proposal.md` ("Why") and `study\36` §1–§4; for the behaviour, the delta specs. How the
parts stand in this compiler (2026-10-24, branch `m5` at the tree of `main`'s merge of `m4-sub-gl`):

- **Line information already exists.** `codegen-cpp\src\names.rs` `lines_in` writes `#line <n> "<path>"` before
  every C++ line of a body, the main file's path as given to the compiler (`source_name`) and an included file's
  path as `Frontend.map` names it. With `-g` these are the DWARF line table (measured, `study\36` §2 item 3).
- **The program's own flags have a place.** `driver\src\build.rs` `program_makefile` writes `program.mk` with
  `private CXXFLAGS += [-O2 ]-fwrapv -I…` for `$(QB_QBX_OBJ)` and `embedded.o` only (design D1 of
  `libqb-object-flags`); `build_with_make` passes make variables (`EXE=`, `QB_QBX_SRC=`, `GENERATE_LICENSE=y`).
  The copy's `Makefile` strips with `-s` on the link line unless `STRIP_SYMBOLS=n` (line 270), as `qb64pe.bas`
  13767 passes it for `StripDebugSymbols=false`.
- **The build cache's key** (`build.rs`, the hasher around line 480) takes `settings.optimize`, `settings.license`,
  the extra link flags and the files they name, the libraries and headers; the fragments and the copy's files.
- **Settings** come from `driver\src\main.rs` `settings()` (`-f:` parsing; `StripDebugSymbols` is an empty arm)
  into `build::Settings` (`optimize`, `extra_link`, `license`, `libraries`, `headers`, `flags`).
- **The names and layouts** are the emitter's: `names.rs` `var_name` (`_<SCOPE>_[ARRAY_]<TYPE>_<NAME>`;
  `__LONG_TOTAL` for a main-module variable, `_SUB_SHOW_INTEGER_CALLS` in a procedure), `proc_name`
  (`SUB_`/`FUNC_` + `c_ident`), `c_type`; `decl.rs` `layout` (member offsets and the type's size, packed, a
  `STRING` member a `qbs*`, a `STRING * n` member `n` bytes), `dim_slot`, `lock_slot`, the descriptor flags
  (`[2]`); `procs.rs` `procedure` declares a procedure's result and locals as C locals (pointer variables, each
  allocated per call), statics and main-module variables as C globals (`global.txt`, allocated in `maindata.txt`),
  parameters as pointer parameters (`regsf.txt`: `void SUB_SHOW(int32*_SUB_SHOW_LONG_N)`).
- **`sema`** has `Var` (`name`, `ty`, `storage`, `shape`, `conventional`), `Proc` (`name`, `kind`, `params`,
  `result`, `line`, `end_line`, `external`), `UserType`/`Member` (`bounds`, `dynamic`), `Program::type_name`
  (the QB spelling of a type), and the symbol table.
- **Dumps and test modes.** `driver\src\main.rs` dispatches `--dump`; `driver\tests\frontend.rs` runs
  `tests\frontend` by mode line and snapshots `cpp`/`coverage` through `dump_cpp`. The CLI tests of
  `driver\tests\cli.rs` build programs through `support::run` with deadlines; `coverage_build_counts_statements`
  is the model of a test that builds and reads a file beside the executable.
- **The toolchain's `lldb`**: `..\QB64pe\internal\c\c_compiler\bin\lldb.exe` (23.1.2), driven in batch mode
  with `-b -o <command>…`; it exits after the commands, killing the process (`study\36` §2 item 3).

## Goals / Non-Goals

**Goals:**

- A build a debugger can use, asked for in one word, that is the same program otherwise.
- A symbol file that lets a reader show the program in BASIC terms with the executable's debug information and
  memory alone; checked in tier 1 from the compiler's own knowledge.
- The stepping facts the proxy's design needs, measured and recorded here.

**Non-Goals:**

- The proxy, the extension, any reading of values by this change (the CLI tests read values through `lldb`'s C
  expressions, which is what the proxy will do; they prove the file's names, not a viewer).
- Debug information for libqb's objects: they stay built without `-g`, so a step into a runtime call has no line
  information and `lldb` steps over it. A stack frame inside libqb shows its C function name; the proxy folds them.
- `-g` for the old compiler's builds, or any change to the copy's `Makefile`.

## Decisions

### D1. `--debug` is the debug build; `StripDebugSymbols` means only what it means in `qb64pe`

`--debug` (`Options.debug` in `main.rs`, `Settings.debug` in `build.rs`) puts `-g` into `program.mk`
(`program_makefile` takes `debug` beside `optimize`; `-O2` is dropped when both are set, with one line on the
console unless `-q`, from `build()`), passes `STRIP_SYMBOLS=n` to make, and has the driver write the symbol file
(D3). `-f:StripDebugSymbols=false` sets `Settings.keep_symbols` (false by default, so `Settings` keeps its derived `Default`), which passes `STRIP_SYMBOLS=n` and nothing else;
the arm in `settings()` parses it like `OptimizeCppProgram`. Both go into the build cache's key (two bytes beside
`optimize`), so the first tier 2 run after this change rebuilds once.

*Alternative:* `StripDebugSymbols=false` as the debug build (the row of `DECISIONS.md` 2026-10-24). Rejected
here: the runner passes it for every upstream and verification program, which would make those suites build with
`-g` (slower compiles, bigger executables, and the suites would stop testing the build a user ships); the row is
amended in task 1.1. *Alternative:* `-g` as an `-f:` setting of our own. Rejected: `--debug` sits with
`--coverage`, a build mode of this compiler, not a `qb64pe` setting.

### D2. `DebugInfo` is produced by `codegen-cpp` beside the fragments

A new module `codegen-cpp\src\debug.rs`: `pub struct DebugInfo` (plain data: `files`, `procedures`, `variables`,
`types`, `layouts`) and `pub fn to_json(&self) -> String` (hand-written, as `coverage.rs` writes its record; no
`serde` in `codegen-cpp`, which has no dependencies today). `emit_with` gains a return of `(Fragments, DebugInfo)`
through a new `emit_debug` that calls the same `Emitter` and then `debug::collect(&emitter)`: the variables through
`var_name` and a new `reach(v: &Var) -> Reach` (pointer, descriptor, string, bytes, param-pointer, from `storage`,
`shape` and `ty`), the procedures through `proc_name` with `QBMAIN` for the main module, the types through
`layout` and `member_size`, the layouts as constants from `dim_slot`, `lock_slot`, the flags slot and `qbs.h`
(`chr` at 0, `len` at 8 on 64-bit; stated by `size_of`-style constants in `debug.rs` with a comment naming
`qbs.h`), `_FLOAT` 32 (as `decl.rs`'s `ty_size` sizes an element; first written here as 16). Temporaries
(`Storage::Temp`) and external procedures, with their parameters and results, are skipped. A procedure's file was
in neither `sema`'s nor the IR's `Proc` (only its lines): both gain `file`, set where the lines are set, so that a
FUNCTION of an included file names that file.
Written by the emitter so that a change of a name or a layout changes the file in the same crate, and the `cpp`
and `debug-info` snapshots change together.

*Alternative:* the driver assembles the file from `sema` and asks the emitter for names. Rejected: the layouts
and the reach of a value are the emitter's decisions and would be repeated.

### D3. The file is written by the driver beside the executable; the dump and the test mode print the same text

`driver\src\lib.rs`: `emit_ir_with` keeps its signature and a new `emit_ir_debug` returns the pair; `main.rs`
calls it when `o.debug` and writes `debug_path(exe)` (`build.rs`, beside `license_path`: `<exe>.debug.json`)
after the executable is in place. Every build removes an earlier `debug_path(exe)` wherever it removes the
earlier executable (`remove_old` in `main.rs`), a failed or a plain build included (amended at the review of
2026-10-25: the first design left it, as with the licence file, but a reader trusts the file's names and layouts,
and an older build's would show wrong values with no error; the licence file follows `qb64pe`, this file has no
such model).
`--dump debug-info` prints `to_json` with the paths as given (`dump_debug_info(fe, resolved)` beside `dump_cpp`);
the frontend test mode `debug-info` calls the same function, so its snapshot is the file's text with the test's
relative paths. The hash of each file is the FNV-1a 64 of a coverage record, which the driver computes
(`fnv1a64` in `driver\src\lib.rs`, as for `coverage_files`) and hands to `emit_debug` with each file's path, so
nothing moves.

### D4. The stepping measurement is a CLI test, and its result is written here

`driver\tests\cli.rs` gets `debug_build_*` tests (`#[ignore]`, as the other builds): each builds a program with
`--debug` into a scratch folder, runs `lldb.exe -b` from the clone's toolchain (`support::lldb()` beside
`unattended`) with `QB64PE_NOPROMPT=y` and a deadline, and asserts on lines of its output. The programs and what
each measures:

| Test | Program | Measures |
|---|---|---|
| `debug_stops_at_a_line` | the probe of `study\36` §2 | a breakpoint by `.bas` line resolves and stops; `p *__LONG_TOTAL` reads 0 then 1; the frame is `QBMAIN` at `dbg.bas:11` |
| `debug_next_steps_by_line` | five plain statements, one line with two statements, a one-line `FOR` loop | how many `next` from line 1 reach line 5; whether the two-statement line is one stop; whether the one-line loop is one stop per pass or one stop |
| `debug_next_over_call_and_into_sub` | `CALL show(x)` with a SUB | `next` steps over the call to the next line; `step` enters at the SUB's first statement; `finish` returns to the caller's line |
| `debug_resume_retry` | `ON ERROR GOTO h`, a statement that raises, `RESUME` | the stops `next` makes around the retried statement (the `do{…}while(r)` wrapper) |
| `debug_reads_each_storage` | one variable per storage, a static array, a user type, a string | the C expressions of the file's `reach` read each value (`p` of the pointer, the descriptor's data slot, `chr`/`len`, the bytes at an offset) |
| `debug_stops_at_error` | a program that raises error 5 | a function breakpoint `error` with a condition on the code (its register, `(int)$ecx == 5`: libqb has no debug information) stops, with a frame of the program's line below it |

What they show goes into the section "Measured" below in task 3.2, as facts of this compiler's build (not of the
old compiler: `verification\` and its README are not involved). If `next` stops twice per line (the `evnt` check
after the statement), the fact is recorded and the proxy's design steps until the line changes; the C++ is not
changed for it in this change.

*Alternative:* `lldb-dap` driven by hand-rolled DAP messages. Rejected for this change: `lldb -b` answers the
same questions with less code, and the proxy's tests will drive `lldb-dap`.

### D5. Where each piece lives

| Piece | Where |
|---|---|
| `--debug`, `-f:StripDebugSymbols` parsing | `driver\src\main.rs` `parse_args`, `settings` |
| `Settings.debug`, `Settings.keep_symbols`, `program.mk`, `STRIP_SYMBOLS=n`, the cache key, the console line | `driver\src\build.rs` `Settings`, `program_makefile`, `build_with_make`, the hasher, `build` |
| `debug_path` | `driver\src\build.rs` beside `license_path` |
| `DebugInfo`, `to_json`, `reach`, `collect` | `codegen-cpp\src\debug.rs`; `emit_debug` in `lib.rs` |
| Writing the file, `--dump debug-info`, `dump_debug_info` | `driver\src\main.rs`, `driver\src\lib.rs` |
| Test mode `debug-info` | `driver\tests\frontend.rs` `MODES`, `run` |
| The CLI tests and `support::lldb` | `driver\tests\cli.rs`, `driver\tests\support\mod.rs` |
| Documentation | `crates\README.md` ("Building and running": `--debug`, the dump; "Tests": the `lldb` tests), `tests\frontend\README.md` (the mode) |

## Risks / Trade-offs

- [`lldb` steps twice per BASIC line, or stops at the `evnt` check] → measured by D4; recorded; the proxy steps
  until the line changes. No C++ change here.
- [The `#line` path of the main file is as given on the command line (relative in the runner's runs)] →
  `lldb` resolves a relative path against its working folder; the CLI tests give absolute paths, and the proxy
  will pass the file's absolute path on `launch`. Recorded in the "Measured" section if it matters.
- [`qbs` offsets stated by hand drift from `qbs.h`] → the layouts section's constants have a tier 1 test that
  reads `libqb\internal\c\libqb\include\qbs.h` and checks the first two members' names and types (`uint8_t *chr`,
  `int32_t len`); the copy is byte for byte at the reference commit, so a change there is a change to see.
- [The upstream and verification suites' executables become unstripped (D1)] → disk only (about +1 MB each
  while they exist; the runner deletes them); the build cache rebuilds once. Said in `tests\upstream\README.md`
  only if the runner's line is documented there (checked in task 1.3).
- [`-g` makes the first debug build compile `qbx.cpp` slower] → the program's objects only; libqb's objects
  are shared with plain builds (the spec's "A debug build first" scenario).

## Measured

From the CLI tests of D4 (task 3.2; `lldb` 23.1.2 of the clone, 2026-10-25; the stop sequences in the log of
`cargo test -p qb64rust-driver --test cli -- --ignored --nocapture debug_`). Each line: what `lldb` did, then
what the proxy's design takes from it.

- **`debug_stops_at_a_line`**: `breakpoint set -f dbg.bas -l 11` resolves to `QBMAIN(void*) + N at dbg.bas:11:1`
  (the `#line` directives name the full path; `lldb` matches the base name); a line inside a `FOR` loop of
  several lines stops once per pass; `p *__LONG_TOTAL` gives 0, then 1. *Proxy:* `setBreakpoints` passes the
  file and line through as they are.
- **`debug_next_steps_by_line`**: `next` stops once per BASIC line, at column 1 (lines 2 to 8, six `next`s): a
  line of two statements is one stop, and a one-line `FOR` loop is one stop for its three passes; a breakpoint on
  that line stops once, as the loop is entered. No stop at the statement's `evnt` check: the risk of `study\36`
  §3.1 does not happen. *Proxy:* `next` within a body is `lldb`'s `next`, with no stepping until the line changes.
- **`debug_next_over_call_and_into_sub`**: `next` over `CALL show(x)` goes to the next line. `step` stops at the
  SUB's header line (`call.bas:7:21`, the prologue, which has the header's `#line`), then in `is_error_pending()` at
  `error_handle.h:24` (an inline function of libqb's headers, compiled into the program's own object, so it has
  line information), at the header again (`7:5`), then at the first statement (`8:1`). `finish` returns to the
  `CALL`'s line at column 5 (the rest of its statement), and `next` from there goes to line 4. `next` from the
  SUB's last statement stops at `END SUB` (`10:15`), then at the `CALL`'s line (`3:5`), then at line 4. *Proxy:* a
  step stops only at a line of a file the symbol file lists, past a procedure's header line (`procedures[].line`):
  it steps on (or out of a frame in another file) until then; after a return, a stop on the caller's line at a
  column other than 1 is the rest of the `CALL` and is stepped past.
- **`debug_resume_retry`**: `next` over the raising line (`s$ = CHR$(n)`, error 5) goes to the next line: the
  handler and the retry run inside libqb's `evnt` call, which `next` steps over. A breakpoint in the handler stops
  at its line in a second `QBMAIN` frame: the stack is `QBMAIN` (`retry.bas:8`) ← `evnt(linenumber=4)`
  (`qbx.cpp`) ← `QBMAIN` (`retry.bas:4:19`) ← `stdcall_thread_wrapper`. `next` from there stops at `RESUME`
  (line 9), then at `retry.bas:12`, a line past the end of the 9-line file (after the last statement's `#line 9`,
  the C++ lines of `sub_end(); return; }` count on from it), then in `evnt` in `qbx.cpp`. Error 11 (division by
  zero) is critical and reaches no handler, so the test raises error 5. *Proxy:* a stop at a line past its
  file's end, or in a file the symbol file does not list, is stepped out of; the stack folds `evnt` and shows the
  handler's frame above the raising line's.
- **`debug_reads_each_storage`**: from the SUB's frame, the expressions built from the file's `c_name`, `reach`
  and layouts read every storage: `p *__LONG_TOTAL` 7 (main, `pointer`; `lldb` names `int32` `int32_t`),
  `*_SUB_SHOW_INTEGER_CALLS` 1 (static), `*_SUB_SHOW_SINGLE_X` 8 (local), `*_SUB_SHOW_LONG_N` 7 (param,
  `param-pointer`), `((float*)__ARRAY_SINGLE_A[0])[2 - __ARRAY_SINGLE_A[4]]` 1.5 (`descriptor`: data slot 0,
  the lower bound of the one dimension at slot 4), `__STRING_S->len` 2 and `*(char(*)[2])__STRING_S->chr`
  `"hi"` (`string`), `*(short*)((char*)__UDT_P + 0)` 3 and `*(char(*)[4])((char*)__UDT_P + 2)` `"abcd"`
  (`bytes`, the members' offsets). The main module's variables are C globals, seen from every frame. *Proxy:*
  `variables` and `evaluate` write these expressions from the symbol file; no `lldb` formatter script is needed.
- **`debug_stops_at_error`**: `breakpoint set -n error` resolves to libqb's `error(int)`, which has no debug
  information, so its code is not a named parameter: it is `(int)$ecx` (the first argument on x64 Windows). With
  `-c "(int)$ecx == 5"` the program stops there at `ERROR 5`, with frame 1 `QBMAIN at err.bas:3:1`; with `== 6`
  it runs to its end. *Proxy:* a stop on an error (of a code, or any) is this breakpoint and condition, and the
  stop is shown at frame 1.

- **`debug_step_into_an_included_function`** (task 3.3): `step` from a call of `twice&`, defined in `lib.bi`,
  stops at its header `lib.bi:1:21`, then at four lines of `data1.txt` (the procedure's data fragment: its
  `#include` has no `#line` of its own, so `lldb` shows the fragment's lines), at the header again, in
  `is_error_pending()`, at the statement `lib.bi:2:1`, at `END FUNCTION` `lib.bi:3:15`, and back on the caller's
  line at column 11. *Proxy:* `data1.txt` is one more file the symbol file does not list, stepped past as the
  others are.

Found on the way:

- **A procedure of an included file had its header and `END` lines under the main file's name** (fixed, task
  3.3, user, 2026-10-25): `procs.rs` `procedure` wrote the header and the footer with `self.lines(proc.line, …)`,
  which names the main file, so `FUNCTION twice&` on line 2 of `inc/debug_info.bi` had `#line 2
  "debug_info_include.bas"` while its statements had the `.bi`'s name, and a `step` into it stopped first at line
  2 of the main file. Both are now written with `lines_in(Some(proc.file), …)`. The C++ of a program with a
  procedure in an included file changes in its `#line` file names only (the `cpp` snapshots of
  `errorlocation_cpp`, 20 directives, and `url_cpp`, whose procedures come from QB64pe's auto-included
  `afterlastline.bm`); every other program's C++ is as before. This is the one change to the C++
  this change makes, and it is the same with and without `--debug`.
- **A procedure closed in an included file had its `END` line under the header's file** (fixed, task 3.4, at the
  review of 2026-10-25): an included file can close a SUB the including file opened (`include_cross_ok.bas`,
  measured in `verification\v16_m7_sub_across`), and the footer took `proc.file`. `Proc` (`sema` and `ir`) now has
  `end_file` beside `end_line`, set from the closing statement's span; the footer is written under it, and the
  symbol file's procedures carry it (`end_file`, spec `compiler/debug-info`). No program in the tests' C++
  snapshots closes a procedure so, so none changes but the symbol files', which gain the field.
- **Stripping keeps the global symbols** (task 1.3): the `Makefile`'s `objcopy --strip-unneeded` removes the local
  symbols and the debug information and leaves the global ones, so `nm` lists `QBMAIN` in a stripped executable
  too; the spec's scenario "Unstripped on request" was corrected to the local symbols.

## Open Questions

- Whether the symbol file should also list labels (for a future "run to label"): left out; a reader can add
  them in version 2 without changing anything here.
