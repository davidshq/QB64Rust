# compiler/unit-tests Specification

## Purpose
The `qb64rust test` subcommand: unit tests of BASIC code, written as SUBs in test files, built by the new compiler
and run one process per test, with results that cannot pass a test that did not run to its end.

## Requirements

### Requirement: Test files and test SUBs
`qb64rust test [<path>...]` SHALL take each `.bas` file named, and every file whose name ends in `_test.bas`
(any case) in a folder named or below it; with no path, the working folder. A test SHALL be a SUB of the test file
itself, not of a file it includes, whose name starts with `Test` (any case) and that has no parameters. Tests SHALL
be found from the parsed program, not by matching text.

#### Scenario: Found in a folder
- **WHEN** `lib\math_test.bas` has `SUB TestAdd`, `SUB test_sub`, `SUB TestWith (x AS LONG)` and `FUNCTION TestF`,
  and `qb64rust test lib` runs
- **THEN** the tests are `TestAdd` and `test_sub`; `TestWith` and `TestF` are not tests

#### Scenario: A SUB of an included file
- **WHEN** `math_test.bas` includes `math.bm`, which has `SUB TestHelper`
- **THEN** `TestHelper` is not a test of `math_test.bas`

#### Scenario: A commented-out SUB
- **WHEN** a test file has the line `' SUB TestOld` and a string holding `SUB TestStr`
- **THEN** neither is a test

#### Scenario: Not a test file by name
- **WHEN** `qb64rust test lib` runs and `lib\helpers.bas` has `SUB TestX`
- **THEN** `helpers.bas` is not taken; `qb64rust test lib\helpers.bas` takes it

### Requirement: The entry point
For each test file the command SHALL build one program: a console program with assertions on (`$ASSERTS`), made of
the assertion library, the test file as written, and an entry point that runs the test SUB the run names, then ends
with `SYSTEM`. The test file SHALL be compiled as if it were included from its own folder, so its `$INCLUDE`s and
the settings file of the fixes resolve as for a build of it. The generated files SHALL NOT be written into the
test file's folder.

#### Scenario: Includes resolve from the test file
- **WHEN** `lib\math_test.bas` has `'$INCLUDE:'math.bm'` and `lib\math.bm` exists
- **THEN** the test file builds, and nothing is added to `lib\`

#### Scenario: Module-level code runs before each test
- **WHEN** a test file's module-level code sets `DIM SHARED n AS LONG: n = 5` and `TestA` does `n = n + 1` and
  asserts `n` is 6, and `TestB` asserts `n` is 5
- **THEN** both pass: each test starts from the module-level code in a process of its own

#### Scenario: The fixes of the test file's folder
- **WHEN** `qb64rust.toml` beside the test file turns `resume-after-block` on
- **THEN** the test program is built with that fix on, as `qb64rust -x` of the test file would be

#### Scenario: A compile error
- **WHEN** a test file has a compile error on its line 4
- **THEN** every test of that file is reported as a build error, with the diagnostic naming the test file and line
  4, and the other test files still run

### Requirement: Running a test
Each test SHALL run in a process of its own, with its working folder the test file's folder, its standard input the
null device, `QB64PE_NOPROMPT=y`, and a timeout (`--timeout <seconds>`, default 30). Tests SHALL run up to
`--jobs <n>` at a time (default: the number of processors). A test that wants a folder of its own SHALL get one from
`TestTempDir$`, made empty before the test and removed after a pass.

#### Scenario: A timeout
- **WHEN** `TestLoop` runs `DO: LOOP` and `--timeout 2` is given
- **THEN** it is ended after about 2 seconds and reported as timed out, and the other tests still run

#### Scenario: Input does not wait
- **WHEN** a test runs `LINE INPUT a$`
- **THEN** it reads the end of the input at once, which raises error 62, "Input past end of file"
  (`QB64PE-DIVERGENCES.md` D-048), so the test is an error; it does not wait for the keyboard

#### Scenario: A folder of its own
- **WHEN** two tests run at once and each writes `TestTempDir$ + "out.txt"`
- **THEN** each reads back what it wrote

### Requirement: How a test is judged
A test SHALL pass only when the program recorded that the test SUB returned, no assertion failed, and it exited
with code 0. It SHALL fail when an assertion failed, with the assertion's message. It SHALL be an error when a
runtime error ended it (with libqb's message), when it ended before its SUB returned (`END`, `SYSTEM`, a crash), or
when the named test is not in the program. Printed output SHALL never by itself decide a result.

#### Scenario: A runtime error
- **WHEN** `TestDiv` runs `DIM a(3): a(9) = 1`
- **THEN** it is an error with the message "Subscript out of range" and its line

#### Scenario: Ending early
- **WHEN** `TestEnd` calls a SUB of the code under test that runs `SYSTEM`
- **THEN** it is an error saying the test ended before its SUB returned, not a pass

#### Scenario: Output that looks like a pass
- **WHEN** `TestFake` prints `PASS` and `ok` and then raises `ERROR 5`
- **THEN** it is an error

#### Scenario: A failed _ASSERT
- **WHEN** `TestA` runs `_ASSERT 1 = 2, "math"` on line 9 of the test file
- **THEN** it fails with the message `math` and line 9 of the test file

### Requirement: The assertion library
The library SHALL provide `AssertTrue (cond, msg$)`, `AssertFalse`, `AssertEqualInt (expected, actual, msg$)` for
`_INTEGER64` values, `AssertEqualSingle`, `AssertEqualDouble`, `AssertNear (expected, actual, tolerance, msg$)`,
`AssertEqualStr` and `Fail (msg$)`, and `TestTempDir$`. A failed assertion SHALL record its message with expected and
actual and end the test. The library SHALL behave the same whatever the test file's `DEFxxx` and `OPTION _EXPLICIT`.

#### Scenario: Expected and actual
- **WHEN** `TestAdd` runs `AssertEqualInt 4, 2 + 3, "sum"`
- **THEN** it fails with a message naming `sum`, expected 4 and actual 5

#### Scenario: The first failure ends the test
- **WHEN** a test runs `Fail "one"` and then `Fail "two"`
- **THEN** it fails with the message `one` only

#### Scenario: Another default type
- **WHEN** a test file starts with `DEFSTR A-Z` and `OPTION _EXPLICIT`, and declares what it uses
- **THEN** it builds, and the assertions report as in a file without them

#### Scenario: A SINGLE compared as a SINGLE
- **WHEN** `x! = 0.1` and a test runs `AssertEqualSingle 0.1, x!, "x"`
- **THEN** it passes

#### Scenario: A name of the library taken
- **WHEN** the code under test has its own `SUB AssertTrue`
- **THEN** the test file fails to build with the compiler's error for a name defined twice

### Requirement: Selecting and listing tests
`--filter <text>` SHALL keep the tests whose `<file>:<SUB>` contains the text (any case). `--list` SHALL print the
tests found, with each test's file and line, and build nothing. A run, or a `--list`, that finds no test SHALL say
so and exit with status 1.

#### Scenario: A filter
- **WHEN** `qb64rust test --filter math_test.bas:TestAdd` runs
- **THEN** only `TestAdd` of `math_test.bas` is built and run

#### Scenario: Nothing found
- **WHEN** `qb64rust test empty` runs on a folder with no test file
- **THEN** it prints that no test was found and exits with status 1

### Requirement: Results and exit status
The text output SHALL print one line per test, with its result (`ok`, `FAIL`, `ERROR`, `TIMEOUT`), file, SUB and
time, then each failure's message and the test's own output, then a count of each result. `--format json` SHALL
print one JSON object per line instead, one per test found and one per result. The exit status SHALL be 0 when
every test passed and 1 otherwise.

#### Scenario: All pass
- **WHEN** every test passes
- **THEN** the last line counts them as passed and the exit status is 0

#### Scenario: One fails
- **WHEN** one test of three fails
- **THEN** its line starts with `FAIL`, its message and output follow, and the exit status is 1

#### Scenario: JSON lines
- **WHEN** `qb64rust test --format json` runs a file with one passing test
- **THEN** every line of the output is a JSON object; one names the test with its file and line, and one gives its
  result `pass` and its time

### Requirement: Build options of a test run
`qb64rust test` SHALL take the options of a compile that choose roots and fixes (`--toolchain-root`,
`--qb64pe-root`, `--libqb-root`, `--include-root`, `--fixes`, `--fix`) with their meaning in a compile, and SHALL
reject the others (`-o`, `--dump`, `-y`) as a usage error with exit status 1.

#### Scenario: A rejected option
- **WHEN** `qb64rust test -o x.exe lib` runs
- **THEN** it is a usage error with exit status 1

#### Scenario: A fix on the command line
- **WHEN** `qb64rust test --fix whole-array-value=on lib` runs
- **THEN** each test program is built with that fix on
