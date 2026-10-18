# Fourth review: does the code match the plan, and where do arrays and `TYPE` go?

Review of 2026-10-07, after `m2-control-flow-slice` groups 1–5 (every operator through to C++, `CONST`, `OPTION
_EXPLICIT`, labels per body, blocks typed by `sema`; the IR gate before task 6). Asked: is the implementation
diverging from the accepted direction; is the architecture or the process wrong anywhere in a way that matters.
Nits were not collected. The method was reading the code (`ir`, `sema`'s typed tree, the emitter, the parser's
shape), the design of the running change (D8, D9), and the two earlier reviews (`study\20`, `study\23`).

## 1. Verdict

Not diverging. The code is what the plan said it would be, and where it could have drifted, it has not:

- The pre-coding decisions hold and are enforced, not only stated: bytes never `str` (`clippy.toml`), the lossless
  tree, hand-written typed accessors (`syntax\src\ast.rs`), a typed tree with explicit conversions, an
  ABI-neutral IR, and an emitter that alone knows `qbs_*`, `evnt`, `error_goto_line`
  (`codegen-cpp\src\lib.rs`). Nothing of the ABI has leaked upward.
- "Measure, never guess" has held under pressure. D8's error rule was corrected by measurement (the former "a
  raising operation skips the rest of its statement" was true only of `PRINT`), and the design records which
  measurement corrected what (`m2-control-flow-slice` design D8 "Measured (1.1)").
- The ratchet (three shrink-only lists, the "not supported yet" marker, the tier-2 `.err` meaning) makes "never
  wrong code" checkable rather than aspirational, and lets any session see at once whether a change moved the
  numbers the right way. It is the most valuable process asset the project has.
- The flat IR with jumps (D8) is the right shape, and it answers `study\20` §3.4 by itself: before task 6
  `ir\src\lower.rs` is a field-by-field copy of the typed tree; after 6.2 the lowering does real work (structured
  blocks to flat bodies, `FOR` to temporaries, `AssignAll` and `Branch`, `ELSEIF` with `UseValue`). The IR review
  stays where `study\23` put it, right after the slice; expect "keep" on the jump and error model.

Considered and left as they are: `ast.rs` hand-written (1,495 lines; a generator only if it passes about 4,000);
`ty` and `qb` on every expression (the old compiler's believed type, measured, cheaper here than reconstructed in
the emitter); building through the clone's `Makefile` until M3; two OpenSpec changes open with one paused (the
pause is explicit and the hand-over of `CONST`/`OPTION` was clean); the progress rate (20 of 279 upstream, 89 of
282 corpus after five days of M2), which is what "never wrong code, never guess" costs, and the ratchet shows it
is not stalling.

## 2. The one strategic change: arrays and `TYPE` are too late

The accepted order (`study\23` §4) puts arrays and `TYPE` at step 8, after the thin language server (5), the
bug-compatibility decisions, differential tester, `Ty` as a type table and unsigned types (6), and 249 plain
built-ins (7). The reason recorded in `study\20` §4 was the pragmatic engineer's: control flow is smaller, nearly
every program needs `IF` and `FOR`, and it moves the corpus number. That was right for the control-flow slice,
which is now nearly done. It is not a reason to keep arrays behind steps 6 and 7:

1. **The yardstick is array-heavy; the corpus is not.** Counted 2026-10-07: 20 of the 282 corpus programs
   dimension an array and 12 use `TYPE`; of the 404 upstream programs, 209 dimension arrays and 193 use `TYPE`
   (`study\20` §2). The corpus made control flow look like the big unlock; the upstream "x of 279" says arrays
   are.
2. **Step 6 designs `Ty` as a type table without the types it is for.** The table is motivated by unsigned types,
   fixed-length strings and user types (`study\20` §3.8). Designing it before any `TYPE` exists repeats the
   situation the running design already flags for places (`Op::Assign { place: VarId }` cannot name an element or
   a member; design, Risks).
3. **Step 7 writes 249 table-driven built-ins over a `Value` model that step 8 may reshape.** Even without the
   array-taking built-ins, an element passed by reference (`Arg::Ref(VarId)` today) is a place.
4. **The IR review will be asked "keep or merge" while the strongest evidence is missing.** `study\20` wanted it
   after arrays and `TYPE`; `study\23` moved it earlier and accepted that the place question stays open. That is
   fine for the jump model, but the answer on *shape* is then provisional, and steps 6 and 7 would build on it.

This is the compiler engineer's dissent of `study\20` §4 (arrays and `TYPE` decide the type and place model). It
was overruled for control flow specifically; the condition that justified that (control flow is small and nearly
every program needs it) does not apply to what comes next.

**Decision (user, 2026-10-07):** after the IR review, a **minimal arrays-and-`TYPE` slice** comes before step 6:
static `DIM a(n)` of scalars, element read and write, an element passed by reference, `LBOUND`/`UBOUND`, a `TYPE`
with scalar members, member read and write. Not in the slice: `REDIM`, dynamic arrays, `OPTION BASE`, member
arrays, whole-array assignment (`SOMEDAY.md`). The slice forces a place notion into the IR and user types into
`Ty`; steps 6 and 7 then target a stable model. Steps 4 and 5 (parser breadth, thin language server) do not touch
the IR and keep their places. The IR review's open place question is closed by this slice, not by step 8.

## 3. The one process change: `STATUS.md` had become a journal

Step 3 of `STATUS.md` "Next" was a 40-line paragraph of task-by-task history that `tasks.md` of the change (its
"Done" notes) and `git log` already hold. The actual instruction for the next session was its last sentence. This
matters here more than in most projects: the documents are the hand-over from the path-setting sessions to the
implementing ones (about 16,000 lines of prose against 14,500 of Rust on 2026-10-07), so their value depends on
the entry point being sharp.

**Decision (user, 2026-10-07):** `STATUS.md` "Next" holds one line per step; the current step names the change
and the task; the per-task narrative lives in the change's `tasks.md` ("Done" and "As built" notes, as already
practised) and in `git log`, not in `STATUS.md`. The "State of `m2-parser-breadth`" block is reduced the same
way. Nothing else about the documentation changes.

## 4. Order of work (accepted 2026-10-07, replaces `study\23` §4)

1. Done: CI green (`rust.yml` with `tier2`, `repo-check.yml`, `vscode-extension.yml`).
2. Done: `m2-parser-breadth` groups 5 and 6.
3. **In progress:** `m2-control-flow-slice`, tasks 6–9 (IR with jumps, emitter, lists, documents). Then the IR
   review (jump and error model; "keep or merge" provisional until step 4).
4. **New:** the minimal arrays-and-`TYPE` slice (§2), its own OpenSpec change, measured first as the other slices
   were. It closes the IR review's place question.
5. `m2-parser-breadth` groups 7 to 9 (statements, `specialformat` templates, `$IF`, `$INCLUDE`, the follow-on
   rule, block crossing marked). May run before 4 if a session wants parser work.
6. The thin language server (`study\23` §2.6).
7. Bug-compatibility decisions (`study\00` §6), the differential tester, `Ty` as a type table (now with user
   types in hand), unsigned types.
8. Plain built-ins (249 of 455), table-driven, with generated tests.
9. The rest of control flow (`SELECT CASE`, `ON … GOTO/GOSUB`, `DEFxxx`), then the rest of arrays and `TYPE`
   (`REDIM`, dynamic arrays, `OPTION BASE`; member arrays stay in `SOMEDAY.md`).

Queued as before: M3 windowed programs with a screen-state oracle; M4 `qb64pe.bas` through its include files.
