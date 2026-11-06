#!/usr/bin/env bash
# Run a command in the Linux copy of this source tree, from WSL (OpenSpec change linux-sync-per-tree, design D1):
# the copy is the one sync.sh fills for this tree (copy.sh names it), so the Linux steps of tools/check/steps.py
# run in the copy of the tree that was synced, whether the checkout or a worktree.
#
#   tools/linux/run.sh <command...>     run from the tree (the check tool runs it as `wsl --cd <tree> -e bash ...`)
#
# The command is given to a login shell (`bash -lc`), which has cargo on PATH.
#
# Nothing a step runs reaches the host's desktop or speakers (DECISIONS.md 2026-11-04). WSLg would show every
# program's window on the host and play its sound there. So the command runs on a display of its own, from
# xvfb-run, as the CI job `linux` does. WAYLAND_DISPLAY is cleared so that nothing reaches WSLg's Wayland
# server. XDG_RUNTIME_DIR names an empty folder of this run: WSLg's PulseAudio socket is in the usual one
# (/run/user/<uid>/pulse/native), and libpulse finds it there even without PULSE_SERVER. Without it, a program
# falls back to ALSA, which the runner points at its null device (tools/linux/asound-null.conf).
set -euo pipefail

src="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=copy.sh
source "$(dirname "${BASH_SOURCE[0]}")/copy.sh"
dest="$(linux_copy "$src")"

if [[ ! -d "$dest" ]]; then
    echo "no Linux copy at $dest: run tools/linux/sync.sh first" >&2
    exit 1
fi
if ! command -v xvfb-run >/dev/null; then
    echo "xvfb-run not found: install it in WSL with 'sudo apt-get install xvfb'" >&2
    exit 1
fi
# The Windows tree, for what lives beside it and not in the copy: the reference clone `$QB64RUST_TREE/../QB64pe`,
# whose tests/compile_tests the upstream suite reads (only read, CLAUDE.md working rule 2).
export QB64RUST_TREE="$src"

runtime="$(mktemp -d)"
trap 'rm -rf "$runtime"' EXIT
unset WAYLAND_DISPLAY PULSE_SERVER
export XDG_RUNTIME_DIR="$runtime"

# -a: the first free display from :99 up. xvfb-run stops its Xvfb when the command ends. -e: Xvfb's own messages
# go to a file, not to /dev/null. Under wsl.exe, /dev/stderr cannot be opened by name. The file is shown only when
# Xvfb failed, so that "Xvfb failed to start" comes with its reason; its usual warnings stay out of the log.
rc=0
xvfb-run -a -e "$runtime/xvfb.log" bash -lc "cd '$dest' && $*" || rc=$?
if grep -q "Fatal server error" "$runtime/xvfb.log" 2>/dev/null; then
    cat "$runtime/xvfb.log" >&2
fi
exit "$rc"
