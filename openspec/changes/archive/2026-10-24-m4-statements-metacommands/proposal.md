# Proposal

## Why

This is step 4 of the six-step plan for the rest of M4 (user, `DECISIONS.md` 2026-10-22): **the statements and
metacommands left, with the shrink-only lists of tier 1**. The user split the step on 2026-10-23: this change,
step 4a, takes the programs that need it. A second change, 4b, takes the built-ins that no program uses yet
(about 55, such as `_SETBIT`, `_ASINH`, `MKSMBF$`, `TRON`, `FILES`, `INP`, `_CAPSLOCK`, `_FPS`), and
`$ERRORLOCATION` and `$USELIBRARY`.

`tests\verification\known.list` holds 47 programs marked "not supported yet". The demand report over them
(`qb64rust --report unsupported`, 2026-10-23) shows 9 that steps 2 and 3 left marked on purpose; they stay
marked (user, 2026-10-23). These are `v24_x268`, `v55_x74`, `x75`, `x84`, `v56_a5`, `f3`, `x08`, `x26` and
`x27`. The other 38 are this change's. The 5 programs of `tests\known_unsupported_rejections.list` are this
change's too: the old compiler rejects each, and we give them only marks.

They fall into two groups:

- **The old compiler rejects 28 of them, and we give a mark where it gives an error.** These are:
  - an unknown or bare metacommand (`$NOSUCHTHING`, `$INCLUDE:` and `$DYNAMIC` without a comment, `$MIDISOUND`,
    `$MIDISOUNDFONTS`, `$MIDISOUNDFONT` without its colon): "Syntax error";
  - `$NOPREFIX`: its own deprecation message;
  - `CALL` of a FUNCTION with a fixed-length name (`fs$5`);
  - `WRITE` with a `;`;
  - `_DEFINE AS LONG` and `DEFINT a AS LONG`;
  - `2 ^ _NEGATE x`, `x = OR` and `IF THEN PRINT`;
  - a `CONST` in the branches of a `$IF _CONSOLE_` between two settings;
  - an omitted argument of `POINT` or `_RGB32`;
  - `RUN` of a label that is also a SUB;
  - `CALL _MEMPUT` and `CALL _MEMFILL`;
  - `ON STRIG(1 GOSUB h` (a missing `)`);
  - `FRE`;
  - `SHARED` in the main module.
- **The old compiler accepts 15 of them, and we mark them.** These are:
  - `PALETTE USING`;
  - `RESUME label` inside a SUB;
  - `SWAP (a), b` and `CALL SWAP(a, b)`;
  - `READ (a)`;
  - `LBOUND((a))`;
  - `DECLARE SUB s ALIAS t ()` outside `DECLARE LIBRARY`;
  - a library FUNCTION and a program SUB of one name;
  - `CONST c = _RED(2222, 0)`;
  - `STRING * n` with a `CONST` as `n` in a `TYPE`;
  - the function `_SCREENHIDE`;
  - a `FOR` or a SUB opened in one file and closed in a file it includes;
  - a `$IF _CONSOLE_` between `$CONSOLE:ONLY` and `$CONSOLE`, which the old compiler compiles with nothing from
    its branches.

  `RUN` is in neither count: no program on the list runs it, but `v28_x86` needs it compiled before its error can
  be given, and we do not compile `RUN` at all.

## What Changes

- **A measurement family `verification\v57_*` comes first.** It records the neighbours of each form that no family
  has measured yet. Examples: other unknown metacommands; `RUN` with no argument, with a line number and with a
  file name; `RESUME` in a SUB when it runs; `SWAP` and `READ` of other parenthesised places; omitted arguments of
  other functions; more lines between two `_CONSOLE_` settings. The code follows the recordings.
- **The old compiler's stubs become its error "Command not implemented"**: `FRE`, `SETMEM`, `FILEATTR`, `IOCTL$`,
  `TRON`, `TROFF`, `RESET`, `IOCTL` and `LIST`. Its table lists each as a stub, so they leave the list of 4b.
- **Each rejected form becomes a compile error**, with the old compiler's wording as a note
  (`tools\diagnostics\old_wording.py`). An error is given only where a recording shows the old compiler rejects
  that form; a neighbour it may accept stays marked.
- **`RUN`** in its three forms: no argument, a label or line number, and a file name. Each form runs as the old
  compiler's C++ does (`sub_run_init`, `sub_clear`, the jump or `sub_run`).
- **`PALETTE USING array(index)` and `PALETTE USING array`** run with `sub_paletteusing` and the element size the
  old compiler passes.
- **`RESUME label` inside a SUB or FUNCTION** compiles, and runs as the old compiler's program does when it is
  reached.
- **`SWAP (a), b`, `CALL SWAP(a, b)`, `READ (a)` and `LBOUND((a))`** do what the old compiler's program does: the
  parentheses do not make a copy there.
- **`DECLARE SUB … ALIAS …` outside `DECLARE LIBRARY`**, and **a library procedure beside a program procedure of
  the same name with another kind**, compile as in the old compiler.
- **`_RED`, `_GREEN`, `_BLUE` and `_ALPHA` of a colour beyond 255 in screen mode 0 in a `CONST`** give the old
  compiler's value.
- **A `CONST` as the length of a `STRING * n` member** is accepted, with the constant's value.
- **The function `_SCREENHIDE`** returns what libqb returns.
- **A block or a SUB opened in one file and closed in a file it includes, or the other way round**, compiles as
  if the included text stood in place, as in the old compiler.
- **A `$IF` that names `_CONSOLE_` between two lines that set it differently** is compiled as the old compiler
  compiles it, by the recordings of `v26_x27`, `x34` and `v57`. If no rule explains the recordings, it stays
  marked with that reason.
- **The shrink-only lists:** the 5 programs leave `tests\known_unsupported_rejections.list`, so the list is
  empty. `tests\known_false_errors.list` and `tests\known_parse_gaps.list` stay empty.
- The 38 programs leave `tests\verification\known.list`. Each one that passes joins
  `tests\verification\pass.list`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `compiler/pipeline`: blocks closed in an included file; the precompiler flag between two settings; metacommands
  the old compiler does not know are its error; syntax errors the parser marked are the old compiler's errors.
- `language/control-flow`: `RUN`.
- `language/error-handling`: `RESUME` inside a procedure.
- `language/builtin-statements`: `SWAP` of a parenthesised operand; `CALL SWAP`, `CALL CLOSE` and `CALL MID$`.
- `language/file-io`: `WRITE` with `;`.
- `language/mem-blocks`: `CALL _MEMPUT` and `CALL _MEMFILL`.
- `language/data-read`: `READ` of a parenthesised variable.
- `language/screen-output`: `PALETTE USING`.
- `language/console-and-window`: the function `_SCREENHIDE`.
- `language/procedures`: a program FUNCTION as a statement, and `CALL` of one named with `$n`; `SHARED` in the
  main module; `ALIAS` outside `DECLARE LIBRARY`.
- `language/external-libraries`: a library procedure and a program procedure of one name, a SUB and a FUNCTION.
- `language/constants`: `_RED` and its kin beyond 255 in screen mode 0.
- `language/fixed-length-strings`: a `CONST` as the length of a `STRING * n` member.
- `language/arrays-and-types`: `LBOUND` and `UBOUND` of a parenthesised array name.
- `language/default-types`: `_DEFINE` and `DEFtype` without letters, or with `AS`.
- `language/builtin-functions`: omitted arguments of a function (`POINT`, `_RGB32`); the old compiler's stubs
  (`FRE`, `SETMEM`, `FILEATTR`, `IOCTL$`, `TRON`, `TROFF`, `RESET`, `IOCTL`, `LIST`).
- `language/events`: `ON STRIG(n` without its `)`.

## Impact

- `crates\syntax`: the parser's metacommand lines, the marks of `syntax_error`/`unhandled_token` for the
  recorded forms, and blocks across included files (`parser\blocks.rs`, `parser\meta.rs`, `parser\proc.rs`).
- `crates\sema`: `RUN`, `PALETTE USING`, `RESUME` in a procedure, places in parentheses (`check\io.rs`), the
  metacommand checker (`check\mod.rs`), omitted arguments (`check\builtins.rs`), `DECLARE` names
  (`check\proc.rs`, `check\library.rs`), colour constants (`colors.rs`), member lengths (`check\decl.rs`), and the
  walk of a block whose closer is in another tree.
- `crates\ir`, `crates\codegen-cpp`: `RUN` (the jump table of `run_from_line`, `sub_run`), `PALETTE USING`.
- The call-site check gets a program for each new form (`tests\callsite\`).
- Tests: `verification\v57_*` with their recordings, the front-end tests, `tests\verification\pass.list` and
  `known.list`, `tests\known_unsupported_rejections.list`, and `crates\base\src\old_wording.tsv`, regenerated.
- `SOMEDAY.md`: a row in "QB64pe behaviours to review" for each questionable behaviour kept QB64pe's way (for
  example `SWAP (a), b` swapping `a`).
