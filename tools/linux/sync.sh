#!/usr/bin/env bash
# Copy the working tree into its Linux folder, run from WSL (openspec change m7-linux, design D7; one copy per
# source tree since linux-sync-per-tree, design D1: copy.sh names it, `~/qb64rust` for the checkout `QB64Rust`,
# `~/qb64rust-full` for the worktree `QB64Rust-full`).
#
# The files of the copy are those of `git ls-files -co --exclude-standard`: the tracked files and the untracked ones
# git does not ignore, uncommitted edits included. Nothing is staged and the Windows tree is only read. What a Linux
# build leaves in the copy (target/, libqb's objects) is ignored by git, so it is never listed and never deleted.
#
# Which files rsync is given (design D2): git's, never a scan of the tree. rsync reading the size and time of each
# of the 24,000 files through /mnt/c takes about 110 s, so the usual sync gives it only the files that changed
# since the last one: the commits between the last synced HEAD and this one (`git diff --name-only`), the files
# `git status` reports now (edited, added, deleted, untracked), and those it reported at the last sync (an edit
# undone, an untracked file removed). A path in the list that is gone from the tree is deleted from the copy
# (--delete-missing-args). The last sync's HEAD and status list are kept in <dest>/.sync-head and .sync-dirty,
# and the full list of its files in .sync-files.
#
# `--checksum` is the exact form (`check merge`, `--force`): every file of the tree and of the last list is given
# to rsync, which compares each by content (about 3 minutes). The first sync into a copy, or one whose last HEAD
# is gone, is a full sync by size and time.
#
# git: Windows git (git.exe, callable from WSL) reads the tree natively, so its status takes under a second; WSL's
# git would stat every file through /mnt/c. WSL's git is the fallback when git.exe is not on PATH.
#
# Usage: tools/linux/sync.sh [--checksum] [dest]
set -euo pipefail

src="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=copy.sh
source "$(dirname "${BASH_SOURCE[0]}")/copy.sh"

checksum=0
if [[ "${1:-}" == "--checksum" ]]; then
    checksum=1
    shift
fi
dest="${1:-$(linux_copy "$src")}"
mkdir -p "$dest"
cd "$src"

if command -v git.exe >/dev/null 2>&1; then
    g() { git.exe "$@"; }
else
    # In a worktree (`just full`), .git is a file naming the repository's folder by its Windows path, which WSL's
    # git cannot follow: it is given the folder by its WSL path. safe.directory: the tree is owned by the Windows
    # user, which WSL's git does not take for the caller.
    where=()
    if [[ -f "$src/.git" ]]; then
        gitdir="$(sed -n 's/^gitdir: //p' "$src/.git" | tr -d '\r')"
        [[ "$gitdir" == [A-Za-z]:* ]] && gitdir="$(wslpath -u "$gitdir")"
        where=(--git-dir="$gitdir" --work-tree="$src")
    fi
    g() { git -c safe.directory='*' "${where[@]}" "$@"; }
fi

now="$(mktemp)"
dirty="$(mktemp)"
list="$(mktemp)"
trap 'rm -f "$now" "$dirty" "$list"' EXIT

head="$(g rev-parse HEAD | tr -d '\r')"
g ls-files -co --exclude-standard -z >"$now"
# One entry per path, `XY path`: --no-renames so that a rename is a deletion and an addition, not a pair.
# --no-optional-locks: status neither takes .git/index.lock nor writes the refreshed index back, so a sync never
# collides with staging in the Windows tree.
g --no-optional-locks status --porcelain -z --untracked-files=all --no-renames | awk 'BEGIN { RS = "\0"; ORS = "\0" } length($0) > 3 { print substr($0, 4) }' >"$dirty"

opts=(-a)
how="full"
if [[ $checksum -eq 1 ]]; then
    opts+=(--checksum)
    how="checksum"
fi
if [[ $checksum -eq 0 && -f "$dest/.sync-head" ]] &&
    g diff --name-only -z --no-renames "$(tr -d '\r' <"$dest/.sync-head")" "$head" >"$list" 2>/dev/null; then
    cat "$dirty" >>"$list"
    [[ -f "$dest/.sync-dirty" ]] && cat "$dest/.sync-dirty" >>"$list"
    how="incremental"
else
    # Every file of the tree and of the last sync (those no longer in the tree are deleted).
    cat "$now" >"$list"
    [[ -f "$dest/.sync-files" ]] && cat "$dest/.sync-files" >>"$list"
fi
sort -zu -o "$list" "$list"

# Exit 24, "some files vanished": the folders of a name of the last list that is gone (a change folder archived)
# are named as vanished though their files were deleted as asked. Not a failure; any other code is.
rc=0
rsync "${opts[@]}" --from0 --files-from="$list" --delete-missing-args "$src/" "$dest/" || rc=$?
if [[ $rc -ne 0 && $rc -ne 24 ]]; then
    exit "$rc"
fi
cp "$now" "$dest/.sync-files"
cp "$dirty" "$dest/.sync-dirty"
printf '%s\n' "$head" >"$dest/.sync-head"

count="$(tr -cd '\0' <"$now" | wc -c)"
given="$(tr -cd '\0' <"$list" | wc -c)"
echo "synced $count files into $dest ($how, $given given to rsync)"
