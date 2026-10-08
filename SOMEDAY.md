# Someday

Features and ideas deliberately left out of the first versions of the rewrite. Each entry says why it was deferred
and where the details are. Nothing here is rejected; it is just not on the roadmap yet.

## Deferred QB64pe features

These are recent upstream QB64pe additions (not the user's own work). They add a lot of complexity to arrays, so they
wait until the core compiler works (roadmap M4 or later, `study\07-expert-panel.md`).

| Feature | Upstream origin | Why deferred | Details |
|---|---|---|---|
| The `_STATIC` / `_DYNAMIC` storage markers of TYPE member arrays (`$UNSTABLE:TYPEFIELDS`) | Petr, from 2026-06-12 | Still behind `$UNSTABLE`; woven through `evaluate`, `refer`, `setrefer`, `dim2`, `allocarray`. Plain member arrays (`AS LONG a(12, 15)` in a `TYPE`) no longer need `$UNSTABLE` and are on the roadmap (2026-10-07, `study\26` §4) | `study\02` "[member-array layer]" notes; `study\01` §6.4 |
| `_ARRAYCOPY` | Petr, 2026-09-12 | Weeks old; depends on the member-array layer; 954-line runtime (`array-copy.cpp`) | `study\01` §8.2, `study\03` §2.3 |
| Whole-array assignment `a() = b()` | Same work | Same layer | `study\02` §2.3 |
| `REDIM _RETAIN` (coordinate-preserving REDIM) | Same work | Same layer; `_PRESERVE` (linear) is kept | `study\02` §3.4 |

Features from the same period that are **kept** in scope: `$USELIBRARY`, `$ERRORLOCATION`, the GLFW runtime.

Test impact: some of the 211 `tests\compile_tests\arrays` tests exercise this layer. When the new compiler is first
measured, list them in `tools\legacy_tests\known_failures.txt` (reported as `KFAIL`) until the features land.

## Strict QuickBASIC 4.5 mode

Not planned (QB64pe has no such mode). What it would take: `study\08-qb45-strict-mode.md`.

## QB64pe behaviours to review

The new compiler does what QB64pe does, also where that looks wrong (user, 2026-10-08, `DECISIONS.md`); only crashes,
hangs, memory corruption and failed builds are fixed (`DIVERGENCES.md`). Each questionable behaviour found is listed
here, implemented QB64pe's way, for the user to review later. Changing one is a decision and a `DIVERGENCES.md` row.

| Behaviour (as implemented, as QB64pe) | Possible change | Source |
|---|---|---|
| The result-changing oddities kept on 2026-10-08: INTEGER arithmetic in 32 bits, `RESUME NEXT` in a `WHILE` condition looping forever, the `ELSEIF` placeholder, one `GOSUB` stack, a SUB with a raising argument doing nothing, a raising `CASE` item running its body, a bad index reading element 0, `HEX$` of a non-place 64-bit -1 giving `""`, `STRING$(n, "")`, `CSNG`/`VAL` not narrowed, a DOUBLE `ON n` narrowed to SINGLE, `_ROUND`/`VAL` beyond `_INTEGER64`, the static `SELECT CASE` copy, `CONST` `^` right-associative (with a warning) | Each could follow its definition or QB 4.5 | `study\00` §6 "Kept", `DIVERGENCES-QB45.md` |
| Comma zones on the console: 10 columns on Windows, one space on Linux and macOS | 14 columns everywhere, as on the window, in files and in QB 4.5. To evaluate: how many console programs print with commas, and whether a program that switches `_DEST` between window and console should lay out the same | Bug-compatibility decisions, 2026-10-08 (`study\00` §6, `DIVERGENCES.md` D-011) |
| `_BIT` and `_BIT * n` parameters: a compile error (QB64pe declares the parameter twice and fails its own C++ build, so no program uses them) | Support them. By reference across `_BIT` widths would need a design: QB64pe passes a `_BIT` variable only as a copy | `m2-numeric-types` measurements, 2026-10-08 (`study\00` §5, `verification\v21_x20`–`x24`; `DECISIONS.md`) |
| `STRING * n` reads n as 32 bits: 4294967297 is length 1 | An error for n beyond 2147483647 | Same |
| `VAL(s$, <unsigned type>)` drops the minus sign: `VAL("-1", _UNSIGNED LONG)` is 1 | Wrap, as a store does (4294967295), or an error | `m2-numeric-types` measurements (`verification\v21_d_builtins`) |
| An integer `CASE` item is compared as written, not converted to the selector's type: `CASE -1` misses a `~%` holding 65535 but matches a `~&` holding 4294967295 | Convert the item to the selector's type, as a float item is | Same (`v21_d_select`) |
| `FOR uq~&& = 1 TO 18446744073709551615~&&` runs no pass (the hidden limit is signed 64-bit) | Unsigned hidden copies for unsigned 64-bit variables | Same (`v21_d_for`) |
| A `_BIT * n` of up to 32 bits computes as its `int32`/`uint32` storage: `u3 * 1000000000` with `u3 = 7` is 2705032704, `b3 - u3` is 4294967286 | Compute as the believed `_INTEGER64` | Same (`v21_b_bit_stores`) |
| A `_BIT` value reads as `_INTEGER64`: an `_UNSIGNED _BIT * 64` holding 2^64-1 prints -1 | Read an unsigned 64-bit `_BIT` as `_UNSIGNED _INTEGER64` | Same |
| A FUNCTION named with a bare `` ` `` or `` ~` `` can be defined but not called ("Name already in use") | Allow the call | Same (`v21_x16`, `x17`) |
| `_UNSIGNED STRING` and `_UNSIGNED STRING * n` are accepted and mean `STRING` | An error, as for `_UNSIGNED SINGLE` | Same (`v21_a_decls`) |
| A suffixed literal or suffixed `CONST` outside its type's range is held as written and converted only by `PRINT`, `STR$` and type-keeping built-ins: `PRINT 300~%%` is 44, `300~%% + 0` and `l& = 300~%%` are 300, `CASE 300~%%` misses 44 | Convert it where it is read (44 everywhere), or a compile error | Same (`v21_a_literals`, `v21_d_const`, `v21_f_literal_uses`) |
| A `_BIT`-suffixed literal or constant is never narrowed: `` PRINT 9`3 `` is 9, `` CONST j~`3 = -1 `` prints -1 | Narrow it to its width and signedness | Same |
| A parameter declared `STRING * n` (or `t$n`) is a `STRING` parameter: only `LEN(t)` is n; the value, stores and comparisons see the caller's whole string | A real fixed-length parameter (cut and padded), or a compile error | Same (`v21_c_fixed_args`, `v21_f_fixed_param`) |

## Other ideas raised during the study

| Idea | Source |
|---|---|
| "Skip lines" debugger feature (no Debug Adapter Protocol equivalent) | `study\06-vscode-parity.md` |
| New compiler compiles the old `qb64pe.bas` as a stress test | `study\07-expert-panel.md` R10 |
| Redesign the runtime ABI (variables as pointers, `qbs` moving heap, `passed` bitmasks) once the old compiler is no longer a producer | `study\07-expert-panel.md` R6 |
| Fuzz the lexer and parser with a coverage-guided fuzzer: "no panic, exact round trip" is an ideal fuzz property. A seeded mutation test over the corpus comes first, in `m2-upstream-tests` (`study\22` §5) | `study\21-rust-review.md` §4 |
| An expression as the `INPUT` prompt (`INPUT p$; x`), a language extension; QB64pe and QB 4.5 accept only a string literal | Bug-compatibility decisions, 2026-10-08 (`study\10` §2.8) |
| The language server notices included files that change on disk while not open (`workspace/didChangeWatchedFiles`, or parsing the includers again on `didSave`); today the change shows at the includer's next edit | Review of `m2-language-server`, 2026-10-08 |
