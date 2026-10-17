# Tasks

Standing rules: `CLAUDE.md` (no subagents, nothing staged, backslashes through the tools, test cadence rule 9).

## 1. Measurements

- [x] 1.1 `verification\v38_a`–`v38_d` written and recorded with `qb64pe.exe`; run with this compiler before the
  change: `v38_c` equal, the other three differ as the proposal says.
- [x] 1.2 The facts in `study\00` §5 (a `v38` paragraph), `verification\README.md` (`v38*`), `DECISIONS.md`.

## 2. Code

- [x] 2.1 D1: a buffer per statement, cleared when grown, `_commit` after every `GET`.
- [x] 2.2 D2: the declared bytes into a member's block that holds them.
- [x] 2.3 D3: D-022 narrowed in `copy_array`; D4's comment.
- [x] 2.4 CLI tests of D-019, D-021, D-022 to the new outputs; `cpp` snapshots read and accepted. Verify:
  `v38_a`, `v38_b`, `v38_d` print their recorded output but for case 4 of `v38_b` (D-019) and cases 3 and 7 of
  `v38_d` (D-022); tier 1, fmt, clippy. Done: two snapshots changed (`typefields_cpp`, `whole_array_cpp`), each as
  the design says; tier 1 322, the four CLI tests that build pass (`get_after_a_failed_read_unpacks_zeros` new).

## 3. Slice program and close

- [x] 3.1 `s126_owner_copies`, recorded with `qb64pe.exe`, on `slice.list`; it passes. Its first recording
  differed between the two runs at the `GET` that never read (error 59: memory nobody wrote, other values each
  run), which made the zeros a divergence (D-021) and not QB64pe's way: those two cases went to a CLI test.
- [x] 3.2 Documents: `DIVERGENCES.md` D-019, D-021, D-022; `tests\corpus\README.md`; the merged specs at the
  archive; `STATUS.md`. `SOMEDAY.md` holds nothing on these behaviours.
- [ ] 3.3 Closing runs are shared with the next two changes (one close for the three, rule 9); the programs
  touched ran here (`--glob s126*`, the four CLI tests, `v38` by hand).
