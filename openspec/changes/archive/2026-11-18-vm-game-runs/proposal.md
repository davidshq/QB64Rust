# Proposal

## Why

Civil War Strategy's QBJS port has a player for its 41 key scripts, `tests\web\play_qbjs.js` (the game's
`docs\qbjs-plan.md`, step 12). It plays them in a headless browser, and with `--native` also in the QB64Rust
build. All 41 at once is a large run, and large runs go in the suites VM (`CLAUDE.md`, working rule 6). The suites
VM runs only `check`'s steps on this repo's tree. It has no Node.js, and no way to copy in a game's repo or the
QBJS fork. It does have a browser: Microsoft Edge 152, which comes with Windows 11 (looked at in the VM on
2026-11-17), and the player already accepts Edge.

The user asked for Node to be put in the VM at run time now (A), and installed in the VM from its next making on
(B) (2026-11-17).

## What Changes

- `tools\winvm\pins.psd1`: the pin of Node.js 24.22.3 (the Windows zip, by SHA-256), read by both scripts below.
- `tools\winvm\game.ps1`, new: runs one command of a game's repo in the suites VM. It starts the VM as `check`
  does, with this repo as the tree, puts Node in, copies the game and the folders named by `-With` (such as
  `qbjs`), runs the command through `step.ps1`, copies the named outputs back, and turns the VM off.
- **A, now:** when the VM has no installed Node, `game.ps1` copies the pinned zip onto the cache disk once and
  unpacks it there.
- **B, at the next making:** `create.ps1 -Shape Suites` unpacks the same zip into `%ProgramFiles%\nodejs` and puts
  it on the machine's PATH, before the checkpoint. `game.ps1` then uses that one and copies nothing.
- `step.ps1` takes three optional fields of a job: `compiler` (which `qb64rust.exe` the toolchain guard asks; `""`
  for none), `path` (guest folders put first on PATH), and `scratch` (the host folder for the outputs' archive).
  It finds `outputs.py` beside itself instead of in the tree, since a game's tree has none.
- `outputs.py`: an output that is a folder brings every file below it, but executables.

`check`'s own steps are unchanged: they set none of the new fields.

## Impact

- `tools\winvm\`: `game.ps1` and `pins.psd1` new; `step.ps1`, `outputs.py`, `test_outputs.py`, `create.ps1` and
  `README.md` changed.
- Spec `testing/suites-vm`: Node in the VM, and a requirement for a game's run.
- The cache disk keeps Node and the copies of the game and of `qbjs`.
- The host downloads Node's zip once into `%LOCALAPPDATA%\qb64rust\winvm\installers\`.
- B is not tried until the suites VM is next made (at its 90 days, or when the toolchain pin moves).
