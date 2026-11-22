# Spec Delta

## ADDED Requirements

### Requirement: The Emscripten SDK
A build with `--target wasm` SHALL take the Emscripten SDK from `--emsdk-root <dir>`, else from
`QB64RUST_EMSDK_ROOT`. The folder is an `emsdk` folder with one release installed and activated. With neither
set, or with a folder that holds no such SDK, the build SHALL fail with status 1 and a message that names the
option, the variable, the pinned release and the two `emsdk` commands that install it. It SHALL download
nothing. Of the host's C++ toolchain a wasm build SHALL use `make` only.

#### Scenario: The SDK by variable
- **WHEN** `QB64RUST_EMSDK_ROOT` names an `emsdk` folder with the pinned release and a wasm build runs
- **THEN** the build runs that SDK's compiler and its Node is not required on `PATH`

#### Scenario: No SDK
- **WHEN** a wasm build runs with neither the option nor the variable set
- **THEN** the exit status is 1 and the message names `--emsdk-root`, `QB64RUST_EMSDK_ROOT` and
  `emsdk install 7.0.0`

#### Scenario: No make
- **WHEN** a wasm build runs on Windows with the SDK found and no host toolchain
- **THEN** the exit status is 1 and the message is that of "No toolchain found", since `make` comes from it

### Requirement: The pinned Emscripten release
A wasm build SHALL accept only Emscripten 7.0.0. With another release in the SDK folder it SHALL fail with
status 1 and a message that gives the release found and the release needed. Changing the release is a change
of its own, judged by the whole wasm family.

#### Scenario: Another release
- **WHEN** the SDK folder holds Emscripten 6.0.13
- **THEN** the build fails, and the message names 6.0.13 and 7.0.0

### Requirement: The wasm path of the toolchain subcommand
`qb64rust toolchain path --target wasm` SHALL print the SDK folder a wasm build would use and where it came
from (option or variable), and exit with status 0. With none found it SHALL print the message of "The
Emscripten SDK" and exit with status 1. `qb64rust toolchain install --target wasm` SHALL print the same
message, install nothing and exit with status 1.

#### Scenario: Found by variable
- **WHEN** `QB64RUST_EMSDK_ROOT` names a good SDK folder and `qb64rust toolchain path --target wasm` runs
- **THEN** it prints the folder and that it came from the variable, and exits with status 0

#### Scenario: Install is by hand for now
- **WHEN** `qb64rust toolchain install --target wasm` runs
- **THEN** nothing is downloaded, the `emsdk` commands are printed, and the exit status is 1
