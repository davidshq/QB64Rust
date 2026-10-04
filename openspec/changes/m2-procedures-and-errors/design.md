# Design

## Context

See `proposal.md` for motivation and the spec deltas for requirements. Background: calling convention in
`study\02` §4.1, procedure shape in `study\02` §8, labels and `ON ERROR`/`RESUME` in `study\02` §6.2–6.3, storage
names in `study\02` §3.1, the runtime's error path in `..\QB64pe\internal\c\libqb\src\error_handle.cpp`
(`error`, `fix_error`). The previous change's design (`openspec\changes\archive\2026-10-04-m2-workspace-and-slice\design.md`,
D1–D12) still holds; this one extends D4–D7.

### What the old compiler emits (measured 2026-10-04, `qb64pe.exe` `16f629784e`, scratch folder, not the repo)

Three probe programs (procedures and by-reference rules; scopes, `DECLARE`, `RESUME label`; `CHR$` raising inside
`PRINT`, nested handlers, an error inside a FUNCTION) were compiled with `-x` and run under the corpus runner's
console helper. Fragments:

| Construct | Emitted | Notes |
|---|---|---|
| `SUB bump (x AS LONG)` | `regsf.txt`: `void SUB_BUMP(int32*_SUB_BUMP_LONG_X);` `mainK.txt`: the prologue/epilogue of `study\02` §8 verbatim, `#include "dataK.txt"`, `#include "freeK.txt"` | one `mainK` per procedure in definition order; `main.txt` includes `main0`…`mainN` |
| `FUNCTION twice& (a AS LONG)` | `int32 FUNC_TWICE(int32*_FUNC_TWICE_LONG_A)`; result variable `_FUNC_TWICE_LONG_TWICE` in `dataK`; `return *_FUNC_TWICE_LONG_TWICE;` after the epilogue | string result: `qbs_maketmp(v);return v;` |
| `CALL bump(n)`, `bump n` (n LONG) | `SUB_BUMP(__LONG_N);` | by reference |
| `bump (n)`, `bump n + 1` | `SUB_BUMP(&(pass1=*__LONG_N));` | by-value temporary; `int32 pass1;` declared in `maindata.txt` (main) |
| `CALL greet(s, 2.5)` to `(t AS STRING, i AS INTEGER)` | `SUB_GREET(__STRING_S,&(pass3=qbr_float_to_long( 2.5E+0 )));` | conversion as for an assignment |
| string parameter | guard in `dataK`: if the incoming `qbs` is `tmp`, `fixed` or `readonly`, replace it by a copy (`oldstrN`); `freeK` copies back into a fixed string and frees the copy | verbatim from the probe |
| local `DIM k AS LONG`, implicit `x` in a SUB | `int32 *_SUB_GREET_LONG_K=NULL; if(...==NULL){ ...=(int32*)mem_static_malloc(4); *...=0; }` in `dataK` | per call; released by the epilogue's `mem_static_pointer` restore |
| `STATIC c AS LONG` in SUB greet | `int32 *_SUB_GREET_LONG_C=NULL;` in `global.txt`, initialiser in `maindata.txt` | |
| `SHARED n AS LONG` in a SUB, `DIM SHARED g AS LONG` | the main-module `__LONG_N` / `__LONG_G` is used directly | |
| `EXIT SUB`, `EXIT FUNCTION` | `goto exit_subfunc;` | |
| `DECLARE SUB …` | nothing (no statement wrapper) | `DIM` and `SHARED` lines get an empty wrapper |
| function call in `PRINT` | `qbs_str((int32)(FUNC_TWICE(__LONG_N)))`: printed with the **function's** type | not `int64` like an integer operation |
| label `handler:` | `LABEL_HANDLER:;` then `if(qbevent){evnt(18);r=0;}` | |
| `ON ERROR GOTO handler` | `error_goto_line=1;`; `mainerr.txt`: `if (error_goto_line==1){error_handling=1; goto LABEL_HANDLER;}` before `exit(99);` | handler numbers from 1 in order of first use |
| `ON ERROR GOTO 0` | `error_goto_line=0; qbs_set(error_handler_history, qbs_new_txt_len("", 0));` | |
| `ERROR 5` | `error( 5 );` | |
| `RESUME` | `if (!error_handling){error(20);}else{error_retry=1; qbevent=1; error_handling=0; error_err=0; return;}` | |
| `RESUME NEXT` | the same without `error_retry=1; qbevent=1;` | |
| `RESUME back` | `…else{error_handling=0; error_err=0; goto LABEL_BACK;}` | |
| `ERR`, `ERL` | `get_error_err()` printed as `(uint32)`, `get_error_erl()` printed as `(double)` | table: `_UNSIGNED LONG`, `DOUBLE` |
| `CHR$(k)` | `func_chr(*__LONG_K)` | error 5 outside 0–255 |

Run results that fix the semantics (all reproduced in the new slice programs, D9):
- By reference: `CALL bump(n)` and `bump n` change `n`; `bump (n)` and `bump n + 1` do not.
- `PRINT "a"; CHR$(k); "b"` with `k = -1` and a handler that sets `k = 65` and `RESUME`s prints `a`, the handler's
  output, then `aAb`: **retry re-runs the whole statement**, including items already printed.
- `PRINT "c"; CHR$(300); "d"` with a `RESUME NEXT` handler prints `c`, then the handler's output: **the rest of the
  statement is skipped, including its line end**.
- An `ERROR 7` inside `FUNCTION f&` called from a `PRINT` in main, handler `RESUME NEXT`: the function **continues
  after its own `ERROR` statement** and returns normally; the `PRINT` completes. Same for a SUB.
- A handler that sets a second handler and `RESUME`s: the retried statement raises again and reaches the second
  handler.
- `ERR` is 0 again after `RESUME`/`RESUME NEXT`. `ERL` is 0 without numeric line labels.
- `INSTR` with a start of -1 does not raise either (prints 3 for `INSTR(-1, "hello", "l")`).

### Measured in task 2.1 (`verification\v14_*`, recorded with `verification\run.sh`)

Scopes and handlers (programs that run):

| Program | Result |
|---|---|
| `v14_dim_shared_after` | a SUB defined **before** `DIM SHARED g` does not see `g`: its `g` is an implicit local (prints 0); main's `g` is unchanged |
| `v14_shared_implicit` | `SHARED h AS LONG` in a SUB binds main's `h&`, which main never `DIM`s; the SUB reads 9 and its `h = 5` is seen by main |
| `v14_shared_then_dim` | `SHARED h AS LONG` in a SUB and a later `DIM h AS LONG` in main: the old compiler emits `__LONG_H` twice and the **C++ compile fails** (`.cxxerror.txt`) |
| `v14_on_error_sub_to_main` | `ON ERROR GOTO mainh` **inside a SUB**, naming a main-module label, compiles; the main handler runs and `RESUME NEXT` continues in the SUB |
| `v14_on_error_in_sub` | a label inside a SUB used by `ON ERROR GOTO`: "Common label within a SUB/FUNCTION" (compile error) |
| `v14_error_values` | `ERROR 0` and `ERROR -1` raise 5; `ERROR 255` and `ERROR 70000` raise themselves; a fractional value is rounded half to even (2.5 → 2, 2.7 → 3, 3.5 → 4); `ERL` 0 throughout |
| `v14_error_256` | `ERROR 256` is critical ("Out of stack space") and ends the program although a handler is active (runtime: 256, 257 and 500+ are critical; the compiler only calls `error()`) |
| `v14_err_exit_sub_in_function`, `v14_err_exit_function_in_sub` | both compile and run: `EXIT SUB` and `EXIT FUNCTION` are interchangeable inside any procedure |
| `v14_declare_mismatch_ok`, `v14_declare_only` | a `DECLARE` that disagrees with the definition, and one for a SUB that does not exist, are both accepted: `DECLARE` is ignored |

Reserved names (old compiler's message in brackets):

| Program | Result |
|---|---|
| `v14_res_param_name`, `_param_cls`, `_param_len` | rejected ("Name already in use (name)", `(cls)`, `(len)`) |
| `v14_res_var_cls`, `_var_print`, `_var_err` | `DIM cls/print/err AS LONG` rejected ("Name already in use") |
| `v14_res_func_len` | `FUNCTION len&` rejected: a suffix does not free a name |
| `v14_res_var_left_str` | `left$ = "x"` rejected (parsed as a call of `LEFT$`) |
| `v14_res_param_left`, `_var_left`, `_sub_chr` | **accepted**: `left` and `chr` are free, because `LEFT$` and `CHR$` must be written with their `$` |
| `v14_res_sub_main_var` | `x = 1` in main and `SUB x`: rejected at `x = 1` ("Name already in use (x)") |

Compile errors of D8:

| Program | Old compiler's message |
|---|---|
| `v14_err_argcount`, `_sub_paren_two_args` | "Syntax error - Reference: s a AS LONG…" |
| `v14_err_argcount_few` | "Incorrect number of arguments" |
| `v14_err_string_for_number` | "Number required for sub" |
| `v14_err_number_for_string`, `_declare_mismatch` | "String required for sub" (the `DECLARE` is ignored; the call is checked against the definition) |
| `v14_err_exit_sub_main` | "EXIT SUB must be used within a SUB" |
| `v14_err_undefined_label`, `_resume_undefined_label` | "Label 'nowhere' not defined" |
| `v14_err_label_in_main_from_sub` | `RESUME back` in a SUB to a main-module label: "Label 'back' not defined" |
| `v14_err_duplicate_label` | "Duplicate label (a)" |
| `v14_err_sub_as_function`, `_func_other_suffix`, `_duplicate_proc` | "Name already in use (s)" / `(f)` / `(s)` |
| `v14_err_func_result_wrong_type` | "Illegal string-number conversion" |
| `v14_err_nested_sub` | "Expected END SUB/FUNCTION before SUB" |
| `v14_err_missing_end_sub` | "Expected END SUB/FUNCTION" |

## Goals / Non-Goals

**Goals:**
- The IR describes procedures, argument passing, storage classes and the raise/resume rules without naming C types,
  libqb symbols, `passN`, `error_goto_line` or `evnt`.
- The 18 corpus programs that need only these features, and the new slice programs, pass end to end.
- No program the old compiler rejects is accepted (tier 1 check on `.err` programs).
- `sema` keeps, for every name it resolves, where it is defined and used (D12), and reads the tree through typed
  accessors (D11).

**Non-Goals:**
- Control flow (`GOTO`, `GOSUB`, `IF`, loops), numeric labels, labels in procedures, arrays: the next changes.
- Matching the old compiler's C++ text beyond the ABI (as before).
- Old error texts; diagnostics stay ours ("not supported yet" for anything outside the subset).

## Decisions

### D1. Parser: procedures are blocks; statements are dispatched by their first word
`SUB`/`FUNCTION` start a `ProcDef` node holding a `ProcHeader` (name, optional `ParamList` of `Param` nodes with
an optional `AsClause`), the body statements, and the closing `END SUB`/`END FUNCTION` statement. The body is
parsed with the same statement loop as the main module. Recovery (one error per statement still holds):
- a `SUB`/`FUNCTION` inside a procedure: error "a procedure cannot be defined inside another", and the inner
  header is treated as ending the outer block (so the rest of the file is not swallowed);
- a missing `END SUB`: error at the header, the block ends at the end of the file;
- `END SUB` outside a procedure, or the wrong `END` kind: error at that statement.
Anything after a procedure's `END SUB` belongs to the main module again: corpus 192, 193 and 196 put a SUB first
and main-module code after it, and pass with `qb64pe.exe`.

New statement nodes: `CallStmt` (`CALL name[(args)]`, and `name args` for any statement starting with a name that
is not followed by `=` and is not a keyword), `ExitStmt`, `DeclareStmt`, `SharedStmt`, `StaticStmt`,
`LabelDef` (an identifier without suffix followed by `:` at the start of a statement, when the word is not a
keyword), `OnErrorStmt`, `ResumeStmt`, `ErrorStmt`. The keyword list lives in the parser (`parser\keywords.rs`):
language words (`FOR`, `IF`, `GOTO`, …) keep their "`X` is not supported yet" error at parse time; built-in SUB
names (`CLS`, `COLOR`, …) parse as `CallStmt` and `sema` reports "not supported yet", because only `sema` knows the
built-in table (D1 of the last change: `syntax` does not depend on `builtins`).

### D2. `sema`: a declaration pass, then bodies
Pass 1 collects every `ProcDef` header into a procedure table: name, SUB or FUNCTION, result type (suffix or
SINGLE), parameters (name, type). Duplicates and reserved names are errors. `DECLARE` lines are parsed (their
header must be well formed) and otherwise ignored, as measured (`v14_declare_mismatch_ok`, `v14_declare_only`).
Pass 2 checks the main module, then each procedure, so calls may come before definitions (`qb64pe` has the same
prepass; probe: `twice&` used on line 11, defined on line 37). A main-module variable with the name of a
procedure is an error (`v14_res_sub_main_var`).

Scopes, each a "name plus type" table as in D5 of the last change:
- **main module**: as now; variables are global storage.
- **procedure**: parameters, the result variable (FUNCTION), `DIM` and implicit locals (per call), `STATIC`
  (one per procedure, global storage), `SHARED name [AS type]` (binds the main-module variable of that name and
  type, created if missing; `v14_shared_implicit`), and the `DIM SHARED` variables of the main module that come
  **before the procedure in the file** (`v14_dim_shared_after`: a later one is not seen, and the name is an
  implicit local).
A `SHARED name AS T` in a procedure creates the main-module variable; a main-module `DIM` of the same name and
type after that is "name already in use" (the old compiler fails in C++ there, `v14_shared_then_dim`, so
rejecting it accepts nothing it accepts). The main module is checked first, but `SHARED` lines are collected in
pass 1 so this order does not matter.

Inside `FUNCTION f&`, an assignment to `f` or `f&` stores the result; any other use of the name is a call
(recursion). A name used as a variable is checked against the procedure table: a function name in an expression
is a call (with or without parentheses); a SUB name or a function name with a different suffix is an error.

### D3. Reserved names
As measured (Context, reserved names): a variable, parameter or procedure is rejected with "name already in use:
`X`" (old compiler: "Name already in use (x)") when its name **without suffix** is a keyword, or the name of a
built-in (any kind in the table) that is written **without a required suffix** (`LEN`, `CLS`, `NAME`, `ERR`).
A suffix does not free such a name (`len&`). A built-in whose suffix is required (`LEFT$`, `CHR$`) reserves only
the suffixed form: `left` and `chr` are free, `left$` is taken. Today `resolve` only rejects function names.

### D4. Arguments
For each argument and parameter type `T` (`study\02` §4.1, probe 1):
- a **plain variable** (a `NameRef`, not in parentheses) of exactly type `T` → passed **by reference**;
- a string parameter and any string expression → by reference to the variable, or to a temporary;
- anything else (another numeric type, an expression, a parenthesized variable) → converted as for an assignment
  (`store`, numeric spec) into a **by-value temporary**; the callee's changes are lost (no copy-back);
- string ↔ number mismatch, wrong argument count → error.
`(n)` keeps a `ParenExpr` in the tree; that is how by-value is recognised, so `ParenExpr` must stay distinct in
the typed tree (`ExprKind::Paren` is not needed: the decision is made from the syntax node before typing).

### D5. Function results and printing
A function call has the function's type as both `ty` and `qb` (measured: `PRINT twice&(n)` is printed with
`(int32)`, not `int64`). `ERR` is typed LONG although the table says `_UNSIGNED LONG`: error numbers are below
2^31, so `(uint32)` and `(int32)` print the same; this avoids an unsigned type in this change and is noted in the
`language/error-handling` spec. `ERL` is DOUBLE. `CHR$` takes a LONG (stored with `store`) and returns a string.

### D6. The IR
```
Program { vars: Vec<Var>, procs: Vec<Proc>, main: Body }
Var     { name, ty, storage: Global | Static(ProcId) | Local(ProcId) | Param(ProcId, index) | Result(ProcId) }
Proc    { name, kind: Sub | Function(Ty), params: Vec<VarId>, result: Option<VarId>, body: Body }
Body    { labels: Vec<Label>, stmts: Vec<Stmt> }          // Label { name, at: stmt index }
Op     += Call { proc, args: Vec<Arg> } | Exit
        | SetHandler(Option<LabelId>) | Raise(Value) | Resume(Retry | Next | To(LabelId))
Arg     = Ref(VarId) | Temp(Value)                        // Temp: a fresh by-value copy, no copy-back
ValueKind += CallProc { proc, args: Vec<Arg> }
```
The statement rule is unchanged and now carries the resume semantics in words: a raising operation skips the
rest of its statement; at the statement boundary a pending error goes to the active handler; `Resume(Retry)`
re-runs the statement that raised, `Resume(Next)` continues after it, `Resume(To(l))` continues at label `l`; a
statement in a procedure resumes in that procedure. `Call` and `CallProc` always may raise. `Arg::Temp` replaces
`passN`; storage classes replace `mem_static_malloc`, `dataK` and `global.txt` placement. Labels are positions in
a body, not ops, so the IR has no jumps.

### D7. The emitter
Writes `mainK.txt`, `dataK.txt`, `freeK.txt` for each procedure K (1-based, definition order), the prototypes in
`regsf.txt`, `#include` lines for them in `main.txt`, and `retK.txt` files only if `qbx.cpp` or a fragment includes
them (the probe's `retK` are included only by `RETURN`, which is not in this change). Names follow `study\02`
§3.1 (`_SUB_BUMP_LONG_X`, `_FUNC_TWICE_LONG_TWICE`). `Arg::Temp` becomes `&(passN=<value>)` with `passN` declared
in `maindata.txt` (main) or `dataK.txt` (procedure); string temporaries pass the `qbs*` directly. Handler numbers
are assigned in order of first `SetHandler`; `mainerr.txt` gets one dispatch line per handler. Labels and the
three `Resume` forms are emitted as in the Context table. As before, `error_track_line` is not emitted.

### D8. Diagnostics for the new statements
Only errors the old compiler also reports, or "not supported yet" (the policy of `study\15` §3). Where we report
an error at a different place or with different words than the old compiler, that is fine (`CLAUDE.md`: new
error messages); where we would *accept* a program the old compiler rejects, it is a bug. Measured in task 2.1
(Context): compile errors for a wrong argument count (also `s (1, 2)` without `CALL`), a string for a number
parameter or the reverse, a string assigned to a numeric FUNCTION result, `EXIT SUB`/`EXIT FUNCTION` in the main
module, an undefined or duplicate label, a label referenced by `RESUME` inside a procedure (labels are per body;
procedures have none here), a SUB used in an expression, a function name with another suffix, a duplicate
procedure, a nested `SUB`, a missing `END SUB`. **Not** errors: `EXIT SUB` inside a FUNCTION and the reverse (both
leave the procedure), a `DECLARE` that disagrees or names nothing, `ON ERROR GOTO` inside a procedure naming a
main-module label (supported: `SetHandler` is global, so it costs nothing extra). `RESUME` outside a handler is a
runtime error 20, not a compile error. A label inside a procedure stays "not supported yet".

### D9. Corpus additions and `slice.list`
New programs in `tests\corpus\slice\` (`SOURCE.md` updated), recorded with `qb64pe.exe`:
- `s08_byref`: by reference and by value for each argument form of D4, string parameters (including a literal
  argument and a temporary), a parameter passed on to another procedure;
- `s09_functions`: INTEGER, LONG, `_INTEGER64`, SINGLE, DOUBLE, `_FLOAT` and STRING results, a function without
  a suffix (SINGLE), the result assigned with and without the suffix, `EXIT FUNCTION` before any assignment
  (result 0 or ""), calls before the definition, a zero-argument function with and without `()`;
- `s10_scopes`: locals, implicit locals, a local and a main variable with the same name, `STATIC` keeping its
  value across calls, `SHARED`, `DIM SHARED`, `DECLARE` lines;
- `s11_on_error`: `RESUME NEXT`, `RESUME label`, retry via two handlers, `ON ERROR GOTO 0`, `ERR` before and after
  `RESUME`, `ERROR` inside a SUB and inside a FUNCTION with the handler in main;
- `s12_error_in_print`: the `CHR$` cases of the Context (retry re-runs the PRINT, `RESUME NEXT` skips the rest and
  the line end), and an **untrapped** error inside a `PRINT` (no handler). Under the runner's default
  `QB64PE_NOPROMPT=y` the runtime would end the program there, so `s12_error_in_print.noprompt` holds `continue`
  (the corpus runner now reads `.noprompt` like the compile-tests runner): the runtime reports the error and goes
  on with the next statement, which pins the pipeline scenario "Error inside a PRINT" end to end.

`slice.list` first gains the 17 programs that already pass (task 1.1), then the corpus programs this change makes
pass (expected: 03, 24, 64, 69, 84, 85, 115, 124, 140, 148, 192, 193, 196, 228, 229, 230, 247, 263) and the five
new slice programs. The tier-1 test counts the list's entries instead of asserting a number.

### D10. Tier 1 keeps rejected programs rejected
`crates\driver\tests\corpus.rs` gains a test: every corpus `.bas` with an `.err` file gets at least one error
from the front end. Programs 25, 47, 48, 122, 141, 190, 191 (FUNCTION `AS type`, `BYREF`, a parameter named
`name`) are the ones this change could otherwise turn into wrong executables.

### D11. Typed accessors over the syntax tree (`study\20` §3.5, added 2026-10-04)
`sema` today reads children by position (`parts.next().unwrap()`, `child_tokens().skip(1)`). With blocks and nine
new statement kinds that gets fragile, so `crates\syntax` gains `ast.rs`: one thin, hand-written wrapper per node
kind (`AssignStmt::target()`, `DimItem::as_clause()`, `ProcDef::header()`, `ProcHeader::params()`,
`CallStmt::args()`, …). A wrapper is `struct X<'a>(Node<'a>)` with `cast(Node) -> Option<X>`; every accessor
returns an `Option` or an iterator, because a node from a statement with a parse error may lack children. No code
generator and no trait machinery at this size. The existing `sema` code moves to the accessors first, with no
snapshot changing (task 3.0), so the new statements are written against them from the start. D4 still decides
by-value from the syntax: `CallStmt::args()` yields the argument nodes, and a `ParenExpr` argument is seen there.

### D12. Symbol table (`study\20` §3.5, added 2026-10-04)
Requirement "Symbol table" of the `compiler/pipeline` delta. `sema::Program` gains `symbols: Symbols`:
```
Symbol   { kind: Var(VarId) | Proc(ProcId) | Label(LabelId), def: Span, refs: Vec<Span> }
Symbols  { list: Vec<Symbol>, by_pos: Vec<(Span, SymbolId)> }      // by_pos sorted by (file, start)
Symbols::at(file, offset) -> Option<SymbolId>                      // binary search
```
The spans are those of the name token (with its suffix), not of the statement. Recording happens where names are
already resolved: `resolve` and `dim` (variables), pass 1 (procedure headers, parameters), calls, label
definitions and uses. An implicit variable's definition is its first use. A `SHARED name AS T` line in a procedure
is a reference of the main-module variable; when it creates that variable (D2), it is its definition. The
function's own name assigned inside its body is a reference of the result variable, not of the procedure.
Statements skipped for an error record nothing (one error per statement stays the rule). Spans carry a `FileId`,
so the table needs no change when `$INCLUDE` arrives. Nothing consumes the table yet except tests: a text dump
(`sema::dump_symbols`: one line per symbol with kind, type, definition and references as line:column) checked by
`insta` snapshots in `crates\sema`, not a new mode line or `--dump` kind (those come with the language server).

## Risks / Trade-offs

- **The symbol table has no consumer yet**, so its shape is a guess at what the language server needs. → It is
  kept to the three questions every server asks (what is at this position, where is it defined, where is it
  used); anything else waits for the server.

- **Recursion of `QBMAIN` for handlers.** The runtime calls `QBMAIN(NULL)` from inside `evnt` to run a handler,
  and the handler's `RESUME` returns from that nested call. Anything our `main0.txt` does at the top of `QBMAIN`
  runs again on each error. → Keep the prologue the old compiler's shape (`S_0:;` after the `mainerr` dispatch,
  which `qbx.cpp` places); check with `s11` (errors from main, a SUB and a FUNCTION).
- **Block parsing is the first non-flat structure in the tree**; recovery bugs could swallow the rest of a file.
  → Parse snapshots for each recovery case of D1, and the corpus round-trip test.
- **`passN` and string temporaries inside a function called during a `PRINT`** (`qbs_tmp_base` handling across
  calls). → `s09` and `s12` print function results among other items; the emitted code follows the probe.
- **The keyword list** is hand-written and could disagree with the old compiler. → Task 2.1 samples it; the
  list is data in one file, easy to extend.
- **Scope of the change** is larger than the last one. → Tasks are ordered so that procedures are complete and
  tested (tier 2) before error handling starts; each group can be committed on its own.

## Open Questions

None left. Answered by task 2.1: `DIM SHARED` after a procedure is not seen by it (D2); `ON ERROR GOTO` inside a
procedure compiles when it names a main-module label, and is supported (D8).
