# Proposal: libqb-object-flags

## Why

The driver gives its flags to `make` as `CXXFLAGS_EXTRA` (`-fwrapv`, `-O2` for `OptimizeCppProgram`, and the copy's
`internal\c` for the headers), and QB64pe's `Makefile` adds `CXXFLAGS_EXTRA` to every object, libqb's among them.
`make` rebuilds an object by file times only, so each libqb object keeps the flags of whichever build made it
first. Which flags libqb was built with depends on the order of builds, and a copy's objects outlive the flags they
were built with. In the extension, the prepared copy is named by the version and `libqb.id`: a local package with
a new `qb64rust.exe` and the same libqb reuses objects built under the old flags, and the user's QB64pe toolchain
can change under the same copy. `STATUS.md` lists this as a known bug, to be fixed in the driver (found at the close
of `libqb-copy`, its design, "Risks").

## What Changes

- The driver's flags go to the program's own objects only (`qbx.o`, and `embedded.o` with `$EMBED`), through a
  second makefile in the build folder that gives them as a target's variable. libqb's objects are built with no
  flag of the driver, as `qb64pe.exe` builds them without `-f:` settings, whatever the build that makes them.
- A stamp at the top of the copy (`objects.stamp`, git-ignored) says what the copy's objects were built with: a
  version line and the clone's toolchain executables (size and time). A build whose stamp does not match removes
  the objects builds wrote in the copy and writes the stamp, holding the objects' lock alone when there is one;
  `make` then builds the objects it needs. The repo's `libqb\` and the extension's prepared copies are both
  covered, since the driver does it.
- The build cache's key goes to version 5 (`build_with_make`'s fixed arguments changed).

No program's output changes: generated code still takes `-fwrapv` (D-001, D-002), and libqb is the same sources.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `compiler/pipeline`: "C++ output and build" says which objects take the driver's flags and that libqb's objects
  are rebuilt when what built them changes.

## Impact

- Changed: `crates\driver\src\build.rs` (`build_with_make`, `lock_objects`, the stamp, the program makefile, the
  key's version), `.gitignore` (`/libqb/objects.stamp`).
- The first build after this change in any copy rebuilds libqb's objects it needs (once).
- Unchanged: the extension's code (the driver fixes its copies), the runner, the call-site check.
