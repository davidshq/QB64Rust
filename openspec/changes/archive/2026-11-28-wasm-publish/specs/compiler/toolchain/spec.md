# Spec Delta

## MODIFIED Requirements

### Requirement: The Emscripten SDK
A build with `--target wasm` SHALL take the Emscripten SDK from `--emsdk-root <dir>`, else from
`QB64RUST_EMSDK_ROOT`, else the one `qb64rust toolchain install --target wasm` installed. The folder is an
`emsdk` folder with one release installed and activated. With none of the three, or with a named folder that
holds no such SDK, the build SHALL fail with status 1 and a message that names the option, the variable, the
pinned release and `qb64rust toolchain install --target wasm`. A build SHALL download
nothing. Of the host's C++ toolchain a wasm build SHALL use `make` only.

#### Scenario: The SDK by variable
- **WHEN** `QB64RUST_EMSDK_ROOT` names an `emsdk` folder with the pinned release and a wasm build runs
- **THEN** the build runs that SDK's compiler and its Node is not required on `PATH`

#### Scenario: The installed SDK
- **WHEN** neither the option nor the variable is set, the SDK was installed by `qb64rust toolchain install
  --target wasm`, and a wasm build runs on a machine with no Python and no Node on `PATH`
- **THEN** the build succeeds with the installed SDK

#### Scenario: A named folder comes first
- **WHEN** the SDK is installed and `--emsdk-root` names a folder that holds no SDK
- **THEN** the build fails with status 1 and names that folder; it does not fall back to the installed SDK

#### Scenario: No SDK
- **WHEN** a wasm build runs with neither the option nor the variable set and no SDK installed
- **THEN** the exit status is 1 and the message names `--emsdk-root`, `QB64RUST_EMSDK_ROOT` and
  `qb64rust toolchain install --target wasm`

#### Scenario: No make
- **WHEN** a wasm build runs on Windows with the SDK found and no host toolchain
- **THEN** the exit status is 1 and the message is that of "No toolchain found", since `make` comes from it

### Requirement: The wasm path of the toolchain subcommand
`qb64rust toolchain path --target wasm` SHALL print the SDK folder a wasm build would use and where it came
from (option, variable or installed), and exit with status 0. With none found it SHALL print the message of
"The Emscripten SDK" and exit with status 1. `qb64rust toolchain install --target wasm` SHALL install the SDK
as "The Emscripten SDK is installed on Windows" says.

#### Scenario: Found by variable
- **WHEN** `QB64RUST_EMSDK_ROOT` names a good SDK folder and `qb64rust toolchain path --target wasm` runs
- **THEN** it prints the folder and that it came from the variable, and exits with status 0

#### Scenario: Found installed
- **WHEN** only the installed SDK exists and `qb64rust toolchain path --target wasm` runs
- **THEN** it prints that folder, says it is the installed SDK, and exits with status 0

#### Scenario: Install is by hand for now
- **WHEN** `qb64rust toolchain install --target wasm` runs on Linux with no SDK named
- **THEN** nothing is downloaded, the `emsdk` commands are printed, and the exit status is 1

## ADDED Requirements

### Requirement: The Emscripten SDK is installed on Windows
On Windows, `qb64rust toolchain install --target wasm` SHALL download the pinned Emscripten release with the
Node and the Python it runs on, check each archive's SHA-256, and unpack them into `emsdk-7.0.0` under the
toolchain's home, ready for a build. It SHALL first print the size of the download and of the folder, and
report progress. Running the command is the user's consent; nothing else installs.

#### Scenario: Install
- **WHEN** `qb64rust toolchain install --target wasm` runs on Windows with network access and no SDK installed
- **THEN** it exits with status 0, prints the installed folder, and a following wasm build finds the SDK there

#### Scenario: Already installed
- **WHEN** the pinned SDK is already installed and complete
- **THEN** the command downloads nothing, prints the folder and exits with status 0

#### Scenario: A wrong checksum
- **WHEN** one of the archives has another SHA-256 than the pinned one
- **THEN** the command exits with status 1 with both sums and leaves nothing behind

### Requirement: An SDK install that does not finish
An install of the Emscripten SDK that was interrupted, or whose download failed, SHALL leave no folder that a
build finds. A failed download SHALL exit with status 1 with the reason and the `emsdk` commands that install
the release by hand. Two installs at once SHALL give one install: the second waits and reports the folder.

#### Scenario: Interrupted
- **WHEN** an install was interrupted during the second archive
- **THEN** no build finds a partial SDK, and the next install installs it whole

#### Scenario: Offline
- **WHEN** the download fails
- **THEN** the exit status is 1, and the message gives the reason, `emsdk install 7.0.0`, `emsdk activate
  7.0.0` and `--emsdk-root`

### Requirement: The Emscripten SDK on Linux is installed by hand
On Linux, `qb64rust toolchain install --target wasm` SHALL download nothing. With an SDK found it SHALL print
its folder and exit with status 0. Otherwise it SHALL print the `emsdk` commands that install the pinned
release and exit with status 1.

#### Scenario: Linux without an SDK
- **WHEN** `qb64rust toolchain install --target wasm` runs on Linux with no SDK named
- **THEN** nothing is downloaded, `emsdk install 7.0.0` is printed, and the exit status is 1
