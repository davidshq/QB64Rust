# Proposal

## Why

QB64pe's `tests\qbasic_testcases` (143 classic programs, read from the clone, never copied) had never been built
with `qb64rust`; tier 1 only puts them through the front end. A first run of the suite with the new compiler
(2026-10-26, `--suite qbasic`, compile only, as QB64pe's own `qbasic_tests.sh`) built 135 and rejected 8, each
with "not supported yet" errors only. The old compiler builds all 143. The user asked for the three gaps to be
built (2026-10-26):

- **Addresses of a procedure's variables and parameters** (6 programs: `misc/ripples`, `pete/simpire/ssimpire`,
  `pete/pp256/usersubs`, `misc/cabsmous`, `n54/big/jpegmake`, `qb45com/action/arqanoid/arqanoid`). `VARPTR`,
  `VARSEG` and `SADD` of a SUB's or FUNCTION's own variables, of an element of an array parameter
  (`VARSEG(ImageArray(1))`) and of a member of a `TYPE` parameter (`VARPTR(State.Leftover)`). `m4-memory-records`
  built the main module only and left these "until a program needs them" (`ROADMAP.md`, M4); these programs are
  that need.
- **A `TYPE` member named like a keyword or built-in** (`pete/wetspot/wetspot`: `Time AS INTEGER`). We accept only
  the four names measured in `verification\v18_h_member_names*`; the old compiler checks a member's spelling only
  (`validname`, `qb64pe.bas` 28249), and rejects a few names elsewhere (`PRINT`: "Name already in use").
- **`CALL ABSOLUTE`** (`n54/big/tictac`, `misc/cabsmous`): machine code run by libqb's x86 emulator, which turns
  the mouse interrupt `INT 33h` into QB64's mouse (`libqb.cpp` 17350, `call_int`). Our copy of libqb has it; the
  compiler never writes the call.

Measured with `qb64pe.exe` while planning (scratch programs; the change records them as a family): a procedure's
variables in conventional memory form a stack (a recursive FUNCTION's local moves 2 bytes down per call, a
`STATIC` one keeps its place); a `TYPE` variable or an array passed to a parameter whose address is taken moves
to conventional memory, through a chain of calls; an expression passed there gets a slot of its call site; **a
plain scalar variable passed there is the old compiler's internal error** ("UNEXPECTED INTERNAL COMPILER ERROR").

## What Changes

- **`VARPTR`, `VARSEG`, `SADD` and `VARPTR$` of a SUB's or FUNCTION's variables** (scalars, strings, fixed-length
  strings, `TYPE` values, members, elements of its arrays; `DIM`, `STATIC` and implicit) compile. Each such
  variable lives in conventional memory, allocated on each call below the caller's and given back at the
  procedure's end, a `STATIC` one once; the offsets are the old compiler's.
- **The same of a parameter**: a `TYPE` parameter, an array parameter and a scalar parameter. The argument of each
  call moves to conventional memory (a variable or array of the caller, a `TYPE` or array parameter passed on,
  through any chain of calls), and an expression argument gets a slot of its call site, as in the old compiler.
  **A plain variable passed to a parameter whose address is taken, a scalar parameter passed on among them, is a
  compile error** (the old compiler fails there, measured in `v61_px*`).
- **A `TYPE` member may be named like a keyword or a built-in** wherever the old compiler accepts the name; the
  names it rejects are compile errors with its wording. The set is measured, one program per name.
- **`CALL ABSOLUTE([args,] offset)`** compiles to libqb's `call_absolute`: each variable argument passed by its
  offset (the variable moved to conventional memory), each other value through a slot in conventional memory, as
  the old compiler writes it.
- **The qbasic suite runs with `qb64rust`** in the closing set of a change and at each merge into `main`, all 143
  expected to build; the runner allows `--jobs` for it.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/conventional-memory`: "What VARPTR and VARSEG accept" takes a procedure's variables and parameters;
  new requirements for a procedure's variables in conventional memory, for parameters and their arguments, and
  for `CALL ABSOLUTE`.
- `language/arrays-and-types`: "User types" takes members named like keywords and built-ins.
- `testing/upstream-tests`: a new requirement, the qbasic programs build with the new compiler.

## Impact

- `crates\sema` (`check\places.rs`: the address check and the member-name check; the propagation of
  conventional memory to arguments; `CALL ABSOLUTE` in `check\proc.rs`), `crates\ir` (the statement and its
  validation), `crates\codegen-cpp` (`cmem.rs`, `decl.rs`: allocation of a procedure's variables and of argument
  slots; the call).
- A new verification family (`v61`) recorded with `qb64pe.exe`, its programs on `tests\verification\pass.list`.
- `tools\legacy_tests\run_legacy_tests.py` (`--jobs` for the qbasic suite, with its test), `tools\close\close.py`
  (the qbasic run in the closing and merge sets, with its test), `CLAUDE.md` working rule 9's lists.
- Tier 1's shrink-only lists lose the programs that now compile (`tests\known_*.list`).
- `SOMEDAY.md` rows 56 (arrays of a procedure) and 249 (`CALL ABSOLUTE`) close; `CALL INTERRUPT[X]` stays there.
