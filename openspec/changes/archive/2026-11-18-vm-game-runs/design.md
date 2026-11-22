# Design

## Context

The suites VM (`vm-suites`) restores its checkpoint `ready` at every run, mounts a cache disk at the parent folder
of this repo, and copies in this repo's tree and the QB64pe clone, so that they are at the host's paths in the
guest. `step.ps1` runs one job there: a command in the test account's session, through a scheduled task, with the
outputs copied back.

A game port's large run needs, beside that:

- the game's repo, which is beside this repo, so it is on the cache disk's volume too;
- other repos beside it (Civil War Strategy's browser tests read the QBJS fork, `..\qbjs`);
- folders git ignores (`tests\web\node_modules`, the `puppeteer-core` package; the VM has no network to install
  it);
- Node.js, which the VM lacks;
- a browser: the VM has Edge 152, from Windows 11 itself (looked at 2026-11-17);
- for runs that build the game with QB64Rust, `target\release\qb64rust.exe` and this repo's `tools\`.

## Decisions

### D1. A script of its own, not a `check` step

`game.ps1` runs a game's command. It is not a step of `check`, whose steps test this repo and whose limits and
pass caches are kept per step. It calls `suites.ps1 -Start` and `-Stop` and `step.ps1`, so the VM's refusals, the
restore, the clock and license checks, the cache disk and the copy of this repo are the same as for `check`.

### D2. Node: copied in now (A), installed from the next making on (B)

The pin is in `pins.psd1`, which both scripts read, so the two cannot differ.

- **A.** `game.ps1` looks for `%ProgramFiles%\nodejs\node.exe` in the guest. Without it, it unpacks the pinned
  zip at `<parent>\qb64rust-vm\node-v<version>-win-x64\` on the cache disk, once: it is there at the next run.
  The host downloads the zip, as `create.ps1` downloads the toolchain's, since TLS downloads in the guest broke on
  2026-11-05 (`README.md`, "The suites VM").
- **B.** `create.ps1 -Shape Suites` unpacks the same zip into `%ProgramFiles%\nodejs`, adds it to the machine's
  PATH and checks `node --version` before the checkpoint. A VM made so needs nothing from A.

The zip rather than the MSI: it installs nothing, it is the same file for A and B, and unpacking it is all the
MSI's Node part does. Node 24 (LTS "Krypton"), the host's major version, on which the player was tried.

### D3. The game and `-With` folders are copied whole each run

Each is archived from `git ls-files -co --exclude-standard` (what git tracks or would track), plus the game's
`-Include` folders, and replaces the guest's last copy. Unlike this repo's tree, no `target\` needs to survive in
them. Neither `-Game` nor `-With` may name this repo, QB64pe, `community` or `qb64rust-vm`, whose
copies the VM keeps itself.

### D4. `step.ps1` learns three optional fields

- `compiler`: the `qb64rust.exe` the toolchain guard asks, relative to the tree, or `""` for no guard. A game's
  job names this repo's `target\release\qb64rust.exe`, which the job also copies in.
- `path`: guest folders put first on PATH in the step's batch file.
- `scratch`: the host folder where the outputs' archive is unpacked from; a game's repo has no `target\check\vm`.

`outputs.py` is found beside `step.ps1`, which is at the same path in the guest as on the host. `check`'s jobs set
none of the fields and run as before.

### D5. Folder outputs

`outputs.py` takes a folder as an output and lists every file below it, still without executables. A browser run's
results are many screenshots in one folder per script.

## Risks

- **B is untried** until the VM is next made. `create.ps1` checks `node --version` before the checkpoint, so a
  failure shows then, not in a run.
- **Edge updates itself** on a networked machine; the VM has no network, so it stays at the base image's
  version until the base is made again. A new Edge may need a newer `puppeteer-core` in the game.
- **The copies take time.** `node_modules` holds a few thousand files; each run copies it again.
