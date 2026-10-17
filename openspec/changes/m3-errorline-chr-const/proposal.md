# Proposal

## Why

The next step of M3 is the user's choice (`STATUS.md`; user, 2026-10-17: "Do these: the CHR$ function inside a
CONST, _ERRORLINE"): two of the first three lines of the upstream demand report, three programs each. They are
unrelated but small, so they make one change and one close.

| Upstream program | Held back by | After this change |
|---|---|---|
| `font/test` | `CHR$` in a `CONST` | can pass |
| `clipboard/clipboard_test` | `CHR$` in a `CONST`, `_CLIPBOARD$` | held back by `_CLIPBOARD$` |
| `const/math_funcs` | every function of the `CONST` evaluator | held back by the others |
| `environ/set_environ` | `_ERRORLINE` | can pass |
| `http/invalid_url`, `http/statuscode_invalid_handle` | `_ERRORLINE`, `_OPENCLIENT`, `_OPENHOST`, `_STATUSCODE` | held back by the http functions |
| `image/curico_test` | `_ERRORMESSAGE$`, `_INCLERRORLINE`, `_INCLERRORFILE$` | can pass |

`_ERRORLINE`'s three siblings (the line and file of an error in an included file, an error's message) are taken
with it: the same part of libqb, the same measurements, and `curico_test` passes with them.

Measured before this proposal (`verification\v43_*`, 54 programs, compiled and run with `qb64pe.exe`; included
files in `verification\v43_inc\`; the facts go into `study\00` §5 in task 1.1):

- **`CHR$` in a `CONST`** (`const_eval.bas` 1027; `v43_c`, `v43_x01`–`x28`): the code is computed as a
  parenthesised group is, converted to `_UNSIGNED _INTEGER64` with a float rounded half to even (`65.5` is 66,
  `66.5` 66, `-0.5` 0), and must lie in 0 to 255 (`256`, `-1`, `255.5`, `-0.6`, `1E300` and the largest
  `_UNSIGNED _INTEGER64` are errors). Every kind of number, a constant, an expression, `NOT` and parentheses may be
  the code; a string, `CHR$` itself, no code, two codes, no parentheses, an integer suffix on the constant, and
  the string mixed with a number, compared, multiplied or negated are errors. The constants of the auto-included
  file made with `CHR$` (69 of them: `_STR_CRLF`, `_CHR_NUL` … `_CHR_TILDE`) take the same path.
- **`_ERRORLINE`** (`v43_a`, `v43_x29`–`x44`) is the line of the main file where the last error was raised: 0
  before any; kept after `RESUME`; in a SUB, a FUNCTION or a GOSUB the line of the statement that raised it.
  **In an included file it is the main file's line of the `$INCLUDE` that brought the outermost included file
  in**, also for a file that file includes and for a SUB of an included file called from main (`v43_b`). libqb
  takes it from the first argument of `evnt`, which this compiler writes as the line in the included file.
- **A line joined to the next by a continuation (`_`) is numbered as the last line of the join**, for every
  statement on it, before or after the underscore, for `_ERRORLINE` and for the message of an error nothing traps
  (`v43_d`, `v43_e`). This compiler numbers a statement by its first token, so its message of an untrapped error
  on such a line names another line today, as `_ERRORLINE` would.
- **`_INCLERRORLINE`** is the line in the innermost included file (0 in the main file), **`_INCLERRORFILE$`** that
  file's name without its folder (empty in the main file); **`_ERRORMESSAGE$[(code)]`** the message of the code,
  of the last error without one, the code a LONG slot rounded half to even (`9.6` is 10).
- Rejected: an argument or empty parentheses where there is no slot, `_ERRORLINE` as a statement or assigned to,
  in a `CONST`, with a suffix; a string or two arguments to `_ERRORMESSAGE$` (`x29`–`x42`). `$NOPREFIX` is
  rejected whole by this version of QB64pe (`x32`).

## What Changes

- **`CHR$(code)` in a `CONST`** is computed as the old evaluator computes it, with its errors; the constants of
  the auto-included file made with it compile. The other functions of the evaluator stay "not supported yet".
- **`_ERRORLINE`, `_INCLERRORLINE`, `_INCLERRORFILE$` and `_ERRORMESSAGE$`** compile as plain built-ins.
- **The event check of a statement in an included file** passes the main file's line of the outermost
  `$INCLUDE` as the old compiler does (`sema` and the IR carry it beside the line).
- **A statement on a line joined by a continuation** is numbered as the last line of the join, as the old
  compiler numbers it (its untrapped-error message and `_ERRORLINE`).
- **A built-in function's name in a `CONST`** is an error, not "not supported yet" (`v43_x40`).
- **Tests**: slice programs `s135_chr_const`, `s136_errorline`, `s137_errorline_include` (included files in its
  `.files` folder, which tier 1 now reads includes from); front-end tests of the errors, the types and the C++ of
  the event checks; seven call-site programs; three upstream programs added to `tests\upstream\pass.list`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/constants`: `CHR$` in a `CONST`; the auto-included constants made with it.
- `language/error-handling`: the line and file of an error, and the numbering of joined lines.
- `compiler/pipeline`: the scenarios of "No follow-on errors after an unsupported declaration" used `CHR$` in a
  `CONST` as their example; they use `EXP`.

## Impact

- Changed: `crates\sema` (`consteval.rs` `chr`; `check\constants.rs`, `check\auto.rs`; `check\mod.rs` the line of
  a joined line and the main file's line of an included statement; `builtins.rs` four rows), `crates\ir`
  (`include_line` on statements and labels, lowered and dumped), `crates\codegen-cpp` (`evnt_args`); the test
  `crates\driver\tests\inputs.rs` (includes from a `.files` folder); `tests\frontend` (new tests, and three whose
  "not supported yet" example was `CHR$` in a `CONST`); `tests\known_unsupported_rejections.list` (one snippet
  out); `crates\base\src\old_wording.tsv` regenerated.
- No change to libqb, the Makefile or the reference clone. `DIVERGENCES.md`: none.
- Not in this change: the other functions of the `CONST` evaluator (`const/math_funcs`); `_CLIPBOARD$`; the http
  functions; numeric line labels (`v43_a` has two; its slice copy has none); a constant with a type suffix inside
  a `CONST` (`v43_x20`, already on the demand report).
- `STATUS.md`, `ROADMAP.md`, `study\00` §5, `verification\README.md` (the `v43` family), `DECISIONS.md` updated.
