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
real programs and fuzzing beside them; the crash-class fixes and new language work as users ask; the fixes of
`UPSTREAM.md` offered to QB64pe as goodwill.

This file was written again on 2026-10-12 from `study\00` §2, `STATUS.md` and `DECISIONS.md`: the earlier file
was never committed and was no longer on disk.

## M6. Runtime modernisation

- [ ] The ABI owned by the new compiler; error model, string heap and threading redesigned behind golden tests.
  This starts as a second backend crate beside `codegen-cpp`, which stays pinned to libqb's ABI and to the
  call-site check; `codegen-cpp` is not modernised in place (`study\archive\30` §7, user 2026-10-16)

## Not tied to a milestone yet

- Deferred features and ideas: `SOMEDAY.md`
