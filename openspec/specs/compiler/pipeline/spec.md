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
The parser SHALL produce a tree whose tokens, printed in order, reproduce the input file byte for byte, including
whitespace, comments, line ends and any text it could not parse.

#### Scenario: Round trip of the corpus
- **WHEN** every `.bas` file under `tests/corpus` is parsed
- **THEN** printing each tree gives the file's exact bytes, and no parse panics

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
allocation or the event loop. Optional arguments SHALL be represented as present or absent; procedure arguments
as a reference to a variable or a by-value copy of a value; variables SHALL carry a storage class (main module,
procedure-static, per-call local, parameter, function result, hidden temporary of a body); operations that may
raise a runtime error SHALL be marked. A body SHALL be a flat list of statements; labels SHALL be positions in a
body, the program's own and those the lowering adds, and control SHALL move between them only by jump, conditional
jump, `GOSUB` and `RETURN` operations of that body. The IR SHALL state that errors are pending and handled per
statement: a raising operation records a pending error and yields a placeholder value, the statement goes on (a
store made with that value still happens), only the points the IR names check for a pending error (each `PRINT`
item: a raising item skips the rest of its statement; a procedure's entry: a procedure entered while an error is
pending returns at once; a jump or conditional jump is not taken while an error is pending, unless the conditional
jump is marked as using the placeholder value), errors are serviced at the boundary of the statement where they
were raised, or, when a conditional jump using the placeholder value leaves that statement, at the boundary of the
next statement that runs; a retry re-runs the statement where the error is serviced, and resuming next continues
after it, in the procedure where it raised.

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
