# Design

## Context

What exists today:
- **Warnings.** `Diagnostics::warning(span, msg)` in `crates/base` makes one. The CLI prints warnings only with
  `-w` (`Frontend::render_with_warnings`, `crates/driver/src/lib.rs`). The language server publishes every
  warning, with or without `-w` (`crates/lsp/src/analysis.rs`). No test runner or tool passes `-w` to
  `qb64rust`, so extra warnings change no recorded output.
- **The symbol table** (`crates/sema/src/symbols.rs`). It holds one `Symbol` per variable, procedure, label,
  constant, type and member. A symbol has a `def` span and the `refs` spans of its other uses. The first span
  recorded for a symbol becomes its `def`. For a label that is its definition, because labels are entered in
  pass 1. For an implicit variable it is its first use. For a procedure it is whichever comes first in the file:
  a `DECLARE` line, a call, or the header.
- **Label uses.** Every way of naming a label goes through `label_for`, `restore` or `label_ref` in
  `check/flow.rs`, and each records the use. This covers `GOTO`, `GOSUB`, `RETURN label`, `ON … GOTO/GOSUB`,
  `RESTORE`, `RESUME`, `ON ERROR GOTO`, `RUN`, the event statements (`check/events.rs`) and `THEN 10`.
- **Shadowing.** `check/decl.rs` already works out `shadows_shared` when a procedure's `DIM` names a main-module
  `DIM SHARED` variable. Today it uses it only to reject a `STATIC` beside one.
- **Statements.** `Program.stmts` is the main module's list and `Proc.stmts` each procedure's. Blocks (`If`,
  `For`, `Do`, `While`, `Select`) hold their own lists. A label or line number is a `StmtKind::Label` in the list.
- **Reports.** `qb64rust --report unsupported` runs the front end over many files without building
  (`crates/driver/src/report.rs`).

## Goals / Non-Goals

**Goals:**
- Few false warnings, measured on the corpus before the change closes (D9).
- One place for the lints, run once on the finished program.
- No change to errors, to the C++ written, or to what any program does.

**Non-Goals:**
- Flow analysis across blocks. "Unreachable" looks at one statement list at a time.
- Silencing a lint for a line or a file.

## Decisions

### D1. The lints run once, on the finished program, in a module of their own

A new module, `crates/sema/src/lints.rs`, holds one function per lint and `pub(crate) fn run(prog, main, diags)`.
`check_with` (`crates/sema/src/check/mod.rs`) calls it after its last `check_once`, and only when the diagnostics
hold no error. `main` is the `FileId` of the file being compiled, the main file of the `ParsedProgram`.

*Alternative:* give each warning where the checker meets the case. That would warn in programs with errors. It
would also repeat the warnings when `check_with` runs `check_once` again, and would spread the lints over a
dozen modules.

### D2. A lint warning carries a code

`Diagnostic` (`crates/base/src/lib.rs`) gets `code: Option<&'static str>`, `None` for every existing diagnostic.
A constructor `Diagnostic::lint(span, code, msg)` makes a warning with a code. The codes are constants in
`lints.rs`.
- The language server sets the LSP diagnostic's `code` from it (`crates/lsp/src/analysis.rs`).
- The CLI does not print the code. The `-w` line format stays `file:line:column: warning: message`, which the
  extension and users already read.
- `--report warnings` groups by it (D8).

*Alternative:* group by the message's wording. Wording changes would then break the report, and the editor could
not show the code.

### D3. Only the file being compiled, for our own lints

Each lint finds its warning's span first, then drops it when `span.file != main`. One filter in `run` applies
this to our five. QB64pe's warnings (D11) skip the filter, because `qb64pe -w` gives them in included files too. For `unreachable` the warning goes on the first dead statement in the main file, so a dead
stretch that starts in an included file and goes on in the main file is still reported.

In a `qb64rust test` build the test file is included from a generated main file, so no lint fires there. That
build never prints warnings anyway.

### D4. `used-once`: a flag on `Var`, and two exemptions

`Var` (`crates/sema/src/lib.rs`) gets `implicit: bool`. It is set where a first use makes a new variable
(`check/decl.rs`, the lookup that ends in "else a new local", in the main module and in procedures). A variable
is reported when it is implicit and its symbol has no `refs`. The warning goes at its `def`.

Two kinds of single use are not typos:
- a `FOR` counter with a bare `NEXT`;
- the target of `INPUT` or `LINE INPUT` that only waits for Enter.

`lints.rs` walks every statement list once and collects the `VarId`s of `FOR` counters and of those targets. It
skips them. Names differing only by suffix are different variables. So `x` set and `x$` printed gives one warning
each, which is right: that is a typo too.

### D5. `unused-procedure`: spans that declare are not calls

Since a `DECLARE` line or the header can be a procedure's `def`, "no `refs`" is not "never called". `Symbols`
gets a set of the spans that declare a procedure: the header's name (`check/proc.rs`, where the header is
recorded) and the name of each `DECLARE SUB/FUNCTION` line. A procedure is reported when none of its spans
(`def` and `refs`) is outside that set. The warning goes at the header's name, the declaring span on
`proc.line` of `proc.file`.

Not reported:
- `SUB _GL`, which the runtime calls;
- a procedure with `external` set (a `DECLARE LIBRARY` routine);
- in a main file whose name ends in `_test.bas` (any case), a SUB whose name starts with `Test` (any case) and
  has no parameters, as `qb64rust test` finds tests (spec `compiler/unit-tests`).

A procedure that calls itself counts as called. A procedure called only by other unused procedures is not
reported. Both are limits, listed as a `SOMEDAY.md` row.

### D6. `unused-label`: named labels with no `refs`

A label is reported when its `line_number` is `None` and its symbol has no `refs`. Line numbers are never
reported: old programs number every line.

A label that is the name of a SUB and a colon is a call, not a label, and has no `Label` entry. It is never seen
here.

### D7. `hides-shared`: found in `check/decl.rs`, given by the lints

Where `shadows_shared` is true and the declarer is `DIM` or `REDIM`, `decl.rs` pushes the name's span and
`VarId` to a new list on `Program`, `hidden_shared` (`pub(crate)`). `lints.rs` turns each entry into a warning.
This keeps D1's rule that lints run only on programs without errors.

### D8. `unreachable`: one pass over each statement list

`lints.rs` walks each list, the main module's, each procedure's, and recursively each block's. After a statement
that always leaves the list, every following statement is dead until the next `StmtKind::Label`.

The statements that always leave are `Goto`, `End`, `System`, `Stop`, `Exit`, the `EXIT DO/FOR/WHILE` kind,
`Return`, `Resume` and `Run`. Three that look similar are not on the list:
- `ERROR n` continues after it when a handler does `RESUME NEXT`.
- `GOSUB` comes back.
- `ON … GOTO` falls through when the number is out of range.

A statement kind that writes no code does not start a dead stretch. Which kinds those are is found while
building. The warning names the leaving statement's keyword and line. It is given once per stretch.

### D9. `--report warnings`, and measuring on the corpus before closing

`report.rs` gets `warnings(inputs, include_root)`. It takes its inputs as `unsupported` does, which reuses its
walk of files and folders, and prints the table of the spec, then each warning. `main.rs` accepts `warnings`
beside `unsupported`.

Before the change closes, the report runs over `tests/corpus`, `tests/upstream` and the programs of
`tests/verification/pass.list`. A sample of each lint's warnings is read:
- A lint whose sample is mostly false warnings is narrowed and measured again.
- Each narrowing is a decision in "Found while building" below.
- The final counts go there too.

This run builds and runs nothing. It is the front end only, one process, a few seconds of CPU. So it is not a
large run under `CLAUDE.md` working rule 6: no program can start, open a port or put a window on the screen. It
runs on the host.

### D10. Tests

- **A new frontend mode, `warnings`** (`crates/driver/tests/it/frontend.rs`). The program must have no errors.
  The snapshot is `render_with_warnings()`, so it shows every warning with its place.
- **One fixture per lint**, `tests/frontend/lints_<code>_warnings.bas`. Each holds the cases where the lint
  fires and those where it must stay quiet: the spec's scenarios, a numbered program, `END` followed by a handler
  label and by `DATA`, `ON TIMER` naming a SUB, a `DECLARE LIBRARY` routine, and `FOR` with a bare `NEXT`.
- **The include rule.** A fixture that includes a toolbox file from `tests/frontend/inc/` and uses one routine of
  it. A fixture named `lints_test.bas` with a test SUB.
- **The CLI.** A test that `-w` prints a lint line and that the program still builds. Two `--report warnings`
  tests, for the spec's two scenarios.
- **The language server.** A check test that the published warning has code `unused-label`
  (`crates/driver/tests/it/check.rs`).

QB64pe gives none of our five warnings, so there is no recorded old-compiler output to match. The frontend
snapshots are the recorded behavior. They are our own warnings, as the value trace is our own option, so there
is no `DIVERGENCES-QB64PE.md` row. QB64pe's four are held to `v68`'s recorded output (D11): a CLI test compares
our `-w` lines with QB64pe's for each `v68` program, line and wording.

### D11. QB64pe's warnings: measured first, then built in its wording

`qb64pe -w` gives warnings we lack (`..\QB64pe\source\qb64pe.bas`):
- **"Unused variable"** (line 13466). Every variable the compiler creates goes into `usedVariableList`
  (`manageVariableList`, 28558). Each later lookup of the name by `findid` marks it used (23398). Those never
  marked are printed after the C++ is written. The detail is the name with its suffix, `()` for an array, and
  the type name (`id2fulltypename$`). Variables in included files are reported at their line there.
- **"Empty SELECT CASE block"** (6927): a statement inside a `SELECT CASE` before its first `CASE`.
- **"Duplicate constant definition"** (2679): a `CONST` of a name that already exists with the same value. The
  detail is `name = value`.
- **"Feature incompatible with $DEBUG mode"** (9760, 9766): `CHAIN` or `RUN` with vWatch on. Whether vWatch is on
  outside the IDE is to be measured. "$DEBUG features won't work in these blocks" (3565) needs the IDE, so it is
  never given from the command line and is not built.

Reading the source does not give the edge cases. Examples: is a variable's first implicit use a lookup? Does an
array's `LBOUND` mark it used? Is a FUNCTION's result variable counted? So a new family, `verification\v68_*`,
records `qb64pe -w -z` output for each case before building. `-z` writes C++ only: nothing runs, and it is a few
dozen programs one at a time.

QB64pe prints `p.bas:3: warning: Unused variable` and then `    x  SINGLE` on its own line. Our line has a column
and the detail on the second line, as the `$VERSIONINFO` warning already does (`check/mod.rs` line 1885). In
`lints.rs`, "Unused variable" uses the symbol table: a variable whose symbol has a `def` and no `refs`, by
whatever rule `v68` shows. "Empty SELECT CASE block" and "Duplicate constant definition" are found where the
checker meets them (`check/blocks.rs`, `check/constants.rs`). They are queued on `Program` as D7 does, so they
too appear only for programs without errors.

`used-once` and "Unused variable" can name the same variable. Then only "Unused variable" is given (spec, "One
warning per variable"). If `v68` shows that QB64pe reports every implicit variable named once, `used-once` is
left with no case of its own and is dropped. Which way it goes is written in "Found while building".

These four follow QB64pe, so they need no `DIVERGENCES-QB64PE.md` row. A difference found while measuring, such
as a column QB64pe does not print, is one.

## Risks / Trade-offs

- [Noise in the editor] The language server shows the lints on every check, without `-w`. A user typing a new
  name sees `used-once` until the second use. → This is the usual behavior of linters in editors. D9 measures
  the noise on real programs, and a lint that is mostly wrong is narrowed before the change closes.
- [Existing tests see new warnings] Tests that count or compare warnings may now see lint warnings:
  `check.rs`'s `warning`, and the `-w` tests in `cli.rs` around lines 1575 and 3694–3815. → Each is read. A
  fixture program gets the missing use, or the expected text gets the lint line, whichever keeps the test's
  point.
- [A missed way of naming] A label or procedure named by a statement that does not record the use would be
  reported falsely. → The corpus run (D9) finds these. Fixing the recording also fixes go-to-definition and
  rename, which use the same table.
- [`Var` and `Diagnostic` grow a field] Both are shared types. → `Diagnostic` gets a constructor and a default.
  `Var` is built in few places. The compiler will list each one.

## Found while building

(Filled in by the tasks: the statement kinds that write no code, any narrowing after the corpus run, and the
final counts.)

**`hides-shared` (task 2.4).**
- A parameter with the name of a `DIM SHARED` variable compiles with no error today. Inside the procedure the name
  means the parameter (`SUB Param (score AS LONG)` in `lints_hides_shared_warnings.bas`). The lint does not
  report it: the spec names `DIM` and `REDIM` only, and a parameter's name is chosen in the header, in plain view.
- Only scalars are reported. An array goes through `declare_array` in `check\places.rs`, which has no
  `shadows_shared` case, so D7's hook does not see it.
- A local of the same name and another type is not reported, as the spec says. This includes a plain `DIM score`
  beside `DIM SHARED score AS LONG`: it makes the SINGLE `score!`, and in that procedure the plain name `score` then
  means the local (`verification\v14_dim_local_plain`). That hides the shared variable too, but the spec asks for
  name and type. A `SOMEDAY.md` row could widen it.

**`unreachable` (task 3.1).**
- The statement kinds that write no code. `DATA`, `DECLARE`, `CONST`, `TYPE`, `DEFINT` and the like, `OPTION
  BASE`, `_DEFINE`, `COMMON`, a `DIM` of a scalar or of a static array, comments and metacommands put no statement
  into the typed lists at all (measured with `--dump typed`). The only statement there that the user does not
  write as one is `$CONSOLE:ONLY` (`StmtKind::ConsoleOnly`, lowered to `Op::SelectConsole`). It does not start a
  dead stretch and is not reported.
- `RUN file$` is not a leaving statement. It raises error 53 when the program cannot be started, and a handler with
  `RESUME NEXT` then goes on after it, as after `ERROR n`. `RUN` and `RUN label` are leaving statements.
- The statement is named by its first word, or its first two for `EXIT SUB`, `EXIT DO` and the like, in upper
  case. A `THEN 10` jump has no keyword of its own, but it stands alone in its branch, so nothing follows it there.

**QB64pe's warnings (task 5.1).** Measured with `verification\v68_*`, thirteen programs; the facts are in that
family's line of `verification\README.md`. What decides each rule:
- "Unused variable" follows the creation of the variable, not the count of its names. A variable made by its
  first use is never reported, because that first use is itself a lookup. A declared one is reported when no name
  of it follows the declaration; an assignment counts. `COMMON`, `COMMON SHARED` and a variable a `SHARED`
  statement made are never reported. So `lints.rs` reports a variable that is not `implicit`, not in
  `Program.not_reported_unused` (filled where `COMMON` and `SHARED` make variables), and whose symbol has no
  `refs`; a FUNCTION's result when it has no symbol at all (the FUNCTION never assigns it), at the header's name.
  A `DECLARE LIBRARY` routine's parameters are left out.
- **`used-once` keeps every case of its own.** It is given only to implicit variables, and "Unused variable" never
  to those. No variable can get both, so the spec's "One warning per variable" holds without a rule in the code.
- "Empty SELECT CASE block" is at the `END SELECT`, for a `SELECT` with no `CASE` and no `CASE ELSE`. A statement
  before the first `CASE` was already an error of ours, at QB64pe's line (`v68_e2`, `e3` pass as rejections).
- "Duplicate constant definition" shows the name as written, without its suffix and in its own letter case
  (`a = 5`, not `A = 5`). The spec's scenario said `A = 5`; it is corrected to `a = 5`.
- "Feature incompatible with $DEBUG mode" is given outside the IDE, so it is built: for `RUN` in `check\flow.rs`
  and for `CHAIN`, which compiles as a plain built-in statement, in `check\builtins.rs`. `$DEBUG` anywhere in the
  program counts, as the parser settles the flag over the whole program.
- How QB64pe prints them differs from our `-w` in three ways, which the CLI test `qbpe_warnings_as_recorded`
  leaves out of its comparison. Our line has a column. We print every warning in source order, where QB64pe prints
  its unused variables after all the others. And QB64pe prints a warning of its compile pass twice when it compiles
  a program twice (`v68_f1`, `g1`, `g2`). That last one already held for `$DEBUG`'s own warning.

**The corpus run (task 6.1).** `qb64rust --report warnings` over `tests\corpus` (492 programs), `tests\upstream`
(348) and the whole `verification` folder (4,396; it holds the programs of `pass.list`, whose names are too many
for one command line), with `--include-root tests\upstream\root`. Front end only: under 4 seconds for all three.
- **One narrowing, of `unreachable`.** Before it, 25 of upstream's 26 warnings were code after a `$IF WIN` branch
  that ends in `SYSTEM`: the code after `$END IF` is what other platforms run (`compile_tests\glut\*`,
  `precomp-flags\*`). Now a `$IF`, `$ELSEIF`, `$ELSE` or `$END IF` line between the leaving statement and the
  next one ends the stretch, as a label does (`pp_boundary` in `lints.rs`; `SUB Platform` in
  `lints_unreachable_warnings.bas`). Upstream went from 26 to 1, which is real (`SYSTEM` twice in a row).
- **Two more, found in the review.** The skipped `$IF` branch hid uses from the other lints too. With a
  `$IF WIN THEN` branch that calls `WinSetup` and a `$ELSE` branch that calls `LinuxSetup`, Windows gave
  ``SUB `LinuxSetup` is never called``. Now a name that
  occurs as a word in a skipped branch gets no `used-once`, `unused-procedure` or `unused-label`
  (`inactive_words` in `lints.rs`; `lints_platform_warnings.bas`). The words are taken from the branch's text, so
  a word in a comment there also keeps a warning back; that errs on the quiet side. And a `READ` target is left
  out of `used-once` as an `INPUT` target is: `READ skipped` throws a `DATA` item away.
- The samples read were otherwise right. `used-once`: every one is a name that occurs once; most are values set and
  never read (`junk$ = INKEY$`, `x& = _SCREENX`) or test programs reading an unset variable on purpose.
  `unused-procedure`: upstream's helper SUBs `CheckText` and `CheckNear` copied into files that do not call them,
  and the corpus's `keep` and `never` SUBs. In `slice\s70_sub_name_labels` the SUBs `viagosub` and the like are
  rightly never called: a jump to `name` makes `name:` a label, as QB64pe has it. `unreachable`: dead code on
  purpose after `SYSTEM`, `END` or `RESUME` in test programs, and three stretches in Civil War Strategy (one is
  `topbar:` after `GOTO menu0`, a call of the SUB `topbar`, not a label). `hides-shared`: only the three
  measurement programs written for that case (`v14_dim_local_same`, `v24_x284`, `v68_a3_more`).
- **QB64pe's four against `qb64pe -w -z` itself**, beyond `v68`: the same warnings, details and padding included, in
  15 programs (`compile_tests\types\all_types` with 30, `types\static`, `arrays\01_integer_family_1d`,
  `slice\s32_bit`, `s90_mem_get_put`, `s50_deftype_param_shared`, `s99_mem_error_308`, `s18_const`,
  `s28_select_case`, `s10_scopes`, `s55_implicit`, `verification\v16_m4_loop_forms`, `v17_e_const_redefine_same`,
  `v20_h_select`, and Civil War Strategy, with none). The corpus's copy of Civil War Strategy seemed to differ, a
  parameter `sec!` reported by us only: QB64pe stopped there at `$EXEICON`, whose icon is not beside the copy, and
  the copy has `TICK`'s body commented out. `v68_a4_params` now pins unused parameters.

Final counts (programs, then warnings):

| Code | corpus | upstream | verification |
|---|---|---|---|
| `used-once` | 18, 46 | 3, 15 | 98, 150 |
| `unused-procedure` | 17, 21 | 13, 21 | 79, 88 |
| `unused-label` | 4, 11 | 0, 0 | 31, 32 |
| `hides-shared` | 0, 0 | 0, 0 | 3, 3 |
| `unreachable` | 19, 28 | 1, 1 | 142, 148 |
| `unused-variable` | 30, 61 | 8, 74 | 235, 1062 |
| `empty-select` | 1, 1 | 0, 0 | 3, 3 |
| `duplicate-const` | 1, 1 | 0, 0 | 2, 6 |
| `debug-incompatible` | 0, 0 | 0, 0 | 2, 2 |

Programs with errors, which get no lint warnings: corpus 4, upstream 12, verification 2,570 (most of the
verification programs measure a rejection).
