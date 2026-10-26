# Design

## Context

See `proposal.md` for why. What the code has now:

- **Addresses are of the main module only.** `Checker::var_address` (`sema\src\check\places.rs`) sets
  `Var::conventional` on the root of the place it names, and refuses ("not supported yet") a root whose storage is
  `Storage::Param`, `Local`, `Static` or `Result` (the `what` match before `conventional = true`). The emitter
  already allocates a variable flagged so from `dblock` (`codegen-cpp\src\decl.rs`, `alloc`, which turns to
  `cmem.rs`, `alloc_dblock`, when `in_dblock` says so) and an array with flag 4 (`alloc_array`), and every
  procedure already saves `cmem_sp` on entry and gives it back on exit (`procs.rs`, `tmp_cmem_sp`), as the old
  compiler does. `in_dblock` and `uses_dblock` look at the flag, not at the storage, except for `_BIT` scalars
  (main module only, as measured in `v56_a3`).
- **The old compiler** marks a variable named by an address word in `cmemlist` and compiles again
  (`qb64pe.bas` 21306–21560). A procedure's variable so marked is allocated in the procedure's data fragment,
  which runs on each call, from `cmem_sp` (17790, 18215–18733). A parameter so marked sets its bit in
  `sfcmemargs` (12810–12833); on the next pass each call marks its argument's variable (12081, 12188, 12202) or,
  for a value, writes a slot `passN` in the caller's data fragment taken from `cmem_sp` (12266–12272).
- **Measured while planning** (`qb64pe.exe` 4.7.0, scratch programs, to be recorded as `v61`, task 1.1): locals
  ` 65516  80  65512  65518 ` (`k`, `VARSEG(k)`, `l AS LONG`, `STATIC st`); a recursive FUNCTION's local 65510,
  65512, 65514, 65516 from the deepest call up; `SADD` of a FUNCTION's string 257, `VARPTR` of it 65506; a `TYPE`
  argument through two SUBs at 65516, segment 80; an array parameter's caller arrays, static, dynamic and of a
  SUB, all in conventional memory; `t 3` gives 65518, `t (x%)` and `t x% + 1` 65518 and 65516; **`t x%`, `t g`
  (`DIM` or `DIM SHARED`) and `t w` (a SUB's local) each "UNEXPECTED INTERNAL COMPILER ERROR"**, while
  `t a(2)` (an element) gives 4 and the array's segment and the `POKE` reaches `a(2)`.
- **`CALL ABSOLUTE`** is refused in `check\proc.rs` (the arm that names `ABSOLUTE`, `INTERRUPT`, `INTERRUPTX`).
  The old compiler (10687–10766) writes `call_absolute_offsets[i]=…` for each argument but the last, then
  `call_absolute(n, offset)`: a variable argument as `VARPTR` of it (which marks it), any other value through a
  slot `passN` in the data fragment (`float`, `double` or `int64` by the value's type).
- **Member names**: `places.rs` keeps `MEMBER_NAMES_FREE` (4 names) and `MEMBER_NAMES_TAKEN` (`PRINT`) and marks
  every other keyword or built-in name. The old compiler's rule is exact (`qb64pe.bas` 2313–2320): a name is
  refused when it is in its reserved table (1250–1395) with `HASHFLAG_TYPE`, `HASHFLAG_CUSTOMSYNTAX`,
  `HASHFLAG_OPERATOR` or `HASHFLAG_XELEMENTNAME`; every other name `validname` passes is taken, the names of
  built-in functions and statements among them.
- **The qbasic suite** runs through `run_legacy_tests.py --suite qbasic`, one job only (the `--jobs` check), under
  the clone's shared lock; `close.py` does not run it.

## Goals / Non-Goals

**Goals:**

- All 143 programs of `qbasic_testcases` build with `qb64rust`, and stay so through the closing runs.
- Every form above gives the old compiler's offsets and bytes, held by a recorded family.

**Non-Goals:**

- `CALL INTERRUPT` and `CALL INTERRUPTX` (stay in `SOMEDAY.md`).
- Running the qbasic programs or recording their screens (they read the keyboard; `SOMEDAY.md` row).
- `VARPTR` of a whole array that is not of numbers, and the forms `SOMEDAY.md` lists as not measured, other than
  those of procedures.

## Decisions

### D1. A procedure's variable is flagged as a main-module one is

`var_address` drops the refusal for `Local`, `Static` and `Result` and sets `conventional` on the root as for
`Main`. The emitter then allocates it from `dblock` with the text it writes for the main module, in the
procedure's allocation of its locals, which runs on each call after `tmp_cmem_sp` is saved, so the bytes are
given back at exit. A `STATIC` variable is allocated once, in `maindata` after the main module's variables (the
old compiler's C++ of `v61_a1`: `_SUB_S_INTEGER_ST` in `maindata.txt`, `k` and `l` in `data1.txt`; `v61_a5`), which
is where the emitter already allocates every variable that lives as long as the program: nothing moves. A local
array of a procedure is a dynamic descriptor with flag 4, allocated at its `DIM` and freed by `free1.txt`, as the
old compiler writes it (`v61_a7`) and as the emitter already does for a flagged dynamic array (`alloc_array`,
`new_block`, `free_block`).
*Code:* `sema\src\check\places.rs` (`var_address`), `codegen-cpp\src\cmem.rs` (`in_dblock`, `uses_dblock`),
`decl.rs` (the locals' and statics' allocation).
*Alternative:* a separate area per procedure: rejected, the offsets would not be the old compiler's.

### D2. Parameters: a fixed point after the bodies are checked

`var_address` on a parameter's place sets `conventional` on the parameter's `Var`. When every body is checked, a
pass over the kept arguments (`Checker::conventional_args` in `check\proc.rs`; each call's places are kept by
`keep_call_arg` as `CallArg { param, root, plain, span }`, so the bodies are not walked again) runs to a fixed
point: for each argument to a parameter flagged so, an `Arg::Ref` sets `conventional` on its place's root (and,
when that root is a `TYPE` or array parameter, flags it, so the next round reaches its callers); an `Arg::Array`
likewise on the array. An `Arg::Ref` whose place is a plain variable not of a user type (not an element, not a
member; a number, a string, a fixed-length string, a scalar parameter passed on among them) is the compile error
"a variable cannot be passed to a parameter whose address is taken": the old compiler's internal error, measured
for every such kind (`v61_px1`–`px8`), so a scalar parameter never carries the need further. It has no note
after it: the internal error names no line ("Caused by (or after):T X"), and the wording table pairs messages by
line (`tools\diagnostics`). A `TYPE` with a
dynamic member array or a `_MEM` reached so is "not supported yet", as for `VARPTR` of one.

An `Arg::Temp` (a number; a string expression's copy is no slot, `v61_p7`) to a parameter flagged so becomes a
slot when the emitter writes the call: `T *passN`, allocated from `cmem_sp` in the body's data, and the argument
`&(*passN=value)` (the old compiler's C++ of `v61_p4`; `static` in the main module, where the old compiler's is
global). The old compiler places the slot among the body's variables in source order (`v61_p8`: `a`, the slot,
`b`), so `Arg::Temp` carries the number of variables made before it, `Arg::Temp(Expr, u32)`, and the emitter
merges the slots into the body's allocations by it (`Emitter::with_slots`).

`VARPTR` and `VARSEG` of a scalar or `TYPE` parameter are libqb's `varptr_dblock_check` and
`varseg_dblock_check` of where it points, as the old compiler writes them (`qb64pe.bas` 21468–21552): its argument
may be an element of an array outside `dblock` (`v61_p5`: segment 4176). For a `TYPE` parameter `VARSEG` keeps the
member's place, whose offset the check adds, an element of a member array too (`v61_p9`); `VARPTR` of a place in
one is its offset from `dblock`, as for a variable, wherever the argument lies: the old compiler finds the
parameter by the place's offset, not its id (`qb64pe.bas` 21464; its C++ of `v61_p1`), so that segment and
offset disagree for a `TYPE` array's element (`v61_p9`). `VARPTR$` of a parameter is likewise its offset from
`dblock` (`v61_p10`). Found in the review of the change; both are rows of `SOMEDAY.md`'s "QB64pe behaviours to
review", with `SADD` of a string value's copy (`v61_p7`).
*Code:* `sema\src\check\proc.rs`, `check\mod.rs` (`CallArg`), `sema\src\lib.rs` (`Arg`), `ir` (`validate.rs`: an
argument to a parameter in conventional memory is there), `codegen-cpp\src\value.rs` (`arg`), `cmem.rs`
(`cmem_slot`, `with_slots`, the checks), `lib.rs` and `procs.rs` (the merge).
*Alternative:* compile twice as the old compiler does: rejected, the checker has no second pass, and the fixed
point gives the same set.

### D3. CALL ABSOLUTE is a statement of its own

`CALL ABSOLUTE(args…, offset)` checks into `StmtKind::CallAbsolute { args: Vec<AbsArg>, offset: Expr }` with
`AbsArg::Address(Expr)` (the `VARPTR` of a variable argument, built by `address_of`, split from `var_address`, so
the variable is flagged) or `AbsArg::Value(Expr, u32)` (`SINGLE` from a `SINGLE`, `DOUBLE` from another float,
else `_INTEGER64`, by the type the old compiler believes the value has, `Expr::qb`: `1.5!` is held as a `DOUBLE`
here; the number places its slot as `Arg::Temp`'s). The offset is an `_UNSIGNED INTEGER`. The emitter writes the
old compiler's text (its C++ of `v61_c4`): `call_absolute_offsets[i]=…;` for each argument, a value's
`(uint16)(((uint8*)&(*passN=value))-((uint8*)dblock))`, then `call_absolute(n,offset);`. libqb has the emulator
and `call_absolute` already (`libqb.cpp` 17350), so nothing is added to the copy. `CALL ABSOLUTE(…)` is the
built-in also where the program has a `SUB ABSOLUTE`, which `ABSOLUTE 5` calls (`v61_c9`); `ABSOLUTE (…)` and
`ABSOLUTE …` without one are "Syntax error" and `CALL ABSOLUTE …` without parentheses "Expected CALL sub-name
(...)" (`c5`–`c7`): errors here.
*Code:* `sema\src\check\proc.rs` (`call_absolute`), `places.rs` (`address_of`), `sema\src\lib.rs` (`AbsArg`),
`ir\src\lib.rs`, `lower.rs`, `validate.rs`, `dump.rs`, `codegen-cpp\src\stmt.rs`.

### D4. Member names by the old compiler's table

`MEMBER_NAMES_FREE` and `MEMBER_NAMES_TAKEN` give way to `MEMBER_NAMES_REFUSED`: the names of the old compiler's
reserved table that carry one of the four flags, copied from `qb64pe.bas` 1250–1395 with the line numbers, and
refused with its wording "Name already in use", and `END`, which the old compiler reads as the block's end
("Expected END TYPE"; `AS`, `DATA`, `ELSE` and `REM`, in the table, fail with other messages too). A name with a
single leading underscore is "Invalid name" (`validname`, 28273; `v61_m__width`, `v61_m__mine`). Every other name
that is a valid name is taken. The family `v61_m*` is generated by `verification\v61_members.py` (one program per
name of the table, and built-in names, `Time` among them), each declaring the member and using it on both sides
of an assignment and in `PRINT`: all 195 recordings are met with no change to the parser.
*Code:* `sema\src\check\places.rs` (the member loop of the `TYPE` block); the generator a script under
`verification\`'s tools as for earlier generated families.

### D5. The qbasic suite in the closing runs, with jobs

The runner takes `--jobs` for `--suite qbasic` with `qb64rust`, each program compiled in its own build folder as
for `compile` (it compiles only; the programs stay in the clone, under the shared lock). `close.py` gains the step
`qbasic` (`--suite qbasic --qb64 target\release\qb64rust.exe --jobs 8`), always in the closing set and in the
merge set, judged as the other runner steps; `CLAUDE.md` working rule 9's two lists name it. No pass list: every
program must pass, so the suite is its own list.
*Code:* `tools\legacy_tests\run_legacy_tests.py` (the `--jobs` check and the qbasic path), its test;
`tools\close\close.py` and `test_close.py`.

## Risks / Trade-offs

- [A `STATIC` variable allocated inside the saved `cmem_sp` would be given back and overwritten by the next call's
  locals] → it is allocated in `maindata` (task 1.2); `v61_a1`, `a5` compare a later call and another
  procedure's call.
- [The fixed point flags more than the old compiler, which flags per pass] → the family has a chain of three
  procedures and members and elements of parameters passed on; offsets are compared, so a variable moved to
  conventional memory that the old compiler leaves out shows as a different offset.
- [A slot's place among the variables follows the order `sema` makes them in, the old compiler's the order it
  compiles them in] → the two agree for declared and implicit variables in one body (`v61_p8`, `v61_a6`); a
  string parameter's copy into conventional memory (`qbs_new_cmem`, when the argument is there) runs after the
  locals' allocations here and before them in the old compiler, a difference no program of the family meets.
- [Conventional memory runs out (error 257) in deep recursion, as with the old compiler] → the same as the old
  compiler; no divergence.
- [`CALL ABSOLUTE` runs machine code in libqb's emulator; an opcode it lacks ends the program] → the same in both;
  the family uses only a far return and a store.
