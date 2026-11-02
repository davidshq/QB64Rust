# Tasks

Standing rules: CLAUDE.md.

## 1. The copy per tree

- [x] 1.1 `tools\linux\copy.sh` with `linux_copy <tree>` (design D1); `sync.sh` takes its default destination from
      it, gives rsync the files git reports changed since the last sync (`.sync-head`, `.sync-dirty`; Windows
      `git.exe` from WSL, D2) and gains `--checksum`, the exact form; `tools\linux\run.sh <command>` runs a command
      in the copy of the tree it is run from.
- [x] 1.2 `tools\check\steps.py`: the Linux steps run through `run.sh` (`IN_COPY`, `linux_whole`), `copy.sh` and
      `run.sh` among their inputs; `Step.forced`, `linux-sync`'s being its command with `--checksum`; `check.py`
      `command_of` uses it under `--force`, which the merge sets. Verified by `test_check.py`: the forced form
      carries `--checksum` and the usual does not; no Linux step names a copy and each reads the two scripts; the
      rule names `qb64rust` and `qb64rust-full` from the two folders.

## 2. Measure

- [x] 2.1 The existing copy renamed `~/qb64rust-full` and `~/qb64rust` seeded from it (D3).
- [x] 2.2 Time `sync.sh` from the checkout: the first sync (every time differs), nothing changed, one file
      edited, as after a checkout, and `--checksum` for comparison; the numbers in `design.md`, "Measured"
      (`target\scratch\2026-11-01\sync-measure.log`). The rsync route alone measured about 95 s (its scan), so
      the git-named list was built in its place (D2): about 2 s.

## 3. Documents and close

- [x] 3.1 `crates\README.md` ("From this Windows machine, in WSL"), `docs\LAYOUT.md` (`tools\linux\`),
      `tools\check\README.md` (`just merge`, "A run in a worktree") name the rule.
- [x] 3.2 `just quick` (the `tools` step ran, 52.5 s, the rest stood; `check_repo.py` 0 findings); `just check
      linux-tier1` from the checkout: `linux-sync` 7.3 s (incremental, 16 paths), `linux-tier1` 28.7 s, 31 test
      binaries passed in `~/qb64rust` (`target\scratch\2026-11-01\close.log`); `DECISIONS-SMALL.md` row;
      `STATUS.md` "Next" item 0's "Proposed next" replaced.
