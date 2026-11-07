# Roadmap

The milestones still open and the significant work remaining. Add newly discovered work; mark items complete as
they land, and move a milestone to `ROADMAP-COMPLETED.md` when it is declared done (M0 to M6 are there). The
remaining items are broad areas, not a fixed order or an exhaustive list of changes. Keep them current as the gaps
become clearer.

No progress numbers and no "now" marker: the current step, immediate next steps, open questions and the numbers
of the last full runs are in `STATUS.md` (`DECISIONS.md` 2026-10-07). The plan behind the milestones is
`study\00-synthesis.md` §2.

## Purpose

**A QB64 for anyone**: a QB64 programmer installs one thing, on Windows or
Linux, and has an editor, a compiler and a debugger for the QB64pe 4.7.0 dialect, with no QB64pe
installed. The project is done when that is true on both platforms. macOS was the third platform, milestone M8.
It was postponed to `SOMEDAY.md` on 2026-11-07 (`DECISIONS.md`). The order that follows from the purpose
(`study\38` §5.4): make it usable by someone else (§5.2), then one milestone per platform, with more real
programs and fuzzing beside them; the crash-class fixes and new language work as users ask. `UPSTREAM.md`
stays as the record of QB64pe's bugs found here; offering them to QB64pe blocks nothing (user, 2026-10-27).

This file was written again on 2026-10-12 from `study\00` §2, `STATUS.md` and `DECISIONS.md`: the earlier file
was never committed and was no longer on disk.

## M7. Linux

The first platform after Windows (`study\38` §4 option 2, §5.4): a GitHub runner exists and libqb's Linux path is
the better exercised. Built as one change, `m7-linux` (user, `DECISIONS.md` 2026-10-28), archived on 2026-10-29
(`openspec\changes\archive\2026-10-29-m7-linux\`); spec `compiler/platforms`. Linux x64 only; 32-bit and ARM are
out of scope. Done when its CI jobs have run (below).

- [x] The C++ toolchain on Linux: the system's `g++`, `make` and `objcopy` and the distribution's development
  packages, not a fetched one (user, `DECISIONS.md` 2026-10-28; a pinned compiler is a `SOMEDAY.md` row);
  `qb64rust toolchain install` names the packages. `lldb-dap` 20 or later from `PATH` (`DECISIONS.md` 2026-10-29)
- [x] The driver builds and links a program on Linux (`make OS=lnx`, no `.exe`); the debugger works there; the test
  runner runs there (`--platform linux`, in WSL after `tools/linux/sync.sh`)
- [x] The extension packaged for Linux (`linux-x64` `.vsix`); the compiler's release asset for Linux
  (`qb64rust-linux-x64-<version>.tar.gz`)
- [x] Linux CI jobs pass: `rust.yml`, `vscode-extension.yml` and `release-vsix.yml` each have a `linux` job
  (`m7-linux` tasks 7.1–7.3; CI is not a gate). All three passed on branch `after-pr25` on 2026-11-07: `rust.yml`
  at `2e03ba49`, the other two at `efbdee1d`
- [x] Recordings judged per platform where QB64pe's output differs by platform: `<name>.linux.output` and
  `.linux.err` sidecars recorded from QB64pe built on Linux, and a `known.linux.list` per suite (spec
  `testing/golden-corpus`, "Recordings per platform")
- [x] Shared libraries in `DECLARE LIBRARY` (`.so`, `.a`, `/usr/lib64` and `/usr/lib`, `dlopen` for
  `DECLARE DYNAMIC LIBRARY`), in the old compiler's search order
- [x] The Linux runs in the test cadence: `tools\close\close.py`'s Linux family, run when a change touches what the
  build is made from or a Linux input, and always at a merge into `main` (`DECISIONS.md` 2026-10-28, 2026-10-29)

## After M7: calls with side effects left to right

- [x] The parts of one expression, call or statement are evaluated in the Windows build's order on every platform
  (user, `DECISIONS.md` 2026-10-28 and 2026-11-07; OpenSpec change `eval-order-left-to-right`, built on branch
  `after-pr25`). QB64pe's Windows build runs a call's arguments left to right and its Linux build often right to
  left. The codegen now holds a part in a temporary wherever another order could change the result
  (`codegen-cpp\src\order.rs`; `QB64PE-DIVERGENCES.md` D-060, with `SWAP`'s D-054 as its first case). The Linux
  sidecars that only recorded the other order are cut (`s86_field_lset`, `v30_g_iif_min_max`). The nine others
  are deleted. `verification\v69_*` shows it on both platforms.

## Not tied to a milestone yet

- Deferred features and ideas: `SOMEDAY.md`
