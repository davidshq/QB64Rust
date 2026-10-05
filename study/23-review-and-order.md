# 23. Third review: path check, panel, and the order of work (2026-10-05)

A review of the codebase as a software architect and pragmatic engineer would read it ("are we on the right path,
any major error"), then the panel (`CLAUDE.md` rule 6) with one extra role, an editor-tooling engineer, because
one finding is about the language server. Accepted by the user 2026-10-05. §4 replaces the order of work of
`study\22` §5.

Read for the review: `STATUS.md`, `crates\README.md`, the core of every crate (`tree.rs`, `program.rs`, the
parser's `mod.rs`, `lexer.rs`, `sema`'s `lib.rs` and `check.rs`, the `ir` crate, `codegen-cpp`, the driver's
`main.rs`), the design and tasks of `m2-parser-breadth`, `study\20` and `study\22`. Not read in full: `vscode\`,
`ast.rs`, the parser's family modules, the tests. No test was run for the review.

## 1. Verdict

The direction is sound and nothing built so far is a major error. The main risk was the order of work: the
hardest back-end questions (control flow, the IR's shape) came last, and one part of the current design is known
not to survive them.

What is right: bytes throughout, the lossless tree, one crate per stage, typed accessors; the old compiler as
oracle (measurements, the "not supported yet" marker, the shrink-only lists), which makes "never wrong code"
checkable; `sema` checking in file order with the `ty`/`qb` pair; reusing `qbx.cpp` and libqb through fragments.

## 2. Findings and the panel's outcome

### 2.1 Control flow reaches the pipeline too late: accepted, widened
Control flow was step 6 of `study\22` §5, after the rest of parser breadth, the language server, the differential
tester, the type table and 249 built-ins. Until then the end-to-end numbers stay at 54 of 275 and 10 of 279 and the
IR stays unvalidated: it is still a 1:1 mirror of the typed tree (`crates\ir\src\lower.rs`, every `Stmt` holds one
`Op`), and a label is "a position before statement N of a flat list" with "the IR has no jumps"
(`crates\ir\src\lib.rs`), which cannot express a label inside an `IF` or `FOR` body. Not tested at all: `RESUME
NEXT` after an error in an `IF` condition or a `FOR` header, `GOSUB`/`RETURN`, labels inside procedures.

Panel: all agree. The reason `study\22` gave for holding the back end (remove the single-file assumption first) is
met since group 4 of `m2-parser-breadth`. The QB64 engineer adds `CONST` and `OPTION _EXPLICIT` to the slice
(`OPTION _EXPLICIT` gates 187 upstream programs, `study\22` §3.3; the auto-included files are mostly `CONST`). The
test engineer asks for a count first: how many corpus and upstream programs are blocked only by these constructs.

**Outcome:** once block nodes exist (group 6), a control-flow slice goes through to C++ as its own change, before
groups 7 and 8. The IR review (`study\20` §3.4: keep or merge into the typed tree) is held right after that slice,
not after arrays and `TYPE`.

### 2.2 Exit criterion of parser breadth: withdrawn
The review called "both lists empty over 673 files" a big-bang criterion. The test engineer: task 8.3 already says
that anything that does not fit "is decided with the user, not left in the lists", and an empty shrink-only list is
the invariant worth keeping. No change.

### 2.3 Follow-on errors (design D10 of `m2-parser-breadth`): revised
D10 tracks the names of every unsupported declaration (`DIM`, `CONST`, `TYPE`, `DEFxxx`, `DECLARE LIBRARY`, ...)
as *unknown* so that their uses report nothing. That is scaffolding: it exists only because `sema` lags the
parser, serves about 50 of the 181 false-error programs, and goes away as each feature is compiled.

Panel: split. Pragmatic and Rust engineers: one flag instead of seven kinds of tracking. Test engineer (dissent,
left standing): a blunt rule can hide a real error in a program the old compiler rejects, which weakens the `.err`
verdict.

**Outcome:** the blunt rule first. After the first declaration reported "not supported yet" (in file order), `sema`
reports only "not supported yet" errors for the rest of the program; real errors found after that point are
dropped. Parser errors are not affected. The lists are regenerated; name tracking is added only for a declaration
kind whose new entries in `tests\known_unsupported_rejections.list` the user finds unacceptable on review. The
"old compiler knows this name, we do not yet" part of D10 (auto-include names, `_GL`, built-in assignment targets)
stays as designed.

### 2.4 Blocks that cross an include boundary: accepted
The old compiler accepts a `FOR` closed by a `NEXT` in an included file (`study\00` §5). The design said D9 "must
handle" it and gave no mechanism; a block node cannot span two trees. QB64 engineer: real code includes whole
declarations at the top and whole procedures at the bottom, a split block is rare.

**Outcome:** a block still open at the end of its file, or a closer without an opener in its file, where the old
compiler would pair them across the include, is "not supported yet" (never a syntax error). The lists show whether
any input needs more.

### 2.5 `Ty` as an enum ordered by `derive`: accepted, no change of plan
Said by both earlier reviews. The type table keeps its place before the built-ins; nothing with unsigned types,
fixed-length strings or `TYPE` goes into `sema` before it.

### 2.6 The thin language server: revised
Symbols are recorded only for statements without an error, and most statements of a real program are "not
supported yet" for a while, so go to definition on variables shows little. Editor-tooling engineer: the early value
needs no `sema` coverage.

**Outcome:** the thin server gives syntax errors, the document outline and folding ranges from the block nodes, and
go to definition for procedures and labels (resolved before the statement pass). The extension's other diagnostics
stay on the old compiler.

### 2.7 Added by the panel
- **CI is unproven** (test engineer): the workflows `rust.yml` and `repo-check.yml` have never run on GitHub, and
  the `tier2` job has not been seen green (`STATUS.md`). Push and confirm before more work depends on them.
- **Windowed programs** (QB64 engineer): requiring `$CONSOLE:ONLY` leaves out most real QB64 code and 47 upstream
  programs until M3's screen-state oracle (`study\22` §3.2). Not urgent; it caps "x of 279" whatever else is done.

## 3. Smaller structural points

To fix as the code grows, not decisions:
- The parser's `word_statement` is a chain of case-insensitive compares and `sema`'s `statement` a chain of casts;
  both become one lookup or a `match` on the kind before tens of statement kinds arrive.
- `crates\sema\src\check.rs` (1,443 lines for a small subset) is split by family, as the parser is.
- `STATUS.md` has grown back into a history; "one home per fact" (`study\20` §3.7) still holds.

## 4. Order of work (accepted 2026-10-05, replaces `study\22` §5)

1. Push; see `rust.yml` (with `tier2`), `repo-check.yml` and `vscode-extension.yml` green.
2. `m2-parser-breadth` groups 5 and 6: member access, `DATA`, line numbers, blocks.
3. **A control-flow slice through to C++** (its own OpenSpec change): `IF`, `FOR`, `DO`, `WHILE`, `GOTO`, `GOSUB`/
   `RETURN`, `CONST`, `OPTION _EXPLICIT`, with labels inside blocks and procedures and the error-handling cases of
   §2.1. First a count of the programs blocked only by these. Then the IR review.
4. `m2-parser-breadth` groups 7 to 9: statements, `specialformat` templates, `$IF`, `$INCLUDE`, the blunt
   follow-on rule (§2.3), block crossing marked (§2.4).
5. The thin language server, scoped as in §2.6.
6. Bug-compatibility decisions (`study\00` §6), the differential tester, `Ty` as a type table, unsigned types.
7. Plain built-ins (249 of 455), table-driven, with generated tests.
8. The rest of control flow (`SELECT CASE`, `ON … GOTO/GOSUB`, `DEFxxx`), then arrays and `TYPE`.

Queued as before: M3 windowed programs with a screen-state oracle; M4 `qb64pe.bas` through its include files.
