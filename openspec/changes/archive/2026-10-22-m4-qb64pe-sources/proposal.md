# Proposal

## Why

`tests\upstream\pass.list` stands at 391 of 404. Of the 13 programs left, 10 are `compile_tests/qb64pe/*`, which
include the old compiler's own sources (`source\utilities\*`, `source\global\constants.bas`) and test its parts.
They are the next step of M4: QB64pe's sources reached through their include files, smallest first (`STATUS.md`,
"Next" item 3). Every one is rejected only with "not supported yet" marks.

What holds each one back was found before this proposal. The demand report (`qb64rust --report unsupported`, build
of `37b4014`) and the diagnostics of `-z` were run in the clone's `tests\compile_tests\qb64pe`:

| Programs | What they need |
|---|---|
| `element`, `element_createElementString`, `element_getStringValue`, `element_getnextelement`, `element_getprevelement`, `hasFunctionElement` | `_INSTRREV` alone |
| `file` | `_READFILE$`, `_WRITEFILE`, the `ASC(s$, i) = n` statement, `base$` as a variable's name |
| `dwd`, `prepass`, `string` | the above, and `STOP`, `exp$` and `key$` as names, `_FULLPATH$`, `_ARCSEC`, `_ARCCSC`, `_ARCCOT`, `_SECH`, `_CSCH`, and `HashListName$(i)` for an array declared `AS STRING * 256` |

The report also names `inclinenumber(…)`, `incname$(…)`, `incIsInternal(…)`, `udtxcanonmode(…)` and other
arrays of `qb64pe.bas` that these programs do not include. Each is marked by the follow-on rule (`check\mod.rs`
`follow_on`: after a marked declaration, a name not declared is not made an implicit array). They are expected to
become implicit arrays once the marked declarations of `key$` and `exp$` compile, as they do in the old compiler.
`-z` stops at 100 errors, so marks behind those may still show up. Task 1 finds them before the code is written.

## What Changes

- **`_INSTRREV([start,] s$, find$)`**: compiled as libqb's `func__instrrev`, in the same way as `INSTR`.
- **`_READFILE$(name$)`, `_WRITEFILE name$, data$` and `_FULLPATH$(path$)`**: compiled as libqb's
  `func__readfile`, `sub__writefile` and `func__fullpath`, with the run-time errors of the old compiler's program.
- **`_ARCSEC`, `_ARCCSC`, `_ARCCOT`, `_SECH`, `_CSCH` at run time**: libqb's `func_arcsec` … `func_csch`, believed
  `_FLOAT` like `_SEC` and `_COT`. The constant evaluator has had them since `m3-constant-evaluation`.
- **The `ASC` statement** `ASC(s$[, i]) = n`: it sets one byte of a string variable, and raises error 5 for an
  empty string or a position out of range, as the old compiler's inline code does (`qb64pe.bas` 8339).
- **`STOP [expression]`**: it ends the program as `SYSTEM` does. The expression is checked and never evaluated
  (`qb64pe.bas` 9905).
- **A reserved word with a suffix as a name** (`key$`, `exp$`, `base$`): it is a variable, parameter or array
  wherever the old compiler accepts it, and "Name already in use" where it rejects it. Today only no suffix and `&`
  are measured, and every other suffix is marked. A new family measures each keyword and built-in with each
  suffix, as `v15_builtin_names` did for no suffix and `&`.
- **`name$(i)` for an array of fixed-length strings**: it refers to the array declared `AS STRING * n`, if the
  family records that the old compiler does so. Today it would be another, implicit array.
- **Tests**: a measurement family `v52` comes first. The 10 programs go onto `tests\upstream\pass.list`. A slice
  program, call-site programs and front-end tests cover each new built-in.

Out of scope: `http/read_example` (a known failure: the site changed), `opengl/test` (`SUB _GL`) and
`const/math_funcs` (D-031). Also out of scope: the rest of `qb64pe.bas` beyond what these 10 programs include.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/builtin-functions`: new requirements "Searching from the end with _INSTRREV" and "The reciprocal and
  hyperbolic functions at run time".
- `language/builtin-statements`: new requirement "ASC statement".
- `language/file-io`: new requirement "Whole files and full paths" (`_READFILE$`, `_WRITEFILE`, `_FULLPATH$`).
- `language/control-flow`: new requirement "STOP".
- `language/procedures`: "Reserved names" changed. A reserved word with a suffix is a name wherever the old
  compiler accepts it, as measured.
- `language/fixed-length-strings`: new requirement "A suffixed name of a fixed-length string array".

## Impact

- `sema`: `builtins.rs` (`SUPPORTED` rows for the functions; the statement table for `_WRITEFILE`),
  `check\decl.rs` `reserved` (the suffix rule from the new measurement), `check\places.rs` (`array_key_ty` /
  `find_array` for the `$` name of a fixed-length string array), `check\mod.rs` (`STOP`; the `ASC` statement).
- `syntax`: the `ASC(…) =` statement, if the parser does not already read it as a store. `STOP` is already parsed
  (`StopStmt`).
- `ir` and `codegen-cpp`: a statement node for `ASC … =`, and `STOP` written as the old compiler writes it.
- Tests: `verification\v52_*`, `tests\corpus\slice` (a new program `s154_…`, recorded with `qb64pe.exe`),
  `tests\callsite` (one program per new built-in form), `tests\frontend`, and the lists
  `tests\upstream\pass.list` and `tests\verification\pass.list` (the 10 programs are on no shrink-only list, being
  accepted programs with marks only).
- No new dependency, and no change to `libqb\`: every runtime entry exists in the copy.
