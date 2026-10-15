# Tasks

Order: group 1, then 2 to 5 in order, then 6. Each of groups 2 to 5 ends with `cargo fmt --check`, `cargo clippy
--workspace --all-targets -- -D warnings` and `cargo test` (tier 1) green and with its tests, lists and
`crates\README.md` lines in place, so the change can stop after any of them.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools or
a script file; `python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done.
**Test cadence** (`CLAUDE.md` working rule 9): while working, tier 1 and only the programs the work touches
(`--glob`, `--category`, or a list with `--pass-cache target\pass-cache`), each run's output sent to a log under
`target\`; tier 2, the run without folding and the full suites run once, in group 6. A behaviour of the old
compiler that looks questionable is implemented its way and listed in `SOMEDAY.md` ("QB64pe behaviours to
review"), unless the user decided otherwise (D-019, D-020, D-004 widened).

## 1. Measurements and decisions

- [x] 1.1 Measure with `qb64pe.exe` before the proposal (`verification\v35_*`, 69 programs, recorded 2026-10-14):
  `v35_a` a member of a variable, `v35_b` members of array elements and the parent's `REDIM`/`ERASE`/`SWAP`/copy,
  `v35_c` every kind of element, `v35_d` procedures, `v35_e` statements and whole-array files, `v35_f` `_STATIC`
  and unmarked members beside `_DYNAMIC`, `v35_g`–`g3` runtime errors, `v35_h` the metacommand, `v35_x01`–`x59`
  one accepted or rejected form each. Verify: every program has its `.compile.txt` and `.out.txt`.
- [x] 1.2 Record the facts and decisions: a `v35` paragraph in `study\00` §5 (the flag's position, the markers,
  the descriptor and its life, the layout, files, the parent's place, the three misuses); `v35*` in `CLAUDE.md`'s
  layout table; `DECISIONS.md` rows (the user's three answers of 2026-10-14: one change for the family, measured
  before the design, the three misuses fixed); `DIVERGENCES.md` D-019 (`GET` into a member with other bounds),
  D-020 (a member too large) and D-004 widened (a store through a parent index outside). Verify:
  `check_repo.py --untracked` is clean.

## 2. The metacommand and the markers

- [x] 2.1 `$UNSTABLE` (design D1): the parser takes the metacommand; the block walk records `type_flags`; the
  errors of the other forms; the warning of `MIDI` and `HTTP` with `-w`. Verify: `tests\frontend\unstable_forms.bas`
  (`check-fail`, with the forms of `v35_x10`–`x12`, `x14`, `x45`) and a `check-ok` program with `v35_h`'s and
  `x13`, `x15`, `x47`'s forms; a CLI test for the warning.
- [x] 2.2 The markers (design D2): every position the old compiler reads, `Member::dynamic`, the errors; `_STATIC`
  compiles as an unmarked member; a `_DYNAMIC` member stays "not supported yet" until group 3. Verify:
  `tests\frontend\typefields_markers.bas` (`check-fail`: `x05`–`x09`, a marker without the flag; `x60`, a marker
  before an `AS type` list, measured and added by the review of the group);
  `member_arrays_errors.bas` without its marker lines and its snapshot redone; `v35_f` without its `_DYNAMIC`
  member and `x34`, `x40`, `x44`, `x53` compile and give their recorded output.
- [x] 2.3 Upstream: the programs of `deferred.list` that need only the flag and `_STATIC` (the `.err` programs
  `t028`–`t044`, `t233`, `t234`, `t_qb64pe_refactor_inline_reset_*`; each checked by itself) leave it, and those
  that pass join `pass.list`. Verify: they run by themselves; tier 1's list tests pass.

## 3. `_DYNAMIC` members of variables and elements

- [x] 3.1 Owner types and the layout (design D3): `owns_desc`; `ty_size`, `member_offset` with the aligned layout
  for owner types; `file_layout`. Verify: `tests\frontend\typefields_cpp.bas` (`cpp`) shows `x52`'s sizes and
  offsets; every existing `cpp` snapshot is unchanged.
  Done 2026-10-15: `owns_desc` in `sema` beside `holds_strings`; the layout is one function, `decl.rs` `layout`
  (offsets and size), which `ty_size`, `member_offset` and `parts` read; `file_layout` is `file_size` (the
  declared, packed size; used by task 5.1). The snapshot shows `b` at 16, `z.i` at 8, `z.d` at 32, sizes 24 and
  40; no other snapshot changed.
- [x] 3.2 The IR (design D5): `Allocate` and `Release` take an `ArrayRef`, validation of a `Member` there, the
  dumps. Verify: `tests\frontend\typefields_ir.bas` (`ir`) shows `Allocate Retain` and `Release` of a member;
  existing `ir` snapshots unchanged.
  Done 2026-10-15; the typed tree's `StmtKind::Allocate` and `Release` changed with them. Validation: a member
  there must be dynamic, an `Allocate` of one has bounds, a `Clear` of one is a problem, an element of one takes
  any number of indexes (IR unit test `dynamic_member_arrays`). The IR dump marks a member array `dynamic` in the
  list of types; `Release` of a member may raise (its place may be an element).
- [x] 3.3 The descriptor and the life functions for numbers and `STRING * n` (design D4, D6 first three rows,
  D9, D10): `udt_new_N`, `udt_free_N`, `udt_clone_N`; variables, `STATIC` locals, locals at each return; `REDIM`,
  `_PRESERVE`, `_RETAIN`, `ERASE` of a member through `allocate_with` and `release`; elements, `LBOUND`, `UBOUND`
  through the guard; the member as an array argument; the parent's place evaluated once. Verify: `v35_a`, `v35_d`,
  `v35_g`, `x01`–`x04`, `x17`, `x21`, `x26`, `x29`–`x33`, `x36`, `x38`, `x58`, `x59` print their recorded output
  (`x58` 3. by D-004); the `cpp` test of 3.1 gains the functions of one owner type.
  Done 2026-10-15 (`codegen-cpp\src\owner.rs`), for every kind of element, not only numbers and `STRING * n`:
  the element functions are written once for any type, so much of group 4 came with it (below). Every program
  listed prints its recorded output (`x03` its compile error at the `REDIM` of the unmarked `w`), `x58` 3. ` 0 `
  by D-004; `v35_a`, `v35_d`, `v35_g` were run as copies without `_ERRORLINE`, which this compiler does not
  compile yet, compared without the line numbers it prints. Found and settled on the way:
  - `x17` needs a `CONST` in the first bounds of a dynamic member (the spec's requirement; design D2 said "not
    supported yet"): the block keeps 0 TO 0 per dimension in pass 1 and reads the bounds in pass 2 when it
    reaches the block, below the constants above it (`places.rs`, `type_const_bounds`). A member that is not
    dynamic keeps the rule of design D2.
  - Names: the functions are `qbr_udtN_new`, `_free`, `_zero`, `_clone`, and per dynamic member
    `qbr_udtN_mM_blank`, `_make`, `_release`, `_free`, `_copy`, `_guard`, `_bound`, `_desc`, in `global.txt` with
    their prototypes first. `_copy` is the old compiler's `AppendDynUDTDescCopy`: a source that is not allocated
    leaves the target as it is.
  - `_PRESERVE` and `_RETAIN` of an array of an owner type copy each kept element deep (`qbr_udtN_clone`), then
    release the old ones, as the old compiler's `AppendDynUDTDescCopy` does at `qb64pe.bas` 14863; design D6
    said the bytes are moved. Same output, no moved slot to zero.
  - Not measured, chosen: a `REDIM` of a member to another number of dimensions gives a new descriptor whatever
    the mode (no element kept with `_PRESERVE`); `CLEAR` treats an owner variable as `ERASE` of a static array
    does (dynamic members released, then error 9 until a `REDIM`).
  - Group 5 forms that would have compiled to wrong code are "not supported yet" until then: `LEN` of an owner
    value, `PUT` and `GET` of one or of a whole array of one (`put_get_write_read_UDT_fixed_string` stays on
    `deferred.list`), `_MEM`, `_MEMELEMENT` and `_OFFSET` of a whole dynamic member array, `_MEM` of an element of
    one. `x.v() = y.v()` and `x.v() = a()` (whole-array assignment, upstream `t003`) are "not supported yet", as
    `a() = b()` is.
  - Already printing their recorded output (tasks of group 4, not ticked): `v35_b` and `v35_c`. `v35_e` waits on
    `READ` and `MID$` into an element of a member array (task 4.2).
  - Tier 1 needed the upstream programs that now compile on a list: 77 compiled cleanly, 76 passed by themselves
    and joined `pass.list` (part of task 5.4; 287 there; corrected with group 4: `t266` and `t267` were among them
    but stop at the `PUT`/`GET` mark of group 5, so they went back to `deferred.list` and 285 are on `pass.list`,
    all passing); `t268` and `t269` left
    `known_unsupported_rejections.list` (their whole-array `PUT`/`GET` of a type with a `STRING` is now the old
    compiler's error).
- [x] 3.4 Slice programs `s114_dyn_members` (`v35_a`, `v35_g`, `x58`) and `s115_dyn_member_procs` (`v35_d`),
  recorded with `qb64pe.exe` (the line of D-004 recorded from a program without it), on `slice.list`. Verify: both
  pass (`--glob`).
  Done 2026-10-15: line 20 of `s114` recorded from the program with `ERROR 9` in place of `p(9).v(5) = 55`;
  `s115` also has a bound named by a `CONST` (`x17`). Both pass on their first run; `slice\` 122,
  `slice.list` 426 (`tests\corpus\README.md`, `CLAUDE.md`).

## 4. Owner values in arrays, copies and every kind of element

- [x] 4.1 Arrays of an owner type (design D6, the array rows): new elements, plain `REDIM`, `_PRESERVE` and
  `_RETAIN` moving kept elements, `ERASE` of a dynamic and of a static parent, `SWAP`. Verify: `v35_b`, `x27`,
  `x28`, `x49`, `x51` print their recorded output.
  Done 2026-10-15: written with task 3.3 (its note); all five print their recorded output (`v35_b` and `x51` as
  copies with `0` for `_ERRORLINE`, compared without the line it prints).
- [x] 4.2 Strings and nested owners in the life functions: `_DYNAMIC` members of `STRING`, of user types with
  strings and of owner types; `STRING` members of owner types. Verify: `v35_c`, `v35_e` 1.–4., `x42` print their
  recorded output.
  Done 2026-10-15. The life functions came with task 3.3 (`v35_c` prints its recorded output). `v35_e` 1.–4. and
  `x42` waited on a rule older than this change: `sema` marked every member of an element and element of a member
  array as a target of `READ`, `INPUT`, `LINE INPUT` and `MID$` "not supported yet" (not measured since
  `m2-defxxx-common-arrays`, `SOMEDAY.md`). Measured first, `verification\v35_x62` (with a `.stdin`): a number is
  read before the indexes and a string after them, as an assignment stores (an index FUNCTION that reads an item
  gets the second item for a number, the first for a string); an index outside raises 9 and a number's item is
  stored into position 0 (D-004's case, now widened to these reads: stored nowhere here), a string's read takes
  nothing; console `INPUT` takes every index after its `? ` and before the line; `MID$` the index before the
  value. The code already did all of this (`store`, `place_ref`, libqb's readers returning while an error is
  pending), so the change is the mark taken away, for fixed and dynamic member arrays and members of elements
  alike (beyond the task as written: the rule was general), and the IR's validation of `Read` and `Input` with it.
  `x42` prints its recorded output; `v35_e` 1.–4. too, as a copy cut before its `PUT` (group 5); `x62` all but
  its D-004 lines 7, 22, 23 and 27 (27 prints what 22 left). The marked lines left `data_marked`, `files_errors`,
  `member_arrays_errors` and `swap_mid_marked` (snapshots: only those errors gone, the rest moved by a line);
  `member_targets_cpp` (new, by the review of group 4) fixes the order of each form in tier 1.
- [x] 4.3 `CopyWhole` of an owner type (design D7): read `AppendDynUDTDescCopy` (`type.bas` 10), measure an owner
  type nested in another (one more `v35_x` program), write the same evaluations. Verify: `x56` and the new
  program print their recorded output; `x57` too; the evaluation order fixed by a `cpp` or `typed` test.
  Done 2026-10-15 (`owner.rs` `copy_owner`). Read with `copy_full_udt_dyn` (`type.bas` 2049): a test that the
  places differ, then two routines: for a type that holds no `STRING` (`udtxvariable`), `CopyDynUDTExpr` (every
  member but the dynamic ones; an inline member array of an owner type element by element) and then
  `AppendDynUDTDescCopy` (each dynamic member, target then source); for one that holds a `STRING`,
  `AppendDynUDTOwnCopyEx` (each member in order, a dynamic one erased, target, and then cloned, source then
  target). Each statement finds both places anew. Measured with `verification\v35_x61` (nested owner types as a
  member and as an inline member array, without and with a `STRING`: 8 pairs and 29 evaluations, as read), which
  also found what the two routines do differently: from a member that is not allocated, the first keeps the
  target's, the second leaves it not allocated (`_set`, new beside `_copy`; `_dup` the deep copy both use;
  listed for review). `x56`, `x57`, `x61` print their recorded output; `typefields_cpp` gains a nested copy and one
  of a type with a `STRING`.
- [x] 4.4 Slice programs `s116_dyn_member_parents` (`v35_b`, `x51`) and `s117_dyn_member_kinds` (`v35_c`, `x56`,
  `x57`), recorded with `qb64pe.exe`, on `slice.list`; `SOMEDAY.md` gains the behaviours to review (`ERASE` of a
  static parent, the evaluations of D7). Verify: both pass (`--glob`).
  Done 2026-10-15: `s117` also holds `x61` and `x62` (with a `.stdin`); its items 23, 38 and 39 (D-004) recorded
  from a copy that reads the item into `q$` and raises `ERROR 9` there. Both pass on their first run; `slice\`
  124, `slice.list` 428. `SOMEDAY.md`: three rows (`ERASE` of a static parent, the evaluations of a copy, the two
  copy routines), the row on the order of a store extended to the reads; `DIVERGENCES.md` D-004 widened to them.

## 5. Files and the rejected forms

- [ ] 5.1 `PUT` and `GET` of a whole array of an owner type (design D8): pack, unpack, the bounds check, D-019.
  Verify: `v35_e` 5.–6. print their recorded output apart from D-019's line (`q(0).v` keeps its element);
  `x55` its own; a CLI or slice test of D-019.
- [ ] 5.2 The forms the old compiler rejects (design D4 last sentence): `LEN`, `_OFFSET`, `COMMON`, `PUT`/`GET` of
  one value, a whole array of a type with a `STRING`, `REDIM` without `TO` or with another type. Verify:
  `tests\frontend\typefields_rejected.bas` (`check-fail`) with `x18`–`x20`, `x39`, `x41`, `x43`, `x46`, `x48`,
  `x50`, `x54`; `_MEM` of an owner value and of a member (`x22`–`x24`, `x52`) compile and print their output.
- [ ] 5.3 A slice program `s118_dyn_member_files` (`v35_e`, `x55`; D-019's line recorded from a program without
  it), on `slice.list`. Verify: it passes (`--glob`).
- [ ] 5.4 Upstream: the rest of the 93 programs leave `deferred.list`; those that pass join `pass.list`; one that
  fails is fixed or, if it needs more than this design, put to the user; the demand report over `tests\upstream`
  names no `$UNSTABLE` or marker. Verify: the 93 run by themselves; tier 1's list tests pass.

## 6. Close

- [ ] 6.1 Closing runs, once (working rule 9): tier 1 with the command-line tests that build, the runner's tests,
  the slice list with and without folding, the upstream and differential pass lists, the two full suites, the
  demand reports, the shrink-only lists regenerated (the call-site check is not affected: no row of the built-in
  table changes; say so). Verify: numbers in `STATUS.md`; nothing on a pass list fails.
- [ ] 6.2 Documents: `STATUS.md` (also the stale line on `m3-asserts-debug-zlib`), `ROADMAP.md`,
  `crates\README.md`, `tests\upstream\README.md`, `SOMEDAY.md` (the `TYPEFIELDS` row goes), `CLAUDE.md`'s layout
  table; `python tools\testtime\test_time.py`. Verify: `check_repo.py --untracked` is clean; the files to stage
  and a commit message are given to the user.

## Workflow follow-up

- Archive the change (its specs merged; `language/unstable-features` becomes a main spec) after the user has
  committed it. Then what `DECISIONS.md` 2026-10-14 leaves: whole-array assignment and `_ARRAYCOPY` stay deferred
  unless the user takes them; the plain copy of `internal\c` when libqb must change.
