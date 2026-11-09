# Proposal: community-programs

## Why

The suites say "QB64Rust does what QB64pe does". Apart from Civil War Strategy, nothing says "programs people
wrote compile". The GitHub survey (`study\44`) found large community programs that QB64pe builds and QB64Rust did
not: DRAW, every InForm program, a quarter of the Libraries Pack. Nine constructs were fixed on 2026-11-09
(`verification\v70`). Without a standing check, the next such gap is found only by chance, and a fix can break a
program that passed. The user asked for this check (`DECISIONS.md` 2026-11-09, answer 2 to `study\44` §6).

## What Changes

- A manifest in this repo, `tests\community\repos.toml`, lists each community repository: its URL, the commit it
  is pinned to, its license, its submodules and the options its programs need (an include root).
- A tool, `tools\community\community.py`, fetches the repositories into clones outside the repo (`..\community\`,
  beside `..\QB64pe\`) at their pinned commits, and checks every `.bas` file of them with `qb64rust -z` (the front
  end and the C++ writer, no C++ build, nothing run).
- A pass list, `tests\community\pass.list`, names the programs that must pass. A run fails when one of them fails.
  The list only grows: a program that passes and is not on it is reported, and added with `--update-list`.
- A `check` step, `community`, in the full layer, run in the suites VM by default. The VM has no network, so the
  clones are fetched on the host and copied to the VM's cache disk, as the QB64pe clone is.
- The repositories are treated as untrusted. Nothing in them is run. The fetch takes submodule URLs from the
  manifest and runs git with symlinks, hooks, `file` remotes and LFS turned off. An audit lists each clone's
  binaries, scripts, symlinks and other risky files, and it is read before a pin is added or moved. The check's
  output is limited to program names and short first errors.
- No file of a community repository is copied into this repo. The Lance Haffner games (AGPL-3.0) and the
  programs with no license are in the set, run from their clones only (`DECISIONS.md` 2026-11-09, answer 3).

Out of scope: building the programs, running them, and recording their output with `qb64pe.exe`. Those are a
later change, a few programs at a time.

## Capabilities

### New Capabilities

- `testing/community-programs`: the manifest of pinned community repositories, their clones, the front-end check
  of their programs, and the pass list it is held to.

### Modified Capabilities

- `testing/suites-vm`: the cache disk also keeps a copy of the community clones, replaced when the manifest's
  commits change.

## Impact

- New: `tests\community\repos.toml`, `tests\community\pass.list`, `tests\community\README.md`,
  `tools\community\community.py` and its tests.
- `tools\check\steps.py` (the step), `tools\check\check.py` (an external input `community`: the clones' commits),
  `tools\winvm\suites.ps1` (the copy to the cache disk).
- `docs\LAYOUT.md` rows; `.gitignore` needs nothing, since the clones are outside the repo.
- The host fetches over the network only when the user or Claude runs `community.py fetch`; the check never
  fetches. About 1 GB of disk beside the tree for the clones (the Museum alone is 335 MB).
