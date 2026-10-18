# Support record: what the compiler supported, M2 to M3 step 16 (to 2026-10-16)

**A closed record. Do not add to it.** This is the catalogue "What the compiler supports so far" that
`crates\README.md` held until 2026-10-16, moved here unchanged when the README was cut to an overview
(`study\31` item 2, `DECISIONS.md` 2026-10-16). Each feature was written up there again by every change, beside its
specs and its `design.md`; from now on the merged specs under `openspec\specs\` hold what a program may use, the
change's `design.md` and `tasks.md` how and where it was built, and `SOMEDAY.md` what it left marked. Kept because
it names, feature by feature, the code that implements it.

Paths of code below are relative to `crates\` (`sema\src\builtins.rs` is `crates\sema\src\builtins.rs`); "below"
and "above" refer to this list. Later changes may have changed what a line says is still marked.

What the compiler supported by the close of `m3-sound-handles`:


- the first slice (`m2-workspace-and-slice`): `$CONSOLE:ONLY`, comments, `:`, `DIM` of scalars (`INTEGER`,
  `LONG`, `_INTEGER64`, `SINGLE`, `DOUBLE`, `_FLOAT`, `STRING`), `[LET] v = e`, implicit variables with suffixes,
  `PRINT` with `;`, `,` and the auto-semicolon, `END`, numeric and string literals, unary `-`, `+ - * /`,
  parentheses, string `+`, and `INSTR`;
- procedures (`m2-procedures-and-errors`): `SUB` and `FUNCTION` with parameters, calls with and without `CALL`
  (by reference or by value as in QB64pe), function calls in expressions, `EXIT SUB`/`EXIT FUNCTION`, `DECLARE`
  (ignored, as QB64pe does), local and implicit variables, `STATIC`, `SHARED`, `DIM SHARED`, reserved names;
  `SYSTEM` without an exit code;
- error handling: labels in the main module, `ON ERROR GOTO label` and `ON ERROR GOTO 0` (also inside a
  procedure), `RESUME`, `RESUME NEXT`, `RESUME label`, `ERROR n`, `ERR`, `ERL`, `CHR$`;
- every operator (`m2-control-flow-slice`): comparisons (`=`, `<>`, `<`, `>`, `<=`, `>=`, numbers and strings),
  `NOT`, `AND`, `OR`, `XOR`, `EQV`, `IMP`, `_ANDALSO`, `_ORELSE`, `_NEGATE`, `\`, `MOD`, `^`, typed as QB64pe types
  them (`sema\src\check\ops.rs`); an `IMP` whose left operand is an `IMP` is "not supported yet" (QB64pe
  computes `a IMP b IMP c` as `a OR b OR c`);
- `CONST` (`m2-control-flow-slice` task 4.2): the old compiler's constant evaluator (`sema\src\consteval.rs`, the
  walk in `sema\src\check\constants.rs`), its typing (`_INTEGER64`, DOUBLE, or the suffix's type), main and
  procedure scopes as measured; uses become literal nodes. Floats are computed in `f64` and checked for being the
  value the old compiler's `_FLOAT` path gives; where that cannot be shown, and for the evaluator's functions,
  the constant is "not supported yet";
- `OPTION _EXPLICIT` and `_EXPLICITARRAY`, program-wide wherever they stand, as measured (an array used
  without `DIM` is an error under either);
- control flow (`m2-control-flow-slice`): `IF` in both forms with `ELSEIF`/`ELSE`, `IF c GOTO label`, `FOR …
  NEXT` (with `STEP`, `NEXT j, i`, the old compiler's temporaries and their widths), `DO … LOOP` in its four
  forms, `WHILE … WEND`, `EXIT FOR`/`DO`/`WHILE`, `GOTO`, `GOSUB`, `RETURN` and `RETURN label`, labels in every
  body (main and procedures, also inside blocks). The IR is flat with explicit jumps (`ir\src\lower.rs`); a
  raising block header behaves as in QB64pe (the pending-error rule, `ir\src\lib.rs`);
- arrays and `TYPE` (`m2-arrays-and-types`, `sema\src\check\places.rs`): static arrays of the main module (`DIM`
  and `DIM SHARED`, one or more dimensions, `lower TO upper`, constant bounds; which bounds count as constant is
  under group 4 below) of the numeric types, `STRING` and user types; element read and write (indexes rounded half to even, error 9 outside the
  bounds), elements passed by reference, `LBOUND`/`UBOUND`; `TYPE` blocks of the main module with numeric and
  nested members, used anywhere in the file; `TYPE` variables in every storage class; member read and write
  (`p.x`, `p.a.b`, `a(i).m`), members passed by reference, dotted plain names (`a.b` without a `TYPE` variable
  `a`). Each place has its own store rule (`ir\src\lib.rs`); a member store into an element with a bad index
  stores nothing (`DIVERGENCES.md` D-004; member arrays: group 9 below). The IR shares `sema`'s value tree
  (`Expr`, `Place`, `ArrayRef`, `Arg`);
- the preprocessor and included files (`m2-parser-breadth`): `$IF`/`$ELSEIF`/`$ELSE`/`$END IF`/`$LET`/`$ERROR`
  evaluated as the old compiler does (Windows 64-bit, version 4.7.0; a branch not taken is kept as
  `InactiveCode`), comment `$INCLUDE` with the old compiler's lookup, `$INCLUDEONCE`; the program is checked and
  compiled across its files, and a runtime error in an included file names it as QB64pe does;
- procedures named like built-ins as measured (`verification\v19_proc_names`): a SUB may take a built-in
  function's name, a FUNCTION a built-in statement's;
- 107 built-in functions (`m2-core-builtins`; the file functions, `RND`, `TIMER`, `SHELL`, `COMMAND$`, `ENVIRON$` and
  14 added by demand with `m2-builtin-statements`; `_DEST`, `_SOURCE`, `_CONSOLE` and the ten window functions
  with `m2-console-screenhide`, the five screen functions with `m3-window-output`, ten more with
  `m3-get-put-bload`, `PLAY` with `m3-sound`, eight with `m3-console-files`, all below), checked by one
  table-driven checker: `sema\src\builtins.rs` lists
  them, each with its `Rule` (one per kind of special-casing in the old compiler's `evaluatefunc`, not per
  function), and gives each call a held type (the C++ type of the libqb call) and a believed type (the old
  compiler's); `sema\src\check\builtins.rs` checks arity, argument kinds and the conversion of each argument to
  its slot (LONG as a store, DOUBLE exactly, `_FLOAT` as it is, any-numeric cast to its believed type);
  `codegen-cpp\src\builtins.rs` writes each rule as the old compiler does. String: `LEN`, `LEFT$`, `RIGHT$`,
  `MID$`, `ASC` (one and two arguments), `CHR$`, `STR$`, `VAL` (also with a type), `STRING$`, `SPACE$`, `LTRIM$`,
  `RTRIM$`, `_TRIM$`, `UCASE$`, `LCASE$`, `HEX$`, `OCT$`, `_BIN$`, `_TOSTR$`, `INSTR`; math: `ABS`, `SGN`, `INT`,
  `FIX`, `SQR`, `SIN`, `COS`, `TAN`, `ATN`, `LOG`, `EXP`, `CINT`, `CLNG`, `CSNG`, `CDBL`, `_ROUND`, `_PI` (also
  bare), `_ATAN2`, `_HYPOT`; files: `EOF`, `LOF`, `LOC`, `SEEK` (the last three held `_INTEGER64`, as libqb
  returns them, and believed LONG), `FREEFILE` and `_CWD$` (bare only), `_FILEEXISTS`, `_DIREXISTS`; `RND` and
  `TIMER` (bare or with their argument; `TIMER` held DOUBLE and believed SINGLE); the `SHELL` function (an
  `_INTEGER64`, the command's exit code), `COMMAND$` (bare or with an index), `ENVIRON$` (a name or an index: two
  libqb entries, the rule `StrOrIndex`), `_STARTDIR$` (bare only); `_ACOS`, `_ASIN`, `_SINH`, `_COSH`, `_TANH`,
  `_CEIL` (`std::` functions, held as C++ overloading gives for the argument), `_COT`, `_CSC`, `_SEC`, `_D2R`,
  `_R2D` (held DOUBLE), all believed `_FLOAT`; `_STRCMP`, `_STRICMP`; and `LBOUND`, `UBOUND`, `ERR`,
  `ERL`. A function that needs an argument, named without one, is an error, as in QB64pe. Adding one of the
  remaining built-ins is a row plus tests where an existing rule fits, and a program in `tests\callsite` (the
  call-site check, below); tier 1 requires each listed one in a `slice.list` program and a `typed` test. Left
  marked on purpose at the time: `_ERRORLINE` and the other functions the old compiler writes itself (`MKL$`,
  `CVL` and their kin came in with `m3-mem`; `_MIN`, `_MAX`, `_CLAMP`, `_IIF` and the shifts with
  `m3-console-files`, below);
- `SELECT CASE` and `SELECT EVERYCASE` (lists, `TO`, `IS`, `CASE ELSE`; the selector copied once into a static
  hidden variable unless it is a plain variable, as QB64pe; `DIVERGENCES-QB45.md` Q-002) and `ON n GOTO`/`ON n
  GOSUB` to labels (`n` above 255 falls through, Q-001), lowered to the IR's branches and jumps
  (`ir\src\lower.rs`);
- the full numeric type set (`m2-numeric-types`): `sema::Ty` has 17 numeric variants (`_BYTE`, `INTEGER`, `LONG`,
  `_INTEGER64`, each also `_UNSIGNED`; `_OFFSET` and `_UNSIGNED _OFFSET` as their own variants; `_BIT * n` and
  `_UNSIGNED _BIT * n` for n from 1 to 64; `SINGLE`, `DOUBLE`, `_FLOAT`), an enum with an explicit rank and no
  derived order (design D3). Every `AS` spelling and suffix, in every place: scalars of every storage class, static
  arrays, `TYPE` members, parameters (by reference also across signedness, as measured), FUNCTION results,
  `CONST`, `FOR`, `SELECT CASE` and the special-cased built-ins. Values compute as the old compiler's C++ does,
  tracking each value's [held and believed type](../GLOSSARY.md#held-type--believed-type) (`sema\src\check\ops.rs`
  `held`, `sema\src\literal.rs`); `conversion` (`sema\src\lib.rs`) gives each store's rule, including the `_BIT`
  store mask and sign extension. Checked against `qb64pe.exe` by the differential programs
  (`tests\differential`, made by `difftest`);
- fixed-length strings (`m2-numeric-types` group 7, design D6): `STRING * n` (n a number or an integer constant,
  read in 32 bits) and `name$n` variables of every storage class, static arrays of them and `TYPE` members, as the
  old compiler builds them: a fixed `qbs` over n NUL bytes per variable, a temporary one over an element's or
  member's bytes, stores by `qbs_set` (which cuts and pads); their values are plain `STRING`s (`Ty::held_value`),
  and one passed to a `STRING` parameter goes by reference, also in parentheses;
- built-in statements (`m2-builtin-statements`): a statement that is one call of the runtime is a row in
  `sema\src\builtins.rs` (`STATEMENTS`, the twin of the function list) and `Op::Builtin` in the IR: the table entry
  of the form written and one slot per argument or choice of its template (a value, a place, the word chosen, or
  absent). `sema\src\check\builtins.rs` matches the statement against its forms as the parser did and converts each
  argument to its slot; `codegen-cpp\src\builtins.rs` writes the call by the old compiler's template rule
  (`builtins\src\passing.rs`, a port of `seperateargs`' pass rules: which parts are C arguments, `NULL` for an
  absent one, the bits of the `passed` mask). The arguments are evaluated in order and the call is made also
  after a raising one, as measured. So far: `KILL`, `MKDIR`, `RMDIR`, `CHDIR`, `NAME`, `ENVIRON` (also `CALL
  KILL(…)`), `OPEN` in both forms with every mode, access and lock word, `SEEK`, `RANDOMIZE` (with a seed, with
  `USING`, and without one: the runtime then asks for it), `SHELL` in its three forms (plain, `_HIDE` first,
  `_DONTWAIT` first; with or without a command), and three with a rule of their own: `CLOSE` (one call per file number), `SWAP` (two places of one
  type but for signedness, any two strings, one `TYPE`; never a `_BIT`) and the `MID$` statement. Tier 1 requires each listed statement in a `slice.list` program and an `ir` test;
- sequential file I/O and the program's data, as operations of their own in the IR because they have items or
  targets (`sema\src\check\io.rs`, `codegen-cpp\src\io.rs`): `PRINT #` (not `USING`; `TAB` and `SPC` are functions that give a string, so plain items), `WRITE` to a
  file and to the console, `INPUT #` and `LINE INPUT #` into variables, elements and members (a pending-error
  check after each item or target, as the old compiler); `DATA` (the items of the whole program in file order,
  procedures and included files where they stand), `READ` (no check between its targets, as measured) and
  `RESTORE` to the start or to a label of any body; console `INPUT` and `LINE INPUT` (the prompt forms, the `;`
  before the prompt, one `,` after the last target): the compiler hands the runtime the type and address of each
  target, and the runtime reads, converts and stores (it never asks again; `study\00` §5). A corpus program that
  reads the console has its answers in `<name>.stdin` and ends with `SYSTEM` (`tests\corpus\README.md`);
- default types (`m2-defxxx-common-arrays` group 2): `DEFINT`, `DEFLNG`, `DEFSNG`, `DEFDBL`, `DEFSTR` and
  `_DEFINE … AS type` (every type a suffix can give, `STRING * n` and `_BIT * n` included). One table of 27
  entries, A to Z and `_`, follows the file and nothing else (`Checker::defaults`, `sema\src\check\decl.rs`):
  a statement applies to every line below it, run or not, also below the block or procedure it stands in. Pass 1
  walks the whole file to know the table at each procedure header (parameters and FUNCTION results); pass 2
  sets it at each statement. A name without a suffix or `AS` clause is the name plus the default at its line
  (`default_ty`), so the same plain name above and below a statement is two variables, as in QB64pe; a
  variable declared with `AS`, a `CONST` and a FUNCTION's name are not typed by it;
- group 3 of the same change: a variable's `Shape` (`Scalar`, `Static` with bounds, `Dynamic` with a number of
  dimensions; `sema\src\lib.rs`, shared by the IR) in place of a list of bounds, only static arrays made so
  far; `OPTION BASE 0`/`1` and `'$DYNAMIC`/`'$STATIC` as checker state that follows the file like the default
  types (`OPTION BASE` folded into the lower bound, the number 0 in plain digits staying 0 to 0); the type
  before the names in `DIM`, `DIM SHARED`, `STATIC` and `SHARED` (`DIM AS LONG a, b(3)`);
- group 4 of the same change, dynamic arrays in the main module. **What an array is, is settled by the first
  declaration of it in the file** (`declare_array`, `sema\src\check\places.rs`): static when that is a `DIM`
  whose bounds are all constant *by their spelling* (one number, a minus before it included, or one `CONST`
  name, in parentheses or not) and `$DYNAMIC` is not in effect; dynamic otherwise, so also `DIM a(2 + 3)`. A
  static array's `DIM` is no statement. Every other `DIM`, `REDIM`, `REDIM _PRESERVE` and `REDIM _RETAIN` is an `Allocate` (its
  mode, and a lower and an upper bound per dimension as `_INTEGER64` values), `ERASE` a `Clear` of a static
  array or a `Release` of a dynamic one: the same three in the typed tree and the IR, which names no
  descriptor and no size (`ir\src\lib.rs` states the order of an allocation's checks and errors;
  `ir\src\validate.rs` checks array kind, number of bounds and their type). `REDIM` without a type keeps the
  type of an array declared with `AS`; `REDIM [_PRESERVE | _RETAIN] SHARED` (any other order of the words an error); `DIM a()`, `REDIM a()` and `REDIM a AS
  type` declare a scalar, as in QB64pe; a `REDIM` or `ERASE` in a procedure reaches a `DIM SHARED` array. The
  emitter (`codegen-cpp\src\arrays.rs`, `decl.rs`) makes a dynamic array's descriptor "not allocated" (every
  use raises error 9) and writes the allocation after the old compiler's sequence: error 10 for a static array
  and, in a `DIM`, for an allocated one; the new bounds in a descriptor of their own; error 5 for a dimension
  without an element, without leaving; error 257 ("Out of memory", which ends the program) for what cannot be
  held; the new block made before the old one is released; `_PRESERVE` copying by flat position, `_RETAIN`
  (`m3-redim-retain`) by the indexes: the old compiler's walk over the overlap of each dimension, with the old
  descriptor kept where the old block is captured. `_BIT`
  arrays, static and dynamic, are packed storage read with `getbits` and stored with `setbits`. A dynamic
  array of the main module is not released when the program ends;
- group 5 of the same change, arrays used without `DIM` (`implicit_array`, `sema\src\check\places.rs`):
  `name(…)`, `LBOUND(name)` and `UBOUND(name)` of a name that is no visible array make one, `OPTION BASE` to 10
  per index written, of the suffix's type or the default type at that line, where the name may be a variable,
  no SUB or FUNCTION has it and the spelling calls no built-in function (`chr(1)` and `__foo(1)` are arrays; a
  scalar or a `CONST` of the name does not stand in the way). In the main module it is a static array; below
  `$DYNAMIC` a dynamic one whose `Allocate` is *joined* to the statement that names it first (the IR puts it
  first in that statement's operations, so an error it raises skips the statement's stores and `RESUME NEXT`
  goes on after the statement, as measured); in a SUB or FUNCTION a local dynamic array that nothing allocates,
  so every use raises error 9, as in QB64pe (`DIVERGENCES-QB45.md` Q-009; its descriptor is made on each call
  and its `_MEM` lock freed at the end). `OPTION _EXPLICIT` and `OPTION _EXPLICITARRAY` make it an error. Not
  made an array, and still marked: a name used after a declaration that was marked (the follow-on rule), a name
  with one leading underscore, the name of a SUB or FUNCTION, a built-in function's own spelling as a place,
  below `$DYNAMIC` a first use where the old compiler's allocation is not the first thing the statement does
  (the line of a block, `IF`, `FOR`, `DO`, …; a later item of `PRINT` or `READ`, an item of `WRITE` or `PRINT #`,
  a target of `INPUT`), and anywhere a first use in a bound of `REDIM` (`NoJoin`, `sema\src\check\mod.rs`);
- group 6 of the same change, arrays in procedures (design D6, "As built"): a `DIM` or `REDIM` in a SUB or
  FUNCTION makes a local dynamic array, with constant bounds too, new on every call (its descriptor is made per
  call; its strings, its block and its lock are freed on every way out, in `freeK.txt`). `STATIC a(n)` is a
  dynamic array that lives for the program and is allocated when the program starts: the shape carries its
  bounds (`Shape::Dynamic`, `at_start`) and the emitter writes the allocation into `maindata.txt`, which runs
  again when an error handler is entered, so an array that `ERASE` released comes back there, as in QB64pe.
  `STATIC a()` lists a dynamic array that lives for the program; a `DIM`, a `REDIM`, an index or `LBOUND` gives
  it its number of dimensions. `SHARED a()` names the main module's array, which must stand above the procedure.
  **`STATIC` after a procedure header** makes the procedure's own variables and arrays live for the program,
  down to its first `STATIC` statement only, as in QB64pe (`DIVERGENCES-QB45.md` Q-010). `DIM SHARED` and
  `REDIM SHARED` in a procedure, `STATIC` in the main module and `SHARED a(3)` are errors. Still marked: `STATIC
  a(n)` with a bound that is not one number or constant, `STATIC` beside a shared main array of the name, `REDIM
  e AS type` without parentheses in a procedure.
- group 7 of the same change, array parameters and `COMMON` (design D6, "As built in group 7"): a parameter
  written `name()` takes a whole array by reference (`Arg::Array`; in C++ the caller's descriptor), static or
  dynamic, of any element type an array may have; the element types are compared as QB64pe compares them
  (width or user type, `_BIT`, string, fixed length, float; not signedness). **Its number of dimensions is found
  after every body is checked** (`solve_dims`, `sema\src\check\mod.rs`): every index count, `DIM` or `REDIM`
  of it and every array passed, above or below the procedure, must agree, and the error stands at the later of
  two that disagree. `ERASE` of a parameter is `EraseParam`: it clears a static argument and releases a dynamic
  one. A name that names no array becomes one of one dimension when it is passed in the main module, and is
  an error in a procedure. `COMMON` declares as `DIM` does, `COMMON SHARED` as `DIM SHARED`; `COMMON a()` lists
  a dynamic array that a `DIM` or `REDIM` below it allocates. A block name and `COMMON` in a procedure are
  errors. Still marked: `CHAIN`, `COMMON a()` of an array that exists already, an array parameter named `t$n`
  or typed `_UNSIGNED STRING`.
- group 8 of the same change, the rest of `TYPE` (design D9, "As built in group 8"): a member `AS STRING`, a
  string of its own in every value of the type (in C++ 8 bytes of the value, the pointer to its string; made
  where the value is made, released where it goes; `Emitter::parts`, `codegen-cpp\src\decl.rs`); `LEN` of a
  value of such a type is an error. `q = p` between two places of one user type is `CopyWhole`: every member,
  not guarded (an index outside raises 9 and the first element stands in), the target found before the source,
  and for a type that holds a `STRING` both found again for every part, as QB64pe (`SOMEDAY.md`). A parameter
  `AS` a user type takes a variable, an element or a member of exactly that type by reference (`Arg::Ref`; in
  C++ its address); anything else is an error, a place in parentheses too.
- group 9 of the same change, member arrays (design D8, "As built in group 9"): a `TYPE` member declared as an
  array (`v(0 TO 3) AS LONG`, `AS INTEGER q(2), r`), of any member type and any number of dimensions; its
  bounds are part of the type (`Member::bounds`), read where the block stands (`OPTION BASE` as it is there,
  collected before the blocks are read). **`ArrayRef`** (`sema\src\lib.rs`) names an array wherever one is
  named as a whole, an array variable or a member array of a place: in an element (`t.v(i)`, `a(i).v(j)`,
  `x(0).n(1).q(2)` are one place each), in `LBOUND`/`UBOUND`, in `Clear` and as an array argument. The
  elements lie in the value (in C++ from the member's offset, first dimension fastest), so `LEN`, a new
  variable, a local on each call and a whole-value copy take them along; nothing allocates or releases them.
  `REDIM` with the declared bounds and `ERASE` are a `Clear` (not guarded, as QB64pe: an index outside on the
  way raises 9 and element 0's member array is cleared), `REDIM _PRESERVE` and `REDIM _RETAIN` are nothing; other bounds are an
  error. `LBOUND`, `UBOUND` and `LEN` do not evaluate the indexes of the place. A whole member array (`fill
  t.v()`) is passed as a static array over its bytes (a descriptor made for the call). **A store through a
  member with an index on the way** (`a(i).m`, `t.v(i)`) evaluates a number before the indexes and a string
  after them, as QB64pe's program does, and stores nothing after an index outside (`DIVERGENCES.md` D-004;
  `store_through_member`, `codegen-cpp\src\stmt.rs`). `SWAP` takes such a place. Still marked: a bound that
  names a `CONST` (the blocks are read before the constants), a member array of an array element beside other
  targets of one `ERASE` or `REDIM` (such a place as a target of `INPUT`, `READ` or the `MID$` statement and the
  markers: `m3-typefields` below).
- **Signs** (`syntax\src\parser\expr.rs`, `signs_after`; measured, `verification\v25_*`): there is no unary
  plus, but QB64pe removes signs from an expression before it reads it, and so does the parser: a `+` right
  after a `+` or a `-`, then two `-` right after an operator. The dropped tokens stay in the tree. In a
  `CONST` a run of `+` and `-` is one sign.
- **A console and a window** (`m2-console-screenhide`; measured, `verification\v26_*`): a program
  with `$CONSOLE` is compiled and built with a console and a window, one with `$CONSOLE:ONLY` (with or without
  `$CONSOLE`) with a console and no window; `$SCREENHIDE` and `$SCREENSHOW` say whether the window starts
  hidden, the last one in the file counting. The four count wherever they stand in compiled code (any line, a
  procedure, an included file, after the last statement that runs); each `$CONSOLE:ONLY` selects the console
  where it stands. `sema` matches them exactly (`check\mod.rs`, `console_meta`): anything else on the line is
  an error, as in QB64pe, and so it is for `$INCLUDEONCE` (a tab or a word after it). The mode is `Program::settings` in `sema` and in the IR (`ir::Settings`: console,
  window, hidden at the start, the optional parts of the runtime; the IR dump's first line). The emitter
  writes the two start-up globals from it, the driver the make flags (`driver\src\build.rs`, `make_flags`:
  `DEP_CONSOLE_ONLY=y` without a window, `DEP_CONSOLE=y` with a console, as QB64pe passes them), which are
  part of the build cache's key. A program with a window links another variant of libqb, built once into the
  clone's git-ignored folders. **Every metacommand** must be the first thing on its line: after a statement
  and a colon, or with a tab before it, it is an error (`syntax\src\parser\meta.rs`); in a `$IF` branch that
  is not taken such a line is skipped text. **A program with neither `$CONSOLE` nor `$CONSOLE:ONLY`** is one
  with a window and no console (`m3-window-output`; measured, `verification\v27_*`): `console=0`, no make flag,
  so the clone's `Makefile` links the libqb with the window and `-mwindows`; `$SCREENHIDE` alone hides the window.
  `PRINT`, `WRITE`, `INPUT`, `LINE INPUT`, `END` and a runtime error are the same calls as in a console program
  (what differs is libqb's); what they do in the window is checked by screen programs
  (`tests\corpus\screen`, on `slice.list`). **`_DEST`, `_SOURCE` and `_ECHO`** are statement rows
  and `_DEST`, `_SOURCE` and `_CONSOLE` function rows of `sema\src\builtins.rs` (the handle a LONG value, the
  functions LONG and called by their bare names), so a `$CONSOLE` program prints to its console after `_DEST
  _CONSOLE` and reads it with `INPUT` and `LINE INPUT`. `CALL _DEST(h)`, `CALL _SOURCE(h)` and `CALL
  _ECHO(t$)` are the same statements (a statement whose template is one argument is read like a SUB call after
  `CALL`, `check\proc.rs`). `_ECHO` in a program that turns out to have no console is an error, reported after
  the walk. The statement `_CONSOLE ON`/`OFF` is "not supported yet".
- **Window built-ins, compiled and never run** (`m2-console-screenhide` group 4; `sema\src\builtins.rs`,
  `WINDOW_BUILTINS`): the statements `_SCREENHIDE`, `_SCREENSHOW`, `_SCREENICON`, `_TITLE`, `_ICON`,
  `_MOUSESHOW`, `_MOUSEHIDE`, `_MOUSECURSOR`, `_WINDOWSIZELIMIT`, `_FREEIMAGE` and the functions
  `_SCREENEXISTS`, `_SCREENICON`, `_TITLE$`, `_DESKTOPWIDTH`, `_DESKTOPHEIGHT`, `_SCREENX`, `_SCREENY`,
  `_WINDOWHASFOCUS`, `_NEWIMAGE`, `_WINDOWHANDLE` (an `_UNSIGNED _OFFSET`, bare or with one argument; `_NEWIMAGE`
  and `_FREEIMAGE` are run since `m3-images-fonts-mouse`), each a
  row, in every form of its template. What they do shows only in a window, so no test runs them: their slice
  program holds them below its `SYSTEM`, and each has call-site programs (`tests\callsite`), which the coverage
  test requires for the names of `NEVER_RUN` (the list's name since `m3-sound`, when `BEEP` joined it). A
  built-in's `dependency` in the table is the optional part
  of the runtime it needs (`sema::RuntimePart`, in `Program::settings`): `_ICON` gives the make flag
  `DEP_ICON=y`, `SOUND`, `PLAY` and the sound handles `DEP_AUDIO_MINIAUDIO=y`; a built-in whose part no build provides yet is "not
  supported yet". Still marked: the function `_SCREENHIDE`, and `_SCREENEXISTS` as the
  direct argument of `HEX$`, `OCT$` or `_BIN$` (QB64pe passes another width than for a LONG).
- **The screen statements and functions** (`m3-window-output`; design D2 of the change holds the call of every
  form, read from the old compiler's C++; accepted and rejected forms `verification\v27_*`). Each is a row of
  `sema\src\builtins.rs` and the plain call of its table entry and template, in every form the old compiler
  accepts: the text screen (`SCREEN` with its page arguments and `_MANUALDISPLAY`, `CLS`, `COLOR`, `LOCATE`,
  `WIDTH` also with `#` and `LPRINT`, `VIEW PRINT`), drawing (`PSET`, `PRESET`, `LINE` with `STEP`, `B`, `BF`
  and a style, `CIRCLE`, `PAINT`, `DRAW`, `VIEW`, `WINDOW`, `PALETTE`, `PCOPY`), pacing (`_LIMIT`, `_DELAY`,
  `SLEEP`, `_DISPLAY`) and the functions `INKEY$` and `CSRLIN` (bare only), `POS(n)`, `POINT(x, y)` and
  `POINT(n)` (a DOUBLE), `SCREEN(row, column[, flag])` (an `_UNSIGNED LONG`). `VIEW` is two table entries, the
  graphics viewport and `VIEW PRINT`; `SCREEN` is a statement and a function, told apart by position. Slot
  kinds (`Slot`): a coordinate, a radius and an angle are passed in the argument's own type (the table's
  `_FLOAT` and SINGLE; libqb takes a `float` or `double`); a colour the table types `_UNSIGNED LONG` is
  `Slot::ULong`, a store into an `_UNSIGNED LONG`; the fill of `PAINT` is `Slot::ColorOrTile`, a number (an
  integer as it is, a float rounded) or a tile string, written with the old compiler's cast (`(uint32)`,
  `(qbs*)`), which picks one of libqb's two `sub_paint`. A form the template does not give is an error of the program (the parser's "matches none of its
  forms"). **A built-in statement's name written `name (…) = …`** is an assignment to the old compiler and so
  "name already in use" here too, for every supported statement (`not_an_assignment`,
  `sema\src\check\builtins.rs`; before this change `SEEK (1) = 2, 3` compiled as `SEEK` with a comparison).
  **A built-in statement's name and a colon at the start of a line** (`CLS : LOCATE 1, 1`), which the parser
  reads as a label, is the statement with no argument, as in the old compiler (`bare_stmt`; a statement that
  needs an argument is an error there; `tests\frontend\stmt_name_colon_ir.bas`).
  Unlike the window built-ins these run: their slice programs are the screen programs of
  `tests\corpus\screen`, checked by the state of the screen they leave, and each form has a call-site program
  (301 of them). Still marked: an argument left out of a function call (`POINT(, 2)`), an array named `WIDTH`
  (`SOMEDAY.md`).
- **Sound, `$EXEICON`, and a statement with a template after `CALL`** (`m3-sound`; the facts in `study\00` §5,
  `verification\v29_*`; with it Civil War Strategy compiles with no diagnostic and builds):
  - **`SOUND`, `PLAY` and `BEEP`** are three rows of `STATEMENTS` and `PLAY` one of `SUPPORTED`
    (`sema\src\builtins.rs`), each the plain call of its table entry and template in every form the old
    compiler accepts: `SOUND` with no argument, with two to seven and with `WAIT` or `RESUME` as its last word,
    `PLAY` with one to four strings, the function `PLAY[(voice)]` (a DOUBLE, also bare). `PLAY` is no event
    word to the parser any more (`syntax\src\parser\errors.rs`): `ON PLAY(n) GOSUB label` is `ON … GOSUB`
    with the function, and `PLAY ON` is an error, both as in the old compiler. `SOUND` and `PLAY` need
    `RuntimePart::Audio`, the make flag `DEP_AUDIO_MINIAUDIO=y`, written last (`driver\src\build.rs`,
    `make_flags`); `BEEP` needs nothing. **Tests make no noise:** the slice programs `s74_sound_queue`,
    `s75_sound_wait` and `s76_sound_errors` play at volume 0 and print what a program can observe (what a
    statement adds to `PLAY(n)` in tenths of a second, which statements hold the program as yes or no against
    one-sided limits, error 5); `BEEP`, the system's bell, is on `NEVER_RUN`: compiled, compared by its
    call-site programs, never reached.
  - **`$EXEICON:'name'`** (`sema\src\check\mod.rs`, `exe_icon_meta`): the statement `_ICON` with no argument
    where the line stands, and the icon among the program's settings (`Settings::exe_icon`: the name as
    written, taken as UTF-8; the IR carries only that there is one, as its dump says). A name that starts with
    `./` or `.\` is looked for in the folder of the main source file (the rest added to the folder as text, as
    the old compiler does), an absolute one (`C:\…`, `\\…`) as written; any other relative name is
    "not supported yet" (the old compiler looks in its own folder, `SOMEDAY.md`). `sema` reads no file: that
    the file is there is checked by the driver after the front end (`check_exe_icon`, `driver\src\lib.rs`),
    for every command that reports diagnostics; the language server does not check it. For the build the
    driver copies the file to `<build>\temp\icon.ico`, writes `icon.rc` beside it and passes `DEP_ICON_RC=y`
    (`build::write_icon`); both files are part of the build key.
  - **A built-in statement with a template after `CALL`** (`syntax\src\parser\template.rs`, `template_call`):
    the old compiler's rule, for every such statement: after the name nothing, or one pair of parentheses
    around all that follows, whose content is matched by the statement's template (`CALL LINE((1, 2)-(3, 4),
    5, BF)`, `CALL SOUND(440, 1 WAIT)`, `CALL CLS`). The match is a `BuiltinStmt` node that starts with `CALL`
    (`ast::BuiltinStmt::after_call`), so `sema` reads it as the statement without `CALL`. Anything else is an
    error, as there (`CALL CLS()`, `CALL LOCATE 1, 2`, `CALL PSET(1, 2)`). Still marked after `CALL`: `SWAP`,
    `CLOSE` and the `MID$` statement, which have rules of their own.
  - Still marked or not checked by a run: `SOMEDAY.md`, "Left marked by `m3-sound`".
- **Sound handles** (`m3-sound-handles`; the facts in `study\00` §5, `verification\v39_*`): with them every name
  of libqb's audio part has a row.
  - **Ten functions and sixteen statements**, each a plain row (`SUPPORTED`, `STATEMENTS` in
    `sema\src\builtins.rs`) in every form the old compiler accepts: `_SNDOPEN`, `_SNDCOPY`, `_SNDLEN`,
    `_SNDGETPOS`, `_SNDPLAYING`, `_SNDPAUSED`, `_SNDRATE` and `_SNDOPENRAW` (bare only), `_SNDRAWLEN` (bare or with
    its handle), `_SNDNEW`; `_SNDCLOSE`, `_SNDPLAY`, `_SNDPLAYCOPY`, `_SNDPLAYFILE`, `_SNDPAUSE`, `_SNDSTOP`,
    `_SNDLOOP`, `_SNDVOL`, `_SNDBAL`, `_SNDSETPOS`, `_SNDLIMIT`, `_SNDRAW`, `_SNDRAWBATCH`, `_SNDRAWDONE`,
    `_MIDISOUNDBANK` and `_WAVE`. LONG handles and states, DOUBLE lengths and positions. Each needs
    `RuntimePart::Audio`, as `SOUND` does.
  - **The arrays of `_WAVE` and `_SNDRAWBATCH`** are `Slot::Block`, the bytes from an element to the array's end,
    as for graphics `PUT`; `check\builtins.rs` (`typed_array_arg`) also wants the old compiler's element type,
    `_BYTE` and `SINGLE`, and makes a variable, a member or another type its error where `block_arg` alone would
    mark some of them.
  - **`$MIDISOUNDFONT:`** (any value, any case) is the old compiler's error, which names `_MIDISOUNDBANK`
    (`check\mod.rs`, `meta`); without the colon it is an unknown metacommand, "not supported yet" as any other.
    A built-in statement's name used as an array or FUNCTION (`l = _SNDPLAY(h)`, `_LIMIT (60) = 60`) is an error
    when the name has a leading underscore and no function shares it (`check\places.rs`, `implicit_array`).
  - **`_MEMSOUND` is run** (off `NEVER_RUN`): on the block of a `_SNDNEW` sound and of one decoded from memory.
  - **Tests make no noise:** the slice programs `s127_sound_handles`, `s128_sound_new_memory` and
    `s129_sound_files_raw_midi` build their WAV and MIDI data in strings (and write the one WAV file they open),
    play only silence or at volume 0, and print only what is exact on any device: lengths and positions of whole
    half-seconds, `_SNDRATE` and `_SNDRAWLEN` against one-sided limits, handle numbers only before the first close.
    The front-end tests `sound_handles_ir`, `sound_handles_typed`, `sound_handles_cpp` (an array from an element)
    and `sound_handles_errors`; 131 call-site programs (`snd*`, `midisoundbank_*`, `wave_*`).
- **The console programs of the corpus that waited** (`m3-console-files`; the facts in `study\00` §5,
  `verification\v30_*`):
  - **`_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_SHL`, `_SHR`, `_ROL`, `_ROR`** are eight rows of `SUPPORTED`
    (`sema\src\builtins.rs`). `_IIF(condition, a, b)` is no call but an expression kind of its own,
    `ExprKind::Choice`, written as C++'s `?:`, so only the part chosen is evaluated; its condition is an
    integer (a float rounded), its parts both strings or both numbers, as they are. `_MIN`, `_MAX` and `_CLAMP`
    (`Rule::Widest`) take numbers as they are and are written with a C++ template argument
    (`std::min<int16_t>(…)`, `func_clamp<float>(…)`), which converts them. **The result of the four is typed by
    a rule that is not the operators'** (`check\ops.rs`, `widest`, on what the arguments are believed): a
    float wins over any integer (`_MAX(s!, q&&)` is a SINGLE), of two integers the wider, of two of one width
    the unsigned one; a unit test reads the 794 forms of `verification\v30_result_types.txt`. `_ROL` and `_ROR`
    (`Rule::Rotate`) turn two integers within the width of the first one's believed type (a literal by its
    size) and give the unsigned type of that width; `_SHL` and `_SHR` are plain calls of an `_UNSIGNED
    _INTEGER64` and a LONG. None of the eight may stand in a `CONST`, as in the old compiler. Slice program
    `s78_choice_min_shifts`, 75 call-site programs (`func_min_*`, `func_iif_*`, …).
  - **`EXIT SELECT` and `EXIT CASE`** (`StmtKind::ExitSelect`; `ir\src\lower.rs`): a jump. `EXIT SELECT` goes
    to the end of the `SELECT` whose `CASE` or `CASE ELSE` it stands in, also from an `IF` or a loop inside it
    (a loop around the `SELECT` goes on). `EXIT CASE` goes to the end of its `CASE` body: a plain `SELECT` is
    left, a `SELECT EVERYCASE` goes on with its next `CASE` and still counts as matched; in a `CASE ELSE` it is
    `EXIT SELECT`. With no `CASE` open either is the parser's error. Kept as the old compiler has it and
    listed for review (`SOMEDAY.md`): an `EXIT CASE` before a `SELECT` in the same `CASE` goes on after that
    inner `SELECT`. Slice program `s79_exit_select`.
  - **`PRINT USING`** to the screen and to a file is a statement kind of its own (`StmtKind::PrintUsing`,
    `Op::PrintUsing`; `sema\src\check\io.rs`, `codegen-cpp\src\io.rs`): the items before `USING` (printed
    first, as a `PRINT` with no line end prints them), the format (any string expression, evaluated once), the
    values, and whether the line ends. A value is a string, a number formatted as a SINGLE, DOUBLE, `_FLOAT`,
    `_UNSIGNED _INTEGER64` or `_INTEGER64` by what it is believed (the number stays as it is: libqb's entry
    converts it), or `TAB(n)`/`SPC(n)`. The C++ is the old compiler's, call for call: one libqb call per value
    with a jump out after each when an error is pending, and a block that prints what was built, without a
    line end after an error. **The statement is one unit:** the IR's rules for a pending error do not apply
    inside it. The parser reads `USING` first or after items (`PRINT "a"; USING "##"; 5`, `PRINT 1 USING …`);
    what a format prints is libqb's and is shown once per topic by `s80_print_using_numbers`,
    `s81_print_using_strings` and `s82_print_using_file`.
  - **`LPRINT`** is `PRINT` and `PRINT USING` with the printer as the destination (`PrintTo::Printer`): the
    same code with libqb's two printer entries, and the runtime part `RuntimePart::Printer` (make flag
    `DEP_PRINTER=y`). It is on `NEVER_RUN`: compiled, linked (below the `SYSTEM` of `s82_print_using_file`),
    compared by its call-site programs, never reached. It has no table row, so the coverage test of tier 1
    takes it by name (`OWN_NODE` in `driver\tests\inputs.rs`) and wants it as a statement in a call-site
    program (`WIDTH LPRINT` does not count).
  - **`GET` and `PUT` on a file** (`FOR RANDOM`, `FOR BINARY`) are the second row of each name
    (`StmtRule::File`, beside `StmtRule::Graphics`): the file number, the position or nothing, the variable or
    nothing, also after `CALL`. A statement that starts with a point stays the graphics form, as in the old
    compiler. The variable is `StmtArg::Bytes(place)`: a variable, element or member of any type but `_BIT`
    and a `TYPE` with a `STRING` in it; or a whole array `a()` (`StmtArg::Block` with no index) of anything
    but variable-length strings. It decides libqb's entry (`codegen-cpp\src\builtins.rs`, `file_get_put`):
    none, `field_get`/`field_put` (the record buffer); a variable-length string, `sub_get2` with the string
    and `sub_put2` with its characters and their number; anything else `sub_get`/`sub_put` with its address
    and size. A name alone is a scalar variable, also when an array has the name. What a file then holds, the
    positions and the errors are libqb's, shown by `s83_binary_file` and `s84_random_file`.
  - **`LOCK` and `UNLOCK`** are two plain rows: a file number and one record, a range with `TO`, or neither
    (`LOCK #1, TO 4` is an error, as there). Slice program `s85_file_locks`, in one process.
  - **`FIELD`, `LSET` and `RSET`** are a statement kind each (`StmtKind::Field`, `StmtKind::Lset`;
    `sema\src\check\io.rs`), written as libqb's calls: `field_new(file)` and one `field_add(string, width)` per
    part; `sub_lset(target, value)` or `sub_rset`. **A field variable is an ordinary variable-length string
    to the compiler**: the record buffer, the list of a file's field strings and a string's link to its field
    are libqb's, so nothing changes in assignment, `GET` or `CLOSE`. A target of `FIELD` is a variable-length
    string variable, element or member; of `LSET` and `RSET` a string place of either kind. The forms the old
    compiler rejects are errors (a fixed-length string in `FIELD`, a number or a `TYPE` value as a target,
    `CALL FIELD(…)`, `LSET (a) = …`). Still marked: a whole array `e()` as the string (`SOMEDAY.md`). Slice
    program `s86_field_lset` (28 lines).
- **What Civil War Strategy needs to build, apart from sound** (`m3-get-put-bload`; the facts in `study\00` §5,
  `verification\v28_*`; the game's report named only `SOUND`, `PLAY`, `BEEP` and `$EXEICON` after it):
  - **A line that is the name of a SUB and a colon** (`sema\src\check\flow.rs`): a call of the SUB when it
    takes no argument and no jump of the whole program names the word, a label in every body when one does;
    the name of a SUB with a parameter and of a FUNCTION is always a label. The words the jumps name are
    collected from every tree before the labels are entered (`collect_jump_words`: `GOTO`, `GOSUB`,
    `RETURN`, `RESTORE`, `RESUME`, `ON ERROR GOTO`, `ON n GOTO/GOSUB`, `ON <event> GOSUB`, compiled or not);
    where a line the parser could not read has the word, the line keeps a mark.
  - **Rows**: the functions `TAB` and `SPC` (strings: `PRINT`, `PRINT #` and `WRITE` take them as any
    string item), `PEEK`, `_RGBA32`, `_FULLSCREEN`, `DATE$`, `TIME$`, and `_RGB32` with a rule of its own
    (`Rule::GivenArgs`: one to four arguments, the call written with those given); the statements `DEF SEG`,
    `POKE`, `BLOAD`, `BSAVE`, `_FULLSCREEN`. `_FULLSCREEN` is a window built-in: compiled, compared by the
    call-site check, never run.
  - **`VARPTR` and `VARSEG`** are two expressions of the IR (`ExprKind::VarPtr`, `VarSeg`), typed
    `_INTEGER64`: the checked position of the element times its size, and the array's distance from the
    start of libqb's conventional memory in segments (the index of `VARSEG` is not evaluated, as in the old
    compiler). **An array named in one of them is in conventional memory** (`Var::conventional`, set
    wherever the use stands): a static one takes its elements from `cmem_static_pointer`, a dynamic one
    starts with flag 4, and every allocation and release of a dynamic array of numbers tests that flag
    (`new_block`, `free_block` in `codegen-cpp\src\decl.rs`), since the array may be a parameter. Only an
    array of numbers of the main module (`DIM`, `DIM SHARED`, `COMMON`, `REDIM`) may be named; every other
    kind of variable, `SADD` and `VARPTR$` are marked (`SOMEDAY.md`).
  - **Graphics `GET` and `PUT`** (`StmtRule::Graphics`: the entries of the name whose call is
    `sub_graphics_…`; the file form keeps a mark): every form of the two templates. The array is a slot of
    its own kind (`Slot::Block`, `StmtArg::Block`): the bytes of an array of numbers, whole or from an
    element, static or dynamic, of any body or a parameter, written as libqb's `byte_element` with a
    temporary declared with the variables of its body.
  Checked by the console slice programs `s70` to `s73`, the screen programs `d17`, `d18`, `g06` to `g11` and
  `m32` (five of them cut from the game with its pictures beside them, `tests\corpus\README.md`) and 147
  call-site programs.
- **Colours and images** (`m3-images-fonts-mouse` group 2; design D1 of the change holds the call of every form,
  the facts in `study\00` §5, `verification\v31_*`): each a plain row of `SUPPORTED` or `STATEMENTS`, in every
  form of its template the old compiler accepts.
  - **The colour functions** `_RGB`, `_RGBA` (an `_UNSIGNED LONG`, with an optional handle), `_RED`, `_GREEN`,
    `_BLUE`, `_ALPHA` (a LONG of a colour or an index, with an optional handle), `_RED32`, `_GREEN32`,
    `_BLUE32`, `_ALPHA32`, `_HSB32`, `_HSBA32` (DOUBLE slots), `_HUE32`, `_SATURATION32`, `_BRIGHTNESS32` (a
    DOUBLE).
  - **The image functions** `_LOADIMAGE`, `_COPYIMAGE`, `_WIDTH`, `_HEIGHT`, `_PIXELSIZE`, `_CLEARCOLOR`,
    `_BLEND`, `_PALETTECOLOR`, `_AUTODISPLAY` (each a LONG) and `PMAP` (a SINGLE), and **the statements**
    `_SAVEIMAGE`, `_PUTIMAGE`, `_MAPTRIANGLE`, `_CLEARCOLOR`, `_SETALPHA`, `_BLEND`, `_DONTBLEND`,
    `_PALETTECOLOR`, `_COPYPALETTE` and `_AUTODISPLAY`, also after `CALL`. `_NEWIMAGE` and `_FREEIMAGE` are run
    since then (off `NEVER_RUN`). `_MAPTRIANGLE`'s 3-D form (a z on each destination point, drawn into a
    hardware image) is compiled and compared by its call-site programs, and held below the `SYSTEM` of `i04`,
    but run by no test.
  - **A function's slot may be left empty** where its template has it alone in brackets after a comma
    (`_LOADIMAGE(f$, , r$)`, template `?[,[?][,?]]`; `check\builtins.rs`, `filled_slots`) and an argument
    follows; any other gap stays "not supported yet" (`INSTR(, a$, b$)` is not measured).
  - **Runtime parts**: `_LOADIMAGE` and `_SAVEIMAGE` need `RuntimePart::ImageCodec` (`DEP_IMAGE_CODEC=y`,
    the first flag); `RuntimePart::Font` (`DEP_FONT=y`, group 3) and `RuntimePart::ScreenImage`
    (`DEP_SCREENIMAGE=y`, group 5) serve the font functions and `_SCREENIMAGE`, in the old compiler's order
    (`driver\src\build.rs`, `make_flags`).
  Checked by the screen programs `i01` to `i08` (the images `i02` loads are written by
  `tools\screen\make_images.py`; `i03` saves and loads six formats), the front-end tests `images_typed`,
  `images_ir` and `images_errors`, and 169 call-site programs.
- **Fonts** (`m3-images-fonts-mouse` group 3; same design and facts): the functions `_LOADFONT`, `_FONT`,
  `_FONTWIDTH`, `_FONTHEIGHT`, `_PRINTWIDTH`, `_PRINTMODE`, `_UFONTHEIGHT`, `_ULINESPACING`, `_UPRINTWIDTH`,
  `_UCHARPOS`, `_MAPUNICODE` (each a LONG) and the statements `_FONT`, `_FREEFONT`, `_PRINTSTRING`, `_PRINTMODE`,
  `_UPRINTSTRING`, `_MAPUNICODE`, also after `CALL`, each a plain row. `_LOADFONT`, the `_U…` functions and
  `_UCHARPOS` need `RuntimePart::Font`. **`_UCHARPOS`'s array** is a function's `Slot::Block`: an array of LONG
  or `_UNSIGNED LONG`, whole (`p()`) or from an element (`p(2)`), passed as its bytes as a graphics `GET` passes
  its array (`ExprKind::Block`, `check\builtins.rs`, `long_array_arg`); anything else is the old compiler's
  "Expected LONG array-name"; an array used without `DIM` there (`nd&()`) stays "not supported yet", as for
  `GET`. Checked by the screen programs `f01` (the built-in fonts) and `f02` (Windows' `cour.ttf` and
  `arial.ttf`), the front-end tests `fonts_typed`, `fonts_ir`, `fonts_cpp` (the element forms, which the
  call-site check does not compare) and `fonts_errors`, and 81 call-site programs.
- **The mouse** (`m3-images-fonts-mouse` group 4; same design and facts): the functions `_MOUSEINPUT`,
  `_MOUSEBUTTON`, `_MOUSEHIDDEN`, `_MOUSEDISABLED` (each a LONG), `_MOUSEX`, `_MOUSEY`, `_MOUSEWHEEL`,
  `_MOUSEMOVEMENTX`, `_MOUSEMOVEMENTY` (each a DOUBLE) and the statement `_MOUSEMOVE`, each a plain row.
  `_MOUSEMOVE` moves the machine's cursor and is on `NEVER_RUN`. Checked by the screen programs `u01` (the queue,
  buttons, wheel and movement in `SCREEN 12`) and `u02` (columns and rows in `SCREEN 0`), fed by the mouse steps
  of their keys files (`tools\screen\README.md`), the front-end tests `mouse_typed`, `mouse_ir` and
  `mouse_errors`, and 24 call-site programs.
- **Images no test runs** (`m3-images-fonts-mouse` group 5; same design and facts): the functions
  `_SCREENIMAGE` (bare or with four LONG slots, all or none: the only function template whose optional group
  holds several slots, so a count between is "takes 0 or 4 arguments", `check\builtins.rs`, `template_counts`)
  and `_CLIPBOARDIMAGE` (each a LONG, `RuntimePart::ScreenImage`), and the statements `_CLIPBOARDIMAGE = h`
  (the same part), `_PRINTIMAGE` (`RuntimePart::Printer`) and `_DEPTHBUFFER` with its four words, each a plain
  row on `NEVER_RUN`: they read the desktop or the clipboard, print a page, or act on hardware images. They and
  the 3-D form of `_MAPTRIANGLE` stand below the `SYSTEM` of `i04`. Checked by the front-end tests
  `screenimage_typed`, `screenimage_ir` and `screenimage_errors`, and 19 call-site programs.
- **Memory blocks** (`m3-mem`; the facts in `study\00` §5, `verification\v32_*`; the old compiler's C++ of every
  form in the change's `design.md`):
  - **The type `_MEM`** (group 2, design D1) is a user type defined by `sema` (`sema::mem_type`, 56 bytes: the
    members `OFFSET`, `SIZE`, `TYPE`, `ELEMENTSIZE`, `IMAGE`, `SOUND` and two lock members a program cannot
    name), added to the program's types when the program first names it (so the dumps of a program without it
    do not change). It is everything a user type is: a variable of any storage class, an array, a member, a
    parameter, a whole copy, `SWAP`, `LEN` (56). A store to a member is an error, as in the old compiler.
  - **Built-ins that give a block** (`MEM_RESULTS`: `_MEMNEW`, `_MEMIMAGE`, `_MEMSOUND`, and in group 3 `_MEM`
    and `_MEMELEMENT`) are rows whose call is typed `_MEM`; such a call stands only as the value of an `Assign`
    into a `_MEM` place (`check\mod.rs`, `assign`, the checker's `mem_store`), written as the old compiler
    writes it: a `mem_block funcN` temporary in `global.txt` and a `memcpy` of its 56 bytes (`store_block`,
    `codegen-cpp\src\stmt.rs`). Anywhere else it is an error, also stored into a number (whose C++ the old
    compiler fails to build).
  - **A block as an argument** is `Slot::Mem` (`_MEMEXISTS`; `_MEMFREE` and both blocks of `_MEMCOPY a, o, n TO
    b, p`): a variable, element or member of type `_MEM`, passed as its address (`mem_ref`,
    `codegen-cpp\src\place.rs`); an offset or a size is `Slot::Offset`, a store into an `_OFFSET`.
    `_MEMSOUND` (libqb's audio part) is run since `m3-sound-handles` (`s128_sound_new_memory`). Checked by the slice program `s88_mem_new_image`, the
    front-end tests `mem_type_typed`, `mem_blocks_typed`, `mem_blocks_ir` and `mem_errors`, and 26 call-site
    programs (`mem*`).
  - **The block of a reference** (group 3, design D3): `_MEM(reference)` and `_MEMELEMENT(reference)` are
    `ExprKind::MemOf`, `_MEM(offset, size)` `ExprKind::MemAt` (`func__mem_at_offset`; the offset an `_OFFSET` or
    `_UNSIGNED _OFFSET`, the size stored as an `_OFFSET`), `_OFFSET(reference)` `ExprKind::AddressOf`, an
    `_UNSIGNED _OFFSET` (all three `Rule::Own`: checked in `sema\src\check\mem.rs` before the table). A
    reference (`sema::Reference`) is a variable, element or member, or a whole array: `a()`, a member array `t.v()`
    or `t.v`. The emitter (`codegen-cpp\src\mem.rs`) writes the five arguments of `func__mem` by the kind of
    place, as the old compiler's helpers do: a type value (`Type2MemTypeValue`), the bytes from an element to
    the array's end, the user-type rule for anything reached through a member, the array's own lock for an
    array of numbers or fixed-length strings and `sf_mem_lock` for the rest; an index is written twice, as
    there. An array parameter takes its own lock slot also where nothing above gave it its dimensions
    (`DIVERGENCES.md` D-016). Errors as the old compiler's: a variable-length string or a `_BIT` (also
    `_OFFSET` of a `_BIT` array), a value, `_MEMELEMENT` of a whole member array, an offset that is no `_OFFSET`.
    Not supported yet: a `TYPE` that holds a `STRING`, a member array that is not the last step of the place or
    whose elements are of a user type, `_OFFSET` of an element, member or array of variable-length strings, a
    name with `()` that names no array. Checked by `s89_mem_references` (the kinds of `verification\v32_a_blocks`
    and `_OFFSET` against `OFFSET`), the front-end tests `mem_refs_typed`, `mem_refs_ir`, `mem_refs_cpp` (the
    element and member forms, which the call-site check does not compare) and `mem_errors`, and 43 call-site
    programs (`mem_*`, `memelement_*`, `offset_*`).
  - **Reading and writing through a block** (group 4, design D4): the `_MEMGET` function is
    `ExprKind::MemRead` (`func__memget` read as the type: a number, `STRING * n` as a `STRING` value, a user type
    only stored into a place of it, `store_block`); the statements `_MEMGET`, `_MEMPUT` and `_MEMFILL` are
    `StmtKind::MemGet`, `MemPut`, `MemFill` (`Op::…` in the IR), checked in `sema\src\check\mem.rs` (the parser's
    `MemStmt` for `_MEMPUT` and `_MEMFILL`, a call statement for `_MEMGET`) and written inline as the old compiler
    writes them (`codegen-cpp\src\mem.rs`): the offset into `tmp_long`, the checks of the block (errors 309,
    308, 300) and the copy, through an `int8` … `int64` for a size of 1, 2, 4 or 8 and `memmove` otherwise; a
    value `AS type` stored through a pointer of the type, with the conversions C++ makes on a store left to it
    (`stored`); a string that is no variable through `g_tmp_str`; `_MEMFILL` the runtime's `sub__memfill_N` or
    `sub__memfill`. What is read or written is a `sema::MemValue`: a typed value, the bytes of a reference, a
    string. A type argument may be `STRING * n` or `_BIT * n` (`syntax\src\parser\expr.rs`, `type_arg`); a user
    type's name there is read from the expression. Every runtime error 300 to 313 is a slice program of its
    own (`s91_mem_error_300` … `s104_mem_error_313`): none is trapped. Checked by `s90_mem_get_put`, those 14,
    the front-end tests `mem_rw_typed`, `mem_rw_ir` and `mem_errors`, the parser test `mem_forms`, and 69
    call-site programs (`memget_*`, `memput_*`, `memfill_*`).
  - **A number and its bytes** (group 5, design D5): `MKI$`, `MKL$`, `MKS$`, `MKD$` and `_MK$` are
    `Rule::Mk`, `CVI`, `CVL`, `CVS`, `CVD` and `_CV` `Rule::Cv` (`sema\src\check\mem.rs`, `number_bytes`), calls of
    libqb's `<t>2string` and `string2<t>` (`bit2string(1,x)` for `_BIT`). The number of an `MK` is converted to an
    integer type as a store converts it (to 16 bits or fewer through `qbr_float_to_long` of the value as it is)
    and passed to a float type as it is. `_MK$` and `_CV` take a type first (any numeric type, `_BIT` and
    `_UNSIGNED _BIT` one bit wide, a width being the old compiler's error), kept in their first slot as a 0 of
    that type; `_CV` of a `_BIT` is an `_INTEGER64`. Checked by `s105_mk_cv` (the program of
    `verification\v32_c_mk_cv`), the front-end tests `mk_cv_typed` and `mk_cv_errors`, and 55 call-site programs
    (`mki_*` … `cv_*`).
- **The constants of QB64pe's auto-included file** (`m2-console-screenhide` group 5; `sema\src\check\auto.rs`):
  `_TRUE`, `_FALSE`, `_ONE_MB`, the limits of the numeric types and the others of `beforefirstline.bi` are
  known in every program. The file is not included: `tools\builtins\builtins.json` carries each constant's
  suffix and the text of its expression (the branches of a Windows 64-bit build), and a constant becomes a
  constant of the main module the first time a program names it, computed by the `CONST` evaluator's
  arithmetic. 231 of the 310 are computed; the others are "not supported yet" where they are used, with the
  reason: 69 need `CHR$` and `_E` needs `EXP` in a `CONST`, and nine are numbers the evaluator does not read
  yet (`_DOUBLE_MIN`/`_MAX`, the three `_FLOAT_` ones, `_INTEGER64_MIN`, `_UINTEGER64_MAX`, `_OFFSET_MIN`,
  `_UOFFSET_MAX`). Their names are reserved like any name with a leading underscore. The three functions of
  the file included after the last line and the `$COLOR` constants keep the mark of an auto-included name.
- **Precompiler flags** (`m2-console-screenhide` group 6; `syntax\src\pp.rs`, `program.rs`): `_CONSOLE_`,
  `_EXPLICIT_`, `_EXPLICITARRAY_`, `_ASSERTS_`, `_DEBUG_` and `_SOCKETS_` in `$IF` have the values QB64pe gives
  them from the whole program. `qb64rust_syntax::parse_settled` is the parser's entry point: it parses in
  rounds as QB64pe compiles in rounds, a feature met in one round (`$CONSOLE`, `OPTION _EXPLICIT`, ...) being a
  flag's value in the next, until nothing changes; a program with no `$IF` on a flag is parsed once. The
  settled flags are `ParsedProgram::flags`: `sema` takes the console and the two options from them as well as
  from the lines it walks, because a line may set its flag from inside the `$IF` branch the flag switches off.
  Still marked: a `$IF` that names `_CONSOLE_` between `$CONSOLE:ONLY` and a later `$CONSOLE` or the other way
  round (QB64pe's two passes read it with different values). `_SOCKETS_` follows `$DEBUG` (below).
- **`$ASSERTS`, `_ASSERT` and `$DEBUG`** (`m3-asserts-debug-zlib` group 2; the facts in `study\00` §5,
  `verification\v33_*`): `$ASSERTS`, `$ASSERTS:CONSOLE` and `$DEBUG` are settings of the whole program checked in
  `sema`'s `meta` with the line rules of `$CONSOLE` (`console_meta`). `$ASSERTS` sets `Settings::asserts` (also
  from the settled flag), written as the global `int32 asserts=1;`; `$ASSERTS:CONSOLE` also gives a console.
  `_ASSERT condition[, message$]` is a plain statement row (`sub__assert`), its condition `Slot::Integer` (a float
  through `qbr_float_to_long`, an integer as it is); libqb checks nothing while `asserts` is 0, and raises 314 or,
  with a console and a message, writes the message and raises 315, neither trapped by `ON ERROR`. **`$DEBUG`
  writes no debugger code** (user, design D2; `DIVERGENCES.md`): it sets `_DEBUG_` and `_SOCKETS_` (the old
  compiler's vWatch uses sockets) and, with `-w`, warns "$DEBUG features only work from the IDE"; `vwatch` stays
  0. Checked by `s106_asserts`, `s107_assert_message`, `s108_asserts_off_debug`, the front-end tests
  `asserts_errors`, `asserts_forms`, `asserts_cpp`, `flags_asserts_debug` and `console_meta_tab_after`, the
  command-line test `debug_warning`, and 18 call-site programs (`assert_*`).
- **Checksums, compression and base64** (`m3-asserts-debug-zlib` group 3; the facts in `study\00` §5,
  `verification\v33_b*`): `_ADLER32`, `_CRC32` (each an `_UNSIGNED LONG`), `_DEFLATE$(s[, level])`,
  `_INFLATE$(s[, size])`, `_BASE64ENCODE$` and `_BASE64DECODE$`, each a plain row of `RuntimePart::DataProcessing`
  (the table's `DEPENDENCY_ZLIB`), which the driver builds with `DEP_ZLIB=y` after `DEP_FONT=y` and before
  `DEP_CONSOLE=y`, as the old compiler orders it (`build.rs`, `make_flags`; a call in code never run links it
  too). Checked by `s109_data_processing` (also run by the upstream `console_only/libz`, `image/pcx_test` and
  `image/webp_test`), the front-end tests `data_processing_typed` and `data_processing_errors`, a unit test of
  `make_flags`, and 28 call-site programs (`adler32_*`, `crc32_*`, `deflate_*`, `inflate_*`, `base64*`). A name
  written without its `$` (`_DEFLATE("x")`) is "not supported yet", as for every such name (`_TRIM(...)`).
- **`END n` and `SYSTEM n`** (`m3-asserts-debug-zlib` group 4; `verification\v33_c*`): `StmtKind::End` and
  `System` (and `Op::End`, `Op::System`) carry the exit code: a float converted through `qbr_float_to_long`, an
  integer as it is (`check\mod.rs`, `end_or_system`). The emitter writes the old compiler's two lines before the
  end, the statement's event check and `exit_code=<n>;` (`codegen-cpp\src\stmt.rs`, `exit_code`); the process's
  exit code is the low 32 bits. The corpus runner compares a program's exit code with its `<name>.exitcode`
  (`tools\legacy_tests\README.md`). Checked by `s110_system_code`, `s111_end_code`, the front-end tests
  `exit_code_ir`, `exit_code_cpp` and `exit_code_errors`, and the upstream
  `arrays/t_qb64pe_refactor_preserve_inline_varstrings`.
- **`$UNSTABLE` and the member markers** (`m3-typefields` group 2; the facts in `study\00` §5, `verification\v35_*`):
  `$UNSTABLE:TYPEFIELDS` is read by pass 1 in the walk that keeps `OPTION BASE` at each `TYPE` block
  (`check\mod.rs`, `collect_block_settings`, `type_flags`), so it reaches the blocks below it in the branches
  taken; `MIDI` and `HTTP` give the old compiler's warning with `-w`; other forms are errors (`unstable_meta`).
  The parser reads each `_DYNAMIC`/`_STATIC` as a `FieldMarker` node in every place the old compiler takes one
  (`syntax\src\parser\blocks.rs`, `type_field`); `sema` counts them (`places.rs`, `member_markers`: the flag, not
  before an `AS type` list, one marker, arrays only) and sets `Member::dynamic`. `_STATIC` compiles as an
  unmarked member. Checked by `unstable_forms`, `unstable_forms_ok`, `typefields_markers`, the CLI test
  `unstable_stable_flag_warning` and 18 upstream programs (`t028`–`t044`, `t233`, `t234`,
  `t_qb64pe_refactor_inline_reset_*`).
- **`_DYNAMIC` member arrays** (`m3-typefields` group 3; design D3–D6, D9, D10): a type that owns one (`owns_desc`,
  in itself or in a member of a user type) has the old compiler's aligned layout (`decl.rs`, `layout`: such a
  member and a member of an owner type at a multiple of 8, the size rounded up to 8); other types keep the packed
  one. The member is a pointer to a descriptor of a dynamic array's shape, its number of dimensions in slot 3,
  with its own `_MEM` lock. Its first bounds may name a `CONST` above the block, read in pass 2 (`places.rs`,
  `type_const_bounds`). `REDIM` of it is an `Allocate` of an `ArrayRef::Member` (any number of dimensions, the
  modes of an array; the first dimension `lower TO upper` unless the type is written with `AS` or a suffix),
  `ERASE` a `Release`; an element has as many indexes as written, checked when it runs. The emitter
  (`codegen-cpp\src\owner.rs`) writes into `global.txt` one function per owner type for making, releasing,
  zeroing and copying a value (`qbr_udtN_new`, `_free`, `_zero`, `_clone`) and per dynamic member its descriptor
  functions (`_blank`, `_make`, `_release`, `_free`, `_copy`, `_guard`, `_bound`, `_desc`); a variable, a `STATIC`
  local, a local at each return, the elements of an array (`alloc_array`, `allocate_with`, `release`) are made
  and released through them, `ERASE` of a static array of them zeroes them (dynamic members released, null). An
  element goes through the guard (error 9 when not allocated or of another number of dimensions; a dummy element
  then), a store through it stores nothing after an index outside on the way (D-004 widened, `stmt.rs`,
  `member_path`); `LBOUND`/`UBOUND` find the place; the member as an array argument is its descriptor. A
  whole-value copy (group 4, design D7, `owner.rs` `copy_owner`) is the old compiler's sequence of statements,
  each finding both places anew: a test that they differ, then for a type that holds no `STRING` the other parts
  and then each member through `_copy` (a source not allocated keeps the target's), for one that holds a `STRING`
  each part in order, a dynamic member erased and then set through `_set` (a source not allocated leaves the
  target not allocated). Arrays of an owner type keep, move and release their elements' members with `REDIM`,
  `_PRESERVE`, `_RETAIN`, `ERASE` and `SWAP`. **Files** (group 5, design D8): `PUT` and `GET` of a whole array of
  an owner type go through a buffer in the type's declared layout (`owner.rs` `owner_file`: `qbr_udtN_pack`,
  `_unpack`, `_file`, `_commit`, written for the types a file statement names); a dynamic member with other
  bounds raises 9 (`PUT` writes nothing; `GET` keeps that member, D-019); after a read that raised an error
  nothing is unpacked (D-021). `_MEM` and `_MEMELEMENT` of a whole dynamic member and of an element of one take
  its descriptor's block and lock (`mem.rs` `dyn_block`, `qbr_dyn_mem_all`, `qbr_dyn_mem_one`; one that is not
  allocated raises 9, D-021); `ERASE` of the member makes such a block invalid. `LEN`, `_OFFSET`, `COMMON` and
  `PUT`/`GET` of one owner value are the old compiler's errors. Still marked (`SOMEDAY.md`, "Left marked by
  `m3-typefields`"): `_OFFSET` of a whole dynamic member, `_MEMGET`/`_MEMPUT`/`_MEMFILL` with one (whole-array
  assignment came with `m3-whole-array-assign`, below). Checked by `typefields_cpp`, `typefields_ir`, `typefields_rejected`, the IR unit test
  `dynamic_member_arrays`, `s114_dyn_members`, `s115_dyn_member_procs`, `s116_dyn_member_parents`,
  `s117_dyn_member_kinds`, `s118_dyn_member_files`, the CLI tests of D-019 to D-021 and 81 upstream programs.
- **Targets through a member** (`m3-typefields` task 4.2; `verification\v35_x62`): a member of an element and an
  element of a member array (fixed or dynamic) are targets of `READ`, `INPUT`, `LINE INPUT` (console and file) and
  the `MID$` statement, stored by the rule of their place (`stmt.rs` `store`, a number before the indexes and not
  stored after one outside, D-004; a string by reference after them).
- **Whole-array assignment** (`m3-whole-array-assign`; the facts in `study\00` §5, `verification\v36_*`):
  `target() = source()`, each side an array variable or a member array (`x.v()`, `e(i).v()`, a `_DYNAMIC` one),
  the source also in parentheses, is `StmtKind::CopyArray` and `Op::CopyArray`. `sema` (`check\whole.rs`, design
  D1, D2) takes the path when both sides are written whole, resolves the target first, and gives the old
  compiler's errors for other element types, `STRING * n` lengths and numbers of dimensions (an array
  parameter's number from its uses, `DimFact`), and, by its guard `CheckAsgRHS`, for a whole array `name()` on
  one side only; a type suffix on both names is marked (the old compiler copies element 0). The emitter
  (`arrays.rs` `copy_array`, design D4) writes the old compiler's `AppendWholeArrayAssign` block: the
  descriptors target first (a member array's made as for an array argument, a dynamic member's own), error 9 when
  either is not allocated or the bounds differ, nothing copied then or after an index outside on the way
  (D-004 widened); one `memmove` for numbers, `_BIT`, `STRING * n`, `_MEM` and user types of bytes, `qbs_set`
  per `STRING`, part by part for a user type with strings, `copy_owner` per element of an owner type, in place
  (D-022: other bounds copy nothing). Checked by `whole_array_ir`, `whole_array_errors`, `whole_array_cpp`, the
  IR unit test `copy_between_arrays_of_another_type_or_shape`, `s119_whole_array`, `s120_whole_array_members`,
  the CLI tests `whole_array_to_a_member_through_an_index_outside` (D-004) and
  `whole_array_of_an_owner_type_with_other_bounds` (D-022), and 20 upstream programs.
- **`DECLARE LIBRARY` in `sema`** (`m3-declare-library` group 2; the facts in `study\00` §5, `verification\v37_*`):
  pass 1 (`check\library.rs`) enters each block, wherever it stands (a SUB, an included file), as a `Library`
  (its form and names as written, `Program::libraries`) and each declaration as a `Proc` with no statements and an
  `External` part (its library, its C name: the `ALIAS` text or the name as written, and which parameters are
  `BYVAL`), after the program's own procedures and with the default types at the block. Its parameters are
  read as a program procedure's, apart from a library's rejections (an array, `STRING * n`, a `STRING` with
  `BYVAL`, a `TYPE` defined below the block) and two allowances (a built-in's name, a `_BIT`, whose use is the
  error). A call goes through the shared checks; a `BYVAL` argument is `Arg::Value` (converted as for an
  assignment) or `Arg::Null` (written `0`, `(0)`, `0&` or as a `CONST` 0: the old compiler's C++ `NULL`, decided
  from the argument as written, since folding makes `1 - 1` a 0 too). The parser rejects a name without quotes,
  two names without a comma and `STATIC` after a declaration; `sema` the other forms, each with the old
  compiler's reason (`library_errors`); `library_typed` holds the accepted forms. A library FUNCTION beside a
  program SUB of the name (`v37_x22`) is marked.
- **`DECLARE LIBRARY` in the IR and the C++** (`m3-declare-library` group 3, design D3): the IR carries
  `Program::libraries` and each library procedure's `External` (dump: `Library` lines and `external … of
  library N`); validation checks that a `BYVAL` parameter takes `Arg::Value` or `Arg::Null` and nothing else does.
  `codegen-cpp\src\library.rs` takes a `Resolved` (what the driver found: per block its headers, files and DLL name,
  per declaration without a header its C or C++ linkage) as an input of `emit`, and writes into `regsf.txt`, before
  the program's prototypes: each block's headers, once per name that finds one, as the old compiler includes
  them, a prototype per declaration of a named block
  without a header (`extern "C"` for C linkage; none for a block with no name, the C runtime), a `DYNAMIC` block's
  `HINSTANCE` (named as the old compiler names it) and `DLLCALL_`
  pointers, a `CUSTOMTYPE` block's `CUSTOMCALL_` pointers; into `maindata.txt`, first, the `LoadLibrary` and
  `GetProcAddress` lines with errors 259 and 260 and the `CUSTOMTYPE` pointers; into `global.txt` the declaration
  of a function a `CUSTOMTYPE` block without a header points to (`extern "C"` for a C name, `nm`'s demangled text,
  carried by `Linkage::Cpp`, for a C++ one). A call:
  the C name (or the pointer `FUNC_X`/`SUB_X`), a result cast to its type or a `STRING` one through
  `qbs_new_txt`, a string argument as `(char*)(s)->chr`, a `BYVAL` value with its widening and truncating
  conversions left to C (a float to an integer keeps `qbr`). A library procedure has no fragments of its own:
  procedure K is the K-th program procedure. Tests: `library_ir` (`ir`), `library_cpp` (`cpp`, with a fixed
  `Resolved` in `frontend.rs`: `./name` a library file, `cpp` in it C++ linkage, any other name a header).
- **`DECLARE LIBRARY` in the driver** (`m3-declare-library` group 4, design D2, D4; `driver\src\library.rs`):
  after `sema`, for a build, `-z` and the `cpp` dump, each name of a block is looked for as the old compiler
  does on Windows, with the reference clone as its folder (so a program with a named block needs the clone, also
  for `-z`): a `.lib`, `.a`, `.o` by the written folder, then by the name alone, then a header `.h`, `.hpp`, also
  from the folder of the file the block stands in; `DYNAMIC` a `.dll`, also in System32 and beside the source;
  `./` is the source's folder. "LIBRARY not found" and "DYNAMIC LIBRARY not found" at the name. For a block
  without a header the clone's `nm` gives each declaration's linkage, from the block's last library only, as the
  old compiler's ("Could not find sub/function …", "Unable to resolve multiple instances …" at the block).
  Where the old compiler's build would fail the program is rejected first: a header found a second time that has
  no guard (`#pragma once`, or an `#ifndef X` whose `X` is then defined) and defines something (a `{` that opens
  no `extern "C"` and no namespace; one that only declares builds, `v37_x68`), a second `DYNAMIC` block whose
  handle has an earlier one's name, a block without a header whose last name finds no library file (`v37_x69`),
  and a library file whose path holds a `$` (`DECISIONS.md` 2026-10-16). The library files go on the link command after `ExtraLinkerFlags`, and their bytes
  and the headers' into the build cache's key; **the files a header itself includes are not in the key**, so a
  change to such a file alone needs a build without the cache. The demand report runs no lookup, and **the LSP
  reports no lookup error** (its crate does not depend on the driver). Tests: the lookup's, `nm`'s and the key's
  unit tests; CLI tests (`--ignored`) that build `verification\v37_lib`'s libraries into a temporary folder and
  link, call and reject them; the slice programs `s121`–`s125` (group 5: a header in each `<name>.files`, the C
  runtime, `kernel32`) and the 8 upstream programs with a block (`pass.list`, run from the clone: two need its
  `lib-windows.a` and `lib.dll`).
