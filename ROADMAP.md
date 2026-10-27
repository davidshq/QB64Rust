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

## M6. Runtime modernisation

- [ ] The ABI owned by the new compiler; error model, string heap and threading redesigned behind golden tests.
  This starts as a second backend crate beside `codegen-cpp`, which stays pinned to libqb's ABI and to the
  call-site check; `codegen-cpp` is not modernised in place (`study\archive\30` §7, user 2026-10-16)

## M7. Linux

The first platform after Windows (`study\38` §4 option 2, §5.4): a GitHub runner exists and libqb's Linux path is
the better exercised. Today libqb builds there, but the driver's build, the runner, the extension's packaging and
CI are Windows only.

- [ ] A pinned C++ toolchain for Linux, fetched as on Windows (`m6-toolchain`), with `lldb-dap` from it
- [ ] The driver builds and links a program on Linux; the test runner runs there
- [ ] The extension packaged for Linux; the compiler's release asset for Linux (`m6-compiler-download`)
- [ ] A Linux CI job
- [ ] Recordings judged per platform where QB64pe's output differs by platform: `printf` float text, console
  comma zones (one space on Linux, D-011)
- [ ] Shared libraries in `DECLARE LIBRARY` (`.so`, `/usr/lib`, `dlopen`; moved from `SOMEDAY.md`, left out
  of `m3-declare-library`)

## M8. macOS

The same items as M7 for macOS (`.dylib`, the toolchain and `lldb-dap`, packaging, a CI job, recordings judged
per platform), after Linux.

## Not tied to a milestone yet

- Deferred features and ideas: `SOMEDAY.md`
