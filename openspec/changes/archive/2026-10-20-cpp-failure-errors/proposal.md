# Proposal: cpp-failure-errors

## Why

Six verification programs fail in the C++ build of both compilers, and the new compiler prints nothing more than
the toolchain's log: an `$EXEICON` file that is no icon (`verification\v29_e16_not_an_icon`), and five
`DECLARE LIBRARY` forms whose C++ fails (`v37_x04`, `x17`, `x23`, `x30`, `x31`). The user's ruling of 2026-10-19
(`DECISIONS.md`, the first sorting of the first full compare) is that the new compiler should say why, as an error
in the program; `STATUS.md` lists it as the one known bug, and `tests\verification\known.list` holds the six with
the reason `bug:`.

Only the icon can be checked before the C++ without reading C. The library programs need what a header declares
(`x23`, `x30`, `x31`: the name is not declared; `x31`: in another letter case), the type of a header's parameter
(`x17`: `5` to a `const void*`) or the symbols libqb and the C runtime hold (`x04`: `f` is defined nowhere). A
search of a header's text could give a false error (a name made by a macro, or declared in a file the header
includes), which the project never accepts. So the user chose (2026-10-20): the icon before the C++, the library
programs from the C++ build's own messages, mapped back to the BASIC line.

## What Changes

- **The icon is checked before the C++.** A `$EXEICON` file that is there but does not start with an icon
  directory (reserved 0, type 1, at least one image, each image's bytes inside the file) is an error at the line,
  as a missing file is: in the CLI, `-z` and the language server alike.
- **A C++ build that fails on a library function is an error in the program.** When the C++ build fails and every
  error of clang and of the linker names the C function of a `DECLARE LIBRARY` declaration (not `DYNAMIC`), each
  becomes an error of the program: a compile error at the BASIC line clang names (the call), with clang's message
  and the note that explains it (`no known conversion from 'long long' to 'const void *'`), and an undefined
  symbol at the declaration. They are printed as the front end's errors are, with the summary line, and the exit
  status is 1. A C++ failure with any other error is reported as today, since it may be the compiler's own fault.
- The six programs move from `known.list` to `pass.list`; the known bug leaves `STATUS.md`.

The check on save (language server, `-z`) gets the icon error; it builds no C++, so it cannot get the library
errors.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/console-and-window`: "Errors of $EXEICON" adds a file that is no icon.
- `language/external-libraries`: a new requirement, "Errors the C++ build finds": a failure of the C++ build that
  names a library function is an error at the call or the declaration.

## Impact

- Changed: `crates\driver\src\lib.rs` (`check_exe_icon`), `crates\driver\src\build.rs` (the failure keeps make's
  output for the mapping), a new module `crates\driver\src\cpp_errors.rs` (reading clang's and the linker's
  messages), `crates\driver\src\main.rs` (the mapped errors printed with the summary).
- Tests: a front-end test of the icon error; unit tests of the reading of the toolchain's messages; the six
  verification programs on `pass.list`.
- Unchanged: generated C++, the runner (its rule for a rejection already takes the summary line), the extension.
