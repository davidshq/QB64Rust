#!/usr/bin/env bash
# Runs the CLI tests that build (`cli -- --ignored`) on Linux in Docker Desktop, as the CI job `linux` does, some
# number of times, stopping at the first failure: for a failure that comes now and then (the DAP hangs of
# 2026-10-29). Run from Git Bash on Windows, or from a Linux or macOS shell.
#
# The image (tools/linux/Dockerfile) is built per lldb version, `qb64rust-linux:lldb<n>`. The container gets the
# working tree read-only at /src and copies it with sync.sh into the volume `qb64rust-linux-work`, where it builds:
# a Linux build leaves its objects in the tree, which the Windows tree must not get. The volumes
# `qb64rust-linux-rustup` and `qb64rust-linux-cargo` keep the toolchain and the crates between runs. Each run's
# output goes to target/linux-docker/lldb<n>-<run>.log; a passing run's log is removed.
#
# lldb needs ptrace, which Docker gives with SYS_PTRACE and no seccomp profile.
#
# Usage: tools/linux/docker.sh [--lldb <n>] [--times <n>] [--cpus <n>] [--all | <test filter>]
#   --lldb   lldb's version, 20 (Ubuntu's) or one apt.llvm.org has (default 23, that of the Windows toolchain)
#   --times  how many runs (default 1)
#   --cpus   the cores the container may use (default all; the CI runner has 4)
#   --all    every test of `cli -- --ignored`, as CI runs them; otherwise those whose names hold the filter
#            (default `dap_`)
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
lldb=23
times=1
cpus=
filter=dap_
while [[ $# -gt 0 ]]; do
    case "$1" in
        --lldb) lldb="$2"; shift 2 ;;
        --times) times="$2"; shift 2 ;;
        --cpus) cpus="$2"; shift 2 ;;
        --all) filter=; shift ;;
        -*) echo "unknown option $1" >&2; exit 2 ;;
        *) filter="$1"; shift ;;
    esac
done

image="qb64rust-linux:lldb$lldb"
docker build --build-arg "LLDB=$lldb" -t "$image" "$here"

out="$repo/target/linux-docker"
mkdir -p "$out"
src="$repo"
if command -v cygpath >/dev/null; then
    # Git Bash: Docker takes Windows paths, and MSYS must not rewrite the container's.
    src="$(cygpath -m "$repo")"
    out="$(cygpath -m "$out")"
    export MSYS_NO_PATHCONV=1
fi

docker run --rm --init \
    --cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
    ${cpus:+--cpus="$cpus"} \
    -v "$src:/src:ro" -v "$out:/out" \
    -v qb64rust-linux-work:/work -v qb64rust-linux-rustup:/opt/rustup -v qb64rust-linux-cargo:/opt/cargo \
    -e "TIMES=$times" -e "FILTER=$filter" -e "LLDB=$lldb" \
    "$image" bash /src/tools/linux/docker-loop.sh
