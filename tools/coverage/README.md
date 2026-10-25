# Coverage of a program's runs

`coverage.py` reports which lines of a program its runs executed, from the counts a coverage build of the new
compiler writes, and checks Civil War Strategy's against the list of the lines its scripts do not reach (OpenSpec
change `cws-whole-game`, design D1 and D2; spec `testing/coverage`).

`builtin_runs.py` reports which built-ins the test runs reach, from the names in the same counts, and holds them
to `tests\never_run.list` (OpenSpec change `m4-coverage-review`; below, "The built-ins reached").

```
python tools/coverage/coverage.py --check                  # run every script of g12_game with coverage, then check
python tools/coverage/coverage.py --check <counts>...      # check counts made earlier
python tools/coverage/coverage.py <counts>...              # merge and report, no check
python tools/coverage/builtin_runs.py [--check] <counts>... # the built-ins run and not run; with the list's check
python -m unittest discover -s tools/coverage -p "test_*.py"   # their own tests
```

## The counts

`qb64rust --coverage` builds a program with a counter before each source statement (`crates\codegen-cpp\src\
coverage.rs`, `crates\README.md`). When the program exits with `QB64RUST_COVERAGE` naming a file, it appends one
record:

```
coverage 2
file	0	<FNV-1a 64 of the file's bytes, 16 hex digits>	g12_game.bas
<file>	<line>	<column>	<count>	<names>        one line per statement, unrun ones included
end
```

`<names>` are the built-ins the statement calls, space-separated, empty for none (`builtin_runs.py`, below;
design D1 of `m4-coverage-review`). A name is the statement's that holds it innermost, so a name in an `IF`
block's body is its body statement's, not the `IF`'s. A record of version 1 (`coverage 1`, before that change)
has no fifth field; both are read.

A statement's column is where it starts; 0 for a statement the lowering made on another line than its span (a
`CASE`'s test). A loop's bottom (a `FOR`'s step, the test or jump of `LOOP` or `WEND`) starts at its `NEXT`, `LOOP`
or `WEND`, so a one-line loop counts its top and bottom apart. Statements at one position share a line. The jumps the lowering adds for an `IF` or a `CASE` have
none, so a line counts as run only when its own statements ran.

## The report

The records are merged (any number of files, any number of records in each). Each file a record names is read
from the main file's folder (`--program`, default `tests\corpus\screen\g12_game.bas`) and its hash compared: a
record of another text stops the tool with a message naming the file, and nothing is reported. Each line with a
statement is then **run** (every statement that starts on it ran), **partly run** (the columns that did not run
are named), or **not run**. The report gives the totals per file, the lines not run as ranges, and the partly run
lines.

## The check

`--check` holds the main file to `tests\programs\civil-war-strategy.unreached` (`--unreached`), beside the game's
folder, which is a byte-for-byte copy and takes no file of ours. The copy `g12_game.bas` keeps the game's line
numbers, so its lines are `CWSTRAT.BAS`'s. One entry per line:

```
<line>[:<column>,<column>...]<TAB><reason>[ <more>]
```

with the columns of the statements that do not run for a partly run line. `#` starts a comment line. The reasons:

| Reason | Meaning |
|---|---|
| `edit` | removed by an edit of the copy (`tools\screen\cut_game.py`); may hold no statement in the copy |
| `file` | a file the game does not ship |
| `dead` | no path reaches it; say why after the reason |
| `input` | input the screen-state check cannot give |
| `bug` | a bug of the game that ends or hangs it |
| `rare` | reachable, but only in a state of the game no script creates; name the state after the reason (`DECISIONS.md` 2026-10-19). Write a script first where a branch is large |

It fails on a line with a statement that is not run in full and not listed; on a listed line that ran in full,
or whose listed columns are not the ones that did not run (so the list only shrinks); on a line of another reason
than `edit` that holds no statement; and on a reason outside the table.

Without counts files, `--check` makes them first: it runs the corpus runner on every test of the program
(`--category screen --glob "g12_game.*" --coverage <file>`, `tools\legacy_tests\README.md`): its own keys file
and each script `g12_game.<script>.keys`, built once with `--coverage` and the screen-state file, with the build
cache, into `target\coverage\`. The counts go to `target\coverage\g12_game.txt`. A script that fails fails the
check too: its states differ from the old compiler's, and its counts may stop short. Nothing of the old compiler is
needed: which lines run is the same for both builds while their states agree, and the scripts compare the states.

## The built-ins reached

`builtin_runs.py` merges the records of any number of runs of any programs, of the suites' runs with
`--coverage` (`tools\legacy_tests\README.md`; corpus, verification and compile). No source is read, so no hash is
checked. Each built-in a record names is **run** when a statement that names it ran at least once, else **not
run**; the report gives the totals and each built-in not run with the programs that hold it. It is a measure by
name: a name in a statement that ran counts as run though its own call was skipped (the other side of `_IIF`).
The OpenGL routines are no rows of the built-in tables, and no record names them.

`--check` holds the counts to `tests\never_run.list` (`--list`), one built-in per line:

```
<name><TAB><reason>[ <note>]
```

`#` starts a comment line. The reasons:

| Reason | Meaning |
|---|---|
| `desktop` | the desktop, a window not the program's, the clipboard, or the console the program shares |
| `printer` | a page printed on the default printer |
| `keyboard` | the machine's keyboard state, or its bell |
| `person` | a dialog waits for a person's answer |
| `compiler` | another compiler is started (`CHAIN`) |

It fails on a built-in not run and not listed; on a listed one that ran (so the list only shrinks); on a listed one
that no record names (the counts are of a part of the suites, or no program holds it any more); on a line of
the list that is not of that form (only an empty line is skipped); and on a built-in of `sema`'s tables that no
record names, listed or not, as one held only where no statement is counted (a `CONST`). The tables are read from
the text of `crates\sema\src\builtins.rs` (`--table`), one row per line, a form tier 1 holds. Tier 1 holds the same list to the built-ins `sema` compiles and wants a
call-site program for each (`crates\driver\tests\inputs.rs`). It runs at a merge into `main`, on the counts of the
three full suites (`CLAUDE.md` working rule 9).
