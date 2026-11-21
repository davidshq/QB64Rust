# Roadmap

The milestones still open and the significant work remaining. Add newly discovered work; mark items complete as
they land, and move a milestone to `ROADMAP-COMPLETED.md` when it is declared done (M0 to M7 are there). The
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

## M9: the web target

Programs built for the browser from the same C++ and the same libqb, compiled with Emscripten to `wasm32`
(user, `DECISIONS.md` 2026-11-19). The number M8 stays with macOS, which is postponed. The study behind it is
`study\51`: §5 has the design, §8 the steps in detail. Each step ends at a point where the work can stop and
something still stands. Each step is an OpenSpec change of its own.

| Step | What | Ends with |
|---|---|---|
| W0, `wasm-console` | The Emscripten SDK as a toolchain, libqb built for `wasm32` by edits, `--target wasm` in the driver for `$CONSOLE:ONLY` programs, the test runner's `--platform wasm` under Node | The share of the console verification programs that pass, and a list of where the rest differ |
| W0.5 | `_OFFSET` and `_MEM` at 4 bytes on this target, in `sema` and `codegen-cpp`; libqb's cross-thread flags as `std::atomic` | The programs W0 rejects for `_OFFSET` or `_MEM` build and are judged |
| W1 | A window in the browser: the GLFW port, the browser's main loop, a WebGL 2 presentation path, Web Audio, the virtual file system with storage, the list of words the target does not have | Civil War Strategy and the `screen/` corpus running in Chromium, judged by the screen state |
| W2 | The folder a user uploads, `qb64rust serve`, the SDK download for users, "Run in browser" in the extension, a reference chapter | A user can build a game and put it on a host |
| P1 (not decided) | Phones through the browser: touch as a mouse, a key bar, the keyboard for `INPUT`, a web app manifest | Civil War Strategy playable on a phone from a link |

Open, to decide after W1: a one-thread build for hosts that cannot send the two headers threads need
(`study\51` §5.1, "What a one-thread build would cost"). Store packaging and native phone apps (`study\51` §7,
P2 and P3) are not part of this milestone.

## Not tied to a milestone yet

- Deferred features and ideas: `SOMEDAY.md`
