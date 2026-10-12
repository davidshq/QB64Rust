# Golden corpus

What the old compiler (QB64pe 4.7.0, `16f629784e`, Windows) does with small headless programs: their exact output,
or the compile error the old compiler reports for them. It is the first conformance target for the new compiler:
any compiler that takes qb64pe's command-line flags can be checked against it. The corpus records behaviour; it does
not say whether that behaviour is right (that is the divergence register's job).

## Layout

| Path | Content |
|---|---|
| `runtime_comparison\` | 261 programs from QB64Fresh (`SOURCE.md`), unchanged copies |
| `verification\` | `v11_wrap_o2` and `v12_wrap_int64` from `verification\` (LONG and `_INTEGER64` overflow, `SOURCE.md`) |
| `slice\` | 65 programs written for the new compiler (numeric rules, PRINT forms, CP437 bytes, procedures, error handling, control flow, operators, constants, arrays and `TYPE`, built-ins, the numeric types, fixed-length strings, built-in statements, files, data, console input, default types, dynamic arrays, arrays used without `DIM`, arrays in procedures, array parameters, `COMMON`, `STRING` members, whole `TYPE` values and parameters of a user type, member arrays; `SOURCE.md`) |
| `screen\` | 29 programs with a window, checked by the state of the screen they leave: 23 written for the method (text cells, every screen mode, pages, keys, `END`, a runtime error) and 6 drawing programs cut from Civil War Strategy (`SOURCE.md`, `tools\screen\README.md`). None is on `slice.list`: the new compiler builds console programs only so far |
| `slice.list` | The 287 programs the new compiler must pass: all 65 of `slice\`, 220 of `runtime_comparison` and `verification\` `v11`, `v12` (see below) |

Each `<name>.bas` has exactly one of:

- `<name>.output`: the program must compile, run, and print this (stdout and stderr merged, byte-exact with CRLF
  line ends as recorded on Windows; `.gitattributes` keeps the bytes);
- `<name>.err`: the compile must fail with this compiler output;
- `<name>.norun`: compile only, never run; the file gives the reason;

and may have `<name>.normalize`: rules (a Python regular expression, a tab, a replacement; `#` lines are comments)
applied to each output line before comparing and before recording, for values that differ between machines or runs,
and `<name>.noprompt`: the value of `QB64PE_NOPROMPT` for the run instead of `y` (`continue` lets a program go on
after an untrapped runtime error; `slice\s12_error_in_print`),
and `<name>.stdin`: the program's standard input, byte for byte, for a program that reads the console with `INPUT`
or `LINE INPUT` (without it the program gets no input). **A program with a `.stdin` file must end with `SYSTEM`**:
`END` waits for a key that never arrives while standard input is a file (measured, 2026-10-10).

A program with an `.output` may also have `<name>.screen`: the state of the screen it must leave at exit, for a
program with a window (text cells or pixels, palette, cursor, pages; format in `tools\screen\README.md`), and with
it `<name>.keys`: the keys it is given. All 29 are in `screen\`; their `.output` is empty.

A program that reaches `END` prints an empty line and `Press any key to continue` (no line end) on Windows; that
trailer is part of its `.output`.

## Counts (recorded 2026-10-03; `slice` as each change added its programs, the last 2026-10-11)

| Kind | `runtime_comparison` | `verification` | `slice` |
|---|---|---|---|
| `.output` | 235 | 2 | 64 |
| `.err` | 20 | 0 | 0 |
| `.norun` | 1 (`239_lprint`: would print a page on the default printer) | 0 | 0 |
| Known failure, no expected file | 5 | 0 | 0 |
| `.normalize` sidecars | 3 (`212` `PATH` length, `234` `TIMER`, `238` a folder name in `cmd`'s error) | 0 | 0 |
| `.noprompt` sidecars | 0 | 0 | 2 (`s12_error_in_print`, `s19_header_errors`: `continue`) |
| `.stdin` sidecars | 0 | 0 | 1 (`s42_console_input`) |

The 20 compile errors are programs written against QB64Fresh's idea of the language. They still test that a
compiler rejects them, but not what their names say:

| Cause | Programs |
|---|---|
| `FUNCTION f (…) AS type` (QB64pe has no `AS` return type; reports `Expected )`) | 25, 65, 70, 83, 122, 123, 141, 142, 194, 195 |
| `BYREF` | 47, 190, 191 |
| `REDIM PRESERVE` (QB64pe spells it `_PRESERVE`) | 28, 202 |
| Single-line `IF … THEN … ELSEIF` | 19 |
| `TRIM$` (QB64pe has `_TRIM$`) | 16 |
| Parameter named `name` (`NAME` is a statement) | 48 |
| `FRE` (not implemented) | 218 |
| `DIM` of an already static array | 205 |

Known failures (`tools\legacy_tests\known_failures.txt`, reported KFAIL):

| Program | Cause |
|---|---|
| 80, 126, 187, 251 | `PRINT` with a comma pads to the next zone by asking the console for the cursor column. The runner gives each program a console (for `END`) but sends stdout to a file, so the column never moves and the program prints spaces until the 60 s timeout. Old-runtime bug (`study\00` §5). |
| 147 | `RESUME 0` re-runs the line that raised the error, which is `ERROR 5`: the program loops forever by design. |

Found while recording: `CHAIN` to a missing program (238) writes `chainNNN.tmp`, "moves" it to the program's own
folder, then runs `<program folder>\qb64pe.exe -c <target>.bas` through `cmd` (`sub_chain`, `qbx.cpp`) before
raising "File not found". A `qb64pe.exe` next to a chaining program would be started.

Recorded under the same setup and stable, but not what a screen would show: `CSRLIN` and `POS` (237) read the
console cursor, which stays at row 1, column 1 because the output goes to a file.

## Checking a compiler

```
python tools\legacy_tests\run_legacy_tests.py --suite corpus                       # old compiler (..\QB64pe\qb64pe.exe)
python tools\legacy_tests\run_legacy_tests.py --suite corpus --qb64 path\to\new.exe
python tools\legacy_tests\run_legacy_tests.py --suite corpus --glob "1*.bas"      # a few
```

Each program is compiled with `-q -m -x` in its own scratch folder under `target\legacy-tests\corpus\` and run
there with no arguments and `QB64PE_NOPROMPT=y` (or its `.noprompt`); details and options in
`tools\legacy_tests\README.md`. Known failures without an expected file are compiled but not run. A full run takes about 11 minutes with the old
compiler, almost all of it the C++ compile of each program. Baseline: `baselines\qb64pe-16f629784e-win64-corpus.json`.

## Recording

```
python tools\legacy_tests\run_legacy_tests.py --suite corpus --record
```

Compiles each program once, runs it twice, and writes the expected file only if both runs agree. A program whose
runs differ needs a `.normalize` rule (anchored, one value each), never an edited program. Before staging, check
that no recorded file holds a local path or a user or host name:

```
grep -rniE '[a-z]:[\\/]|users' tests/corpus --include=*.output --include=*.err
```

## Behaviour that depends on the C++ optimiser (`--cpp-opt`)

`--suite corpus --cpp-opt` builds with `-f:OptimizeCppProgram=true` (`-O2`) and compares with the same files.
Programs that fail there:

| Program | Default build | `-O2` build |
|---|---|---|
| `verification/v11_wrap_o2` | `x + 1 > x` false for `x& = 2147483647`; `PRINT x + 1` gives `-2147483648` | `x + 1 > x` true; `PRINT x + 1` gives ` 2147483648` (the stored `y = x + 1` still wraps) |
| `verification/v12_wrap_int64` | `x + 1 > x` false for `x&& = 9223372036854775807` | `x + 1 > x` true; the printed and stored values wrap in both builds |
| `slice/s17_operators` | `(l AND l) * 1000000000` and `(l \ 1) * 1000000000` with `l& = 5` and `7` wrap in 32 bits (`705032704`, `-1589934592`) | they print `5000000000` and `7000000000` |

Measured 2026-10-03 (`s17` 2026-10-06, the other `s13`–`s19` give the same output in both builds); every
`runtime_comparison` program gives the same output in both builds. That group has no
`_INTEGER64` program and no LONG overflow, which is why `v12` was added (`study\16` §8). Both differences come from
signed overflow being undefined in the generated C++ (no `-fwrapv`); the new compiler wraps for both LONG and
`_INTEGER64` (`DECISIONS.md`, 2026-10-03), so it must match the default-build files.

## The `slice` group and `slice.list` (new compiler)

`slice\` holds programs written for QB64Rust, recorded with `qb64pe.exe` like the rest: `s01`–`s07` by the change
`m2-workspace-and-slice`, `s08`–`s12` (procedures, error handling) by `m2-procedures-and-errors`, `s13`–`s19`
(control flow, operators, constants, header errors) by `m2-control-flow-slice`, `s20`–`s22` (arrays, `TYPE`, the
store rule of each place) by `m2-arrays-and-types`, `s23` by `m2-parser-breadth`, `s24` for a bug fix, `s25`–`s29` (built-ins,
`SELECT CASE`, `ON … GOTO`) by `m2-core-builtins`, `s30`–`s34` (the numeric types, `_BIT`, fixed-length strings) by
`m2-numeric-types`, `s35`–`s44` (built-in statements, files, data, console input, `SHELL`, functions by demand) by
`m2-builtin-statements`, `s50` and `s51` (three programs each: default types; `OPTION BASE`, the type before the names, a minus right of
`^`), `s52`–`s55` (six programs: dynamic arrays, `REDIM`, `REDIM _PRESERVE`, `ERASE`, arrays used without `DIM`),
`s56`–`s58` (arrays in procedures, array parameters, `COMMON`), `s59` and `s60` (`STRING` members, whole `TYPE`
values, parameters of a user type), `s61` (member arrays) and two for bugs fixed on the way (`s17_signs`,
`s52_dim_list`) by `m2-defxxx-common-arrays` (`SOURCE.md`). `s52_dynamic_memory` ends with the runtime's "Out of memory" message,
which is part of its `.output`; the runner's `QB64PE_NOPROMPT` keeps that from opening a dialog.
Each is named after what it pins (`s02_integer_wrap`, `s08_byref`...). `s07_cp437_bytes` has CRLF
line ends and bytes 0x80–0xFF, kept exactly by `.gitattributes`. None uses a `PRINT` comma (see above).

`slice.list` names the 287 programs the new compiler must pass (tier 2, `study\19`), also with
`QB64RUST_NO_FOLD=1`; the three `s50` programs and `235_deftype` joined with `m2-defxxx-common-arrays` task 2.2,
the three `s51` programs and `236_option_base` with its tasks 3.2 and 3.3, the five `s52`–`s55` programs and
`40_erase`, `120_redim_no_preserve`, `201_erase_udt_array`, `203_redim_multiple` with its group 4,
`s55_implicit` with its group 5, `s56_proc_arrays` and `206_shared_redim` with its group 6,
`s57_array_params`, `s58_common`, `s17_signs` and `s52_dim_list` with its group 7,
`s59_type_strings`, `s60_type_whole`, `57_type_string_field`, `200_type_assign_whole` and `242_types_simple`
with its group 8, `s61_member_arrays` and `197_type_array_field` with its group 9;
57 joined with `m2-builtin-statements` (`s35`–`s44` and 47 of `runtime_comparison`, named
task by task in its `tasks.md`). `s13`–`s19` joined it with `m2-control-flow-slice` (`s17` with task 3.2, the rest with task
8.1), `s20`–`s22` and 13 `runtime_comparison` programs with `m2-arrays-and-types` task 7.1, 61 programs with
`m2-core-builtins` (below), `s30`–`s34` and `198_type_fixed_string` with `m2-numeric-types`.

```
cargo build --release
python tools\legacy_tests\run_legacy_tests.py --suite corpus --qb64 target\release\qb64rust.exe --list tests\corpus\slice.list
```

`cargo test` (tier 1, `crates\driver\tests\inputs.rs` since `m2-upstream-tests`, together with the upstream
copy and the other input sets of `tests\upstream\README.md`) runs the front end over every corpus program and
requires no panic, an exact round trip, no diagnostics for the listed ones, at least one error for every program
with an `.err` file, and the two shrink-only lists `tests\known_false_errors.list` and
`tests\known_unsupported_rejections.list`. Add a program to the list when the compiler supports everything it
uses. The seeded mutation test (`crates\driver\tests\mutate.rs`) also starts from the corpus programs.

### Full corpus with `qb64rust` (2026-10-12, after member arrays; not a pass criterion)

After group 9 of `m2-defxxx-common-arrays` (its task 9.3, release build, against the reference clone, `--jobs 8`),
357 programs (the slice program of group 9 new):

| Result | Programs |
|---|---|
| Pass | 306: the 287 of `slice.list` and 19 `.err` programs |
| Rejected with a diagnostic (exit status 1) | 45 (15 `.output` programs, all 29 of `screen\` and the compile-only `239_lprint`) |
| `.err` programs that get only "not supported yet" errors | 1 (`218` `FRE`) |
| Known failures | 5, as below |
| Compiler crash, or an executable for a rejected program or with wrong output | 0 |

One corpus program carried the mark group 9 removed ("arrays as `TYPE` members"): `197_type_array_field`. It
became clean, passes and is on `slice.list`. The 15 `.output` programs still rejected wait on `PRINT USING`,
`TAB` and `SPC`, random and binary files, `EXIT SELECT`, `CHAIN`, `WIDTH`, `CSRLIN`/`POS` and a few functions
(`_SHL`, `_ROL`, `_MIN`, `_CLAMP`, `_IIF`).

### Full corpus with `qb64rust` (2026-10-11, after the rest of `TYPE`; not a pass criterion)

After group 8 of `m2-defxxx-common-arrays` (its task 8.3, release build, against the reference clone, `--jobs 8`),
356 programs (the two slice programs of group 8 new):

| Result | Programs |
|---|---|
| Pass | 304: the 285 of `slice.list` and 19 `.err` programs |
| Rejected with a diagnostic (exit status 1) | 46 (16 `.output` programs, all 29 of `screen\` and the compile-only `239_lprint`) |
| `.err` programs that get only "not supported yet" errors | 1 (`218` `FRE`) |
| Known failures | 5, as below |
| Compiler crash, or an executable for a rejected program or with wrong output | 0 |

Three corpus programs carried the marks group 8 removed ("`STRING` members", "assigning a whole `TYPE` value",
"a `TYPE` parameter"): `57_type_string_field`, `200_type_assign_whole` and `242_types_simple`. All three became
clean, pass and are on `slice.list`.

### Full corpus with `qb64rust` (2026-10-11, after array parameters and `COMMON`; not a pass criterion)

After group 7 of `m2-defxxx-common-arrays` (its task 7.3, release build, against the reference clone, `--jobs 8`),
354 programs (the four slice programs of group 7 new):

| Result | Programs |
|---|---|
| Pass | 299: the 280 of `slice.list` and 19 `.err` programs |
| Rejected with a diagnostic (exit status 1) | 49 (19 `.output` programs, all 29 of `screen\` and the compile-only `239_lprint`) |
| `.err` programs that get only "not supported yet" errors | 1 (`218` `FRE`) |
| Known failures | 5, as below |
| Compiler crash, or an executable for a rejected program or with wrong output | 0 |

One corpus program carried the marks group 7 removed ("array parameters", "`COMMON`"):
`194_array_param_2d`, an `.err` program (a FUNCTION typed with `AS`), which is rejected as before. No program
of `runtime_comparison` became clean; the four new programs are the group's own.

### Full corpus with `qb64rust` (2026-10-11, after arrays in procedures; not a pass criterion)

After group 6 of `m2-defxxx-common-arrays` (its task 6.3, release build, against the reference clone, `--jobs 8`),
350 programs (the 14 slice programs of groups 2 to 6 new):

| Result | Programs |
|---|---|
| Pass | 295: the 276 of `slice.list` and 19 `.err` programs |
| Rejected with a diagnostic (exit status 1) | 49 (19 `.output` programs, all 29 of `screen\` and the compile-only `239_lprint`) |
| `.err` programs that get only "not supported yet" errors | 1 (`218` `FRE`) |
| Known failures | 5, as below |
| Compiler crash, or an executable for a rejected program or with wrong output | 0 |

Three `.err` programs left the only-marked row since the run before: `28_redim` and `202_redim_preserve` with
group 4 (`REDIM PRESERVE` without its underscore is a syntax error), and `16_ltrim_rtrim` with group 5: `TRIM$`
is no built-in, so `TRIM$("  xy  ")` is an array used without `DIM` with a string as its index, the old
compiler's own error ("Illegal string-number conversion"). No program became clean with group 5: every corpus
program that still names an array without `DIM` does so after a declaration that is marked. With group 6,
`206_shared_redim` (`SHARED a() AS LONG` in a SUB) became clean and passes; the run before it, after group 5,
had 349 programs, 293 passing and 50 rejected.

### Full corpus with `qb64rust` (2026-10-10, after the screen programs; not a pass criterion)

After `m2-screen-state-check` (its task 6.1, release build, against the reference clone), 336 programs: the table
below with 29 more rejected. Pass 271, rejected with a diagnostic 56 (the 27 below and all 29 of `screen\`, each
for "programs without `$CONSOLE:ONLY`" first), `.err` programs with only marked errors 4, known failures 5;
`slice.list` 255 of 255. With the old compiler: 331 pass and the 5 known failures.

### Full corpus with `qb64rust` (2026-10-10, after the built-in statements; not a pass criterion)

After `m2-builtin-statements` (task 8.2, release build, against the reference clone), 307 programs (`s35`–`s44`
new):

| Result | Programs |
|---|---|
| Pass | 271: the 255 of `slice.list` and the same 16 `.err` programs |
| Rejected with a diagnostic (exit status 1) | 27 (26 `.output` programs and the compile-only `239_lprint`) |
| `.err` programs that get only "not supported yet" errors | the same 4 (`16`, `28`, `202`, `218`) |
| Known failures | 5, as below |
| Compiler crash, or an executable for a rejected program or with wrong output | 0 |

47 `runtime_comparison` programs joined `slice.list` over the change, of the about 60 its proposal counted as
carrying only marks it removes (an upper bound: a first mark hid `PRINT USING`, `GET`/`PUT`, `TAB` or a dynamic
array in the others). The 26 still rejected wait on step 11 of `STATUS.md` "Next" (`REDIM`, dynamic arrays,
`ERASE`, `OPTION BASE`, `DEFINT`, `STRING` and array members, whole-`TYPE` assignment: 11 programs), on random and
binary files (`GET`, `PUT`, `LOCK`: 3), on `PRINT` forms (`TAB`, `SPC`, `USING`, `WIDTH`: 4), on functions typed
by their arguments (`_MIN`, `_MAX`, `_CLAMP`, `_IIF`, the shifts: 5), and on `EXIT SELECT`, `CHAIN` and
`CSRLIN`/`POS` (one each); the list by program is in task 7.4 of the change.

### Full corpus with `qb64rust` (2026-10-09, after the numeric types; not a pass criterion)

After `m2-numeric-types` (task 10.1, release build, against the reference clone), 297 programs (`s30`–`s34` new):

| Result | Programs |
|---|---|
| Pass | 214: the 198 of `slice.list` and the same 16 `.err` programs |
| Rejected with a diagnostic (exit status 1) | 74 (73 `.output` programs and the compile-only `239_lprint`) |
| `.err` programs that get only "not supported yet" errors | the same 4 (`16`, `28`, `202`, `218`) |
| Known failures | 5, as below |
| Compiler crash, or an executable for a rejected program or with wrong output | 0 |

One `runtime_comparison` program joined `slice.list` over the change, `198_type_fixed_string`; no other program
was waiting only on the new types or fixed-length strings. The rejected ones are blocked by what step 9 of
`STATUS.md` "Next" brings (built-in statements and functions: file I/O, `DATA`/`READ`, `SWAP`, `RND`, `SHELL`,
`ENVIRON$`, the `MID$` statement, `_SHL`, `_MIN`, `TAB` and the rest) and step 11 (`REDIM`, dynamic arrays, `OPTION BASE`, `DEFxxx`, string and array
members, whole-`TYPE` assignment).

### Full corpus with `qb64rust` (2026-10-08, after the core built-ins; not a pass criterion)

After the core built-ins, `SELECT CASE` and `ON … GOTO/GOSUB` (`m2-core-builtins` task 8.1, release build, against
the reference clone), 292 programs (`s25`–`s29` new):

| Result | Programs |
|---|---|
| Pass | 208: the 192 of `slice.list` and 16 `.err` programs (the 15 before, and `194_array_param_2d`, now rejected with a real error) |
| Rejected with a diagnostic (exit status 1) | 75 (74 `.output` programs and the compile-only `239_lprint`) |
| `.err` programs that get only "not supported yet" errors | 4 (`16` `TRIM$` read as an implicit array, `28` and `202` `REDIM`, `218` `FRE`) |
| Known failures | 5, as below |
| Compiler crash, or an executable for a rejected program or with wrong output | 0 |

61 programs joined `slice.list` over the change: the five slice programs, `v11_wrap_o2`, `v12_wrap_int64` and 54
of `runtime_comparison`. The proposal's census (58 programs carrying only marks the change removes, not recorded
by name) cannot be matched one by one; every corpus program still rejected that uses one of the change's
constructs is blocked by something outside it: file statements (`176`, `177`, `179`, `213`, `214`, `216`, `219`),
`DATA`/`READ` (`220`, `223`), `ENVIRON$` (`212`), `COMMAND$` (`217`), `_CEIL` (`77`), `EXIT SELECT` (`151`), the
`MID$` statement (`42`).

### Full corpus with `qb64rust` (2026-10-07, after arrays and `TYPE`; not a pass criterion)

After the arrays-and-`TYPE` slice (`m2-arrays-and-types` task 7.2, release build), 285 programs (`s20`–`s22`
new):

| Result | Programs |
|---|---|
| Pass | 144: the 129 of `slice.list` and 15 `.err` programs (the 14 before, and `205_dim_to_same`, now rejected for the old compiler's reason: a second `DIM` of a static array) |
| Rejected with a diagnostic (exit status 1) | 131 (130 `.output` programs and the compile-only `239_lprint`) |
| `.err` programs that get only "not supported yet" errors | 5 (`16`, `28`, `194`, `202`, `218`: `REDIM`, array parameters, `FRE`, `LTRIM$`) |
| Known failures | 5, as below |
| Compiler crash, or an executable for a rejected program or with wrong output | 0 |

The 13 `runtime_comparison` programs that joined `slice.list` (`02_lbound_ubound`, `110_array_assign`,
`121_type_nested`, `199_type_nested_deep`, `204_lbound_ubound_2d`, `255_type_print`, `261_many_dimensions`,
`29_multi_dim`, `30_type_udt`, `43_array_string`, `58_array_udt`, `71_dim_zero_bound`, `72_three_dim`) are 13 of
the 17 the change's proposal counted as blocked only by array or `TYPE` diagnostics; the other 4 need whole-`TYPE`
assignment (`200`), member arrays (`197`), `SHARED a()` and `REDIM` (`206`) or `MID$` (`42`). `30_type_udt`
(`TYPE Point`) joined after the `TYPE`-name measurements `v18_h_type_named_*` (design D3).

### Full corpus with `qb64rust` (2026-10-07, not a pass criterion)

After the control-flow slice (`m2-control-flow-slice` task 8.2, release build), 282 programs:

| Result | Programs |
|---|---|
| Pass | 127: the 113 of `slice.list` and the same 14 `.err` programs as before |
| Rejected with a diagnostic (exit status 1) | 144 (143 `.output` programs and the compile-only `239_lprint`) |
| `.err` programs that get only "not supported yet" errors | 6 (`16`, `28`, `194`, `202`, `205`, `218`: arrays, `REDIM`, `FRE`, `LTRIM$`) |
| Known failures | 5, as below |
| Compiler crash, or an executable for a rejected program or with wrong output | 0 |

The whole run took 4 min 24 s; built programs 2.2 s on average (1.9–5.2 s), rejected ones under 1 s.

Task 8.1 added 38 programs to `slice.list`: `s13`–`s16`, `s18`, `s19` and 32 of `runtime_comparison` (`IF`, `FOR`,
`DO`, `WHILE`, `GOTO`, `GOSUB`, `EXIT`; all also pass with `QB64RUST_NO_FOLD=1`). With the 20 that joined with
tasks 3.1, 3.2 and 4.2, 52 `runtime_comparison` programs joined over the change, against the 53 its proposal
counted on 2026-10-05 as blocked only by its constructs. That count was not recorded by name, so the missing one
cannot be named; likely a program the count took for `EXIT` or `GOTO` (`151_exit_select`, `226_on_goto`). Every
corpus program still rejected is blocked by something outside the change: a built-in function (most of them; e.g.
`LEN`, `VAL`, `STR$`, `MID$`, `SQR`), `SELECT CASE` (6 programs blocked by it alone), `ON … GOTO`, `DEFINT`,
`OPTION BASE`, `EXIT SELECT`, `SWAP`, `PRINT USING`, `TAB`/`SPC`, arrays, `TYPE` and file statements.

### Full corpus with `qb64rust` (2026-10-06, not a pass criterion)

After the constant evaluator (`m2-control-flow-slice` task 4.2), 282 programs:

| Result | Programs |
|---|---|
| Pass | 89: the 75 of `slice.list` and 14 `.err` programs (rejected with at least one error not marked "not supported yet", the tier-2 meaning since `m2-parser-breadth` task 2.1) |
| Rejected with a diagnostic (exit status 1) | 182 (181 `.output` programs and the compile-only `239_lprint`) |
| `.err` programs that get only "not supported yet" errors | 6 |
| Known failures | 5, as below |
| Compiler crash, or an executable for a rejected program or with wrong output | 0 |

The 15 `runtime_comparison` programs that joined `slice.list` with tasks 3.1 and 3.2 use comparisons, logic, `\`,
`MOD` or `^`; the 5 that joined with task 4.2 (`26`, `104`, `207`, `241`, `256`) use `CONST`. They also pass with
`QB64RUST_NO_FOLD=1`.

### Full corpus with `qb64rust` (2026-10-04, not a pass criterion)

One run of the whole corpus (275 programs) after `m2-procedures-and-errors`, to check that the compiler fails
safely outside what it supports:

| Result | Programs |
|---|---|
| Pass | 54: exactly the programs of `slice.list` (none passes outside it) |
| Rejected with a diagnostic (exit status 1) | 196 (195 `.output` programs and the compile-only `239_lprint`) |
| `.err` programs rejected, with the new compiler's message instead of the old text | 20 |
| Known failures | 5: the four `PRINT`-comma programs and `147` compile but are never run |
| Compiler crash, or an executable for a rejected program or with wrong output | 0 |

The rejections are almost all "not supported yet" (statements, operators, arrays, `TYPE`). One wording to fix
when `TYPE` arrives: a `TYPE` block also gives follow-on errors (`` `n` is not a SUB `` for each member line,
"cannot store a string in a number variable" for `v.s = "..."`), 3 programs.

Time per program: rejected ones at most 0.4 s; built ones 4.7 s on average (1.8–23 s; the C++ compile of
`qbx.cpp` and the link, libqb reused; the slow ones vary from run to run). The whole run took 4 min 47 s, against
about 11 min with `qb64pe.exe`. The first run (2026-10-03, slice compiler) passed 31 in 1 min 53 s.
