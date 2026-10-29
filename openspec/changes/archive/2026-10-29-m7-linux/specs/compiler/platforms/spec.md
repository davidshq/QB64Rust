# Spec Delta

## Purpose

The platforms `qb64rust` builds programs for (Windows x64, Linux x64), how a build's platform is chosen, and what
follows from it: the executable's name, the C++ the old compiler writes differently per platform, and whose
behaviour a program has on each.

## ADDED Requirements

### Requirement: Supported platforms
`qb64rust` SHALL build, check, format, serve the editor and debug on Windows x64 and on Linux x64 (glibc). A build
SHALL target the platform `qb64rust` runs on. There is no cross-compilation. On any other platform, building SHALL
fail with a message naming the platforms supported. Checking (`-z`, `--dump`, `lsp`, `-y`) SHALL still work there.

#### Scenario: Build on Linux
- **WHEN** `qb64rust -x hello.bas` runs on Linux x64 with the system toolchain present
- **THEN** an ELF executable `hello` is written beside the source, and it runs

#### Scenario: Checking needs no supported platform
- **WHEN** `qb64rust lsp` runs on a platform that is not supported
- **THEN** the language server works as on Windows

### Requirement: The C++ follows the platform
The C++ `qb64rust` writes for a program SHALL be what the old compiler writes on the build's platform. Where
`qb64pe.bas` writes something else with `os$ = "LNX"` (dynamic libraries, library link names, quoting of file
names), a Linux build SHALL write the Linux form. Everything else SHALL be the same on both platforms.

#### Scenario: Same program, two platforms
- **WHEN** a program without `DECLARE LIBRARY` is compiled with `-z` on Windows and on Linux
- **THEN** the C++ fragments are equal byte for byte

#### Scenario: A dynamic library
- **WHEN** a program with `DECLARE DYNAMIC LIBRARY` is compiled with `-z` on Linux
- **THEN** its fragments call `dlopen` and `dlsym` and hold no `LoadLibrary`, `GetProcAddress` or `CALLBACK`

### Requirement: A program behaves as QB64pe's on its platform
A program built on Linux SHALL behave as the same program built by QB64pe 4.7.0 at the reference commit on Linux,
apart from the rows of `QB64PE-DIVERGENCES.md`. Where QB64pe's Linux and Windows builds differ, the Linux build
SHALL follow QB64pe's Linux build. `_OS$` SHALL report `[LINUX][64BIT]`.

#### Scenario: Comma zones on the Linux console
- **WHEN** a `$CONSOLE:ONLY` program prints `1, 2` on Linux
- **THEN** the items are separated as QB64pe's Linux build separates them (D-011), not in 10-column zones

#### Scenario: The platform's name
- **WHEN** a program prints `_OS$` on Linux x64
- **THEN** it prints `[LINUX][64BIT]`
