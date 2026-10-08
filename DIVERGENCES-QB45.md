# Divergences from QuickBASIC 4.5

Where QB64Rust behaves differently from QuickBASIC 4.5. The companion of `DIVERGENCES.md`, which registers the
differences from the old compiler (QB64pe); QB64Rust follows QB64pe where the two differ unless a row there says
otherwise, so most rows here are differences QB64Rust inherits from QB64pe. A row is added when a behaviour is
decided and QB 4.5 is known or believed to differ (decided 2026-10-07, user).

There is no QB 4.5 oracle on the development machine yet (DOSBox is installed, QB 4.5 is not). Each row says where
its QB 4.5 behaviour comes from: **documented** (Microsoft's QB 4.5 manuals or help), **believed** (from memory of
how QB 4.5 works, not checked), or **measured** (run under DOSBox, with the program in `verification\`). Rows that are
not measured are to be checked once QB 4.5 runs under DOSBox (`study\08` "Effort estimate": the oracle a strict mode
would need).

| Id | Behaviour | QuickBASIC 4.5 | QB64Rust (= QB64pe unless noted) | Source of the QB 4.5 column | Decided | Pinned by |
|---|---|---|---|---|---|---|
| Q-001 | `ON n GOTO` / `ON n GOSUB` with `n` above 255 | Error 5, "Illegal function call" (`n` must be 0 to 255) | Continues with the next statement, as for `n` = 0 or `n` beyond the number of labels; a negative `n` raises error 5 in both | Documented | 2026-10-07, user (`m2-core-builtins`) | Slice program of `m2-core-builtins` |
| Q-002 | `SELECT CASE` in a recursive procedure: the copy of the test expression | Kept in the procedure's stack frame, so each call has its own and a recursive call between the test and a later `CASE` does not change it | One static copy per `SELECT` statement (QB64pe since its first version, `qb64pe.bas` "static … sc_"), so a recursive call overwrites it and the outer `SELECT` tests the inner call's value at its later `CASE`s (`verification\v04_accidental`) | **Believed; to measure with QB 4.5 under DOSBox** | 2026-10-07, user (`m2-core-builtins`: keep QB64pe's behaviour) | Slice program of `m2-core-builtins` |

Known differences not yet entered (QB64pe's, inherited; to be entered as they are decided or measured): the list in
`study\08-qb45-strict-mode.md` (numeric checks, literal typing, `DEF FN`, memory model, timing, hardware access) and
the QB 4.5 notes in `study\00` §6 (NUL-filled fixed-length strings, console comma zones).
