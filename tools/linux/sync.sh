#!/usr/bin/env bash
# Copy the working tree into a Linux folder (default ~/qb64rust), run from WSL (openspec change m7-linux, design D7).
#
# The files copied are those of `git ls-files -co --exclude-standard`: the tracked files and the untracked ones git
# does not ignore, uncommitted edits included. Nothing is staged and the Windows tree is only read. What a Linux
# build leaves in the copy (target/, libqb's objects) is ignored by git, so it is never listed and never deleted.
#
# A file that was copied by the last sync and is gone from the list now (deleted or renamed on Windows) is deleted
# from the copy: the list of the last sync is kept in <dest>/.sync-files, and rsync's --delete-missing-args removes
# each name of it that no longer exists in the tree.
#
# Usage: tools/linux/sync.sh [dest]
set -euo pipefail

src="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
dest="${1:-$HOME/qb64rust}"
mkdir -p "$dest"

now="$(mktemp)"
both="$(mktemp)"
trap 'rm -f "$now" "$both"' EXIT

# safe.directory: the tree is owned by the Windows user, which WSL's git does not take for the caller.
git -c safe.directory='*' -C "$src" ls-files -co --exclude-standard -z >"$now"

# The names to give rsync: this sync's and the last one's (those no longer in the tree are deleted).
if [[ -f "$dest/.sync-files" ]]; then
    cat "$now" "$dest/.sync-files" | sort -zu >"$both"
else
    sort -zu "$now" >"$both"
fi

# Exit 24, "some files vanished": the folders of a name of the last list that is gone (a change folder archived)
# are named as vanished though their files were deleted as asked. Not a failure; any other code is.
rc=0
rsync -a --checksum --from0 --files-from="$both" --delete-missing-args "$src/" "$dest/" || rc=$?
if [[ $rc -ne 0 && $rc -ne 24 ]]; then
    exit "$rc"
fi
cp "$now" "$dest/.sync-files"

count="$(tr -cd '\0' <"$now" | wc -c)"
echo "synced $count files into $dest"
