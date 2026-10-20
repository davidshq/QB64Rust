# Coverage of a program's runs

`coverage.py` reports which lines of a program its runs executed, from the counts a coverage build of the new
compiler writes, and checks Civil War Strategy's against the list of the lines its scripts do not reach (OpenSpec
change `cws-whole-game`, design D1 and D2; spec `testing/coverage`).

```
python tools/coverage/coverage.py --check                  # run every script of g12_game with coverage, then check
python tools/coverage/coverage.py --check <counts>...      # check counts made earlier
python tools/coverage/coverage.py <counts>...              # merge and report, no check
python -m unittest discover -s tools/coverage -p "test_*.py"   # its own tests
```

## The counts

`qb64rust --coverage` builds a program with a counter before each source statement (`crates\codegen-cpp\src\
coverage.rs`, `crates\README.md`). When the program exits with `QB64RUST_COVERAGE` naming a file, it appends one
record:

```
coverage 1
file	0	<FNV-1a 64 of the file's bytes, 16 hex digits>	g12_game.bas
<file>	<line>	<column>	<count>        one line per statement, unrun ones included
end
```

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
