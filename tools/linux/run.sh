#!/usr/bin/env bash
# Run a command in the Linux copy of this source tree, from WSL (OpenSpec change linux-sync-per-tree, design D1):
# the copy is the one sync.sh fills for this tree (copy.sh names it), so the Linux steps of tools/check/steps.py
# run in the copy of the tree that was synced, whether the checkout or a worktree.
#
#   tools/linux/run.sh <command...>     run from the tree (the check tool runs it as `wsl --cd <tree> -e bash ...`)
#
# The command is given to a login shell (`bash -lc`), which has cargo on PATH.
set -euo pipefail

src="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=copy.sh
source "$(dirname "${BASH_SOURCE[0]}")/copy.sh"
dest="$(linux_copy "$src")"

if [[ ! -d "$dest" ]]; then
    echo "no Linux copy at $dest: run tools/linux/sync.sh first" >&2
    exit 1
fi
# The Windows tree, for what lives beside it and not in the copy: the reference clone `$QB64RUST_TREE/../QB64pe`,
# whose tests/compile_tests the upstream suite reads (only read, CLAUDE.md working rule 2).
export QB64RUST_TREE="$src"
exec bash -lc "cd '$dest' && $*"
