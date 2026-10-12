# testing/differential-tests Specification

## Purpose
Generated BASIC programs that compare the new compiler's numeric results with the old compiler's, for every operator,
pair of numeric types and conversion, so that typing and conversion rules are checked by measurement rather than
derived by hand (`study\07` test layer 3, `study\20` §3.3).

## Requirements

### Requirement: Generated programs
A generator SHALL write the differential programs under `tests/differential/` from a fixed seed, so that running it
again writes byte-identical files. Together the programs SHALL apply every binary operator to every ordered pair of
numeric types, every unary operator to every numeric type, and store every numeric type into every numeric type,
each at the types' boundary values and at seeded random values, with operands held in variables and, in separate
programs, as literals. Each result SHALL be printed on its own line, labelled so that a differing line names its
operator, types and values.

#### Scenario: Regenerated without change
- **WHEN** the generator is run twice
- **THEN** the files it writes are identical, and identical to the files in the repository

#### Scenario: Every pair covered
- **WHEN** the programs for `+` are read
- **THEN** they contain a line for each ordered pair of the numeric types

### Requirement: Safe programs
A generated program SHALL run to its end with both compilers: it SHALL NOT divide an integer by 0 (fatal), SHALL
NOT use a case that crashes the old compiler's program (the smallest integer `\ -1` and `MOD -1`) or that a
`DIVERGENCES.md` row decides differently (a `_BIT * n` with n > 32 SHALL have a `_BIT * 32` pad DIMmed just
before it, so that the old compiler's overlap, D-009, hits only the pad), SHALL NOT use a form either compiler
rejects or reports "not supported yet" (`^` with an `_OFFSET` operand, a signed radix literal wider than its type),
SHALL trap every other runtime error with an `ON ERROR` handler that prints `ERR` and resumes with the next
statement, and SHALL NOT use a `PRINT` comma.

#### Scenario: Error inside an expression
- **WHEN** a generated `^` line raises error 5 (a negative base with a fractional exponent)
- **THEN** the program prints the error number on that line's place and goes on with the next line

#### Scenario: Excluded case named
- **WHEN** the generator leaves out a value pair because of a divergence
- **THEN** the program's header comment names the `DIVERGENCES.md` row

### Requirement: Recorded with the old compiler
Each generated program SHALL have its expected output in a `.output` file beside it, recorded with `qb64pe.exe`
(default build) by the corpus runner's `--record`, which runs each program twice and writes only equal output. A
program whose text changed SHALL be recorded again before it is committed (a stale recording shows as a tier 2
failure).

#### Scenario: Missing recording
- **WHEN** a generated program has no `.output` file
- **THEN** tier 1 fails and names the program

### Requirement: Run in every tier
Tier 1 (`cargo test`) SHALL parse and check every differential program without C++ (no panic, exact round trip)
and require that each regenerates unchanged. Tier 2 and CI SHALL build and run with `qb64rust` every program named
in `tests/differential/pass.list` through the corpus runner (`--corpus-root tests/differential`) and require that
each passes. The list SHALL only grow; by the end of the change that introduces it, it SHALL name every program.

#### Scenario: Tier 2 run
- **WHEN** `run_legacy_tests.py --suite corpus --corpus-root tests/differential --list tests/differential/pass.list
  --qb64 target/release/qb64rust.exe` runs
- **THEN** every listed program passes

#### Scenario: A difference is found
- **WHEN** a differential program prints a line that differs from its recording
- **THEN** the run fails and the line shows the operator, the types and the values that differ
