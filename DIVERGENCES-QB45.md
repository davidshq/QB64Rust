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
| Q-001 | `ON n GOTO` / `ON n GOSUB` with `n` above 255 | Error 5, "Illegal function call" (`n` must be 0 to 255) | Continues with the next statement, as for `n` = 0 or `n` beyond the number of labels; a negative `n` raises error 5 in both | Documented | 2026-10-07, user (`m2-core-builtins`) | `tests\corpus\slice\s29_on_goto` (`ON 258 GOTO`, 255, 256 in a loop; measured `verification\v20_i_on_goto`) |
| Q-002 | `SELECT CASE` in a recursive procedure: the copy of the test expression | Kept in the procedure's stack frame, so each call has its own and a recursive call between the test and a later `CASE` does not change it | One static copy per `SELECT` statement (QB64pe since its first version, `qb64pe.bas` "static … sc_"), so a recursive call overwrites it and the outer `SELECT` tests the inner call's value at its later `CASE`s (`verification\v04_accidental`) | **Believed; to measure with QB 4.5 under DOSBox** | 2026-10-07, user (`m2-core-builtins`: keep QB64pe's behaviour) | `tests\corpus\slice\s28_select_case` (`rec(1) = 2`; measured `verification\v20_h_select`) |
| Q-003 | Integer division by zero (`7 \ 0`, `7 MOD 0`) with an `ON ERROR` handler | Error 11, "Division by zero", trapped by the handler | Error 11 is fatal: "Runtime error: Division by zero" and the program ends, handler or not (`verification\v02b_idiv_zero`) | Believed | 2026-10-08, user (`study\00` §6: keep QB64pe's) | `tests\corpus\slice\s17_operators`, `s11_on_error` |
| Q-004 | INTEGER arithmetic beyond the INTEGER range inside an expression (`i% = 32767: PRINT i% + 1`) | Error 6, "Overflow" | Computed in 32 bits: prints 32768; only the store into an INTEGER wraps (`i% = i% + 1` gives -32768). LONG wraps too (`DIVERGENCES.md` D-001) | Believed | 2026-10-08, user (`study\00` §6: keep QB64pe's) | `tests\corpus\slice\s02_integer_wrap` |
| Q-005 | The initial bytes of a fixed-length string (`DIM f AS STRING * 4`, also in a `TYPE`) | Spaces | NUL bytes (0); assignment still pads with spaces (`verification\v03_storage`) | Believed | 2026-10-08, user (`study\00` §6: keep QB64pe's) | Not yet: `STRING * n` is not supported |
| Q-006 | `PRINT` with a comma when the program writes to the console (`$CONSOLE`), e.g. a QB 4.5 screen program built as `$CONSOLE:ONLY` | 14-column zones on the screen (QB 4.5 has no console) | Windows console: 10-column zones; Linux and macOS console: one space (read, not run); window and files: 14-column zones (`study\00` §6, `DIVERGENCES.md` D-011) | Documented | 2026-10-08, user (`study\00` §6: exactly as QB64pe; 14 everywhere to evaluate, `SOMEDAY.md`) | Not yet: console comma output is the runtime's |
| Q-007 | `STRING$(n, "")` | Error 5, "Illegal function call" | Reads the first byte of the empty string (`verification\v20_c_string_edges`) | Believed | 2026-10-08, user (`study\00` §6: keep QB64pe's) | Measured on the old compiler only (`verification\v20_c_string_edges`); no slice program yet |
| Q-008 | `CONST c = 2 ^ 3 ^ 2` (a chain of `^` in a `CONST`) | Believed left-associative, as at run time: 64 | Right-associative in a `CONST` only: 512 (64 at run time), with a compiler warning (`verification\v17_e_const`) | Believed | 2026-10-08, user (`study\00` §6: keep QB64pe's, add the warning) | `tests\corpus\slice\s18_const` |

Known differences not yet entered (QB64pe's, inherited; to be entered as they are decided or measured): the list in
`study\08-qb45-strict-mode.md` (numeric checks, literal typing, `DEF FN`, memory model, timing, hardware access).
