# Proposal: dry-cleanup

## Why

A review of the codebase for duplication (2026-10-21, with the pragmatic engineer's verdict on each finding)
found little, but a handful of repeated pieces are worth one small, behaviour-free change.

- The numeric types are written out variant by variant at 21 `match` sites in `sema`, `ir` and `codegen-cpp`
  (about 290 lines). The ban on `_ =>` (`crates\README.md`, "Lints") is why the lists exist, and `sema` already
  exports one pattern macro for this kind of list, `place_only_types!`; `codegen-cpp` has two private copies
  (`numbers!` in `decl.rs`, `numeric!` in `mem.rs`).
- `clear_temp`, which empties the reference clone's `internal\temp` but keeps its tracked `temp.bin`, is copied
  word for word in `tools\callsite\callsite.py` and `tools\legacy_tests\run_legacy_tests.py`. It guards the
  read-only clone, so two copies that drift apart are a real risk.
- Smaller repeats: the `array_check` index expression in `codegen-cpp\src\place.rs` (twice); the methods of
  `Var` in `sema` and `ir`, which read only the shared `Shape`; `ProcKind` and `RuntimePart`, which `ir` defines
  again and `lower` copies variant for variant; the words of an `AS` clause collected six times in `sema\check`, and
  a procedure's record pushed twice in `check\proc.rs`; five screen-state tests in `driver\tests\cli.rs` that
  repeat the same build-and-run setup.

Why now: it is small, and the closing runs can show cheaply that nothing changed (unchanged C++ gives a
byte-identical executable with the build cache, so the pass cache runs nothing). It comes before
`m4-handlers-logging-checking`, which is planned but not started, so that a refactoring diff is not mixed into
feature work.

## What Changes

- `sema` exports two pattern macros beside `place_only_types!`: the numeric types with `_BIT`, and without it.
  The 21 hand-written lists and `codegen-cpp`'s two private macros use them. Every `match` stays exhaustive.
- `clear_temp` moves to `tools\legacy_tests\clone_lock.py`, the module about the tools' use of the clone (its
  docstring already describes emptying `internal\temp`); both tools import it.
- One helper in `codegen-cpp\src\place.rs` writes the flat index of an element against a descriptor's name.
- `Shape` gets `is_array`, `dim_count` and `bounds`; `Var` in `sema` and in `ir` delegates to them.
- `ir` re-exports `sema`'s `ProcKind` and `RuntimePart` in place of its own copies; `lower` stops converting them.
  `ir::Resume` stays its own: its `To` holds the IR's `LabelId`, not `sema`'s.
- `sema\check` gets a helper for an `AS` clause's words and its leading `_UNSIGNED`, and one for entering a
  procedure's record.
- `driver\tests\cli.rs` gets one helper that builds a program with the screen-state file and runs it with a keys
  file, used by its five screen-state tests.

Not changed: the `ExprKind` and `BinOp` lists (each leaves out a different variant, or there are too few to
matter), the `Ty`-to-text tables (each is a different vocabulary), the two whole-array copy emitters of
`codegen-cpp\src\arrays.rs` (they write two different texts of the old compiler), the syntax crate's node lists,
the two dumps' `ty` functions, and the two `Rng`s.

No behaviour changes: every diagnostic, typed tree, IR and C++ stays the same.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None. The change has no delta spec (`skip_specs`): it changes no behaviour.

## Impact

- `crates\sema`: `lib.rs` (the macros, `Shape`'s methods), `builtins.rs`, `check\{builtins,decl,io,mem,places,proc}.rs`.
- `crates\ir`: `lib.rs` (re-exports, `Var`), `lower.rs`, `validate.rs`, `dump.rs`.
- `crates\codegen-cpp`: `arrays.rs`, `builtins.rs`, `decl.rs`, `mem.rs`, `procs.rs`, `place.rs`.
- `crates\driver\tests\cli.rs`.
- `tools\legacy_tests\clone_lock.py`, `run_legacy_tests.py`; `tools\callsite\callsite.py`.
- `ir`'s public API: `ProcKind` and `RuntimePart` become re-exports of the same names, so paths such as
  `qb64rust_ir::RuntimePart::Sockets` (`driver\src\build.rs`) keep working.
