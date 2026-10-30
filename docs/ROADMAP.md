# Roadmap

The milestones still open and the significant work remaining. Add newly discovered work; mark items complete as
they land, and move a milestone to `ROADMAP-COMPLETED.md` when it is declared done (M0 to M5b are there). The
remaining items are broad areas, not a fixed order or an exhaustive list of changes. Keep them current as the gaps
become clearer.

No progress numbers and no "now" marker: the current step, immediate next steps, open questions and the numbers
of the last full runs are in `STATUS.md` (`DECISIONS.md` 2026-10-07). The plan behind the milestones is
`study\00-synthesis.md` §2.

## Purpose

**A QB64 for anyone**: a QB64 programmer installs one thing, on Windows,
Linux or macOS, and has an editor, a compiler and a debugger for the QB64pe 4.7.0 dialect, with no QB64pe
installed. The project is done when that is true on all three platforms. The order that follows from it
(`study\38` §5.4): make it usable by someone else (§5.2), then one milestone per platform, Linux first, with more
real programs and fuzzing beside them; the crash-class fixes and new language work as users ask. `UPSTREAM.md`
stays as the record of QB64pe's bugs found here; offering them to QB64pe blocks nothing (user, 2026-10-27).

This file was written again on 2026-10-12 from `study\00` §2, `STATUS.md` and `DECISIONS.md`: the earlier file
was never committed and was no longer on disk.

## M6. The runtime without QB64pe

Rewritten on 2026-10-28 from "runtime modernisation" (`study\37` §5.1, `DECISIONS.md` 2026-10-28): measured, none
of the runtime's design debts shows as a failure a user meets, so the second backend and the redesign of the error
model, string heap and threading are a `SOMEDAY.md` row, with the conditions that would bring them back. Done when
the extension builds, runs and debugs a program with the toolchain and compiler it fetches, and the rows the user
ruled to change are edits of libqb with their tests; the check of that on a machine with no QB64pe is postponed
(below).

- [x] A C++ toolchain and `lldb-dap` the project fetches, pinned and checked, QB64pe's install a fallback
  (`m6-toolchain`)
- [x] The compiler as a download of its own beside the `.vsix`, one version number for both (`m6-compiler-download`)
- [x] The fixes switch and the crash-class rulings of `study\39`: the compiler half (`fixes-switch`) and libqb
  repaired in place through `tools\libqb\edits` (`libqb-fixes`)
- The check on a machine with no QB64pe (the released `.vsix` installs, fetches its toolchain, and builds, runs and
  debugs a program) is postponed to `SOMEDAY.md` (user, `DECISIONS.md` 2026-10-28); M6 is done without it, and
  moves to `ROADMAP-COMPLETED.md` at the next merge into `main`

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
- [ ] Linux CI jobs pass: `rust.yml`, `vscode-extension.yml` and `release-vsix.yml` each have a `linux` job
  (`m7-linux` tasks 7.1–7.3; CI is not a gate). The first two ran on `272be0d` and failed; the release job has not
  run yet
- [x] Recordings judged per platform where QB64pe's output differs by platform: `<name>.linux.output` and
  `.linux.err` sidecars recorded from QB64pe built on Linux, and a `known.linux.list` per suite (spec
  `testing/golden-corpus`, "Recordings per platform")
- [x] Shared libraries in `DECLARE LIBRARY` (`.so`, `.a`, `/usr/lib64` and `/usr/lib`, `dlopen` for
  `DECLARE DYNAMIC LIBRARY`), in the old compiler's search order
- [x] The Linux runs in the test cadence: `tools\close\close.py`'s Linux family, run when a change touches what the
  build is made from or a Linux input, and always at a merge into `main` (`DECISIONS.md` 2026-10-28, 2026-10-29)

## After M7: calls with side effects left to right

Decided (user, `DECISIONS.md` 2026-10-28), not yet built. QB64pe's Windows build runs a call's arguments left to
right and its Linux build right to left. One change makes the codegen run every such call left to right on every
platform, then deletes the Linux sidecars that only record the other order (seven corpus programs, `SOMEDAY.md`,
"QB64pe behaviours to review"). `SWAP` already works this way (`QB64PE-DIVERGENCES.md` D-054).

## M8. macOS

The same items as M7 for macOS (`.dylib`, the toolchain and `lldb-dap`, packaging, a CI job, recordings judged
per platform), after Linux. `m7-linux`'s `Platform` value, sidecars and `known.linux.list` files are where a
macOS target would plug in. On Apple silicon, the runtime ABI's reliance on x86 memory order is one of the
conditions in `SOMEDAY.md` for bringing back the runtime redesign.

## Not tied to a milestone yet

- Deferred features and ideas: `SOMEDAY.md`
