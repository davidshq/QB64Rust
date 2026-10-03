# 09 — Verification of study claims against the old compiler

Run on 2026-10-02 with `..\QB64pe\qb64pe.exe` (4.7.0-GLFW, llvm-mingw, Windows 11, x64). Programs, compiler output
(`*.compile.txt`) and program output (`*.out.txt`) are in `verification\`. Rerun with `verification\run.sh [name...]`
(Git Bash).

**Runtime-error dialogs:** a program stopped by an untrapped runtime error opens a native message box that someone
must click, even under `$CONSOLE:ONLY`, unless the environment variable `QB64PE_NOPROMPT=y` is set
(`internal\c\libqb\src\error_handle.cpp:295`). Then the runtime writes the error to stderr and ends the program.
`run.sh` sets it, so all programs run unattended (re-recorded 2026-10-02, review session; the only change was
that the dialog's "Continue?" line no longer appears in `v02c`'s output). **A fatal runtime error still exits with
code 0**, so a test runner must detect it from the output, not the exit code.

Legend: **Confirmed** = the study claim holds. **Corrected** = the observed behaviour differs from the claim.
**New** = something not in the study.

## Numeric behaviour (`v01_numeric`)

| Check | Result | Verdict |
|---|---|---|
| `PRINT 1 / 3` | ` .3333333333333333` (16 digits). QB4.5 prints ` .3333333`. `a% / b%` is the same; storing in `s!` or `1! / 3` gives ` .3333333` | Confirmed: int/int is not SINGLE. **Corrected** detail: printed with 16 digits (DOUBLE style), not the ~19 digits a `_FLOAT` value would have |
| `a% = 32767: a% = a% + 1` | `-32768` (wraps, no error 6) | Confirmed |
| `x% = 70000` | `4464` (truncated, no error) | Confirmed |
| `l& = 2147483647: l& = l& + 1` | `-2147483648` | Confirmed |
| `PRINT c% * d%` (200 × 200) | `40000`: the expression is computed in C `int`, not INTEGER; storing it in `e%` gives `-25536` | Confirmed (QB4.5: error 6 on the multiply) |
| `d# = 0.1` | `.1` (exact double 0.1) | Confirmed |
| `d# = 0.1!` | `.1`: **even an explicit `!` suffix** gives the double 0.1. Only via a SINGLE variable (`s! = 0.1: d# = s!`) do you get `.1000000014901161` | Confirmed, and wider than stated |
| `s! = 2.1: IF s! = 2.1` | true; `s! = d#` (d# = 2.1) also true | Confirmed (narrowing to float on both sides) |
| `CINT(0.5), (1.5), (2.5), (3.5), (-2.5)` | `0 2 2 4 -2` | Confirmed: banker's rounding |
| `x% = 2.5`, `x% = 3.5`, `l& = 2.5`, `x% = d#` (2.5) | `2`, `4`, `2`, `2` | Confirmed |
| `CLNG(2.5); CLNG(3.5)`, `_ROUND(2.5); _ROUND(3.5)` | `2 4`, `2 4` | Confirmed |
| DOUBLE narrowed to SINGLE before rounding into INTEGER | `d# = 2.5000001#: x% = d#` → `2` (the float value is 2.5, rounded to even); `l& = d#` → `3` | Confirmed: the INTEGER path rounds the single-precision value |
| `&HFFFF; &H8000; &HFFFF&; &H8000&` | `-1 -32768 65535 32768` | Confirmed |
| `&HFFFFFFFF; &H80000000` | `-1 -2147483648` | Matches QB4.5 |
| `-2147483648` typing | `-2147483648 * 2` = `-4294967296` (real 64-bit), so the literal is typed `&&` | Confirmed (boundary falls through to `&&`) |
| LONG literal arithmetic | **New:** `-2147483647 * 2` = `2` and `2147483647 * 2` = `-2`: two literals of LONG/INTEGER type multiply in 32-bit C `int` and wrap silently, even in a `PRINT` expression. `2147483648 * 2` = `4294967296` | New |
| `2 ^ 3 ^ 2` | `64` (left-associative) | Confirmed |
| `-2 ^ 2` | `-4` | Confirmed |
| `CONST c1 = 2 ^ 3 ^ 2` | `512`: **CONST evaluation is right-associative**, so the same text gives 64 at run time and 512 in a CONST | Confirmed (a real result difference) |
| `CONST c2 = -2 ^ 2` | `-4` | Consistent with run time |

## Errors (`v02_errors`, `v02b_idiv_zero`, `v02c_no_handler`)

| Check | Result | Verdict |
|---|---|---|
| `1 / z!`, `1 / z#`, `1 / 0` (float) | No error; prints `INF`. **New:** the formatting is odd: ` INF           ` for SINGLE and ` INF              D     ` for DOUBLE (padding plus a stray `D`, the exponent letter) | Confirmed (IEEE); new formatting defect |
| `CINT(40000)` with `ON ERROR` | trapped, `ERR` = 6 | Matches QB4.5 |
| `7 MOD z%` (z% = 0) with `ON ERROR` | Not trapped: "Runtime error: Division by zero", program ends | Confirmed: fatal |
| `7 \ z%` (z% = 0) with `ON ERROR` | Same, not trapped | Confirmed: fatal |
| Mechanism | Error 11 comes from `qb_safe_idiv`/`qb_safe_mod` calling `error(11)`, and 11 is in the critical list (`error_handle.cpp`), so it is fatal. Not a SIGFPE as `study\03` §2.10 says; the SIGFPE handler is only a backstop | **Corrected** |
| Untrapped error (`CINT(40000)`, no handler) | Message box "Unhandled error #6 … Line: 3 … Overflow … Continue?"; with `QB64PE_NOPROMPT=y` the same text on stderr, no dialog, exit code 0 | New for the study: the run-time error UI is a native dialog even for console-only programs. `QB64PE_NOPROMPT` (already in `study\05` §2) is the non-interactive mode; the rewrite's runtime must keep an equivalent, ideally with a non-zero exit code |

## Storage (`v03_storage`)

| Check | Result | Verdict |
|---|---|---|
| `DIM f AS STRING * 4` initial bytes | `0 0 0 0` | Confirmed: NUL bytes (QB4.5: spaces) |
| Fixed string inside a TYPE | `0 0 0` | Confirmed |
| `f = "ab"` | `97 98 32 32`: assignment pads with spaces | As expected |
| `REDIM _PRESERVE a(1 TO 2, 1 TO 3)` → `a(1 TO 3, 1 TO 3)` | Rows `11 22 0 / 21 13 0 / 12 23 0`: elements keep their flat (column-major) position, not their coordinates | Confirmed |
| Growing only the last dimension | Rows `11 12 0 / 21 22 0`: coordinates kept | Confirmed |

## Accidental behaviours from `study\02` §9.2 (`v04_accidental`)

| Check | Result | Verdict |
|---|---|---|
| Two `_BIT * 33` scalars | `a = 5: b = -1` → `a` reads `4294967295`. Then `a = 2147483647` → `b` reads `9223372036854775807`. Adjacent variables overlap by 4 bytes and corrupt each other | Confirmed: a real memory-corruption bug |
| Static `sc_N` SELECT temporary in recursion | `f%(1)` returns `1` instead of `0`: the inner call overwrote the outer SELECT's value | Confirmed |
| `ON n GOTO` with n = -1 | error 5 (trappable) | **Corrected:** negative values are checked |
| `ON n GOTO` with n = 256, 257 | falls through, no error (QB4.5: error 5) | Confirmed for n > 255 |

## Probable defects from `study\01` §11.3

| Defect | Test | Result | Verdict |
|---|---|---|---|
| Prepass label stripping (2130–2131) | `v05_label_const`: `lbl1: CONST MyConstant = 42` | Compile error "NULL string; nothing to evaluate" | **Confirmed and reachable**: a label followed by CONST on the same line does not compile |
| | `v05b_label_const_short`: `alongerlabelname: CONST MyConstant = 42, Other = 7` | Compile error "Unexpected element '='" | Confirmed (the error depends on the label length) |
| | `v05c_label_type`: `lbl3: TYPE MyType` | Compiles and runs correctly | The TYPE path is not affected in practice |
| ELSE level check (6627–6638) | `v06_else_level`: `ELSE` while a `FOR` opened inside the IF block is still open | The BASIC front end accepts it; C++ compilation fails (`error: expected expression`) | **Confirmed**: should be a front-end error |
| `symboltype("##")` returns the DOUBLE code (`type.bas:370`) | `v08_float_suffix` | `x##` is a real `_FLOAT` (`LEN` = 32, like `DIM y AS _FLOAT`; `LEN(d#)` = 8). All callers of `symboltype` (`type.bas:734, 753`, `ide_methods.bas:21169`) only test it for zero | Confirmed in the code, **harmless**: no observable effect |
| `nm` dynamic retry opens the static dump (13870–13875) | Not executed: it needs a C++ DLL used with `DECLARE DYNAMIC LIBRARY` | Confirmed by reading: the branch generates `nm_output_file_dynamic$` and then reads `nm_output_file$` | Confirmed by reading only |
| Warning exit code (14228) | `v07_warning` (`$UNSTABLE:HTTP` gives an unconditional warning). With `-x`: exit 0 and the warning is not printed under `-q` | `-c` not executed (it may open windows); the code is `IF (compfailed <> 0 OR warningsissued <> 0) AND NOT ConsoleMode THEN END 1` | `-x` behaviour confirmed; `-c` confirmed by reading only |

## Manual check: CP437 round trip in VS Code (open)

`verification\cp437_all_bytes.bin` holds bytes 0–255 in order; `cp437_all_bytes.sha256` holds its hash
(`40aff2e9…944880`). To check: copy it to a `.bas` name, open it in VS Code with encoding "DOS (CP 437)", type a
character and delete it, save, and compare the hash (`sha256sum`). Watch especially 0x00, 0x0A, 0x0D, 0x1A and 0x7F.
Not done yet; needs a person at VS Code.

## Consequences for the rewrite

- Decide explicitly (panel item 7, bug compatibility) on: CONST `^` associativity; 32-bit wrap of INTEGER/LONG
  arithmetic inside expressions; banker's rounding with single-precision narrowing for INTEGER targets; NUL-filled
  fixed strings; linear `REDIM _PRESERVE`; fatal integer division by zero; 10-column comma zones on the Windows
  console (14 elsewhere; `study\10` §2.2).
- Fix without compatibility concern: `_BIT * n` (n > 32) overlap, static SELECT temporaries in recursion, label +
  CONST prepass bug, ELSE inside an open inner block, `INF` print formatting, the endless `tab()` loop on a
  redirected console and the `CONOUT$` handle leak in `func_pos` (`study\10` §2.2).
- Test infrastructure: set `QB64PE_NOPROMPT=y` for every test run (the legacy runner and `run.sh` do), and treat
  a "Runtime error" in the output as a failure, because the exit code stays 0. Screen `PRINT` with a comma must not
  be used in `$CONSOLE` programs whose output is redirected (it never terminates).

Later checks (session 3) are in `study\10-gaps.md`: `v09_dim` (§1) and `v10_print` (§2).
