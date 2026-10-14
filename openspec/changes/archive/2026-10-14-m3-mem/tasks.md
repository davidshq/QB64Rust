# Tasks

Order: group 1 first (the measurements), then 2, 3 and 4 in that order (each builds on the one before), 5 at any
point after 1, then 6 (design D10). Each of groups 2 to 5 ends with `cargo fmt --check`, `cargo clippy
--workspace --all-targets -- -D warnings` and `cargo test` (tier 1) green and with its tests, lists and
`crates\README.md` lines in place, so the change can stop after any group.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools or
a script file; `python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done.
**Test cadence** (`CLAUDE.md` working rule 9): while working, tier 1 and only the programs the work touches
(`--glob`, `--category`, or a list with `--pass-cache target\pass-cache`), each run's output sent to a log under
`target\`; tier 2, the run without folding and the full suites run once, in group 6. CI is not a step of this
change. A new measured fact goes into a `v32` paragraph of `study\00` §5 in the task that measures it. A
behaviour of the old compiler that looks questionable is implemented its way and listed in `SOMEDAY.md` ("QB64pe
behaviours to review"); the first is known already: `_MEMEXISTS` of a block of an erased array is -1. A form that
turns out not to parse today is fixed in `crates\syntax` in the task that finds it, with a parser test.

## 1. Measurements before code

- [x] 1.1 Measure every form of the proposal with `qb64pe -z`, outside the repository (design D3, D4): `_MEM` and
  `_MEMELEMENT` of each kind of place D3 lists, `_MEM(offset, size)`, each function with a `_MEM` result as a
  store, an argument and in `PRINT` of a member, `_MEMGET` (function and statement), `_MEMPUT` and `_MEMFILL` by
  each numeric type, `STRING * n`, a user type, a variable and `AS type`, `_MEMCOPY`, `_MEMFREE`, `_MEMEXISTS`,
  `_MEMIMAGE` with and without a handle, `_MEMSOUND`, `_OFFSET` of each kind of place, `MKI$` … `CVD` with each
  argument type, `_MK$` and `_CV` with every type word and `_BIT * n`; and the rejected forms. Keep them as
  programs `verification\v32_*` (`v32_x*` one form each; `v32_a`–`v32_d` as design D8 lists), recorded with
  `--suite verification`. Verify: D3 has one line per kind of place with its C++ in place of the table; the `v32`
  paragraph of `study\00` §5 has the facts; each scenario of the specs names forms and values the old compiler
  does give, corrected if not; the reference clone's `git status --porcelain` is empty before and after.
  Found: about 230 accepted forms (`v32_x01_accepted`), 115 other forms one program each (`x02`–`x116`), the
  values in `v32_a` (the members of every kind of block), `v32_b` (get, put, fill, copy, an image, when a block
  stops being valid) and `v32_c` (`MKI$` … `_CV`). Every scenario of the specs gives the values it names. Against
  the plan: **a function's `_MEM` result is accepted only as the value stored into a `_MEM` place** (not as an
  argument, not with a member read; design D2 corrected), and stored into a number (`k = _MEMNEW(4)`, `x80`) the
  old compiler's C++ fails to build; **a `_BIT` width cannot be written in `_MK$` and `_CV`** (design D5 and the
  proposal corrected); the lock of a block is that of the body where `_MEM` stands, also for a variable of the
  main module; and the lock slot of an array parameter depends on whether anything above `_MEM(p())` gave it its
  number of dimensions (slot 0, the elements' pointer, when nothing did). The `_MEMGET` statement into a
  variable-length string is accepted (its characters at their present length). The clone's status was empty
  before and after.
- [x] 1.2 Measure the runtime errors (design D8): one program per error 300 to 313 that a program can reach
  (freed memory, outside the region, a block of a procedure that returned, of a redimensioned array, an
  uninitialised block, `_MEMCOPY` source and destination), each under `ON ERROR` to show it is not trapped, with
  its message and exit code as the runner records them. Verify: `v32_d` programs recorded; D8 says which errors
  are reachable.
  Found: every error 300 to 313 is reachable (`v32_d01`–`d32`, design D8): the program stops with `Runtime error:
  Line: N (in main module)` and the message, exit code 0, and the handler does not run. Beside them: `_MEMNEW(-1)`
  raises error 5, which is trapped (`d30`); after `ERASE` of a dynamic array its block stays valid and `_MEMPUT`
  writes into the released memory with no error (`d29`); `_MEMFREE` of a `_MEMNEW(0)` block after another block was
  freed ends the program with exit code 127 (`d33`, a double free in libqb); a block of a main-module or `STATIC`
  variable taken in a SUB stops being valid when the SUB returns (`d34`).
- [x] 1.3 Record `v32*` in `CLAUDE.md`'s layout table, and tell the user of any kind of place that stays "not
  supported yet" (D3) or any form that needs more than design D1 to D6 describe, with the old compiler's code
  for it. Verify: the row names the programs; `check_repo.py --untracked` is clean.
  Found: no kind of place the compiler supports stays "not supported yet" (only the deferred `_DYNAMIC` and
  `_STATIC` members). Put to the user: the lock slot 0 of `_MEM(p())` for an array parameter whose dimensions
  nothing above has given; the user chose the fix (`DECISIONS.md` 2026-10-14, `DIVERGENCES.md` D-016). Four
  questionable behaviours are listed in `SOMEDAY.md` (a block after `ERASE`, the lock of the body where `_MEM`
  stands, the double free of a `_MEMNEW(0)` block, the type of `_MEMELEMENT` and `_MEM` of members). A Python edit
  through the shell turned four `\v` into vertical tabs in `SOMEDAY.md` (working rule 7); found by the scan and
  repaired.

## 2. The type and the blocks that are calls

- [x] 2.1 `_MEM` as a predefined user type (design D1): declared before pass 1, its members read, a store to one
  the old compiler's error, `_MEM` out of the "not supported yet" type words; the five front-end tests that used
  `DIM m AS _MEM` as an unsupported declaration take `CONST c$ = CHR$(65)` (D8). Verify: a `typed` test with
  `DIM m AS _MEM`, arrays of it, a member of another type and a parameter; a `check-fail` test of the store;
  `LEN(m)` is 56 in a slice program; the five tests pass with their new example.
  Found: **the type is added when a program first names it, not before pass 1** (`Checker::mem_ty`): a type
  added to every program would put a `TYPE _MEM` block into every `typed` dump. Everything a user type does
  worked for it with no other change. Of the five tests, three were check-fail tests of the follow-on rule
  (`mem_follow_on`, `unsupported`, `explicit_after_unsupported`; now `CONST c$ = CHR$(65)`); `deftype_errors`
  had `_DEFINE q AS _MEM`, which the old compiler rejects as it rejects a user type (measured, "Cannot convert
  type (_MEM) to symbol"): it is now that error, moved above the marked line; `template_forms` is a
  `parse-ok` test and keeps `DIM m AS _MEM`. Tests `mem_type_typed`, `mem_errors`; `LEN` 56 in
  `s88_mem_new_image` line 1.
- [x] 2.2 A function with a `_MEM` result (design D2) and the rows `_MEMNEW`, `_MEMFREE`, `_MEMEXISTS`,
  `_MEMIMAGE`, `_MEMCOPY` (its template with `TO`) and `_MEMSOUND` (on `NEVER_RUN`, D7), with the slot kind for a
  `_MEM` argument. Verify: call-site programs for every form are equal to the old compiler's; a slice program
  (new memory, an image read through `_MEMIMAGE`, a copy) passes; `_MEMSOUND` is held as code not reached.
  Found: two slot kinds, `Slot::Mem` (a place of type `_MEM`, passed as its address) and `Slot::Offset`; a
  block-valued call is allowed only as the value of a store into a `_MEM` place (the checker's `mem_store`),
  an error elsewhere, also `_MEMNEW(4).SIZE`. 26 call-site programs, all equal on their first run;
  `s88_mem_new_image` (16 lines) passed on its first run. **The image's pixels are not read here**: that needs
  `_MEMGET` (group 4), so `s88` prints the image block's members and the pixel is read by group 4's program.
  Beyond the task: **five upstream programs compile cleanly now** (`types/all_types`, `dim`, `dim_array`,
  `static`, `static_array`, which declare `_MEM` variables), passed and joined `pass.list` (158) when tier 1
  asked for them; and **`LEN` of a whole member array** (`LEN(p(0).a())`, accepted by the old compiler) was a
  false error that the marked `DIM … AS _MEM` had hidden in `arrays/test_mem_vs_len`: it is now "not supported
  yet".

## 3. The block of a reference

- [x] 3.1 `_MEM(reference)`, `_MEMELEMENT(reference)` and `_MEM(offset, size)` (design D3), for each kind of
  place 1.1 measured, with the old compiler's errors. Verify: call-site programs for each kind of place are
  equal; `v32_a`'s forms in a slice program print the old compiler's `SIZE`, `TYPE` and `ELEMENTSIZE`; an `ir`
  test shows the node.
  Found: `v32_a_blocks` built by the new compiler printed every line of the old compiler's but the one D-016 changes,
  before any test was written. Nine more forms were measured on the way (`_MEMELEMENT` of a member of a user
  type and of a `_MEM` member, nested members, a dynamic array of a user type): a member that is itself a `_MEM`
  or user type has that type's size as its element size, and `_MEMELEMENT` of any whole user-type value is typed
  32768 (design D3). Call-site programs compare variables, whole arrays and `_MEM(offset, size)` (32 of them,
  equal once `_MEMELEMENT(ua())` kept the old compiler's offset `((0)*18+0)`); elements and members are pinned
  by the `cpp` test `mem_refs_cpp`, since the check does not compare them. `s89_mem_references` passed on its
  first run. **Left "not supported yet", not measured**: a `TYPE` that holds a `STRING`, a member array that is
  not the last step of the place or whose elements are of a user type, a name with `()` that names no array.
  Beyond the task: four more upstream programs compile cleanly (`arrays/09_layout_and_mixed_members`,
  `46_Mem_Parent`, `multiple_arrays_same_type`, `nonzero_bounds_negative`), passed and joined `pass.list` (162).
  Twice more a Python edit through the shell turned `\v` into a vertical tab (`check\builtins.rs`, the new slice
  program); both found and repaired (working rule 7).
- [x] 3.2 `_OFFSET(reference)` (design D6). Verify: call-site programs; a slice line comparing it with `_MEM`'s
  `OFFSET`; `check-fail` tests of a value and a `_BIT` array.
  Found: as designed; the address of a variable-length string variable is its characters, of an element, member
  or array of them not measured ("not supported yet"). 11 call-site programs, equal once they store into an
  `_UNSIGNED _OFFSET` (a store into `_OFFSET` or a LONG is a narrowing the check does not compare);
  `s89_mem_references` compares `_OFFSET` with `OFFSET` for a variable, an element, an element of a member
  array and an address block.

## 4. Reading and writing through a block

- [x] 4.1 `_MEMPUT`, the `_MEMGET` statement and `_MEMFILL` written inline (design D4), and the `_MEMGET`
  function, by every type 1.1 measured. Verify: the C++ of each form matches the old compiler's, line by line, in
  one compared program per form; a slice program puts, gets and fills by each type and prints the result; `ir`
  and `typed` tests.
  Found: `verification\v32_b_get_put` built by the new compiler printed the old compiler's output on its first
  run, and so did all 34 `v32_d` programs (the double free of `d33` included). 69 call-site programs, each the
  statement's lines compared token by token (the call-site check compares every line of a statement, so the
  inline checks too), all equal once a whole member array's address kept the old compiler's `((0+2))`; a
  literal offset outside a call (`5ll` against `5`) is a spelling the check does not compare, so that program
  was left out. **The parser needed one change** (as the task's standing rule says): a type argument with `*
  n` after `STRING` or `_BIT` (`_MEMGET(m, o, STRING * 4)`), with the parser test `mem_forms`; a user type's
  name as the third argument stays an expression, read as a type by the checker. A value `AS type` is stored
  through a pointer of its type with C++'s own conversions left to it (`k + 1 AS INTEGER` is written `*(int16*)
  tmp_long=…` with no cast, as there); a `_MEMGET` of a user type stands only as the value stored into a place
  of that type. `s90_mem_get_put` (the program of `v32_b`) passed on its first run. Beyond the task: four more
  upstream programs compile cleanly (`arrays/nonzero_bounds_positive`, `whole_member_array_ops_nonzero_bounds`,
  `offset/offset_test`, `putimage/putimage_test`), passed and joined `pass.list` (166).
- [x] 4.2 The runtime errors (task 1.2) as slice programs, one per error. Verify: each passes against its
  recorded output and exit code.
  Found: 14 programs, `s91_mem_error_300` to `s104_mem_error_313`, made from the `v32_d` programs; each passed on
  its first run.

## 5. A number and its bytes

- [x] 5.1 `MKI$`, `MKL$`, `MKS$`, `MKD$`, `CVI`, `CVL`, `CVS`, `CVD`, `_MK$` and `_CV` (design D5), with the old
  compiler's errors. Verify: call-site programs for each type; a slice program (`v32_c`'s values) passes;
  `typed` tests; the `CVL(5)` error.
  Found: two rules, `Rule::Mk` and `Rule::Cv`, one call each; `_MK$` and `_CV` keep the type named in their
  first slot as a 0 of that type, so a `_BIT` (one bit wide) is told from an `_INTEGER64`. `MKI$` of a float is
  written `qbr_float_to_long(x)` of the value as it is, as the old compiler writes it (the store into an
  INTEGER writes a float cast before it, which C++ makes anyway). `verification\v32_c_mk_cv` built by the new
  compiler gave the old compiler's output on its first run; 55 call-site programs, all equal on their first
  run; `s105_mk_cv` passed on its first run; `mk_cv_typed`, `mk_cv_errors` (the `CVL(5)` error among them).
  Beyond the task: **eight upstream programs compile cleanly** (the six `paint/tile_*` the proposal named, and
  `view/window_view_mandelbrot`, `view/window_view_screen_line`), passed and joined `pass.list` (174), when tier 1
  asked for them.

## 6. Upstream, lists, closing runs and documents

- [x] 6.1 Upstream (design D9) and the lists: run the upstream programs that name the change's built-ins; those
  that pass join `tests\upstream\pass.list`; regenerate the shrink-only lists; run the demand report. Verify:
  tier 1 passes; `qb64rust --report unsupported` on upstream names no built-in of this change except forms the
  proposal leaves out.
  Found: the programs had joined `pass.list` group by group as tier 1 asked for them: 21 in all (5 in group 2,
  4 in group 3, 4 in group 4, 8 in group 5), 153 to 174. The demand report over upstream names no built-in of
  this change; its one line of `m3-mem` is `LEN` of a whole member array (`arrays/test_mem_vs_len`), which the
  proposal leaves out. Every other upstream program that names `_MEM`, `_OFFSET`, `MKL$` or `MKI$` is held by
  something else: the deferred `_DYNAMIC`/`_STATIC` members and `$Unstable` (18), `_BASE64DECODE$` and `_INFLATE$`
  (the image tests of `imageassert.bm`: `font/test`, `image/curico_test`, `pcx_test`, `webp_test`), `$CHECKING`
  (`source_ordering/checking_off`), a `TYPE` in a SUB (`mem/mem_checking_off`), `_SNDNEW`, blanks around a dot,
  a whole array `z()` where no parameter takes it. The shrink-only lists did not change.
- [x] 6.2 The closing runs, each once, with `--jobs 8 --build-cache` and without `--pass-cache`: tier 1 and the
  tests that build (`--ignored`, run alone), the runner's own tests, the call-site check, the slice list with and
  without folding (`QB64RUST_NO_FOLD=1`), the upstream and differential pass lists, the two full suites. Verify:
  every run passes, or a failure is fixed and only what the fix can affect is rerun, with a note of which runs
  were not repeated and why that is safe; `STATUS.md` has the numbers.
  Found: every run passed once: tier 1 306, the 19 tests that build, the runner's 68, the call-site check 1631 of
  1631, the slice list 416 of 416 and 416 without folding, upstream 174 of 174, differential 59 of 59, the full
  corpus (435 pass, `218_fre` with only marked errors, 5 known failures) and the full upstream suite (174 pass,
  229 rejected, none wrong). After tier 1, a fix of line ends (CRLF a patch script had written into four Rust
  files and `mem_errors.bas`) changed no character but those; only the front-end tests were rerun for it (they
  passed), since the release build the tier 2 runs used was made from the same text.
- [x] 6.3 The documents: `STATUS.md` (step 7's `_MEM` part done; `$ASSERTS` and `$DEBUG` next), `ROADMAP.md`,
  `crates\README.md`, `tests\corpus\README.md`, `tests\upstream\README.md`, `study\00` §5, `SOMEDAY.md`,
  `DIVERGENCES-QB45.md` where needed, `CLAUDE.md`'s layout table, `design.md` "As built", and a `DECISIONS.md`
  row for any choice made while building. Verify: `check_repo.py --untracked` is clean; `openspec validate
  m3-mem --strict` passes.
  Found: all written; `DIVERGENCES-QB45.md` needed nothing (`_MEM` is QB64's; `MKI$` … `CVD` behave as in
  QuickBASIC for the values measured). `tests\corpus\README.md` still had counts from before
  `m3-images-fonts-mouse`; they are today's now.
- [x] 6.4 The record of test runs: `python tools\testtime\test_time.py`. Verify: its three files are named among
  those to stage, with a commit message for the change.
  Found: `runs.csv`, `sessions.csv` and `REPORT.md` updated (218 sessions, 4136 runs on record); named with the
  files to stage at the close.

## 7. Review

- [x] 7.1 Fix what the review of the change found (2026-10-14). Verify: each fix measured where it changes what is
  accepted; fmt, clippy and tier 1 green; the slice programs it touches pass.
  Found: **`READ m.SIZE` and `INPUT #1, m.SIZE` were accepted**; the old compiler rejects them as it rejects `m.SIZE
  = 4` (its `setrefer`, `verification\v32_x117`, `x118`): the check moved into `read_only_member`, called by the
  assignment, `READ` and `INPUT #`. `_MEMELEMENT` of a member of an element of a user-type array and of an element
  of a member array of one were compiled unmeasured: measured (`v32_x119`), the emitter gave the old compiler's
  values, now run by `s89_mem_references` (re-recorded). `_MEMGET(m, o, _UNSIGNED STRING * 4)` was accepted, now
  the old compiler's error (`x120`). Docs: `StmtKind::Assign` (a user-type value from a built-in), `Op::MemFill`
  (the order of evaluation), `bit_store`'s comment back on its function; the IR validator checks the type of a
  user-type value an `Assign` stores. Runs: tier 1 (306), `s89` recorded with the old compiler, and the 17 slice
  programs of `_MEM` (`s88`–`s104`) built by the new one, all passed; the validator runs only in a debug build,
  so the debug binary's `--dump ir` was run over those, `s105_mk_cv` and the 174 programs of the upstream pass list (173
  found), with no problem. The other tier 2 runs were not repeated: the fixes reject only forms of a `_MEM` block
  no other program uses, and only the checker's two new paths give an `Assign` a user-type value (the lowering's
  own stores are of numbers), which those programs hold.

## Workflow follow-up

- Archive the change (`/opsx:archive`) once the user has committed it; its specs merge then.
