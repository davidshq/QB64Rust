# Proposal: m4-checker-groups

## Why

`Checker` in `crates\sema\src\check\mod.rs` holds every piece of the checker's state as one flat list of fields
(84 on 2026-10-20, 72 when the ninth review counted them). Among them are flags a caller sets around a call so
that a callee several frames down behaves differently: `len_place` (a whole user-type place is a place, not a
value, when `LEN`, `VARPTR` or a `_MEM` argument asks) and `mem_store` (a built-in that gives a `_MEM` block may
stand only as the value stored into a `_MEM` place). M4's targets (`qb64pe.bas`, and the upstream and
verification programs that remain) will add more such state. The ninth review (`study\archive\30` §6) and
`ROADMAP.md` ask for the regrouping at the start of M4, before that work adds to it. This is that start.

## What Changes

- The fields of four concerns move into four structs that `Checker` holds:
  - **the position in the file**: what follows the file as it is read (the default types, `OPTION BASE`,
    `$DYNAMIC`, `$CHECKING`, the `STATIC` body, the follow-on rule);
  - **the program's settings**: what the program's metacommands and options set wherever they stand, read after
    the walk (console, window, `$ASSERTS`, `$EXEICON`, `$VERSIONINFO`, `_ECHO` lines, runtime parts, the
    `OPTION _EXPLICIT` pair, folding);
  - **the statement being checked**: its error flag, the names it resolved, the allocation it may not run first,
    the place of the `CONST` being checked;
  - **the dimension solver**: the facts, the open arrays, the deferred `_ARRAYCOPY` checks.
- `len_place` and `mem_store` stop being fields. How a value is used is an argument of the expression check, given
  where these flags are set today and read where they are read today.
- The symbol tables and the rest stay on `Checker`.

No behaviour changes: every diagnostic, typed tree, IR and C++ stays the same, which tier 1 (the `typed`, `ir` and
`cpp` snapshots among it) and the closing runs show.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None. The change has no delta spec (`skip_specs`): it changes no behaviour.

## Impact

- `crates\sema\src\check\*.rs`: every module of the checker, mechanically (`self.option_base` becomes
  `self.at.option_base` and so on); `expr.rs`, `places.rs`, `builtins.rs`, `mem.rs`, `io.rs` and `mod.rs` for
  the argument that replaces `len_place` and `mem_store`.
- Nothing outside `sema`; no public API of `sema` changes.
