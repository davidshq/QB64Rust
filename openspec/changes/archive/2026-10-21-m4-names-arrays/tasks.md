# Tasks

Standing rules: CLAUDE.md. What this change does differently: four parts in one change (user, 2026-10-20), each
built and tier-1-tested in its own group. The `v48` recording with `qb64pe.exe` is batched in task 1.1, before the
code.

## 1. Measurements and records

- [x] 1.1 The family `verification\v48_*` (design D7): `v48_a_scalar_array`, `v48_b_redim_scalar`,
  `v48_c_type_anywhere`, `v48_d_len_whole` and the `v48_x*` forms, recorded with `--suite verification`, with one
  line in `verification\README.md`. Where a recording differs from a delta spec or the design, the spec or design
  is changed to match in this task. That covers the pairings of D1 and D2, whether a second run of `REDIM i AS
  LONG` clears `i`, the `OPTION BASE` of a nested block, `LEN` of an erased array, and the three `LEN` errors.
  Verify: every program has its recorded output or error; `openspec validate m4-names-arrays --strict` passes
  after any edit of the specs.
- [x] 1.2 `DECISIONS.md`: a row for the change. It records the scope (the core declarations and `LEN` of a whole
  array; a whole array as a value, and the errors and suffixes of names, left for later; user 2026-10-20) and that
  a `TYPE` block anywhere defines a type of the whole program. Verify: the row names
  `openspec\changes\m4-names-arrays\design.md`.

## 2. A scalar and an array of one name

- [x] 2.1 `sema`: the `typed_array` mark in `variable` (`check\decl.rs`) goes (design D1). The array without `AS`
  beside a typed name in `declare_array` (`check\places.rs`) is made with the default type and takes the plain
  name from `array_plain` (design D2). Each pairing `v48_a` shows to behave in some other way keeps a narrowed
  mark. Verify: `tests\frontend\names_arrays_typed.bas` (`typed`: the plain scalar beside a typed array in the
  main module, in a SUB and beside a shared array; the default type under `DEFLNG`; the `v24_x126` pairing) and
  the pairings that stay marked in `names_arrays_errors.bas` (`check-fail`) pass; `cargo test -p qb64rust-sema`.

## 3. REDIM of a scalar in a procedure

- [x] 3.1 `sema`: the mark in `declare_each` (`check\decl.rs`) goes, and the item takes the scalar declaration
  `DIM` uses in a procedure (design D3). If `v48_b` shows that a run clears the scalar, that store is added and
  lowered. Verify: `names_arrays_typed.bas` gains `REDIM v(LEN(s)) AS LONG, i AS LONG` in a SUB and `REDIM i` in a
  `SUB … STATIC`; the upstream demand report no longer names "`REDIM i` without bounds in a SUB or FUNCTION";
  `cargo test -p qb64rust-sema`.

## 4. TYPE blocks anywhere

- [x] 4.1 `sema`: `collect_block_settings` (`check\mod.rs`) lists every `TYPE` block in file order, and
  `declare_types` (`check\places.rs`) declares them from that list; the marking arm of `type_block` goes (design
  D4). Verify: `names_arrays_typed.bas` gains a `TYPE` in a SUB used above it in the main module and as another
  SUB's parameter type, and a `TYPE` in an `IF` block; `v48_c`'s `OPTION BASE` case gives the recorded bounds;
  `cargo test -p qb64rust-sema`.
- [x] 4.2 The follow-on tests `tests\frontend\unsupported.bas`, `explicit_after_unsupported.bas` and
  `mem_follow_on.bas` take `TYPE cos … END TYPE` as their marked declaration, and their comments list the former
  example (design D6); their snapshots are updated. Verify: each snapshot still shows the real error before the
  mark and none after it; `cargo test -p qb64rust-driver --test frontend`.

## 5. LEN of a whole array

- [x] 5.1 `sema`: `len` (`check\builtins.rs`) makes `ExprKind::ArrayBytes(VarId)` for a whole array and the
  number for a whole member array, checked by `bytes_reference` with the old compiler's "UDT must have fixed size"
  for a `TYPE` with a `STRING` member, made by `LEN` itself (`_MEM` of one is no error, `v48_x11`). The "whole
  array" mark of `element` no longer reaches `LEN` (design D5). Verify: `names_arrays_errors.bas` holds the three
  `LEN` errors and the marked dynamic member array; `cargo test -p qb64rust-sema`.
- [x] 5.2 `ir` and `codegen-cpp`: `ArrayBytes` validated (`ir\src\validate.rs`), and written with
  `bytes_count` (`codegen-cpp\src\mem.rs`). Verify: `tests\frontend\len_whole_cpp.bas` (`cpp`: a static, a dynamic,
  a fixed-length-string and a user-type array, and a whole member array) pins the expressions; the C++ builds (a
  CLI test with `--ignored`, as the other `cpp` front-end tests that build); `cargo test -p qb64rust-ir`.

## 6. Programs and lists

- [x] 6.1 `arrays/t001_whole_array_core` off `tests\upstream\deferred.list` (left empty), and onto
  `tests\upstream\pass.list` with `arrays/size`, `arrays/test_mem_vs_len`, `arrays/test_mem_vs_len_classic` and
  `types/udt_array`. Then onto `tests\verification\pass.list`, off `known.list`: `v16_m4_type_in_sub`,
  `v18_h_type_in_sub`, `v18_d_index_none`, `v24_d_erase`, `v24_x126`, `v30_x47`, `v30_x67` and the `v48`
  programs. Verify: each passes with this compiler (`--list` of the programs moved); each verification program is
  named once across the two lists (tier 1); a tier-1 test that assumed `deferred.list` has a line still passes.

## 7. Close

- [x] 7.1 The closing runs: `tools\close\close.py --check` passes (it includes the demand reports, the shrink-only
  lists, the wording table and `counts.py`). Verify: the upstream demand report no longer names the plain scalar
  beside a typed array, `REDIM i` without bounds, `TYPE` blocks inside a SUB, or the whole arrays of `LEN`; the
  clone's `qb64pe` report no longer names "an array `args` beside a variable or array typed by `DIM … AS`".
- [x] 7.2 Documents. `STATUS.md`'s "Next". In `SOMEDAY.md`: the rows of `LEN` of a whole member array ("Left
  marked by `m3-mem`") and of `GET #1, , a` ("Left marked by `m3-console-files`") closed; a row for each pairing
  or form that task 1.1 left marked; the questionable behaviours kept, under "QB64pe behaviours to review" (a
  plain scalar beside a typed array silently of another type, a `TYPE` in a SUB visible everywhere). Verify:
  `openspec validate m4-names-arrays --strict` passes.
- [x] 7.3 `python tools\testtime\test_time.py`. Verify: its five files are named among those to stage.

## 8. After the review (user, 2026-10-21: the pragmatic engineer's recommendations)

- [x] 8.1 `verification\v48_x27`, `x28` (a `TYPE` block in a SUB whose `_DYNAMIC` member array is bounded by a
  `CONST`: the SUB's own, also beside a main-module one of the name) and `x29` (`LEN` of a whole array with a dotted
  name), recorded and on `tests\verification\pass.list`. Verify: `x27` and `x28` pass as built.
- [x] 8.2 `sema`: `len_whole` no longer leaves a dotted name to the normal path (a name whose first part is a `TYPE`
  variable is `member_array`'s); `len` and `len_whole` share `len_fixed_size`; the member path no longer wraps and
  unwraps its array (its check still comes before the size, which an array of strings has none of); `type_blocks` holds `ast::TypeBlock`s. `ir`: the validator holds `ArrayBytes` to elements of
  a fixed size (no `TYPE` with a `STRING` member or a dynamic member array). Verify: `x29` passes; the validator
  test `array_bytes`; tier 1 with fmt and clippy; the 219 listed verification programs that use `LEN`.
- [x] 8.3 `verification\v48_x30` (`LEN` of a whole member array of variable-length strings: no error in the old
  compiler, 32 for four elements), on `known.list`; `len_whole` marks it "not supported yet" instead of the
  `_MEM` error it gave, with a `SOMEDAY.md` row. Verify: `names_arrays_errors.bas` holds the mark (before the fix of
  8.2's member path it was an internal compiler error); tier 1; the `v48` programs.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
