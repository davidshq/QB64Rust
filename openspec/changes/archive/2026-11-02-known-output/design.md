# Design: check what the new compiler prints for a known divergence

## Context

See `proposal.md` for why. The current state:

- `verification_compare` in `tools\legacy_tests\run_legacy_tests.py` judges a program by QB64pe's recording,
  `verification\<name>.out.txt`, or by its Linux sidecar on Linux (`expected_file`). A program on `known.list`
  that fails is a known failure (`KFAIL`), and the runner looks no further.
- `fixed\<name>.out.txt` (change `fixes-switch`, design D4) already holds output written by us, not by
  `qb64pe.exe`. That program is built a second time with `--fixes on`. This change borrows the idea, but a
  `diverged\` file replaces the recording instead of adding a second run.
- `divergences\<name>.divergences` lists the rows a program's run may take. The runtime's trace
  (`QB64RUST_DIVERGENCES`) names only a few rows: D-003, D-004, D-006, D-009, D-012, D-016, D-018 to D-022,
  D-026, D-028 and D-031. The other rows have no run-time path to trace.
- In the last full run (2026-11-02, worktree at `df67128`) the 124 D programs failed in three ways. 112 built
  and printed other output than QB64pe's. 7 were rejected where QB64pe built them. 5 built where QB64pe rejected
  them. None timed out.

## Goals / Non-Goals

**Goals:**

- Every D program whose output we can pin is held to that output, all of its lines.
- A pinned output can only be written after a review that ties each differing line to the program's D row.

**Non-Goals:**

- No change to the compiler or to any recording of `qb64pe.exe`.
- No sidecar for a program whose output changes from run to run.
- The shorter timeout of `TODO.md` row 1 (see D7).

## Decisions

### D1. The sidecar replaces the recording

`diverged\<name>.out.txt` stands where `<name>.out.txt` stood, and `diverged\<name>.log.txt` where
`<name>.log.txt` stood. Every other rule stays: the folder mask, the closing `exit=`, `.normalize`, the D-012 rule
of `wanted_verification_text`, the log, and the check of the trace against `.divergences`.

*Code:* a new `diverged_sidecar(name)` beside `fixed_sidecar`. `verification_compare` picks `recorded_file` from it
before `expected_file`, and does the same for the log. The pass cache key already holds the bytes of the recording
it judged by, so a changed sidecar misses the cache. The log sidecar's bytes go in as well.

*Instead of:* a second result per program, as `fixed\` does. That would keep the program on `known.list`, and it
would show a `KFAIL` as well as a pass for one program. One result per program keeps the counts simple.

### D2. The program moves to pass.list, and its reason moves to .divergences

A program with a sidecar passes, so it goes on `pass.list`, and its `known.list` row goes. The reason it differs
from QB64pe must not be lost, so its `divergences\<name>.divergences` must name the row or rows it takes. That is
the file `known.list`'s reason pointed to anyway, for traced rows. For an untraced row the line is never
checked against the trace. That is harmless: `unexpected_rows` only fails rows that are traced and not listed.

*Code:* the tier-1 test `verification_lists_are_exact` in `crates\driver\tests\it\inputs.rs` gets the checks of
the spec's "The diverged sidecars are checked". It reads the D rows from `docs\DIVERGENCES-QB64PE.md`'s table.

*Instead of:* keeping the program on `known.list` with a sidecar. Then a regression would still be a known
failure, which is the problem this change fixes.

### D3. A diverged rejection is pinned by lines.list

For 12 programs, our compiler rejects a program QB64pe builds (7), or builds one it rejects (5). The second kind
gets an ordinary output sidecar. The first kind gets `(not compiled)`. QB64pe's `compile.txt` names no `LINE n:` for
a program it built, so today's rule would compare no line and pass any error at all. Instead, the line comes from
the program's row of `lines.list`, and a row is required.

*Code:* in `verification_compare`, a diverged rejection passes the `lines.list` line to `rejection_line` as the
line wanted, in place of the line read from `compile.txt`. With no row it fails at "error line". The note
`LINES_NOT_NEEDED` is never given for such a row.

### D4. Which programs get a sidecar

All 124 D programs of `known.list`, apart from:

- The 6 that a CLI test names already: `v24_h_member_store`, `v32_a_blocks`, `v35_e_dyn_statements`,
  `v35_g3_dyn_bound_too_large`, `v56_a5_type_string_member` in `crates\driver\tests\it\cli.rs`, and
  `v45_b_funcs` in `consts_v45.rs`. `TODO.md` row 1 left these out.
- The 2 `D-050` programs, `v32_d33_free_zero_size` and `v63_c2_memnew_zero_two`. Whether they crash depends on
  the heap (`STATUS.md`, "Known bugs").
- Any program whose output differs between two runs of the same build. These keep their row, with the reason
  `varies` added after the D row.

### D5. How a sidecar is written and reviewed

1. Run the candidates twice with today's build and keep each run's output and log. A program whose two runs differ
   is left out (D4).
2. Build each candidate file from the run as the compare would build it, with `verification_text`. This is done
   by a script under `target\`, which is not committed. The runner gets no "bless" option. Writing a sidecar
   must stay a step that someone reviews.
3. Compare each candidate with QB64pe's recording, line by line. Each line that differs must be one the
   program's D row describes, in its "This compiler" column. A program with any other difference keeps its
   `known.list` row, with its reason changed to `bug: <what>`, and gets a line in `STATUS.md`'s known bugs.
4. Record the review in a table at the end of this file: the program, its rows, how many lines differ, and why.

### D6. Counts

`tools\legacy_tests\counts.py` adds one line to the membership block: how many programs of `pass.list` pass by a
`diverged\` sidecar. `known.list`'s table of reasons shrinks by itself as rows move.

### D7. No shorter timeout

`TODO.md` row 1 also asked to judge a program recorded as a timeout after about 10 s instead of 60. The row was
written on 2026-10-27, when seven programs ran to 60 s each, which was about 17% of a full run's program time.
Since then D-048 (console reads at end of input) and D-036 (`STOP` in `SUB _GL`) were built. In the full run of
2026-11-02, none of the 13 programs recorded as timeouts timed out with our compiler. Each ended in 4.3 to 4.8 s.
A shorter timeout would save nothing and would add a rule, so it is not built. The 13 are D programs and get
sidecars like the others.

## Risks / Trade-offs

- [A sidecar blesses a bug] → The review of D5 ties every differing line to a D row. The sidecars are new files
  the user reads before staging.
- [A sidecar hides a later fix] → When a later change makes a D program match QB64pe again, the program fails
  against its sidecar. That is the wanted signal: the change removes the sidecar and the D row's mention.
- [Linux prints something else] → The sidecar is used on both platforms. A program whose Linux output differs
  gets a row in `known.linux.list`, as other programs do.
- [About 116 new files for the user to read] → Most differ from QB64pe's recording in one or two lines. The
  table of D5 says which lines, so the reading is quick.

## Review of the sidecars

Filled in by task 3 (2026-11-02). The 116 candidates ran twice with the release build of `d6d341e` and gave the
same output both times, so none was left out as `varies`. No log differed from QB64pe's, so there is no
`diverged\<name>.log.txt`. Every differing line is one the program's D row explains, so no program stayed on
`known.list` as a bug. "Lines that differ" counts the recorded lines replaced or added, after `.normalize`.
Where an untrapped error ends the program, the exit code 1 is D-012's.

| Program | Rows | Lines that differ | Why |
|---|---|---|---|
| `v02_errors` | D-052 | 6 of 12 | Error 11 is trappable; untrapped, the message names the line. |
| `v02b_idiv_zero` | D-052 | 3 of 4 | Error 11 is trappable; untrapped, the message names the line. |
| `v04_accidental` | D-009 | 2 of 14 | Each wide `_BIT` variable has its own storage. |
| `v05_label_const` | D-008 | built (QB64pe rejected it) | A label and a `CONST` on one line compile. |
| `v05b_label_const_short` | D-008 | built (QB64pe rejected it) | A label and a `CONST` on one line compile. |
| `v17_d_return_label_none` | D-003 | 3 of 3 | `RETURN` with no `GOSUB` raises error 3 and the program goes on; QB64pe crashes later. |
| `v17_e_const_big_float` | D-007 | 1 of 2 | The `CONST` value is the float it is, or `1 / 0` is a compile error. |
| `v17_e_const_fdiv0` | D-007 | rejected on line 3 (`lines.list`) | The `CONST` value is the float it is, or `1 / 0` is a compile error. |
| `v17_e_const_label_line` | D-008 | built (QB64pe rejected it) | A label and a `CONST` on one line compile. |
| `v17_g_int64_min_div` | D-006 | 4 of 1 | `\ -1` of the smallest value raises error 6 and `MOD -1` gives 0; QB64pe crashes. |
| `v17_g_long_min_div` | D-006 | 5 of 1 | `\ -1` of the smallest value raises error 6 and `MOD -1` gives 0; QB64pe crashes. With no handler, error 6 ends the program with exit 1 (D-012). |
| `v17_g_mod_zero` | D-052 | 3 of 4 | Error 11 is trappable; untrapped, the message names the line. |
| `v17_g_precedence` | D-005 | 1 of 18 | `a IMP b IMP c` is `(a IMP b) IMP c`. |
| `v18_b_index_raises` | D-004 | 1 of 7 | A store into a place whose index is out of range raises error 9 and stores nothing. |
| `v18_b_member_store` | D-004 | 4 of 15 | A store into a place whose index is out of range raises error 9 and stores nothing. |
| `v18_h_string_member` | D-004 | 1 of 9 | A store into a place whose index is out of range raises error 9 and stores nothing. |
| `v21_b_bit_overlap` | D-009 | 1 of 7 | Each wide `_BIT` variable has its own storage. |
| `v21_e_scenarios` | D-009 | 1 of 51 | Each wide `_BIT` variable has its own storage. |
| `v21_g_fixed_function_twice` | D-014 | 2 of 4 | Each call of the FUNCTION returns its own copy. |
| `v22_d_randomize_noseed` | D-048 | 4 of 3 | A console read at the end of input raises error 62, and `END` does not wait; QB64pe times out. Error 62 is not trapped, so the program ends with exit 1 (D-012). |
| `v22_e_eof` | D-048 | 6 of 3 | A console read at the end of input raises error 62, and `END` does not wait; QB64pe times out. |
| `v22_x148_input_trailing_comma` | D-048 | 4 of 2 | A console read at the end of input raises error 62, and `END` does not wait; QB64pe times out. Error 62 is not trapped, so the program ends with exit 1 (D-012). |
| `v22_x50_close_trailing_comma` | D-048 | 2 of 3 | A console read at the end of input raises error 62, and `END` does not wait; QB64pe times out. |
| `v22_x59_write_trailing_comma` | D-048 | 2 of 3 | A console read at the end of input raises error 62, and `END` does not wait; QB64pe times out. |
| `v24_x272_static_of_shared` | D-041 | 3 of 4 | `STATIC a()` in the SUB is an array of its own; QB64pe stops with "Duplicate definition". |
| `v24_x313_member_index_order` | D-004 | 3 of 10 | A store into a place whose index is out of range raises error 9 and stores nothing. |
| `v24_x89_negative_count_dim` | D-040 | 4 of 4 | A dimension of fewer than no element raises error 5 and allocates nothing. |
| `v24_x90_negative_count_redim` | D-040 | 4 of 4 | A dimension of fewer than no element raises error 5 and allocates nothing. |
| `v24_x91_negative_count_preserve` | D-040 | 4 of 4 | A dimension of fewer than no element raises error 5 and allocates nothing. |
| `v25_signs_const` | D-028 | 1 of 3 | `^` of integers in a `CONST` is a float, or 0 to a negative power is a compile error. |
| `v26_e2_fall_off` | D-048 | 2 of 4 | A console read at the end of input raises error 62, and `END` does not wait; QB64pe times out. |
| `v26_e_end` | D-048 | 2 of 4 | A console read at the end of input raises error 62, and `END` does not wait; QB64pe times out. |
| `v26_x114_auto_const_accepted` | D-027 | 1 of 7 | The `CONST` holds the value beyond `_INTEGER64` as written. |
| `v32_b_get_put` | D-043 | 1 of 67 | The `_MEM` block of the released array is invalid. |
| `v32_d29_erase_dynamic` | D-043 | 5 of 4 | The `_MEM` block of the released array is invalid. The `_MEMPUT` ends the program with "Memory has been freed" and exit 1 (D-012). |
| `v33_x49_debug_vwatch_sub` | D-017 | built (QB64pe rejected it) | `$DEBUG` writes no debugger code, so the program builds. |
| `v34_d2_redim_bound_redims` | D-018 | 3 of 1 | The `REDIM` takes the array as the FUNCTION in its bound left it; QB64pe crashes. |
| `v34_d3_preserve_bound_redims` | D-018 | 3 of 1 | The `REDIM` takes the array as the FUNCTION in its bound left it; QB64pe crashes. |
| `v34_d_retain_bound_redims` | D-018 | 3 of 1 | The `REDIM` takes the array as the FUNCTION in its bound left it; QB64pe crashes. |
| `v35_g2_dyn_bounds_reversed` | D-020, D-040 | 4 of 4 | A dimension of fewer than no element raises error 5 and allocates nothing. |
| `v35_x58_parent_index_outside` | D-004 | 1 of 4 | A store into a place whose index is out of range raises error 9 and stores nothing. |
| `v35_x62_targets_through_members` | D-004 | 4 of 33 | A store into a place whose index is out of range raises error 9 and stores nothing. |
| `v35_x65_files_nested` | D-019 | 1 of 10 | The member with other bounds raises error 9 and keeps its elements. |
| `v35_x67_get_erased_array` | D-021, D-042 | 3 of 5 | `GET` into the erased array raises error 9 and reads nothing; the whole array with no storage raises error 9 and reads and writes nothing. The run traces D-042 as well, so its `.divergences` names both rows. |
| `v36_x48_member_index_outside` | D-004 | 1 of 7 | A store into a place whose index is out of range raises error 9 and stores nothing. |
| `v36_x52_owner_bounds_differ` | D-022 | 1 of 5 | The whole-array assignment raises error 9 and copies nothing. |
| `v37_x57` | D-032 | built (QB64pe rejected it) | The folded 0 passes a null pointer, so the program builds. |
| `v37_x66` | D-044 | 1 of 2 | C gets a copy of the `STRING * n` bytes with a NUL after them. |
| `v38_b_file_dims` | D-019 | 1 of 16 | The member with other bounds raises error 9 and keeps its elements. |
| `v38_d_owner_bounds` | D-022 | 2 of 10 | The whole-array assignment raises error 9 and copies nothing. |
| `v41_b_kinds` | D-025 | 1 of 18 | `_ARRAYCOPY` of `_FLOAT` elements copies the elements named. |
| `v41_i_float` | D-025 | 2 of 7 | `_ARRAYCOPY` of `_FLOAT` elements copies the elements named. |
| `v41_j_outside` | D-004 | 5 of 12 | A store into a place whose index is out of range raises error 9 and stores nothing. |
| `v43_a_errorline` | D-052 | 2 of 22 | Error 11 is trappable; untrapped, the message names the line. `_ERRORMESSAGE$(11)` is "Division by zero"; the program ends on an untrapped error, exit 1 (D-012). |
| `v44_x59` | D-051 | 3 of 1 | `_STATUSCODE` of a handle not open raises error 52; QB64pe crashes. |
| `v45_a_auto` | D-027 | 3 of 7 | The `CONST` holds the value beyond `_INTEGER64` as written. Line 3's `_UINTEGER64_MAX + 0` gives -1, as QB64pe's own sum does (`v21_g_const_suffix`, on `pass.list`). |
| `v45_a_lanes` | D-027 | 1 of 5 | The `CONST` holds the value beyond `_INTEGER64` as written. |
| `v45_b_conv` | D-027 | 1 of 7 | The `CONST` holds the value beyond `_INTEGER64` as written. |
| `v45_sample` | D-031 | 500 of 9501 | `_COTH` is the hyperbolic cotangent; at 0 in a `CONST` a compile error. Only lines 7001 to 7500, function 15 (`_COTH`), differ. |
| `v45_x25` | D-029 | rejected on line 3 (`lines.list`) | A `CONST` beyond `_FLOAT` range is a compile error. |
| `v45_x26` | D-026 | 1 of 2 | The `CONST` takes the correctly rounded value. |
| `v45_x27` | D-026 | 1 of 2 | The `CONST` takes the correctly rounded value. |
| `v45_x32` | D-031 | rejected on line 3 (`lines.list`) | `_COTH` is the hyperbolic cotangent; at 0 in a `CONST` a compile error. |
| `v45_x38` | D-029 | rejected on line 3 (`lines.list`) | A `CONST` beyond `_FLOAT` range is a compile error. |
| `v45_x40` | D-028 | rejected on line 3 (`lines.list`) | `^` of integers in a `CONST` is a float, or 0 to a negative power is a compile error. |
| `v49_x18_cast_out_of_range` | D-047 | 1 of 2 | `_CAST` out of range gives x86-64's conversion. |
| `v54_d_whole_erased` | D-042 | 3 of 7 | The whole array with no storage raises error 9 and reads and writes nothing. |
| `v54_x26_negative_beyond` | D-034 | rejected on line 3 (`lines.list`) | A negative literal beyond `_INTEGER64` is the compile error "Overflow". |
| `v54_x38_const_unsigned` | D-027 | 1 of 2 | The `CONST` holds the value beyond `_INTEGER64` as written. |
| `v54_x42_erased_string_print` | D-042 | 4 of 4 | The whole array with no storage raises error 9 and reads and writes nothing. Error 9 is not trapped: exit 1 (D-012). |
| `v54_x43_erased_fixed_string` | D-042 | 4 of 4 | The whole array with no storage raises error 9 and reads and writes nothing. Error 9 is not trapped: exit 1 (D-012). |
| `v54_x44_erased_string_read` | D-042 | 4 of 3 | The whole array with no storage raises error 9 and reads and writes nothing. Error 9 is not trapped: exit 1 (D-012). |
| `v55_b_implicit_whole` | D-042 | 4 of 13 | The whole array with no storage raises error 9 and reads and writes nothing. |
| `v55_d3_static_shared_never_called` | D-041 | 3 of 4 | `STATIC a()` in the SUB is an array of its own; QB64pe stops with "Duplicate definition". |
| `v55_x72_whole_in_sub_twice` | D-042 | 3 of 3 | The whole array with no storage raises error 9 and reads and writes nothing. |
| `v57_r02_label_main` | D-046 | 1 of 3 | `RUN` from the procedure or the console program runs the main module again; QB64pe loses output. |
| `v57_r03_line_main` | D-046 | 2 of 4 | `RUN` from the procedure or the console program runs the main module again; QB64pe loses output. |
| `v57_r04_label_sub` | D-046 | 1 of 4 | `RUN` from the procedure or the console program runs the main module again; QB64pe loses output. |
| `v57_r05_line_sub` | D-046 | 2 of 4 | `RUN` from the procedure or the console program runs the main module again; QB64pe loses output. |
| `v58_e21_off_div_main` | D-052 | 3 of 5 | Error 11 is trappable; untrapped, the message names the line. Untrapped, exit 1 (D-012). |
| `v59_f9_stop` | D-036 | 4 of 3 | `STOP` in `SUB _GL` ends with error 271; QB64pe hangs. Error 271 ends the program with exit 1 (D-012, decided after D-036, whose text still says exit 0). |
| `v60_c3_for_member_element_index_error` | D-004 | 4 of 2 | A store into a place whose index is out of range raises error 9 and stores nothing. The loop's counter `r.m(7)` stores nothing, so the loop runs to its `EXIT FOR`. |
| `v60_d6_string_type_nested_used` | D-037 | 2 of 2 | The `TYPE` variable's strings are made after its bytes are zeroed; QB64pe crashes. |
| `v62_a1_dim_below_zero` | D-040 | 4 of 4 | A dimension of fewer than no element raises error 5 and allocates nothing. |
| `v62_a2_redim_below_zero` | D-040 | 4 of 4 | A dimension of fewer than no element raises error 5 and allocates nothing. |
| `v62_b1_static_shared` | D-041 | 5 of 4 | `STATIC a()` in the SUB is an array of its own; QB64pe stops with "Duplicate definition". |
| `v62_c1_whole_erased_print` | D-042 | 5 of 6 | The whole array with no storage raises error 9 and reads and writes nothing. |
| `v62_c2_whole_erased_read` | D-042 | 3 of 4 | The whole array with no storage raises error 9 and reads and writes nothing. |
| `v62_c3_whole_erased_swap_input` | D-042 | 5 of 6 | The whole array with no storage raises error 9 and reads and writes nothing. |
| `v62_c4_whole_erased_get_put` | D-042 | 2 of 5 | The whole array with no storage raises error 9 and reads and writes nothing. |
| `v62_c5_whole_erased_field_lset` | D-042 | 4 of 5 | The whole array with no storage raises error 9 and reads and writes nothing. |
| `v62_d3_mem_after_clear` | D-043 | 5 of 4 | The `_MEM` block of the released array is invalid. The `_MEMPUT` ends the program with "Memory has been freed" and exit 1 (D-012). |
| `v62_d4_mem_after_run` | D-043 | 5 of 4 | The `_MEM` block of the released array is invalid. The `_MEMPUT` ends the program with "Memory has been freed" and exit 1 (D-012). |
| `v62_e1_fixed_element_char` | D-044 | 2 of 3 | C gets a copy of the `STRING * n` bytes with a NUL after them. |
| `v62_e2_fixed_member_char` | D-044 | 2 of 3 | C gets a copy of the `STRING * n` bytes with a NUL after them. |
| `v62_f5_resume_sub_mem` | D-045 | 1 of 5 | `RESUME NEXT` in the SUB leaves it through its exit, so its block is invalid. |
| `v62_g1_run_sub_many` | D-046 | 46 of 28 | `RUN` from the procedure or the console program runs the main module again; QB64pe loses output. The program runs to its end; QB64pe runs out of stack space. |
| `v62_g2_run_label_console` | D-046 | 1 of 2 | `RUN` from the procedure or the console program runs the main module again; QB64pe loses output. |
| `v62_g3_run_line_sub_console` | D-046 | 1 of 5 | `RUN` from the procedure or the console program runs the main module again; QB64pe loses output. |
| `v62_g4_run_function_in_expression` | D-046 | 1 of 4 | `RUN` from the procedure or the console program runs the main module again; QB64pe loses output. |
| `v62_h2_cast_constants` | D-047 | 11 of 13 | `_CAST` out of range gives x86-64's conversion. |
| `v63_a1_line_input_eof` | D-048 | 4 of 2 | A console read at the end of input raises error 62, and `END` does not wait; QB64pe times out. |
| `v63_a2_last_line_no_newline` | D-048 | 5 of 2 | A console read at the end of input raises error 62, and `END` does not wait; QB64pe times out. |
| `v63_a3_input_string_stdin` | D-048 | 6 of 2 | A console read at the end of input raises error 62, and `END` does not wait; QB64pe times out. |
| `v63_a4_input_string_eof` | D-048 | 4 of 2 | A console read at the end of input raises error 62, and `END` does not wait; QB64pe times out. |
| `v63_a5_input_values_eof` | D-048 | 4 of 2 | A console read at the end of input raises error 62, and `END` does not wait; QB64pe times out. |
| `v63_b1_bload_top` | D-049 | 1 of 3 | `BLOAD` or `BSAVE` past the block raises error 5. |
| `v63_b2_bload_size_zero_long` | D-049 | 4 of 1 | `BLOAD` or `BSAVE` past the block raises error 5. |
| `v63_b3_bsave_top` | D-049 | 2 of 4 | `BLOAD` or `BSAVE` past the block raises error 5. |
| `v63_b4_bload_header_place` | D-049 | 1 of 3 | `BLOAD` or `BSAVE` past the block raises error 5. |
| `v63_d1_statuscode_closed` | D-051 | 3 of 3 | `_STATUSCODE` of a handle not open raises error 52; QB64pe crashes. |
| `v63_e1_error_11_handler` | D-052 | 3 of 5 | Error 11 is trappable; untrapped, the message names the line. |
| `v63_e2_division_types_handler` | D-052 | 9 of 4 | Error 11 is trappable; untrapped, the message names the line. |
| `v64_a1_erase_dynamic_member_list` | D-055 | 4 of 8 | The index of `ERASE` is evaluated once. |
| `v64_a2_erase_dynamic_member_alone` | D-055 | 1 of 4 | The index of `ERASE` is evaluated once. |
| `v66_d06_const_sin_infinite_const` | D-029 | rejected on line 3 (`lines.list`) | A `CONST` beyond `_FLOAT` range is a compile error. |
