# Proposal

## Why

`linux-sync` takes about 3 minutes on every `just full` and `just merge` (175.5 s measured 2026-11-01, in the
worktree's second run) to copy a handful of files. `tools\linux\sync.sh` runs `rsync --checksum`, which reads and
hashes all 24,000 files on both sides, the Windows side through `/mnt/c`, WSL's slow route to Windows files. The
checksums are there because one Linux copy (`~/qb64rust`) is synced in turn from the checkout and from the
worktree of `full`: each switch gives every file a new modification time, so sizes and times cannot tell what
changed. The step's inputs are the Rust sources, libqb, `tests\**` and `verification\**`, so nearly every full
run pays the 3 minutes, as long now as `verification` itself from a warm pass cache.

## What Changes

- **A Linux copy per source tree.** `sync.sh` names its destination from the source tree's folder
  (`~/qb64rust` for `QB64Rust`, `~/qb64rust-full` for `QB64Rust-full`), so each copy follows one tree and its
  modification times stay meaningful. The Linux steps of `steps.py` (`linux-tier1`, `linux-slice`, the merge's
  `linux-*` whole suites) take the copy from the same rule instead of the hardcoded `~/qb64rust`.
- **`--checksum` dropped.** With one tree per copy, rsync compares sizes and times: an unchanged tree checks in
  seconds and copies nothing; an edit or a `git checkout` copies what it touched. `--delete-missing-args` and the
  `.sync-files` list stay as they are.
- **Measured before and after** in `design.md`: a sync with nothing changed, and one after a `git checkout` of
  the next commit in the worktree.
- **Not changed:** what is copied (`git ls-files -co --exclude-standard`, uncommitted edits included); the
  Linux build and pass caches, which live inside each copy and so stay separate by themselves.

## Capabilities

### Modified Capabilities

- `testing/check-workflow`: the Linux steps' copy is the source tree's own.

## Impact

- `tools\linux\sync.sh`, `tools\check\steps.py` (the three Linux steps and `linux_whole`), `test_check.py`.
- `crates\README.md` and `docs\LAYOUT.md` name `~/qb64rust`; each gets the rule.
- One-time cost: the worktree's copy builds Linux from cold once (a few minutes), and a second copy takes disk in
  WSL (about the size of `~/qb64rust` with its `target\`).
- Alternative considered: a git clone in WSL that fetches the commit from the Windows repository, for `full` and
  `merge`, which run on a commit anyway. Faster still, but it would drop the untracked files `sync.sh` copies
  today, a change of behavior the rsync route avoids.
