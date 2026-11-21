## MODIFIED Requirements

### Requirement: The wasm steps
`check` SHALL have three steps for the wasm target. `wasm-slice` runs `tests\wasm\slice.list` with
`--platform wasm` on the host, in the `full` and `merge` layers. `wasm-verification` runs the verification
suite with `--platform wasm` in the suites VM, in no layer: it runs only when named (user, `DECISIONS.md`
2026-11-20). `wasm-rerun` runs the verification programs named in `tests\wasm\rerun.list` the same way, also
only when named; the list is empty in the repository. Their inputs are the release build's sources, libqb's
copy and edits, the runner and the lists.

#### Scenario: The whole family goes to the VM
- **WHEN** `check wasm-verification --where vm` runs
- **THEN** the suite runs in the suites VM with the SDK found there, and the step's record says `vm`

#### Scenario: Not in the quick layer
- **WHEN** `just quick` plans its steps after a change to a crate
- **THEN** no wasm step is planned

#### Scenario: A list of programs goes to the VM
- **WHEN** `tests\wasm\rerun.list` names 270 programs and `check wasm-rerun --where vm` runs
- **THEN** those 270 programs run for the wasm platform in the suites VM, and no other program runs

#### Scenario: The whole family only when named
- **WHEN** `just full` or `just merge` plans its steps after a change to libqb's copy
- **THEN** `wasm-slice` is planned and `wasm-verification` is not

