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
| `slice\` | 19 programs written for the new compiler (numeric rules, PRINT forms, CP437 bytes, procedures, error handling, control flow, operators, constants; `SOURCE.md`) |
| `slice.list` | The 113 programs the new compiler must pass: all 19 of `slice\` and 94 of `runtime_comparison` (see below) |

Each `<name>.bas` has exactly one of:

- `<name>.output`: the program must compile, run, and print this (stdout and stderr merged, byte-exact with CRLF
  line ends as recorded on Windows; `.gitattributes` keeps the bytes);
- `<name>.err`: the compile must fail with this compiler output;
- `<name>.norun`: compile only, never run; the file gives the reason;

and may have `<name>.normalize`: rules (a Python regular expression, a tab, a replacement; `#` lines are comments)
applied to each output line before comparing and before recording, for values that differ between machines or runs,
and `<name>.noprompt`: the value of `QB64PE_NOPROMPT` for the run instead of `y` (`continue` lets a program go on
after an untrapped runtime error; `slice\s12_error_in_print`).

A program that reaches `END` prints an empty line and `Press any key to continue` (no line end) on Windows; that
trailer is part of its `.output`.

## Counts (recorded 2026-10-03; `slice` `s08`–`s12` 2026-10-04, `s13`–`s19` 2026-10-06)

| Kind | `runtime_comparison` | `verification` | `slice` |
|---|---|---|---|
| `.output` | 235 | 2 | 19 |
| `.err` | 20 | 0 | 0 |
| `.norun` | 1 (`239_lprint`: would print a page on the default printer) | 0 | 0 |
| Known failure, no expected file | 5 | 0 | 0 |
| `.normalize` sidecars | 3 (`212` `PATH` length, `234` `TIMER`, `238` a folder name in `cmd`'s error) | 0 | 0 |
| `.noprompt` sidecars | 0 | 0 | 2 (`s12_error_in_print`, `s19_header_errors`: `continue`) |

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
`_INTEGER64` (`CLAUDE.md`, 2026-10-03), so it must match the default-build files.

## The `slice` group and `slice.list` (new compiler)

`slice\` holds programs written for QB64Rust, recorded with `qb64pe.exe` like the rest: `s01`–`s07` by the change
`m2-workspace-and-slice`, `s08`–`s12` (procedures, error handling) by `m2-procedures-and-errors`, `s13`–`s19`
(control flow, operators, constants, header errors) by `m2-control-flow-slice` (`SOURCE.md`).
Each is named after what it pins (`s02_integer_wrap`, `s08_byref`...). `s07_cp437_bytes` has CRLF
line ends and bytes 0x80–0xFF, kept exactly by `.gitattributes`. None uses a `PRINT` comma (see above).

`slice.list` names the 113 programs the new compiler must pass (tier 2, `study\19`), also with
`QB64RUST_NO_FOLD=1`. `s13`–`s19` joined it with `m2-control-flow-slice` (`s17` with task 3.2, the rest with task
8.1).

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
