# Design

## Context

See `proposal.md` for why. What shapes the approach:

- **The C++ is written as text.** `Emitter::value` (`crates\codegen-cpp\src\value.rs`) is the one function that
  turns an IR expression into C++. It calls itself for each part and joins the texts with `format!`. Places go
  through `load_place`, `place_ref` and `bytes_of` (`place.rs`). Statements go through `Emitter::op`
  (`stmt.rs`), which calls a writer per kind. Most of these write the call the old compiler writes, with its
  arguments in the old compiler's order.
- **QB64pe compiles with `-std=gnu++20`** (`..\QB64pe\Makefile` line 244). C++20 fixes these orders:
  - `&&`, `||`, `?:` and the comma operator: left first, and the right side only when needed;
  - `a = b` and compound assignment: `b` before `a`;
  - a function call: the function before its arguments.

  These are left open:
  - the arguments of one call, relative to each other;
  - the two operands of `+`, `-`, `*`, `/`, `&`, `|`, `^`, `==` and the other arithmetic and comparison
    operators.

  Clang evaluates the open ones left to right. `g++` on x86-64 often evaluates call arguments right to left.
- **Errors are pending and the first one wins.** libqb's `error()` records a number only when none is pending
  (`..\QB64pe\internal\c\libqb\src\error_handle.cpp`, `if (!new_error)`). A user procedure entered while an
  error is pending returns at once, and many libqb functions return a placeholder while one is pending. So two
  parts that only raise errors still give another result in another order. `verification\v18_c_byref` shows it:
  `two x(9), ASC(empty)` reports error 9 on Windows and error 5 on Linux.
- **The IR already states the orders it relies on** (`crates\ir\src\lib.rs`, module documentation): "a built-in
  statement evaluates its arguments in order", the indexes of an element store before its value, and others. The
  emitter writes some of these as separate C++ statements, which are already ordered. Others are arguments of one
  call, which Linux breaks.
- **`SWAP` is the first case** (`builtins.rs`, `StmtRule::Swap`; `DIVERGENCES-QB64PE.md` D-054). When an index of
  either place calls a FUNCTION, both places go into `auto` variables in the call's order. Otherwise the C++ is the
  old compiler's.
- **What the Linux runs found** (`proposal.md` lists the programs): `_ASSERT`'s condition against its message
  (`s106`, `s108`, `v33_a03`, `v33_a04`); `FIELD`'s file number, width and target index (`s86` line 17,
  `v30_k`); `LSET`'s target index against its value (`s86` line 27, `v30_l`); `_MIN`, `_MAX` and `_CLAMP`
  arguments (`s78` line 17, `v30_g`); a SUB's arguments that raise errors (`s22` line 87, `v18_c`).
  `s86` line 20 follows from line 17: the record was laid out with the other width.

## Goals / Non-Goals

**Goals:**

- Every part of a group whose order the C++ leaves open is evaluated in the order the Windows build evaluates it,
  on every platform, wherever the order could change the result.
- One rule in one module. Each IR expression kind and statement kind is classified in an exhaustive `match`, so a
  new kind does not compile until it is classified.
- The C++ of a group stays as it is unless two of its parts conflict. Programs without such groups compile to the
  same C++ as before.

**Non-Goals:**

- Evaluating the indexes of a whole `TYPE` copy once instead of once per part (`SOMEDAY.md`, the row on
  `a(one(1)) = b(one(2))`). That is how often an index is evaluated, not in which order. This change keeps it.
- Changing an order that C++20 already fixes, even where it is not left to right. A number stored through a
  member with an index on the way is evaluated before that index on Windows and Linux alike, and stays so.
- Other differences between QB64pe's Windows and Linux builds: console flushing (`s12_error_in_print`), `$IF WIN`
  (`s136_errorline`), NaN text (`v30_g`'s other lines).
- A run-time order for `$CHECKING:OFF` programs beyond what follows from the same rule. Without checks an array
  index raises nothing, so fewer parts conflict.

## Decisions

### D1. A group is the parts the C++ writes into one unordered construct

A **group** is the parts of one IR node that the emitter writes as the arguments of one C++ call, or as the
operands of one operator that C++ leaves unordered. Its parts are listed in the order of the C++ text, which is
the order Clang evaluates them in, and so the Windows build's order. That is often not the BASIC order:

- `FIELD #fn, wd AS e(ix)` evaluates `fn`, then `ix`, then `wd` on Windows (`v30_k` line 30), because the old
  compiler's call takes the place before the width.
- `SWAP` of two whole `TYPE` values passes the second place first.

The groups, by kind:

| IR node | Group | Why |
|---|---|---|
| `ExprKind::Binary`, except `AndAlso`, `OrElse` | `lhs`, `rhs` | `+`, `-`, `*` and the rest leave operand order open; `qb_safe_idiv`, `qb_safe_mod` and `pow2` are calls |
| `ExprKind::Binary` with `AndAlso` or `OrElse` | none | `&&` and `\|\|` are ordered, and the right side runs only when needed |
| `ExprKind::Concat`, `ExprKind::StrCompare` | both sides | `qbs_add` and `qbs_equal` are calls |
| `ExprKind::Choice` (`_IIF`) | none | `?:` is ordered, and only one branch runs |
| `ExprKind::Call` (a built-in function) | its arguments, in the C++ text's order | one libqb call, or an inline template whose parts are operands |
| `ExprKind::CallProc` (a FUNCTION, a library function) | its arguments | one call |
| `ExprKind::MemAt`, `MemRead`, and the other kinds with two or more evaluated parts | their parts | one call each |
| A place's indexes (`flat_index`) | the indexes of one element | they are joined with `+` and `*` |
| `Op::Call` (a SUB), `Op::Builtin` written as one call, `Op::Field` (each `field_add`), `Op::Lset`, `Op::MemFill`, `Op::OnEvent`, `Op::Key` | the call's arguments | one C++ call statement |
| `Op::Log`, `Op::PaletteUsing` | none | one part evaluated: `Op::Log`'s message; `PALETTE USING`'s array bytes, beside a constant |
| `Op::CallAbsolute` | none | each argument's offset is a C++ statement of its own, then the call with the offset alone |
| `Op::MemGet`, `Op::MemPut` | none | the offset goes into `tmp_long` as a statement of its own, then the copy |
| `Op::Assign` of a string (`qbs_set(target, value)`) | the target, the value | one call |
| `Op::Assign` of a number | none between target and value | an element's index is a statement of its own, written first; through a member, C++20 evaluates the value first, on both platforms. The indexes inside the target are still a group |
| `Op::Print`, `Op::Write`, `Op::Input`, `Op::Read`, an element store with its pending-error check | none across items | the emitter writes each item or step as its own C++ statement |

The table lives as two functions in a new module, `crates\codegen-cpp\src\order.rs`: `expr_group(&Expr)` and
`op_group(&Op)`. Each is a `match` with no wildcard arm. A built-in statement's rule (`StmtRule` in `builtins.rs`)
says whether its arguments are one call (`Plain`, `Graphics`, `File`, `Swap`) or several statements (`Close`,
which writes one `sub_close` per number).

*Instead of* a rewrite of the IR before the emitter (a lowering pass that adds temporaries as IR statements): an
IR statement is an error boundary, where a pending error is serviced. A part moved into a statement of its own
would service its error before the rest of the original statement runs, and `RESUME NEXT` would then skip
something else. A part inside one branch of `_IIF` cannot be moved to a statement at all. The IR is also
ABI-neutral and does not know which of its nodes become one C++ call.

### D2. What a part does: changes state, raises, or reads

Each part is classed by what can be seen of it:

- **Changes state**:
  - a call of a user FUNCTION or of a `DECLARE LIBRARY` procedure;
  - a built-in on a list of built-ins that change program state, such as `RND`, `INKEY$`, `INPUT$`, `EOF` and
    the other reads of files and devices, `_MOUSEINPUT`, `FREEFILE`, `_NEWIMAGE`, and the `_MEM` built-ins that
    allocate;
  - a part that contains any of these.
- **Raises**:
  - an element read with run-time checks on (error 9);
  - `\` and `MOD` (division by zero), `^`;
  - a built-in that is not on a list of built-ins that cannot raise;
  - a part that contains any of these.
- **Reads**: anything that is not a constant. A variable, an element and a built-in all read state.
- **Constant**: a number or a string literal.

The lists are names, held in `order.rs`. Every built-in counts as raising unless it is on the "cannot raise"
list. Only the built-ins on the "changes state" list count as changing state, and that list is read against
libqb's sources (user, Q2, 2026-11-07). *Instead of:* treating every built-in as changing state, which changes the
C++ of far more programs (`x$ + LEFT$(a$, 1)` would hold `x$`); or classifying all of the roughly 600 built-ins
one by one, which is mostly re-reading code to confirm "raises".

### D3. When two parts conflict, and which parts are held

Two parts of a group **conflict** when another order could change what the program does:

- one changes state and the other reads; or
- both raise, unless both can raise only error 9 from index checks (user, Q1, 2026-11-07). Two such parts give
  the same error number and the same element 0 in either order, so `a(i) + b(j)` keeps its C++.

A part is **held** when it conflicts with a later part of its group. Holding evaluates it into a temporary
before the group, in the group's order. The last part is never held.

`f(1) - f(2)` holds `f(1)` only. `x + f(1)` always holds `x`, because `f` may change `x` (user, Q3, 2026-11-07).
*Instead of:* checking whether `f` can change `x` through SHARED, STATIC or by-reference parameters. That is more
code and more ways to be wrong, and QB64 programs often change SHARED variables in FUNCTIONs anyway.

An order C++20 already fixes is kept as Windows has it, even where it is not left to right (user, Q4,
2026-11-07). A number stored through a member with an index on the way is one C++ assignment, whose value C++
evaluates first on both platforms, so `r(f(1)).n = f(2)` runs `f(2)` first. A store into an element of an array
writes the index as a statement of its own, so `a(f(1)) = f(2)` runs `f(1)` first. `v69_j_assign` records both
(corrected by task 2.2: this paragraph first said `a(f(1)) = f(2)` ran `f(2)` first).

This is enough. Take a part that is not held and a later part that is: the two do not conflict, so evaluating the
later one first changes nothing. Take two held parts: they are evaluated in order.

### D4. How a hold is written: a comma expression at the node

A hold is a temporary declared with the body's other temporaries (`pass_decls`, as `passN` is) and assigned in a
C++ comma expression placed at the node itself:

```cpp
// PRINT f(1) - f(2)
(seq1=FUNC_F(&(pass1=1)), (seq1-FUNC_F(&(pass2=2))))
```

The comma operator is ordered on every C++ compiler, and the expression stays where the node was. A node inside
one branch of `_IIF` or on the right of `AND ALSO` holds its parts only when that branch runs. No pending-error
check comes between the temporary and the node, so an error a held part raises is raised at the same point as on
Windows, and the rest of the node runs as it does there. A statement's group is written the same way, as
`seq1=…; seq2=…;` lines just before the statement's one call. D1 allows that only for statements written as one
call.

The emitter finds a held part through two maps on `Emitter`:
- `held_exprs`, from the address of an IR `Expr` to its temporary's name, checked first in `value`;
- `held_places`, from the address of an IR `Place`, checked first in `load_place`, `place_ref` and `bytes_of`.

`Emitter::with_holds(group, write)` plans the holds (D3) and writes each held part's text into its temporary in
order. It then calls `write` to build the node's text, which finds the temporaries through the maps, removes the
map entries, and wraps the text. `value` calls it for every expression and `op` for every statement.

*Instead of* changing each writer to take its parts as texts: there are more than a hundred writers, and a missed
one would go unnoticed. The maps let every writer stay as it is.

### D5. A place is held as its position

A place is held as the position it resolves to:
- an element of an array variable as its flat index (`int64`), computed by `flat_index` with its `array_check`s;
- a member, or an element reached through a member, as the address of its bytes (`bytes_of`).

The index checks, and their error 9, then happen where the old compiler's Windows C++ has them, in the
argument's turn. `load_place`, `place_ref` and `bytes_of` build the place from the held position, so loads,
references and string slots all work from one temporary. A `_BIT` element is held as its flat index too.

`SWAP`'s own `auto swap_x`/`swap_y` code goes, and `SWAP` becomes an `op_group` row. Its C++ for places with a
FUNCTION in an index changes from the two `auto` variables to the general form. Its behavior does not.

### D6. Effects and the temporaries' types

A held value's temporary has the C type of the part (`c_type`). A string is held as its `qbs*`. That is what the
call is given anyway, so later changes to the same string are seen as on Windows, where the pointer is passed and
read inside the call. A whole `TYPE` is held as its address, and a whole array argument is a descriptor name,
which is never held. The temporaries are named `seqN` from the emitter's counter for `passN`, so they cannot clash
with a program's names (`names.rs` mangles those).

### D7. Tests

- **Unit tests** in `crates\codegen-cpp`:
  - each group kind gives the expected holds;
  - a group without a conflict gives the old C++, byte for byte;
  - `_IIF` and `AND ALSO` hold nothing across a branch;
  - `order.rs`'s lists name only built-ins that exist in `builtins.json`.
- **A new verification family, `verification\v69_*`**, recorded with `qb64pe.exe` on Windows. One program per
  kind of group, each printing which calls ran in which order:
  - the operands of an operator, including `\`, `MOD`, `^`, `+` of strings and a string comparison;
  - the arguments of a built-in function, nested built-ins, a FUNCTION and a library procedure;
  - SUB arguments by reference, whose index checks raise errors (as `v18_c`);
  - built-in statements written as one call, such as `FIELD`, `LSET`, `_ASSERT` with a message, `LINE` and
    `PSET` with FUNCTION coordinates, and `PUT` with a FUNCTION record number;
  - a string assignment through a member, and a numeric one, to show that the value comes first;
  - `_IIF` and `AND ALSO` with FUNCTIONs in both branches, which must stay conditional;
  - errors in two parts with different numbers, to show which one is reported;
  - places no existing program reaches: `MID$(s$, f(1), f(2))`, `INSTR(f(1), a$, b$)`, `_RGB32(f(1), f(2), f(3))`,
    `a(f(1), f(2))`, `CHR$(f(1)) + CHR$(f(2))`.

  The family goes on `tests\verification\pass.list`. Its Linux run must match the Windows recordings with no
  sidecar.
- **The sidecars** of `proposal.md`'s list are deleted. Those programs must pass on Linux against their Windows
  recordings. `v30_g`'s sidecar is cut to its NaN lines instead of being deleted. `s12` and `s136` keep their
  sidecars, after a Linux run shows they still differ only for the reasons in "Non-Goals".
- **How many programs' C++ changes**, measured once with `-z` over the corpus and the verification programs,
  before and after. The count goes in "Found while building", so that a rule much wider than expected shows up.

### D8. Records

- A `DIVERGENCES-QB64PE.md` row: on Linux, where QB64pe's build evaluates parts of one expression or call in
  another order, QB64Rust follows the Windows order. D-054 points to it as its first case.
- A `DECISIONS.md` row with the answers to the open questions.
- Removed: the `SOMEDAY.md` row on the order of calls with side effects.
- `ROADMAP.md`'s "After M7" item ticked, and `STATUS.md`'s "Next" updated.
- `verification\README.md`'s line on `v18_c`, `v30_k`, `v30_l`, `v33_a03`, `v33_a04` and `v30_g` rewritten.
- The specs: `language/evaluation-order` (new) and `compiler/platforms` (modified), merged when the change is
  archived.

## Risks / Trade-offs

- [A writer reads a part's text, for example `bare` strips its parentheses, and gets a temporary's name instead] →
  A name is a bare token, so `bare` and `inner` leave it alone. The unit tests compile each group kind, and `g++`
  and Clang build the `v69` family.
- [A built-in left off the "changes state" list → a conflict missed, and that program still differs on Linux] →
  The default is "raises", and the list is read against libqb's sources for every built-in that keeps state. A
  mistake shows in the Linux runs as one program, not as a wrong result on Windows.
- [The rule holds more than needed → the C++ of more programs changes] → No change in behavior on Windows. D7
  measures the count. The answers to Q1–Q3 keep the rule narrow.
- [A part is held whose evaluation the old compiler skips] → D1 lists no group across `&&`, `||`, `?:` or the
  emitter's separate statements, and `with_holds` wraps at the node, inside any branch.
- [The value-trace and coverage builds (`trace.rs`, `coverage.rs`) record calls around `value`] → They wrap the
  held part's text when it is written into its temporary, so they record in the new order, which is Windows'.
  The `value-trace` tests run in the close.

## Open Questions

None. Q1–Q4 were answered by the user on 2026-11-07 (`DECISIONS.md`), as written in D2 and D3.

## Found while building

- **How many programs' C++ changes** (task 1.4). The C++ of every program of `tests\corpus` and `verification`
  (4,913) was written with the parent commit's build (`2e03ba49`) and with this change's, through `--dump cpp`.
  77 programs' C++ differs. A 78th, `v37_x51`, differs only because the parent's build ran from a worktree with
  no compiler root, so its library was looked up elsewhere. None fails to compile with the new build. The 77
  include every program whose Linux sidecar this change deletes (`proposal.md`). A sample read for a real
  conflict: `n * FACT(n - 1)` (`s160_layout_procs`), `_KEYHIT = code` (`k04_keyclear`), FUNCTION indexes on both
  sides of a `TYPE` copy (`v24_x310`), two file built-ins that can each raise (`files_cpp` in `tests\frontend`).
  The C++ of the 2,650 call-site programs (`tests\callsite`) is the same as before.
- **`CHR$` of a constant from 0 to 255 is a constant.** The first count was 115 programs. Most of the extra 38
  were chains like `CHR$(0) + "x" + CHR$(0)`: `CHR$` can raise error 5, so two of them conflicted, but not for a
  constant in that range (`func_chr` in libqb's `qbs.cpp`). That one case is now classed a constant.
- **`LBOUND` and `UBOUND` raise only error 9**, as an index check does, and give 0 then. So they count with the
  index checks for Q1: `UBOUND(d, 2) + LEN(t$(n))` keeps its C++ (`dynamic_arrays_cpp` in `tests\frontend`).
- **`FREEFILE` and `EOF` are not on the "changes state" list.** D2 named them as examples. Read against libqb,
  `func_freefile` only looks for the lowest free number, and `func_eof` only reads the file's state. A part that
  changes the file, such as `INPUT$`, still conflicts with them, because they read. The list is in `order.rs`,
  with the source file of each entry.
- **The "cannot raise" list** was made by a scan of the body of each built-in's libqb entry for `error(`, then
  cut to the common functions whose whole body was read, and the ones the emitter writes inline (`LEN`, `INT`,
  `_MIN`, the rotations, C++'s own mathematics). The bit functions (`_READBIT` and its kin) are left off: the fix
  `bit-number-check` makes their bit number raise.
- **Parts the emitter cannot hold.** The bytes of an array (`ExprKind::Block`, `StmtArg::Block`, `StmtArg::Rest`)
  are written as a `byte_element` call, not a value of a C type, so they are never held. Their class still counts
  for the parts before them. Their index cannot call a procedure (`sema`), so such a part only reads and can raise
  error 9. A part of a user type is not held either: it stands only as the value of a `_MEM` store, which has no
  group.
- **The member array's first element is a part too.** `bytes_of` of an element of a member array writes the
  address of the array's first element plus the flat position, as two operands of `+`. So the base place (an
  element with its own index, `a(f(1)).v(f(2))`) and each index term are one group. `SWAP`'s `TYPE` case and the
  stores through members need this.
- **An index term is held with its check.** In `a(f(1), f(2))` the term `array_check(f(1)-…)` is held, not only
  `f(1)`. Else the first index's error 9 would come after `f(2)` runs, which a FUNCTION entered with an error
  pending skips.
- **The descriptor's reads are not counted** (found in review, 2026-11-07). Each term of an array's flat
  position reads the descriptor's bounds (`desc[4]`, `desc[5]`), and the element is reached through its data
  pointer (`desc[0]`). These change only when a FUNCTION in an index `REDIM`s the same array. `REDIM` frees the
  old storage, and C++17 reads `desc[0]` before the index of `desc[0][…]` on every platform. So the old
  compiler's program then uses freed storage on Windows and Linux alike, and there is no order to keep. It is a
  row of `SOMEDAY.md`, "QB64pe behaviours to review". A constant index before a FUNCTION index is still held
  when checks are on, because its error 9 conflicts with the FUNCTION's errors (`m(1, f(1))` in
  `tests\frontend\eval_order_cpp.bas`).
- **A hold no writer found stops the compiler** (`release_holds`). It would have evaluated its part twice. No
  program of the corpus, the verification families or the call-site set meets it.
- **A store into an element runs its index first** (task 2.2). The spec's scenario said `a(f(1)) = f(2)` prints
  `2` before `1`. `v69_j_assign` records `1` first: the emitter writes the index as `tmp_long=…;` before the
  store, as the old compiler does. Only a number stored through a member, `r(f(1)).n = f(2)`, runs the value
  first. The spec, D1 and D3 now say so. Nothing in the code changed.
- **The `v69` programs were adjusted while recording.** `POINT` in `v69_h_graphics` needed `_SOURCE` set to the
  image. `1 \ z` in `v69_l_two_errors` ended the program ("Division by zero", `DIVERGENCES-QB64PE.md` D-052:
  critical in QB64pe), so `CINT` overflow (error 6) stands in for a second error number.
- **`s86_field_lset` keeps a Linux sidecar, cut to its line 20** (task 3.1). "Context" said line 20 followed from
  line 17. It does not: with line 17 now in the Windows order, line 20 still differs. It prints the file's bytes
  after the error cases of lines 18 and 19, and the file is 32 bytes on Windows and 40 on Linux, also in QB64pe's
  Linux build (the old sidecar has the 40). That is a difference in file handling, not in order. So the sidecar
  is the Windows recording with line 20 from Linux, as `v30_g`'s is with its NaN line. The other nine sidecars
  go: their programs' Linux output now equals the Windows recording byte for byte (run in WSL on 2026-11-07).
- **The tests of C++ text are in `tests\frontend\eval_order_cpp.bas`**, a `cpp` snapshot of the driver's tests,
  not in `crates\codegen-cpp`: that crate has no front end to compile BASIC with. The unit tests of the rule
  itself are in `order.rs`.
