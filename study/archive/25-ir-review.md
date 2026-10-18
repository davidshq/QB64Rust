# 25. The IR review: keep or merge (2026-10-07)

Step 3 of `STATUS.md` "Next", asked for by `study\20` §3.4 and placed after the control-flow slice by `study\23`
§4. The question: does the IR (`crates\ir`) earn its place as a layer between the typed tree (`crates\sema`) and
the C++ emitter (`crates\codegen-cpp`), or should it be merged into the typed tree? `study\24` §2 warned that
the answer on *places* (array elements, `TYPE` members, an element passed by reference) cannot be given yet;
this review decides what it can and states what step 4 must settle.

Evidence: `crates\ir\src\lib.rs` (394 lines, the IR), `lower.rs` (541, the lowering), `sema\src\lib.rs` (the
typed tree), `codegen-cpp\src\lib.rs` (999, the emitter), the archived design D8 of `m2-control-flow-slice`, the
spec `openspec\specs\compiler\pipeline` ("ABI-neutral IR"), `study\02` §2 (how the old compiler emits
elements and members), and tier 1 (`cargo test`: green, 2026-10-07).

## 1. What the IR is now

**Shape.** A `Program` is a flat list of `Var`s (with a `Storage` class, including the lowering's `Temp`), the
`Proc`s and a main `Body`. A `Body` is a flat list of `Stmt`s plus `Label`s, which are positions, not
operations. A `Stmt` holds one or more `Op`s (`Assign`, `AssignAll`, `Print`, `Call`, `Jump`, `Branch`, `Gosub`,
`Return`, `SetHandler`, `Raise`, `Resume`, `Exit`, `End`, `System`, `SelectConsole`). Values are trees
(`Value { ty, kind }`) with explicit conversions, no `qb` (the old compiler's believed type: `sema` has already
inserted the conversions it implies, so the emitter never needs it). Types, operators and conversion kinds are
`sema`'s, re-exported (`study\20` §3.4's "share the duplicated enums" is done).

**What the lowering does** (`lower.rs`): the statement side does real work, the expression side none.

| Part | Lines | Work |
|---|---|---|
| `if_block`, `loop_block`, `for_loop`, labels, `exits` | ~250 | Blocks to flat bodies with made labels; `ELSEIF` with `UseValue`; `EXIT` to the innermost loop's exit; `FOR` to four typed temporaries, `AssignAll`, an entry label and the old compiler's test, with the narrowing store to the variable; label ids renumbered per body; label positions placed |
| `stmt` for plain statements | ~80 | A one-to-one mapping (`Goto` → `Jump`, `Print` → `Print`, …) |
| `value_of`, `args_of`, `storage` | ~70 | A field-by-field copy of `sema::Expr` into `ir::Value`, dropping `span` and `qb` |
| `may_raise`, `uses_strings`, `op_may_raise` | ~60 (in `lib.rs`) | Facts the emitter asks per operation and per value |

**What the emitter still does** that one could call lowering: `PRINT` into `tqbs`, a `skipN` label and the
per-item error check; by-value numeric arguments into `passN`; `qbs_cleanup` after anything that used strings;
handler numbering; the `RETURN` switch (`retK.txt`) and `RETURN_G` labels; the event check after user labels;
`if (!is_error_pending())` before a jump whose statement may have raised. All of these are the old runtime's
ABI (the spec forbids them in the IR: no libqb names, `passed` masks, by-value temporaries, event loop), so they
belong where they are.

## 2. Findings

### 2.1 The jump and error model: keep, final
The flat body with positions as labels and four control operations is the right shape, and it is doing the job
`study\20` asked for: the typed tree keeps `If`, `For`, `Do`, `While` as structured nodes (the language server
and the dumps want them), the emitter sees only statements and `goto`s (the old runtime's `do{…}while(r)`
wrapper and the per-statement error service need exactly that). The pending-error rule is stated once, in the
IR's doc comment and the spec, and the emitter's `raised` flag, `OnError::Skip`/`UseValue` and the procedure
prologue are its encoding. Measured behaviour (`verification\v17_*`, `s12_error_in_print`) backs it. Nothing
here is provisional.

### 2.2 The expression side is a copy
`ir::Value`/`ValueKind` mirror `sema::Expr`/`ExprKind` variant for variant; `value_of` is 35 lines of
`Box::new(value_of(..))`. The same for `Arg`, `Const`, `PrintItem`, `Resume`, `Storage` (plus `Temp`),
`ProcKind`. This is the "1:1 copy" `study\20` §3.4 objected to, now confined to values. Two readings:

- It is harmless duplication (about 120 lines of types and mapping) that buys a value tree without `span` and
  `qb`, and a `Storage::Temp` that `sema` has no business knowing.
- It is a layer that will have to be extended twice for every new expression form (built-ins at step 8 add
  none, since calls are table-driven; arrays and `TYPE` add places; unsigned types add `Ty` variants, shared
  already).

**Decision: leave it until step 4, then decide by one criterion.** The arrays-and-`TYPE` slice is the first
change where expression lowering could do work (§3). If after it `value_of` is still a mapping, replace
`ir::Value` by `sema::Expr` the way `Ty` and `BinOp` were shared: the IR re-exports it, the emitter ignores `span`
and `qb`. If the slice gives the lowering real work on values (index arithmetic, bounds checks, member offsets
as IR operations), the copy stays, because it is then a different tree. Do not merge the statement side either
way.

### 2.3 Small things, fixed or not now
- `Stmt.may_raise` is computed by the lowering and used only by the IR dump; the emitter recomputes per
  operation (`raised |= op.may_raise()`), which it must, since the rule is per operation. Keep the field: the
  dump is a test surface (`lower_header_errors.bas`). No change.
- `Value::uses_strings` is an emitter concern ("string results need temporary cleanup") sitting on the IR. It is
  ABI-neutral in wording (it names no libqb symbol) and the emitter would otherwise walk the value again. Keep.
- `LabelIds::new` renumbers labels per body with a `HashMap`; `Lowerer::new` filters `p.labels` per body. Fine at
  this size; a `sema::Body` holding its own labels would remove both, but `sema` keeps labels flat because a
  label's body is a property, not a position (design D5). No change.
- Temporaries are named `for{n}.value` etc. and the emitter maps `.` to `_`; `c_ident` maps `.` in BASIC names
  to `__046__`. Two conventions for one character, both documented. No change.
- `Program.vars` is one flat list with the temporaries appended, numbered after `sema`'s ids, so `VarId(v.0)`
  works without a map. This stays true as long as the lowering only appends. A comment on `lower()` says so.
- Tier 1 lowers and emits only the snapshot tests and the slice (`slice_list_programs_have_no_diagnostics` checks
  diagnostics only). The 127 accepted corpus programs and the 23 upstream ones are lowered first in tier 2.
  **Add to tier 1** (test engineer, §4): lower and emit every accepted input under `catch_unwind` (no C++
  compile) and an `ir::validate` that checks every label is placed (`at <= stmts.len()`), every jump names a
  label of its own body, `Resume::To` and `SetHandler` name main-module labels, every `Var` referenced exists.
  About a second more in `cargo test`; it catches a lowering bug the moment places arrive instead of in tier 2.

## 3. The place question, stated for step 4

Today a place is a `VarId` in three spots: `Op::Assign { place }`, `AssignAll(Vec<(VarId, Value)>)`,
`Arg::Ref(VarId)`; and a read is `ValueKind::Var(VarId)`. The slice adds elements and members. What the old
compiler emits (`study\02` §2.1 to §2.3) says what the IR must express:

| Place | Old compiler's read | Old compiler's store |
|---|---|---|
| Element `a(i)` | `((int32*)(A[0]))[array_check((i)-A[4],A[5])]`, index rounded with `qbr` | `tmp_long=<idx>; if (!is_error_pending()) ((int32*)(A[0]))[tmp_long]=<rhs>;` |
| Member `u.m` | `*(int32*)(((char*)__UDT_U)+(off))`, `off` a constant | `*(int32*)(((char*)__UDT_U)+(off))=<rhs>;` **not guarded** |
| Member of element `a(i).m` | base `__ARRAY_UDT_A[0]`, index inside `off` | index inside `off`, not hoisted, **not guarded** |
| Element by reference | address of the element, taken before the call | — |

So the slice needs, in the IR:

1. A `Place` enum: `Var(VarId)`, `Element { array: VarId, index: Vec<Value> }`, `Member { base: Box<Place>,
   member: MemberId }`; `ValueKind::Var` becomes `Load(Place)`, `Op::Assign { place: Place }`, `Arg::Ref(Place)`.
   A `Place::may_raise` (an index may raise error 9 or, when it needs rounding, nothing; a member never does).
2. **An exception to the pending-error rule, to be measured, not assumed:** an element store is guarded by
   `is_error_pending()` *after* the index and *before* the value, so an error pending from the index (or from
   earlier in the statement) skips the store, while an error from the value still stores the placeholder; a
   member store is never guarded. The rule in the IR doc comment ("a store made with that value still
   happens") is true for `Var` and must gain a per-place clause. Measure: `x(9) = f` with `f` raising, `x(99) = 5`
   under `RESUME NEXT`, `u.m = f`, `a(99).m = 5`, and `x(99) = f` (both raise: which error number does `ERR`
   report?).
3. **Evaluation order of an element passed by reference:** the old compiler takes the address before the call,
   so a `SUB` that `REDIM`s the array gets a stale pointer (not in the slice: static arrays only; but the order
   still decides when the index raises relative to the other arguments). Measure with two raising arguments.
4. `Ty` gains `User(TypeId)` for `TYPE` values (whole-`TYPE` assignment is in the slice as "member read and
   write" only; whole-`TYPE` copy `copy_full_udt` can wait). Every exhaustive `match` on `Ty` in `sema`, `ir` and
   `codegen-cpp` (`c_type`, `size`, `is_qbs`, `type_word`, `clear.txt`) then fails to compile until handled:
   that is the no-wildcard lint doing its job (`study\21`).
5. Whether index arithmetic (`array_check`, the flattening of several indexes, the `qbr` rounding) is an IR
   operation or the emitter's encoding of `Element`. Recommendation: **the emitter's**, since `array_check` and
   the descriptor layout `A[4]`, `A[5]`, `A[6]` are libqb's ABI; the IR says "element `i` of `a`, index already
   converted to `_INTEGER64`" and the emitter writes the check. This is what makes §2.2's criterion likely to
   come out as "still a mapping": then share `sema::Expr`.

Not for step 4 (unchanged from `study\24` §2): `REDIM`, dynamic arrays, `OPTION BASE`, member arrays,
whole-array assignment.

## 4. Panel

Roles as `CLAUDE.md` rule 6, plus a **language-server engineer** (the typed tree's second consumer decides where
the split between tree and IR should lie) and a **debugger engineer for M5** (the IR's statement and line
structure is what a debugger steps by).

- **Pragmatic engineer.** Keep. The crate is 940 lines, the mapping part 120; deleting it saves less than the
  arguments about it cost. Nothing in the order of work is blocked by the expression copy. Agrees with the §2.2
  criterion because it is mechanical: count the lines of `value_of` that are not `Box::new(value_of(..))`.
- **QB64 engineer.** Keep, and §3 item 2 is the important finding of this review: the old compiler's element
  store is guarded where its scalar store is not, and its member store is not guarded even with an index inside.
  That is three different rules for "a store with an error pending", and the slice must measure them before
  writing any of them down. Also: `array_check` is a runtime call, so a bounds error is raised inside the
  expression like any built-in: `may_raise` is right to be per place.
- **Compiler/languages engineer.** Keep the statement IR; it is a proper lowering now (structured to flat, named
  temporaries, a stated error model), and the review in `study\20` §3.4 was right to wait for this. The value
  copy is the usual two-tree tax; the test in §2.2 is the standard one (does the lowering ever rewrite a value?).
  Dissent noted for the record: an IR with *places* and *loads* is a different tree from a typed expression tree
  even when the mapping is mechanical, and sharing `sema::Expr` would put `Place` into `sema`, which is fine
  (`sema` resolves `a(i)` to an element anyway). So "share" is the likely outcome either way, and nothing
  depends on deciding now.
- **Rust engineer.** Keep. Two things to do with the slice, not before: `ValueKind::Var` → `Load(Place)` is a
  rename that the no-wildcard lint makes safe; `Ty::User` forces about eight `match` arms, all wanted. Sharing
  `sema::Expr` later means `sema` types gain `Storage::Temp` or the IR keeps its own `Var` list (keep the own
  list: temporaries are the lowering's). No `unsafe`, no `HashMap` worth replacing, `debug_assert` for placed
  labels is the right level for an internal invariant, promoted to `validate` by §2.3.
- **Test engineer.** Keep, with the tier-1 addition of §2.3: lower and emit every accepted input, and validate
  the IR. Today a lowering panic on a corpus program would first show in tier 2 as a build failure. The D8 rows
  all have `ir` snapshots (`lower_if`, `lower_loops`, `lower_for`, `lower_jumps_proc`, `lower_header_errors`,
  `labels_ir`, `error_lowering`, `lowering_pairs`); step 4 adds rows for each `Place` form and each store rule
  of §3 item 2, with a `cpp` snapshot per row and an end-to-end program per measured rule.
- **Language-server engineer.** Keep, and this is the strongest reason: the language server reads the typed
  tree (symbols, `qb` for hover, spans, blocks for the outline and folding) and never the IR. Merging the IR into
  the typed tree would make the server's input the flat jump list, or make the tree carry both forms. The split
  is between "what the program means" and "how it runs", and the server wants only the first.
- **Debugger engineer (M5).** Keep. A debugger steps by `Stmt` with its `line` and `span`, and the IR's
  one-source-statement-to-one-or-more-`Stmt`s with the block's `end_line` on the made statements is what
  "step over `NEXT`" needs. Ask for one thing at step 4: keep `Stmt.span` the whole source statement also for
  the made statements (it is today: `s.span`), so a breakpoint on a `FOR` line hits its header and its `NEXT`
  statement by line, not by span (an `IF` branch's statement carries the condition's span today; by line it is
  still the `IF` or `ELSEIF` line).

**Outcome: keep the IR; the jump and error model is final; the expression copy is decided after step 4 by the
criterion of §2.2; the place question is answered by step 4 with the measurements of §3.** No order change.

## 5. Decisions and follow-ups

| What | When | Where it is recorded |
|---|---|---|
| IR kept; flat bodies with explicit jumps and the pending-error rule are final | now | `DECISIONS.md`, this file |
| `ir::Value` vs `sema::Expr`: decide after the slice by whether `value_of` does work | step 4, last task | step 4's `design.md` |
| `Place` enum, `Load(Place)`, `Arg::Ref(Place)`, per-place store rule measured first | step 4 | step 4's `design.md` D-rows, `verification\v18_*` |
| Tier 1 lowers and emits every accepted input; `ir::validate` | first task of step 4 (it protects the slice) | step 4's `tasks.md` |
| Spec `compiler\pipeline` "ABI-neutral IR": add the store rule per place once measured | step 4 | the change's spec delta |

Nothing in this review changes code. The `STATUS.md` step 3 line becomes "done" and step 4 starts.
