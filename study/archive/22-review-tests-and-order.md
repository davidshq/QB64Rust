# 22 — Second review: test inputs, yardstick and order of work

Written 2026-10-04 (session 11). A review of code and plan after `m2-procedures-and-errors` groups 1–5, asked to
look for real problems rather than nits, followed by a survey of the test files in `..\QB64pe\tests` and in
QB64Fresh (`<qb64contain>`). The user accepted the outcome ("do it"); the order in §5 replaces the one in `study\20`
§4 and is the plan (`STATUS.md`, "Next").

Read for the review: `STATUS.md`, `study\00`, `study\20`, `crates\README.md`, the design and tasks of
`m2-procedures-and-errors`, `crates\sema\src\check.rs`, `crates\ir`, `crates\codegen-cpp`, the parser, lexer, tree
and driver. Run: `cargo test` (passes). Not run: tier 2. Not reviewed: `vscode\`, `study\01`–`05`.

## 1. Verdict

No change of direction. The oracle discipline, "not supported yet" instead of wrong code, bytes throughout and C++
against the existing libqb ABI hold up, and nothing new is said here about the IR (`study\20` §3.4 stands). What
needs changing is the yardstick, the order, and three gaps the plan does not cover.

## 2. Measured for this review

Counted with regular expressions over the sources (string literals and `'` comments removed; good to a few
programs, not exact; script kept outside the repo).

**Upstream `tests\compile_tests` (404 programs).** 211 are in `arrays\`. 125 of them (31 % of the suite) use a
feature deferred to `SOMEDAY.md`: `$UNSTABLE:TYPEFIELDS` 109, `REDIM _RETAIN` 29, whole-array assignment 21,
`_ARRAYCOPY` 4 (a program may use several). So **279** programs are in reach of the plan. Feature use: `IF` 246,
arrays 196, `TYPE` 193, `OPTION _EXPLICIT` 187, unsigned types or `_BYTE`/`_BIT`/`_OFFSET` 115, `FOR` 97, without
`$CONSOLE:ONLY` 47, fixed-length strings 44, `$IF` 42, `CONST` 35, `$INCLUDE` 31.

Upper bound of upstream programs that can pass after each feature group (deferred features excluded; ignores
built-ins and operators):

| After adding | Programs |
|---|---|
| nothing (today's subset) | 25 |
| control flow (`IF`, `FOR`, `DO`, `SELECT`, `GOTO`, `GOSUB`) | 43 |
| unsigned types | 45 |
| arrays | 59 |
| `TYPE`, fixed-length strings | 115 |
| `$IF`, `$INCLUDE` | 149 |
| `CONST`, the auto-included constants | 176 |
| `DEFxxx`, `DATA`, line numbers | 195 |
| programs without `$CONSOLE:ONLY` | 242 |
| `INPUT`, `OPEN`, `PRINT USING` | 251 |

`$IF`/`$INCLUDE` unlock 34, more than control flow alone (18).

**Golden corpus (275 programs).** 146 use none of the feature groups above, yet 54 pass: the gap there is
built-ins and operators, which step 6 of §5 addresses.

**`qbasic_testcases` (143 programs).** None is `$CONSOLE:ONLY`; 90 % use `IF`, 79 % `FOR`, 74 % `DO`, 73 % screen
graphics, 50 % `INPUT`. They are compile-only upstream, so at run time nothing checks them.

## 3. Findings

### 3.1 The upstream suite is skewed, and the array deferral makes it more so
Half of the suite is `arrays\`, and 125 programs there need what `SOMEDAY.md` defers. Reported as "x of 404",
progress would stay low for long and look like failure. **Change:** report upstream progress as x of 279, with the
125 named in a list; keep the deferral, but design the storage and place model of the arrays/`TYPE` step so that
member arrays fit later (109 programs use `$UNSTABLE:TYPEFIELDS`).

### 3.2 Programs without `$CONSOLE:ONLY` are in no step and have no oracle
That is every `qbasic_testcases` program, 47 upstream programs and most real QB64 code. Lifting the restriction in
the compiler is probably small (no `$CONSOLE:ONLY`, the windowed libqb build); checking the result is not, because
there is no stdout to compare. **Change:** a queued item for M3: an oracle such as a dump of the screen state at
exit, from the libqb copy, compared between old and new compiler.

### 3.3 The plan stops at step 8, and the milestone table is stale
The language server and the formatter (both M2 deliverables in `study\00` §2) are in no step, so the lossless tree
and the symbol table still have no consumer. "Decide the bug-compatibility choices before M3 codegen" has passed
its trigger: 32-bit INTEGER arithmetic and half-to-even rounding are implemented. Missing from the steps:
`CONST`, `OPTION _EXPLICIT` (187 upstream programs, cheap but gating), `DEFxxx`, `GOSUB`, `INPUT` and files, the
auto-included BASIC files. **Change:** §5.

### 3.4 Single-file assumptions are growing in `sema`
`sema` reads source text through one `SourceFile` (`check.rs`, `text()` ignores the span's file) and keys
procedures and labels by node offset alone (`proc_of_def`, `label_of_def`). Each feature added before `$INCLUDE`
raises the cost of the retrofit, and every program has includes (the auto-included files). How the 172
`specialformat` statements are shaped in the tree is the largest open front-end question (`syntax` cannot see the
built-in table). **Change:** `$IF`/`$INCLUDE` and the `specialformat` statements move from the last step into the
parser-breadth step; `sema` keys by (`FileId`, offset).

### 3.5 No test reads the upstream suite, and nothing end to end runs in CI
Tier 1 reads only `tests\corpus` (and `tests\frontend`); the 404 upstream programs, the 143 `qbasic_testcases` and
the 72,700 lines of BASIC in the old compiler's own sources (`source\`, `internal\support`) are not used by any Rust
test, though "no panic, exact round trip" and "rejects what the old compiler rejects" cost nothing to check on them.
`rust.yml` runs tier 1 only. **Change:** the change `m2-upstream-tests` (§5 step 2).

## 4. Test files from other projects

### 4.1 QB64pe (`..\QB64pe\tests`, MIT, `COPYING.txt` and `licenses\`)

| Set | What | Decision |
|---|---|---|
| `compile_tests` | 404 programs: 331 with `.output`, 56 with `.err`, 17 compile-only; `.flags`, `.noprompt`, `.compile-from-base`, `.<os>.license`, `.bi`/`.bm`/`.h`/`.c` beside them. Text files 3.1 MB; images, fonts, DLLs and sound make the folder 239 MB | **Copy the text files** into `tests\upstream\` at the pinned commit, with QB64pe's licence. Tier 1 reads the copy (CI has no clone). End-to-end runs keep using the clone, which has the binary assets |
| `qbasic_testcases` | 143 classic programs by various authors, some with Microsoft copyright notices (`misc\gor64.bas`, `misc\nib64.bas`) | **Never copied.** Read from the clone by tier 1 (front end only); skipped when the clone is missing |
| `format_tests` | 5 programs, 24 expected outputs | Acceptance tests for the formatter when it is written; not now |
| `source\`, `internal\support` | 51 BASIC files, 72,700 lines: the old compiler and the auto-included files | Read from the clone by tier 1 (front end only); the auto-included files must compile anyway (`study\00` §4) |
| `c\`, `converter_tests`, `dist` | C++ unit tests of libqb, one converter test, a distribution check | Not ours; nothing to take |

### 4.2 QB64Fresh (`<qb64contain>\QB64Fresh`, the user's MIT code; `CLAUDE.md` rule 5)

| Set | What | Decision |
|---|---|---|
| `tests\runtime_comparison` | The 261 programs | Already taken (`tests\corpus`) |
| `tests\integration_tests.rs` | 449 inline BASIC snippets, each asserting only that QB64Fresh compiles it | **Taken as inputs only:** extract the snippets, label each accept/reject with `qb64pe.exe -z`, keep them as front-end tests (§5 step 2). Small probes of one feature each (`IF`, `FOR`, `SELECT`, suffixes…), which is what parser breadth needs. Their assertions are not taken |
| `tests\golden`, `tests\fixtures` | 22 programs with outputs produced by QB64Fresh | Not taken: not checked against QB64pe, and they overlap the corpus |
| `tests\bootstrap_tests.rs`, `tests\qb64pe_incremental` | Compiling `qb64pe.bas` in phases through extracted sections and stubs, against QB64Fresh's own C output | **The idea is taken, not the files:** reach `qb64pe.bas` (the M4 exit criterion) through its own include files, smallest first |
| `fuzz\`, `tests\proptest_tests.rs` | No-panic fuzzing of lexer and parser, inputs converted to `str` | Not taken (bytes rule); a fresh seeded mutation test over the corpus instead (§5 step 2) |

## 5. Order of work (accepted 2026-10-04, replaces `study\20` §4; replaced 2026-10-05 by `study\23` §4)

1. Finish `m2-procedures-and-errors` (group 6).
2. **`m2-upstream-tests`**: copy the upstream text files; tier 1 over upstream, `qbasic_testcases` and the old
   compiler's sources (no panic, exact round trip); upstream `.err` programs stay rejected; the "not supported yet"
   marker on diagnostics; a no-false-syntax-error ratchet list; a seeded mutation test; tier 2 over upstream with a
   pass list and a deferred list ("x of 279"); the QB64Fresh snippets as labelled inputs; shared
   `Ty`/`BinOp`/`Conv`; a tier-2 CI job if the QB64pe release can build our output.
3. **Parser breadth, the whole language**: blocks, control flow, labels, line numbers, `DATA`, comment
   metacommands, `$IF`/`$INCLUDE` (a tree per file), the `specialformat` statements; `sema` keyed by (`FileId`,
   offset). Done when the ratchet holds every program the old compiler accepts in the corpus, upstream,
   `qbasic_testcases` and its own sources. The driver's panic hook at the start (`study\21` item 6).
4. **A thin language server**: diagnostics (only those without the "not supported yet" marker, `study\15` §3) and
   go to definition, wired into the extension.
5. **Bug-compatibility decisions** (`study\00` §6), then the differential tester, `Ty` as a type table with an
   explicit promotion rank, and the unsigned types.
6. **Plain built-ins** (249 of 455), table-driven, with generated tests.
7. **Control flow through the pipeline**, with `CONST`, `OPTION _EXPLICIT` and `DEFxxx`.
8. **Arrays and `TYPE`**, storage designed with member arrays in mind; then the IR review (`study\20` §3.4).

Queued for M3: programs without `$CONSOLE:ONLY` with a screen-state oracle (§3.2); M4: `qb64pe.bas` through its
include files (§4.2).

Why parser breadth before the differential tester and built-ins (it was third of eight before too, but split, with
`$IF`/`$INCLUDE` and `specialformat` last): it is front-end only, so it is fast to test on every program we have;
it removes the single-file assumption before more `sema` code depends on it; and it is what makes a language
server useful on real files.
