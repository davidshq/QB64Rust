#!/usr/bin/env bash
# The container's half of tools/linux/docker.sh (see there): copies the tree from /src into /work/tree, builds,
# runs clippy on the driver's tests (it compiles the #[cfg(unix)] code a Windows check skips), then runs the CLI
# tests that build $TIMES times, those whose names hold $FILTER (all with an empty one), stopping at the first
# failure. Each run's output: /out/lldb$LLDB-<run>.log, removed when the run passes.
set -euo pipefail

bash /src/tools/linux/sync.sh /work/tree
cd /work/tree
rustup toolchain install
lldb-dap-"$LLDB" --version | head -1

cargo clippy -p qb64rust-driver --tests --locked -- -D warnings
cargo test -p qb64rust-driver --test cli --locked --no-run

# shellcheck disable=SC2086 # an empty filter is no argument
for i in $(seq 1 "$TIMES"); do
    log="/out/lldb$LLDB-$i.log"
    start=$SECONDS
    if xvfb-run -a cargo test -p qb64rust-driver --test cli --locked -- --ignored $FILTER >"$log" 2>&1; then
        echo "run $i of $TIMES: passed ($((SECONDS - start)) s)"
        rm -f "$log"
    else
        echo "run $i of $TIMES: FAILED ($((SECONDS - start)) s), its output: target/linux-docker/lldb$LLDB-$i.log"
        grep -E "^test .* FAILED|^    [a-z_]+$|panicked at|test result" "$log" || true
        exit 1
    fi
done
