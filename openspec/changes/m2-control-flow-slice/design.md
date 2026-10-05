# Design

## Context

See `proposal.md` for motivation and the spec deltas for requirements. Background: control-flow emission in
`study\02` §6.1, §6.2 and §6.5 (read from `qb64pe.bas`, not all measured), operators in §1.4, the constant
evaluator in §7, the order of work in `study\23` §4. The designs of the earlier changes still hold
(`openspec\changes\archive\…-m2-workspace-and-slice\design.md` D1–D12, `…-m2-procedures-and-errors\design.md`
D1–D12), and so do D4 ("Blocks are nodes", with its "As built") and D10 of `m2-parser-breadth`.

Where the code stands: the parser builds every block node with a header node (`ast.rs`: `IfBlock`, `IfStmt`,
`ForBlock`, `DoBlock`, `WhileBlock`, …); `sema` marks each block "not supported yet" at its header and still
checks the statements inside (`check.rs`, `block_parts`). `GOTO`, `GOSUB` and `RETURN` parse and are marked.
`sema` knows only `+ - * /` and unary `-` (`BinOp`). `CONST` and `OPTION` have no node yet. The IR is one `Stmt`
per source statement with one `Op` each; `Label { name, at }` is a position in a flat body, main module only.

### Probed 2026-10-05 (`qb64pe.exe` `16f629784e`, scratch folder, not the repo)

Three programs, compiled with `-x` and run with `QB64PE_NOPROMPT=y`. The handler prints `ERR` and does `RESUME
NEXT`; in the header program it also makes the failing call succeed every third time, so the loops end.

| Construct | Result |
|---|---|
| `IF CHR$(-1) = "a" THEN … ELSE …` (single-line and block) | handler, then the **`THEN`** branch |
| `WHILE LEN(CHR$(-1)) = 1 AND n < 2` | handler, then the **body**; with the error every pass, `RESUME NEXT` **loops forever** (seen before the cap was added) |
| `DO WHILE …` with the same condition | as `WHILE` |
| `DO … LOOP UNTIL CHR$(-1) = "a" OR n >= 3` | body once, handler, then **after the loop** |
| `FOR i = 1 TO LEN(CHR$(-1)) + 2` | handler, then the body with **`i` = 0** (not assigned); prints 0, 1, 2; `i` = 3 after |
| `FOR i% = 32760 TO 32767 STEP 4` | ends with `i%` = **-32768** (32768 wraps in `i%`; the wider temporary ends the loop) |
| `FOR b& = 2147483640 TO 2147483647 STEP 5` | ends with `b&` = -2147483646 |
| `FOR s! = 0 TO 1 STEP 0.1` | 10 passes, `s!` = 1 after |
| `FOR k = 1 TO 5: k = k + 1` | prints 2, 4, 6 (the step is added to the current value) |
| `FOR m = 3 TO 1` | body not run, `m` = 3 |
| `FOR m = 1 TO e` with `e` changed in the body | 3 passes (limit evaluated once) |
| `FOR z = 1 TO 2 STEP 0` with `z = z + 1` in the body | prints 2, 3 |
| `3 > 2; 2 > 3; "a" < "b"` | `-1  0 -1` |
| `1.5 AND 3; 2.5 OR 0; NOT 0; NOT 1.5; 5 XOR 3` | `2  2 -1 -3  6` (floats rounded half to even) |
| `7 \ 2; -7 \ 2; 7.5 \ 2; -7 MOD 3; 7.5 MOD 2` | `3 -3  4 -1  0` |
| `2 ^ 3 ^ 2; 5 EQV 3; 5 IMP 3` | `64 -7 -5` |
| `1 _ANDALSO 2; 0 _ORELSE 0; _NEGATE 0; _NEGATE 5` | `-1  0 -1  0` |
| `a! = 2.1: a! = 2.1; 2.1 = a!` | `-1 -1` |
| `2 ^ 0.5` | `1.414214` (typed SINGLE) |
| `CONST c1 = 2 ^ 3 ^ 2, c2 = 7 \ 2, c3 = 1 / 3, c4 = "x" + "y", c5% = 3.7, c6 = 3 > 2` | `512  3  .3333333333333333 xy 4 -1` |
| `GOSUB lab` inside a SUB, `lab:` in the SUB | works; `RETURN` continues in the SUB |
| `RETURN` in a SUB with no `GOSUB` pending | error 3, trapped by the main handler; `RESUME NEXT` continues in the SUB |

The rest (D1) is measured as task 1 and recorded in `verification\v17_*`.

## Goals / Non-Goals

**Goals:**
- The IR expresses jumps into and out of blocks, `GOSUB`/`RETURN`, labels in procedures and the measured
  header-error behaviour, still without C or libqb names. The IR review after this change then has a real IR to
  judge.
- The 53 corpus and 12 upstream programs blocked only by these constructs pass end to end, if nothing else is
  hidden behind the marks (a header's condition is not checked today).
- No program the old compiler rejects is accepted; nothing unmeasured is guessed.

**Non-Goals:**
- Matching the old compiler's C++ text beyond the ABI (as before).
- `SELECT CASE`, `ON … GOTO/GOSUB`, line numbers, `DEFxxx` (step 8 of the order of work).
- The rest of `m2-parser-breadth` (statements, templates, `$IF`, `$INCLUDE`, the blunt follow-on rule).

## Decisions

### D1. Measure first
Before code, `verification\v17_*` programs (run with `verification\run.sh`), findings in `study\00` §5. The
questions:
- The pending-error rule itself (D8): `x = 5: x = ASC("")` under `RESUME NEXT`, then `PRINT x` (does the store
  happen, and with which value?); the same for a string (`s$ = "a": s$ = CHR$(-1)`), for a SUB called with a
  raising argument (is the SUB entered?), and the placeholder value of each raising call the slice programs use.
- `ELSEIF` condition that raises: the old compiler writes `if (e){` without the error-pending term, so the
  branch depends on the failed call's placeholder value. Which branch runs, for a string and a numeric condition?
- `RESUME` (retry) after an error in each header kind; an error in a `LOOP WHILE` condition; an error in the
  step and in the start of a `FOR`; an error in the expression of a `GOTO`-free single-line `IF … ELSE`.
- `GOTO` into a `FOR` body from outside and out of one; `GOTO` into a `WHILE`; `EXIT FOR` inside a `WHILE` inside
  a `FOR`; `NEXT` with a wrong variable, the wrong order, a variable with and without its suffix; `FOR` with a
  string variable, a constant as the variable.
- `RETURN label`; `RETURN` in a SUB after a `GOSUB` in main that called it; `GOSUB` from a handler; a label with
  the same name in main and in a SUB; `GOTO` from a SUB to a main label (error text); `ON ERROR GOTO` naming a
  SUB label (known: "Common label within a SUB/FUNCTION").
- `CONST`: visibility before its line and in earlier and later procedures, a procedure's constant in main, a
  `CONST` and a variable of the same name (each order), a `CONST` inside a block, the type of each kind of value
  (how `PRINT c3` prints, `c = 1 / 3` vs `c# = 1 / 3`, an integer-valued float), overflow, string comparison in a
  `CONST`, `CONST` in a single-line `IF`, `label: CONST` on one line (`study\00` §6 says it fails to compile).
- `OPTION _EXPLICIT`: where it may stand (after other statements, in a SUB, twice), what counts as a declaration
  (parameters, the function name, `SHARED`, `STATIC`, `CONST`, a `FOR` variable), `_EXPLICITARRAY` with scalars.
- Comparisons and logic: `_INTEGER64` operands near the limits; `\` and `MOD` with `_INTEGER64` and with float
  operands beyond 2^63; `MOD` by zero (fatal, like `\`); string comparison of bytes above 127; `NOT` of an
  INTEGER printed (type); `^` with a negative base.

Each answer either fixes a rule in D3–D9 or makes the case "not supported yet". A fact that changes a spec
scenario is written into the spec delta before the code.

### D2. Parser: `CONST` and `OPTION`
`parser\decl.rs` gains `ConstStmt` (`CONST name[suffix] = expr {, name[suffix] = expr}`, items `ConstItem`) and
`OptionStmt` (`OPTION` and one word: `_EXPLICIT`, `_EXPLICITARRAY`, `BASE n`; also the `$NOPREFIX` spelling
`EXPLICIT` only if measured). Accessors in `ast.rs`. This takes `CONST` and `OPTION` out of `m2-parser-breadth`
task 7.2, which is edited to say so.

### D3. `sema`: split first, then real blocks
First `check.rs` is split by family (`study\23` §3): `check\expr.rs`, `check\decl.rs`, `check\proc.rs`,
`check\flow.rs`, `check\blocks.rs`, with no snapshot changing. Then the typed tree gains block statements that keep
the source structure (the language server and dumps read it): `StmtKind::If { branches: Vec<(Expr, Vec<Stmt>)>,
else_: Option<Vec<Stmt>> }`, `For { var, start, end, step, body }`, `Do { test: Option<(Pre|Post, While|Until,
Expr)>, body }`, `While { cond, body }`, `Goto(LabelId)`, `Gosub(LabelId)`, `Return(Option<LabelId>)`, `ExitLoop(
For|Do|While)`. A single-line `IF` becomes the same `If` (the old compiler rewrites it to block form). Conditions
must be numeric. `block_parts` keeps marking only `SELECT CASE`, `TYPE`, `DECLARE LIBRARY` and `DEF FN`.

The `FOR` variable must be a numeric scalar (a `CONST`, a string or an array element is an error or "not
supported yet" as measured). `NEXT` variables are resolved like any variable and compared, by identity, with the
variables of the `FOR` blocks they close (`NEXT j, i`: the inner block's `NextStmt` and the outer block's
"closed by child" accessor of `m2-parser-breadth` D4).

### D4. Operators
`BinOp` gains `Eq, Ne, Lt, Gt, Le, Ge, And, Or, Xor, Eqv, Imp, AndAlso, OrElse, IDiv, Mod, Pow`; a unary
`UnOp { Neg, Not, Negate }` replaces `ExprKind::Neg`. Types follow `study\02` §1.4 with the `ty`/`qb` pair of the
first slice: comparisons are LONG (`ty` I32, value -1/0), both float operands converted to the narrower float
first; string comparisons are their own `ExprKind::StrCompare` (they call the runtime); logical operators, `\` and
`MOD` convert float operands with `qbr` rounding to I64 (explicit `Convert` nodes) and compute in the operands'
promoted integer width, as `+` does (32-bit for INTEGER and LONG, 64-bit with an `_INTEGER64`), `qb` I64; `^`
computes in F80 with the `qb` of §1.4. Constant folding extends to comparisons and the logical operators on
integers (wrapping); `\` and `MOD` by a literal 0 are never folded (error 11 belongs to run time) and `^` is not
folded. `may_raise`: `IDiv`, `Mod` (error 11) and `Pow` (error 5). `_ANDALSO`/`_ORELSE` keep their short circuit
into the IR (`BinOp` with that meaning; the emitter writes `&&`/`||`).

The typing rules live in one place: one function in `check\expr.rs` takes the operator and the operands' (`ty`,
`qb`) pairs and returns the computation type, the believed type and the conversion of each operand; nothing else
in `sema`, the folder or the constant evaluator (D6) matches on `Ty` to type an operator. *Why:* `Ty` becomes a
type table with unsigned types at step 6 of the order of work (`study\23` §2.5), after these 16 operators exist;
that step then rewrites this function, not every use.

### D5. Labels per body
Labels are collected per body (main module, each procedure) in a pass before the statements, so a forward `GOTO`
resolves and the language server can find labels without the statement pass (`study\23` §2.6). A label inside a
block belongs to the enclosing body. `LabelId` stays program-wide; each `Label` records its body. `ON ERROR GOTO`
accepts only main-module labels (measured). The symbol table records labels as before.

### D6. Constants
A new `sema\consteval.rs` implements the old compiler's evaluator (`study\02` §7) over the typed syntax, not over
text: 64-bit signed integer arithmetic with wrap (unsigned arithmetic waits for unsigned types), `/` in F80, `^`
right-associative, comparisons -1/0 (numbers only), logical operators on integers, string `+`; operands are
literals and constants already defined. A function call is "not supported yet"; anything the evaluator rejects is
an error. Each constant gets a type and a value as measured in D1 (the old compiler re-reads its printed result,
so an integer-valued float may come back as `_INTEGER64`); a suffix on the name converts with rounding. Uses of the
constant become literal nodes of that type in the typed tree (so folding and printing treat them as the old
compiler treats the substituted text). Scope: main-module constants are visible from their line on, also in
procedures defined later; procedure constants in that procedure from their line on. Right-associative `^` is
implemented as measured, the "keep" default of `study\00` §6, where it stays listed for the step-6 decisions.

### D7. `OPTION _EXPLICIT`
The `OPTION` statement sets a flag for the rest of the file (or as measured in D1). With it, the place where
`resolve` would create an implicit variable reports "variable `x` is not declared" instead. `_EXPLICITARRAY` sets
only the array flag, which matters once arrays exist; today an implicit array is "not supported yet" anyway.

### D8. The IR: flat bodies, explicit jumps
```
Body    { labels: Vec<Label>, stmts: Vec<Stmt> }
Label   { name: Option<String>, line, at }      // None: made by the lowering
Storage += Temp(Option<ProcId>)                  // hidden; per body (static in main, per call in a procedure)
Op     += Jump(LabelId)
        | Branch { cond: Value, when: Zero | NonZero, to: LabelId, on_error: Skip | UseValue }
        | AssignAll(Vec<(VarId, Value)>)         // evaluate and store each in order, then the statement rule
        | Gosub(LabelId)                         // continue at the label; RETURN comes back after this statement
        | Return(Option<LabelId>)                // to the last GOSUB, or forget it and go to the label; error 3 if none
```
**The error rule, restated.** The IR said "a raising operation skips the rest of its statement". That was true
only of `PRINT`: the emitter stores an assignment's value and makes a call without any check, as the old compiler
does. The rule the IR states from this change on (module documentation, pipeline spec):

- A raising operation records a pending error and yields a placeholder value; the statement goes on, and a store
  or a call made with that value still happens (pinned by the first question of D1).
- Only named points check for a pending error: each `PRINT` item (skips the rest of the statement), `Jump` and
  `Gosub` (not taken), and `Branch` with `on_error: Skip` (not taken).
- `Branch` with `on_error: UseValue` does not check: it tests the placeholder value.
- Errors are serviced at the statement boundary; retry and resume work on statements, as before.

`UseValue` and `AssignAll` are therefore the rule, not exceptions to it: `AssignAll` is three stores that all
happen. With the lowering below, the rule gives the measured header behaviour:

| Source | Lowered (each line one statement; `L…` labels made by the lowering) |
|---|---|
| `IF c THEN a ELSE b` | `Branch(c, Zero → Lelse)`; `a`; `Jump(Lend)`; `Lelse:` `b`; `Lend:` |
| `ELSEIF c THEN` | `Branch(c, Zero → Lnext, on_error: UseValue)` if D1 confirms the placeholder rule, else `Skip` |
| `WHILE c` … `WEND` | `Ltop:` `Branch(c, Zero → Lexit)`; body; `Jump(Ltop)`; `Lexit:` |
| `DO UNTIL c` / `DO WHILE c` | as `WHILE` with `NonZero` / `Zero` |
| `LOOP UNTIL c` / `LOOP WHILE c` | `Branch(c, Zero / NonZero → Ltop)`, falling through to `Lexit:` |
| `FOR v = a TO b STEP s` | `AssignAll(t = a, f = b, st = s); Jump(Lentry)`; `Lbody:` body; `Lnext:` `t = st + v`; `Lentry:` `v = t` (stored as an assignment), `Branch(past(t, f, st), NonZero → Lexit)`, `Jump(Lbody)`; `Lexit:` |
| `EXIT FOR/DO/WHILE` | `Jump(Lexit)` of the innermost such block |
| `GOTO x` / `GOSUB x` / `RETURN` | `Jump(x)` / `Gosub(x)` / `Return(None)` |

So with an error pending, the `Branch` of an `IF`, `WHILE` or `DO WHILE` condition is not taken and control falls
into the body; in `LOOP UNTIL` the backward branch is not taken and the loop is left; in a `FOR` header the
`Jump(Lentry)` is not taken and control falls into `Lbody` with `v` unassigned. All three limits are stored
first, by `AssignAll`, from placeholder values where a call failed (the probe's `LEN(CHR$(-1)) + 2` loop prints 0,
1, 2, which fits a limit of 2); as separate statements, an error in the limit would be serviced before the step
is stored, the step would stay 0 and the loop would never end. `t`, `f`, `st` are
`Temp` variables of the widened types of `study\02` §6.5 (SINGLE → DOUBLE, DOUBLE/`_FLOAT` → `_FLOAT`, INTEGER →
LONG, LONG/`_INTEGER64` → `_INTEGER64`); `past(t, f, st)` is `(st < 0 AND t < f) OR (st >= 0 AND t > f)` written
with the IR's own operators. Statements the lowering makes (`Lnext`, `Lentry`) carry the `NEXT` line and cannot
raise unless their values can.

**Alternatives.** A structured IR (`If`, `Loop`, `For` nodes with nested bodies) mirrors the typed tree, which is
exactly what the review questions (`study\20` §3.4), and it still needs jumps for `GOTO` into a block and a rule
per node kind for header errors. A graph of basic blocks gives the most freedom for later optimisation, but
retry and resume work on statements, not blocks, so it would need statements inside blocks anyway; nothing needs
the optimisation yet. The flat form keeps the current model (labels are positions) and adds five operations.

### D9. The emitter
- Each statement keeps `do{ … if(!qbevent)break;evnt(N);}while(r);`. `Jump` is `goto L;`, preceded by
  `if (!is_error_pending())` when an earlier operation of its statement may raise (D8: not taken while an error
  is pending); `Branch` is
  `if ((!(c))&&(!is_error_pending())) goto L;` for `Zero`/`Skip` (the condition is evaluated first, so a raising
  condition never jumps), `if (!(c)) goto L;` for `UseValue`; string results are threaded through
  `qbs_cleanup(qbs_tmp_base, …)` as in `study\02` §1.6. A `goto` out of the `do{}` skips the epilogue only when no
  error is pending, as in the old compiler.
- Labels: user labels `LABEL_<name>:;` (C++ labels are per function, so the same name in main and in a SUB is
  fine), lowering labels `L_<n>:;`, both followed by the old compiler's `if(qbevent){evnt(N);r=0;}` for user labels
  only.
- `Gosub` emits the old sequence (`return_point[next_return_point++]=G; … goto LABEL_x; RETURN_G:;`, the label
  inside the statement's `do{}`), and adds `case G: goto RETURN_G; break;` to `retK.txt` of its body; `Return(None)`
  includes `retK.txt` (in a procedure, `case 0` is `error(3)`), `Return(Some(l))` emits the main-only form of
  `study\02` §6.2.
- `Temp` variables are declared like locals (`maindata.txt` static in main, `dataK.txt` in a procedure), with the
  emitter's own names. `AssignAll` is a list of assignments.
- No declaration is ever emitted inside a statement, so a `goto` into a block never crosses an initialisation
  (C++ rejects that).

### D10. Diagnostics
As before: only errors the old compiler also reports, or "not supported yet". New errors: a string condition, a
wrong `NEXT` variable, a jump to a label of another body, `ON ERROR GOTO` to a procedure label, an undeclared
variable under `OPTION _EXPLICIT`, an assignment to a constant, a constant the evaluator rejects, a `FOR` variable
that is not a numeric scalar, as measured in D1. "Not supported yet": line-number targets, `RESUME` in a
procedure, functions in `CONST`, `OPTION BASE`, `SELECT CASE`, `ON … GOTO`.

### D11. Tests and corpus
- New `tests\corpus\slice\` programs, recorded with `qb64pe.exe` (`SOURCE.md` updated; no `PRINT` with a comma,
  which hangs under a redirected console): `s13_if` (both forms, `ELSEIF`, nesting, `IF … GOTO`), `s14_loops`
  (`DO` in four forms, `WHILE`, `EXIT`), `s15_for` (the FOR rows of the probe table, every numeric type, negative
  and float steps, `NEXT j, i`), `s16_goto_gosub` (jumps into and out of blocks, `GOSUB` in main and in a SUB,
  `RETURN label`, labels with the same name in two bodies), `s17_operators` (the operator rows), `s18_const`
  (scope, typing, the evaluator's quirks), `s19_header_errors` (each header kind with `RESUME NEXT` and `RESUME`,
  handlers capped so that every loop ends).
- `tests\frontend\`: `check-fail` tests for each D10 error, `ir` snapshots for each lowering row of D8, `cpp`
  snapshots for `GOSUB`/`RETURN` and a `FOR` in a procedure.
- `slice.list` gains the new programs and every corpus program that passes (expected: the 53 of the count, listed
  in task 9.1); `tests\upstream\pass.list` gains the upstream programs that pass; the three shrink-only lists are
  regenerated (entries may only go away).

## Risks / Trade-offs

- [The count hides errors behind the marks: a block header's condition is not checked today] → The count is an
  upper bound; the result is whatever tier 2 shows, and the gap is reported in task 9.
- [`RESUME NEXT` after an error in a `WHILE` condition loops forever in the old compiler, and so in ours] → Kept
  (it is the measured behaviour, and the statement rule gives it for free); `s19` caps its handler. Listed with the
  bug-compatibility choices of `study\00` §6 for step 6.
- [The constant evaluator reads and re-reads text in the old compiler; evaluating trees may differ in corners] →
  D1 measures typing and the corpus and upstream `CONST` programs; any corner that differs is "not supported yet".
- [The `FOR` temporaries' widths and the narrowing store decide results at type limits] → `s15` covers every
  numeric type at its limit, with folding on and off.
- [Splitting `check.rs` while adding features] → The split is its own task with no snapshot change, committed
  before any feature.
- [Flat IR loses the structure the IR review may want] → The typed tree keeps the structure; the review compares
  both with real code in hand.
- [The IR review after this change sees only scalar places: `Op::Assign { place: VarId }` cannot name an array
  element or a `TYPE` member, and those come at step 8] → The review decides the jump and error model and "keep
  or merge" on that evidence, and records the place question (elements, members, an element passed by reference)
  as open until arrays and `TYPE`; "keep" is not final for places.
- [The constant evaluator is the riskiest part and tells nothing about the IR] → Task order: operators, blocks,
  IR and emitter (groups 3, 5, 6, 7) are done and committed before constants and `OPTION` (group 4).

## Open Questions

None that change the approach. D1's answers fix details (the `ELSEIF` error rule, constant typing, `OPTION`
placement) and are written into the spec deltas before the code that depends on them.
