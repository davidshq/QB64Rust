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

## Other ideas raised during the study

| Idea | Source |
|---|---|
| "Skip lines" debugger feature (no Debug Adapter Protocol equivalent) | `study\06-vscode-parity.md` |
| New compiler compiles the old `qb64pe.bas` as a stress test | `study\07-expert-panel.md` R10 |
| Redesign the runtime ABI (variables as pointers, `qbs` moving heap, `passed` bitmasks) once the old compiler is no longer a producer | `study\07-expert-panel.md` R6 |
| Fuzz the lexer and parser with a coverage-guided fuzzer: "no panic, exact round trip" is an ideal fuzz property. A seeded mutation test over the corpus comes first, in `m2-upstream-tests` (`study\22` §5) | `study\21-rust-review.md` §4 |
| A policy for bumping the pinned Rust toolchain (1.88, about 15 months old on 2026-10-04) | `study\21-rust-review.md` §4 |
| `cargo-deny` for licences and advisories, once a dependency beyond `insta` and `serde_json` is added | `study\21-rust-review.md` item 7 |
