# A Linux like the CI job's (.github/workflows/rust.yml, job `linux`) for Docker Desktop: Ubuntu 24.04, the
# packages of that job, and lldb of the version the build argument LLDB names (20: Ubuntu's own, which hung now
# and then in a DAP test on 2026-10-29; any other: apt.llvm.org's). Rust comes from rustup, its toolchain from
# rust-toolchain.toml at the first run (into a volume, see docker.sh). Built and run by tools/linux/docker.sh.
FROM ubuntu:24.04

ARG LLDB=23

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        ca-certificates curl wget gnupg git rsync python3 xvfb xauth \
        build-essential libglu1-mesa-dev libasound2-dev libpng-dev libcurl4-openssl-dev libx11-dev libxcursor-dev \
        libxrandr-dev libxinerama-dev libxi-dev \
    && if [ "$LLDB" != 20 ]; then \
        wget -qO- https://apt.llvm.org/llvm-snapshot.gpg.key > /etc/apt/trusted.gpg.d/apt.llvm.org.asc \
        && echo "deb http://apt.llvm.org/noble/ llvm-toolchain-noble-$LLDB main" > /etc/apt/sources.list.d/llvm.list \
        && apt-get update; \
    fi \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends lldb-$LLDB \
    && lldb-dap-$LLDB --version \
    && rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo PATH=/opt/cargo/bin:$PATH
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain none --no-modify-path
