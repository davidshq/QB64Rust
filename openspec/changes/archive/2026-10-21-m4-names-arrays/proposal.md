# Proposal

## Why

The third change of M4 (user, 2026-10-20) takes the declaration rules of `ROADMAP.md`'s M4 item "Remaining name,
declaration and array rules" that the demand reports and `tests\verification\known.list` name, and `LEN` of a
whole array. Together they hold back 5 upstream programs: `arrays/t001_whole_array_core`, the last program of
`tests\upstream\deferred.list`, and the four `LEN(a())` programs. They also hold back 7 recordings on
`known.list` and the old compiler's own sources (`qb64pe/element` and `hasFunctionElement` read through the clone,
the M4 target `qb64pe.bas`). The user left two parts for later: a whole array used as a value (`PRINT a()`, `SWAP
x(), y()`) and the old compiler's errors for names that are no array or no SUB.

Found before this proposal, from the demand reports (upstream with `--include-root tests\upstream\root`, and the
clone's `tests\compile_tests\qb64pe`), `qb64pe.bas` and the recordings already in `verification\`:

- **A plain scalar beside an array typed by `DIM … AS`** (`DIM a(3) AS LONG`, then `a = 5`): `sema` marks it
  "not supported yet" because it was not measured whether the array's type types the scalar too
  (`check\decl.rs` `variable`). Three recordings say it does not. `v30_x67` writes a 4-byte SINGLE for `PUT #1,
  1, a` beside `DIM a(1 TO 3) AS INTEGER`. `v18_d_index_none` and `v30_x47` read the plain name as a scalar that
  starts at 0. So the scalar is a variable of its own, of the default type. The reverse case is marked too
  (`check\places.rs`, "an array `a` beside a variable or array typed by `DIM … AS`"). `v24_x126` shows `STATIC
  a() AS DOUBLE` followed by `DIM a(3)` making a second array, a SINGLE one, which the later uses name. Upstream:
  `t001` (`REDIM SHARED T(1 TO 8) AS INTEGER` beside the scalar `t` under `DEFLNG`). Recordings: `v18_d_index_none`,
  `v24_d_erase`, `v30_x47`, `v30_x67`, `v24_x126`. In the old compiler's sources: `DIM argCount AS LONG, args(5)
  AS ParseNum` (`const_eval.bas`).
- **`REDIM name AS type` without parentheses in a SUB or FUNCTION**: marked (`check\decl.rs`, "`REDIM i` without
  bounds in a SUB or FUNCTION"). In the main module it declares a scalar (`v24_x43`). In a procedure, `DIM a()`
  and `REDIM r()` are measured (`v24_x273`), but the form without parentheses is not. The old compiler's own
  sources use it: `ReDim provided(LEN(argString) / 4) As Long, i As Long` in a SUB (upstream `qb64pe/element`,
  `hasFunctionElement`). It is the first line of the upstream report.
- **`TYPE` blocks inside a SUB, FUNCTION or block**: marked (`check\places.rs` `type_block`). The old compiler's
  prepass defines every `TYPE` block as a type of the whole program, wherever it stands (`qb64pe.bas` 2483,
  `lasttype`). Recordings: `v16_m4_type_in_sub`, `v18_h_type_in_sub`. Upstream `mem/mem_checking_off` has one too,
  but it also needs `$CHECKING:OFF`. Three front-end tests and the pipeline spec's follow-on scenarios use this
  mark as their example of an unsupported declaration, so they move to one that stays marked.
- **`LEN` of a whole array, `LEN(a())`, and of a whole member array, `LEN(p(0).a())`**: marked (`check\places.rs`
  "the whole array `a()`", `check\builtins.rs` `len`). The old compiler gives the size in bytes of all the
  elements, by the code that `_MEM(a())` uses (`qb64pe.bas` 20961, `evaluatetotyp` −5, `method2useall`). An array
  of variable-length strings is its error "Cannot pass array of variable-length strings". Upstream:
  `arrays/size`, `arrays/test_mem_vs_len`, `arrays/test_mem_vs_len_classic`, `types/udt_array`.

## What Changes

- **A scalar and an array of one name, one typed by `DIM … AS` and the other not**: two variables. The scalar
  without a suffix or an `AS` clause has the default type, and an array declared without `AS` beside a typed one
  is an array of the default type. Uses name them as the old compiler does; `v48` measures each pairing. A pairing
  `v48` shows to behave in a way the spec does not describe stays "not supported yet".
- **`REDIM name AS type` and `REDIM name` in a SUB or FUNCTION** declare a local scalar, as `DIM name AS type`
  does, with what `v48` measures about a second run and a `STATIC` procedure.
- **`TYPE` blocks anywhere in the program**: in a SUB, a FUNCTION, an `IF` or a loop, each defines a type of the
  whole program, usable above its block and in every procedure. The marked declaration of the pipeline spec's
  follow-on scenarios becomes `TYPE cos` (a `TYPE` named like a built-in, which stays marked).
- **`LEN` of a whole array and of a whole member array**: the size in bytes of all its elements (0 for a dynamic
  array that is not allocated, as `v48` measures). It is a compile error for an array of variable-length strings,
  as in the old compiler.
- **Tests**: a measurement family `v48` in `verification\`, front-end tests, `t001` and the four `LEN` programs
  onto `tests\upstream\pass.list` (`deferred.list` left empty), and the seven recordings above with the `v48`
  programs onto `tests\verification\pass.list`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/arrays-and-types`: "User types" (`TYPE` blocks anywhere) and "Arrays in procedures" (`REDIM name AS
  type` in a procedure) change; new requirements "A scalar and an array of one name", "The plain name between two
  arrays", "An array listed with a type", "LEN of a whole array" and "Arrays LEN does not take" (split so after
  task 1.1's measurements).
- `compiler/pipeline`: "No follow-on errors after an unsupported declaration": the example declaration is `TYPE
  cos` instead of a `TYPE` block inside an `IF` block.

## Impact

- `sema`: `check\decl.rs` (`variable`: the plain scalar beside a typed array; the `REDIM` mark in a procedure),
  `check\places.rs` (the array beside a typed name; `declare_types` walks procedures and blocks, and `type_block`
  no longer marks), `check\builtins.rs` `len` (a whole array and a whole member array); a new `ExprKind` for the
  size of a whole array, or `MemOf`'s size reused (design D4).
- `ir` and `codegen-cpp`: the size expression of a whole array, from `array_bytes` and `element_count`
  (`codegen-cpp\src\decl.rs`), and of a whole member array, from the member's layout.
- Tests: `tests\frontend\unsupported.bas`, `explicit_after_unsupported.bas` and `mem_follow_on.bas` take `TYPE
  cos` as their marked declaration; their snapshots are updated.
- `SOMEDAY.md`: the rows of `LEN` of a whole member array ("Left marked by `m3-mem`") and of `GET #1, , a` beside
  a typed array ("Left marked by `m3-console-files`") close.
- No new dependency, no libqb change.
