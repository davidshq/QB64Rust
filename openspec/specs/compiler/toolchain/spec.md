# compiler/toolchain Specification

## Purpose

Which C++ toolchain (compiler, linker, `make`, `lldb-dap`) a build and the debug adapter of `qb64rust` use, where
it is looked for, and how `qb64rust toolchain install` puts the pinned release in place, only when asked.

## Requirements

### Requirement: The pinned toolchain
The toolchain `qb64rust` installs SHALL be the upstream release `llvm-mingw-20260922-ucrt-x86_64.zip` of
`github.com/mstorsjo/llvm-mingw`, accepted only when its SHA-256 is
`e3ad77d117a4bea19a7a3b333341824d79a5a371004a10e25b8504e7b3047666`. Changing the release is a change of its own,
judged by the full test set.

#### Scenario: Wrong checksum
- **WHEN** the downloaded file's SHA-256 differs from the pinned one
- **THEN** the install fails with a message giving both checksums, the file is deleted, and no toolchain folder is
  left behind

### Requirement: Toolchain lookup
A build and `qb64rust dap` SHALL take the toolchain from `--toolchain-root`, else `QB64RUST_TOOLCHAIN_ROOT`, else
the QB64pe folder given by `--qb64pe-root` or `QB64RUST_QB64PE_ROOT` (its `internal\c\c_compiler`), else the
installed toolchain, else the default clone `..\QB64pe` next to the repository the binary was built from. A
candidate counts only if it holds the compiler driver the build runs.

#### Scenario: Installed toolchain without QB64pe
- **WHEN** the toolchain is installed, no option or variable is set, and no clone exists next to the repository
- **THEN** a program builds and runs, and no QB64pe folder is read

#### Scenario: Explicit root wins
- **WHEN** `--toolchain-root <dir>` names a toolchain folder and an installed toolchain exists too
- **THEN** the build runs the compiler under `<dir>`

#### Scenario: A named QB64pe folder wins over the installed toolchain
- **WHEN** `--qb64pe-root <q>` is given and an installed toolchain exists
- **THEN** the build uses `<q>\internal\c\c_compiler`

#### Scenario: A named folder that is not QB64pe's
- **WHEN** `--qb64pe-root` or `QB64RUST_QB64PE_ROOT` names a folder without `internal\c\qbx.cpp`
- **THEN** `qb64rust toolchain path` and `qb64rust dap` fail with the message a build gives, naming the option and
  the variable, even when a toolchain is installed

### Requirement: No toolchain found
When a build or the debug adapter needs a toolchain and none is found, `qb64rust` SHALL fail with exit status 1
and a message that names `qb64rust toolchain install` with the download's size and source, `--toolchain-root` and
`QB64RUST_TOOLCHAIN_ROOT`. It SHALL NOT download anything.

#### Scenario: Fresh machine
- **WHEN** a program is compiled to an executable on a machine with no toolchain at any place of the lookup
- **THEN** compilation fails with exit status 1, the message names `qb64rust toolchain install`, and no network
  access is made

#### Scenario: Checking needs no toolchain
- **WHEN** a program is checked (`-z` without an executable, the language server, `-y`) with no toolchain present
- **THEN** it is checked as before

### Requirement: Toolchain install
`qb64rust toolchain install` SHALL download the pinned release, check its checksum, unpack it into
`%LOCALAPPDATA%\qb64rust\toolchain\llvm-mingw-20260922-ucrt-x86_64` without the folders of the other three target
architectures, and print the folder. It SHALL report progress on standard output. Running the command is the
user's consent; nothing else installs.

#### Scenario: Install
- **WHEN** `qb64rust toolchain install` runs with network access and no toolchain installed
- **THEN** it exits with status 0, prints the installed folder, and a following build finds the toolchain there

#### Scenario: Already installed
- **WHEN** the pinned toolchain is already installed and complete
- **THEN** the command downloads nothing, prints the folder and exits with status 0

#### Scenario: Interrupted install
- **WHEN** an install was interrupted during the download or the unpacking
- **THEN** no build finds a partial toolchain, and the next `qb64rust toolchain install` installs it whole

#### Scenario: Two installs at once
- **WHEN** two `qb64rust toolchain install` commands run at the same time
- **THEN** one installs, the other waits for it and then reports the installed folder

#### Scenario: Offline
- **WHEN** the download fails (no network, proxy refused, HTTP error)
- **THEN** the command exits with status 1 with the reason, the release URL to fetch by hand, and
  `--toolchain-root`, and leaves nothing behind

### Requirement: Toolchain path
`qb64rust toolchain path` SHALL print the toolchain folder a build would use and where it came from (option,
variable, QB64pe folder, installed, default clone), and exit with status 0; with none found it SHALL print the
message of "No toolchain found" and exit with status 1.

#### Scenario: Installed
- **WHEN** only the installed toolchain exists
- **THEN** it prints that folder, says it is the installed toolchain, and exits with status 0
