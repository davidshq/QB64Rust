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
| `slice\` | 7 programs written for the new compiler's first slice (numeric rules, PRINT forms, CP437 bytes; `SOURCE.md`) |
| `slice.list` | The 14 programs the slice must pass: the 7 of `slice\` and 7 of `runtime_comparison` (see below) |

Each `<name>.bas` has exactly one of:

- `<name>.output`: the program must compile, run, and print this (stdout and stderr merged, byte-exact with CRLF
  line ends as recorded on Windows; `.gitattributes` keeps the bytes);
- `<name>.err`: the compile must fail with this compiler output;
- `<name>.norun`: compile only, never run; the file gives the reason;

and may have `<name>.normalize`: rules (a Python regular expression, a tab, a replacement; `#` lines are comments)
applied to each output line before comparing and before recording, for values that differ between machines or runs.

A program that reaches `END` prints an empty line and `Press any key to continue` (no line end) on Windows; that
trailer is part of its `.output`.

## Counts (recorded 2026-10-03)

| Kind | `runtime_comparison` | `verification` | `slice` |
|---|---|---|---|
| `.output` | 235 | 2 | 7 |
| `.err` | 20 | 0 | 0 |
| `.norun` | 1 (`239_lprint`: would print a page on the default printer) | 0 | 0 |
| Known failure, no expected file | 5 | 0 | 0 |
| `.normalize` sidecars | 3 (`212` `PATH` length, `234` `TIMER`, `238` a folder name in `cmd`'s error) | 0 | 0 |

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
there with no arguments and `QB64PE_NOPROMPT=y`; details and options in `tools\legacy_tests\README.md`. Known
failures without an expected file are compiled but not run. A full run takes about 11 minutes with the old
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

Measured 2026-10-03; every `runtime_comparison` program gives the same output in both builds. That group has no
`_INTEGER64` program and no LONG overflow, which is why `v12` was added (`study\16` §8). Both differences come from
signed overflow being undefined in the generated C++ (no `-fwrapv`); the new compiler wraps for both LONG and
`_INTEGER64` (`CLAUDE.md`, 2026-10-03), so it must match the default-build files.

## The `slice` group and `slice.list` (new compiler)

`slice\` holds programs written for QB64Rust by the change `m2-workspace-and-slice`, recorded with `qb64pe.exe`
like the rest; each is named after what it pins (`s02_integer_wrap`, `s05_instr`...). `s07_cp437_bytes` has CRLF
line ends and bytes 0x80–0xFF, kept exactly by `.gitattributes`. None uses a `PRINT` comma (see above).

`slice.list` names the programs the new compiler must pass (tier 2, `study\19`):

```
cargo build --release
python tools\legacy_tests\run_legacy_tests.py --suite corpus --qb64 target\release\qb64rust.exe --list tests\corpus\slice.list
```

`cargo test` (tier 1) runs the front end over every corpus program and requires no diagnostics for the listed
ones. Add a program to the list when the compiler supports everything it uses.

### Full corpus with `qb64rust` (2026-10-03, not a pass criterion)

One run of the whole corpus with the slice compiler, to check that it fails safely outside the slice:

| Result | Programs |
|---|---|
| Pass | 31: the 14 of `slice.list`, and 17 others that use only slice features (`125`, `130`, `132`, `133`, `144`, `145`, `163`, `188`, `208`–`211`, `27`, `74`, `75`, `87`, `92`) |
| Rejected with a diagnostic (exit status 1) | 214 |
| `.err` programs rejected, with the new compiler's message instead of the old text | 20 |
| Known failures | 5 (the four `PRINT`-comma programs compile but are never run; `147` is rejected) |
| Compiler crash, or an executable with wrong output | 0 |

Time per program: rejected ones at most 0.1 s; built ones 2.8 s on average (1.9–11 s; the C++ compile of
`qbx.cpp` and the link, libqb reused). The whole run took 1 min 53 s, against about 11 min with `qb64pe.exe`;
input for the runner's parallel option (`study\19` §5).
