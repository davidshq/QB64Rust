# Tasks

Standing rules: CLAUDE.md. What this change does differently: every recording with `qb64pe.exe` (the family
`v57`) and the reading of the old compiler's C++ (`-z`) are batched in group 1, before the code. They may change
the delta specs and the design before anything is built. Group 2, the cursor walk of design D8, changes no output
and runs before any crossing is built.

## 1. Measurements and records

- [x] 1.1 The family `verification\v57_*` of design D1, recorded with `qb64pe.exe` (the verification suite), with
  one line in `verification\README.md`. Where a recording differs from a delta spec or the design, the spec or
  design is changed here, and a form shown to be accepted or rejected moves to the right decision. Verify: every
  program has its recorded output or error; `tools\repo_check\check_repo.py --untracked` finds nothing in them;
  `openspec validate m4-statements-metacommands --strict` passes after any edit.
- [x] 1.2 The old compiler's C++ (`qb64pe.exe -z`) of `RUN` in its three forms (main module and SUB), `RESUME` of
  each kind in a SUB, `PALETTE USING`, `SWAP (a), b`, `READ (a)` and the `v57` crossing programs. Design D5, D7,
  D8 and D11 name the C++ lines they rest on and are corrected where they differ. Verify: no file of the clone is
  changed (`git -C ..\QB64pe status` is clean).
- [x] 1.3 `DECISIONS.md`: a row for the change (step 4a of the plan of 2026-10-22, split by the user on
  2026-10-23; 4b takes the built-ins no program uses and `$ERRORLOCATION`/`$USELIBRARY`; the 9 programs that
  steps 2 and 3 left stay marked). It names `openspec\changes\m4-statements-metacommands\design.md`. Verify: the
  row is there and names the design.

## 2. The cursor walk of `sema` (no change of output)

- [x] 2.1 `sema` (`check\mod.rs` `check_statement`, `check\blocks.rs` `block_body` and every block checker): the
  main module and each procedure body are walked through a cursor over the flattened stream of statements, as
  design D8 says. A block's body is pulled from the cursor until its closer. No crossing is built yet, and the
  marks of `parser\blocks.rs` stay. Verify: `cargo test` (tier 1) with every `typed`, `ir` and `cpp` snapshot
  unchanged; the call-site check unchanged; fmt and clippy clean.
- [x] 2.2 The slice list with `--pass-cache target\pass-cache`. Verify: every program passes, or each executable
  is unchanged.

## 3. Blocks across included files

- [x] 3.1 `syntax` (`parser\meta.rs` `include`, `parser\blocks.rs`, `parser\proc.rs`): the included parser
  starts with the open blocks of the including file, and the parser records `cross_closers` both ways (design
  D8). The marks "a block closed in another file" go wherever `v57` shows the old compiler compiles the form. A
  form it rejects becomes its error. Verify: parser tests for a `FOR` closed in an included file, a block opened
  in one and closed in the main file, and a SUB across files; `tests\known_parse_gaps.list` stays empty.
- [x] 3.2 `sema`: a block's checker finds its closer through `cross_closers` (`NEXT`'s variable, the end line of
  an error), and a SUB's body ends at its `END SUB` in the other file. Verify: `v16_m7_block_across`,
  `v16_m7_sub_across` and the `v57` crossing programs pass (`--suite verification --glob`); the front-end test
  `tests\frontend\include_cross_ok.bas` (`check-ok`) with its included files in `tests\frontend\inc\`; the
  scenarios "Block closed in an included file" and "SUB closed in an included file" each have a test.

## 4. Errors for the forms the old compiler rejects

- [x] 4.1 Metacommands (design D3, `check\mod.rs` `meta`): the old compiler's list closed; `$NOPREFIX` its
  error; `$ERRORLOCATION` and `$USELIBRARY` still marked. Verify: `v16_m6_unknown_meta`, `v16_m7_bare`,
  `v16_m7_bare_blank`, `v24_x38`, `v39_x74`, `x79`–`x81`, `v43_x32` and the `v57` metacommand programs are
  errors with no mark; `tests\frontend\metacommands_errors.bas` (`check-fail`); the two snippets with `$DYNAMIC`
  leave `tests\known_unsupported_rejections.list`.
- [x] 4.2 Stubs (design D4, `check\builtins.rs`, `check\proc.rs`): every stub of the old compiler's table with no
  rule of `sema` is "Command not implemented". After `CALL` the rules apply as in D4. Verify: `218_fre`,
  `v32_x89`, `v32_x91` and the `v57` stub programs are errors with no mark; `tests\frontend\stubs_errors.bas`
  (`check-fail`); `CALL SWAP` passes (`v26_x128`, group 5).
- [x] 4.3 The parser's marks with a recorded rejection (design D2): `x = OR`, `2 ^ _NEGATE x`, `IF THEN`, `;` in
  `WRITE`, `ON STRIG(1 GOSUB`, `_DEFINE AS`, `DEFINT a AS`. Verify: `v51_x50`, `v24_x238`, `v22_x57`, `v22_x58`,
  `v47_x08`, `v24_x11`, `v24_x19` and the snippet `error_detection__invalid_syntax` are errors with no mark; a
  parser test for each; `tests\known_false_errors.list` stays empty (tier 1 over every accepted program).
- [x] 4.4 The rest of the rejections: `SHARED` in the main module; `CALL fs$5(…)` and the program FUNCTION as a
  statement by `v57`; omitted arguments where `v57` records an error (`POINT(, 2)`, `_RGB32(1, , 3)`); the
  disagreeing `$IF` with a `CONST` by design D9 (`v26_x34`). Verify: `v21_x56`, `v27_x50`, `v28_x49`, `v26_x34` and
  the snippet `shared_variables__shared_outside_procedure_error` are errors with no mark;
  `tests\known_unsupported_rejections.list` is empty; `tests\frontend\statements_errors.bas` (`check-fail`).
- [x] 4.5 The wording table: `python tools\diagnostics\old_wording.py`, which writes
  `crates\base\src\old_wording.tsv` with the notes of the new errors. Verify: `old_wording.py --check` passes;
  each program of 4.1–4.4 shows the old compiler's message as its note (`qb64rust -z`); `cargo test`, fmt,
  clippy.

## 5. The accepted forms

- [x] 5.1 `RUN` (design D5): `sema` (`check\flow.rs`), `ir` (`StmtKind::Run`, validation, dump), `codegen-cpp`
  (`procs.rs`, `runline.txt`). `RUN one` with a SUB `one` is "Name already in use" (`v28_x86`). Verify: the `v57`
  `RUN` programs pass, the one that starts a second program with that program in its `.files` folder;
  `tests\frontend\run_ir.bas` (`ir`) and `run_cpp.bas` (`cpp`); a program for each form in `tests\callsite\`, and
  the call-site check equal.
- [x] 5.2 `RESUME` inside a procedure (design D7). Verify: `v17_d_resume_sub_label`,
  `v17_d_resume_main_label_from_sub_both` and the `v57` `RESUME` programs pass, or each one that stays marked has
  its reason in `known.list`.
- [x] 5.3 Places in parentheses (design D6): `SWAP (a), b`, `CALL SWAP(a, b)`, `READ (a)`, `LBOUND((a))`, and the
  `v57` neighbours the old compiler accepts. Verify: `v22_x103`, `v26_x128`, `v22_x84`, `v51_x34` and those `v57`
  programs pass; `tests\frontend\paren_places_typed.bas` (`typed`).
- [x] 5.4 `PALETTE USING` (design D11): the parser's mark goes, `sema` and codegen write `sub_paletteusing`.
  Verify: `v27_x64` passes; a screen program `tests\corpus\screen\p0x_palette_using` (the next free number) with
  its state recorded from the old compiler's build passes and joins `slice.list`; a program in `tests\callsite\`.
- [x] 5.5 The small ones of design D11: `DECLARE … ALIAS` outside a library (`v37_x08`), the library and program
  procedure of one name (`v37_x22`, by `v57`), `STRING * n` with a `CONST` (`v56_x30`), the function
  `_SCREENHIDE` (`v26_x74`; a row of the built-in table and a program in `tests\callsite\`), `_RED` beyond the
  palette (`v45_x64`, design D10, with its `DIVERGENCES.md` row). Verify: each program passes.
- [x] 5.6 The disagreeing `$IF` (design D9): no branch compiled where the model holds. Verify: `v26_x27` and the
  `v57` flag programs pass, or the line stays marked with its reason in `known.list` and the spec scenario is
  changed to say so.

## 6. Lists and closing

- [x] 6.1 `SOMEDAY.md`: the rows that name a form of this change as "not supported yet" are removed or narrowed.
  Rows of "QB64pe behaviours to review" are added for what is kept: `SWAP (a), b` swapping `a`, `READ (a)`
  reading into `a`, blocks across included files, the disagreeing `$IF` compiling no branch, and anything else
  `v57` shows. Verify: no row says a form of this change is "not supported yet" unless it still is.
- [x] 6.2 `tests\verification\known.list` and `pass.list`: every program that now passes moves to `pass.list`,
  and one whose first mark changed keeps its line with the new reason. The `v57` programs go on one list each.
  Verify: the tier-1 tests of the lists (each program named once); the demand report over `known.list`'s "not
  supported yet" programs names only the 9 that steps 2 and 3 left and any of this change's that stayed marked
  with a reason.
- [x] 6.3 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then
  `tools\close\close.py --check` passes. Then the counts of the test READMEs by `tools\legacy_tests\counts.py`,
  and `STATUS.md`'s "Next" names step 4b of the plan. Then `python tools\testtime\test_time.py`. Verify: `--check`
  passes.

## 7. Review follow-up

- [x] 7.1 Measure what the review's findings ask of the old compiler: `v57_j01`–`j06`, `b21`–`b25`, `r20`–`r24`
  (`verification\README.md`). Verify: each recorded with `qb64pe.exe` and on `tests\verification\pass.list`.
- [x] 7.2 The label pass drops a disagreeing `$IF`'s branches as pass 2 does, and a block crossing its `$END IF`
  ends the drop at the block's end (design D9). Verify: `v57_j01`–`j06` pass;
  `tests\frontend\flags_between_drop_errors.bas` (`check-fail`).
- [x] 7.3 `RUN` of a name that means a string: a string `CONST`, a `DEFSTR` name (design D5). Verify: `v57_r20`–`r24`
  pass.
- [x] 7.4 Pass 1's `CONST` names follow the walk across files (design D8). Verify: `v57_b22`–`b25` pass.
- [x] 7.5 The parser's closer inside a crossed `$IF` pairs with a block of another file (design D8). Verify:
  `v57_b21` passes; `tests\frontend\cross_next_in_pp_if_errors.bas` (`check-fail`) has one error.
- [x] 7.6 Comments: `error_in`'s doc, the branch after `ELSE` in `if_block`, the error cap and a `CrossCloser` in
  `walk.rs`. The review's findings on the error cap and on `error_in` need no change: a capped tree's output is
  already full, and `Diagnostics::push` applies the cap and `Skips` reads the final list.
- [x] 7.7 The runs these can affect: tier 1 with fmt and clippy, the CLI tests that build, the slice list, the
  upstream pass list and `tests\verification\pass.list` with the pass and build caches, the call-site check, and
  `close.py --check`.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
