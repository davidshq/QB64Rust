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
array (one index per dimension, already converted to a 64-bit integer; the array a variable or a member array of
a place of a user type), or a member of a place of a user type;
values SHALL be read from places and stored into places. Types SHALL include user types, named by an id of the
program's list of user types. A body SHALL be a flat list of statements; labels SHALL be positions in a body, the
program's own and those the lowering adds, and control SHALL move between them only by jump, conditional jump,
`GOSUB` and `RETURN` operations of that body. The IR SHALL state that errors are pending and handled per
statement: a raising operation records a pending error and yields a placeholder value, the statement goes on (a
store made with that value still happens, by the rule of its place below), only the points the IR names check for
a pending error (each `PRINT` item: a raising item skips the rest of its statement; a procedure's entry: a
procedure entered while an error is pending returns at once; a jump or conditional jump is not taken while an
error is pending, unless the conditional jump is marked as using the placeholder value; a store into an element
of an array variable: its indexes are evaluated first, and while an error is pending after them the value is not
evaluated and the store is skipped), errors are serviced at the boundary of the statement where they were raised, or, when
a conditional jump using the placeholder value leaves that statement, at the boundary of the next statement that
runs; a retry re-runs the statement where the error is serviced, and resuming next continues after it, in the
procedure where it raised. The first error raised in a statement SHALL be the one serviced. A store into a variable
or into a member of a variable SHALL NOT check for a pending error. A store into a place reached through a member
with an index on the way (a member of an array element, an element of a member array) SHALL evaluate a numeric
value before the indexes and a string value after them (measured, `verification\v24_x313`, `x316`: the old
compiler's string store evaluates its place first), and SHALL be skipped when an index is out of range or when
the indexes raised an error while none was pending before them (`DIVERGENCES.md` D-004). An index out of range
SHALL raise error 9, and a read with it SHALL give the value at position 0 of that dimension.

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

#### Scenario: String value after the indexes
- **WHEN** `z$ = "before": a(setz(1)).s = z$` runs for a `STRING` member `s`, where FUNCTION `setz` stores
  `"after"` into the shared `z$` and returns its argument
- **THEN** `a(1).s` holds `after`, as with the old compiler; the same for `z$ + "!"` (`after!`), for a
  `STRING * n` member and for an element of a member array (`verification\v24_x316`)

#### Scenario: Element of a member array with a bad index
- **WHEN** `t.v(9) = 5` runs for a member `v(0 TO 3) AS LONG` with `t.v(0)` holding 100, and `a(1).v(9) = 6` for
  an array `a` of the type, and the handler executes `RESUME NEXT`
- **THEN** error 9 is serviced each time and no element changes (the old compiler writes `t.v(0)` and
  `a(1).v(0)`: `DIVERGENCES.md` D-004)

### Requirement: C++ output and build
The C++ emitter SHALL write every fragment that `qbx.cpp` includes into a build folder that is inside neither the
QB64pe reference clone nor this repo's copy of libqb, and the driver SHALL build the executable with the copy's
`Makefile`, `qbx.cpp` and libqb sources and the clone's toolchain, compiling generated code with `-fwrapv`. The
driver's flags (`-fwrapv`, `-O2` for `OptimizeCppProgram`) SHALL reach the program's own objects only; libqb's
objects SHALL be built with none of them, whatever build makes them. The libqb objects a build needs SHALL be
written into the copy's git-ignored places, never into the clone, and a copy whose objects were built by another
toolchain or with other flags of the driver SHALL have them built again. A build SHALL NOT change any tracked file
of the reference clone or of the copy.

#### Scenario: Reference clone unchanged
- **WHEN** a slice program is built
- **THEN** `git status --porcelain` in the reference clone prints the same as before the build

#### Scenario: The copy unchanged
- **WHEN** a slice program is built for the first time after the copy is checked out, so that libqb's objects
  are built
- **THEN** `git status --porcelain libqb` in this repo prints nothing

#### Scenario: Same output as the old compiler
- **WHEN** a program in `tests/corpus/slice.list` is compiled by `qb64rust` and run by the corpus runner
- **THEN** its output equals the `.output` recorded from `qb64pe.exe`

#### Scenario: An optimised build first
- **WHEN** the copy has no libqb objects and a program with `OptimizeCppProgram` is built first
- **THEN** `qbx.cpp` is compiled with `-O2 -fwrapv` and libqb's sources without either

#### Scenario: Objects of an earlier build
- **WHEN** the copy holds libqb objects built before its stamp was written, or by another toolchain, and a program
  is built
- **THEN** those objects are removed and built again, and the copy's stamp names the toolchain

### Requirement: The copy of libqb equals the reference commit
This repo SHALL hold a copy of QB64pe's `Makefile`, `internal\c` (without the toolchain), `licenses` and
`COPYING.txt`, each file byte for byte as at the commit its `SOURCE.md` names, which SHALL be the reference
clone's commit, but for the copy's own edits. Each edit SHALL be a file of `tools\libqb\edits` that names its
target file and the exact lines it replaces, and `SOURCE.md` SHALL list the edits. A tool SHALL write the copy from
the clone and its edits and SHALL say, with `--check`, which files differ from the commit's with the edits applied,
are missing or are extra; an edit whose lines do not occur exactly once in its target SHALL stop the tool.

#### Scenario: A file edited by hand
- **WHEN** a file of the copy is edited and the check is run
- **THEN** it exits with status 1 and names the file

#### Scenario: The copy as written
- **WHEN** the copy has just been written by the tool and the check is run
- **THEN** it exits with status 0

#### Scenario: An edit of the copy
- **WHEN** an edit file names a file of the copy and lines it holds once, and the tool writes the copy
- **THEN** that file holds the edit's new lines, `SOURCE.md` names the edit, and the check exits with status 0

#### Scenario: An edit that does not fit
- **WHEN** an edit's old lines do not occur in its target, or occur twice
- **THEN** the tool stops with status 1 and writes nothing

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
- **WHEN** a program contains `DO: RUN: LOOP UNTIL a > 0`
- **THEN** the tree has a `DoBlock` holding a `RunStmt`, and the only diagnostic is one "not supported yet" error
  at `RUN`

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
- **WHEN** a program has the block `TYPE cos … END TYPE` (a `TYPE` named like a built-in function), and after it
  `PRINT 1`
- **THEN** the only error is the "not supported yet" error at the `TYPE` block

#### Scenario: Real error after an unsupported declaration
- **WHEN** a program has the block `TYPE cos … END TYPE` and, later, a statement that stores a string in a number
  variable
- **THEN** the only error is the "not supported yet" error at the `TYPE` block, and the compile fails

#### Scenario: Real error before an unsupported declaration
- **WHEN** a program has a statement that stores a string in a number variable and, later, the block `TYPE cos …
  END TYPE`
- **THEN** both errors are reported

#### Scenario: Auto-included constant
- **WHEN** a program contains `PRINT _TRUE`
- **THEN** the error at `_TRUE` is marked "not supported yet"

### Requirement: Built-in statements in the IR
The IR SHALL represent a built-in statement by the built-in's identity, never by a libqb name: one slot per
argument in table order, each a value, a place or absent, and the words chosen where the statement's form offers a
choice. File output and input, `READ` and console input SHALL be operations of their own that list their items or
targets in order. Each SHALL be marked as possibly raising, and the IR SHALL state where each checks for a pending
error (file output and input before each item or target, `READ` nowhere, as measured) so that the emitter decides
none of it.

#### Scenario: A statement call lowered
- **WHEN** `OPEN f$ FOR OUTPUT AS #1` is lowered
- **THEN** the IR names the built-in `OPEN` with the name value, the chosen mode word, the file number value and
  the absent slots, and no `sub_open`, mode number or `passed` mask appears before the C++ output

#### Scenario: A place argument
- **WHEN** `SWAP x(i), y` is lowered
- **THEN** the IR names two places, and no temporary or address-of appears before the C++ output

#### Scenario: Items checked one by one
- **WHEN** `PRINT #1, "a"; CHR$(-1); "b"` is lowered
- **THEN** the IR lists three items for file 1 under the rule that a raising item skips the rest of the statement,
  as for `PRINT`

### Requirement: Program data in the IR
The IR SHALL hold the program's `DATA` items as one ordered list of byte strings, each marked quoted or not, with
the position in that list of each label that a `RESTORE` names. `READ` SHALL name its target places and `RESTORE`
a position, with the label that gave it; the encoding of the list in the executable SHALL be the emitter's.

#### Scenario: Data lowered
- **WHEN** a program with `DATA 1,"two"` and, after the label `lab:`, `DATA 3` and `RESTORE lab` is lowered
- **THEN** the IR's data list has three items, the second quoted, the restore names position 2 and the label
  `LAB`, and no `data_offset` name or separator byte appears before the C++ output

### Requirement: Array storage in the IR
An array of the IR SHALL be static (its bounds are constants, it exists for the whole program) or dynamic (it has
a number of dimensions and no bounds). A dynamic array that lives for the program MAY carry constant bounds it is
allocated with when the program starts (`STATIC a(3)` in a procedure); no operation allocates it then, and the IR
SHALL NOT say where that allocation stands in the output. The IR SHALL have operations that allocate a dynamic
array with bounds given as values (stating whether an allocated array is an error, is replaced, is replaced
keeping its elements by flat position, or is replaced keeping its elements by their indexes), that clear a static
array, that release a dynamic one, that erase an
array parameter, which is either kind when the program runs, and that copy the elements of one array into
another of the same element type and number of dimensions (each an array variable or a member array; the
checks of bounds and the copy by kind follow from the types and SHALL NOT appear as other operations). The allocation SHALL be allowed to name a static array or a
parameter (it then raises an error when it runs). An array SHALL carry a storage class
as a scalar does, and an array parameter SHALL be a reference to the caller's array. The IR SHALL NOT state when a
per-call array is released, nor name a descriptor, an allocation function or a size in bytes.

#### Scenario: DIM with a variable bound
- **WHEN** `n = 4: DIM d(n) AS LONG` is lowered
- **THEN** the IR holds a dynamic array `d` of one dimension and an allocation with the bounds 0 and `n` converted
  to a 64-bit integer, marked as raising, and no descriptor slot or `calloc` appears before the C++ output

#### Scenario: REDIM _PRESERVE
- **WHEN** `REDIM _PRESERVE d(9) AS LONG` is lowered
- **THEN** the IR holds the same operation marked as keeping the elements

#### Scenario: REDIM _RETAIN
- **WHEN** `REDIM _RETAIN d(2 TO 9) AS LONG` is lowered
- **THEN** the IR holds the same operation marked as keeping the elements by their indexes, and no loop over the
  dimensions or old descriptor appears before the C++ output

#### Scenario: Static array
- **WHEN** `DIM s(5) AS LONG` is lowered in the main module without `$DYNAMIC`
- **THEN** the IR holds a static array with the bounds 0 and 5 and no operation at the `DIM` line

#### Scenario: STATIC array of a procedure
- **WHEN** a SUB with `STATIC c(3) AS LONG` is lowered
- **THEN** the IR holds a dynamic array `c` of one dimension with static storage, allocated at the start with the
  bounds 0 and 3, and no operation at the `STATIC` line

#### Scenario: Local array
- **WHEN** a SUB with `DIM t(5) AS LONG` is lowered
- **THEN** the IR holds a local dynamic array `t` and an allocation at the `DIM` line, and nothing that releases it

#### Scenario: Array argument
- **WHEN** `fill x()` is lowered for `SUB fill (a() AS LONG)`
- **THEN** the argument is a reference to the array `x`, and inside the SUB `a(1)` is an element of the parameter

#### Scenario: Whole-array assignment
- **WHEN** `d() = s()` and `t.v() = a()` are lowered
- **THEN** the IR holds one copy from `s` to `d`, marked as raising, and one from `a` to member `v` of `t`, and no
  loop over the elements, descriptor or bounds check appears before the C++ output

### Requirement: Member arrays in the IR
An element place SHALL name its array as an array variable or as an array member of a place of a user type, so
that `x(0).n(1).q(2)` is one place built from elements and members. The bounds of a member array SHALL be part of
the user type in the program's list of types, with whether it is a dynamic member array. A whole user-type store
SHALL be one operation from a place to a place of the same type. The operation that clears an array SHALL be
allowed to name a member array, and then finds the place it is part of once and is not guarded (an index outside
its bounds on the way raises error 9 and the member array of the element at position 0 is cleared, as in the old
compiler); no operation SHALL allocate or release a member array that is not dynamic. A dynamic member array
SHALL be allocated and released by the operations that allocate and release a dynamic array, naming it as a
member array, with the same modes and the same order of evaluation and errors. A bound of a member array and a
whole member array as an array argument SHALL name it the same way; the bound SHALL NOT evaluate the indexes on
the way to a member array that is not dynamic. Byte offsets, the layout of a member array, how a `STRING` member
or a dynamic member array is held, and the setting up, release and copy of the dynamic member arrays an owner
value holds SHALL NOT appear in the IR: they follow from the type.

#### Scenario: Chain of members and elements
- **WHEN** `x(0).n(1).q(2) = 77` is lowered
- **THEN** the IR shows one store whose place is element 2 of member `q` of element 1 of member `n` of element 0
  of `x`, each index converted to a 64-bit integer, and no offset arithmetic

#### Scenario: Whole value
- **WHEN** `q = p` is lowered for two variables of one user type
- **THEN** the IR holds one whole-value store from `p` to `q`

#### Scenario: REDIM and ERASE of a member array
- **WHEN** `REDIM a(i).v(0 TO 3)` and `ERASE t.v` are lowered for a member `v(0 TO 3) AS LONG`
- **THEN** the IR holds a clear of member `v` of element `i` of `a`, marked as raising, and a clear of member `v`
  of `t`, not marked; `REDIM _PRESERVE a(i).v(0 TO 3)` gives no operation

#### Scenario: REDIM and ERASE of a dynamic member array
- **WHEN** `REDIM _RETAIN a(i).v(2 TO 5) AS LONG` and `ERASE t.v` are lowered for a member `v(1) _DYNAMIC AS LONG`
- **THEN** the IR holds an allocation of member `v` of element `i` of `a` with bounds 2 and 5 and the mode
  `Retain`, and a release of member `v` of `t`

#### Scenario: Copy of an owner value
- **WHEN** `q = p` is lowered for two variables of a type with a dynamic member array
- **THEN** the IR holds one whole-value store from `p` to `q`, as for any user type

### Requirement: $STATIC and $DYNAMIC take effect
`$DYNAMIC` and `$STATIC` in a metacommand comment SHALL decide, for the declarations that follow by the old
compiler's rule for where the metacommand starts to apply, whether a `DIM` with constant bounds makes a static or
a dynamic array. They SHALL NOT be reported "not supported yet".

#### Scenario: Dynamic, then static again
- **WHEN** `'$DYNAMIC`, `DIM a(5)`, `'$STATIC`, `DIM b(5)` stand on four lines of the main module
- **THEN** `a` is a dynamic array and `b` a static one

#### Scenario: Not a metacommand
- **WHEN** `' note $DYNAMIC` stands above `DIM a(5)`
- **THEN** `a` is a static array

### Requirement: Precompiler flags
A `$IF` or `$ELSEIF` condition that names `_CONSOLE_`, `_EXPLICIT_`, `_EXPLICITARRAY_`, `_ASSERTS_`, `_DEBUG_` or
`_SOCKETS_` SHALL be evaluated with the value the old compiler gives the flag from the whole program, the same on
every line of the code that is compiled. The values SHALL be settled as the old compiler settles them, by
compiling again until they no longer change. Such a condition SHALL NOT be reported "not supported yet", except
where the old compiler's own passes disagree about the flag's value on that line.

#### Scenario: Above the line that sets it
- **WHEN** a program tests `$IF _CONSOLE_ = 1 THEN` on a line above its `$CONSOLE`
- **THEN** the branch is taken

#### Scenario: $CONSOLE:ONLY alone
- **WHEN** a program with `$CONSOLE:ONLY` and no `$CONSOLE` tests `_CONSOLE_`
- **THEN** its value is 2

#### Scenario: $CONSOLE:ONLY, then $CONSOLE
- **WHEN** a program has `$CONSOLE:ONLY` and below it `$CONSOLE`
- **THEN** `_CONSOLE_` is 1

#### Scenario: OPTION _EXPLICIT
- **WHEN** a program has `OPTION _EXPLICIT` on its last line and `$IF _EXPLICIT_ THEN` and `$IF _EXPLICITARRAY_
  THEN` above it
- **THEN** both branches are taken

#### Scenario: OPTION _EXPLICITARRAY
- **WHEN** a program has `OPTION _EXPLICITARRAY` on its last line and tests both flags above it
- **THEN** the `_EXPLICITARRAY_` branch is taken and the `_EXPLICIT_` branch is not

#### Scenario: Between two lines that set the flag differently
- **WHEN** a program has `$CONSOLE:ONLY`, then `$IF _CONSOLE_ = 2 THEN` … `$END IF`, then `$CONSOLE`
- **THEN** that `$IF` is reported "not supported yet" (the old compiler's two passes disagree about it: its
  program prints nothing from the branches, and a `CONST` in them is "Name already in use"), and no branch is
  compiled with a guessed value

#### Scenario: Nothing sets a flag
- **WHEN** a program with none of these features tests `$IF _CONSOLE_ THEN`, `$IF _ASSERTS_ THEN`, `$IF _DEBUG_
  THEN` and `$IF _SOCKETS_ THEN`
- **THEN** none of the branches is taken

#### Scenario: A metacommand inside the branch it changes
- **WHEN** a program's only `$CONSOLE` stands inside `$IF _CONSOLE_ = 0 THEN` … `$END IF`
- **THEN** the program has a console and `_CONSOLE_` is 1 everywhere else in it, as with the old compiler

#### Scenario: $CONSOLE:ONLY inside the branch it changes
- **WHEN** a program's only `$CONSOLE:ONLY` stands inside `$IF _CONSOLE_ = 0 THEN` … `$END IF`
- **THEN** `_CONSOLE_` is 2, and the program has a console and a window (the line that would make it one without
  a window is not compiled), as with the old compiler

#### Scenario: OPTION _EXPLICIT inside the branch it changes
- **WHEN** a program's only `OPTION _EXPLICIT` stands inside `$IF _EXPLICIT_ = 0 THEN` … `$END IF`, and a line
  below uses a variable that is not declared
- **THEN** `_EXPLICIT_` is 1 and that use is the error of a variable that is not declared, as with the old
  compiler (the option is in force by its flag, though its line is not compiled in the last round)

#### Scenario: Three rounds
- **WHEN** a program's only `$CONSOLE` stands inside `$IF _EXPLICIT_ THEN` … `$END IF` and its only `OPTION
  _EXPLICIT` on the last line
- **THEN** the program has a console, and `_CONSOLE_` and `_EXPLICIT_` are 1 on every line

#### Scenario: A setting inside a branch a line above has switched off
- **WHEN** a program has `$CONSOLE` on its first line and below it `OPTION _EXPLICIT` inside `$IF _CONSOLE_ = 0
  THEN` … `$END IF`
- **THEN** `_EXPLICIT_` is 0: the branch is not taken in any round, as with the old compiler

### Requirement: Program settings in the IR
The IR SHALL say of a program, in terms that name no libqb symbol and no make flag: whether it has a console,
whether it has a window, whether the window is hidden at the start, which optional parts of the runtime its
built-ins need, and whether its executable carries an icon. The IR dump SHALL show them, and SHALL NOT show
the folder the icon's file is in.

#### Scenario: Console and hidden window
- **WHEN** a program with `$CONSOLE` and `$SCREENHIDE` is lowered
- **THEN** the IR says it has a console and a window that starts hidden

#### Scenario: Console only
- **WHEN** a program with `$CONSOLE:ONLY` is lowered
- **THEN** the IR says it has a console and no window

#### Scenario: Sound
- **WHEN** a program with `SOUND 440, 1` is lowered
- **THEN** the IR says it needs the audio part of the runtime, and a program with only `BEEP` does not

#### Scenario: An icon
- **WHEN** a program with `$EXEICON:'./game.ico'` is lowered
- **THEN** the IR says its executable carries an icon and that it needs the icon part of the runtime

### Requirement: The build follows the program
The emitter and the driver SHALL give a program the start-up settings and the make flags the old compiler gives
it: a console, a console without a window, a window hidden at the start, each optional part of the runtime
one of its built-ins needs, the icon `$EXEICON` names as a resource of the executable, and the library files its
`DECLARE LIBRARY` blocks name on the link command. Two builds that differ in any of them, or in the bytes of the
icon, of a library file or of a header a block names, SHALL NOT share a cached executable.

#### Scenario: A program with a console and a window
- **WHEN** a program with `$CONSOLE`, `$SCREENHIDE` and `_DEST _CONSOLE` is built and run
- **THEN** it links against the runtime with the window, shows no window, and its output equals the old
  compiler's

#### Scenario: A built-in that needs a part of the runtime
- **WHEN** a program names `_ICON`
- **THEN** it is built with the make flag the old compiler passes for the icon dependency

#### Scenario: Reference clone unchanged
- **WHEN** a program with `$CONSOLE` is built for the first time
- **THEN** `git status --porcelain` in the reference clone prints the same as before the build

#### Scenario: A program with sound
- **WHEN** a program names `SOUND`, `PLAY` or another built-in of the audio part (`_SNDOPEN`, `_SNDRAW`, `_MIDISOUNDBANK`, `_WAVE`, `_MEMSOUND`, …)
- **THEN** it is built with the make flag the old compiler passes for the audio dependency, and a program
  without them is built without it

#### Scenario: An icon in the executable
- **WHEN** a program with `$EXEICON:'./game.ico'` is built
- **THEN** the executable holds the images of `game.ico` as its icon resource, as the old compiler's does

#### Scenario: A changed icon
- **WHEN** the icon file's bytes change and the program is built again with the build cache
- **THEN** the executable is built again, not copied from the cache

#### Scenario: A file that is not an icon
- **WHEN** `$EXEICON` names a text file
- **THEN** the build fails, as the old compiler's does

#### Scenario: A library on the link command
- **WHEN** a program's block names `"./v37c"` and `v37c.a` lies beside it
- **THEN** the executable is linked with that file, as the old compiler's is

#### Scenario: A changed header
- **WHEN** the bytes of a header a block names change and the program is built again with the build cache
- **THEN** the executable is built again, not copied from the cache

### Requirement: The version resource in the build
A program with `$VERSIONINFO` SHALL be built with the version resource and the manifest the old compiler writes
for it, the same bytes, after the icon of `$EXEICON` when it has one, and with the make flag the old compiler
passes for them. Two builds whose version resources differ SHALL NOT share a cached executable.

#### Scenario: The files of every measured program
- **WHEN** the front end and the driver write the resource files of each accepted `verification\v40_*` program
- **THEN** the resource file, its header and the manifest equal those `qb64pe.exe` wrote for it, apart from the
  manifest's file name, which follows the executable's

#### Scenario: Read back from the executable
- **WHEN** a slice program with `$VERSIONINFO` lines is built and prints its own version resource
- **THEN** its output equals the old compiler's program's

#### Scenario: With an icon
- **WHEN** a program has `$EXEICON:'./icon.ico'` and `$VERSIONINFO:ProductName='p'`
- **THEN** the executable carries both, and its resource file has the icon line first, as the old compiler's

#### Scenario: A changed value
- **WHEN** a `$VERSIONINFO` value changes and the program is built again with the build cache
- **THEN** the executable is built again, not copied from the cache

### Requirement: Partial array copies in the IR
The IR SHALL have one operation for `_ARRAYCOPY`: a source and a destination, each an array variable or a member
array, with the source's ranges (none for the whole array) and the destination's starts (none for the whole
array) as 64-bit integer values, evaluated in the statement's order. Bounds checks, the copy by kind and the
choice of runtime entry point SHALL follow from the operands and SHALL NOT appear as other operations.

#### Scenario: A range to a start
- **WHEN** `_ARRAYCOPY s(2 TO 4) TO d(f(1))` is lowered
- **THEN** the IR holds one copy from `s` with the range 2 to 4 to `d` at the value of `f(1)`, each converted to a
  64-bit integer, marked as raising, and no loop, descriptor or bounds check appears before the C++ output

#### Scenario: Member arrays
- **WHEN** `_ARRAYCOPY t(1).v() TO a(1, 1)` is lowered for a member array `v` of two dimensions
- **THEN** the IR holds one copy from member `v` of `t(1)` as a whole to `a` at the two starts

### Requirement: Event handlers in the IR
The IR SHALL have one operation that sets up an event handler: the event (timer or key), its operands as values of
the types the old compiler converts them to, and the handler, either a label of the main module or a procedure with
its argument as a 64-bit integer value. The label MAY belong to the main module while the operation stands in a
procedure; no other jump SHALL leave its body.

#### Scenario: A timer handler set up in a procedure
- **WHEN** `ON TIMER(t, 1.5) GOSUB h` inside a SUB is lowered, `h` a label of the main module
- **THEN** the IR holds one handler setup for timer `t` (a 32-bit integer) every 1.5 seconds (a double) with the
  main module's label `h`, it passes the IR's validation, and the main module keeps `h` as a place that can be
  entered

#### Scenario: A key handler with a SUB
- **WHEN** `ON KEY(3) s x%` is lowered for `SUB s (n AS INTEGER)`
- **THEN** the IR holds one handler setup for key 3 with procedure `s` and the value of `x%` converted to a 64-bit
  integer
