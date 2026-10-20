# Design

## Context

`check_exe_icon` (`crates\driver\src\lib.rs`) is the front end's only check that reads the icon: it reports a
missing file. `check_program` runs it for the CLI, `-z` and the language server alike. A file that is there goes
to `build::write_resources`, and `llvm-rc` rejects a file that is no icon ("Incorrect icon/cursor Reserved field;
should be 0", `v29_e16_not_an_icon`).

`build::build` (`crates\driver\src\build.rs`) returns `Err(String)` when `make` fails: "C++ build failed (make exit
status …); build folder kept: …" and make's stdout and stderr. `main.rs` prints it as `qb64rust: …` with exit
status 1 and no summary line, which the runner reads as "not an error in the program" (`new_compiler_rejection` in
`run_legacy_tests.py`).

The C++ carries a `#line` directive before each statement, naming the BASIC file as the source map does
(`names::line_name`), so clang names the BASIC file and line of a failing call. Measured on the five programs
(2026-10-20, the clone's clang 23 and `ld.lld`):

| Program | The toolchain's error |
|---|---|
| `v37_x30` | `v37_x30.bas:6:46: error: use of undeclared identifier 'h_none'` |
| `v37_x31` | `v37_x31.bas:6:46: error: use of undeclared identifier 'H_L'; did you mean 'HKL'?` |
| `v37_x23` | `v37_x23.bas:6:48: error: use of undeclared identifier 'c_half'; did you mean 'GLhalf'?` |
| `v37_x17` | `v37_x17.bas:6:46: error: no matching function for call to 'h_isnull'`, then `…/v37h.h:43:9: note: candidate function not viable: no known conversion from 'long long' to 'const void *' for 1st argument` |
| `v37_x04` | `ld.lld: error: undefined symbol: f`, then `>>> referenced by …qbx.o:(QBMAIN(void*))` and `clang-23: error: linker command failed with exit code 1` |

The column is that of the C++ line, not of the BASIC line. Each library procedure's C name is
`Proc::external.c_name` (the `ALIAS` text, or the name as written), and the symbol table holds its declaration
and each use with their spans.

## Goals / Non-Goals

**Goals:**

- The six programs fail with a summary line, so the runner counts them as rejected, and with a message that says
  why at a BASIC line.
- No false error. A C++ failure that cannot be traced to a library declaration stays the toolchain's output.

**Non-Goals:**

- Finding these errors before the C++ (the user's ruling, 2026-10-20: a search of header text could give a false
  error), and so in the language server's check.
- Mapping C++ errors of the compiler's own code. Those are the compiler's bugs, and they stay visible as such.
- Rewording clang's messages. They are passed on as clang writes them, without the guesses `; did you mean …`,
  which name libqb's own identifiers.

## Decisions

### D1. The icon is checked by its directory

The file is read and checked as an icon directory (`ICONDIR`): at least 6 bytes, reserved word 0, type 1, a count
of at least 1, then that many 16-byte entries inside the file, each with its image's offset and size inside the
file. This is what `llvm-rc` needs to read the images. No more is checked (not the bitmap or PNG inside), so a
file that passes here and fails in `llvm-rc` still fails in the C++ as it does today. The error is at the
metacommand's span, like the missing file: "the file `<name as written>` of `$EXEICON` is not an icon file". The
repo's icons (`cws.ico` of five images, the test icons of one) start `00 00 01 00`, and the real-program test
holds `cws.ico` to no diagnostic.

Code: `crates\driver\src\lib.rs` (`check_exe_icon`, and a pure `is_icon(&[u8]) -> bool` beside it with its unit
tests).

### D2. A failed build keeps the toolchain's output apart

`build::build` returns `Result<(), BuildError>`: `BuildError::Toolchain { output, folder }` when `make` ran and
failed (stdout then stderr, as now), `BuildError::Other(String)` for everything else (a file it cannot write, a
lock, a rename). `Display` for `Toolchain` prints today's text, so where nothing is mapped the user sees what they
see now.

Code: `crates\driver\src\build.rs` (`build`, `build_with_make`, the cache path that calls it).

### D3. Reading the toolchain's messages

A new module `crates\driver\src\cpp_errors.rs` reads the output line by line into a list of failures, each
`Compile { file, line, name, message }` or `Link { name }`, or gives up (`None`) at the first error it cannot read:

- A compile error is a line `<file>:<line>:<col>: error: <message>` where `<file>` is the name of one of the
  program's files exactly as the `#line` directives wrote it (the source map's names, the longest first), the line
  a number. The message is cut at `; did you mean`. The `note: candidate …` lines that follow it, clang's reason
  an overload was not taken, add their messages after `; `; other notes (where a guessed name is declared) are
  left. `name` is the first identifier in single quotes in the message.
- An undefined symbol is `ld.lld: error: undefined symbol: <sym>` or GNU ld's ``undefined reference to `<sym>'``;
  `name` is `<sym>` up to a `(` (lld prints a C++ symbol demangled, `f(int)`).
- Lines that only follow from an error are skipped: `In file included from`, the code and caret lines, `>>>`
  lines, `N errors generated.`, the compiler driver's `error: linker command failed`, and make's own lines; so
  are warnings and the commands make echoes, whose first word is the tool (`…\bin\c++.exe`; the link command
  names `error_handle.o`, found on `v37_x04`).
- Any other line with `error` in any letter case (an error in `qbx.cpp`, in a header, `llvm-rc`'s `Error in`, a
  `fatal error:`) gives up.

Then each failure is matched with a library procedure: a declaration in a block that is not `DYNAMIC` whose C name
equals `name`, or ends with `::<name>`, or with `<name>` before a `<` (a template). A failure with no match gives
up. Giving up anywhere keeps `BuildError::Toolchain` as it is (spec: "Another C++ failure").

Alternative: map every compile error at a BASIC line, whatever it names. Rejected: an error in the code this
compiler writes would then read as the program's fault, and the corpus would count it as a rejection.

Code: `crates\driver\src\cpp_errors.rs` (`read`, `library_proc`, `diagnostics`), its unit tests on the measured texts above
with the paths made relative (`CLAUDE.md` rule 2).

### D4. Where the errors stand and how they read

- A compile error stands at the procedure's first use on that line of that file (the symbol table's uses), or at
  the start of the line if no use is found there (a use the table did not record). It reads "the C++ compiler
  rejects the call of `<BASIC name>` (`<C name>` of `DECLARE LIBRARY`): <message>".
- An undefined symbol stands at the declaration's name and reads "`<BASIC name>` is defined in no library or
  header the program uses (the linker finds no `<C name>`)".

They are added to the front end's diagnostics and printed by the same code as its errors, then the summary line;
the build folder is removed as after a build that worked (kept with `--keep-build`), since the error is the
program's. Exit status 1.

Code: `crates\driver\src\main.rs` (`run`, after `build::build`), `crates\driver\src\cpp_errors.rs` (`diagnostics`).

## Risks / Trade-offs

- [The toolchain's wording changes with a new clang or lld] → an unread line gives up, so the worst case is
  today's output; the unit tests hold the measured texts, and the six verification programs on `pass.list` show
  a change at the next run.
- [A bug of this compiler in a library call (a wrong argument type) reads as the program's error] → only where
  clang names the library function; the message carries clang's own text and its note, and every library call of
  the corpus, upstream and verification suites builds today.
- [The check on save shows none of the library errors] → accepted (Non-Goals); a build shows them at the line.
