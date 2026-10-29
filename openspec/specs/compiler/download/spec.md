# compiler/download Specification

## Purpose

The compiler as a download of its own, apart from the VS Code extension: what the archive is called, what it holds
and how it is laid out, the licence notices it carries, its manual, and that it works unpacked into any folder the
user can write to.

## Requirements

### Requirement: Download contents
Each release SHALL offer `qb64rust-win-x64-<version>.zip` and `qb64rust-linux-x64-<version>.tar.gz`. Each SHALL
hold one top folder `qb64rust-<version>` containing the binary built in release mode (`qb64rust.exe` in the zip,
`qb64rust` with its execute permission in the tar), `libqb` (the repository's tracked libqb files, no built objects),
`LICENSE.txt`, `THIRD-PARTY-NOTICES.txt` (the licences of the Rust crates linked into that binary) and `MANUAL.md`.
Neither SHALL hold a C++ toolchain, object files or the extension.

#### Scenario: Listing the download
- **WHEN** the files of `qb64rust-win-x64-0.2.0.zip` are listed
- **THEN** each is under `qb64rust-0.2.0/`, they include `qb64rust.exe`, `libqb/internal/c/qbx.cpp`,
  `libqb/COPYING.txt`, `LICENSE.txt`, `THIRD-PARTY-NOTICES.txt` and `MANUAL.md`, and none ends in `.o` or `.a` or
  lies under a `c_compiler` folder

#### Scenario: A file is missing
- **WHEN** the release makes a zip without `libqb/COPYING.txt`, or with a licence notice whose text differs from
  the repository's
- **THEN** the check fails, naming the file, and nothing is published

#### Scenario: Listing the Linux download
- **WHEN** the files of `qb64rust-linux-x64-0.4.0.tar.gz` are listed
- **THEN** each is under `qb64rust-0.4.0/`, they include `qb64rust` (mode with execute bits),
  `libqb/internal/c/qbx.cpp`, `libqb/COPYING.txt`, `LICENSE.txt`, `THIRD-PARTY-NOTICES.txt` and `MANUAL.md`, and
  none ends in `.o`, `.a` or `.exe`

### Requirement: Usable where it is unpacked
The unpacked folder SHALL work with no option, environment variable or installation step beyond
`qb64rust toolchain install` on Windows, or the system packages on Linux (spec `compiler/toolchain`, "The system
packages"): the binary SHALL use the `libqb` folder beside it, and included files are looked up from that folder as
for any `qb64rust` binary (spec `compiler/cli`, "Compiler root for included files").

#### Scenario: First use
- **WHEN** the zip is unpacked into a folder the user can write to on a machine without QB64pe or this repository,
  `qb64rust toolchain install` has run, and `qb64rust.exe -x hello.bas -o hello.exe` is run in a command prompt
- **THEN** `hello.exe` is built, libqb's objects are built in the unpacked `libqb` folder, and `hello.exe` runs

#### Scenario: Before the toolchain is installed
- **WHEN** the same command is run before `qb64rust toolchain install`
- **THEN** it fails with the message of spec `compiler/toolchain`, "No toolchain found", which names the install
  command

#### Scenario: First use on Linux
- **WHEN** the tar is unpacked with `tar -xzf` into the user's home on Ubuntu with the system packages installed,
  and `./qb64rust -x hello.bas` is run
- **THEN** `hello` is built, libqb's objects are built in the unpacked `libqb` folder, and `./hello` runs

### Requirement: The manual
The download's `MANUAL.md` SHALL describe: what the download is and the systems it needs (the Windows versions;
Linux x64 with glibc and the packages per distribution); unpacking it and installing the toolchain or the packages;
compiling, running and formatting a program; the options of the usage text; the exit statuses; where the compiler
and its programs differ from QB64pe 4.7.0 (`QB64PE-DIVERGENCES.md`); and where to report a problem. Every option it
names SHALL be one the usage text names.

#### Scenario: An option in the manual
- **WHEN** the manual names an option or subcommand, such as `--include-root` or `toolchain path`
- **THEN** `qb64rust --help` names it too

#### Scenario: The Linux packages in the manual
- **WHEN** the manual's Linux section names the packages for Ubuntu
- **THEN** they are the packages the message of "No toolchain found" names on Ubuntu
