# Design

## Context

Order of work: `study\22` §5 step 3 (`STATUS.md` "Next", step 2). Baseline after `m2-upstream-tests`
(`tests\upstream\README.md`): 181 accepted programs with a real error, 82 rejected programs with only marked
errors, upstream progress 10 of 279.

Measured for this design (2026-10-04, `qb64rust --dump tree` over the 1,280 files tier 1 reads): **491 parse
without an `Error` node** (corpus 149 of 275, upstream 87 of 416, snippets 248 of 406, `qbasic_testcases` 2 of
143, old compiler sources 5 of 40). Files with an `Error` node, by the first word of the statement: `END` 437,
`IF` 436, `FOR` 248, `NEXT` 247, `TYPE` 239, `DIM` 221, `REDIM` 191, `ELSE` 157, `DO`/`LOOP` 130, `PRINT` 99,
`SCREEN` 98, `OPEN` 93, `SELECT`/`CASE` 92, `LINE` 87, `DATA` 78 (19,539 lines), `CLOSE` 74, `CONST` 73, `READ`
72, `PUT`/`GET` 65, `INPUT` 63, `PSET` 61, `GOTO` 58, `GOSUB`/`RETURN` 57, `WHILE`/`WEND` 53, `RESTORE` 49,
`DEFINT` 48, `ELSEIF` 44, `CIRCLE` 40, `DEF` 38, `EXIT` 30, `MID$` 26, `PLAY`, `PAINT`, `ERASE` 25,
`DECLARE` 24, `LOCATE` 23, `VIEW` 20, `OPTION` 16; plus about 70 files whose failing statement starts with a
variable (`work(0).nums(1) = 5`, member access after an index). Line numbers occur in 37 `qbasic_testcases`
files; `$IF` in 55 files; `$INCLUDE` or `'$INCLUDE` in 40.

The false errors (`tests\known_false_errors.list`, first error per program): member access after an index
(`a(i).b` in `PRINT`, arguments and parentheses) about 120; "cannot store a string in a number variable" 24 and 5
similar type errors, all after an unsupported declaration (`TYPE` member, `DIM … AS STRING * n`, `CONST`,
`DEFSTR`); 22 reserved-name errors for names the compiler does not know (`_TRUE`, `_FALSE`, `_E`, `_LOG_TRACE`
from QB64pe's auto-included BASIC files, `SUB _GL`, `_CLIPBOARD$ =`); 7 `FUNCTION … BYVAL` lines inside
`DECLARE LIBRARY`; the rest single forms (`DIM AS type a, b`, `ON ERROR GOTO _NEWHANDLER`, `f(a, , b)`).

Comment metacommands, read in `qb64pe.bas` 25426–25496 (`lineformat`): a comment (`'` or `REM`) whose first
non-blank character is `$` is a metacommand comment; in it, every `$STATIC`, `$DYNAMIC` and `$INCLUDE` (not
`$INCLUDEONCE`) is processed, anywhere in the comment and not only at a word boundary; `$FORMAT:ON/OFF` is for the
IDE only. `$INCLUDE` takes effect after the line it is on. Other text after `'$` is an ordinary comment
(`print\auto_semicolon_insertion.bas` has `'$Format:Off`).

## Goals / Non-Goals

**Goals:**
- Every program the old compiler accepts, in every input set, parses with no parser diagnostic and no `Error`
  node; `tests\known_parse_gaps.list` and `tests\known_false_errors.list` end empty.
- One tree per file, `$IF` evaluated, `$INCLUDE` followed; `sema` no longer assumes one file.
- Never wrong code: every construct `sema` does not compile is marked, as today.
- The tree is what the language server (next step) and later the formatter need: blocks are nodes, every
  statement has a kind and typed accessors.

**Non-Goals:**
- Compiling the new constructs. `sema` marks them "not supported yet"; control flow, built-ins, arrays and `TYPE`
  are later steps (`study\22` §5 steps 6–8). Exceptions, because they are what makes the tree per file real:
  `$INCLUDE`, `$INCLUDEONCE`, `$IF`/`$LET` and comment `$INCLUDE` are compiled (a program whose included and
  active code is supported compiles).
- The auto-included BASIC files (`_TRUE`, `_FALSE`, ...): their names become "not supported yet"; including them
  is a later step.
- `$NOPREFIX` (no input uses it): parsed, marked "not supported yet".
- Old compiler error texts.

## Decisions

### D1. Measure: a third list, and the definition of done
`inputs.rs` gains `tests\known_parse_gaps.list`: accepted programs (in every set, and every file of the clone's
old compiler sources) whose parse reports any diagnostic or leaves an `Error` node. Same format and rules as the
other two lists (sorted, set prefix, `#` comment with the first parser diagnostic, shrink-only,
`QB64RUST_UPDATE_LISTS=1` rewrites it). Generated at the start of the change (about 790 entries) and worked down
family by family (tasks group 4–8). The change is done when this list and `known_false_errors.list` are empty.
`known_unsupported_rejections.list` may change only by review: an entry may be added in this change only if a task
names it and gives the reason (D10 can turn a real error into a marked one); the default is that it shrinks.

### D2. Comment metacommands
The lexer gives a comment whose text after `'` or `REM` and blanks starts with `$` its own token kind,
`MetaComment`. It is not trivia: a comment always ends its line, so the parser takes it as a statement of its own
(`MetaCommentStmt`) after whatever statement precedes it on the line, and `at_stmt_end` counts it as an end. Its
directives are found by the old compiler's rule above (a function in `syntax::meta`, unit-tested against the
rule's examples, `REM $FOO $DYNAMIC` included).

First task of the change, before anything else: `sema` reports a `MetaCommentStmt` holding `$INCLUDE`, `$STATIC`
or `$DYNAMIC` as "not supported yet", which removes the wrong code found in `m2-upstream-tests` (4 upstream
programs). D9 later compiles `'$INCLUDE`; `$STATIC`/`$DYNAMIC` stay marked until arrays.

### D3. Lexer changes
- `.` after a token that can end an operand (`)`), with or without blanks on either side, followed by a name, is
  a `Dot` token, so `a(1).b` lexes as `a ( 1 ) . b`; `a(2) .b` and `a(2). b` are member access too (M8). `a.b`
  stays one `Ident` (QB64 names may contain dots; `sema` decides between a dotted name and member access, as the
  old compiler does: without a `TYPE` variable `a`, `a.b` is a plain variable, M8).
- `DATA` at the start of a statement switches the lexer to data mode: the rest of the statement becomes one
  `DataText` token (items and commas are split by the parser from the bytes, never re-tokenized). Measured (M2):
  data mode ends only at a `:` outside quotes or the line end; `'` and `REM` inside an unquoted item are data.
- A `Number` at the start of a line (not after `:`) is a line number; the lexer does not change, the parser gives
  it a `LineNumber` node. Measured (M3): decimals (`10.5`), a suffix (`10&`), numbers above 32 bits and a number
  glued to the statement (`10PRINT`) are accepted; a label may follow the number (`10 lab:`) but not precede it
  (`lab: 10` is a syntax error); a number after `:` is a syntax error.

### D4. Blocks are nodes
Every block statement becomes a node holding its header, body and closer, like `ProcDef`: `IfBlock` (branches
`IfBranch`, `ElseIfBranch`, `ElseBranch`, closer `END IF`/`ENDIF`), `IfStmt` (single-line `IF … THEN … [ELSE …]`,
including `THEN 100` and `IF … GOTO`), `ForBlock`/`NextStmt`, `DoBlock`/`LoopStmt` (`WHILE`/`UNTIL` at either
end), `WhileBlock`/`WEND`, `SelectBlock`/`CaseClause` (`CASE IS < 3`, `CASE 1 TO 5`, `CASE ELSE`), `TypeBlock`
(both field forms, `name AS type` and `AS type name, name`; element arrays parsed though deferred),
`DeclareLibraryBlock` (procedure headers with `BYVAL`), `DefFnBlock`/`DefFnStmt` (multi-line and single-line).

Recovery (the parser keeps one error per statement): the parser keeps a stack of open blocks. A closer that
matches the innermost block closes it; a closer that matches an outer block is an error at the closer ("`END IF`
without `IF`" style, wording measured in M4) when the old compiler rejects it, and the innermost block then ends
at the end of its parent; a missing closer is an error at the block's header; a `SUB`/`FUNCTION` header still ends
every open block (as now for procedures) except `$IF` entries (D8: a whole `SUB` inside an active `$IF` is
accepted). `NEXT i, j` closes two `FOR` blocks: the `NextStmt` is the closer of the
inner block, and the outer `ForBlock` records that it was closed by its child (an accessor, not a second node).
Crossing forms the old compiler accepts are written down; any it accepts that does not fit a tree is reported "not
supported yet" at the closer, never as a syntax error. Measured (M4, M7): it **rejects** every crossing tried
(`NEXT` inside an `IF` closing an outer `FOR`, `LOOP` closing a `WHILE`, `DO`/`FOR` crossed), so a block tree
fits; it **accepts** `EXIT FOR`/`EXIT DO` from inside nested other blocks, and a block opened in one file and
closed in an included one (`FOR` … `NEXT`, `SUB` … `END SUB`, M7). Decided 2026-10-05 (`study\23` §2.4): a block
node cannot span two trees, so such a block is **"not supported yet"**, never a syntax error: a block still open at
the end of its file, and a closer without an opener in its file, are marked when an include boundary lies between
them (the block is open where a file is included, or the file being parsed is an included one). The lists show
whether any input needs more. Single-line `IF`: each
`ELSE` belongs to the innermost `IF`; `ELSE IF` is an `ELSE` branch holding a new `IfBlock`.

As built (group 6, 2026-10-05; measurements added then in `study\00` §5, "Blocks, more"):
- **Every block has a header node** (`IfHeader`, `ForHeader`, `DoHeader`, `WhileHeader`, `SelectHeader`,
  `CaseHeader`, `TypeHeader`, `DeclareLibraryHeader`, `DefFnHeader`), as `ProcDef` has `ProcHeader`, so `sema` can
  tell a header with a parse error from a body that has one. `IF` and `ELSEIF` share `IfHeader`. Word closers
  (`END IF`, `ENDIF`, `WEND`, `END SELECT`, `END TYPE`, `END DECLARE`, `END DEF`) are one kind, `BlockEnd`;
  `NextStmt` and `LoopStmt` have their own kinds because they carry variables and conditions. A single-line `IF`
  is `IfStmt` (`IfHeader`, `LineBranch`, optional `ELSE` and second `LineBranch`); `THEN 10` is an `ImplicitGoto`.
- **One statement loop** (`parser\blocks.rs`, `Parser::body`) serves the main module, procedures and every block;
  it stops at a procedure header, the end of the file, a closer, or (inside a single-line `IF`) the line end and
  `ELSE`. Inside a single-line `IF` a block must close on its line (measured: a whole `FOR` there works, one closed
  on the next line is rejected), and a closer cannot close a block outside the `IF`.
- **`TYPE` and `DECLARE LIBRARY` hold no statements**: a statement inside, or the end of the file, ends the block
  with one error, and the rest of the file is parsed as usual (the old compiler stops there; a later `END TYPE`
  then gets its own "without" error).
- **`EXIT FOR/DO/WHILE/SELECT/CASE/DEF`** is checked by the parser against the open blocks of the current
  procedure (measured for `EXIT FOR`; read in `qb64pe.bas` 7440–7530 for the others).
- **`NEXT` variables** are not compared with the `FOR` variables by the parser (a name with and without a suffix
  can be one variable); `NEXT i, j` in the wrong order is left to `sema` in the control-flow slice and is only
  marked until then.
- **Labels** stand only at the start of a line, after an optional line number (measured, M3 addition); after a
  `:` a name and a colon are a call. Before group 6 every statement start took labels; no input depended on it.
- **Nesting** stops at 200 open blocks (the rest is "not supported yet"), so a pathological file cannot overflow
  the stack through the recursion.
- **`DEF FN`** parses (`DefFnStmt`, `DefFnBlock`); `sema` reports it as a real error, as the old compiler does
  ("Command not implemented"). `DEF SEG` stays "not supported yet" (group 7).

### D5. Statements: one module per family
`parser\` gets one module per family, dispatched by the first word as now (FreeBASIC lesson L1): `flow.rs`
(`GOTO`, `GOSUB`, `RETURN`, `ON … GOTO/GOSUB`, `ON TIMER/KEY/STRIG/… GOSUB`, `STOP`, `SLEEP`-like plain calls stay
calls), `blocks.rs` (D4), `decl.rs` (`DIM`/`REDIM` with bounds `a(1 TO 5, 3)`, `a()`, `_PRESERVE`, `SHARED`,
`AS [_UNSIGNED] type`, `AS STRING * n`, `AS type name`, `DIM AS type a, b`; `CONST`, `DEFxxx` and `_DEFINE`
letter ranges, `COMMON [SHARED]`, `ERASE`, `OPTION BASE/_EXPLICIT/_EXPLICITARRAY`, `STATIC` header suffix;
`CONST` and `OPTION` moved to `m2-control-flow-slice` D2 on 2026-10-06),
`data.rs` (`DATA`, `READ`, `RESTORE`), `io.rs` (`PRINT [#n,] [USING fmt;]`, `LPRINT`, `WRITE`, `INPUT [;]
["prompt"{;|,}] vars`, `INPUT #`, `LINE INPUT`, `OPEN` both forms, `CLOSE`, `GET`/`PUT` file forms, `FIELD`,
`SEEK`, `NAME … AS`), `assign.rs` (`LET`, `MID$(…) = `, `LSET`/`RSET`, `SWAP`, and assignments to built-in
"variables" such as `_CLIPBOARD$ = ` as their own node), `meta.rs` (D8). Every new statement gets a node kind and
accessors in `ast.rs` (every child an `Option`, as the existing ones).

### D6. `specialformat` statements are parsed by their template
The parser cannot otherwise tell `LINE (0, 0)-(9, 9), , BF` from an expression list, and the old compiler reads
these statements by the template too (`seperateargs`, `study\02` §4.3). So:
- `crates\builtins\build.rs` parses each `specialformat` string once into a grammar value (`?` an argument, `{A|B
  C}` word alternatives, `[…]` optional, punctuation literal) and emits it in the table; a malformed template fails
  the build. Unit tests over all 172 templates (each parses, and prints back to its string).
- **New dependency `syntax -> builtins`** (data only; `builtins` depends on nothing). The parser, at a statement
  whose first word is a built-in SUB with a template, matches the template: arguments are expressions, words are
  matched by text (so `B`, `BF`, `STEP` are words there, as in the old compiler). Names with several entries
  (`GET`, `PUT`, `OPEN`, `VIEW`, `SCREEN`) try them in the order of the table; the first full match wins. The node
  is `BuiltinStmt`: the name, `FormWord` nodes, punctuation tokens, `FormArg` nodes. `sema` maps children to slots
  by running the same matcher over the node.
- No template matches: a real syntax error at the first token no alternative could take (the old compiler rejects
  it too; M5 checks a sample).
- Built-in SUBs without a template and user SUBs keep the generic `CallStmt`. Built-in functions with a template
  (74, only `?`, `[`, `]`) need no parser support: `ArgList` allows omitted arguments (`f(a, , b)`), its accessor
  gives one `Option<Expr>` per position, and `sema` checks them against the template.
*Alternative rejected:* a template-free "loose argument list" in the parser with matching in `sema`. Simpler
dependency graph, but the tree would be wrong for `(a, b)-(c, d)` and `STEP`, and every consumer (language server,
formatter) would need the matcher.

### D7. Expressions
Postfix member access after an index or call: `FieldExpr` (`a(1).b`, `a(1).b(2).c`); omitted arguments (D6); the
remaining forms found by the lists (keyword-named functions such as `TIMER`, `SCREEN(…)`, `INPUT$(…)`, `PLAY(n)`
already parse as names). No precedence change.

### D8. Metacommands and the preprocessor
`MetaStmt` keeps its one token (the line), and `syntax::meta` splits it into name and argument text with the old
compiler's rules (read in `qb64pe.bas`, task 7.1). `$IF cond THEN`, `$ELSEIF cond THEN`, `$ELSE`, `$END IF`, `$LET
name = value`, `$ERROR text` are **evaluated in the parser**, in file order, with a `PpState` (the `$LET` names and
the predefined ones: the condition grammar and the predefined names, such as `WIN`, `WINDOWS`, `LINUX`, `MAC`,
`32BIT`, `64BIT`, `VERSION`, are read from `qb64pe.bas` and checked with M6). The target is Windows 64-bit. An
inactive branch becomes one `InactiveCode` node holding its tokens (lexed, not parsed, so lossless); nested
`$IF`s inside it are counted to find its end. `$IF` lines are statements, not block nodes. An active `$ERROR` is
a real error.

Changed by M6 (2026-10-05): the design said a `$IF` may split a block (`$IF WIN THEN` / `IF a THEN` / `$ELSE` /
`IF b THEN` / `$END IF` / … / `END IF`). The old compiler rejects that ("END IF without IF"), even with a single
header inside an active `$IF`; a `$IF` inside a block is fine. The cause (`qb64pe.bas` 3430–3436): `$IF` pushes an
entry on the block stack and `$END IF` pops the top entry, whatever it is. Measured in the review (2026-10-05):
an `IF` opened before a `$IF` and closed inside it is rejected too, a whole `SUB` inside an active `$IF` is
accepted, and `EXIT FOR` works from inside a `$IF` inside a `FOR`. So the parser does the same: an active `$IF`
is an entry on its block stack, and a closer that does not match the top entry (a block closer meeting the `$IF`
entry, or `$ELSE`/`$END IF` meeting a block) is a real error; `SUB`/`FUNCTION` headers and `EXIT` look past the
`$IF` entry. Proper nesting falls out of the stack, with no special case and no splitting. The error is reported
where the nesting breaks: at the `$ELSE`/`$END IF` that meets an open block (the old compiler, whose `$END IF`
pops without checking, reports the later `END IF` instead; our messages are new anyway). Also measured:
`$IF`/`$LET` only at the start of a line (after `:` or in a single-line `IF`: real error); conditions as in
`study\00` §5 (no `NOT`, no parentheses; `DEFINED(A)` and `(A = 1)` are silently false, which we follow);
`$LET` of a predefined name does not override it.

### D9. Included files: one tree per inclusion
- The parser does no I/O. `parse` takes an `&mut dyn Loader` (`fn load(&mut self, map: &mut SourceMap, from:
  FileId, path: &[u8]) -> Result<FileId, LoadError>` adding the file to the `SourceMap`). Changed in task 4.1: the
  map is an argument of `parse` and `load` rather than held by the loader, and a `SourceFile`'s bytes are an
  `Arc<[u8]>`, so the parser can hold the including file's bytes while the loader adds another file. At an active `$INCLUDE:'…'` (or comment
  `$INCLUDE`, after its line), the parser parses the included file at once, with the current `PpState`, and
  continues with the state the include leaves behind (a `$LET` in a `.bi` reaches the main file).
- The result is a `ParsedProgram`: trees, each with a `TreeId` and its `FileId`, and a map from (`TreeId`, offset
  of the include statement) to the included tree. The same file included twice gives two trees (they may differ by
  `$IF` state); `$INCLUDEONCE` in a file makes later inclusions of it empty (the old compiler's rule: read in
  `qb64pe.bas` 3141–3165, the `$INCLUDEONCE` line itself).
- Path resolution as the old compiler does it (the upstream tests `include_paths\*` and `include_once\*` are the
  check). Read in `qb64pe.bas` 3120–3171 and measured (M7, 2026-10-05): a leading `.\` or `./` is dropped; the
  including file's folder first (the main file's folder at the top level), then the path as written relative to
  the compiler's own folder (every QB64 program changes to its exe's folder at start, `libqb.cpp` 25997); the main
  file's folder is not searched for a nested include, and the caller's working directory plays no part. We do the
  same with a **compiler root**: by default the folder of `qb64rust.exe`, set with `--include-root <dir>`; an
  absolute include path is used as written. The tier-2 runner passes `--include-root tests/upstream`, and tier 1's
  loader in `inputs.rs` uses the same root, so the upstream tests that include `'tests/compile_tests/extra/…'`
  (`include_fixed_compile_location`, `include_multiple`) resolve against our copy in both tiers.
  Missing file, empty name: real errors at the include. Changed by M7: the depth limit is **100** (the old
  compiler's, "Too many indwelling INCLUDE files"), not 32. Changed by the review (2026-10-05): **no cycle check**,
  as in the old compiler; a file that includes itself under a `$IF` whose condition a `$LET` changes ends on its
  own and is accepted there (measured: `verification\v16_m7_self_guarded`), so a cycle check would be a false
  error. Past the limit, the error names the file at
  the deepest level instead of listing every level.
- Changed by M1 (2026-10-05): only the comment forms exist (`'$INCLUDE:'f'`, `REM $INCLUDE:'f'`, also after a
  statement on the same line, blanks around the colon allowed); `$INCLUDE:'f'` without a comment is a syntax error
  in the old compiler, and stays a real error here.
- Each tree is lossless for its file. `--dump tree` prints every tree, headed by its file name.
- A file's tree is parsed as statements at file level; whether its content is legal at the include point (a `SUB`
  in a file included inside a `SUB`) is checked by `sema` against the old compiler's verdict (M7).

### D10. `sema`: keys, marking, no follow-on errors
- **Keys.** `proc_of_def`, `label_of_def` and every other node-keyed map use (`TreeId`, offset), not the offset
  alone; `FileId` is not enough because one file can be included twice. `check` takes the `SourceMap` and the
  `ParsedProgram` and reads text through the `SourceMap` (`text()` uses the span's file). The walk descends into an
  included tree at its include statement. The symbol table keeps spans with `FileId` (unchanged format).
- **Marking.** Every new statement or expression kind `sema` does not compile yet gets one "not supported yet"
  diagnostic at its first token, naming it (same words as the parser uses today, so most messages stay). The
  statements inside an unsupported block are still checked, as today. (Group 6: a block is marked unless its
  header has a parse error; its statements are checked one by one; main-module labels inside blocks are entered in
  pass 1, so `ON ERROR GOTO` may name them.)
- **No follow-on errors.** Changed 2026-10-05 (`study\23` §2.3; the first design recorded the names of each
  unsupported declaration as *unknown* and silenced their uses, which is scaffolding that goes away as each
  feature is compiled). The rule is blunt instead: after the first **declaration** reported "not supported yet", in
  file order (`DIM`/`REDIM`/`COMMON`/`SHARED`/`STATIC`, `CONST`, a `TYPE` block, `DECLARE LIBRARY`, `DEF FN`,
  `DEFxxx`, `_DEFINE`), `sema` reports only "not supported yet" errors for the rest of the program; a real error
  found after that point is dropped (the statement still fails, with no diagnostic). Parser errors are not
  affected. The program already has the marked error, so it is still rejected, and no type is guessed. Name
  tracking as first designed is added only for a declaration kind whose new entries in
  `known_unsupported_rejections.list` the user does not accept on review (D1). Names that the old
  compiler knows and we do not (the constants and functions of the auto-included files, from a list extracted by
  `tools\builtins\extract_builtins.py` from the clone's auto-include files; `_GL`; built-in statement names used as
  assignment targets) get a "not supported yet" error instead of the reserved-name error.
- **Risk to "rejected stays rejected":** dropping real errors can hide one in a program the old compiler rejects
  (the test engineer's dissent, `study\23` §2.3). The third list rule (D1) makes every such case visible and
  reviewed.

### D11. Panic hook and the tier-2 `.err` meaning
- `main` installs a panic hook: it prints `qb64rust: internal compiler error: <message> at <file>:<line>` and the
  input name, and the process exits with code 3 (`main.rs` uses 1 for every other failure: errors in the program,
  bad options, a failed build). A hidden environment variable `QB64RUST_TEST_PANIC=1` panics on purpose for the CLI test.
- `run_legacy_tests.py`: for `--qb64` other than `qb64pe`, an `.err` test passes when the compile exits non-zero,
  writes no executable, and its output has at least one `error:` line not followed by `not supported yet`. The
  `.err` text is not compared. Same meaning as tier 1's "must reject". The upstream `.err` programs that pass are
  added to `pass.list` (task 2.3).

### D12. Tests
- `tests\frontend\` gets a `parse-ok` file per family with every form found in the inputs, and `check-fail` files
  for the recovery rules of D4 and the errors of D6 and D9; parser snapshots for the tree shapes of D4, D6 and D8.
- Measurements (M1–M8 below) go to `verification\v16_*` with recorded output, as before; anything new learned about
  the old compiler goes into `study\00` §5.
- The mutation test runs unchanged (it now reaches the new code paths).

### Measurements (with `qb64pe.exe`, before the matching task)
- **M1** comment metacommands: `PRINT 1 '$INCLUDE:'x.bi'` (after a statement), `REM $INCLUDE`, `' $DYNAMIC`, two
  `$INCLUDE`s in one comment (only the last counts, per the source).
- **M2** `DATA`: `'`, `:` and `REM` inside unquoted items; leading and trailing blanks; empty items.
- **M3** line numbers: `10 PRINT`, `10 :`, `10` alone, after a label, decimal (`10.5`), out of order, duplicates;
  `GOTO 10` to a missing number.
- **M4** blocks: `NEXT j, i`; `NEXT` without a variable; `NEXT` closing a `FOR` outside an `IF`; `END IF` without
  `IF`; `LOOP` closing a `WHILE`; single-line `IF` with `ELSE` and nested `IF`s; `ENDIF`; `END SELECT` missing.
- **M5** template statements: a wrong `LINE` form, `B` and `BF` as variable names, `PUT` graphics vs file form.
- **M6** `$IF`: conditions (`=`, `<>`, `AND`, `OR`, `NOT`, `DEFINED`, `VERSION >= 3.0`), predefined names,
  `$LET` redefinition, garbage inside an inactive branch.
- **M7** `$INCLUDE`: search order (including file's folder, main file's folder, compiler folder), the same file
  twice, a `SUB` in a file included inside a `SUB`, a block open across the boundary.
- **M8** member access: `a.b` as a plain variable name next to a `TYPE` variable `a`.

## Risks / Trade-offs

- **Size.** This is the largest change so far (tens of node kinds). Mitigation: tasks are per family, each ends
  with the three lists regenerated and reviewed, and each can be checked in on its own.
- **Template matching differs from the old compiler** (order of alternatives, word matching inside names). The
  lists and M5 show it; a mismatch is a false error, which tier 1 catches.
- **The preprocessor in the parser** couples parsing to `$LET` state and the target platform. Accepted: the old
  compiler does the same, and only active code has a meaning.
- **Dropping follow-on errors** can hide a real error (D10). Mitigation: the rejection list and its review rule.
- **The change pauses after group 6** for the control-flow slice (`study\23` §4 step 3), which may change block
  nodes or their accessors. Groups 7 to 9 start from whatever that slice leaves.
- **Tier-1 time**: more work per file. Measured at the end (task 9.2); the budget is a minute.

## Open Questions

- None left. (Where a `$IF` may appear: decided by M6, only at the start of a line.)
