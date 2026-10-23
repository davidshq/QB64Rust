# Proposal

## Why

This is step 3 of the six-step plan for the rest of M4 (user, `DECISIONS.md` 2026-10-22): **memory and record
forms**. `tests\verification\known.list` holds 69 programs marked "not supported yet". The demand report over them
(`qb64rust --report unsupported`, 2026-10-23) puts 30 under this step. They fall into three groups:

- **Addresses and conventional memory.** The old compiler accepts `VARPTR` and `VARSEG` of a scalar, a string, a
  fixed-length string, a member, and an array of a `TYPE`, of strings or of fixed-length strings, and `VARSEG` of a
  `TYPE` variable (`VARPTR` of one is its internal error, measured in task 1.1).
  It also accepts `SADD(t$)` and `VARPTR$(v)` (`v28_x01_accepted`), and `VARPTR` or `VARSEG` as the direct argument
  of `HEX$`, `OCT$` or `_BIN$` (`v28_x95`). We mark all of these. Only arrays of numbers were built
  (decided 2026-10-13: "arrays only until a program needs one"; this step is that need).
- **The array's place of a block statement.** Graphics `GET` and `PUT` take a plain variable, an undeclared name,
  or an array of strings, of fixed-length strings or of a `TYPE` (`v16_m5`, `v28_x01`, `x12`, `x13`, `x20`, `x23`,
  `x59`). File `GET` and `PUT` take an undeclared array `und()` or `und$()` (`v30_x176`–`x178`). `_UCHARPOS` takes
  an undeclared array `nd&()` (`v31_x01`). We mark all of these.
- **Records (`TYPE`s).** These are a member array's bound that names a `CONST`, `_PI` or a function of the
  constant evaluator (`v24_x196`, `v51_x09`–`x11`, `x14`), a `TYPE` member as a `FOR` variable (`v17_c`), and a
  member array of an array element beside other targets of `ERASE` or `REDIM` (`v24_x312`). They also cover `LEN`
  of a whole member array of variable-length strings (`v48_x30`) and `_MEM` of an array of a `TYPE` with a
  `STRING` member (`v48_x11`).

Some forms nearby are rejected by the old compiler, and we mark them instead of giving its error: `SADD` of a
fixed-length string or a literal, `VARPTR$` of a number array (`v28_x55`–`x57`), `STRING * n` with a variable as
`n` (`v51_x75`), and the name `fs$5` of a FUNCTION reused by a SUB, a `CONST` or a `TYPE` member
(`v21_x54`, `x55`, `x57`).

Each of these was left because its text in the old compiler's C++ differs from the forms already built. Most
need one thing that does not exist yet: **a variable in conventional memory** (libqb's `dblock`). This change
measures each form first, then builds the accepted forms and makes each rejected form the old compiler's error.

## What Changes

- **A variable named by `VARPTR`, `VARSEG`, `SADD` or `VARPTR$` lives in conventional memory**, as in the old
  compiler. This covers a scalar of the main module, a string (a descriptor in conventional memory), a
  fixed-length string, and a `TYPE` variable. `VARPTR` of each SHALL give the offset the old compiler's program
  gives, `VARSEG` 80 (or the array's segment). `PEEK` and `POKE` through them reach the variable's bytes.
- **`VARPTR` and `VARSEG` of an element of an array of a `TYPE`, of strings or of fixed-length strings, and of a
  member** give what the old compiler's program gives.
- **`SADD(t$)`** gives the offset of the string's bytes. **`VARPTR$(v)`** gives the three-byte string of
  `func_varptr_helper`. `SADD` of a fixed-length string or a literal, and `VARPTR$` of an element of a number
  array, are the old compiler's errors.
- **`HEX$`, `OCT$` and `_BIN$` of `VARPTR`, `VARSEG` or `_SCREENEXISTS`** are passed the width the old compiler
  passes (0).
- **Graphics `GET` and `PUT`** take a plain variable (also one never declared), and an array of strings, of
  fixed-length strings or of a `TYPE`, whole or from an element. Each is passed the bytes the old compiler
  passes. A variable that is too small raises the old compiler's error 5 (`v16_m5`).
- **File `GET` and `PUT` of an undeclared array** (`und()`, `und$()`), and **`_UCHARPOS` of an undeclared array**
  (`nd&()`), do what the old compiler's program does.
- **A member array's bound may name a `CONST` above the `TYPE` block**, a constant of the auto-included file or a
  function of the constant evaluator. The type is laid out once those constants are known.
- **A `TYPE` member as a `FOR` variable** (`FOR r.v = 1 TO 2`) runs as in the old compiler.
- **A member array of an array element beside other targets of one `ERASE` or `REDIM`** runs each clear as the
  old compiler does, the other targets going on when one target's index raises an error (`v24_x312` lines 9, 10).
- **`LEN` of a whole member array of variable-length strings** gives the old compiler's number, and **`_MEM` of a
  value of a `TYPE` with a `STRING` member** gives the old compiler's block.
- **The rejected forms above become compile errors**, each with the old compiler's wording as a note: "SADD only
  accepts variable-length string variables", "VARPTR$ only accepts variable-length string arrays", "Number/Constant
  expected after *", "Type symbols after a SUB name are invalid", "Invalid constant type", "Invalid name".
- The edges of each rule are measured first, in a new family `verification\v56_*`. That covers the offsets
  `VARPTR` gives in each kind of declaration and order, what `VARPTR$` and `SADD` give, and graphics `GET` and
  `PUT` of each new kind at run time. A form the family does not settle stays marked.
- Every program of `tests\verification\known.list` that then passes moves to `pass.list`.

Out of this change: `VARPTR` and `VARSEG` of a variable, array or parameter of a SUB or FUNCTION (another
allocation per call, `SOMEDAY.md`), `VARPTR$` in a `PLAY` or `DRAW` string, and an index that calls a FUNCTION
in the array of `GET` or `PUT` (the old compiler writes the index twice). Each keeps its mark and its row.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/conventional-memory`: "What VARPTR and VARSEG accept" (scalars, strings, `TYPE` values, members, arrays
  of every kind of the main module; `SADD` and `VARPTR$`, and their rejected forms); a new requirement for
  variables in conventional memory; a new requirement for `VARPTR` in `HEX$`, `OCT$`, `_BIN$`.
- `language/screen-output`: "Forms of graphics GET and PUT" (a plain variable, an undeclared name, arrays of
  strings, fixed-length strings and `TYPE`s).
- `language/record-files`: "The forms of GET and PUT on a file" (an undeclared array).
- `language/fonts`: "_UCHARPOS takes a LONG array" (an undeclared array).
- `language/arrays-and-types`: "Member arrays" (a bound naming a constant; a member array of an element in a
  list of `ERASE` or `REDIM`), "Member array bounds that name no constant", "LEN of a whole array" (a whole member
  array of strings).
- `language/control-flow`: "FOR loops" (a `TYPE` member as the variable).
- `language/mem-blocks`: "A block of a reference" (a value of a `TYPE` with a `STRING` member).
- `language/fixed-length-strings`: "Fixed-length string declarations" (`STRING * n` with a variable).
- `language/procedures`: the requirement for `FUNCTION f$n` (its name reused by a SUB, a `CONST`, a member).

## Impact

- `crates\sema`: `check\places.rs` (`var_address`, `member_bounds`, `clear_member_array`), `check\builtins.rs`
  (`block_arg`, `bytes_arg`, `long_array_arg`, the radix arguments, `LEN` of a whole member array),
  `check\mem.rs`, `check\blocks.rs` (`for_var`), `check\decl.rs` (`split_name`, the length of `STRING * n`),
  `check\expr.rs` (`SADD`, `VARPTR$`); `lib.rs` (`Var::conventional` for any variable, new `ExprKind`s).
- `crates\ir` and `crates\codegen-cpp`: variables in `dblock` (`decl.rs`, the data fragment), the address
  expressions (`value.rs`), the `byte_element` of graphics `GET`/`PUT` for each new kind, a `FOR` over a member.
- Tests: the family `verification\v56_*`; front-end tests for each form and error; call-site programs for
  `SADD`, `VARPTR$` and the new `GET`/`PUT` texts; the wording table regenerated; `tests\verification\known.list`
  shrinks by about 30 programs.
- `SOMEDAY.md`: rows 22, 23, 60, 62–65, 93, 109, 214, 235 closed or turned into rows of "QB64pe behaviours to
  review". `DECISIONS.md`: a row for the change, which replaces "arrays only" of 2026-10-13.
