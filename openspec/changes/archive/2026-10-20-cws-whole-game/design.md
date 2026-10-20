# Design

## Context

What there is (see proposal.md for why):

- **The two-turn script.** `tests\corpus\screen\g12_game.bas` is `CWSTRAT.BAS` with 17 edits, written by
  `tools\screen\cut_game.py` (`COPIES`), with line n of the copy being line n of the game. Its edits:
  - `RANDOMIZE 1861`, no `_FULLSCREEN`, and `TICK`'s `TIMER` loop commented out;
  - an announcement (`DEF SEG = 0: POKE 1264, …`) before each of the 14 places the game waits for a key: 12
    `DO WHILE INKEY$ = "": LOOP`, the `INPUT a$` of the high-score name, and the menu's `crsr:` loop.

  Its configuration sets `noise` 0 and `history` 0. Its keys file plays two turns with a `prompt` before every
  key. It writes 23 full states, 2.5 MB.
- **`screen_state.cpp`.** It reads the keys file into steps. A key thread gives each key when `INKEY$`'s buffer is
  empty, and waits for the k-th announcement (byte `0x4F0`) at the k-th `prompt`. It writes states on `state` and
  `atexit`.
- **The runner.** `run_legacy_tests.py` treats a corpus program with `<name>.screen` as a screen program
  (`<name>.keys` given as `QB64_SCREEN_KEYS`). It runs it on a hidden desktop and compares the state file as text.
- **The new compiler's statements.** Each source statement is one IR `Stmt` with a span and a line, the parts of a
  single-line `IF … THEN a: b ELSE c` included (measured with `--dump ir`, 2026-10-19). The lowering adds `Stmt`s
  of its own (the `Jump` at the end of a `THEN` part or a `CASE`) on the line of the block. `codegen-cpp`'s
  `Emitter::body` (`crates\codegen-cpp\src\procs.rs`) writes each `Stmt` in turn and knows its file and line
  (`stmt_at`).
- **The game's menus.** Every menu goes through `SUB menu`, which reads keys at `crsr:` and takes the first letter
  of an entry as a hotkey: `E` is END TURN on the main menu, `Y` is Yes. A month of computer turns brings a number
  of "hit any key to continue" waits (line 1091 and others) that depends on its battles.

## Goals / Non-Goals

**Goals:**
- A measure of how much of the game the scripts run, from the new compiler, with nothing added to the game.
- Scripts that reach every ending and every menu without counting how many waits a month brings.
- Every script compared with the old compiler's build at many points, without a megabyte per point.

**Non-Goals:**
- Coverage from the old compiler. It would need the game's lines rewritten. Which lines run is the same for both
  builds while their states agree, and the states are compared.
- Coverage as a general report over the corpus or upstream. The option works for any program, but only the game is
  held to a list.
- Sound that is heard, `_FULLSCREEN`, and real `TICK` pauses. These stay with the edits and are listed as removed by
  an edit.
- Running the unedited `CWSTRAT.BAS` from a test. It stays as `tests\programs\` says: built by a test, run by hand.

## Decisions

### D1. Coverage is counted by the new compiler's build, per source statement

`qb64rust --coverage` adds `qbr_cov[k]++;` before statement k's C++ in `Emitter::body`. It also writes a table of
the statements, `(file index, line, column)`. Statements that share a position share one entry, so the lowering's
own `Jump`s fold into the statement they belong to; task 1.1 checks this on the `IF`/`CASE` forms. A function
registered with `atexit` on the first statement appends the record when `QB64RUST_COVERAGE` names a file. That is
the hook `screen_state.cpp` uses (`atexit(write_state)`), and libqb's ends (`SYSTEM`, `END`, the window closed, a
runtime error) all reach it. The record holds:
- a header with each source file's path, relative to the main file's folder, and a hash of its text;
- one line per entry: file index, line, column, count.

Code: the option in `crates\driver\src\main.rs` → the build options; the counter and table in
`crates\codegen-cpp\src\procs.rs` (`body`) and `decl.rs` (`global`, beside `qbr_diverged`). The option is part of
the build cache's key.

*Alternatives.*
- An instrumented copy of the game, a counter written into each line of BASIC. It cannot stand before a `CASE`,
  inside a single-line `IF`, or on a `SUB` line, and it changes what the old compiler is compared on.
- gcov on the C++. Its lines are the C++'s, and mapping them back is the work D1 does directly.
- Counting by line only. `IF x THEN …` lines would count as run when only the test ran, which is most of this
  game's logic.

### D2. Lines are reported from statements, against a list of unreached lines

`tools\coverage\coverage.py` reads the records and checks each file's hash against the file on disk. It then gives
each line a status: run, partly, not run, or none. The partly run lines name the columns of the statements that
did not run.

`--check` fails on a line with a statement that is not run in full and not in the list. It also fails on a listed
line that ran, so the list only shrinks. The list is `tests\programs\civil-war-strategy.unreached`, in the form
`<line>[:<col>,…]\t<reason>`. It sits beside the game's folder, not in it, because that folder is a byte-for-byte
copy. The reasons are:
- `edit`: removed by an edit of the copy;
- `file`: a file the game does not ship (for example the `.wav` files of `sndblst`);
- `dead`: no path reaches it, with the reason in a comment;
- `input`: input the check cannot give;
- `bug`: a bug of the game that ends or hangs it;
- `rare`: reachable, but only in a state of the game no script creates, named in a note (user, 2026-10-19, at 83 %
  of the lines run: `DECISIONS.md`). Used once the scripts cover the large branches.

The run itself is `coverage.py --check`. It:
1. builds `g12_game` once with `qb64rust --coverage` and the screen-state file, with the build cache;
2. runs every script through the runner's own screen-program run, with `QB64RUST_COVERAGE` set: a runner option
   `--coverage <file>` that adds the option and the variable;
3. merges and checks.

It needs no recording, and nothing of the old compiler.

### D3. One copy, several scripts

A screen program may have `<name>.<script>.keys`, `.screen` and `.output`. Each is a test named `<name>.<script>`
that shares the program's build. The runner finds them where it finds `<name>.keys`, `collect_corpus`, and the
results and lists use the test name. `g12_game.keys` stays as the two-turn script under its old name.

*Alternative.* A copy of the game per script, as `g13_…`, `g14_…`. Each copy is 162 kB of `.bas` and 152 kB of
data files, and every edit of `cut_game.py` writes them all again. The runner change is smaller than what it saves,
and it is tested in `test_run_legacy_tests.py`.

### D4. Places, rules and `play until` in the keys file

Each announcement in the copy becomes `DEF SEG = 0: POKE 1265, <place>: POKE 1266, <letter>: POKE 1264, …`.
- The place is the number of the edit: 1 to 14, in the order of the lines of `COPIES`.
- The letter is `ASC(mtx$(0))` at the menu's `crsr:`, the first letter of the menu's title, so `14:M` is the main
  menu. It is 0 elsewhere.

`screen_state.cpp` reads both bytes when the count changes:
- `rule <place>[:<letter>] <steps>`, where a step is `key <name>` or `text <chars>`, sets the answer.
- `play until <place>[:<letter>]` loops: at each new announcement it stops if the place matches, else it gives the
  rule's keys. If the rule has more than one key, the first goes now and the rest at the waits that follow.
- A wait with no rule ends the run with code 125 and a message naming the place and letter.

All of this lives in the key thread, beside `prompt` (`tools\screen\screen_state.cpp`).

A full game to the time limit is then a few lines:

```
rule 7 key space          # hit any key to continue
rule 14:R text a          # a Retreat menu: Attack
…
play until 14:M           # the main menu of the next month
text e                    # END TURN
prompt
text y                    # Yes
```

repeated for 48 months by a `repeat <n>` … `end` block around steps. That is the one construct added for length.
The keys a run gives depend only on the places the game announces, and those depend only on the game's state,
which is the same in both builds while they agree. So `--record`'s two runs check that the rules leave nothing to
timing, as they do for `prompt`.

*Alternative.* A Python driver that plays the game live and writes a plain keys file. It needs a two-way channel
to the program and a second program to maintain, and its output would be a keys file thousands of lines long that
no one reads.

### D5. `digest` instead of most states

`digest` writes `digest <16 hex digits>`, FNV-1a 64 of the bytes `state` would write. It runs in `screen_state.cpp`
and uses the same writer into a buffer. A script takes:
- a `digest` at each month's main menu and at each new kind of screen;
- full `state`s only at the screens a reader needs to see when a digest fails: each ending, and a few per script.
  The rule is at most 6 per script.

When a digest differs, the runner's message names the first line that differs. `state_to_png.py` cannot show a
digest, so rerunning with `QB64_SCREEN_DIGEST_AS_STATE=1`, which makes `digest` write a full state, gives the
pictures. The runner passes the variable with `--full-states` for one test. Budget: the new `.screen` files
together under 15 MB. `g12_game.screen` stays as it is.

### D6. Further edits of the copy

Each is appended to a line that is already there, so line numbers stay:
- line 218: `RANDOMIZE 1861: PLAY "V0"`. Measured first (task 2.1): whether `V0` holds for the game's later `PLAY`
  strings and for `SOUND`, and whether the old build's waits for notes are unchanged at volume 0.
- line 241, where `DATE$` goes into `cws.his`: `"10-19-2026"` in its place. The recap screen shows it.
- the place and letter bytes of D4 in the 14 announcements.

`SHELL "copy cws.his oldhist.ory"` stays. It runs `cmd` in the program's folder, the same in both builds.

### D7. The scripts

They are written in the order coverage asks for them, each recorded with `qb64pe.exe`, compared, and then
measured. A first set, by what each reaches:

| Script | Covers |
|---|---|
| `union_time` | Union, solo, END TURN each month to the time limit: the computer's moves, battles, retreats, events, `victor`'s time ending, the high-score `INPUT` |
| `rebel_capital` | Rebel, moves and attacks to take Washington: the player's battles, `capture`, the capital ending |
| `two_player` | 2-player game: `blanken`, the "press a key" waits, both sides' menus |
| `options` | every Utility entry and its sub-menus, sound at `V0`, End Cond, Vary Start, Realism, History, Recap, Chk Links |
| `files` | save, load, new game, Resume Saved Game from the title, Quit |
| `commands` | every Commands entry; Ships (both types, attack, sail, raid, invasion); Railroad; Recruit |
| `endings` | the cities, income, force-ratio and destroyed endings, through End Cond settings or the INI's `vicflag`, with "Play More" and "Press Onward" |

The coverage report after each script decides the next. A line no script can reach goes in the list with its
reason.

### D8. Differences found are fixed here

A script that fails with the new compiler is a parity bug of the kind this change exists to find. It is fixed in
this change with a front-end or slice test that shows it alone, as `verification-compare` did. A fix too large for
the change is put to the user before the script is left out.

## Risks / Trade-offs

- **[A whole game runs long]** 48 months with every animation can take minutes, run twice to record. → `TICK` is
  already empty; `turbo!` is set to Fast by the scripts (a Utility entry). Measure the first full game (task 4.1).
  If a script takes over 60 s, split it at a save, and have the next script load that save.
- **[The real cursor or focus]** No mouse is used, and the hidden desktop takes no keys. Nothing new.
- **[A digest hides what differs]** → `--full-states` for that one test, and a few full states per script.
- **[The rule set reacts to a wait it does not know]** → the run stops with the place named, never guesses.
  Writing a script is: add rules until it stops nowhere.
- **[Disk]** → D5's budget; the two-turn script stays the largest recording.
- **[Hashes of the source in records]** A record for another text is refused (D2), so a stale counts file cannot
  report false coverage.

## Open Questions

- Whether the INI's `vicflag` or the End Cond menu is the shorter way to each ending. Found while writing
  `endings`; either way it is a script.

## As built

- **D1, the lowering's own jumps (task 1.3).** They do not share a position with their block's statement: the jump at
  the end of a `THEN` part and the end of a `CASE` carry the block's span on the `END IF` or `END SELECT` line. So
  they are marked instead (`Stmt::own` in `crates\ir`, set by `emit_own` in `lower.rs`; also the `EVERYCASE` flag's
  store and skip) and get no entry. Other statements the lowering makes stay counted: a `FOR`'s statements on the
  `NEXT` line, a loop's bottom on the `LOOP` or `WEND` line, a `CASE`'s test. A statement's column is where its span
  starts when that is on its line, else 0 (a `CASE` test carries the `SELECT`'s span); a loop's bottom carries the
  span of its `NEXT`, `LOOP` or `WEND` keyword (sema's `end_at`), so on a one-line loop the step or bottom test is
  an entry apart from the header's (one `NEXT j, i` is one entry for both steps). A `FOR`'s entry test keeps the
  header's span: on a one-line loop it shares the header's entry, and on the `NEXT` line of a block it is column 0
  beside the step's column, so a loop whose body never runs leaves its `NEXT` line partly run. `ON … GOSUB`'s statements
  share the `ON` statement's position and fold into one entry. A plain `SELECT CASE v` makes no statement on its
  line. The writer is registered by a static initialiser in `global.txt`, not on the first statement: nothing of
  `main0.txt` changes.
- **First measure (task 3.3, 2026-10-19).** The two-turn script `g12_game.keys` alone, `coverage.py --check`: of the
  copy's 3761 lines with a statement, 1946 run (52 %), 298 partly run, 1517 not run. The scripts of groups 4 and 5
  start from there.
- **The scripts (tasks 4.1 and 5.1).** `union_time` (the time limit, a passive enemy), `options` (every Utility
  entry; sound on, so a run takes about 26 s), `commands` (each command, report, ships, railroad, an invasion),
  `files`, `two_player`, `rebel` (the Rebels attack Washington and lose: the force-ratio ending with No-Override and
  Press Onward, the Union's victory screen) and `endings` (three games in one run: income, capital, annihilation).
  `rebel_capital` of D7 became `rebel`: a captured capital comes from the computer in `endings`. Every script
  plays in 1.5 to 3 s but `options`, and passes with the new compiler state for state and digest for digest; no
  difference from the old compiler was found. Their `.screen` files take 1.3 MB.
- **Found while writing them.** The game's menus take Up, Down, PgUp and PgDn, not End; a "-" entry is chosen by
  the key `-`; the third Sounds entry starts with a note character, not `&`; Vary Start reads the configuration
  again. `_FILEEXISTS("*.sav")` takes no wildcard, so Load is always "-" and `filer`'s loading never runs.
- **Coverage with the eight tests (2026-10-19):** 3107 run (83 %), 244 partly run, 410 not run.
