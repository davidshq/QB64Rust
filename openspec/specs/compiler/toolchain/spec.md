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
On Windows, a build and `qb64rust dap` SHALL take the toolchain from `--toolchain-root`, else `QB64RUST_TOOLCHAIN_ROOT`, else
the QB64pe folder given by `--qb64pe-root` or `QB64RUST_QB64PE_ROOT` (its `internal\c\c_compiler`), else the
installed toolchain, else the default clone `..\QB64pe` next to the repository the binary was built from. A
candidate counts only if it holds the compiler driver the build runs. On Linux, the order SHALL be
`--toolchain-root`, else `QB64RUST_TOOLCHAIN_ROOT` (a folder holding `bin/g++` and `bin/make`), else the system
toolchain: `g++`, `make` and `objcopy` found on `PATH`.

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

#### Scenario: The system toolchain on Linux
- **WHEN** a program is built on Linux with no option or variable set and `g++`, `make` and `objcopy` on `PATH`
- **THEN** the build runs those programs, and no QB64pe folder and no download is used

#### Scenario: An explicit root on Linux
- **WHEN** `--toolchain-root <dir>` names a folder with `bin/g++` and `bin/make` on Linux
- **THEN** the build runs the programs under `<dir>/bin` and not those on `PATH`

### Requirement: No toolchain found
When a build or the debug adapter needs a toolchain and none is found, `qb64rust` SHALL fail with exit status 1
and a message. On Windows the message SHALL name `qb64rust toolchain install` with the download's size and source,
`--toolchain-root` and `QB64RUST_TOOLCHAIN_ROOT`. On Linux it SHALL name the programs missing and the command that
installs the packages on the distribution detected (from `/etc/os-release`: Debian or Ubuntu, Fedora, Arch), or the
list of what is needed when the distribution is not one of them. It SHALL NOT download anything.

#### Scenario: Fresh machine
- **WHEN** a program is compiled to an executable on a Windows machine with no toolchain at any place of the lookup
- **THEN** compilation fails with exit status 1, the message names `qb64rust toolchain install`, and no network
  access is made

#### Scenario: Checking needs no toolchain
- **WHEN** a program is checked (`-z` without an executable, the language server, `-y`) with no toolchain present
- **THEN** it is checked as before

#### Scenario: No g++ on Ubuntu
- **WHEN** a program is compiled to an executable on Ubuntu with no `g++` on `PATH`
- **THEN** compilation fails with exit status 1 and the message names `g++` and a `sudo apt-get install` line
  with the packages of "The system packages"

### Requirement: Toolchain install
On Windows, `qb64rust toolchain install` SHALL download the pinned release, check its checksum, unpack it into
`%LOCALAPPDATA%\qb64rust\toolchain\llvm-mingw-20260922-ucrt-x86_64` without the folders of the other three target
architectures, and print the folder. It SHALL report progress on standard output. Running the command is the
user's consent; nothing else installs. On Linux it SHALL download nothing: with the system toolchain and packages
present it SHALL say so and exit with status 0; otherwise it SHALL print the message of "No toolchain found" and
exit with status 1.

#### Scenario: Install
- **WHEN** `qb64rust toolchain install` runs on Windows with network access and no toolchain installed
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

#### Scenario: Install on Linux
- **WHEN** `qb64rust toolchain install` runs on Linux without `g++`
- **THEN** nothing is downloaded, the message names the packages to install, and the exit status is 1

### Requirement: Toolchain path
`qb64rust toolchain path` SHALL print the toolchain folder a build would use and where it came from (option,
variable, QB64pe folder, installed, default clone, or `system` on Linux, where the folder is the one holding the
`g++` found), and exit with status 0; with none found it SHALL print the message of "No toolchain found" and exit
with status 1.

#### Scenario: Installed
- **WHEN** only the installed toolchain exists
- **THEN** it prints that folder, says it is the installed toolchain, and exits with status 0

#### Scenario: System on Linux
- **WHEN** `g++` is `/usr/bin/g++` and nothing is set
- **THEN** it prints `/usr/bin` on its first line and `system` on its second, and exits with status 0

### Requirement: The system packages
On Linux, a build SHALL need the system's C++ compiler, `make`, and the development files libqb compiles and links
against (OpenGL, GLU, X11 and its extensions, ALSA). The message of "No toolchain found", and a C++ build that fails
on a missing one of those headers or libraries, SHALL name the install command and the packages for the
distribution: the packages QB64pe's `setup_lnx.sh` installs there, less those no libqb build reads.

#### Scenario: A missing header
- **WHEN** a program is built on Ubuntu with `g++` present and `libglu1-mesa-dev` missing
- **THEN** the build fails with exit status 1 and the message names the `apt-get install` line as well as the C++
  compiler's error

#### Scenario: The debugger's package
- **WHEN** `qb64rust dap` finds no `lldb-dap` on Ubuntu
- **THEN** the error names the package `lldb` and `lldb-dap` as the program looked for
