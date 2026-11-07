# Tasks

Standing rules: CLAUDE.md.

## 1. The rule

- [x] 1.1 Add `crates\codegen-cpp\src\order.rs`, with:
      - `expr_group` and `op_group`: the table of design D1, each a `match` with no wildcard arm;
      - the classes of design D2, with the "changes state" and "cannot raise" lists of built-ins. Read the
        "changes state" list against libqb's sources and name the source file of each entry in a comment;
      - the conflict test and the choice of held parts of design D3.

      Verify with unit tests in the module:
      - each group kind;
      - two index checks do not conflict, and `x` before a FUNCTION does;
      - `_IIF` and `AND ALSO` give no group;
      - every name on the two lists is a built-in of `builtins.json`.
- [x] 1.2 Add the hold mechanism to `Emitter` (design D4–D6):
      - the `held_exprs` and `held_places` maps and `with_holds`;
      - the `seqN` temporaries in `pass_decls`;
      - the checks at the top of `value`, `load_place`, `place_ref` and `bytes_of`;
      - places held as their flat index or byte address.

      Call `with_holds` from `value` and from `op`. Verify with `cargo test -p qb64rust-codegen-cpp`. A test
      compiles `f(1) - f(2)`, `x + g`, `MID$(s$, f(1), f(2))` and a SUB call with two raising by-reference
      arguments to C++ and checks the comma expression and the order of the `seqN` assignments.
      Built: that test is `tests\frontend\eval_order_cpp.bas`, a `cpp` snapshot of the driver's tests, since
      `codegen-cpp` has no front end (design, "Found while building").
- [x] 1.3 Fold `SWAP`'s own `auto swap_x`/`swap_y` code into the general rule (design D5). Verify that
      `tests\corpus\slice\s61_member_arrays` passes on Windows, and that `SWAP` without a FUNCTION in an index
      gives the same C++ as before.
- [x] 1.4 Measure how many programs' C++ changes (design D7). Write the `-z` output of the corpus and of the
      verification programs before and after, with the parent commit's build and with this change's, and count the
      programs that differ. Read a sample of the differences, to check that each has a real conflict. Write the
      count in design.md's "Found while building". Verify that `tests\callsite` gives the same C++ (call-site
      check run on a few programs) and that `cargo test -p qb64rust-codegen-cpp` passes.

## 2. The new family

- [x] 2.1 Write `verification\v69_*`, one program per kind of group in design D7, each printing which calls ran in
      which order. Include places no existing program reaches (`MID$(s$, f(1), f(2))`, `INSTR(f(1), a$, b$)`,
      `_RGB(f(1), f(2), f(3))`, `a(f(1), f(2))`, `CHR$(f(1)) + CHR$(f(2))`). Record them with `qb64pe.exe` on
      Windows (`run_legacy_tests.py --suite verification`, these programs named one by one). Verify that the new
      compiler's builds match the recordings on Windows. Add them to `tests\verification\pass.list`, and add a
      line for the family to `verification\README.md` saying what it shows.
- [x] 2.2 Check the spec's scenarios against the recordings. Each scenario of `language/evaluation-order` is shown
      by a `v69` program, and the value of a numeric assignment comes first. Correct the spec where the recording
      shows otherwise.

## 3. Linux

- [x] 3.1 Delete the Linux sidecars that only record the other order:
      - `tests\corpus\slice\`: `s106_asserts`, `s108_asserts_off_debug`, `s22_store_rules`,
        `s78_choice_min_shifts` and `s86_field_lset` (each a `.linux.output`);
      - `verification\`: `v18_c_byref`, `v30_k_field_errors`, `v30_l_lset_rset`, `v33_a03_assert_off` and
        `v33_a04_assert_evaluated_on` (each a `.linux.out.txt`).

      Cut `v30_g_iif_min_max.linux.out.txt` down to the Windows recording with only its NaN lines changed.
      Remove any `known.linux.list` row of these programs. Verify with `tools/linux/sync.sh` and `run.sh` for
      these 11 programs, the 13 `v69` programs (fewer than about 20 per run), `s12_error_in_print` and
      `s136_errorline`:
      - the 11 pass against their Windows recordings;
      - the `v69` programs pass with no sidecar;
      - `s12` and `s136` still differ only as their sidecars record (design, "Non-Goals").

      Done on 2026-11-07. The nine sidecars were deleted with `git rm` at the user's request ("you rm the
      sidecars"). The 13 `v69` programs pass on Linux with no sidecar; `s12` and `s136` pass against their sidecars; the nine
      programs whose sidecars go give the Windows recording byte for byte; `v30_g` passes with its cut sidecar.
      `s86_field_lset` keeps a sidecar cut to its line 20 (design, "Found while building"). No `known.linux.list`
      row names any of them.
- [x] 3.2 Run the Linux slice and the Linux verification list with `check` (`linux-slice`,
      `linux-verification`). Verify that nothing that passed before fails.
      Done on 2026-11-07. `linux-slice` passed (238 s). `linux-verification` passed (945 s, 4,393 programs):
      93 KFAIL (each a known row), 17 SKIP, the rest PASS. `v42_a_timer` and `v42_e_service` failed with `--jobs 8` while
      the slice ran beside them, and passed when the runner ran them again alone (timing-sensitive).

## 4. Records and close

- [x] 4.1 Add the `QB64PE-DIVERGENCES.md` row (design D8) and point D-054 to it. Remove the `SOMEDAY.md` row on
      the order of calls with side effects. Rewrite `verification\README.md`'s line on `v18_c`, `v30_k`, `v30_l`,
      `v33_a03`, `v33_a04` and `v30_g`. Update `crates\ir\src\lib.rs`'s module documentation to state the order as
      a guarantee on every platform. Verify with `tools\repo_check`.
- [x] 4.2 Tick `ROADMAP.md`'s "After M7" item, and update `STATUS.md`'s "Next". The user asked for this item
      before the merge, as a step of this change.
- [x] 4.3 Run `just quick` in the background, and the verification list narrowed to this change's programs. Say
      what ran and what stood, and give the user the files to stage and a commit message.
      Done on 2026-11-07. Every step of `just quick` stood from today's runs. `upstream` and `verification` were
      left for `check full`, since the change reaches more than programs.

## Workflow follow-up

- Archive the change with `openspec-archive-change`, which merges the two delta specs into `openspec\specs\`.
