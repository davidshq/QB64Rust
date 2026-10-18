# 20 — Review of code and plan after the first slice, and the order of work

Written 2026-10-04 (session 10). A review of the architecture, the plan and the Rust code (about 4,300 lines, seven
crates) by a different model than the one that wrote them, followed by a panel (`CLAUDE.md` rule 6). The user
accepted the outcome; the order in §4 is the plan (`STATUS.md`, "Next").

Read for the review: `STATUS.md`, `study\00`, parts of `study\07`, the design of `m2-procedures-and-errors`, and
most of `crates\`. Not run: the tests. Not reviewed in depth: `vscode\`, `study\01`–`05`.

## 1. Verdict

The architecture and the plan are sound; no change of direction. What holds up: the old compiler as oracle with
measurements before code, the divergence register, C++ against the existing libqb ABI first, bytes throughout, our
own lossless tree, "not supported yet" instead of wrong code, the early vertical slice. `-fwrapv` does reach the
reference clone's `Makefile` (through `CXXFLAGS_EXTRA`).

The risk is the order of work: the pipeline is proven on a small flat subset only, and some things that are cheap
to change now get expensive as `sema` grows.

## 2. Measured for this review

| What | Result |
|---|---|
| Built-in table (`tools\builtins\builtins.json`) | 249 of 455 entries are plain rows (no `specialformat`, with a C entry point); 172 have a `specialformat` (statement syntax) |
| Argument slot types in the table | LONG 299, STRING 135, `_FLOAT` 67, `_UNSIGNED LONG` 39, SINGLE 36, DOUBLE 29, any-numeric 28, others fewer than 10 each |
| Upstream `tests\compile_tests` (404 programs) | 209 dimension arrays, 193 use `TYPE`, 91 `FOR`, 42 `$IF`, 39 `_MEM`, 10 `$INCLUDE`, 10 `DECLARE LIBRARY` |

(Counted with regular expressions over the files; good to a few programs, not exact.)

## 3. Findings and what the panel made of them

Panel: pragmatic engineer, QB64 engineer, compiler/languages engineer, Rust engineer, test engineer, and a
language-server engineer (added because findings 1 and 5 are about what the editor needs from the tree and `sema`).

### 3.1 The parser is far behind the pipeline
The tree handles five flat statement kinds; everything else goes into an `Error` node, so the corpus round-trip
test passes trivially. Not designed yet: `$INCLUDE` (files that are not parsable alone), `$IF` regions,
metacommands inside comments (`'$INCLUDE:`, `'$DYNAMIC`; the lexer makes `'` comments trivia), single-line `IF`,
`DATA`, line numbers. The roadmap's M2 was "the whole front end before code generation"; the work has in fact
grown as vertical slices.

**Outcome: accepted, split in three.** (a) blocks, control flow, labels, `DATA`, comment metacommands; (b) `$IF`
and `$INCLUDE`; (c) the 172 `specialformat` statement syntaxes. Metric for each: no parse error on programs the old
compiler accepts (upstream tests, the corpus, `qb64pe.bas`). `$INCLUDE` model proposed: each file has its own tree,
an include node links to another `FileId`. The auto-included BASIC files mean every program has includes, so (b)
cannot wait long.

### 3.2 Built-ins are added by hand
`INSTR` has its own function in `sema`; the next change adds `CHR$`, `ERR`, `ERL` the same way. The table holds
types as strings.

**Outcome: accepted.** A generic table-driven call path for the 249 plain rows (typed slots, optional arguments,
functions and subs); type names become an enum in `build.rs` so an unknown name fails the build; one generated
call per supported entry is compared with the old compiler. Parsing `specialformat` (R9) is a separate change.

### 3.3 Numeric typing is hand-derived, without the differential tester
The `ty`/`qb` pair is a good finding, but promotion relies on the derive order of `Ty`, which stops working with
unsigned types, `_BIT`, `_BYTE` and `_OFFSET` (C++ promotion is not a total order). Test layer 3 of `study\07`
(differential testing of expressions) does not exist yet.

**Outcome: accepted, raised in priority.** Build the tester before the unsigned types, which arrive with the
built-ins (39 `_UNSIGNED LONG` slots). Start with every pair of types per operator, not random expressions; seeded
and deterministic; programs that found a difference go into the corpus. Replace the ordering trick by an explicit
table checked by the tester.

### 3.4 The IR mirrors the typed tree
`crates\ir\src\lower.rs` is a 1:1 copy with duplicate `Ty`, `BinOp` and `Conv` enums; its ABI-neutral purpose serves
M6.

**Outcome: softened.** Keep the IR: arrays, `TYPE` members and by-reference arguments need a notion of *place*
that the typed tree should not carry. Share the duplicated enums from one crate now. Review after arrays and
`TYPE`: if lowering still does no real work then (by-value temporaries, PRINT desugaring, loop expansion,
places), merge the IR into the typed tree.

### 3.5 `sema` is coupled to the raw tree and keeps nothing for a language server
`sema` reads children by position with `unwrap()`; its output has no declaration spans and no map from a token to
its symbol.

**Outcome: accepted.** Thin hand-written typed accessors over the tree; a symbol side table (definition span,
reference spans, type), added with the procedure work because that change introduces scopes. The salsa-style
incremental analysis of `study\07` is dropped: resolution is position-dependent and whole-program and files are
small, so a full reparse and recheck with cancellation is enough.

### 3.6 Progress metric and CI
Tier 2 follows QB64Fresh's programs, written against a different idea of the language. The `.err` rejection test
proves little while nearly everything is rejected as "not supported yet". There is no CI workflow for the Rust
workspace, although tier 3 (`study\19`) is "in CI on each push".

**Outcome: partly accepted.** Report two numbers: the corpus (week-to-week progress) and the upstream
expected-output tests (real coverage; about half need arrays or `TYPE`, so alone it would stay near zero for
long). "The new compiler compiles `qb64pe.bas` and the result passes the suite" becomes the M4 exit criterion, not
a stretch. Diagnostics get a "not supported yet" marker, and the `.err` test counts a program as rejected only by
an error without it. Rust CI now.

### 3.7 Process weight
About 15,500 lines of markdown against 4,300 of Rust; measured facts are restated in `STATUS.md`, `study\00` §5,
design documents, the corpus README and `CLAUDE.md`.

**Outcome: accepted with a limit.** One home per fact (preferably a corpus or `verification\` program plus one
line); `STATUS.md` holds the phase, the next step and links. The measurement tables in design documents stay.

### 3.8 Added by the panel
- Arrays and `TYPE` are the largest unlock (about half of the upstream suite) and stress the type, storage and
  place model hardest. The first draft of the order left them out.
- `Ty` must become an interned type table before unsigned types, fixed-length strings and user types.

## 4. Order of work (accepted 2026-10-04)

1. Finish `m2-procedures-and-errors`; add the symbol side table and typed accessors inside it.
2. Rust CI workflow; shared `Ty`/`BinOp`/`Conv`; the "not supported yet" marker on diagnostics.
3. Parser breadth (a): blocks, control flow, labels, `DATA`, comment metacommands.
4. Differential tester (type pairs per operator), then the full numeric type set, with `Ty` as a type table.
5. Plain built-ins (249), table-driven, with generated tests.
6. Control flow through the pipeline.
7. Arrays and `TYPE`, then the IR review (§3.4).
8. Parser breadth (b) and (c): `$IF`, `$INCLUDE`, the `specialformat` statements.

One disagreement was left standing: the compiler engineer would put 7 before 6 (arrays and `TYPE` decide the type
and place model); the pragmatic engineer keeps control flow first (smaller, nearly every program needs `IF` and
`FOR`, and it moves the corpus number). The order above follows the pragmatic engineer.
