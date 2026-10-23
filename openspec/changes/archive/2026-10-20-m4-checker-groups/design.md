# Design

## Context

`Checker<'a>` (`crates\sema\src\check\mod.rs`) is built in `check_with` with all its fields listed, and every
module of `crates\sema\src\check\` adds methods to it (`impl Checker<'_>`). Of its 84 fields, two are set around
one call so that a callee reads them several frames down:

- `len_place: Option<Span>`. Seven places set it to the span of the argument they check, around `self.expr(…)`:
  `LEN`, the `_MEM` argument (`mem_arg`), and the block and bytes arguments of a statement (`block_arg`,
  `bytes_arg`), all in `builtins.rs`; `VARPTR`/`VARSEG` (`var_address`, `places.rs`); the string of an I/O
  statement (`io.rs`); and one in `mem.rs`. `Checker::load` (`places.rs`) reads it: a whole user-type place loaded with exactly that
  span is the place itself, not the error "a whole `TYPE` value cannot be used here". `load` is called only from
  `expr` and `call` (`expr.rs`), each time with the span of the node being checked, so the flag reaches the
  argument's own node and no node inside it (a `LEN` in an index sets its own and restores the outer one).
- `mem_store: bool`. `Checker::assign` (`mod.rs`) sets it around `self.expr(value)` when `gives_block(value)`
  holds, or when the target is a user type and the value names `_MEMGET`. `builtin` (`builtins.rs`), `mem_fn`
  and `mem_get_fn` (`mem.rs`) take it (`std::mem::take`) as their first act. `gives_block` and `names_word`
  (`mod.rs`) are true only for a `Call` or `NameRef` node naming one of `MEM_RESULTS` or `_MEMGET`, and `expr`
  sends such a node straight to `call` or to `builtin`, so the first taker is the value's own node.

`no_join` is the same kind of flag (set by `unjoined` around a whole statement part, read by `implicit_array`
deep in it). It stays a field, in the statement group: it covers every node of the part, not one.

## Goals / Non-Goals

**Goals:**

- Four structs by concern, so that a reader of a method sees which kind of state it touches by the path
  (`self.at.option_base`, `self.settings.console`).
- `len_place` and `mem_store` replaced by an argument of the expression check.
- Byte-for-byte the same output: diagnostics, symbol table, typed tree, IR and C++.

**Non-Goals:**

- Splitting `impl Checker` into separate types, or moving methods between modules.
- Grouping the symbol tables (procedures, labels, constants, types, scopes). They are the checker's subject, not
  a mode, and grouping them would touch the same lines again without making any callee's state smaller.
- Making `no_join` or `follow_on` an argument: each holds for many nodes, and as an argument it would go through
  every `expr` call of a statement.

## Decisions

### D1. The four groups

All four are defined in `crates\sema\src\check\mod.rs` beside `Checker`, with the fields' doc comments moved
with them, and `Checker` holds one field of each type.

| Field of `Checker` | Struct | Fields moved |
|---|---|---|
| `at` | `FileState`: what follows the file as it is read | `defaults`, `option_base`, `dynamic_mode`, `checking`, `static_scope`, `follow_on` |
| `settings` | `Settings`: what the program's metacommands and options set wherever they stand | `console`, `console_only`, `window_hidden`, `asserts`, `exe_icon`, `version_info`, `version_lines`, `echoes`, `runtime_parts`, `explicit`, `explicit_array`, `fold` |
| `stmt` | `StmtState`: the statement being checked | `stmt_error`, `names`, `no_join`, `const_at` |
| `dims` | `DimSolver`: the number of dimensions found after the walk | `dim_facts`, `dims_open`, `copy_checks` |

The review placed `explicit` with the position in the file. It is not one: it is found once, before pass 1, for
the whole program (`has_option` in `check_with`; measured, `verification\v19_explicit_*`), so it goes with the
settings, and `explicit_array` with it. `follow_on` holds from the first marked declaration to the end of the
file, so it follows the file.

Each struct gets a `new` (or `Default` where every field's start is its type's default): `FileState::new()` starts
`defaults` at `DEFAULTS_AT_START` and `checking` at `true`; `Settings::new(fold)`.

A method that touches only one group keeps taking `&mut self`; borrowing a group alone where `self` is borrowed
elsewhere is the only reason to pass `&mut self.at` instead, and is done only where the compiler asks for it.

### D2. How a value is used is an argument

`expr.rs` gets

```rust
/// How the value being checked is used, for the node itself and no node inside it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Use {
    /// A value: a whole user-type place is an error.
    Value,
    /// A place whose size or address is taken (`LEN`, `VARPTR`, a `_MEM` argument, a statement's place): a whole
    /// user-type place is the place.
    Place,
    /// The value stored into a `_MEM` place, or into a user-type place from `_MEMGET`: a built-in that gives a
    /// block may stand.
    Stored,
}
```

and `expr(node)` becomes `self.expr_as(node, Use::Value)`. `expr_as` passes the `Use` to `call` and from there to
`builtin`, `mem_fn`, `mem_get_fn` and `load`, and to the `load` and `builtin` calls in its own `NameRef` and
`Field` arms. It passes `Use::Value` everywhere else: to `Paren`'s inner node, the operands of `Prefix` and
`Bin`, the arguments of a user function, every index. `load` tests `use_ == Use::Place` where it tested
`len_place == Some(span)`; `builtin`, `mem_fn` and `mem_get_fn` test `use_ == Use::Stored` where they took
`mem_store`. The seven setters of `len_place` call `self.expr_as(node, Use::Place)` and `assign` calls
`self.expr_as(value, Use::Stored)`.

`len_place` matched by span and `mem_store` was taken by the first taker, so both reached the node itself and
nothing inside it (Context). The argument says that directly. The only path where the two could differ is a
user `FUNCTION` whose name `names_word` reads as `_MEM`, `_MEMNEW` and the rest (the state would have reached the
first such built-in among its arguments); task 2.3 checks whether such a program can be written at all, and if it
can, adds it as a front-end test of what the old code did.

Alternatives: keeping the flags and wrapping them in a `with_use(…, |c| …)` helper (fewer lines touched, but the
state stays ambient and the review's point stays open); a `Use` field of a typing-context struct (the same state
under another name).

## Risks / Trade-offs

- The rename touches most lines of `sema\check` that name these fields. It is done by a script of plain
  `self.<field>` to `self.<group>.<field>` replacements over the checker's modules, each field named in D1, and
  the compiler finds what the script missed (a field used through `c.` in `check_with`, or in a closure).
- A typo in the routing of `Use` would show as a changed diagnostic or a changed typed tree: the front-end tests
  of `LEN`, `VARPTR`, `_MEM`, `_MEMGET` and `PRINT #`'s strings, and the `typed` snapshots, cover each setter.
  The closing runs on top hold every output of the slice, upstream and verification lists.
