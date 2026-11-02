# Design

## Context

See `proposal.md`, "Why". `tools\linux\sync.sh` fed one Linux folder, `~/qb64rust`, from whichever Windows tree it
was run in: the checkout `QB64Rust` or the worktree `QB64Rust-full` that `just full` and `just merge` run in. Each
switch gave every file in the copy the other tree's modification times, so the script ran `rsync --checksum`,
which reads and hashes all 24,000 files on both sides, the Windows side through `/mnt/c`. The `linux-sync` step
took 173.7 s in the `just full` of 2026-11-01 on the worktree (175.5 s in the run before it). Its inputs are the
Rust sources, libqb, `tests\**` and `verification\**`, so nearly every change pays it. The review of 2026-11-01
(the pragmatic and test engineers, inline) set two conditions on the proposal: the measurement decides whether the
rsync route is enough, and the merge keeps the exact copy.

## Decisions

### D1. One Linux copy per source tree, named from the tree's folder

`tools\linux\copy.sh` holds the one rule, a function `linux_copy <tree>` that prints `$HOME/<folder name in lower
case>`: `~/qb64rust` for `QB64Rust`, `~/qb64rust-full` for `QB64Rust-full`. `sync.sh` sources it for its default
destination (the positional `dest` still overrides it); `tools\linux\run.sh <command>` sources it to run a command
in the copy of the tree it is run from (`bash -lc`, so cargo is on `PATH`), and refuses with a message when the copy
does not exist yet. The Linux steps of `tools\check\steps.py` (`linux-tier1`, `linux-slice`, the merge's
`linux-corpus`, `linux-upstream`, `linux-verification` from `linux_whole`) run through `run.sh` from the tree
(`IN_COPY`: `wsl --cd "$(pwd -W)" -e bash tools/linux/run.sh '<command>'`), so no step names a copy; `run.sh` and
`copy.sh` are among their inputs (`LINUX_SCRIPTS`). Alternative considered: computing the copy's name in
`steps.py` from the tree's folder. The rule would then live in two languages; one shell file both scripts source
keeps it in one.

Two trees of the same folder name under different parents would share a copy. Not guarded: the project has one
checkout and one worktree, both beside each other (`CLAUDE.md`, "Where things live").

### D2. git names the files to copy; content under `--force` and at the merge

The proposal's route, `rsync -a` comparing sizes and times, was built first and measured ("Measured", runs 1 to
7 of the first table): about 95 s with nothing changed, against 169 s for the old form. The cost is rsync's own
scan, 113 s to stat the 24,000 files through `/mnt/c` (`rsync -n` alone), where listing them with git takes 2 s.
So the usual sync never scans the tree: `sync.sh` gives rsync the files git reports changed since the last sync,
and nothing else. Three lists, unioned: `git diff --name-only --no-renames <last HEAD> HEAD` (the commits since,
a rename as a deletion and an addition), `git status --porcelain -z --untracked-files=all --no-renames` now (edits,
additions, deletions, untracked files), and the status list of the last sync (an edit undone, an untracked file
removed, which `--delete-missing-args` deletes from the copy). The last HEAD and status list are kept in
`<dest>/.sync-head` and `.sync-dirty`; `.sync-files`, the full list, is still written each time. git is Windows'
`git.exe`, callable from WSL and reading the tree natively (status 0.4 s, `ls-files` 0.6 s), with WSL's git as the
fallback when it is not on `PATH` (its status would stat every file through `/mnt/c`). A copy without
`.sync-head`, or whose last HEAD is gone, gets a full sync by size and time.

`sync.sh --checksum` is the exact form: every file of the tree and of the last list, compared by content. `Step`
gets a field `forced`, the command used when the step runs under `--force` (`check.py`, `command_of`, with the
note "forced: the exact form"); `linux-sync`'s is its command with `--checksum`. `check merge` forces every step
(`make_plan`, `main`), so the merge's Linux results rest on a copy checked by content, as before, and `just force`
does the same. The risk accepted for `quick` and `full`: the usual form trusts git's view of the tree, so a change
git cannot see (a file written in the copy by something other than a build, an edit of a git-ignored file) is not
corrected until the next exact sync; no tracked file is a build output (`.gitignore`: libqb's objects, the `v37`
libraries, `target\`), so a Linux build in the copy alters nothing the list would skip. The Linux step's
fingerprint is taken from the Windows files, so a stale copy would be called valid; the merge run, which the pass
lists and `main` rest on, does not carry that risk.

Alternative considered: a git clone in WSL that fetches the commit (`proposal.md`). As fast, but it drops the
untracked files and uncommitted edits the sync copies today, and a `git checkout` in the copy would rewrite every
file's time once, so the Linux build would start over. The git-named list keeps the copy's behavior and its
build.

### D3. The first sync into a copy fed by another tree

The copy's files have the other tree's times, so its first sync (full, by size and time) copies every file once
(rsync writes 24,000 files through `/mnt/c`), and the Linux build after it rebuilds, as cargo keys on times. One
time per copy. On 2026-11-01 the existing `~/qb64rust`, last fed from the worktree, was renamed `~/qb64rust-full`
(so the worktree's first sync copies nothing) and `~/qb64rust` was seeded from it with rsync inside WSL (15 GB,
`target\` included, so the checkout's first Linux build has the dependencies built).

## Measured

Timings of `sync.sh` from the checkout on 2026-11-01, the copy seeded from the worktree's (D3), each a separate
run of `wsl --cd <tree> -e bash tools/linux/sync.sh` from Git bash, wall time including WSL's start
(`target\scratch\2026-11-01\sync-measure.log`, written by `sync-measure.sh` beside it):

The proposal's route first (`rsync -a` by size and time, the copy's files touched so that every time differed):

| Run | Seconds |
|---|---|
| Every time differs (as the first sync into a copy fed by another tree) | 160.6 |
| Nothing changed | 101.2, 92.2, 88.1, 108.1 |
| One file edited | 105.0 |
| `--checksum`, nothing changed (the old form) | 169.0 |
| `rsync -n` alone over the 24,000 files, inside WSL (the scan) | 113.4 |
| `git ls-files -co` inside WSL, the list alone | 2.4 |

Then the git-named list (D2), on the same copy:

| Run | Seconds |
|---|---|
| First sync (no `.sync-head`: full, by size and time) | 103.7 |
| Nothing changed (15 paths given to rsync, the tree's uncommitted edits) | 0.8, 1.6, 0.6 |
| One file edited | 1.7 |
| The edit undone | 1.8 |
| As after a checkout of two commits (`.sync-head` set to `aa9ce2df`; 39 paths) | 2.4 |
| `--checksum`, nothing changed (the exact form) | 181.5 |

`git.exe` from WSL: `status --porcelain -z -uall` 0.4 s, `ls-files -co` 0.6 s, `rev-parse` 0.1 s; WSL's git `diff
--name-only` of two commits 1.7 s. The rsync route alone would have left the step at about 95 s (its scan); the
git-named list takes it to about 2 s, so the clone route of `proposal.md` is not needed.

## Risks

- A change git cannot see is not copied by the usual form (D2); the exact form at the merge corrects it.
- `git.exe` from WSL depends on WSL's Windows interop; without it, WSL's git is used and the status takes about
  as long as the old scan. The message of the sync names which form ran.
