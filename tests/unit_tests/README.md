# Fixtures of `qb64rust test`

Test files for the CLI tests of `qb64rust test` (`crates\driver\tests\unit_tests.rs`, one test per scenario of the
spec `compiler/unit-tests`; OpenSpec change `basic-unit-tests`). Several fail, error or time out on purpose. Run
them by hand from this folder: `qb64rust test <folder>`.

## What a test file is

- A **test file** is a `.bas` file whose name ends in `_test.bas` (any case), found in a folder named on the command
  line or below it; a `.bas` file named on the command line is taken whatever its name.
- A **test** is a `SUB` of the test file itself (not of a file it includes) whose name starts with `Test` (any
  case) and that takes no parameters. `qb64rust test --list` names a SUB that takes parameters as "not a test".
- The test file's **module-level code** runs before each test, in a process of its own: it is each test's setup.
- The **assertions**: `AssertTrue cond, msg$`, `AssertFalse`, `AssertEqualInt expected, actual, msg$`
  (`_INTEGER64`), `AssertEqualSingle`, `AssertEqualDouble`, `AssertNear expected, actual, tolerance, msg$`,
  `AssertEqualStr`, `Fail msg$`, and `TestTempDir$`, a folder of the test's own. The first failed assertion ends the
  test. `_ASSERT` works too and gives its line.
- Tests are for **console-only code**: the program is built `$CONSOLE:ONLY`. Images work (`_NEWIMAGE`, `_DEST` and
  `_SOURCE` of an image, `PSET`, `_RGB32` and `POINT` into it, `_PUTIMAGE`, `PRINT` into an image, `_FREEIMAGE`);
  drawing on the console itself (`PSET` or `CIRCLE` with no image as the destination) is error 5, "Illegal
  function call", reported as ERROR (`graphics\image_test.bas`, `TestDrawOnConsole`).

## The fixtures

| Folder | What it shows |
|---|---|
| `lib\` | `math_test.bas` includes `math.bm` from its own folder: two tests; `TestWith` takes parameters, `TestF` is a FUNCTION, `math.bm`'s `TestHelper` and the comment `' SUB TestOld` are not tests. `helpers.bas` is not a test file by its name |
| `judge\` | How a test is judged: a pass, a runtime error, `SYSTEM` in the code under test, output that looks like a pass, a failed `_ASSERT` on line 9, a failed assertion, two `Fail`s, a `SINGLE`, `LINE INPUT` at the end of the null device |
| `loop\` | A test that never returns (run with `--timeout 2`) beside one that does |
| `setup\` | Module-level code run before each test |
| `deftype\` | `DEFSTR A-Z`, `OPTION _EXPLICIT` and `OPTION BASE 1` in the test file |
| `temp\` | Two tests writing the same name into `TestTempDir$` at once |
| `clash\` | The code under test defines `AssertTrue`: a build error |
| `build_error\` | A compile error on line 4 of `bad_test.bas`; `good_test.bas` still runs |
| `fixes\toml\` | `qb64rust.toml` turns `resume-after-block` on for the test file |
| `fixes\cmdline\` | `PRINT a()`, a compile error with `--fix whole-array-value=on` |
| `graphics\` | Which image functions work under `$CONSOLE:ONLY`, and drawing on the console, an error |

Nothing is written into these folders: a run builds into `<temp>\qb64rust-test\<hash of the test file's path>\<process
id>\`, removed when all the file's tests passed and kept otherwise, with each failed test's own folder named in the
output.
