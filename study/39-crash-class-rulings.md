# 39. The crash-class rows of "QB64pe behaviours to review": the rulings (2026-10-26)

Asked by the user after `study\38` §2 and §5.3 item 1 ("the 165 rows"): the user chose to leave the list of
"QB64pe behaviours to review" (`SOMEDAY.md`) as it is, and to go through the rows that end the program, hang,
touch memory or are undefined behaviour one at a time, each with the panel's view (working rule 6). Two decisions
came out of the first rows and stand on their own in `DECISIONS.md` (2026-10-26): **the fixes switch** (named
fixes with one master switch, in `qb64rust.toml` and on the command line, off for an existing program and on for
a new project; crash, hang and memory fixes always on, with no switch) and **the QB 4.5 oracle** (`tools\qb45\`).
This document holds the rulings row by row, so that the change that builds them has one place to start from.

Nothing here is built yet (`STATUS.md` "Next" item 4). The 18 ruled rows left `SOMEDAY.md`; row 1 was part of
its row of the oddities kept on 2026-10-08, and only that part left it.

## 1. Summary

| Row | Behaviour | Always fixed | Behind the switch | Kept |
|---|---|---|---|---|
| 1 | `RESUME NEXT` after an error in a block's condition | | after the block | |
| 2 | Console input that has run out | error 62; `END` without the key wait | | |
| 3 | `DIM`/`REDIM` with a count below one | below zero: error 5 | zero, and a bound that raises: error 5, nothing allocated | |
| 4 | `STATIC a(5)` beside a shared array of that name and type | beside a static one: the SUB's own | beside a dynamic one: the SUB's own | |
| 5 | `BLOAD` and `POKE` past the array their address came from | past libqb's block: error 5 | | inside the block |
| 6 | A whole array `a()` as a value | | compile error | |
| 7 | A whole array with no storage used whole | error 9 | | |
| 8 | A `_MEM` block of a dynamic array after `ERASE` | the block invalid | | |
| 9 | `_MEMFREE` of a `_MEMNEW(0)` block (double free) | lock type 0 | | |
| 10 | `STRING * n` member or element to a C `char*` | copy in, copy out | | |
| 11 | `_STATUSCODE` of a handle never opened | error 52 | | |
| 12 | Division by zero, error 11, untrappable | trappable | | |
| 13 | `RESUME` written in a SUB or FUNCTION | the normal exit path | compile error | |
| 14 | `RUN` in a console-only program; `RUN` from a procedure | output kept; unwind first | compile error | |
| 15 | OpenGL errors 270 and 271 fatal | | | both |
| 16 | `_CAST` of an out-of-range float constant | a variable's value | | |
| 17, 18 | Bit number or shift count outside 0 to 63 | count masked | error 5 for the bit functions | |
| 19 | An error under `$CHECKING:OFF` serviced late | | serviced at the first checked statement | |

Rows 1, 6, 15, 19 and the inside-the-block half of 5 turned out not to be of the crash class: each is
deterministic and touches no memory it does not own. Rows 16, 17 and 18 are decided by working rule 12 (a
well-defined value where QB64pe's depends on the C++ compiler or the machine).

## 2. The rulings

**(1)** `RESUME NEXT` after an error in a block's condition (`WHILE`, `DO WHILE`, `DO UNTIL`, `IF`, `ELSEIF`,
`LOOP WHILE`/`UNTIL`): a fix; with it on, `RESUME NEXT` goes on after the whole block (after `WEND`, `LOOP`,
`END IF`), so it cannot loop for ever (choice A of three; not a hang to fix always, since it loops only when the
condition raises on every pass).

**(2)** Console input that has run out (`verification\v22_e_eof`): **fixed always, no switch** (a hang, by the
2026-10-08 rule; panel unanimous): a console read at the end of a redirected input raises error 62, trappable, and
`END` skips its key wait when the input is no console (at a console it still waits). First measured with
`qb64pe.exe` for `LINE INPUT` and `INPUT$(n)` as well as `INPUT`, so the fix covers every console read; a
`QB64PE-DIVERGENCES.md` row says a program trapping error 62 on its input would hang under QB64pe.

**(3)** A dynamic `DIM`/`REDIM` with a count below one (`verification\v24_b_dynamic`, `v24_x89`–`x96`), split by
kind: **a count below zero is fixed always** (today it ends the program with "Out of memory", which no handler
receives, exit code 0): it raises error 5, trappable; **a count of zero leaving the array allocated with no
element, and a bound that raises still allocating, are a fix behind the switch**: error 5 for every count below
one and nothing allocated after an error in the statement. A trappable error 7 for a really huge array (`v24_x92`)
is a candidate, not ruled.

**(4)** `STATIC a(5)` in a SUB beside a shared main-module array of that name and type (`verification\v24_x272`,
`v55_d1`–`d4`): the `STATIC` declares **an array of the SUB's own**, as QB64pe already does when the types differ
and as QB 4.5 does (measured under DOSBox, `verification\qb45\q45_static_local`: with `STATIC a() AS LONG` and
`DIM a(5)` the SUB's array is local and kept between calls, the shared one untouched; in QB 4.5 the number in
`STATIC a(n)` counts dimensions, `q45_static_dims`, `DIVERGENCES-QB45.md` Q-015). **Fixed always for a static
shared array** (today the program ends when it starts, "Duplicate definition", before any handler); **a fix behind
the switch for a dynamic one** (today the `STATIC` gives the shared array its bounds and the main `DIM` raises
error 10). The name is resolved once in `sema`, and every later use of it (and the language server) follows that
answer (panel unanimous).

**(5)** `BLOAD` and `POKE` writing past the array their address came from (`verification\v28_b_bload`;
`GET (1, 2)-(3, 4), 5` is already a plain error here, `v28_x15`): **inside libqb's block of conventional memory
kept, with no switch** (the purpose of `BLOAD` and `POKE`, as in QB 4.5; `POKE` and `PEEK` cannot leave the block,
segment and offset being 16 bits each). **Past the end of the block fixed always** (read in `libqb.cpp`'s
`sub_bload`, not yet run: the length is never checked, so a `BLOAD` near the top of the block, or of a file whose
header gives size 0 and which is longer than 64 KB, writes into the process's memory after `cmem`): `BLOAD`, and
`BSAVE` after its code is read, raise error 5 before touching anything when start plus length runs past the
block. Both cases measured under QB64pe first, with a `QB64PE-DIVERGENCES.md` row (panel unanimous, with a security
engineer).

**(6)** A whole array `a()` as a value in an expression (`PRINT a()`, `a() + 1`, …: the array's first element;
`verification\v54_c_whole_values`): not a memory row (the first element of an allocated array exists; an erased one
is row 7); **a fix behind the switch**: a compile error wherever `a()` stands as a value, as QB64pe gives for
`x = a()` and as QB 4.5 gives everywhere (measured: "Syntax error", `verification\qb45\q45_whole_array_value`).

**(7)** A whole array with no storage (after `ERASE`) used whole (`verification\v54_d_whole_erased`,
`v54_x42`–`x44`: its data is libqb's `nothingvalue`, 8 bytes shared by every unallocated array, so `READ s()`
writes over it and every later `PRINT` ends with the text read): **fixed always**: every whole-array use of an
array with no storage (`PRINT a()` with the fixes off, `READ`, `INPUT #`, `SWAP`, whole-array `GET`/`PUT`) raises
error 9 before touching anything, as an element of it does, checked in one place where a whole array's data is
taken (panel unanimous, with a security engineer). QB 4.5 has no whole-array `READ` or `SWAP` ("Syntax error",
`verification\qb45\q45_whole_array_erased`).

**(8)** A `_MEM` block of a dynamic array still valid after `ERASE` (`verification\v32_d29_erase_dynamic`:
`_MEMEXISTS` -1 and `_MEMPUT` writes into the released memory; libqb renews an array's lock only in `REDIM`):
**fixed always**: `ERASE` of a **dynamic** array renews its lock as `REDIM` does, through one emitter helper the
two share, so a block of it raises the error a block raises after `REDIM` (its number measured, not taken from the
row); `ERASE` of a static array only clears it, and its blocks stay valid. The other paths that release a dynamic
array (`CLEAR`, `RUN`, a SUB's local array at its end) are measured under QB64pe and fixed in the same change where
they fail the same way (panel unanimous, with a security engineer).

**(9)** `_MEMFREE` of a `_MEMNEW(0)` block taken after another block was freed
(`verification\v32_d33_free_zero_size`, exit 127: the 0-byte branch of `memblock.cpp`'s `func__memnew` sets no lock
type, so a reused lock keeps type 1 and its old pointer, freed a second time): **fixed always**, a libqb edit
setting the lock's type to 0 (owns no memory, as `_MEM` at an address) for a 0-byte block; tests in the other
orders (freed first, two in a row, between two normal blocks, freed twice gives 307); an `UPSTREAM.md` entry with
the one-line patch (panel unanimous).

**(10)** A `STRING * n` member or array element passed to a library's `char*` (`verification\v37_x66`: no NUL after
its n bytes, so C reads on into the next member; the call passes the string's own bytes, `(char*)(s)->chr`, so C's
writes come back, `v37_d_strings`): **fixed always, by copy in and copy out** for every fixed-length string that
is not a variable of its own (a member, an array element): the n bytes go into a temporary of n + 1 with a NUL,
the temporary is passed, and the n bytes are copied back after the call, so writes still reach the member and a
NUL written at n lands in the temporary. The place is evaluated once; variables and variable-length strings keep
the direct pointer. Written in codegen, no libqb edit; a C function that keeps the pointer after it returns is not
served (it never was for a member), said in the `QB64PE-DIVERGENCES.md` row (panel unanimous, with a Windows API
programmer and a security engineer).

**(11)** `_STATUSCODE` of a negative handle never opened (`verification\v44_x59`–`x68`: an access violation, exit
139, since `libqb.cpp`'s `func__statusCode` reads the entry `list_get` did not find): **fixed always**, error 52 as
the other network functions raise, a libqb edit checking for the missing entry; a handle opened and then closed,
and a handle of another type, tested too; an `UPSTREAM.md` entry. This replaces the ruling of 2026-10-17 ("kept
for now", its condition, libqb copied, now met) (panel unanimous, with a network programmer).

**(12)** Error 11, division by zero, ends the program and no handler receives it (`verification\v02b_idiv_zero`;
libqb learns of an integer division by zero from the CPU's exception, `qbx.cpp`'s `division_by_zero_handler`, and
cannot resume after it, so 11 is in `error_handle.cpp`'s critical list): **fixed always**: the generated code
checks the divisor of every integer `\` and `MOD` (one helper per type) and raises error 11 the usual way with 0
as the value, never dividing, and libqb's critical list loses 11, so `ERROR 11` is trappable too; the signal
handler stays as a backstop. As QB 4.5 (measured: `\`, `MOD`, `ERROR 11` and float `/` by zero all trapped as 11,
`verification\qb45\q45_div_zero`; `DIVERGENCES-QB45.md` Q-003 becomes measured). The `_INTEGER64` minimum `\ -1`
measured under QB64pe first; constant folding kept consistent. Float `/` by zero giving infinity is not part of
it: a separate candidate for the switch. This replaces the 2026-10-08 ruling for Q-003 (panel unanimous, with a
QuickBASIC programmer).

**(13)** `RESUME`, `RESUME 0`, `RESUME NEXT` written in a SUB (`verification\v57_x01`–`x04`: a bare C++ `return`,
skipping the SUB's exit code, so its strings leak and the locks of its locals stay valid, a `_MEM` block of a
local then writing into memory no longer its own) and `RESUME label` there: **fixed always**: the SUB or FUNCTION
is left through its normal exit path (the label `EXIT SUB` uses), so nothing leaks and no lock stays; **a fix
behind the switch**: every form of `RESUME` inside a SUB or FUNCTION is a compile error, as in QB 4.5 (measured:
"Subprogram error", `verification\qb45\q45_resume_in_sub`). The forms in a FUNCTION measured under QB64pe first
(panel unanimous, with a QuickBASIC programmer).

**(14)** `RUN` (`verification\v57_r02`–`r07`): **`RUN label` or `RUN n` in a `$CONSOLE:ONLY` program losing all
later output, fixed always** (it goes on in screen 0 of a window that does not exist; the console stays the
output, as at the first start); **`RUN` from a SUB or FUNCTION, fixed always** (QB64pe calls the main module again
inside the procedure, so the stack grows and the procedure's strings stay with each run): every active procedure
is left through its normal exit path first, by the same unwind `END` and `SYSTEM` from a procedure use, built with
row 13's exit path; **a fix behind the switch**: `RUN` in a SUB or FUNCTION is a compile error with a hint (set a
flag, `RUN` from the main module), as in QB 4.5 (measured: "Subprogram error" for `RUN` and `RUN 100`,
`verification\qb45\q45_run_from_sub`). How many runs from a SUB it takes QB64pe to run out of stack measured first
(panel unanimous, with a game programmer).

**(15)** Errors 270 (an OpenGL routine outside `SUB _GL`) and 271 (`END` or `SYSTEM` inside it) as libqb's
critical errors (`verification\v59_f1`–`f8`): **kept as QB64pe, no switch, and not of the crash class**: designed
fatal errors with a clear message; 271 is raised on the drawing thread, where a handler of the main module cannot
safely run, and 270's wrappers call OpenGL after raising it (`gl_helper_code.h`), so a trappable 270 would need
every wrapper changed, for a programming error a handler cannot recover from (the QB64 engineer would have made 270
trappable behind the switch). Noted for later: a compile-time warning for an OpenGL call outside `SUB _GL`, and the
five routines that never raise 270 (`SOMEDAY.md`).

**(16)** `_CAST` of a float constant outside the integer type's range (`verification\v49_x18`: undefined in C++,
g++ folds it to another value on each build): **fixed always** (working rule 12): `_CAST` from a float gives what
the same conversion of a variable gives on x86-64, type by type as `verification\v49_a_cast` measures (`LONG`
through a 32-bit conversion, -2147483648 out of range; the narrow types through the low bits; NaN and infinity the
CPU's value), through one helper in the generated code for every `_CAST` from a float, so constants, variables and
the run without folding agree; `v49_x18` leaves `known.list`; a `QB64PE-DIVERGENCES.md` row (panel unanimous; saturating
rejected, as it would differ from a variable).

**(17, 18)** A bit number outside 0 to 63 in `_READBIT`, `_SETBIT`, `_RESETBIT`, `_TOGGLEBIT`, and a shift count
outside it in `_SHL` and `_SHR` (`bitops.h`: a C++ shift by it, undefined; with the count in a variable x86 masks
it to its low 6 bits, `verification\v58_f09`): **fixed always** (working rule 12): a libqb edit of `bitops.h` masks
the count to 0–63, so a constant gives what a variable gives today, and constant folding follows the same rule; an
`UPSTREAM.md` entry. **A fix behind the switch**: error 5 for a bit number outside 0 to 63 in the four bit
functions; what the switch should do for the shifts (0 for 64 or more, error 5 for a negative count) is a separate
candidate. `_ROL` and `_ROR` already mask (panel unanimous).

**(19)** An error raised under `$CHECKING:OFF` serviced at the first checked statement, so `RESUME NEXT` skips that
statement and `RESUME` runs it again (`verification\v50_c_checking`): **a fix behind the switch**: a pending error
is serviced before the first checked statement run after unchecked ones (the `$CHECKING:ON` line, the end of a
region, a label reached from one), so `RESUME NEXT` goes on at it; the error still waits through the unchecked
statements, and libqb's routines still do nothing meanwhile (panel unanimous).

## 3. To measure before building

Each of these was named in a ruling as measured under `qb64pe.exe` first; collected here for the change's first
task group.

- Row 2: `LINE INPUT` and `INPUT$(n)` at the end of a redirected input.
- Row 5: a `BLOAD` at `&HFFFF:&HFFF0` of a 64 KB file, and of a file whose header gives size 0 and which is longer
  than 64 KB; `BSAVE`'s code read for the same limit.
- Row 8: the error number a block raises after `REDIM`; `CLEAR`, `RUN` and a SUB's local dynamic array at its end.
- Row 10: whether an element of a `STRING * n` array has a NUL after it (read in the code, not run).
- Row 12: the `_INTEGER64` minimum `\ -1`.
- Row 13: the forms of `RESUME` in a FUNCTION.
- Row 14: how many `RUN`s from a SUB QB64pe takes to run out of stack.

## 4. Candidates noted

Rows of `SOMEDAY.md`'s "QB64pe behaviours to review", not ruled: a float `/` by zero giving infinity (QB 4.5 raises
11), the shifts' behaviour under the switch, a trappable error 7 for an array too large for memory, a compile-time
warning for an OpenGL routine outside `SUB _GL`.
