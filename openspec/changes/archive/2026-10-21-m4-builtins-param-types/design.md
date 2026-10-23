# Design

## Context

For the motivation, scope and the findings before this proposal, see `proposal.md`. For the behaviour, see the
delta specs. Where the parts stand in this compiler (2026-10-21):

- **Built-ins.** A built-in function is compiled when it has a row in `sema\src\builtins.rs` `SUPPORTED`, and a
  statement when it has one in `STATEMENTS`. Every other name of the table is "not supported yet". A row's
  `dependency` chooses the libqb part (`RuntimePart::of_dependency`, `check\builtins.rs` `runtime_part`), so
  `_MD5$` links the font part with no new code. The coverage test (`crates\driver\tests\inputs.rs`) wants each
  supported name in a slice program and in a `typed` front-end test, and each name of `NEVER_RUN` in a call-site
  program. `DECISIONS.md` 2026-10-10 asks for a call-site program for every new plain built-in.
- **A type as an argument.** The parser reads `_CAST(_UNSIGNED _BYTE, x)` as a call whose `ArgList` has a
  `TypeArg`, as for `_MK$` and `_CV` (`check\mem.rs` `number_bytes`, `bytes_type`).
- **Conversions.** `ExprKind::Conv` with a `ConvKind` (`Widen`, `Truncate`, `RoundEven`, `Nearest`) is how
  `sema` hands a conversion to the IR. A float that goes to an integer is rounded half to even. `_CAST` truncates
  toward zero instead (`_CAST(INTEGER, 6.283)` is 6, upstream `cast_test`).
- **`CLEAR`.** `codegen-cpp` already writes `clear.txt` for every program (`lib.rs`, `decl.rs` `clear`),
  mirroring `qb64pe.bas` 12734–12760. libqb's `sub_clear` (`qbx.cpp` 510) includes it and resets `DATA`, the
  files, the images, `RETURN` and the error handler.
- **Parameter types.** Pass 1 (`check\mod.rs` `check`) declares every `TYPE` block of the program before any
  procedure header (`m4-names-arrays` D4), so `declare_params` (`check\proc.rs`) finds a type defined anywhere.
  `DECLARE LIBRARY` already holds its parameters to the types above its block (`check\library.rs`,
  `walk_libraries`, `types_before`).

## Goals / Non-Goals

**Goals:**

- The six upstream programs pass, and the six marks leave the upstream demand report.
- `v48_x12` and `x25` pass: the old compiler's error for a parameter typed below its procedure.
- Each new built-in has what the coverage test wants: a slice program, a `typed` test and a call-site program.

**Non-Goals:**

- `_CAST` in a `CONST`: the old compiler rejects it (`v49_x11`), so it is an error here too and nothing more.
- `_OS` and `_FILES` without their `$` and the other forms the table does not have: "not supported yet", as for every built-in
  named without its required suffix.
- `RUN`, `CHAIN`'s `COMMON` and anything else `clear.txt` serves: not this change.

## Decisions

### D1. `_OS$`, `_MD5$`, `_CLIPBOARD$` and `_FILES$` are plain rows

Each gets a `Rule::Plain` row in `SUPPORTED` (`builtins.rs`, after `_BASE64DECODE$` for `_MD5$`, with the file
functions for `_FILES$`, a new comment block for `_OS$` and `_CLIPBOARD$`), and `_CLIPBOARD$` a
`StmtRule::Plain` row in `STATEMENTS` for its `=?` form, as `_CLIPBOARDIMAGE = h` has. `_FILES$`'s optional slot
is the `[?]` form that `Rule::Plain` already writes with its `passed` flag (`_WINDOWHANDLE[(kind)]`). If `v49`
shows a form the plain path writes otherwise than the old compiler, that form gets its own rule here. The wrong
forms of `v49_x40`–`x50` are the table's arity and argument-kind errors. `_OS$ = "a"` is the old compiler's
"Expected variable =" (`x48`), and `CALL _CLIPBOARD$("abc")` its "Syntax error" (`x50`). If the plain path
accepts one of these, it gets an error in `check\builtins.rs`.

### D2. `_CAST` is a conversion of its own

`Rule::Cast` (`builtins.rs`) sends the call to `check\builtins.rs`, which reads the `TypeArg` with the type
reader `_MK$` uses (`bytes_type`, `check\mem.rs`) and accepts the numeric types of `qb64pe.bas` 20398:
`_BYTE` … `_INTEGER64`, signed or `_UNSIGNED`, `_OFFSET`, `SINGLE`, `DOUBLE` and `_FLOAT`. `_BIT`, `STRING`,
`STRING * n` and a user type are the error "_CAST TYPE unsupported". The value is checked as a number ("Expected
numeric argument" for a string or a whole user-type value) and wrapped in `ExprKind::Conv` with a new
`ConvKind::Cast`. The result is held and believed as the type named.

`ConvKind::Cast` means the C++ cast `((ctype)(value))`, which is what the old compiler writes. The IR validator
accepts it from any number to any number. `codegen-cpp` writes it as the old compiler does, with the C++ type of
`typ2ctyp`. Constant folding (`sema`'s evaluator) leaves `Cast` alone, so the cast is always the C++ one and both
compilers give the same value with the same g++ for every value in range, and for a float variable out of range
(`v49_a`: the x86 instruction's value, `-2147483648` for `LONG`).

A float *constant* out of the integer's range is undefined in C++, and g++ folds it to whatever its optimiser
leaves: `v49_x18` printed `5  1008726880  1008726880` on one recording and `6 -360710944  3934256352` on the
next, from the same source. No value can be matched, so `x18` goes on `tests\verification\known.list`. We write the
same cast as the old compiler and diverge in nothing, so `DIVERGENCES.md` gets no row. It is a row of
`SOMEDAY.md`'s "QB64pe behaviours to review" (a `_CAST` whose value is undefined).

Errors (`v49_x01`–`x16`): `STRING`, `_BIT`, `_UNSIGNED _BIT` are "_CAST TYPE unsupported"; any other name that is
no numeric type (`STRING * 4`, a user type, `_MEM`, an unknown name, a number) is "Cannot convert type (…) to
symbol", which `bytes_type` already gives for `_MK$` (`v32_x113`); a string or a whole user-type value is "Expected
numeric argument"; one argument, three, or none is "Incorrect number of arguments". `_CAST` in a `CONST` is an
error in the old compiler ("Unexpected element"), so the constant evaluator's error for an unknown function serves.

*Alternative:* `RoundEven` followed by `Truncate`. Rejected: 6.283 would round to 6 but 6.5 to 6 and 7.5 to 8,
where the C++ cast gives 6 and 7.

### D3. `_CLIPBOARD$` is run by the tests

The slice program and `clipboard_test` set the machine's clipboard. The old compiler's recorded runs of the
upstream suite already do (`baselines\`), and nothing else in the suites reads it. A run while the user copies
text could fail, which is the cost. `_CLIPBOARDIMAGE` stays on `NEVER_RUN`: it reads the desktop's image, which a
test cannot set. *Alternative:* `NEVER_RUN` with a call-site program only. Rejected: `clipboard_test` would then
stay off `pass.list`, and the statement and the function would never be run against each other.

### D4. `CLEAR` is a plain statement with two checks

`CLEAR` gets a `StmtRule::Plain` row; the plain path writes `sub_clear(a, b, c, passed)` from its special format.
Before it, the statement check (`check\builtins.rs`, the statement path) makes the two errors of `qb64pe.bas`
11318: in a SUB or FUNCTION, "CLEAR cannot be used inside a SUB/FUNCTION", and an opening or closing parenthesis
anywhere among its tokens, "Incorrect number of arguments". That includes one inside an argument
(`CLEAR , , LEN("a")`, `v49_x33`). A fourth argument ("Syntax error", `x34`) and a string ("1st sub argument
requires a number", `x35`) are the special format's errors, which the plain path already gives for other
statements. `CALL CLEAR` is accepted and clears (`x36`). `clear.txt` is not changed unless `v49_e` shows a variable
it resets otherwise than the old compiler. `v49_e` measures the `STATIC` variables of procedures, `DIM SHARED`,
`COMMON SHARED` and implicit variables, fixed-length strings and user types. `95_type_codegen_clear_inline_owner`
measures the member arrays of owner types. After `CLEAR` in a `GOSUB`, `RETURN` is error 3 (`v49_e2`). After
`CLEAR` in an error handler, `RESUME NEXT` is the runtime error "RESUME without error" (`x38`).

### D5. A parameter's type must stand above the procedure

`collect_block_settings` (`check\mod.rs`), which already walks every block in file order into included files, also
records, at each `ProcDef` it meets, the upper-case names of the `TYPE` blocks met so far
(`Checker::types_above: HashMap<NodeKey, HashSet<String>>`). The header is reached before the procedure's own body.
`declare_params` (`check\proc.rs`) makes the error "Illegal SUB/FUNCTION parameter" at a parameter, scalar or
array, whose user type is not in its procedure's set. Measured: a `TYPE` in the procedure's own body is below its
header (`v49_x60`), a `TYPE` in a later FUNCTION too (`x65`), a FUNCTION's parameter and an array parameter
likewise (`x61`, `x62`), the error stands for a SUB never called (`x63`), and a `TYPE` in an `IF` block above and
one in an included file above are above (`x66`, `v49_f`). The message is "the `TYPE late` of a parameter must be
defined above the SUB or FUNCTION", like the library's. `check\library.rs` keeps its own walk: its blocks are not
`ProcDef`s, and merging the two walks is not needed for this. `DECLARE SUB` lines change nothing: the old
compiler ignores them, and `v49_f` checks one above the `TYPE` block.

### D6. The measurement family `v49`

Recorded with `qb64pe.exe` in task 1.1, before the code:

- `v49_a_cast`: every type named, of a negative and a positive float, an integer out of range, an unsigned value,
  a `_FLOAT`, a `_BIT` variable; what `PRINT` shows of each result (its believed type).
- `v49_b_files`: `_FILES$` of a folder made by the program, of a pattern, of a name with no match, a bare call
  before any call with an argument, and after the listing ended.
- `v49_c_md5_os`: `_MD5$` of several strings (with `CHR$(0)` and of 1,000 bytes), `INSTR(_OS$, …)` for the
  parts.
- `v49_d_clipboard`: set, get, clear, a string with `CHR$(0)` and extended characters.
- `v49_e_clear`: what `CLEAR` resets: numbers, strings, fixed-length strings, static and dynamic arrays (a use
  after it), `DIM SHARED`, `STATIC` variables of a SUB, `COMMON SHARED`, a user type with strings, the `DATA`
  position, open files (`FREEFILE`), `ON ERROR`, `RETURN` after `CLEAR` in a `GOSUB`, `CLEAR , , n`, and what it
  does not reset (`CONST`).
- `v49_f_param_type`: a `TYPE` in a SUB above as a parameter type, in the procedure's own body, an array
  parameter, a FUNCTION's parameter, a `DECLARE SUB` above the block, a `TYPE` in an included file above.
- `v49_x*`: one rejected or accepted form each: `_CAST` with `STRING`, `STRING * 4`, `_BIT`, `_UNSIGNED _BIT`,
  a user type, an unknown name, a string value, a user-type value, one argument, three, in a `CONST`; `CLEAR` in a
  SUB, a FUNCTION, `CLEAR (1)`, `CLEAR , , LEN("a")`, four arguments, a string argument; `_FILES$` with two
  arguments, a number; `_MD5$` of a number; `_CLIPBOARD$` with an argument; `_OS` without `$`.

Where a recording differs from a delta spec or this design, the spec or design is changed to match in task 1.1.

## Risks / Trade-offs

- [The clipboard is shared with the user's desktop] → one program of each suite uses it; a failure there is
  rerun alone before it is called a bug (D3).
- [`_FILES$` lists a folder whose contents the runner may change] → `v49_b` and `filesys_test` make and list
  folders of their own.
- [`CLEAR` reaches every variable kind `clear.txt` was written for but never run until now] → `v49_e` covers each
  kind, and `95_type_codegen_clear_inline_owner` the owner types.
