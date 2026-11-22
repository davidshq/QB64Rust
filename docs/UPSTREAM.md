# Fixes to offer QB64pe

Bugs of QB64pe that this project fixes in its own code or its copy of libqb, kept here so they can be reported or
sent to QB64pe later (user, 2026-10-18). Each entry says what is wrong, where in QB64pe's sources at the
reference commit (`16f629784e`), the fix, and this project's divergence row. When one is sent, its entry says
where (issue or pull request) and what came of it. Fixes QB64pe has merged are in
`docs\archived\UPSTREAM-completed.md`.

## `_COTH(x)` is not the hyperbolic cotangent

- **Wrong:** `_COTH(x)` returns `2 * EXP(x) - 1`: `_COTH(0)` is 1, `_COTH(.2)` 1.442805516320339709 and
  `_COTH(1)` 4.4366 (the hyperbolic cotangent: undefined, 5.0665 and 1.3130). It also raises error 5 for any
  argument above 44.014845, where the hyperbolic cotangent is 1 to double precision. Measured in this repo:
  `verification\v45_x32`, `v45_b_funcs`.
- **Where:** `internal/c/libqb/include/extended_math.h`, `func_coth`. The `CONST` evaluator
  (`source/utilities/const_eval.bas`, `CASE "_COTH"`) calls `_COTH` of the compiler itself, so the one fix
  corrects both a statement and a `CONST`.
- **Fix:** the hyperbolic cotangent, `(EXP(2x) + 1) / (EXP(2x) - 1)`, written so it does not overflow for large
  arguments (for example `1 / std::tanh(num)`), with error 5 at 0 as the neighbouring functions raise for a
  division by zero (`func_csch`), and no upper limit.
- **Here:** `DIVERGENCES-QB64PE.md` D-031, built by `const-rulings`: the edit `tools/libqb/edits/d031-coth.edit` of this
  repo's copy of libqb (`1 / std::tanh(num)`, error 5 at 0) is the patch to offer, and the `CONST` evaluator here
  computes the same function.
- **Sent:** draft pull request QB64-Phoenix-Edition/QB64pe#823 (2026-11-18), from branch
  `fix-coth-hyperbolic-cotangent` of `davidshq-contribute/QB64pe`. It adds the test `tests/compile_tests/math/coth`
  and corrects `const/math_funcs`, which expected the old `_COTH(.2)` of 1.4428. Not yet reviewed.

## A `TYPE` with a `STRING` in conventional memory has no strings

- **Wrong:** a `TYPE` variable of the main module that holds a `STRING` member, and that `VARPTR`, `VARSEG`,
  `SADD` or `VARPTR$` names anywhere in the program, is allocated in conventional memory with its bytes zeroed
  and its strings never made: the first use of the string reads a null pointer and the program crashes (exit
  139). Measured in this repo: `verification\v56_a5`, `v60_d6`.
- **Where:** `source/qb64pe.bas`, `dim2`, the scalar user-type branch (lines 17790–17852 at the reference
  commit): `initialise_udt_varstrings` and `free_udt_varstrings` are written only in the `ELSE` branch (ordinary
  memory), not after the `memset` of the `cmemlist` branch.
- **Fix:** the same two calls in the conventional-memory branch, after its `memset` (the string descriptors are
  ordinary `qbs` values, so no offset in `dblock` moves).
- **Here:** `DIVERGENCES-QB64PE.md` D-037, built by `m4-left-marked` (`crates\codegen-cpp\src\cmem.rs`, `alloc_dblock`).
- **Sent:** not yet.

## `-y` writes a number in `_ARRAYCOPY` subscripts as its internal element

- **Wrong:** formatting `_ARRAYCOPY g(1.5 TO 3.5) TO gd(0.5)` with `-y` (or the IDE's auto-format) writes
  `_ArrayCopy g(1.5E+0,1.5 To 3.5E+0,3.5) To gd(0.5E+0,0.5)`: each number with a point or an exponent comes out
  as the element `lineformat$` makes of it, the value and the spelling joined by a comma. The commas make another
  program (four subscripts where there were two). Found by this repo's format oracle:
  `tests\format\corpus\slice\s131_arraycopy.bas`, line 167.
- **Where:** `source/utilities/arrcpy.bm`, `ArrayCopyRefCaseLayout$`, which lays out the raw elements of the
  reference through `CompactMemberRefLayout$` (`source/qb64pe.bas`); a number element is never turned back into
  the text after its comma, as `fixoperationorder$` does for an expression's layout.
- **Fix:** lay out each number element by the part after its comma (the spelling as written), as the expression
  layout does; or build the subscripts' layout from `fixoperationorder$`'s.
- **Here:** `DIVERGENCES-QB64PE.md` D-038, built by `m5b-formatter`: the formatter writes the number as written.
- **Sent:** not yet.

## `-y` with `AutoLayout` off can change a line it lays out

- **Wrong:** with `AutoLayout` off (`-f:autolayout=0`, or the IDE's setting), a line whose layout differs from the
  user's text by more than spacing and letter case comes out changed. `SELECT CASE x: CASE 1 + +2: PRINT " three":
  END SELECT` is laid out `Case 1 + 2` (the sign folded), and the line written is `Select Case x: Case 1 + +2:
  PRINT " thre"": END SELECT`: the string loses a letter and `END SELECT` goes into a string, so the program no
  longer compiles. Found by this repo's format oracle:
  `tests\format\settings\autolayout=0\corpus\slice\s17_signs.bas`, line 26.
- **Where:** `source/utilities/format.bas`, `apply_layout_indent$`, the loop that walks the user's text beside the
  laid-out one (`olay$`) to carry the keywords' case over. It skips spaces on either side and takes any other
  difference as one character for another, so a character the layout left out puts the two out of step for the
  rest of the line. The closing-quote case then puts a quote in place of a letter.
- **Fix:** stop carrying at the first difference that is not spacing, letter case, `?` or a missing closing quote,
  and keep the rest of the user's text as it is. Or keep `layout2$` whenever the result, without spaces and with
  letters outside strings in capitals, equals neither `olay$` nor the user's text.
- **Here:** `DIVERGENCES-QB64PE.md` D-039, built by `m5b-format-settings`: `crates\format\src\apply.rs` keeps QB64pe's
  result where it is the laid-out or the user's line in that sense, and otherwise does the first of these.
- **Sent:** not yet.

## Console input waits for ever at the end of a redirected input

- **Wrong:** in a console program whose standard input is a file, a pipe or the null device, `INPUT` and `LINE
  INPUT` at the end of the input sleep and read again for ever, also on a last line with no newline; `INPUT$(n)`
  with no file number reads only the keyboard buffer, so it waits for ever even while data is on its input; `END`
  prints "Press any key to continue" and waits for a console key that cannot come. A program fed from a file
  never ends. Measured in this repo: `verification\v22_e_eof`, `v63_a1`–`a5`, `v26_e_end`.
- **Where:** `internal/c/libqb.cpp`: `qbs_input` (the console branch of its key loop, `fgetc(stdin)`),
  `func_input` (the keyboard branch), `sub_end` (the wait after "Press any key to continue").
- **Fix:** at `EOF`, take a line read in part as ended, and with nothing read raise error 62 and leave through
  the clean-up of the Enter path; `INPUT$` reads standard input when it is not a console and raises 62 at its end;
  `END` waits for a key only when `GetConsoleMode` succeeds on the input (the null device is a character device,
  so `GetFileType` cannot tell). On Linux and macOS QB64pe's `END` already ends at `EOF`.
- **Here:** `DIVERGENCES-QB64PE.md` D-048; the edits `tools/libqb/edits/d048-console-eof-input.edit`,
  `d048-console-eof-input-string.edit`, `d048-console-eof-end.edit`.
- **Sent:** not yet.

## `BLOAD` and `BSAVE` run past the block of conventional memory

- **Wrong:** `BLOAD` reads the header's length, or the whole file when the header gives size 0 and the file is
  64 KB or more, to its destination in `cmem` without checking that it fits, so a `BLOAD` near `&HFFFF:&HFFFF`
  writes into the process's memory after the block (a crash, or silent damage); `BSAVE` likewise reads past the
  block into the file. Measured in this repo: `verification\v63_b1`–`b4`.
- **Where:** `internal/c/libqb.cpp`, `sub_bload` (before `fh.read`) and `sub_bsave` (before the file is opened).
- **Fix:** compare the destination's offset in `cmem` plus the length with `sizeof(cmem)` and raise error 5
  before reading or writing anything.
- **Here:** `DIVERGENCES-QB64PE.md` D-049; the edits `tools/libqb/edits/d049-bload-past-block.edit`,
  `d049-bsave-past-block.edit`.
- **Sent:** not yet.

## Division by zero cannot be trapped

- **Wrong:** error 11 is in `error()`'s list of critical errors, so `ON ERROR` never receives a division by zero,
  where QuickBASIC 4.5 traps it (`verification\qb45\q45_div_zero`). The generated code already tests every integer
  `\` and `MOD` for a divisor of 0 and raises 11 without dividing, so nothing needs it to be critical. Measured in
  this repo: `verification\v02b_idiv_zero`, `v63_e1`, `e2`.
- **Where:** `internal/c/libqb/src/error_handle.cpp`, `error()` (the critical switch) and `error_code_to_text`
  (no text for 11); `internal/c/qbx.cpp`, `division_by_zero_handler`.
- **Fix:** drop 11 from the critical switch and give it its text; keep the SIGFPE handler fatal through a function
  of its own (returning from it would run the faulting division again).
- **Here:** `DIVERGENCES-QB64PE.md` D-052; the edits `tools/libqb/edits/d052-*.edit`.
- **Sent:** not yet.

## A bit number or shift count of 64 or more is undefined

- **Wrong:** `_READBIT`, `_SETBIT`, `_RESETBIT`, `_TOGGLEBIT`, `_SHL` and `_SHR` shift by the count as given,
  undefined in C++ outside 0 to 63: a variable count gets x86's masking, a constant count whatever g++ folds it to.
  Measured in this repo: `verification\v58_f09_bits`, `v63_f1`.
- **Where:** `internal/c/libqb/include/bitops.h`.
- **Fix:** `b1 & 63` in the six functions, as `func__rol` and `func__ror` already mask.
- **Here:** `DIVERGENCES-QB64PE.md` D-053; the edit `tools/libqb/edits/d053-bit-numbers-modulo-64.edit`.
- **Sent:** not yet.

## A program ended by a runtime error exits with code 0

- **Wrong:** a program that ends on an error no handler takes (with `QB64PE_NOPROMPT=y`, or "no" at the
  "Continue?" dialog) or on a critical error (300 to 315, a failed `_ASSERT`, out of memory) exits with code 0, so
  a script or test runner cannot tell it from a clean end. Measured in this repo: `verification\v02c_no_handler`,
  and every recording of `verification\` that ends in `Runtime error` (`exit=0`).
- **Where:** `internal/c/libqb/src/error_handle.cpp`: `fix_error` calls `end()` with `exit_code` still 0, and
  `exit_after_fatal_error` calls `libqb_exit(0)`.
- **Fix:** `exit_code = 1` before `end()` in `fix_error`'s "no" path, and `libqb_exit(1)` in
  `exit_after_fatal_error`. A program that goes on after the error (`QB64PE_NOPROMPT=continue`, "yes") keeps the
  code its end gives.
- **Here:** `DIVERGENCES-QB64PE.md` D-012; the edits `tools/libqb/edits/d012-error-exit-code*.edit` (which also
  write this project's run-time trace, a part not to offer).
- **Sent:** not yet.

## `_SCREENCLICK` misses under a display scale

- **Wrong:** on Windows with a display scale other than 100%, `_SCREENCLICK x, y` clicks at the scale times its
  place on the desktop (at 125%, a click at `_SCREENX + 100` landed 67 pixels further right), and one near the
  desktop's far side misses the program's window. `x, y` are in GLFW's pixels, as `_SCREENX` and `_DESKTOPWIDTH`
  give them, while the rectangle the place is scaled by is read on the program's thread, where the scale shrinks it.
  Measured in this repo in Windows Sandbox at 125% (`tools\sandbox\README.md`).
- **Where:** `internal/c/libqb.cpp`, `sub__screenclick`: `GetWindowRect(GetDesktopWindow(), &rect)` as the size
  for `SendInput`'s 0..65535.
- **Fix:** scale by `func__desktopwidth()` and `func__desktopheight()` (GLFW's video mode), keeping the rectangle
  where they are 0 (a `$CONSOLE:ONLY` program).
- **Here:** `DIVERGENCES-QB64PE.md` D-056; the edit `tools/libqb/edits/d056-screenclick-display-scale.edit`.
- **Sent:** not yet.

## Strings of more than 4 GB together crash

- **Wrong:** a program crashes (exit 139) once its variable-length strings make libqb's string pool grow past
  4 GB. Two strings of 1.5 GB at once crash before anything is printed. One string of 1,123,123,123 bytes
  assigned seven times crashes at the fourth assignment. Measured in this repo: `verification\v63_g2` (the
  second program; the first was measured once, 2026-11-18, and not kept, as it needs 6 GB).
  These are QB64pe issues 278 (open) and 559 (open, the second program).
- **Where:** `internal/c/libqb/src/qbs.cpp`, lines 79 and 80: `qbs_data_size` and `qbs_sp` are `uint32_t`. In
  `qbs_concat`, `qbs_data_size * 2 + bytesrequired` wraps to a small value, and `realloc` shrinks the pool under the
  strings. `qbs_sp * 2` in the same function wraps once the strings end past 2 GB.
- **Fix:** `size_t` for both, as issue 278 proposes, and error 512 ("Out of memory") in `qbs_concat` when the new
  size would not fit in a `size_t`, which can happen in a 32-bit build.
- **Here:** `DIVERGENCES-QB64PE.md` D-066; the edits `tools/libqb/edits/d066-string-pool-size.edit` and
  `d066-string-pool-growth.edit`.
- **Sent:** not yet.

## A concatenation past 2 GB crashes

- **Wrong:** `b$ = a$ + a$` with `a$` of 1.2 GB crashes (exit 139). A string's length is an `int32_t`, and the
  sum of the two lengths wraps negative. libqb then makes a string of that negative length. Measured in this repo:
  `verification\v63_g3_concat_over_2gb`. No QB64pe issue reports it: a search of its issues on 2026-11-18 found
  only 559, which is about the pool's size (the entry above).
- **Where:** `internal/c/libqb/src/qbs.cpp`, `qbs_add`: `qbs_new(str1->len + str2->len, 1)`. `qbs_new` takes any
  negative size. `internal/c/libqb.cpp`, `func__readfile`, reads a file's length with `ftell` into an `int`, which
  gives -1 for a file of 2 GB or more on Windows. It ends in error 75 there only because `fread` of that length
  fails.
- **Fix:** add the lengths in 64 bits and raise error 5 when the sum is past `INT32_MAX`, as QB 4.5 raises 5 for a
  string past its own limit. Raise error 5 in `qbs_new` for a negative size, for any other caller. In
  `func__readfile`, read the length with `_ftelli64` (Windows) or `ftello`, and raise error 75 for a file too long.
- **Here:** `DIVERGENCES-QB64PE.md` D-067; the edits `tools/libqb/edits/d067-string-add-too-long.edit`,
  `d067-string-new-negative.edit` and `d067-readfile-too-long.edit`.
- **Sent:** not yet.

## `REDIM` without `AS` of a `TYPE` array makes a second array

- **Wrong:** after `REDIM a(2) AS f`, with `f` a `TYPE`, `REDIM a(5)` leaves the `TYPE` array as it was. It makes
  a second array `a!()` of the default type with the new bounds, so `a(4).foo = 2` raises error 9. The same
  `REDIM` of a `LONG` or `DOUBLE` array resizes it. This is QB64pe issue 95 (open since 2022-06-04). Measured in
  this repo: `verification\v24_x327_redim_type_no_as`, `v24_x328_redim_type_no_as_values`.
- **Where:** `source/qb64pe.bas`, `dimgottyp` (line 9111 at the reference commit): a `REDIM` with no type adopts
  the existing array's type only when `id.mayhave` is set. That field holds a type's suffix, and a `TYPE` has
  none.
- **Fix:** adopt the type of a `TYPE` array too: `IF LEN(RTRIM$(id.mayhave)) OR (id.arraytype AND ISUDT) THEN`.
  `id2fulltypename$` already gives a `TYPE`'s name. Not tried in QB64pe's source.
- **Here:** `DIVERGENCES-QB64PE.md` D-068. QB64Rust takes the existing array's type for every type.
- **Sent:** not yet.

## Other candidates

Bugs this project fixes in the `CONST` evaluator (built by `const-rulings`), which QB64pe might also want (not
ruled to be sent; listed so they are not forgotten):

- Values beyond `_INTEGER64` in a `CONST` clamped, and the four limits `_INTEGER64_MIN`, `_OFFSET_MIN`,
  `_UINTEGER64_MAX`, `_UOFFSET_MAX` wrong by it (`DIVERGENCES-QB64PE.md` D-027).
- `^` of two integers in a `CONST` rounded to an integer, `2 ^ -1` being 0 (D-028).
- A `CONST` beyond `_FLOAT` range being 0 or "Unexpected element 'INF'" (D-029).
