# Proposal

## Why

This is step 2 of the six-step plan for the rest of M4 (user, `DECISIONS.md` 2026-10-22; design D6 of
`m4-line-numbers-whole-arrays`): **names and implicit arrays**. `tests\verification\known.list` holds 97
programs marked "not supported yet". The demand report over them (`qb64rust --report unsupported`, 2026-10-22)
puts about 30 under this step. They fall into two groups:

- **Programs the old compiler compiles and runs, which we mark.** These cover an array used without `DIM` where
  its allocation is not the first thing its statement does: the line of a block (`v24_x262`), a later item of
  `PRINT`, `READ`, `WRITE` or `INPUT` (`x264`, `x265`), and a bound of `REDIM` (`x266`, `x268`). They also cover
  `STATIC v(2 + 3)` (`x280`), `STATIC a(5)` beside a shared array (`x272`), and `LEN(x())` of a name that is not
  an array (`v48_x04`, `x14`). The group ends with names written with blanks around `.` in a declaration
  (`DIM a . b`, `SUB a . b`, `v47_x17`, `x18`) and an array named `WIDTH` (`v27_x51`).
- **Programs the old compiler rejects, where we give a "not supported yet" mark instead of an error.** These
  are a built-in function's name where a variable is expected (`rnd(1) = 2`, `TAB(5) = 1`, `READ ABS(a)`), and
  a name with one leading underscore and parentheses that calls nothing (`_foo(1)`, `_TRUE(1)`, `_CRC32$(…)`,
  `_CAST&(…)`, `_OPENCLIENT&(…)`). Then a built-in statement written with a suffix (`KILL$`, `_ASSERT$`), a
  label named like a built-in statement (`GOTO BEEP`, `CLS`, `SOUND`), and a name used as a statement that is
  no SUB (`IF c THEN jumped`, `LEN("a")`). The last two are a store to a FUNCTION's name (`f(3) = 5`) and a
  FUNCTION named like a built-in with a suffix it does not take (`FUNCTION exp$`). The old compiler's errors for
  these were left for later on 2026-10-20 (`DECISIONS.md`). Two snippets of tier 1's shrink-only list
  `tests\known_unsupported_rejections.list` are of this kind too (`CALL UndefinedSub`, `_IIF$(…)`).

Each mark was put there because the edges of the rule had not been measured: which built-ins a name may not be,
and where the old compiler writes an allocation. This change measures those edges first, then builds the
accepted forms and makes each rejected form a real error with the old compiler's wording.

## What Changes

- **An array used without `DIM` is allocated where the old compiler allocates it**, in every place that is marked
  today:
  - **The line of a block**: before an `IF`, `FOR`, `WHILE`, `DO WHILE` and `SELECT CASE`; inside the block for
    `ELSEIF`, `LOOP UNTIL` and `CASE`, as measured.
  - **A later item of `PRINT` or `READ`, any item of `WRITE` or `PRINT #`, a target of `INPUT` or `LINE INPUT`**:
    after the items before it, so that the allocation's error 10 on a second run comes after them.
  - **A bound of `REDIM`**: the array is dynamic wherever it stands and is allocated in the `REDIM`'s own way.
- **`STATIC name(bounds)` with a bound whose value is known when compiling** (`2 + 3`) compiles. A bound that
  names a variable is a compile error where the old compiler's C++ build fails (`v24_x121`).
- **`STATIC a(n)` beside a shared main-module array of that name and type** does what the old compiler does: its
  allocation at the start of the program allocates the shared array again, and the program ends with "Duplicate
  definition" (`v24_x272`).
- **`x()` of a name that is not an array** is an implicit array of one dimension. `LEN(x())` gives its size, as
  in the old compiler (44 for eleven `SINGLE` elements).
- **A declared name with blanks around `.`** (`DIM a . b AS LONG`, `SUB a . b`) is the name `a.b`, as at a use.
- **`WIDTH` as an array**: `WIDTH (80) = 80` and `PRINT WIDTH(1)` use an implicit array named `WIDTH`.
- **The forms the old compiler rejects become compile errors**, not marks, each with the old compiler's wording
  as a note:
  - "Expected variable" and "Expected variable =" for a built-in function's name or a FUNCTION's name where a
    variable is expected;
  - "Invalid array name" for a name with one leading underscore that calls no built-in function with that
    spelling;
  - "Syntax error" for a built-in statement with a suffix it does not take, a name used as a statement that is
    no SUB, and a function used as a statement;
  - "Invalid label!" for a label named like a built-in statement;
  - "Expected CALL sub-name" for `CALL` of a name that is no SUB;
  - "Name already in use" for a procedure named like a built-in with a suffix it does not take.

  The edges of each rule (which built-ins, which suffixes) are measured in a new family `verification\v55_*`
  before the code. A form the family does not settle stays marked.
- Every program of `tests\verification\known.list` that then passes moves to `pass.list`. The two snippets leave
  `tests\known_unsupported_rejections.list`. `v39_a_handles` and `v45_sample` no longer have a mark of any kind
  and get their right line.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/arrays-and-types`: "Implicit arrays" (a whole `x()` of a name that is not an array, `WIDTH`, the
  rejected names as errors); "Arrays in procedures" (`STATIC` with a bound whose value is known when compiling,
  `STATIC` beside a shared array); a new requirement for where an implicit array is allocated within a statement
  and a block's line; a new requirement for declared names with blanks around `.`.
- `language/procedures`: "Reserved names" (a procedure named like a built-in with a suffix is the old compiler's
  error, not a mark); a new requirement for a statement that names no SUB.
- `language/builtin-functions`: a new requirement for names that call no built-in (one leading underscore, a
  function's name where a variable is expected, a function as a statement).
- `language/builtin-statements`: a new requirement for a built-in statement written with a suffix.
- `language/control-flow`: a new requirement for a label named like a built-in statement.

## Impact

- `crates\sema`: `check\places.rs` (`implicit_array`, `not_an_array`, `static_array_free`, the declaration of
  `STATIC` arrays), `check\mod.rs` (`NoJoin`, `unjoined`), `check\blocks.rs`, `check\io.rs` (the items),
  `check\proc.rs` (statements that name no SUB, calls with blanks around `.`), `check\flow.rs` (labels),
  `check\builtins.rs` (`WIDTH`, a statement's suffix), `lib.rs` (`StmtKind::Allocate`, the parts of a block).
- `crates\syntax`: `parser\decl.rs` and `parser\proc.rs` (declared names with blanks around `.`).
- `crates\ir` (`lower.rs`: where an allocation joins its statement; the item ops split at an allocation) and
  `crates\codegen-cpp` (a `STATIC` array's bounds at the start, `WRITE`'s separator before an allocation).
- Tests: the family `verification\v55_*`; front-end tests for each new error and placement; the wording table
  regenerated (`tools\diagnostics\old_wording.py`); `tests\verification\known.list` shrinks by about 30 programs.
- `SOMEDAY.md`: rows closed for the forms built, and rows added to "QB64pe behaviours to review" for the
  accidents kept (the allocation's error 10 inside a statement, the `STATIC` allocation of a shared array).
  `DECISIONS.md`: a row for the change.
