# Design

## Context

See `proposal.md` for why. The rulings are `study\39` §2, rows 2, 5, 9, 11, 12, 17 and 18. The switch and its
mechanisms are `fixes-switch`'s (`openspec\changes\archive\2026-10-27-fixes-switch\design.md`, D1 to D4), used
unchanged. What shapes the approach:

- `libqb\` is a byte-for-byte copy of QB64pe's at the pinned commit, changed only by the edit files of
  `tools\libqb\edits\` that `tools\libqb\copy_libqb.py` applies in name order (several may edit one file; each
  edit's old lines must occur exactly once). There is one edit today, `d031-coth.edit`. `libqb\SOURCE.md` lists
  them, and `copy_libqb.py --check` holds the copy to them. The driver rebuilds libqb's objects when a file under
  `libqb\internal\c` changes (`build.rs`, the stamp of the tree), and the build cache's key follows the same
  stamps, so an edit reaches every later build.
- What each row's code is today (read at the pinned commit):
  - Row 2: `qbs_input` (`libqb.cpp`, the console branch of its key loop) reads `fgetc(stdin)`; at `EOF` it sleeps
    10 ms and loops, for ever. `LINE INPUT` is the same function with its `lineinput` flag. `INPUT$(n)` with no
    file (`func_input`, "keyboard/piped") reads `qbs_inkey()`, libqb's keyboard buffer, which console events fill;
    it never reads standard input, so with redirected input it may wait even while data remains (measured in task
    1). `sub_end`, with the console and the window hidden, prints "Press any key to continue" and loops on
    `func__getconsoleinput()` until a key-down event, which redirected input never gives.
  - Row 5: `sub_bload` reads the 7-byte header and then `fh.read` of the header's size (or of the file's length
    less 7 when the size is 0 and the file is 64 KB or more) to `defseg + offset`, or to the header's segment and
    offset; no length is checked against `cmem`'s 1,114,099 bytes. `sub_bsave` writes `size` bytes (up to 65,536)
    from `defseg + offset`, again unchecked: a read past the block.
  - Row 9: `func__memnew` (`libqb\src\memblock.cpp`) sets the lock's type in every branch but the 0-byte one, so a
    lock taken again from the freed list keeps type 1 and its old pointer, and `_MEMFREE` frees that pointer again.
  - Row 11: `func__statusCode` (`libqb.cpp`) reads `sh->type` of `list_get`'s result without testing it for
    `NULL`, as `func__connected` does.
  - Row 12: the generated code already tests every integer `\` and `MOD` for a divisor of 0 and calls `error(11)`
    (`codegen-cpp` `decl.rs`, `qb_safe_idiv` and `qb_safe_mod`, the old compiler's templates), and the folder
    leaves a division by 0 to run time (`sema` `check\ops.rs` `fold_binary`). Error 11 is fatal only because
    `error()` (`libqb\src\error_handle.cpp`) puts `QB_ERROR_DIVISION_BY_ZERO` in its list of critical messages.
    `qbx.cpp`'s `division_by_zero_handler`, the SIGFPE handler, calls `error(11)` too, and returning from it
    re-runs the faulting instruction.
  - Rows 17, 18: `bitops.h` shifts by the count as given (`1ull << b1`, `a1 << b1`), undefined outside 0 to 63;
    g++ folds a constant count to another value, while a variable gets x86's masking. `sema` does not fold these
    functions. `_ROL` and `_ROR` already mask.

## Goals / Non-Goals

**Goals:**
- Each always-on row is one edit file per place it touches, with the reason and the divergence row in its header,
  so that `UPSTREAM.md` can offer the same lines.
- With every fix off, the generated C++ is unchanged: rows 2 to 18 here live in libqb, not in the emitter. Only
  `bit-number-check` adds C++, and only with the fix on.

**Non-Goals:**
- A float `/` by zero (infinity today; QB 4.5 raises 11) and the shifts under the switch: candidates of
  `study\39` §4.
- `INPUT$` in a window program. It reads the window's keyboard, and its input never runs out.
- Making the SIGFPE path trappable. It stays the fatal backstop (D5).

## Decisions

### D1: One edit file per row and place, named by its divergence row

The rows get the divergence numbers D-048 to D-053, in row order: D-048 row 2, D-049 row 5, D-050 row 9, D-051
row 11, D-052 row 12, D-053 rows 17 and 18. Each edit file is `d0nn-<what>.edit`, and a row that touches two
places has two files (`d048-console-eof-input.edit`, `d048-console-eof-end.edit`, and so on). Their header gives
the reason, the `QB64PE-DIVERGENCES.md` row and the `UPSTREAM.md` section, as `d031-coth.edit` does.
`libqb\SOURCE.md` lists each file, and `copy_libqb.py` rewrites `libqb\`.

*Alternative:* one edit file for all rows of `libqb.cpp`. Rejected: an edit's old lines are one block, so a
multi-place edit would span thousands of lines, and an upstream offer is made row by row.

### D2: Row 2, console input at the end of the input (`libqb.cpp`)

- `qbs_input`: at `EOF` from `fgetc(stdin)`, when the line read so far (`inpstr`) is not empty, the key becomes
  Enter (13), so a last line without a newline is taken as a line. When it is empty, `error(62)` is raised and
  the function leaves through the same clean-up as its Enter path (strings freed, images freed, `_SOURCE` and
  `_AUTODISPLAY` restored), with no new line printed. The variables keep the 0 or "" `qbs_input` gave them first.
  The generated code tests `is_error_pending()` after the call already, as for any libqb call.
- `func_input` (`INPUT$(n)`, no file): when the program's output is the console and its standard input is not a
  console (`GetConsoleMode` fails on the input handle), it reads with `fgetc(stdin)` instead of the keyboard
  buffer, and at `EOF` before n bytes it raises `error(62)` and gives what it read. At a console nothing changes.
  Task 1 measures first whether QB64pe's `INPUT$` reads redirected input at all. If it does not, reading it is
  part of this fix (it is the only way the read can end), and the `QB64PE-DIVERGENCES.md` row says so.
- `sub_end`: with the console and the window hidden, "Press any key to continue" is printed as now, and the wait
  for a key runs only when `GetConsoleMode` succeeds on the input handle. Otherwise the program ends at once. The
  line stays so that the corpus programs with a `.stdin` file, which reach `END` with redirected input, keep the
  output they were recorded with. QB64pe on Linux also prints its line and then ends at `EOF`.

`GetConsoleMode` is the test, not `GetFileType`: the null device is `FILE_TYPE_CHAR` too, and the verification
suite runs its programs with input from it.

*Alternative:* treat an `EOF` with part of a line as error 62 too. Rejected: QuickBASIC and QB64pe on Linux take a
last line without a newline, and a file written by an editor often ends that way.

### D3: Row 5, `BLOAD` and `BSAVE` past the block (`libqb.cpp`)

`sub_bload`: after the header is read and the length worked out, and before `fh.read`, the destination's offset
in `cmem` plus the length is compared with `sizeof(cmem)`. If it runs past, the file is closed and `error(5)` is
raised. `sub_bsave`: the same test of `defseg + offset` plus `size`, before the file is opened, so no file is
created. Error 5 is the ruling's. QB 4.5 has no such case (its block is the whole 1 MB address space), so it has
nothing to compare with.

### D4: Rows 9 and 11, a missing type and a missing test

- Row 9 (`libqb\src\memblock.cpp`, `func__memnew`): the 0-byte branch sets `mem_lock_tmp->type = 0`, the type of
  a block that owns no memory (as `_MEM` at an address has). `_MEMFREE` then frees the lock and nothing else.
- Row 11 (`libqb.cpp`, `func__statusCode`): `if (!sh || sh->type != special_handle_type::Http)` raises 52.

### D5: Row 12, error 11 trappable (`error_handle.cpp`, `error_handle.h`, `qbx.cpp`)

`error()` loses `case QB_ERROR_DIVISION_BY_ZERO` in its critical switch, so `error(11)` from `qb_safe_idiv`,
`qb_safe_mod` or `ERROR 11` sets `new_error` like any other error. A new function in `error_handle.cpp`,
`void fatal_division_by_zero()`, keeps the old critical path (`fatal_error_alert` with "Division by zero" and the
location, then `exit_after_fatal_error`). It is declared in `error_handle.h`, and `qbx.cpp`'s
`division_by_zero_handler` calls it in place of `error(11)`. The handler is reached only by a division the
emitter did not guard: a float-to-integer conversion trap or libqb's own code. Returning from it would run the
faulting instruction again, so it must stay fatal.

Nothing changes in the emitter or the folder: they already raise 11 the usual way, give 0, and leave a division by
0 to run time. Error 11's message with no handler becomes the ordinary one for an untrapped error ("Runtime error:
Line: n …" and "Division by zero"), as for error 9. *Found in task 2.5:* `error_code_to_text` had no text for 11,
which only the critical path printed, so an untrapped 11 printed "Unprintable error"; a fourth edit
(`d052-division-error-text.edit`) gives it "Division by zero". The ordinary message turned out to equal the old
critical one with `$ERRORLOCATION:ON` (`v58_e20`–`e25`, `e29` still pass); without the setting it now names the
line (`v58_e21`).

*Alternative:* keep 11 critical and raise a different number from the generated code. Rejected: `ERR` must be 11,
as in QuickBASIC (`q45_div_zero`).

### D6: Rows 17 and 18, the count modulo 64 (`bitops.h`)

`func__shl`, `func__shr`, `func__readbit`, `func__setbit`, `func__resetbit` and `func__togglebit` use `b1 & 63` in
place of `b1`. That is x86's masking of a variable count, so the values a variable gets today stay as they are
(`v58_f09_bits`, `v30_h_shifts`), and a constant now gets the same. `sema` folds none of these functions, so no
folder changes, and the run without folding agrees by construction.

### D7: The switched fix `bit-number-check` (`base` `fixes.rs`, `codegen-cpp` `builtins.rs`)

`Fix::BitNumberCheck`, named `bit-number-check` and described as "a bit number outside 0 to 63 in _READBIT,
_SETBIT, _RESETBIT and _TOGGLEBIT is error 5", is appended as the eighth entry of `FIXES`, so the other bits keep
their places. In `codegen-cpp` `builtins.rs` `call`, for those four functions with the fix on, the bit argument
(already converted to its LONG slot by `table_call`) is wrapped in `qbr_bit_number(…)`. That helper, written into
the program's prelude (`decl.rs`) only when the fix is on, raises `error(5)` outside 0 to 63 and returns the number
unchanged, so libqb's mask gives the value the fix-off program gives. With the fix off nothing is emitted: the C++
is unchanged, so the call-site check is unaffected. The build key already carries the fix bits (`fixes-switch` D4).

*Alternative:* a libqb entry per function that checks. Rejected: libqb would then depend on the fix set, while
every other fix decides in the compiler and leaves libqb one build for all programs.

### D8: The recordings that change, and the tests

- Family `v63` ("the libqb fixes", task 1) records QB64pe's side with `qb64pe.exe` before any edit: `INPUT`,
  `LINE INPUT` and `INPUT$` at the end of redirected input and with a last line without a newline; `END` with
  input from the null device; `BLOAD` at `&HFFFF:&HFFF0` of a file of 1,000 bytes, and of a file with size 0 and
  200,000 bytes; `BSAVE` there; `_MEMNEW(0)` freed in the other orders and twice; `_STATUSCODE` of a handle
  opened and closed and of a host handle; `ERROR 11` under a handler; `_SETBIT(0, 64)` and kin with constants.
  A program whose QB64pe run writes past memory may crash or print garbage, and its recording keeps that, as
  `v62`'s do.
- The programs whose recording shows the old behaviour go to `tests\verification\known.list` with their `D-0nn`:
  of the ten that record a division by zero, the four whose output changes (`v02_errors`, `v02b_idiv_zero`,
  `v17_g_mod_zero`, `v58_e21`; `v58_e20`, `e22`–`e25` and `e29` still pass, task 2.5), `v22_e_eof`, the four that time out at `END` (`v22_x50`, `v22_x59`,
  `v26_e2_fall_off`, `v26_e_end`), `v32_d33_free_zero_size`, `v44_x59`, and the `v63` programs that differ. A
  program whose only difference is the trace keeps passing with a `.divergences` sidecar.
- The corpus's three recordings of a fatal division by zero (`slice\s11_on_error`, `slice\s17_operators`,
  `runtime_comparison\228_err_erl_values`) go to `tools\legacy_tests\known_failures.txt` with the reason D-052.
  They are not re-recorded: a recording is the old compiler's output.
- `bit-number-check`: a `tests\verification\fixed\v63_<name>.out.txt` sidecar for its `v63` program (D4 of
  `fixes-switch`), and the CLI test of "The list" now expects eight lines.
- `copy_libqb.py --check` and its own tests (`test_copy_libqb.py`) pass with the new edits.
- Each fix's trace: the paths of rows 2, 5, 9 and 11 run in libqb, where `qbr_diverged` (a function of the
  generated program) cannot be called. Those rows are "no run-time path" in `QB64PE-DIVERGENCES.md`'s list of
  untraced rows, as D-031 already is.

## Risks / Trade-offs

- [A `.stdin` corpus program that reached `END` waited for the helper's Shift press, and now ends at once] → Its
  output is the same: the line is still printed (D2). The helper's presses go unread, which is harmless.
- [A program at a real console that relied on `INPUT` waiting after Ctrl+Z] → At a console, Ctrl+Z is the end of
  input, so error 62 is QuickBASIC's answer too. The divergence row says so.
- [An untrapped error 11 prints a different message] → Ten verification recordings and three corpus ones change
  (D8). The message is the ordinary one for an untrapped error.
- [The SIGFPE handler is still fatal] → Only for divisions the emitter does not guard. If one shows up, it is a
  missing guard in the emitter, to fix there.
- [Each edit makes `libqb\` drift further from the pinned commit] → Each is a small, named block that
  `copy_libqb.py` re-applies to a new commit or reports as no longer matching. `UPSTREAM.md` offers them so the
  drift can shrink.

## Migration Plan

Nothing to migrate. libqb's objects are rebuilt once by the first build after `copy_libqb.py` runs. The build
cache misses once, since its key follows libqb's stamps. `vscode\CHANGELOG.md` gets a line on the fixes.

## Open Questions

- Whether QB64pe's `INPUT$` reads redirected input at all (D2). *Answered in task 1.2:* it does not; it waits for
  ever even with data on its input (`verification\v63_a3`), so reading it is part of D-048.
- Which line an untrapped error 11 on a joined line names (`v58_e23`). *Answered in task 2.5:* the line the old
  critical message names, so `v58_e23` still passes.
