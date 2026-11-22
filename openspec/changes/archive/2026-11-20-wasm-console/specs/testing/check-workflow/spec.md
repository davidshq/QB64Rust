# Spec Delta

## ADDED Requirements

### Requirement: The wasm steps
`check` SHALL have two steps for the wasm target. `wasm-slice` runs `tests\wasm\slice.list` with
`--platform wasm` on the host, in the `full` and `merge` layers. `wasm-verification` runs the verification
suite with `--platform wasm` in the suites VM, in no layer: it runs only when named (user, `DECISIONS.md`
2026-11-20). Their inputs are the release build's sources, libqb's copy and edits, the runner and the lists.

#### Scenario: The whole family goes to the VM
- **WHEN** `check wasm-verification --where vm` runs
- **THEN** the suite runs in the suites VM with the SDK found there, and the step's record says `vm`

#### Scenario: Not in the quick layer
- **WHEN** `just quick` plans its steps after a change to a crate
- **THEN** neither wasm step is planned

#### Scenario: The whole family only when named
- **WHEN** `just full` or `just merge` plans its steps after a change to libqb's copy
- **THEN** `wasm-slice` is planned and `wasm-verification` is not

### Requirement: A wasm step without the SDK
When the Emscripten SDK is not found where a wasm step would run, the step SHALL be reported as not run, with
the reason in one line, and SHALL NOT fail the run. In a `merge` run it SHALL fail.

#### Scenario: A full run on a machine without the SDK
- **WHEN** `just full` runs and `QB64RUST_EMSDK_ROOT` is not set
- **THEN** the report lists `wasm-slice` as not run, with the reason, and the run's result does not depend
  on it

#### Scenario: A merge without the SDK
- **WHEN** `just merge` runs and the SDK is not found
- **THEN** `wasm-slice` fails with the reason
