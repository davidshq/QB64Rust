# Tasks

Standing rules: CLAUDE.md. What this change does differently: every recording with `qb64pe.exe` (the family
`v60`) is batched in group 1, before the code; the recordings may change the delta specs and the design (D2
above all) before anything is built.

## 1. Measurements and records

- [x] 1.1 The family `v60` (design D5): `v60_a*`, `v60_b*`, `v60_c*`, `v60_d*`, recorded with `qb64pe.exe`
  (`--suite verification` with `--glob "v60_*"`), with its line in `verification\README.md`. Verify: each has its
  `.compile.txt` and, where it compiles, its `.out.txt`; `git -C ..\QB64pe status` is clean.
- [x] 1.2 The old compiler's C++ (`-z`) of `v55_x75`, a `v60_a` program with a handler and `v56_f3`, read for the
  start code's guard (D1) and the loop's uses of the place (D4). Verify: D1 and D4 name what was read, or are
  changed to it.
- [x] 1.3 D2's rule against every `v60_b` recording, `v24_x268` case 5 and `v55_x84`; the design and the
  arrays-and-types delta changed to the rule they fit. Record the decision of this change (the four forms, D-037)
  in `DECISIONS.md`. Verify: `openspec validate m4-left-marked --strict` passes.

## 2. `STATIC` bounds read by the start code (D1)

- [x] 2.1 `sema`: `StartBounds` in `Shape::Dynamic` (`lib.rs`), `Read` taken in `places.rs` for a shared variable,
  a main-module `CONST` and a FUNCTION; the dumps (`sema\src\dump.rs`, `ir\src\dump.rs`) and the IR's validation
  (`ir\src\validate.rs`) follow. Verify: front-end tests `static_bound_read_typed` and `_ir`; the `check-fail` test
  of a procedure variable in a bound still fails with its error.
- [x] 2.2 `codegen-cpp` `allocate_at_start` writes `Read` bounds through `value()` in the main module's scope.
  Verify: a `cpp` front-end test; `v55_x74`, `x75` and the `v60_a` programs pass (`--glob`).

## 3. `REDIM` of a shared array with a new array in its bound (D2)

- [x] 3.1 `places.rs`: the rule of 1.3 in place of the mark. Verify: front-end tests `redim_shared_implicit_typed`
  and `_ir`; `v24_x268`, `v55_x84` and the `v60_b` programs pass.

## 4. A `TYPE` that holds a `STRING` in conventional memory (D3)

- [x] 4.1 `places.rs` `var_address` stops marking `holds_strings`; `decl.rs` `alloc_dblock` and the conventional
  array allocation make the strings. `DIVERGENCES.md` D-037, `UPSTREAM.md` entry. Verify: `v56_x08`, `x26`, `x27`
  and the `v60_d` programs pass; a CLI test that builds `v56_a5` checks its first two lines against the record and
  `abc  3` after them; the `conventional-memory` front-end tests of the mark are changed to `check-ok`.

## 5. `FOR` over an element of a member array (D4)

- [x] 5.1 `blocks.rs` `for_var` returns the element's place. Verify: an `ir` front-end test shows three uses of the
  place in the loop over `r.m(idx(3))`; `v56_f3` and the `v60_c` programs pass.

## 6. Lists and closing

- [x] 6.1 `SOMEDAY.md`: the rows of the four forms in "Left marked by …" (rows of `v56_a5`, `v55_x74`, `x84`,
  `v56_f3`) are removed or narrowed to what stays marked (D-037's `GET`/`PUT`, dynamic members); rows of "QB64pe
  behaviours to review" for D1, D2 and D4. Verify: no row says a form of this change is "not supported yet" unless
  it still is.
- [x] 6.2 `tests\verification\known.list` and `pass.list`: the eight programs and every passing `v60` program move
  to `pass.list`; `v56_a5` (and any `v60_d` program that crashes in the old compiler) lists `D-037`. Verify: the
  tier 1 tests of the lists; `known.list` has no "not supported yet" reason; the demand report over the programs
  of `known.list` names nothing.
- [x] 6.3 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then
  `tools\close\close.py --check` passes. Then the counts of the test READMEs by `tools\legacy_tests\counts.py`, and
  `STATUS.md`'s "Next" names step 6 of the plan (the coverage review). Then `python tools\testtime\test_time.py`.
  Verify: `--check` passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
