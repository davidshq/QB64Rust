# Design

## Context

For the 10 programs and what holds each back, see `proposal.md`. For the behaviour, see the delta specs. This is
how the parts stand in this compiler (2026-10-21, `37b4014`):

- **Built-in functions.** A function is compiled when it has a row in `SUPPORTED` (`sema` `builtins.rs`) with its
  `Rule`. `Rule::Plain` takes the slots, the optional mask and the result from the generated table
  (`qb64rust_builtins`), and the emitter writes the plain call. The table has every function needed here:
  `_InStrRev` (`func__instrrev`, `[?],?,?`, LONG, as `INSTR`), `_ReadFile` and `_FullPath` (`$` required, STRING),
  and `_Arcsec` … `_Csch` (`_FLOAT`). Tier 1's coverage check requires each row to be called in a `slice.list`
  program and in a `typed` front-end test. A new plain built-in comes with a program in `tests\callsite`
  (`DECISIONS.md` 2026-10-10).
- **Built-in statements.** These are rows of the statement table (`StmtRule`). `_WriteFile` is a plain `sub` of two
  slots (`sub__writefile`). `ASC` has a `sub_stub` row, as `MID$` has, because the old compiler writes it inline
  (`qb64pe.bas` 8339–8440). `MID$ … =` is the model: `check\mod.rs` sends an assignment whose left side is a call
  of `MID$` to `check\io.rs` `mid_assign`, which pushes `StmtKind::Builtin` with `StmtRule::MidAssign`. The emitter
  writes it in `codegen-cpp` `builtins.rs`, and `ir` `validate.rs` checks its slots. Today `ASC(a$, i) = n` reaches
  `implicit_array` and is marked "as a place or an array (a built-in function's name)" (`check\places.rs` 924).
- **`STOP`.** The parser reads it (`StopStmt`, `word_and_operand`). The checker marks it (`check\mod.rs` 1225).
  The old compiler checks an optional expression as `_INTEGER64` and drops it, then writes
  `close_program=1; end();`, which is `SYSTEM`'s code without the exit code and without `if (sub_gl_called)
  error(271);` (`qb64pe.bas` 9905–9925 against 9869–9898).
- **Reserved names.** `check\decl.rs` `reserved` rejects a keyword or a built-in name written without a required
  suffix when it has no suffix or `&` ("Name already in use", measured by `verification\v15_builtin_names`). With
  any other suffix it marks the name "a reserved word with this suffix is not measured". The old compiler's
  sources use `key$`, `exp$` and `base$` as variables and parameters, so `$` is accepted for at least those.
- **The follow-on rule.** After a marked declaration (`FileState::follow_on`, `check\mod.rs` 625), a name that is
  not declared is marked (`not_an_array`) instead of becoming an implicit array. The arrays of `qb64pe.bas` that
  these programs do not include (`inclinenumber`, `incname$`, `incIsInternal`, `udtxcanonmode`, …) are marked
  this way. They follow the `DIM section$, key$` of `ini.bm`, which is marked.
- **Array keys.** An array is found by `(name, element type)` (`check\places.rs` `find_array`, `array_key_ty`).
  The suffix gives the type, so `HashListName$(i)` looks for `(HASHLISTNAME, Ty::Str)`. The array is
  `REDIM … AS STRING * 256`, keyed by its fixed-length type. Without the follow-on rule, `HashListName$(i)` would
  quietly become another, implicit array. That is a wrong value, not an error, and the reason for D6.

## Goals / Non-Goals

**Goals:**

- The 10 `qb64pe/*` programs compile, build, and print what the old compiler's builds print, and join
  `tests\upstream\pass.list`.
- Every rule this change writes is one the old compiler is recorded to follow (family `v52`). A form `v52` does not
  record keeps its mark.

**Non-Goals:**

- Any other part of `qb64pe.bas` (the next step of M4, by the same method).
- `$` with a reserved name in a place the family does not measure (a `TYPE` member, a label). These
  keep their marks unless `v52` covers them.
- `$DEBUG`'s watch code after `STOP` (`vWatchOn`). `$DEBUG` is compiled without the watch (`m3-asserts-debug-zlib`).

## Decisions

### D1. The measurement family `v52` comes first, and finds the hidden marks

Two parts, both recorded with `qb64pe.exe` before any code:

- **`verification\v52_builtin_name_suffixes.py`** writes `v52_builtin_name_suffixes.txt`, a sibling of
  `v15_builtin_names.py`. It covers every keyword of `keywords.rs` and every name of `builtins.json`, with each
  suffix that `v15` did not measure: `$` (for names that need no `$`), `%`, `!`, `#`, `%%`, `&&`, `##`, `~%`,
  `~&`, `~%%`, `~&&`, `` ` ``. The form is the one of `v15`, `name<suffix> = value: PRINT name<suffix>`: compiled
  with `-z`, and each form that compiles is built and run, to tell a variable from a statement or function of the
  name. The script runs its compiles side by side (`--jobs`): the old compiler takes a temporary folder of its own
  per instance. About 12,000 forms, mostly `-z`. That is machine time in the background, not Claude's.
- **Programs `v52_x*`** cover the forms of the specs: `_INSTRREV` edges; the five functions in and out of their
  domains; `_READFILE$`, `_WRITEFILE` and `_FULLPATH$` on missing files and paths; the `ASC` statement (position
  1, out of range, an empty string, a value beyond a byte, a fixed-length string, an element, a member, a number
  as the place, a literal); `STOP` (console, window, in a SUB, after `THEN`, with an expression that calls a
  FUNCTION, with a string); `key$`, `exp$`, `base$` in `DIM`, as a parameter, in an expression, as an array
  `key$(1)`, and in `SHARED`; `name$(i)` and `UBOUND(name$)` of a `STRING * n` array, in the main module and in
  a SUB through `DIM SHARED`.

In the same task, the 10 programs are compiled by a build that turns off the follow-on rule for the run
(`QB64RUST_NO_FOLLOW_ON=1`, debug builds only, removed at the close). Each mark behind it is listed. A mark not
in the proposal gets a `v52` program and a task, or a line in the proposal's out-of-scope with the user's word.
Where a recording disagrees with a delta spec, the spec is changed in the same task.

**As measured** (task 1.3, 2026-10-21): with the follow-on rule off, `--report unsupported` over the 10 programs
names only the marks of the proposal's table (`_INSTRREV`, `base$`, `ASC(…)`, `_READFILE$`, `_WRITEFILE`,
`exp$`, `key$`, `STOP`, `_FULLPATH$` and the five functions). The arrays of `qb64pe.bas` that the proposal named
become implicit arrays, as in the old compiler. A `$` name of a fixed-length string makes no mark, but quietly
another, implicit variable or array; a search of the included sources for the `$` name of each `STRING * n`
declaration found `HashListName$(i)` (`hash.bas`) and `pathsep$` (`build.bas`, `file.bas`, of `DIM SHARED pathsep
AS STRING * 1` in the test `file.bas`). The second is a scalar: it got the program `v52_x39` and is built by D7.

One more mark showed once the others were gone (task 5.2, found at the first run of tier 1): `string.bas` includes
`const_eval.bas`, whose `FUNCTION LogicalNot&` reads `num.typ`, while `num` is a `TYPE` variable of the main
module that the FUNCTION does not share. The old compiler makes `num.typ` a variable of the FUNCTION's own, of the
default type (`v52_x65`, `x66`), as the spec "Member access and dotted names" already says for a `TYPE` variable
that is not visible; this compiler had marked it. Code: `check\places.rs` `user_head`. (In QB64pe it is a slip
that works: `num.typ` is always 0 there, and the test of `state.num.typ` beside it decides.)

The recordings of task 1.1 changed the specs in four places: `_FULLPATH$(".")` does not equal `_CWD$ + "\"` in
the old compiler's program (`v52_c_files`), so the scenario holds the last character only; the errors of the
whole-file functions are 53 and 76; `FUNCTION exp$` and `CONST key$` are rejected (`x28`, `x32`), so a procedure
or `CONST` of a reserved word with a suffix keeps its mark; and D7 covers the scalar and the two clashes. The
review of the change measured `CONST exp$` and `CONST key!` (`x67`, `x68`): rejected as `x32`, as `qb64pe.bas`
2651–2692 reads (a constant's name is looked up without its suffix), so a `CONST` of a keyword, or of a built-in
without a required suffix, is "Name already in use" with any suffix, not a mark.

*Alternative:* read the old compiler's name rules (`qb64pe.bas` 2300–2700, 9100–9250) and measure a sample.
Rejected: those rules depend on the hash table's flags for each name, and `v15` showed that reading them misses
cases. The full table costs only machine time.

### D2. `_INSTRREV`, `_READFILE$`, `_FULLPATH$` and the five functions: rows of `SUPPORTED`

`("_INSTRREV", Rule::Plain)`, beside `INSTR`, which has the same shape. `("_READFILE$", Rule::Plain)` and
`("_FULLPATH$", Rule::Plain)` go with the file functions. The five math functions go beside `_COT`, under the
comment "all believed `_FLOAT`". Their libqb entries are `static inline` in `extended_math.h`, which the generated
C++ already includes for `_COT`. If `v52` shows a result type or slot conversion that differs from the table, a
`Rule` override cites it, as the rest of `SUPPORTED` does. Code: `sema` `builtins.rs` `SUPPORTED`. The callsite
programs are `tests\callsite\func_instrrev_*`, `func_readfile_*`, `func_fullpath_*`, `func_arcsec_*` … (one per
argument type, as `func_cot_*`).

### D3. `_WRITEFILE`: a plain statement row

`("_WRITEFILE", StmtRule::Plain)` in the statement table. Two string slots, `sub__writefile(name, data)`. Code:
`sema` `builtins.rs`; nothing in `codegen-cpp` beyond `plain_stmt`. Callsite: `tests\callsite\writefile_*`.

### D4. The `ASC` statement: `StmtRule::AscAssign`, modelled on `MID$`

- **Check.** In the assignment arm of `check\mod.rs`, a left side that is a call of `ASC` (no suffix) goes to a
  new `asc_assign` in `check\io.rs`, beside `mid_assign`. It takes one or two arguments: the place
  (`place_operand`, which must be a string place: variable-length or fixed-length, a variable, an element or a
  member), and an optional position, and the value. The position and the value are numbers stored as LONG
  (`store(e, Ty::I32)`). The errors for a non-string place, a literal, a string position and a string value are
  the ones `v52` records. `implicit_array`'s mark for `ASC` goes; the mark stays for the other built-in functions.
- **IR.** `StmtKind::Builtin` with the `ASC` stub's id and `[Place, Value | Absent, Value]`. `ir` `validate.rs`
  checks the slots, as for `MidAssign`.
- **Emit.** `codegen-cpp` `builtins.rs` writes the old compiler's inline code: `tqbs=<place>; if
  (!is_error_pending()){ tmp_long=<value>; … if (tqbs->len){tqbs->chr[0]=tmp_long;}else{error(5);} }}`, and with
  a position, `tmp_fileno=<position>` and the range test (`qb64pe.bas` 8410–8436). For a fixed-length string,
  the place is the `qbs` view the old compiler's `evaluatetotyp(…, ISSTRING)` gives. The emitter already writes
  that for `MID$`'s target (`place_ref`).

*Alternative:* a `StmtKind::AscStore` node of its own. Rejected: the stub row exists to give it an id, as for
`MID$`, so `validate` and the emitter keep one path for both.

### D5. `STOP`: `StmtKind::Stop`

The checker reads the optional expression as `end_or_system` does, a number and else an error, and then drops it
(no code). It pushes `StmtKind::Stop`, a new variant with no fields, in `sema` `lib.rs` and `ir`. The emitter writes
`close_program=1; end();`. Code: `check\mod.rs` (the `StopStmt` arm), `codegen-cpp`, where `System` is written.

*Alternative:* lower it to `StmtKind::System(None)`. Rejected: `SYSTEM` writes `if (sub_gl_called)
error(271);`, which `STOP` does not, and the dump and the IR should say what was written.

### D6. Reserved names with a suffix: from the record, checked against it

`reserved` (`check\decl.rs`) replaces the mark with the rule that `v52_builtin_name_suffixes.txt` shows. The rule
is expected to depend on the word's kind (keyword, statement, function, function with a required suffix) and on
the suffix. It is written as a small table or predicate in `check\decl.rs`, and a tier 1 test,
`crates\driver\tests\names.rs` (beside the one for `v19_proc_names`), holds the predicate to every line of the
`.txt`. A word whose forms do not fit a rule of kind and suffix keeps the mark for that word and suffix: a
"keep marked" list in the same file, each entry citing its line. Where the record says "statement" (the name
compiles but runs a statement or reads a function), the form stays marked: a store is not a variable there.

### D7. `name$(i)` of a `STRING * n` array, and `name$` of a `STRING * n` variable

`v52` records both (`x39`–`x44`, `x47`–`x49`): `name$(i)` is the element of the `STRING * n` array of the name, and
`UBOUND(name$)` that array; `name$` is the `STRING * n` variable of the name, as a value and as a place, in the
main module and in a SUB that shares it. `file.bas` needs the scalar form (`pathsep$`, found by task 1.3), so both
are built. So `find_array` also tries `(name, any fixed-length string type)` when the suffix is `$` and
`(name, Ty::Str)` has no array, and the scalar lookup does the same for a variable. The two clashes are as
recorded: `DIM a$(3)` beside a `STRING * n` array `a` is "Type mismatch" (`x45`), and `DIM s$` beside a
`STRING * n` variable `s` declares nothing new: `s$` stays the fixed-length variable (`x49`). The review
measured the other declarers (`x69`–`x74`): `REDIM s$`, and `STATIC s$` in a SUB, behave as `DIM s$`, also
where `DIM` and `STATIC` are mixed in a SUB (`x73`, `x74`); `SHARED s$`
in a SUB binds a variable-length `s$` of the main module, not the fixed-length `s`. `COMMON` is not measured and
keeps a mark. Code:
`check\places.rs` `find_array` and the scalar lookup beside it, and the declaration of a `$` name in
`check\decl.rs`.

### D8. The lists and the corpus

- A slice program `tests\corpus\slice\s154_qb64pe_sources_builtins.bas` calls each new function and statement
  (one call per row, as the coverage check needs) and is recorded with `qb64pe.exe`. Its file round trip uses a
  name under the test's own folder.
- The 10 programs go onto `tests\upstream\pass.list`. Each `v52` program that passes goes onto
  `tests\verification\pass.list`, and every other one onto `known.list` with its reason.

## Risks / Trade-offs

- [More marks behind the follow-on rule than the proposal lists] → D1 runs the 10 programs without the rule before
  the code is written. What it finds is added here or, with the user's word, left out of scope.
- [A reserved name with a suffix accepted where the old compiler rejects it] → D6 holds the rule to every recorded
  form in tier 1. Tier 1's front-end runs over every accepted input set catch the opposite (a false error).
- [`_FULLPATH$` and `_READFILE$` results depend on the machine's folders] → The slice program prints only what
  does not depend on the folder: the length of a file read back, whether `_FULLPATH$(".")` ends with `\`, and
  equality with `_CWD$`. An error number is printed through a handler.
- [The `v52` name script runs long] → It runs in the background with `--jobs` (working rule 9, "Work goes on
  during a run"). The `v52_x*` programs, which the code needs first, are recorded before it.
- [`STOP` in a program with a window ends without the prompt] → `v52` records it. The window case is a screen
  program only if the console form cannot show it.
