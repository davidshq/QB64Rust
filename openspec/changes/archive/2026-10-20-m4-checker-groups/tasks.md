# Tasks

Standing rules: `CLAUDE.md`. No delta spec (`skip_specs`); no `DECISIONS.md` row (the regrouping is
`ROADMAP.md`'s M4 item and the ninth review's §6, already decided). The rename script lives in the scratchpad,
not in the repo.

## 1. The four groups (design D1)

- [x] 1.1 `FileState`, `Settings`, `StmtState` and `DimSolver` in `crates\sema\src\check\mod.rs`, with the fields
  of D1's table and their doc comments; `Checker` holds `at`, `settings`, `stmt`, `dims`; `check_with` builds
  them. Check: `cargo build -p qb64rust-sema` fails only at the uses of the moved fields.
- [x] 1.2 The uses renamed (`self.<field>` and `c.<field>` to the group's path) across `crates\sema\src\check\`,
  by a script of the field names of D1; what it misses fixed by hand. Check: `cargo test -p qb64rust-sema`, and
  no snapshot changed (`git status` shows no `.snap` file).

## 2. How a value is used (design D2)

- [x] 2.1 `Use` and `expr_as` in `expr.rs`; `expr` calls `expr_as(node, Use::Value)`; `call`, `load`,
  `builtin`, `mem_fn` and `mem_get_fn` take a `Use`, routed as D2 says.
- [x] 2.2 The seven setters of `len_place` call `expr_as(…, Use::Place)` and `assign` calls
  `expr_as(…, Use::Stored)`; the fields `len_place` and `mem_store` removed. Check: `grep -rn
  "len_place\|mem_store" crates\sema` finds nothing; tier 1 passes with no snapshot changed.
- [x] 2.3 Whether a user `FUNCTION` can carry a name `names_word` reads as one of `MEM_RESULTS` (with and without
  `$NOPREFIX`): try it with `qb64rust -z`. If one is accepted, a front-end test `tests\frontend\mem_store_user_fn`
  (`typed`) holds what the old code did, run before 2.1 (its snapshot taken on the old code). Check: the answer
  written in this task.
  **Answer (2026-10-20, on the code before 2.1):** no such program can be written. `FUNCTION _MEM (x)`, and the
  same for `_MEMELEMENT`, `_MEMNEW`, `_MEMIMAGE`, `_MEMSOUND` and `_MEMGET`, each stored into a `_MEM` place, is
  "names starting with `_` are reserved"; with `$NOPREFIX` the same, beside "not supported yet" for the
  metacommand, and a name without the `_` (`MEM`) is not one `names_word` reads. No test added.

## 3. Close

- [x] 3.1 `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`.
- [x] 3.2 `STATUS.md`'s "Next": this change done.
- [x] 3.3 The closing runs (`python tools\close\close.py --plan`, then `--check`).
- [x] 3.4 `python tools\testtime\test_time.py`.
