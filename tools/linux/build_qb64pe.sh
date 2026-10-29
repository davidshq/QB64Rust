#!/usr/bin/env bash
# Build QB64pe's own compiler on Linux at the reference commit, in a scratch folder (openspec change m7-linux,
# design D6). The result records the Linux sidecars (run_legacy_tests.py --record --platform linux --qb-root <scratch>).
#
# The reference clone is only read (CLAUDE.md working rule 2): it is cloned with `git clone --no-checkout` into the
# scratch folder, which then checks out the commit `libqb/SOURCE.md` names and builds there, as QB64pe's
# `setup_lnx.sh` does (`make OS=lnx BITS=64 BUILD_QB64=y`). Not QB64pe's Linux release archive: it is not shown
# equal to the reference commit (m6-toolchain, design D1).
#
# Usage: tools/linux/build_qb64pe.sh <scratch> [clone]
#   <scratch>  a folder that does not exist yet or is empty (on Linux's own disk, not /mnt/c)
#   [clone]    the reference clone; default: QB64pe beside this repository
# Prints the folder of the built qb64pe on its last line.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
scratch="${1:?usage: build_qb64pe.sh <scratch> [clone]}"
clone="${2:-$here/../QB64pe}"

commit="$(sed -n 's/^Commit: `\([0-9a-f]*\)`.*/\1/p' "$here/libqb/SOURCE.md")"
[[ -n "$commit" ]] || { echo "no commit in $here/libqb/SOURCE.md" >&2; exit 1; }
[[ -d "$clone/.git" ]] || { echo "no clone at $clone" >&2; exit 1; }
if [[ -e "$scratch" && -n "$(ls -A "$scratch")" ]]; then
    echo "$scratch is not empty" >&2
    exit 1
fi

# safe.directory: the clone may be owned by the Windows user, which WSL's git does not take for the caller.
git -c safe.directory='*' clone --quiet --no-checkout "$clone" "$scratch"
cd "$scratch"
git -c advice.detachedHead=false checkout --quiet "$commit"

make OS=lnx BITS=64 BUILD_QB64=y -j3 >build.log 2>&1 || { tail -40 build.log >&2; exit 1; }
[[ -x qb64pe ]] || { echo "no qb64pe was built (build.log)" >&2; exit 1; }
pwd
