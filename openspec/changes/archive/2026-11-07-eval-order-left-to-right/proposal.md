# Proposal

## Why

QB64pe writes C++ in which several parts of one expression, or the arguments of one call, are evaluated in an
order that C++ leaves to the C++ compiler. On Windows QB64pe builds with Clang (llvm-mingw), which evaluates them
left to right. On Linux it uses `g++`, which on x86-64 often evaluates them right to left. QB64Rust writes the same
C++, so a program can print one thing on Windows and another on Linux. With a FUNCTION `f` that adds 1 to a shared
counter, `PRINT f(1) - f(2)` prints `-1` on Windows and `1` on Linux.

The user decided on 2026-10-28 that the code generator fixes the order: left to right on every platform, as
Windows does today (`DECISIONS.md` 2026-10-28; `ROADMAP.md`, "After M7"). It is to be done as one pass that covers
every place where the C++ leaves the order open, not program by program, so that code no test reaches is covered
too. It lands on branch `after-pr25` before that branch is merged into `main`.

## What Changes

- The C++ emitter (`crates\codegen-cpp`) finds every group of parts whose C++ leaves their order open: the
  arguments of a call and the operands of an operator. Where two parts of a group could give another result in
  another order, it evaluates the earlier ones into temporaries first, in Windows' order. The rule is decided in
  one place, by an exhaustive match over the IR's expression and statement kinds, so a new kind cannot be added
  without being classified.
- A part counts when it can change state (a call of a user FUNCTION or a library procedure, `RND`, a read from a
  file or device, and other built-ins on a list) or raise an error. Where no two parts of a group conflict, the C++
  stays exactly as it is now, so most programs compile to the same C++ as before. `SWAP` (D-054, done in M7)
  becomes a case of the general rule.
- Orders that C++20 already fixes are kept: `AND ALSO`, `OR ELSE`, `_IIF`, the value of a number stored through a member
  before its target, and the statements the emitter already writes one after another (each item of a `PRINT`).
  Nothing is moved out of a branch that may not run.
- New verification family `verification\v69_*`, recorded with `qb64pe.exe` on Windows: one program per kind of
  place, including places no existing program reaches.
- The Linux sidecars that only record the other order are deleted: the corpus programs `s106_asserts`,
  `s108_asserts_off_debug`, `s22_store_rules` and `s78_choice_min_shifts`, and the verification programs
  `v18_c_byref`, `v30_k_field_errors`, `v30_l_lset_rset`, `v33_a03_assert_off` and
  `v33_a04_assert_evaluated_on`. Two are cut to their other lines: the `<t>` lines of `v30_g_iif_min_max` go, and
  `s86_field_lset` keeps only its line 20, which differs for another reason (design, "Found while building").
- Two of the programs the request names differ on Linux for other reasons and keep their sidecars:
  - `s12_error_in_print`: PRINT evaluates each item in its own C++ statement. What differs is where the runtime
    error's message falls among the console's buffered output.
  - `s136_errorline`: its line 28 raises `ERROR 13` only under `$IF WIN`.

  The Linux run in task 4 confirms both.
- **On Linux, QB64Rust no longer behaves as QB64pe's Linux build** where the order shows. A new
  `DIVERGENCES-QB64PE.md` row records this.

## Capabilities

### New Capabilities

- `language/evaluation-order`: the order in which the parts of an expression and the arguments of a call or
  statement are evaluated, which parts it applies to, and that it is the same on every platform.

### Modified Capabilities

- `compiler/platforms`: "A program behaves as QB64pe's on its platform" gains an exception: where QB64pe's Linux
  build evaluates parts in another order than its Windows build, a Linux build follows the Windows order.

## Impact

- **Code.** `crates\codegen-cpp`: a new module for the groups and the rule, hooks in `value.rs` (`value`),
  `place.rs` (`load_place`, `place_ref`, `bytes_of`) and `stmt.rs` (`op`), and `builtins.rs`'s `SWAP` folded into
  the general rule. The IR (`crates\ir`) does not change, except that its module documentation states the order as
  a guarantee.
- **C++ output.** The generated C++ changes only where a group has two conflicting parts. The call-site check
  (`tests\callsite`, one statement with plain arguments) is not expected to change. Programs whose C++ changes run
  as before on Windows.
- **Tests.** The new `v69` family; the sidecars and `known.linux.list` rows above deleted; unit tests in
  `codegen-cpp`.
- **Records.** A `DIVERGENCES-QB64PE.md` row, a `DECISIONS.md` row, the `SOMEDAY.md` row removed, the `ROADMAP.md`
  item ticked, `STATUS.md`'s "Next", the line in `verification\README.md` about these programs, and the merged
  specs.
