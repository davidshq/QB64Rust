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
The IR SHALL NOT refer to libqb or `qbx.cpp` symbols, C types, `passed` masks or the event loop. Optional
arguments SHALL be represented as present or absent; operations that may raise a runtime error SHALL be marked;
the IR SHALL state that errors are handled per statement: after a raising operation the rest of the statement is
skipped, and errors are serviced at the statement boundary.

#### Scenario: Optional argument absent
- **WHEN** `INSTR("hello", "ll")` is lowered
- **THEN** the IR call has three argument slots, the first absent, and the C++ passes a placeholder with a
  `passed` mask of 0

#### Scenario: Error inside a PRINT
- **WHEN** an item of a `PRINT` statement raises a runtime error that is not trapped
- **THEN** the remaining items of that statement are not printed, as with the old compiler

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
