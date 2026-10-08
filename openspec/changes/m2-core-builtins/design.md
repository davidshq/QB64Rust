# Design

## Context

See `proposal.md` for motivation and the spec deltas for requirements. Background: how the old compiler handles
built-in calls (`study\02` §4.3 optional arguments and the `passed` mask, §5.5 the functions special-cased in
`evaluatefunc`, §9 items 16 and 19), `SELECT CASE` and `ON … GOTO` code (`study\02` §6, "SELECT CASE" and
`xongotogosub`), the order of work (`study\26` §6, §8). The designs of the earlier changes still hold, in particular
D8 of `m2-control-flow-slice` (flat bodies, explicit jumps, the pending-error rule) and D5, D6, D10 of
`m2-arrays-and-types` (places, store rules, the IR shares `sema`'s value tree).

Where the code stands (2026-10-07):
- The built-in table (`crates\builtins`, 455 entries from `tools\builtins\builtins.json`) gives per entry a name,
  kind, `callname`, decoded slot types (`LONG`, `STRING`, `_FLOAT`, `any-numeric`, ...), the optional mask, the
  return type and the required suffix. **Many core functions have an empty `callname`** (`LEN`, `VAL`, `ABS`, `INT`,
  `FIX`, `EXP`, `HEX$`, `OCT$`, `_BIN$`, `CINT`, `CLNG`, `CSNG`, `CDBL`, `_ROUND`): the old compiler emits them
  itself in `evaluatefunc`. Some table facts are known to be wrong (`CLNG` returns INTEGER in the table; `ASC` has
  one slot but takes two; `LBOUND`/`UBOUND` are LONG in the table but `_INTEGER64` in fact).
- `sema` checks `INSTR` and `CHR$` by hand (`check\expr.rs` `instr`, `chr`), and `LBOUND`/`UBOUND` as their own
  node (`ExprKind::Bound`); every other built-in function is marked. The typed tree's `ExprKind::Call { builtin,
  args }` has one slot per table argument, each already converted; the emitter's `call` writes `callname(args,
  mask)` generically.
- The parser gives `SelectBlock` (header with `CASE` or `EVERYCASE` and the selector; `CaseClause`s with
  `CaseHeader` items) and `OnJumpStmt`; `sema` marks both (`check\blocks.rs`, `check\mod.rs`).
- The IR has `Jump`, `Branch { cond, when, to, on_error }`, `Gosub`, `Return`, `AssignAll`, and storage classes
  `Global`, `Static`, `Local`, `Param`, `Result`, `Temp` (a lowering temporary: once in main, per call in a
  procedure).
- `codegen-cpp\src\lib.rs` is 1,324 lines: fragments, names, declarations, procedures, statements, places and
  values in one `Emitter`.

## Goals / Non-Goals

**Goals:**
- One table-driven path for every built-in function `sema` supports; adding one of the remaining plain built-ins
  (step 9) is a table row plus tests, not new checking code.
- Every typing rule, argument conversion, edge value and error number measured with `qb64pe.exe` before it is
  coded; nothing taken from the table or the old source on trust.
- `SELECT CASE`, `SELECT EVERYCASE`, `ON … GOTO/GOSUB` lowered to the existing IR operations under the existing
  pending-error rule.
- The corpus programs blocked only by these constructs pass end to end.

**Non-Goals:**
- Built-ins in `CONST` expressions (the evaluator's functions stay marked).
- Constant folding of built-in calls (the emitter calls libqb; folding is a later optimisation with its own risk).
- The type table and unsigned types (step 8); an unsigned or `_BIT` argument stays marked.
- Matching the old compiler's C++ text beyond what results depend on.

## Decisions

### D1. Measure first
Before code, `verification\v20_*` programs (run with `verification\run.sh`), findings in `study\00` §5. Each program
prints what it measures; error cases trap with a handler that prints `ERR` and does `RESUME NEXT`. The questions:

- **Slot conversion, once per slot type** (`study\26` §8): a float to a `LONG` slot (`LEFT$("abcdef", 2.5)`, `3.5`,
  `-0.5`), a value beyond LONG (`LEFT$(s, 3E9)`, `SPACE$(2147483648#)`: overflow error, wrap, or clamp), an
  `_INTEGER64` to a `LONG` slot, an integer to a `_FLOAT` slot (`SQR(2&)`), a value to an `any-numeric` slot
  (passed in its own type?).
- **Result types**, printed so the type shows (`PRINT f(x) / 3`, a large value, `LEN(STR$(f(x)))`): `ABS`, `INT`,
  `FIX`, `SGN` for each slice type; `SIN`/`COS`/`TAN`/`ATN`/`SQR`/`LOG`/`EXP` for each argument type; `VAL`,
  typed `VAL` for each slice type; `CINT`, `CLNG` (the table says INTEGER), `CSNG` of an integer (`study\02` §5.5:
  typed SINGLE but not narrowed), `CDBL`, `_ROUND`; `_PI`, `_PI(x)`, `_ATAN2`, `_HYPOT`; `ASC`, `LEN`, `INSTR`.
- **String edge values**: `LEFT$`/`RIGHT$` with 0, past the end, negative; `MID$` start 0, negative, past the end,
  length 0, negative, past the end; `ASC("")`, `ASC(s, 0)`, `ASC(s, past end)`; `STRING$` with a string, a code,
  0, negative, a code outside 0–255; `SPACE$` negative; `STR$` of each type, of `-0`, large and small floats;
  `VAL` of `"&H"`, `"&O"`, `"&B"`, `" 1 2"`, `"1e3"`, `"1d3"`, junk, overflow; `HEX$`/`OCT$`/`_BIN$` of each type,
  of negative values, of floats; `_TOSTR$` with and without digits; `LTRIM$`/`RTRIM$`/`_TRIM$` with tabs and NUL;
  `UCASE$`/`LCASE$` above 127.
- **Math edge values**: `SQR(-1)`, `LOG(0)`, `LOG(-1)`, `EXP(1000)` (overflow?), `CINT`/`CLNG` at and past their
  limits and at .5, `INT`/`FIX` of negative halves, `ABS` of the smallest INTEGER/LONG/`_INTEGER64`, `SGN` of 0 and
  -0, `_ROUND` of halves.
- **`LEN`**: of a string expression, of a numeric variable of each type, of a user-type variable, of an element, of
  a member, of a number literal and a numeric expression (accepted or rejected?), of a `STRING` array element.
- **Rejections**: the wrong number of arguments, a string where a number is needed and the reverse, `ASC` with
  three arguments, a built-in written with the wrong suffix (`LEN$`, `LEFT%`), a built-in used as a statement.
- **`SELECT CASE`**: the selector read once (a FUNCTION with a side effect) or at each test (a plain variable, a
  `SHARED` one changed by a FUNCTION in a `CASE` item; an element; a member); a float `CASE` value against an
  integer selector (rounded?), an integer selector against a float range; string selectors with `TO` and `IS`;
  `CASE IS` with each operator; a `CASE` with no matching item and no `ELSE`; `EVERYCASE` with and without `ELSE`;
  an error in the selector and in a `CASE` item under `RESUME NEXT` (which body runs, where the error is serviced);
  a recursive FUNCTION across a `SELECT` (Q-002, to pin the old behaviour); `EXIT` and `GOTO` out of a `CASE`
  body; a label inside a `CASE` body jumped to from outside; a `SELECT` with no `CASE`.
- **`ON … GOTO/GOSUB`**: n = 0, 1, the count, count + 1, 255, 256, 258 (Q-001), -1; a float n (rounded or
  truncated: `study\02` says `int32` storage); an `_INTEGER64` n beyond LONG; a string n (compile error); an error in
  n under `RESUME NEXT`; `ON … GOSUB` in a SUB; `RETURN` after it.

Anything measured as wrong code in the old compiler is put to the user before it is designed (as D-004 was).

### D2. One built-in checker, driven by the table plus a closed rule set
`sema` gains a module `builtins.rs` with the list of supported built-in functions. Each supported function is a
row `(table name, Rule)`; `Rule` is an enum with one variant per **kind of special-casing** in `evaluatefunc`
(not per function), each with an exhaustive `match` (the no-wildcard lint):

```text
Rule = Plain                    slots, optional mask and result from the table (LTRIM$, LEFT$, SPACE$, INSTR, …)
     | ResultOfArg              result has the argument's type (ABS, INT, FIX)
     | FloatByArg               result SINGLE / DOUBLE / _FLOAT by the argument's type (SIN … EXP)
     | Fixed(Ty)                result type overrides the table (CLNG → LONG, LBOUND → _INTEGER64)
     | Len                      any string expression or a variable/element/member (size), see D4
     | Val                      one string, an optional type name (D4)
     | Radix(base)              HEX$/OCT$/_BIN$: width from the argument's type (D4)
     | Asc                      one or two arguments, against the table's one slot
```

The checker: find the row by name (the required suffix as today, `is_builtin_function`), check the arity against
the table's slots and optional mask (or the rule's override), check each argument's kind (string or number), convert
it to the slot's type by the measured rule for that slot type (D3), then type the result by the rule. `INSTR` and
`CHR$` move onto it (their snapshots must not change), `LBOUND`/`UBOUND` keep their node (`ExprKind::Bound`, the
first argument is an array) but take their row's arity and type from the same list, so the coverage check of the
testing delta sees one list.

The table is corrected only by rules, never by editing `builtins.json` (it is generated from the old source and is
evidence; a wrong fact stays visible next to its override). Each override cites the `v20_*` program that measured
it.

*Alternatives:* (a) a hand-written function per built-in, as today: rejected by `study\26` §8, 45 near-copies; (b)
fully data-driven from the JSON with no rules: impossible, a third of the core set has no `callname` and the table
is wrong in places; (c) a per-function enum (`Fn::Len`, `Fn::Abs`, …): exhaustive but 45 arms in every match, and
step 9 would add 200 more; rules group them by behaviour and stay small.

### D3. Argument conversion per slot type
Measured once per decoded slot type (D1), then one function per slot type: `LONG` and `INTEGER` slots as `store`
does today for a variable of that type (provisional until 2.1: a float rounded half to even; beyond the range as
measured); `_FLOAT`, `DOUBLE`, `SINGLE` slots by `convert_exact`; `any-numeric` not converted (the rule decides what
the emitter does with the type); `STRING` passed as it is. The typed tree's `Call` keeps one slot per table argument
with each present argument converted, so the IR and the emitter see no difference between a plain and a special
function except the rule.

### D4. The special rules
Provisional until 2.1; each is corrected there against the measurements.
- **`LEN`**: a string expression gives `((int32)(e)->len)`; a numeric, user-type or element/member place gives its
  byte size, known when compiling (sizes as the emitter's `size`; a user type its layout size, `m2-arrays-and-types`
  D7). Anything else as measured (a number literal: error or size).
- **`VAL`**: `VAL(s$)` is `_FLOAT`; `VAL(s$, type)` takes a type name (parsed already, `m2-parser-breadth`) of the
  slice's types, result that type (`SINGLE` and `DOUBLE` through a `_FLOAT` parse then narrowed; integers through
  the 64-bit parse), as `study\02` §5.5 describes; other type names marked.
- **`HEX$`, `OCT$`, `_BIN$`**: the number of digits a negative value is printed with comes from the argument's type
  (INTEGER 4 hex digits, LONG 8, `_INTEGER64` 16); a non-variable 64-bit expression passes 0, the minimal width
  (`study\02` §5.5); floats use the `_float` variants. The rule records the width; the emitter writes it.
- **`ASC`**: `ASC(s$)` and `ASC(s$, pos)`; `pos` a LONG slot.
- **`_PI`**: callable without parentheses (the parser gives a plain name; `sema` resolves a bare name that is no
  variable to the built-in when its row allows no required argument, as measured for `_PI` only in this change).
- **`CSNG` of an integer**: typed SINGLE; whether the value is narrowed is measured (`study\02` says not), and
  if it is not, the typed tree states it (a conversion kind or a rule flag), not the emitter alone.

### D5. Emission
The emitter keeps its generic `callname(args, mask)` for `Rule::Plain` and gets one function per other rule
(`len`, `val`, `radix`, `float_by_arg`, ...) that writes what the old compiler writes for that case (the libqb
entry point or inline expression, `study\02` §5.5), in a new module `codegen-cpp\src\builtins.rs`. The IR is not
changed: `ExprKind::Call` already names the built-in (ABI-neutral), and the rule is found from the built-in's id.

### D6. Split of the emitter first
Before any built-in lands, `codegen-cpp\src\lib.rs` is split by concern into modules (names indicative):
`names.rs` (C identifiers, labels, `#line`), `decl.rs` (types, sizes, declarations, allocation, descriptors),
`procs.rs` (procedures, signatures, prototypes, bodies), `stmt.rs` (operations), `place.rs` (places, store rules),
`value.rs` (values), `builtins.rs` (D5). A pure move: every `cpp` snapshot and tier 2 unchanged, checked before and
after. *Alternative:* split after the built-ins: the review asked for it "when the built-ins land", and moving code
first keeps the move reviewable on its own.

### D7. `SELECT CASE` lowered to existing operations
No new IR operation. `SELECT CASE e` lowers to:
1. Unless `e` is a plain scalar variable (measured: read at each test), a store of `e` into a hidden variable of the
   selector's type. **Its storage is static** (decided, `DIVERGENCES-QB45.md` Q-002): in the main module a `Temp`
   of main; in a procedure a hidden procedure-static variable (`Storage::Static`), so a recursive call overwrites
   it as in the old compiler. The IR states it; the emitter does not decide it. A string copy is a static `qbs`.
2. Per `CASE`: a `Branch` to the next `CASE` taken when the OR of its items is zero, `on_error` as `IF` uses it (the
   old compiler writes `if ((items)||is_error_pending())`, the same shape as `IF`), the body, then a `Jump` to the
   end. Each item: equal, `a <= sel AND sel <= b`, or `sel op v`, built as typed comparisons by the existing
   operator typing (`check\ops.rs`), so the float/integer rules come from there (the measured rounding of a float
   item against an integer selector, if any, is a conversion in the typed tree).
3. `EVERYCASE`: no `Jump` to the end; a hidden flag set in each body, and `CASE ELSE` guarded by it (the old code's
   `sc_N_var`), storage as in 1.
4. `CASE ELSE`: the body without a test.
Labels of the program inside a `CASE` body are labels of the body as for `IF` (`m2-control-flow-slice`).

*Alternative:* a multi-way IR operation (`Op::Select`): it would need its own error rule; the old compiler's code is
a chain of `if`s with the same per-statement error points as `IF`, so a chain of `Branch`es states it exactly.

### D8. `ON … GOTO/GOSUB` lowered to existing operations
`n` is stored into a hidden `Temp` converted as measured (provisional: to LONG as a store does); then a `Branch` to
a raise of error 5 when `n < 0`, then per target k a `Branch` (`n = k`) to the label, or for `GOSUB` to a
`Gosub`-and-continue sequence; then fall through (0, past the count, above 255: Q-001). The order of the negative
check is the old compiler's (after the label tests) only if a measurement shows a difference; otherwise first.
Line-number targets stay marked until line numbers are compiled.

### D9. Tests and corpus
- `tests\frontend\`: one `typed` test per rule and one per built-in group (the coverage check of the testing delta
  reads them), `check-fail` for every real error and marked form, `ir` and `cpp` for `SELECT CASE`, `EVERYCASE`
  and `ON … GOTO/GOSUB` and for each emission rule.
- `tests\corpus\slice\s25_string_builtins`, `s26_math_builtins`, `s27_select_case`, `s28_on_goto` (names may
  change; split further where a program would mix trapped errors with plain output), recorded with `qb64pe.exe`,
  each built-in called at least once and each measured edge value printed.
- The coverage check (testing delta): a tier-1 test lists the supported built-ins from `sema`'s list and requires
  each name to appear in a `slice.list` program and in a `typed` front-end test (a word match on the source,
  case-insensitive, with its suffix).
- The shrink-only lists are regenerated after each group.

## Risks / Trade-offs

- [The table is wrong for a function we treat as `Plain`] → every supported built-in is in a slice program recorded
  with `qb64pe.exe` (the coverage check), so a wrong slot or return type shows in tier 2 as different output.
- [Printed float formatting differs by type] → result types are measured by printing (D1), and `STR$`/`PRINT` use
  libqb's own formatting functions, chosen by the result type.
- [`LEN` of a place needs layout sizes in `sema`] → sizes are a fact of the language (measured with `LEN` in
  `m2-arrays-and-types`), not of the emitter; `sema` gets a `size_of(Ty)` and the emitter's `size` uses it.
- [Static `SELECT` copies in procedures are a known accident kept on purpose] → decided (Q-002), pinned by a slice
  program, to be measured against QB 4.5 when DOSBox has it.
- [Rules grow into a per-function enum by stealth] → a new `Rule` variant needs a measurement showing that no
  existing one fits; the review of the change checks this.

## Migration Plan

None: new behaviour behind "not supported yet" marks that go away. `INSTR` and `CHR$` move to the new checker with
their snapshots unchanged.

## Open Questions

None that change the plan. Provisional points (D3, D4, D8) are settled in task 2.1 by measurement.
