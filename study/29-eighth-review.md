# 29. Eighth review: before the screen-state check (2026-10-10)

Asked by the user, before step 9a starts: review the code and the documents as an expert panel; anything
significant to change, other options to consider, specific tooling, bugs, ways to go faster. No nits.

The panel (`CLAUDE.md` rule 6, inline): a pragmatic engineer, a QB64 engineer, a compiler/languages engineer, a
Rust engineer, a test engineer, and a **runtime engineer** (added because the main finding is about libqb's exit
path and link line, which none of the other roles owns). Effort is counted by rule 8.

Evidence: `STATUS.md`, `ROADMAP.md`, `DECISIONS.md`, `SOMEDAY.md`, `study\28`, `crates\README.md`, the IR
(`crates\ir\src\lib.rs`), the emitter's statements (`codegen-cpp\src\stmt.rs`, `procs.rs`), the built-in table
(`sema\src\builtins.rs`), the build (`driver\src\build.rs`), `rust.yml`, the runner
(`tools\legacy_tests\run_legacy_tests.py`, `press_any_key.py`), and in the reference clone: `Makefile`,
`source\qb64pe.bas` (the `-f:` settings), `internal\c\libqb.cpp` (`end`, `sub_end`, `end_program`), `graphics.h`,
`main-thread-*.cpp`. Counts: 30,312 lines of Rust in 9 crates; 148 front-end tests, 256 call-site programs, 725
verification programs; no `TODO` or `FIXME` anywhere; `KNOWN BUGS: none`.

## 1. Verdict

**Nothing to change in the code, and no bug found.** The method of the last three reviews still holds and the code
read agrees with its documents. The findings are about step 9a's design, the order after it, and two gaps in the
test tooling.

| § | Finding | Kind |
|---|---|---|
| 2 | The screen-state check needs neither the libqb copy nor a change to libqb | Design of 9a; revises part of a decision |
| 3 | Keep 9a small; step 11 moves every yardstick, 9a moves none until M3 | Order |
| 4 | A demand report in place of hand counts | Speed |
| 5 | The runner, the judge of every test, has no tests of its own | Test engineering |
| 6 | Smaller points: CI's bootstrap of `qb64pe.exe` (withdrawn, §8), one coverage measurement, tooling not taken | Notes |
| 8 | The user's answers and what was done: the demand report's first numbers, the runner's tests, the CI point corrected | Record |

## 2. The screen-state check without the libqb copy

The decision of 2026-10-09 brings "the libqb copy forward to go with" the check, on the assumption that the hook
is a few lines added to libqb. Measured for this review (runtime engineer), it need not be:

- **Both compilers link extra code without any change to the clone.** The `Makefile` adds `CXXFLAGS_EXTRA` and
  `CXXLIBS_EXTRA` (lines 30–31); `qb64pe` exposes them as `-f:ExtraCppFlags=` and `-f:ExtraLinkerFlags=`
  (`qb64pe.bas` 13725, 14529–14532), and `driver\src\build.rs` already passes `CXXFLAGS_EXTRA`. So the hook is
  **one C++ file of its own** (say `tools\screen\screen_dump.cpp`), compiled once per toolchain and given to the
  old compiler as a linker flag and to the new one through `build.rs`. The clone stays untouched (`CLAUDE.md` rule
  3), the build cache keys it like any other input, CI changes by one flag.
- **Everything the dump needs is reachable from another translation unit.** `img`, `page`, `display_page`,
  `write_page`, `read_page` are non-static globals of `libqb.cpp` (lines 2401–2474); `img_struct`
  (`libqb\include\graphics.h` 17–59) holds `text`, `width`, `height`, `bytes_per_pixel`, `compatible_mode`,
  `offset`, `pal`, `color`, `background_color`, `font`, `cursor_x`, `cursor_y`, `cursor_show`. That is the whole
  list of `study\28` §3.1.
- **The hook runs at exit through `atexit`**, registered by a static initialiser of that file. Every end of a
  program reaches `exit()`: `SYSTEM` and `END` set `close_program` and spin in `end()`; the main loop's
  `end_program:` (libqb.cpp 26255) closes files and calls `libqb_exit`, which is `exit()` on the console path
  (`main-thread-console.cpp` 30) and on the GUI path either `exit()` or `GLUTEmu_ProgramExit` (to be read once in
  the design; it ends in `exit()` too or the hook is called from there). Image memory is heap and is not freed
  before that.
- **`END` draws on the screen before any hook can run.** `sub_end` (libqb.cpp 19281–19345) sets the display page
  as destination, `VIEW PRINT`s the bottom line, prints "Press any key to continue" **into the window**, and waits
  for a key through `qbs_inkey()`. Two consequences the design must state: (1) a program that ends with `END` or
  runs off its end leaves that line in its recording (harmless for the comparison, both compilers draw it, but the
  bottom text line is lost); a program that ends with **`SYSTEM`** draws nothing and waits for nothing, which is
  the convention the corpus already uses for `.stdin` programs; (2) `press_any_key.py` writes console input
  events, which a windowed program never reads, so today **no windowed program can be ended by the runner**. Of
  upstream's 47 programs without `$CONSOLE:ONLY`, 11 end with `SYSTEM` and 36 run off the end.
- **Scripted keys need no libqb change either**, in all likelihood: GLFW's window procedure takes
  `WM_KEYDOWN`/`WM_CHAR`, so a windowed twin of `press_any_key.py` can `PostMessage` keys to the program's window
  (found by process id) without focus. That both ends the 36 programs above and feeds `INKEY$` programs. To be
  measured first, as everything is; if it fails, `SendInput` with focus is the fallback, and only then a libqb
  edit.

**Recommendation.** Design 9a as: the dump file (one `.cpp`, an environment variable naming the output file, one
record per page in use plus which page is displayed and which is the destination, text cells or pixels, the
palette for 8 bits and below, cursor); the runner records it with the old compiler through
`-f:ExtraLinkerFlags` and compares it for the new one; a windowed key helper; test programs end with `SYSTEM`.
**The libqb copy stays where the 2026-10-02 decision put it, M3**, and comes when libqb must actually change. What
that saves: 52 MB and 1,757 files (1,200 tracked in the clone: freetype, curl, GLFW, libxmp) in this repo, a second
build root in `build.rs`, the cache keys, the runner's root arguments and CI, none of which 9a needs. The user
decided the copy on 2026-10-09 on the panel's own advice, so this is the panel correcting itself with the
measurement it did not have then.

Limits that stay: sound has no oracle; `RANDOMIZE TIMER` and `TIMER`-paced loops are cut or seeded; a program
must end by itself or by scripted keys; a window needs a desktop session (GitHub's Windows runner has one;
`STATUS.md` step 10 already plans that check).

## 3. Keep 9a small; then step 11

9a produces a recorder and recordings from the old compiler. **The new compiler cannot be compared with any of
them until it compiles a drawing statement**, which is M3 work (`SCREEN`, `COLOR`, `LINE`, `PSET`, …). So 9a moves
no yardstick; it removes the longest-lead unknown, which is why it was brought forward, and that is still right.
But it should be one change of a few tasks (the file, the runner, the key helper, the first recordings from the
game's cut programs and upstream's 47), not a change that waits on a copy of libqb or on a fixed-seed design for
the whole game.

After it, **step 11 before step 10.** Measured now with the release build on `CWSTRAT.BAS` (the first 100
diagnostics, the cap; the true list is longer): `COLOR` 22, `COMMON` 12, a label named `CLS` 10, `LOCATE` 9,
`PLAY` 6, `INKEY$` 6, `CLS` 5, `DEF SEG` 4, labels named like a SUB 3, then one or two each of the drawing
statements, `DEFINT`, `$EXEICON`. Of these, `COMMON`, `DEFINT`, the label rules and arrays in procedures are step
11; nothing is step 10. The corpus has 11 programs waiting on arrays, `TYPE` and `DEFINT`; upstream has 125 on the
deferred array features. Step 10 (`$CONSOLE` with `_DEST _CONSOLE`) moves only upstream, and only part of its
`_DEST` uses are `_CONSOLE`. The sixth review ordered 10 then 11 before the game existed; with the game as the
real-program target the order flips.

## 4. A demand report in place of hand counts

`study\28` §3 counted the built-ins each input set needs by hand, and §3 above was a `grep` over `--dump typed`.
No tool does this. **Recommendation:** a driver option, `qb64rust --report unsupported <file or folder>…`, that
runs the front end over every program with the error cap lifted and prints each "not supported yet" message with
its count and the number of programs it occurs in, over any set: the corpus, upstream, the snippets, the game,
`qbasic_testcases`. Run by hand before each change to choose what comes in; not part of tier 1. The `inputs.rs`
walk already does the loop; it is minutes of work and a few dozen lines to keep. What it gives: the next change's
scope chosen by measured demand rather than by recollection, and a number to put in `STATUS.md` beside "x of 279".

## 5. The runner has no tests of its own

`run_legacy_tests.py` is 1,455 lines and is the judge of tier 2, the recorder of every expected output, the owner
of the normalisation rules, the `.err` rule for the new compiler, the private copies and the locks. It has no unit
test; its correctness was shown by "three full corpus runs equal" (`DECISIONS.md` 2026-10-10). After 9a it also
records and compares the screen state. A change to `norm_output`, `new_compiler_rejection`, `apply_normalize` or
`bash_exit_code` can change what "pass" means without any test noticing (test engineer).

**Recommendation:** `tools\legacy_tests\test_runner.py` (pytest, run by `cargo test`'s Python step or by hand and
in `rust.yml`'s `test` job, which has Python): the pure functions above, `load_list`, `clone_stamp`,
`load_normalize`, plus one end-to-end case on a two-program toy corpus with a fake compiler (a Python script that
writes a given "executable" or a given error) covering pass, wrong output, `.err`, `.norun`, `.stdin` and
`--record`'s two-run rule. Minutes to write; the lasting cost is one file that changes when the runner does. Not
covered on purpose: the locks and copies (they need the clone; the full runs cover them).

## 6. Smaller points

- **CI's `tier2` job builds `qb64pe.exe` from source** (`mingw32-make BUILD_QB64=y`, never tried on a runner,
  10 minutes or more) only so the call-site check can run `-z`. ~~The 16 commits between the release and the
  reference commit change libqb only (`study\28` §2), not `source\`, so the release's `qb64pe.exe` writes the
  same C++. Using it drops the bootstrap and the risk.~~ **Wrong, withdrawn (§8):** the claim was taken from
  `study\28` §2, which lists the libqb files that changed and says nothing about `source\`; it was not measured.
  `source\qb64pe.bas` differs too. The job stays as it is.
- **One coverage measurement, not a CI job (Rust engineer):** `cargo llvm-cov` once over tier 1 plus the
  call-site set would show which arms of `sema\check` and `codegen-cpp` no program exercises, which is where a
  smaller model's slip would hide. A morning's reading, then decide whether any set needs a program.
- **Tooling considered and not recommended:** `cargo-nextest` (tier 1 is 20 s); `cargo-fuzz` (stays in
  `SOMEDAY.md`; the seeded mutation test and the five input sets give the "no panic, exact round trip" property
  enough inputs for now); `cargo-mutants` (the test sets are measured against the old compiler, which is a
  stronger oracle than a mutation score); any new dependency (none needed; the dependency policy holds).
- **`ROADMAP.md` is untracked** although `CLAUDE.md` and `STATUS.md` cite it; it needs staging.
- **Read and found sound:** the IR's pending-error rule and its emitter (`stmt.rs`), the store rules per place,
  the `_BIT` store, the handler and `RESUME` lines, the build cache's key (every input `make` reads, FNV-128,
  rename-into-place), the build folder's name rule, the table-driven built-ins with one rule per kind of
  special-casing, the lint set, the shrink-only lists and "clean programs are listed".

## 7. Questions for the user

1. **9a's design (§2):** the hook as a separate C++ file linked through the existing extra flags, `atexit`,
   programs ending with `SYSTEM`, a windowed key helper by `PostMessage`; **the libqb copy stays at M3** (revising
   the "brought forward" part of the 2026-10-09 decision)?
2. **Order (§3):** 9a as one small change, then step 11 (`DEFxxx`, `COMMON`, arrays in procedures, `REDIM`,
   dynamic arrays, member arrays), then step 10?
3. **The demand report (§4)** and **the runner's tests (§5):** both small; do them inside 9a's change, or as their
   own change first?
4. **CI (§6):** leave as is until CI matters, or switch the call-site check to the release's `qb64pe.exe` now?

## 8. The user's answers (2026-10-10) and what was done

The user asked for the panel's recommendation on each question, then: "do recs".

| Question | Recommendation, accepted | Done |
|---|---|---|
| 1. 9a's design | **Yes**, as §2; the libqb copy stays at M3 and comes the day libqb must change (the first candidate: scripted keys, if posting them to the window fails its measurement) | `DECISIONS.md` 2026-10-10 (revises the "brought forward" half of 2026-10-09); `STATUS.md` step 9a rewritten, with what is measured first; `ROADMAP.md` |
| 2. Order | **Yes**: 9a as one small change (the file, the runner's record and compare, the key helper, first recordings; no copy, no fixed-seed session of the game), then arrays, `TYPE`, `DEFxxx` and `COMMON`, then `$CONSOLE` with `_DEST` | `STATUS.md` "Next": the last two steps swapped and renumbered (10 was 11, 11 was 10); `ROADMAP.md` |
| 3. Demand report and runner tests | **Their own change, first**: the runner's tests should exist before the screen-state recorder is added to the runner, and the report sets the scope of 9a's first inputs and of the step after it | OpenSpec change `m2-demand-report-and-runner-tests` (archived 2026-10-10), below |
| 4. CI | **Switch now** was the recommendation. **Not done: its reason was wrong** | Below |

### The demand report

`qb64rust --report unsupported <file or folder>... [--include-root <dir>]` (`crates\driver\src\report.rs`,
`crates\README.md`). Per "not supported yet" message: the programs it occurs in, the programs it alone holds
back, its uses; the error cap lifted; programs with an `.err` file left out. First results (2026-10-10):

| Set | Programs | Result |
|---|---|---|
| Civil War Strategy | 1 | 1,740 marked errors in 38 messages: `LINE` 899, `COLOR` 190, `LOCATE` 145, `PAINT` 89, `DRAW` 71, `POINT` 34, `SOUND` 33, `PLAY` 31, `PSET` 31, `CIRCLE` 28, `CLS` 26, `INKEY$` 25, `TAB` 22, graphics `PUT` 19, `COMMON` 12, `VIEW` 11, the label `CLS:` 10, `SCREEN` 9, `DEF` (its `DEF SEG`) 8, `GET` 8, `BLOAD` 4, arrays in a procedure 3 (more will show once `COMMON` and `DEFINT` are in: the follow-on rule hides errors after a marked declaration) |
| Corpus | 287 (20 more have an `.err`) | 260 clean, 27 held back; 14 of the 27 by a single message each (`_IIF`, `PRINT USING`, `SPC`, `TAB`, `_CLAMP`, `CHAIN`, `DEFINT`, `EXIT SELECT`, `LPRINT`, `OPTION BASE`, `WIDTH`, whole-`TYPE` assignment, `ERASE`, `GET`) |
| Upstream | 348 (56 more have an `.err`) | 38 clean, 300 held back, 10 with included files only the clone holds. By programs: arrays as `TYPE` members 152, `REDIM` 148, `$Unstable` 85, `LBOUND` of an expression 63, `_DEST` 54, `$CONSOLE` 34, the type `_MEM` 30, `ERASE` 30, `STRING` members 29 |

This confirms §3 with numbers: arrays and `TYPE` (the new step 10) lead upstream by a wide margin over `_DEST`
(the new step 11), and the game needs `COMMON` and `DEFINT` before anything else shows.

### The runner's tests

`tools\legacy_tests\test_run_legacy_tests.py`: 41 tests on the standard library's `unittest` (not `pytest`, as §5
said: nothing to install), under a second, in CI's `test` job. The pure rules, and every outcome of one corpus
program with `run_proc` replaced by a stand-in for the compiler and the program. Checked that they bite: 15
mistakes seeded into the runner one at a time (a bare CR ignored, a marked-only failure counted as a rejection, an
internal compiler error counted as one, differing runs recorded, the output never compared, and ten more), all 15
caught.

### CI: the recommendation was wrong

§6 said the release's `qb64pe.exe` writes the same C++ as the reference commit's because the 16 commits between
them change libqb only. Measured before changing the workflow (`git diff --stat 26e0f6c132 16f629784e` in the
clone): `source\qb64pe.bas` changes too, 9 lines in `evaluatefunc`: `VAL(s$, SINGLE)` and `VAL(s$, DOUBLE)` are
written `(float)qbs_val<long double>(…)` and `(double)qbs_val<long double>(…)` with their own result types, where
the release writes `qbs_val<long double>(…)` typed `_FLOAT` for all three float types. No program of
`tests\callsite` uses typed `VAL` today, so the switch would have passed, and would have compared against the
wrong compiler from the day one did. It also contradicts the decision that the reference clone is the one
reference (`DECISIONS.md` 2026-10-09). **CI keeps building `qb64pe.exe` from the reference commit**; `rust.yml`
says why. A fact for `study\28` §2 as well: the two builds differ in the compiler, not only in libqb.

The new compiler follows the reference commit here (measured, `--dump cpp`): it writes `((float)qbs_val<long
double>(…))`, `((double)qbs_val<long double>(…))` and `qbs_val<long double>(…)` (`codegen-cpp\src\builtins.rs`),
as measured against the clone's `qb64pe.exe` by `verification\v20_b_result_types`, `v20_d_val` and the slice
program `s26_val_len_radix`. A call-site program for typed `VAL` would pin the call itself and is worth adding
with the next built-in work.
