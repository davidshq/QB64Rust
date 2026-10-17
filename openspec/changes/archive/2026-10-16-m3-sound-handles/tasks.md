# Tasks

Order: group 1 first (the measurements), then 2 and 3 (design D7), then 4. Each of groups 2 and 3 ends with
`cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test` (tier 1) green and
with its tests, lists and `crates\README.md` lines in place, so the change can stop after either.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools;
`python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done. **Test cadence**
(`CLAUDE.md` working rule 9): while working, tier 1 and only the programs the work touches (`--glob`,
`--category`, or a list with `--pass-cache target\pass-cache`), each run's output sent to a log under `target\`;
tier 2, the run without folding and the full suites run once, in group 4. CI is not a step of this change. A new
measured fact goes into the `v39` paragraph of `study\00` §5 in the task that measures it. A behaviour of the old
compiler that looks questionable is implemented its way and listed in `SOMEDAY.md` ("QB64pe behaviours to
review").

## 1. Measurements before code

- [x] 1.1 Measure with `qb64pe.exe`, one program each (`verification\v39_…`, the C++ read with `-z` outside the
  repository), what the design and the spec leave to measurement:
  - the accepted forms of each of the 26 names (every optional argument left out in turn, a trailing comma,
    `CALL` with and without parentheses, a colon after a statement's name, each numeric type in a slot) and the
    rejected ones (too few or too many arguments, a string for a number and the other way, parentheses around a
    statement's arguments, a function called bare that needs an argument, a statement used as a function);
  - the arrays of `_WAVE` and `_SNDRAWBATCH` (design D2): a whole array, an element, a member array, an array
    parameter of a `SUB`, an array of each other numeric type, `_UNSIGNED _BYTE`, a variable, an expression;
  - `$MIDISOUNDFONT` alone, with a colon and a value, in lower case, after `'` and `REM` (design D3);
  - the run-time values of the spec's scenarios, from a WAV and a MIDI built in strings and a WAV file the program
    writes: each value at volume 0 and at the default volume (a program with the volume left out run once, by hand,
    not recorded as a test), each program run twice alone and eight at a time (design D5), and whether a value
    differs between an `_SNDOPEN` with and without `"noasync"`;
  - `_SNDPAUSED`, `_SNDGETPOS` and `_SNDLEN` of a copy, of a sound after `_SNDLIMIT`, and of a raw handle;
  - what `_SNDPLAYFILE` leaves to observe (`_SNDOPENRAW`'s next handle number before and after it, read only to
    decide D5's rule, not printed by a test).
  Verify: `verification\` holds the programs with their `.compile.txt` and `.out.txt`; the `v39` paragraph of
  `study\00` §5 and `verification\README.md` say what was found; the design and the spec scenarios name forms and
  values the old compiler gives, corrected where they did not; the reference clone's `git status --porcelain` is
  empty before and after.
  Found (2026-10-16): 84 programs, `v39_a_handles`, `v39_b_new_mem`, `v39_c_files_raw_midi`, `v39_x01_accepted`
  (160 calls, every one the plain call of its row), `x02`–`x72` (62 rejected, 8 accepted: a bare array name
  `_WAVE 0, w`, a suffixed array, an array parameter, `_SNDRAWDONE:`), `x73`–`x81` (`$MIDISOUNDFONT:` only with its
  colon). Corrected in the design: D2 (a member array is rejected, the bare name accepted), D3 (only the prefix
  with the colon), D4 and D5 (a decoded sound's block, length and position follow the device's rate, so only whole
  half-seconds are printed; the open of a file just written failed once in 16 runs under load, and `KILL` of a
  file being played raises 75, so both are tried again), a risk (an access violation after the last line in two
  of about 50 runs of `v39_a`); in the spec, the arrays requirement and `$MIDISOUNDFONT:`. The values of every
  spec scenario were as written. Volume 0 and `"noasync"` changed nothing printed; the clone stayed clean.

## 2. The rows compile (design D1 to D4)

- [x] 2.1 `sema`: the ten functions in `SUPPORTED` and the sixteen statements in `STATEMENTS`, each `Plain`, with
  comments that name this change's design and `v39`; the element type check of `_WAVE` and `_SNDRAWBATCH` in
  `check\builtins.rs` with the old compiler's messages (D2); `$MIDISOUNDFONT` as the old compiler's error (D3);
  `_MEMSOUND` off `NEVER_RUN` (D4). Where a form of task 1.1 does not compile as measured, the fix goes into the
  template plan or the parser for every statement, and this task says which. Tests: an `ir` front-end test
  (`sound_handles_ir.bas`: every accepted statement form, the settings line with `audio`), a `typed` one
  (`sound_handles_typed.bas`: every function form, bare where it may be, with each numeric type, in an
  expression), a `check-fail` one (`sound_handles_errors.bas`: the rejected forms of `v39_x*`, the arrays of D2
  and `$MIDISOUNDFONT`, each an error that is not "not supported yet"). Verify: `cargo test` passes; no existing
  snapshot changes but those that held a mark for one of these names (this task names them).
  Found (2026-10-16): the rows alone compiled every accepted form and rejected every arity and type error as
  measured; no template plan or parser change was needed. Three fixes in `sema`: `typed_array_arg`
  (`check\builtins.rs`) for the arrays of D2, where `block_arg` alone marked a variable, a member and a string,
  `_BIT` or `TYPE` array "not supported yet"; `$MIDISOUNDFONT:` (`check\mod.rs`); and a supported built-in
  statement's name with a leading underscore used as an array or FUNCTION (`l = _SNDPLAY(h)`) is now the error the
  old compiler gives ("Invalid array name", `check\places.rs`). That last one changed one existing snapshot,
  `stmt_name_assigned` (`_LIMIT (60) = 60`, measured "Invalid array name" in `v27_x57`), from a mark to an error.
  Also a `cpp` front-end test, `sound_handles_cpp.bas`, for the arrays from an element, which the call-site check
  does not compare (2.2). `cargo test` was green only with group 3 in place (the coverage check wants the slice
  programs, and the clean-programs check the upstream programs that now compile), so group 2 did not end green
  by itself.
- [x] 2.2 Call-site programs, one per accepted form of task 1.1 (`tests\callsite\snd*.bas`, `midisoundbank_*.bas`,
  `wave_*.bas`). Verify: `python tools\callsite\callsite.py` on these programs alone reports every one equal; a
  difference is fixed in 2.1's code, not in the tool.
  Found (2026-10-16): 131 programs, all equal. Of the first 136, ten differed in what the tool documents it does
  not compare (`tools\callsite\README.md`): an array element's index (the element forms went to
  `sound_handles_cpp.bas` instead), a LONG or SINGLE in a `double` slot of `_SNDSETPOS` and `_SNDLIMIT` (the
  programs pass a DOUBLE variable or a float literal), and arithmetic inside `PRINT`; the programs were changed as
  the README says, no code.
- [x] 2.3 Documents of the group: `crates\README.md` (the rows, the array check, `$MIDISOUNDFONT`, `_MEMSOUND`
  run), the purpose line of `language/sound` if it names only `SOUND`, `PLAY` and `BEEP`
  (`openspec\specs\language\sound\spec.md`, edited directly), `SOMEDAY.md` (the row of "the other sound
  built-ins" taken out; the questionable behaviours of the design's risks added). Verify: `qb64rust --report
  unsupported` on the eight upstream programs of the proposal names only `$EMBED` (and `_EMBEDDED$`) for
  `midisoundbank_embed_test` and nothing for the other seven.
  Found (2026-10-16): as stated. `SOMEDAY.md` also got "Left marked by `m3-sound-handles`".

## 3. Sound handles that run (design D5, D6)

- [x] 3.1 Slice programs, console, silent, from `s127` (design D5): handles and their state (open from memory and
  from no file, length, position, copy, play, pause, stop, loop, volume, balance, limit, a handle that is not open
  under an error handler, `CALL` forms); new sounds and their blocks (`_SNDNEW` with each refused argument,
  `_MEMSOUND` of a new sound and of a decoded one, `_MEMGET` of a sample); files, raw sound and MIDI (a WAV file
  written and opened, `_SNDPLAYFILE` at volume 0, `_SNDOPENRAW`, `_SNDRAW`, `_SNDRAWBATCH`, `_SNDRAWDONE`,
  `_SNDRAWLEN` against limits, `_SNDRATE > 0`, `_WAVE` and `SOUND` with waveform 10 at volume 0, the default bank
  and a missing bank with a MIDI from memory). Each recorded twice with `qb64pe.exe`, the recordings equal, and run
  once eight at a time beside the slice programs of another category. Verify: each passes with the new compiler
  (`run_legacy_tests.py --suite corpus --glob` for them) and is on `tests\corpus\slice.list`; `cargo test`'s
  coverage check passes for all 26 names and `_MEMSOUND`.
  Found (2026-10-16): `s127_sound_handles`, `s128_sound_new_memory`, `s129_sound_files_raw_midi`, each recorded on
  its first try (two equal runs), passed with the old compiler eight at a time beside `s110`–`s126` (whose
  failures there are the three programs recorded from edited copies, `s114`, `s117`, `s118`), and passed with the
  new compiler on its first run. `s129` tries its file's open and `KILL` again (design D5). The recording showed
  that `v39_c` was misread in task 1.1: a sound bank of bad data does not make the next MIDI open fail; `study\00`
  §5 corrected.
- [x] 3.2 The upstream programs of the proposal run from the clone (design D6). Verify: each passes and joins
  `tests\upstream\pass.list`, or joins `tests\known_clean_not_passing.list` with its reason; this task says how
  the runner counts `audio_test/test`, which has no `.output`; `tests\upstream\README.md` has the new count.
  Found (2026-10-16): all seven pass and joined `pass.list` (325 of 399). `audio_test/test` has neither `.output`
  nor `.err`, so the runner compiles it only, as `compile_tests.sh` does ("compile-only" in its result line).
- [x] 3.3 The shrink-only lists regenerated; any other program that now compiles cleanly is run and joins a pass
  list or `tests\known_clean_not_passing.list` with the reason. Verify: `cargo test` passes with the lists as
  written; `tests\corpus\README.md` has the new counts.
  Found (2026-10-16): no list changed; no other program compiles cleanly now. `cargo test`: 328 pass.

## 4. Closing runs and documents

- [x] 4.1 The closing runs, each once, without `--pass-cache` (`--jobs 8 --build-cache target\build-cache`), each to
  a log under `target\`: `cargo test`, the `--ignored` command-line tests, the runner's tests, the call-site check,
  the slice list with and without folding (`QB64RUST_NO_FOLD=1`), the upstream and differential pass lists, the two
  full suites, the demand reports. Verify: every run passes or its failures are known failures; the numbers are in
  `STATUS.md`. A later edit is rerun only for what it can affect, and this task says what was not repeated and why.
  Found (2026-10-16): all after the last code change. `cargo test` 328 pass (the run of task 3.3, after which only
  documents changed); `--ignored` 33 of 33; the runner's tests 80; the slice list 440 of 440, also without
  folding; upstream 325 of 325, differential 59 of 59; the full corpus 459 pass, `218_fre` (only marked) and the 5
  known failures; the full upstream suite 325 pass, 78 rejected (65 at the compile, 13 `.err`), 1 known failure,
  none wrong at run time; the call-site check 1808 of 1808; the demand reports as in `STATUS.md`. Nothing was
  edited after them but documents, so nothing was run again.
- [x] 4.2 Documents: `STATUS.md` (step 16 done, the next step, the numbers), `ROADMAP.md`, `study\00` §5 and its
  document map if a study is added, `CLAUDE.md`'s layout table where a family or count is named (`v39*`). Verify:
  `openspec validate m3-sound-handles --strict` passes; `python tools\repo_check\check_repo.py --untracked` is
  clean; the counts in the documents equal those of the lists.
  Found (2026-10-16): no study added; `CLAUDE.md` names no `v39` family or count (its row points at
  `verification\README.md`), so it is unchanged. Valid, clean; `pass.list` 325, `slice.list` 440,
  `tests\callsite` 1808, as the documents say.
- [x] 4.3 `python tools\testtime\test_time.py`. Verify: its three files are named among those to stage, with the
  commit message for the change.
  Found (2026-10-16): 275 sessions and 4860 runs on record.

## Workflow follow-up

- Archive the change (`openspec archive m3-sound-handles`) when the user has looked at it; the delta specs merge
  into `openspec\specs`.
- `main` gets the merge of `changes` at the user's word, every two or three changes (`DECISIONS.md` 2026-10-16).
