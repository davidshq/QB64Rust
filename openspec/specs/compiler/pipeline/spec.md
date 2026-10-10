# compiler/pipeline Specification

## Purpose
How the new compiler turns a BASIC source file into an executable: the stages, what each one guarantees, and how
the result is built against the QB64pe runtime (libqb). Language coverage grows change by change; the guarantees
here hold for whatever subset is supported.

## Requirements

### Requirement: Source is bytes
The compiler SHALL read source files as bytes and SHALL NOT require them to be valid UTF-8. Every position it
reports or stores SHALL be a file and a byte offset; columns in diagnostics SHALL be 1-based byte columns.

#### Scenario: CP437 bytes in a string literal
- **WHEN** a program contains `PRINT "` followed by the bytes 0xC9 0xCD 0xBB and `"`
- **THEN** it compiles, and the executable writes exactly those three bytes followed by the line end

### Requirement: Lossless syntax tree
The parser SHALL produce one tree per file of a program (the main file and each inclusion of a file), and the
tokens of each tree, printed in order, SHALL reproduce that file byte for byte, including whitespace, comments,
line ends, inactive `$IF` branches and any text it could not parse.

#### Scenario: Round trip of the corpus
- **WHEN** every `.bas` file under `tests/corpus` is parsed
- **THEN** printing each tree gives the file's exact bytes, and no parse panics

#### Scenario: Round trip of an included file
- **WHEN** a program includes `lib.bi` with `'$INCLUDE:'lib.bi'`
- **THEN** printing the main tree gives the main file's bytes, and printing the included tree gives `lib.bi`'s
  bytes

### Requirement: Error recovery
The front end SHALL report at most one error per statement, SHALL continue with the next statement after an
error, and SHALL stop reporting after 100 errors with a final "too many errors" message.

#### Scenario: Two bad statements
- **WHEN** a program has an error in its line 2 and another in its line 5
- **THEN** both are reported, each with its line and column, and nothing else is reported for those lines

#### Scenario: Unsupported construct
- **WHEN** a program uses a statement the compiler does not support yet (for example `FOR`)
- **THEN** it reports "`FOR` is not supported yet" at that statement and produces no executable

### Requirement: Typed tree with explicit conversions
After resolution and type checking, every expression SHALL have a type, every implicit numeric conversion SHALL
be an explicit conversion node, and every operator node SHALL carry the type it is computed in.

#### Scenario: INTEGER plus literal
- **WHEN** the typed tree of `i% + 1` is dumped
- **THEN** the addition is shown computed in 32 bits with both operands converted from INTEGER

### Requirement: ABI-neutral IR
The IR SHALL NOT refer to libqb or `qbx.cpp` symbols, C types, `passed` masks, by-value temporaries, storage
allocation, array descriptors, byte offsets or the event loop. Optional arguments SHALL be represented as present
or absent; procedure arguments as a reference to a place or a by-value copy of a value; variables SHALL carry a
storage class (main module, procedure-static, per-call local, parameter, function result, hidden temporary of a
body); operations that may raise a runtime error SHALL be marked. A place SHALL be a variable, an element of an
array (one index per dimension, already converted to a 64-bit integer), or a member of a place of a user type;
values SHALL be read from places and stored into places. Types SHALL include user types, named by an id of the
program's list of user types. A body SHALL be a flat list of statements; labels SHALL be positions in a body, the
program's own and those the lowering adds, and control SHALL move between them only by jump, conditional jump,
`GOSUB` and `RETURN` operations of that body. The IR SHALL state that errors are pending and handled per
statement: a raising operation records a pending error and yields a placeholder value, the statement goes on (a
store made with that value still happens, by the rule of its place below), only the points the IR names check for
a pending error (each `PRINT` item: a raising item skips the rest of its statement; a procedure's entry: a
procedure entered while an error is pending returns at once; a jump or conditional jump is not taken while an
error is pending, unless the conditional jump is marked as using the placeholder value; a store into an array
element: its indexes are evaluated first, and while an error is pending after them the value is not evaluated
and the store is skipped), errors are serviced at the boundary of the statement where they were raised, or, when
a conditional jump using the placeholder value leaves that statement, at the boundary of the next statement that
runs; a retry re-runs the statement where the error is serviced, and resuming next continues after it, in the
procedure where it raised. The first error raised in a statement SHALL be the one serviced. A store into a variable
or into a member of a variable SHALL NOT check for a pending error. A store into a member of an array element SHALL
evaluate the value before the indexes, and SHALL be skipped when an index is out of range or when the indexes
raised an error while none was pending before them (`DIVERGENCES.md` D-004). An index out of range SHALL raise
error 9, and a read with it SHALL give the value at the array's first position.

#### Scenario: Optional argument absent
- **WHEN** `INSTR("hello", "ll")` is lowered
- **THEN** the IR call has three argument slots, the first absent, and the C++ passes a placeholder with a
  `passed` mask of 0

#### Scenario: By-reference and by-value arguments
- **WHEN** `CALL bump(n)` and `bump (n)` are lowered for `SUB bump (x AS LONG)` and a LONG `n`
- **THEN** the first IR call passes a reference to `n` and the second a copy of its value, and only the C++
  output names the temporary that holds the copy

#### Scenario: Error inside a PRINT
- **WHEN** an item of a `PRINT` statement raises a runtime error that is not trapped (end to end: `s12_error_in_print`
  in `tests/corpus/slice`, run with `QB64PE_NOPROMPT=continue` from its `.noprompt` file, so the program goes on)
- **THEN** the remaining items of that statement, and its line end, are not printed, and the next statement's
  output follows on the same line, as with the old compiler

#### Scenario: FOR loop lowered
- **WHEN** `FOR i% = 1 TO n: PRINT i%: NEXT` is lowered
- **THEN** the IR holds the limit and the step in hidden temporaries wider than `i%`, and the loop is statements
  with labels and jumps; no C type, `fornext_` name or `goto` text appears before the C++ output

#### Scenario: Header error follows from the pending-error rule
- **WHEN** the condition of a `WHILE` raises an error and the program resumes next
- **THEN** the conditional jump that leaves the loop is not taken, because an error is pending, so the body runs,
  as with the old compiler

#### Scenario: A store made with a placeholder value
- **WHEN** `x = 5: x = ASC("")` runs and the handler executes `RESUME NEXT`
- **THEN** `x` holds 0, the placeholder value, as with the old compiler (`verification/v17_a_pending`)

#### Scenario: A SUB called with a raising argument
- **WHEN** `show ASC("")` calls `SUB show (v AS SINGLE)` that prints a line, and the handler executes `RESUME NEXT`
- **THEN** the handler runs and the SUB prints nothing, as with the old compiler

#### Scenario: ELSEIF condition raises
- **WHEN** a block `IF` whose `IF` condition is false reaches `ELSEIF CHR$(k) = "a" THEN` with `k = -1`, its `ELSE`
  branch prints `else`, and the handler executes `RESUME NEXT`
- **THEN** the condition is tested with the placeholder value `""` (false), the error is serviced at the `ELSE`
  branch's `PRINT`, which prints nothing, and execution continues after it; with `RESUME` instead (and `k` set to
  65 by the handler), the `PRINT` is re-run and prints `else`

#### Scenario: FOR limits computed with a placeholder value
- **WHEN** `FOR i = 1 TO LEN(CHR$(-1)) + 2` raises an error in its limit and the program resumes next
- **THEN** all three hidden temporaries are stored (the limit from the placeholder value), the jump to the loop
  entry is not taken, and the body runs with `i` unassigned, as with the old compiler

#### Scenario: Places lowered
- **WHEN** `x(i) = 1: p.m = 2: a(i).m = 3: y = x(i) + p.m` is lowered for an array `x`, a user-type variable `p`
  and an array `a` of that type
- **THEN** the IR names an element place, a member place and a member of an element place, each index converted to
  a 64-bit integer, and no descriptor slot, `array_check` or byte offset appears before the C++ output

#### Scenario: Element store with a raising index
- **WHEN** `x(99) = 5` runs for `DIM x(10)` and the handler executes `RESUME NEXT`
- **THEN** error 9 is serviced and no element of `x` changes, as with the old compiler

#### Scenario: Element store with a raising value
- **WHEN** `x(9) = ASC("")` runs with `x(9)` holding 5 and the handler executes `RESUME NEXT`
- **THEN** `x(9)` holds 0, the placeholder value, as with the old compiler

#### Scenario: Member store with a raising value
- **WHEN** `p.m = ASC("")` runs with `p.m` holding 5 and the handler executes `RESUME NEXT`
- **THEN** `p.m` holds 0, the placeholder value, as with the old compiler

#### Scenario: Member of an element with a bad index
- **WHEN** `a(9).m = 5` runs for `DIM a(3) AS t` with `a(0).m` holding 70 and the handler executes `RESUME NEXT`
- **THEN** error 9 is serviced and `a(0).m` still holds 70 (the old compiler writes 5 there: `DIVERGENCES.md` D-004)

#### Scenario: Value before index in a member-of-element store
- **WHEN** `a(9).m = ASC("")` runs for the same array under a handler that prints `ERR`
- **THEN** the handler prints 5 once, the value's error, as with the old compiler, and no element changes

### Requirement: C++ output and build
The C++ emitter SHALL write every fragment that `qbx.cpp` includes into a build folder that is not inside the
QB64pe reference clone, and the driver SHALL build the executable with the clone's `Makefile` and toolchain,
compiling generated code with `-fwrapv`. A build SHALL NOT change any tracked file of the reference clone.

#### Scenario: Reference clone unchanged
- **WHEN** a slice program is built
- **THEN** `git status --porcelain` in the reference clone prints the same as before the build

#### Scenario: Same output as the old compiler
- **WHEN** a program in `tests/corpus/slice.list` is compiled by `qb64rust` and run by the corpus runner
- **THEN** its output equals the `.output` recorded from `qb64pe.exe`

### Requirement: Symbol table
`sema` SHALL record every variable (including parameters and function results), procedure and label as a symbol
with its kind, its type where it has one, the span of its definition and the spans of all its references, and
SHALL answer which symbol, if any, a byte position in a file refers to. The definition of an implicit variable is
its first use. A statement that has an error MAY contribute no symbols.

#### Scenario: Definition and references of a variable
- **WHEN** a program has `DIM n AS LONG`, then `n = 1` and `PRINT n&`
- **THEN** one symbol `n` of type LONG has the name in the `DIM` as its definition and the two later names as
  references, and a position inside any of the three names gives that symbol

#### Scenario: Same name, different symbols
- **WHEN** the main module uses `x` and a SUB uses its own implicit `x`
- **THEN** the two are different symbols, and a position in the SUB's `x` gives the SUB's symbol

#### Scenario: Call before definition
- **WHEN** `PRINT twice&(n)` comes before `FUNCTION twice& (a AS LONG)`
- **THEN** the name in the call is a reference of the procedure symbol whose definition is the name in the header

### Requirement: Unsupported constructs are marked
Every diagnostic SHALL say whether it reports an error in the program or a construct the compiler does not handle
yet. A diagnostic about a construct not handled yet SHALL be marked "not supported yet"; it SHALL NOT be used for
an error the old compiler also reports.

#### Scenario: Statement not handled yet
- **WHEN** a program uses `FOR`
- **THEN** the diagnostic at `FOR` is marked "not supported yet"

#### Scenario: Real error
- **WHEN** a program calls a SUB with the wrong number of arguments
- **THEN** the diagnostic is not marked

#### Scenario: Syntax the parser does not know
- **WHEN** the parser cannot read a statement because it uses syntax not handled yet (an operator such as `MOD`
  inside an otherwise supported statement)
- **THEN** the diagnostic is marked "not supported yet", not reported as a syntax error

### Requirement: The whole language parses
Every statement and expression form the old compiler accepts SHALL parse into a node of its own kind, without a
parser diagnostic. A construct that the later stages do not compile yet SHALL be reported "not supported yet" by
`sema` at that node, not by the parser.

#### Scenario: Statement not compiled yet
- **WHEN** a program contains `DO: SWAP a, b: LOOP UNTIL a > 0`
- **THEN** the tree has a `DoBlock` holding a `SwapStmt`, and the only diagnostic is one "not supported yet" error
  at `SWAP`

#### Scenario: Built-in statement read by its template
- **WHEN** a program contains `LINE (0, 0)-(9, 9), , BF`
- **THEN** it parses into one `BuiltinStmt` with `BF` as a word, not as a variable

#### Scenario: Member access after an index
- **WHEN** a program contains `PRINT a(1).b`
- **THEN** it parses without a diagnostic, and the item is a member access on `a(1)`

### Requirement: Block structure
Block statements (`IF`, `FOR`, `DO`, `WHILE`, `SELECT CASE`, `TYPE`, `DECLARE LIBRARY`, `DEF FN`, `SUB`,
`FUNCTION`) SHALL be nodes holding their body. A missing closer SHALL be one error at the block's header; a closer
without its block SHALL be one error at the closer; parsing SHALL continue after either.

#### Scenario: Missing END IF
- **WHEN** a program opens `IF a THEN` on line 2 and never closes it
- **THEN** one error is reported at line 2, and statements after it are still parsed

#### Scenario: One NEXT closes two loops
- **WHEN** a program has `FOR i = 1 TO 2`, then `FOR j = 1 TO 2`, then `NEXT j, i`
- **THEN** both loops are closed and no error is reported

### Requirement: Comment metacommands are never ignored
A comment whose first non-blank character after `'` or `REM` is `$` SHALL be examined for `$INCLUDE`, `$STATIC`
and `$DYNAMIC` by the old compiler's rule. Each one found SHALL either take effect or be reported "not supported
yet"; it SHALL NOT be treated as a plain comment.

#### Scenario: Comment include
- **WHEN** a program contains `'$INCLUDE: 'lib.bm'`
- **THEN** `lib.bm` is included after that line

#### Scenario: Not a metacommand
- **WHEN** a program contains `'$Format:Off`
- **THEN** it is an ordinary comment

### Requirement: Preprocessor
`$IF`, `$ELSEIF`, `$ELSE`, `$END IF` and `$LET` SHALL be evaluated in file order across included files, for
Windows 64-bit. Code in an inactive branch SHALL be kept in the tree but not parsed or checked; an active `$ERROR`
SHALL be an error.

#### Scenario: Inactive branch
- **WHEN** a program contains `$IF LINUX THEN`, a line that is not valid BASIC, and `$END IF`
- **THEN** no error is reported for that line

#### Scenario: Block header in a $IF branch, closer outside
- **WHEN** a program contains `$IF WIN THEN`, `IF x THEN`, `$END IF`, a statement and `END IF`
- **THEN** the compile fails with an error at the `$END IF` (the old compiler rejects it too, reporting the
  `END IF`)

#### Scenario: SUB inside a $IF branch
- **WHEN** a program contains `$IF WIN THEN`, a whole `SUB` … `END SUB`, and `$END IF`
- **THEN** no error is reported for the nesting

#### Scenario: Guarded self-include
- **WHEN** `a.bi` contains `$IF DONE = UNDEFINED THEN`, `$LET DONE = 1`, `'$INCLUDE:'a.bi'` and `$END IF`, and the
  main file includes `a.bi`
- **THEN** no error is reported, and `a.bi` is parsed twice, the second time with the branch inactive

#### Scenario: $LET in an included file
- **WHEN** `lib.bi` contains `$LET FAST = 1` and the main file, after including it, contains `$IF FAST = 1 THEN`
- **THEN** the branch is active

### Requirement: Included files
A comment `'$INCLUDE:'file'` (or `REM $INCLUDE:'file'`) SHALL include the file after the line it is on;
`$INCLUDE:'file'` without a comment SHALL be an error, as in the old compiler. An absolute path SHALL be used as
written; a relative one SHALL be looked up in the including file's folder, then relative to the compiler root (see
the CLI spec), never relative to the working directory. `$INCLUDEONCE` in a file SHALL make later inclusions of that file empty. A missing file, an empty name,
or an inclusion deeper than 100 levels SHALL be an error at the include; a file that includes itself SHALL NOT be
an error by itself. Diagnostics in an included file SHALL name that file and its line and column.

#### Scenario: Nested include not looked up in the main file's folder
- **WHEN** the main file includes `sub/a.bi`, `sub/a.bi` includes `b.bi`, and `b.bi` exists only next to the
  main file (and not under the compiler root)
- **THEN** the compile fails with "file not found" at the include in `sub/a.bi`

#### Scenario: Include relative to the compiler root
- **WHEN** a program includes `'$INCLUDE:'extra/x.bi'`, which exists only under the directory given with
  `--include-root`
- **THEN** that file is included

#### Scenario: Include without a comment
- **WHEN** a program contains `$INCLUDE:'lib.bi'` (no `'` or `REM`)
- **THEN** the compile fails with an error at that line

#### Scenario: Error in an included file
- **WHEN** `lib.bm` has a syntax error on its line 4 and the main file includes it
- **THEN** the error is reported at `lib.bm` line 4

#### Scenario: Block closed in an included file
- **WHEN** the main file opens a `FOR` and the matching `NEXT` is in a file it includes
- **THEN** the compile fails with a "not supported yet" error, not a syntax error

#### Scenario: Included twice
- **WHEN** a file that does not contain `$INCLUDEONCE` is included twice
- **THEN** it is parsed twice, and the names in each inclusion are distinct symbols

### Requirement: No follow-on errors after an unsupported declaration
After the first declaration reported "not supported yet", in file order, only "not supported yet" errors SHALL be
reported by the check of the rest of the program; syntax errors SHALL still be reported. Names the old compiler
knows but the new one does not yet (such as the constants of the auto-included files) SHALL be reported "not
supported yet", not as reserved names.

#### Scenario: Unsupported declaration
- **WHEN** a program has `DIM m AS _MEM` and then `PRINT LEN(m)`
- **THEN** the only error is the "not supported yet" error at the `DIM`

#### Scenario: Real error after an unsupported declaration
- **WHEN** a program has `DIM m AS _MEM` and, later, a statement that stores a string in a number variable
- **THEN** the only error is the "not supported yet" error at the `DIM`, and the compile fails

#### Scenario: Real error before an unsupported declaration
- **WHEN** a program has a statement that stores a string in a number variable and, later, `DIM m AS _MEM`
- **THEN** both errors are reported

#### Scenario: Auto-included constant
- **WHEN** a program contains `PRINT _TRUE`
- **THEN** the error at `_TRUE` is marked "not supported yet"
