# Spec Delta

## MODIFIED Requirements

### Requirement: Locating the reference runtime
The QB64pe folder, relative to which `DECLARE LIBRARY` looks for files and the extra link flags name files, SHALL
be taken from `--qb64pe-root`, else from the environment variable `QB64RUST_QB64PE_ROOT`, else from `..\QB64pe`
next to the repository the binary was built from; when none of these exists, the include root takes its place,
and the build goes on. The C++ toolchain is found as spec `compiler/toolchain` says. The copy of libqb SHALL be
taken from `--libqb-root`, else from the environment variable `QB64RUST_LIBQB_ROOT`, else from `libqb` in the
folder of the `qb64rust` executable, else from `libqb` in the repository the binary was built from; it is needed
only to build an executable. If the copy is needed and not found, the compiler SHALL fail with a message naming
its option and its variable. If libqb's objects must be built and the copy's folder cannot be written to, the
compiler SHALL fail with a message naming that folder and `--libqb-root`.

#### Scenario: No clone found
- **WHEN** no QB64pe folder exists at any of the three places, a toolchain is found, and a program without
  `DECLARE LIBRARY` is compiled to an executable
- **THEN** the executable is built

#### Scenario: A library without a clone
- **WHEN** no QB64pe folder exists, and a program at `p\prog.bas` compiled with `--include-root r` declares
  `DECLARE LIBRARY "lib/x"` with `r\lib\x.h` present
- **THEN** the header is found under the include root, as it would be under QB64pe's folder

#### Scenario: No copy of libqb found
- **WHEN** a program is compiled to an executable, a toolchain is found, and no copy of libqb exists at any of
  the four places
- **THEN** compilation fails with exit status 1 and a message naming `--libqb-root` and `QB64RUST_LIBQB_ROOT`

#### Scenario: libqb beside the executable
- **WHEN** `qb64rust.exe` is in a folder that also holds `libqb\internal\c\qbx.cpp`, and neither `--libqb-root`
  nor `QB64RUST_LIBQB_ROOT` is given
- **THEN** a build uses that `libqb` folder, not the repository's

#### Scenario: The option still wins
- **WHEN** `qb64rust.exe` has a `libqb` folder beside it and `--libqb-root other` names another copy
- **THEN** the build uses `other`

#### Scenario: A copy that cannot be written to
- **WHEN** a build must build libqb's objects and writing into the copy's folder fails
- **THEN** compilation fails with exit status 1 and a message naming the folder, saying libqb's objects are built
  there, and naming `--libqb-root` for a copy in a folder that can be written to

## ADDED Requirements

### Requirement: Version and help
`qb64rust --version` SHALL print `qb64rust <version>`, where `<version>` is the release's version (the one in the
names of its `.vsix` and its download), and exit with status 0. `qb64rust --help` and `qb64rust -h` SHALL print
the usage text and exit with status 0. Each SHALL be the only argument; given with others, it is a usage error with
status 1.

#### Scenario: Version
- **WHEN** `qb64rust --version` is run by the binary of release 0.2.0
- **THEN** it prints `qb64rust 0.2.0` on one line and exits with status 0

#### Scenario: Help
- **WHEN** `qb64rust --help` is run
- **THEN** it prints the usage text, which names `toolchain install`, and exits with status 0

#### Scenario: With other arguments
- **WHEN** `qb64rust --version p.bas` is run
- **THEN** it prints that `--version` takes no other argument, with the usage, and exits with status 1
