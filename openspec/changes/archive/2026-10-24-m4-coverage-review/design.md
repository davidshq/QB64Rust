# Design

## Context

See `proposal.md`, "Why". What exists:

- **Line coverage** (`cws-whole-game`): `qb64rust --coverage` puts a counter before each source statement
  (`crates\codegen-cpp\src\coverage.rs`, `Counters::count`); at exit the program appends a record
  (`coverage 1`, one entry per file, line and column, with its count) to the file `QB64RUST_COVERAGE` names. The
  driver builds the `Coverage` value (`crates\driver\src\lib.rs`, `coverage.then(...)`) with the files and a
  `position` callback. The runner takes `--coverage <file>` for `--suite corpus` only
  (`tools\legacy_tests\run_legacy_tests.py`, the check after the arguments are read).
  `tools\coverage\coverage.py` merges records of one program and checks Civil War Strategy.
- **Built-in coverage of tier 1** (`crates\driver\tests\inputs.rs`): each name of `sema`'s tables
  (`builtins::STATEMENTS`, `builtins::supported()`, the test's `OWN_NODE`) must be *written* in a slice program
  and pinned by a front-end test; each name of `builtins::NEVER_RUN` (40) must have a call-site program. Nothing
  says whether the written use is reached.
- Many built-ins are no `Op::Builtin` or `ExprKind::Call` of the IR (`PRINT`, `OPEN`, `LINE`, `GET`, `_MEMGET`
  have nodes of their own), so the IR alone cannot name them.

## Goals / Non-Goals

**Goals:** a name-level answer, from real runs of the suites, to "which built-ins does no test run"; that list
held in one file and checked both ways; runnable tests for the names that can be run.

**Non-Goals:** coverage of forms below the name (the 3-D form of `_MAPTRIANGLE`, an optional argument left out;
the call-site check holds each form's call); coverage of the compiler's Rust code; the OpenGL routines.

## Decisions

### D1. The names of a statement come from its tokens, given to the innermost counted statement

A statement's built-ins are the identifier and keyword tokens of its source whose upper-case text, suffix
included, is a name of `sema`'s built-in tables (`STATEMENTS`, `supported()`, and the statements with a node of
their own, `LPRINT`, `FIELD`, `LSET`, `RSET`, moved from the test's `OWN_NODE` to `sema::builtins::OWN_NODE` so
both read one list). Each token goes to the **innermost** counted statement whose span holds it, so the body of
an `IF` block or a `FOR` loop is its own statements', and the condition of an `IF` is the `IF`'s. Names of one
entry (statements at one position) are joined.

- The driver finds the names (`driver::coverage_builtins`, beside the `position` callback): it has the files'
  tokens and `sema`'s tables. `Coverage` gains `builtins: Vec<(Span, &'static str)>`, each name token with the
  built-in it stands for; `Counters::count` keeps each counted statement's span, and when the table is written
  (`Counters::names`) each token goes to the smallest counted span that holds it (`innermost`), its names joined
  per entry.
- *Over the IR:* rejected; the own-node statements and the functions folded at compile time (`LEN` of a fixed
  string, a `CONST` of `CHR$`) would need a name carried through every node.
- *Over text in Python* (as tier 1's `uses_word`): rejected; it cannot tell which statement of a line holds a
  name, nor a name inside a string, without a second lexer.
- A name evaluated only on one side of `_IIF`, or after a short-circuit, counts as run when its statement ran.
  This is a name-level measure; the risk is below.

### D2. The record's version 2

`coverage 2` adds a fifth tab field to each entry: the names, space-separated, empty for none
(`Counters::table`, a `qbr_cov_names[]` array of C strings). `coverage.py` reads 1 and 2 (it ignores the field);
the Civil War Strategy check keeps working unchanged.

### D3. The runner and `builtin_runs.py`

- `--coverage` is taken by `--suite verification` and `--suite compile` (upstream) as by `corpus`: the flag goes
  to the build, `QB64RUST_COVERAGE` to each run, one locked append per run (the code is already shared,
  `run_legacy_tests.py`, the build and run of a program). Still not with `--record` or `--pass-cache`.
- `tools\coverage\builtin_runs.py <counts>...` merges any number of records of any programs: per name, the summed
  count and the programs whose entries name it. No source is read, so no hash is checked (unlike `coverage.py`,
  whose report needs the text). The OpenGL routines (`_glBegin` …, `sema\src\gl_table.rs`) are no rows of the
  built-in tables, so no record names them (found in task 2.3; the call-site check and "Coverage of the OpenGL
  table" hold them); `_GLRENDER` is a row and is reported as any other.
- `--check [--list tests\never_run.list] <counts>...` as the spec says. A name of the list that no record names
  fails too: either no slice program holds it any more (tier 1 also fails) or the counts are of a partial run, and
  `--check` must not be passed a partial set without saying so (it prints the programs the records cover).
  So does a built-in of `sema`'s tables that no record names, listed or not: one held only where no statement
  is counted (a `CONST`, a declaration) would otherwise be neither run nor not run (found at the review). The
  tables are read from the text of `crates\sema\src\builtins.rs`, one row per line; tier 1 holds the file to
  that form (`inputs.rs`, `builtin_tables_are_one_row_per_line`), so no list of names is generated or kept.
- Its tests, `tools\coverage\test_builtin_runs.py`, on small records written by the test.

### D4. `tests\never_run.list` replaces `NEVER_RUN`

`<name>\t<reason>[ <note>]`, reasons `desktop` (the desktop, a window not the program's, the clipboard, the
console the program shares), `printer`, `keyboard` (the machine's keyboard state or its bell), `person` (a dialog
waits for an answer), `compiler` (`CHAIN` starts `qb64pe.exe`). The reasons of today's comments in `NEVER_RUN`
are carried over as notes. Tier 1 reads the file at test time (`inputs.rs`, from `repo()`), so `sema` keeps no list it does not use itself; `NEVER_RUN` and its comment go from
`crates\sema\src\builtins.rs`, and the doc comments that point to it (`[`NEVER_RUN`]`) point to the file.

### D5. Which of the 40 run (measured by the first report, task 3.1)

A built-in that acts only on the program's own window, or reads a value of the machine, **runs** in a screen
program: its effect is compared where the state shows it, else a property is printed (`_DESKTOPWIDTH > 0`), else
nothing is printed and the program goes on (a run whose effect cannot be read still proves the call is made and
returns).

The first report (the three full suites with `--coverage`, 2,392 records of 2,358 programs) found 390 built-ins
named, 361 run, 29 not run, every one of them among the 40; and **11 of the 40 already run**, in programs added
after `NEVER_RUN` was written. One more, `BEEP` in `slice\s24_proc_names`, was the program's own `FUNCTION beep`;
the driver now leaves out a name of the program's own procedures (`driver::coverage_builtins`), and `BEEP` runs
all the same in `verification\v29_a_sound_queue`.

| Already ran (left the list) | Run by the new screen programs (left the list) | Stay (reason) |
|---|---|---|
| `BEEP` (`v29_a_sound_queue`), `CHAIN` (`238_chain`, to a missing program: no compiler starts), `_ICON` and `_TITLE` (`g12_game`), `_SCREENEXISTS` (`v56_c_radix_width`), `_SCREENHIDE` the function and `_SCREENSHOW` (`v57_h*`), `_SCREENIMAGE` (upstream `screenimage/test`, `console_only/screenimage`), the functions `_CAPSLOCK`, `_NUMLOCK`, `_SCROLLLOCK` (`d19_blink_display`, `s159`, `v58_f12`, `v58_f14`) | `w01_window_builtins`: `_TITLE$`, `_MOUSEHIDE`, `_MOUSESHOW`, `_MOUSECURSOR`, `_WINDOWSIZELIMIT`, `_DESKTOPWIDTH`, `_DESKTOPHEIGHT`, `_SCREENX`, `_SCREENY`, `_WINDOWHANDLE`, `_WINDOWHASFOCUS`, the functions `_SCREENICON` and `_FULLSCREEN`; `w02_depthbuffer`: `_DEPTHBUFFER` on a hardware image | `_CLIPBOARDIMAGE`, `_CONSOLEFONT`, `_MOUSEMOVE`, `_NOTIFYPOPUP`, `_SCREENCLICK`, `_SCREENPRINT`, `_SCREENMOVE` (`desktop`); `LPRINT`, `_PRINTIMAGE` (`printer`); `_COLORCHOOSERDIALOG`, `_INPUTBOX$`, `_MESSAGEBOX`, `_OPENFILEDIALOG$`, `_SAVEFILEDIALOG$`, `_SELECTFOLDERDIALOG$` (`person`) |

So `tests\never_run.list` holds 15 names, none with the reasons `keyboard` or `compiler` (kept in the set for a
later built-in). A name that is both a statement and a function is one row, by name: where one use runs, the
name is not listed (the statements `_SCREENICON`, `_FULLSCREEN`, `_CAPSLOCK` … stay held by their call-site
programs only). `_TITLE$` read back after `_TITLE` gave five bytes in the old compiler's build that differ from
run to run (the runner's hidden desktop), so `w01` runs it and prints nothing of it. Both programs were recorded
with `qb64pe.exe` (two equal runs) and are on `slice.list`.

### D6. The names the first report finds beyond the 40

**None** (task 3.1): every built-in a run of the suites names and does not run is one of the 40. The rule stays
for a later report. Each is put in one home, in this order: a `verification` family `v61` for a value a console program can print
(recorded with `qb64pe.exe` before anything else uses it, batched in one recording); a screen program for a
window's state; the list, with a reason of D4, when neither can be had. A slice program that holds a name only
after `SYSTEM` keeps it there (tier 1's rule is unchanged).

### D7. The cadence (user, 2026-10-24, at this proposal)

At a merge into `main` the three full suites (corpus, upstream, verification) run with `--coverage`, each into its
own file under `target\coverage\merge\`, and `builtin_runs.py --check` on the three follows. Not in
a change's closing set. Working rule 9's merge set and `tools\legacy_tests\README.md` ("The runs at a merge into
`main`") say so; `DECISIONS.md` gets a row. The pass lists of the merge are judged from those runs, so
`--check-list` takes a run with `--coverage` (it refused one since `cws-whole-game`; found in task 5.1,
`run_legacy_tests.py`, `check_list`). The build cache keys a coverage build apart from a plain one (its C++
differs), so the merge's run without folding, which is plain, does not share their builds.

## Risks / Trade-offs

- [A coverage build differs from a plain one, so a merge's suites no longer run the build a user gets] → the
  counters add statements only, no change of a value; the run without folding stays plain, and the closing set of
  every change runs plain builds. A failure seen only under `--coverage` is a bug of the coverage build.
- [Name-level: a name in a statement that ran but whose call was skipped (`_IIF`'s other side, a `CASE` test after
  the matched one) counts as run] → said in the spec of the report; the call-site check and the program's
  compared output hold the call itself.
- [A user name equal to a built-in's text (a SUB `Title` where `_TITLE` needs no underscore under `$NOPREFIX`)]
  → `sema` keeps the built-in only on a plain call (`ExprKind::Call`, `StmtKind::Builtin`), not on the nodes of
  their own (`PRINT USING`, `_IIF`, `_MEM`) or a folded constant (task 1.1), so the table is used
  (`sema::builtins::coverage_name`): a name written without its underscore under `$NOPREFIX` is not counted, which
  can only leave a name unnamed, never name a user's symbol (the old compiler reserves the table's names); a name
  after a member's dot is the member's (`driver::coverage_builtins`).
- [The verification suite with `--coverage` is about 3,800 builds whose C++ is not in the build cache] → only at
  a merge, once; the first run fills the cache for the next merge where nothing changed.
- [A screen program that reads `_SCREENX` may be disturbed by the desktop, as `c15_resize` was] → such values are
  never printed (D5).

## Migration Plan

`coverage 1` records stay readable. `NEVER_RUN` is removed in the same task that adds the file, with tier 1's
check changed to it; nothing outside `inputs.rs` reads the constant.
