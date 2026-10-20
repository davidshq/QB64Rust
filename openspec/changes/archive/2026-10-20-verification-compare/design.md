# Design

## Context

`verification_test` in `tools\legacy_tests\run_legacy_tests.py` records one program. It compiles with
`qb64pe -x -q -m` in `verification\` itself (so `$INCLUDE: 'v16_inc\…'` and the `v37` libraries are found),
writes `<name>.exe` there, and runs it with `run_proc(..., ask_first=True)`, without `press_any_key.py`. It then
writes `<name>.compile.txt` (output, then `exit=<bash code>`) and `<name>.out.txt` (output, then `exit=…`,
`(timed out after 60 s)\nexit=124`, or `(not compiled)`), with the folder masked as `<verification>`. `main()`
refuses the suite for any compiler but `qb64pe` (`is_old_compiler`, at the `--suite verification` check).

What the recordings hold (counted 2026-10-19 over 2,842 programs):

| Recorded | Programs |
|---|---|
| `(not compiled)` | 1,710 (31 where the old compiler's C++ build failed) |
| ran, `exit=0` | 1,059 |
| ran, other code (127, 2, 3, 4, 139, 5, 9) | 20 |
| timed out (`exit=124`) | 7 |
| no `.out.txt` (`v40_g2`, `g3`, `g4`, `g6`: the executable did not start) | 4 |
| binary `.out.txt` (`v45_x35`) | 1 |

The corpus suite already has the parts a compare needs: the rejection rule (`new_compiler_rejection`), the trace
of divergences (`set_trace`, `allowed_rows`, `unexpected_rows`), normalisation (`load_normalize`,
`apply_normalize`), `--pass-cache` (`pass_key`), `--build-cache` (passed to the driver), `run_all` with `--jobs`,
the compiler copy (`compiler_copy.py`), and `--check-list` (`check_list`, `result_key`, `program_inputs`).

## Goals / Non-Goals

**Goals:**

- Every recorded program counts, in one of two lists, each with a reason when it is not a pass.
- A compare can run beside other work and beside other runs: it writes nothing into `verification\` and nothing
  shared between its own programs.
- The compare costs a close no more than the slice list does: with `--pass-cache`, only changed executables run.

**Non-Goals:**

- Comparing the wording of rejections or the compiler's output for accepted programs.
- New recordings, or changes to how the old compiler is recorded.
- The CI `tier2` job (a `SOMEDAY.md` row).
- Matching machine-dependent results (`CLAUDE.md` working rule 12): such a program is listed with its row.

## Decisions

### D1. The compiler picks the mode

`--suite verification` records with `qb64pe` and compares with anything else, as the corpus suite compares `.err`
text with `qb64pe` and applies the rejection rule with anything else. The refusal in `main()` is removed and the
dispatch in `run_suite_test` calls `verification_test` (record, unchanged) or a new `verification_compare`.
Alternative: a `--compare` flag. Rejected, since there is nothing to compare when the compiler is `qb64pe` and
nothing to record when it is not, so a flag could only be given wrongly.

### D2. A fresh folder per program, with the family's support files

`verification_compare` makes `<results>\verification\<name>\`, copies the program, its `.stdin` and `.noprompt`
into it, plus the support items of its family, and compiles there with the output in the same folder. A new
function `verification_support(root, name)` picks the support items: entries of the verification root whose name
begins with the program's family number (the `vNN` of `vNN_…`, so `v37` matches `v37c.a` and `v37_inc\`) or with
no family number at all (`cp437_all_bytes.bin`). It excludes `*.bas`, `*.compile.txt`, `*.out.txt`, `*.exe`,
`*.qb64rust\`, `*.py`, `*.md`, `*.sh`, `*.txt`, `__pycache__\`, other programs' `*.stdin` and `*.noprompt`, and
`v40_rc\` (recorded resource files). The list is taken once per run. The folder is deleted when the program
passes, as in the corpus.

As built: `v37_lib\` is copied, not excluded, since about 70 `v37` programs name its headers
(`DECLARE LIBRARY "v37_lib/v37h"`). The first full run found one program that uses another family's file
(`v40_f_with_icon`: `$EXEICON:'./v29_inc/icon.ico'`), so the rule takes, besides the family's items, every item
whose name the program's source holds. A failure of a program that `known.list` names counts as a known failure
(`KFAIL`), and one of its programs that passes is named at the end of the run, as for `known_failures.txt`.

Alternative: compile in `verification\` as the recorder does. Rejected, since parallel programs would share the
folder and the executables, and the folder that holds the old compiler's truth would be written by the new one.
Alternative: copy the whole support set into every folder (about 800 KB, about 2 GB per run). Rejected for the
writes. A program that includes another family's file shows up in the first run and gets that family too (a
rule in `verification_support`, not a list).

### D3. Run as recorded, out of sight

The run uses the recorder's conditions: `VERIFICATION_RUN_TIMEOUT`, `ask_first=True`, `QB64PE_NOPROMPT` from
`.noprompt` or `y`, stdin from `.stdin` or the null device, and no key presses (a program at `END` waits and times
out, as recorded). Windows go on a desktop that is never shown. `press_any_key.py` gets `--no-keys`: with it,
the helper starts the program on the hidden desktop with stdin as given and writes no key presses.
`--hidden-desktop --no-keys` is used for every compared run. Alternative: run with `press_any_key.py` as the
corpus does. Rejected, since it would end the 7 recorded timeouts early and change what they print.

### D4. What is compared

In `verification_compare`, judged from the recorded `<name>.out.txt`:

- `(not compiled)`: the compile must fail with no executable, and `new_compiler_rejection` must give "" (at
  least one error not marked "not supported yet"). The 31 recorded C++ failures are judged the same way. Where
  the new compiler builds such a program, the difference is a `known.list` reason (a `DIVERGENCES.md` row, or a
  bug).
- Any other recording: the compile must succeed. The actual text is built as the recorder builds it (the run
  folder masked as `<verification>` with the recorder's case- and slash-blind pattern, then
  `exit=<bash_exit_code(rc)>`, or the timeout lines), and is compared with the recorded text by `norm_output`
  after `apply_normalize` on both sides. One function, `verification_text(out, rc, timed_out, folder)`, now
  writes the text for both the recorder and the compare, so the two cannot drift.
- No `<name>.out.txt`: FAIL, stage "expected", "nothing recorded to compare".
- `<name>.compile.txt` is not read. The compiler's output for an accepted program differs in wording, and the
  outcome is already in `.out.txt`.

The result key is `("verification", name)`, as the runner's error path already writes it.

### D5. The lists live in `tests\verification\`

`tests\verification\pass.list` and `tests\verification\known.list` (one name per line; in `known.list` the name,
a tab, the reason), `divergences\<name>.divergences`, `normalize\<name>.normalize`, and a `README.md` with the
counts block that `counts.py` writes. The reasons are `D-0nn`, `not supported yet`, `varies`, `not recorded`,
`recorded elsewhere` (`v37_x70`) and `bug: <what>`. `verification\` stays the old compiler's measurements
only, like the clone's test folder that `tests\upstream\` lists against. Alternative: lists inside
`verification\`. Rejected, since that folder is written by recordings and its README catalogues families.

The tier-1 check that each program is named exactly once goes in `crates\driver\tests\inputs.rs`, beside the
upstream lists' test (`verification_lists_are_exact`). `counts.py` gets `verification_membership` and a results
block from `results-verification.json`, as for upstream. `check_list`, `result_key` and `program_inputs` take
suite `verification`. For `program_inputs`, `.exe` is left out of the `<name>.*` glob, and the two sidecars in
`tests\verification\` are added.

### D6. The first full run is sorted, not fixed wholesale

The first full compare puts every program on one list. A failure is looked at once and classified:

- **Our bug, local** (one form, one fix in one module, its test made from the family's program): fixed in this
  change, and the program goes on `pass.list`.
- **Our bug, wider:** `known.list` with `bug: <what>`, and a line in `STATUS.md`'s known bugs. Those lines are the
  input for the next change.
- **A decided difference:** `known.list` with its `D-0nn`, or a `.divergences` sidecar when the run takes the row
  and still prints the same.
- **Run-dependent output** (time, addresses, handles): a `.normalize` sidecar when a rule makes both sides equal,
  else `varies`.
- **"Not supported yet"** where the old compiler accepted: `known.list`. The demand report already names these.

The policy keeps the change bounded however many failures the first run finds. The count of each reason goes in
`tests\verification\README.md`.

### D7. Cadence

`tools\close\close.py` gets a step `verification`:
`--suite verification {NEW} {CACHE} --list tests/verification/pass.list`, with `copy=True`, judged as a runner
log. Its test in `test_close.py` is updated. This replaces the closing set's hand runs of "an accepted program of
the family that none of them covers" (`CLAUDE.md` working rule 9, whose text is updated). A change that adds a
family adds the family's programs to one of the lists in its own tasks. The merge set gets a full compare and
`--check-list tests/verification/pass.list`. Rule 9 and `tools\legacy_tests\README.md` ("The runs at a merge into
`main`") name it.

### D8. Machine dependence

Under working rule 12, a program whose recorded value comes from the CPU or the C runtime's math is listed with
its row (`v45_b_funcs`: D-026 on 3 of 75 constants; `v45_sample`: the library results themselves). A
`.normalize` sidecar must not hide such a value.

## Risks / Trade-offs

- [A full compare is long: about 1,130 C++ builds] → `--jobs 8 --build-cache target\build-cache`. The 1,710
  rejections build nothing. The first run is in the background while the triage of its early results goes on
  (`study\archive\34`). Closes use `--pass-cache`.
- [Recordings come from one run, and some may vary from run to run] → classified `varies` or normalised (D6),
  never recorded again with the new compiler.
- [Programs that need a device: sound (`v29`, `v39`), the network (`v44_e` listens on 47391, and Windows may ask
  once), windows (`v27`, `v31`)] → hidden desktop (D3). A run that cannot work here is `known.list` with its
  reason. A firewall prompt is answered once by the user, as for the recording.
- [A support file outside the family rule] → the first run shows it as a compile error naming the file. The rule
  is widened, never a per-program list.
- [`v37` libraries are git-ignored and built by `v37_lib\build_libs.py`] → `verification_compare` fails such a
  program with "run v37_lib\build_libs.py first" when the library is missing, rather than as a bug.
- [The lists grow stale when a family is added] → the tier-1 exact-membership test fails until the new programs
  are listed.

## Migration Plan

None. The recorder is unchanged. The new lists start empty and are filled by the first full run.

## Open Questions

- Whether the CI `tier2` job should run `tests\verification\pass.list` (its time limit). This is a `SOMEDAY.md`
  row, to be decided at a merge.
