# The values a program's variables took

`value_trace.py` reports which values each variable of a program took over its runs. It reads the records that a
value-trace build of the new compiler writes (OpenSpec change `value-trace`; spec `compiler/value-trace`).

```
python tools/trace/value_trace.py <records>...                  # merge the records and report
python tools/trace/value_trace.py --json <records>...           # the same report as JSON
python tools/trace/value_trace.py --var NAME <records>...       # only the entries of one name
python tools/trace/value_trace.py --program <main.bas> <records>...   # when the main file is not in this folder
python -m unittest discover -s tools/trace -p "test_*.py"       # its own tests
```

## An example run

From the repository's folder, in `cmd.exe`:

```
target\release\qb64rust.exe -x --trace-values tests\corpus\slice\s08_byref.bas -o %TEMP%\s08_byref.exe
set QB64RUST_TRACE=%TEMP%\s08_byref.values.txt
%TEMP%\s08_byref.exe
python tools\trace\value_trace.py --program tests\corpus\slice\s08_byref.bas %TEMP%\s08_byref.values.txt
```

The report starts like this:

```
1 record(s) of s08_byref.bas
A variable's starting value (0 or an empty string) is not a store and is not listed.

D  DOUBLE
  1 store(s), 0 after-call record(s), range 2.5 to 2.5, 1 distinct value(s)
         1 x  2.5    first at s08_byref.bas:25

I  INTEGER
  1 store(s), 0 after-call record(s), range 5 to 5, 1 distinct value(s)
         1 x  5    first at s08_byref.bas:22

N  LONG
  2 store(s), 3 after-call record(s), range 1 to 3, 3 distinct value(s)
         2 x  1    first at s08_byref.bas:8
         2 x  3    first at s08_byref.bas:11
         1 x  2    first at s08_byref.bas:9
```

`N` was stored into twice, by `n = 1` on lines 8 and 32. The other three records are after-call records: `CALL
bump(n)`, `bump n` and `twice n` were given `n` by reference and changed it.

## What a value-trace build records

`qb64rust --trace-values` builds a program with a record call at each store into a variable
(`crates\codegen-cpp\src\trace.rs`; the run-time part is `trace_rt.h` beside it). When the program exits with
`QB64RUST_TRACE` naming a file, it appends one record to that file. The exit can be `SYSTEM`, `END`, the end of
the program, a runtime error or the window being closed. Without the variable the program writes nothing.

An entry is one of:

- a scalar variable (`A`);
- an array, all its elements together (`X()`);
- a member path of a user type (`P.POS.X`, `SHIPS().HP`, `T.V()`).

Names are in upper case and have no type suffix. The entry's type tells `A%` from `A$`. A variable of the same
name in another procedure is another entry, and so is each parameter. A `_BIT` variable is traced as any integer.

**A store** is recorded with the value the place holds after it. These statements store by name: assignment,
`FOR` and `NEXT`, `INPUT`, `LINE INPUT`, `READ`, `SWAP`, `MID$` and `ASC` as statements, `LSET`, `RSET`, `GET #`
into a variable, `_MEMGET` into a variable, and a copy of a whole user type (each number and string in it).

**A variable's starting value is not a store.** A variable that is never stored into has no entry, and the 0 or
empty string a variable starts with is not among its values.

**A store that a pending error skips is not recorded.** `a(9) = 5` with `DIM a(3)` stores nothing. A statement
that an error handler runs again with `RESUME` stores again, and is recorded again.

**A variable passed by reference** is read again after the statement that passed it. The value is recorded when
it differs from the entry's last recorded value. Such a record is an after-call record, not a store. The stores
the procedure makes are recorded under its parameter's entry. For a local variable of a recursive procedure, the
last recorded value can be another call's, so the count of after-call records can be one too high. The set of
values is still right.

**An array passed by reference**, or an element of one, is not read after the call. The record gets an `alias`
line instead: it links the caller's array to the parameter whose entry holds the values stored through it.

**The limit.** An entry keeps at most 256 distinct values. `QB64RUST_TRACE_LIMIT=<n>` sets another limit. At the
limit, the entry keeps counting its stores and widening its range, and is marked as cut.

**The range** of a number is the smallest and largest value recorded, over every store. A NaN leaves it as it
is. The range of a string is its shortest and longest length.

## Stores that write many elements at once

A statement that fills an array with new values is recorded element by element. Each element counts as one
store, so `b() = a()` with 100 elements adds 100 stores to `B()`.

| Statement | What is recorded |
|---|---|
| `b() = a()` | every element of `b()` |
| `_ARRAYCOPY` of one dimension | the elements it wrote, not the rest of the target |
| `GET #1, , a()` and `_MEMGET m, o, a()` | every element of `a()` |
| `_UCHARPOS(text$, a())` | as many elements as it returns |
| `q = p`, `SWAP p, q` and `GET #1, , p` of a user type | each number and string in it, and every element of a member array in it |

**The strings of a `FIELD`.** `FIELD #1, 4 AS a$` makes `a$` a part of the file's record buffer, and `GET #1`
into that buffer changes it. After each `FIELD` and each such `GET #`, every string the program names in a
`FIELD` is read again. Its value is recorded when it changed. Such a record is counted with the after-call
records. The strings of every `FIELD` are read, since the compiler cannot know which file a `GET #` reads.

## What is not traced by value

These stores are counted by statement and listed as `untraced` lines. The report prints a bulk store under its
array and lists the stores at an address at its end.

| Kind | Statements | The line names |
|---|---|---|
| Bulk store | `ERASE`, `DIM` and `REDIM` of a dynamic array (they write zeros and empty strings, a variable's starting value); a store of a whole `_MEM` value; a `_DYNAMIC` member array inside a copy of a whole user type; and the bulk stores of the next list | the array or variable |
| Store at an address | `POKE`, `_MEMPUT`, `_MEMCOPY`, `_MEMFILL`, `BLOAD`, `CALL ABSOLUTE`, `CLEAR`, `RUN`, a graphics `GET` into an array, a call of a `DECLARE LIBRARY` procedure | nothing |

Three bulk stores are counted because their target cannot be read without evaluating an index a further time:

- `_ARRAYCOPY` of arrays of several dimensions;
- `GET #` into an array from an element on (`GET #1, , a(5)`);
- `_MEMGET` into a member array reached through an index (`_MEMGET m, o, ships(i).v()`).

A `FIELD` string reached through an index (`FIELD #1, 4 AS f$(i)`), and a local `FIELD` string of another
procedure, cannot be read after a `GET #`. A program that has one gets its `FIELD` and `GET #` statements
listed as stores at an address as well.

Also not traced:

- which element of an array held a value, and the order of the values over time;
- reads of a variable;
- the members of a `_MEM` value;
- `_MEMGET` into a variable-length string reached through an index.

A value-trace build behaves as the normal build: the same output, files, errors and exit code. It evaluates
nothing twice. It runs slower and uses more memory (below, "Cost").

## The record format, version 1

Text, one line per item, the fields separated by tabs, the lines ended by LF:

```
trace 1
file     <index> <hash> <path>
entry    <n> <name> <type> <procedure or -> <stores> <after-calls> <distinct> <cut 0|1> <min> <max>
value    <n> <count> <file> <line> <text> [<length> <hash>]
alias    <file> <line> <count> <name> <procedure or -> <callee> <parameter>
untraced <file> <line> <count> <what> <name or -> <procedure or ->
end
```

- **`file`** lines are those of a coverage record (`tools\coverage\README.md`): the file's index, the FNV-1a 64
  hash of its bytes as 16 hex digits, and its path from the main file's folder.
- **`entry`**: `<n>` is the entry's number in this record. Two records of one program may number an entry
  differently. Only entries with a store or an after-call record are written. `<min>` and `<max>` are the range;
  for a string they are lengths; `-` when every value was a NaN.
- **`value`** lines follow their entry, in the order the values were first recorded. `<count>` is the sum of the
  value's stores and after-call records. `<file>` and `<line>` are where it was first recorded.
- **`<text>` of a number** reads back to the same value: an integer in decimal, a `SINGLE` with 9 significant
  digits, a `DOUBLE` with 17, a `_FLOAT` with 21. Two values are distinct when their stored bits differ, so `-0`
  and `0` are two values. Every NaN is written `nan` or `-nan`.
- **`<text>` of a string** is a JSON string in ASCII. A byte outside printable ASCII is written `\u00XX`, so a tab
  or a line end in the string cannot break the line. A string longer than 200 bytes is cut to its first 200.
  `<length>` gives its full length, and `<hash>` the FNV-1a 64 hash of all its bytes as 16 hex digits. The hash
  tells apart two strings that share their first 200 bytes and their length.
- **`alias`**: the statement at this file and line passed the array `<name>` of `<procedure>`, or an element of
  it, to `<parameter>` of `<callee>`, `<count>` times. When the call is a `FUNCTION` inside a value, the count is
  raised after the operation that holds the call. If an error earlier in the statement skipped the call, the count is still
  raised, so it can be too high. The same holds for the `untraced` line of a `DECLARE LIBRARY` function inside a
  value. The values recorded are not affected.
- **`untraced`**: the statement at this file and line stored without a record of the values, `<count>` times.
  `<what>` says what it was (`REDIM`, `ERASE`, `POKE`, `LIBRARY <name>`). A bulk store names its array or
  variable and that one's procedure.

**Strings above 4096 bytes** are told apart by their length and the FNV-1a 64 hash of their bytes, not by their
full text. Two such strings that share both are taken as one value. This bounds the memory of an entry.

In the report, two long strings of one entry that share their first 200 bytes and their length are two values,
told apart by their hash. Only two strings that also share their hash are shown as one value.

## The report

The records are merged (any number of files, any number of records in each):

- stores, after-call records and counts add up;
- the distinct values are the union, and a value's first line is the one of the earliest record;
- an entry is cut when any of its records is;
- the range is the widest of the records;
- `alias` and `untraced` lines of the same statement add their counts.

Each file a record names is read from the main file's folder and its hash compared. A record of another text
stops the tool with a message naming the file, and nothing is reported. The main file is `--program`, or the
first file the first record names, looked for in the working folder.

The report prints one block per entry, sorted by procedure (the main module first) and name, with the values by
count. A `SINGLE` or `DOUBLE` is printed in the shortest text that reads back to the same value. `--json` gives
the same data, each value also as recorded. `--var NAME` keeps the entries of that name in any letter case: `ships`
keeps `SHIPS().HP`, and so do `ships.hp` and `ships().hp`.

## Cost

Only the traced build pays. Measured on 2026-11-05 on the development machine, with the default build (no
`-f:OptimizeCppProgram=true`). The game is Civil War Strategy's copy `tests\corpus\screen\g12_game.bas`, run by
the test runner one script at a time, twice each way.

| Measure | Without the option | With `--trace-values` |
|---|---|---|
| The nine scripts of `g12_game` that do not carry the build, together | 13.2 s and 13.2 s | 13.2 s and 13.3 s |
| One script | 1.0 to 2.4 s | the same, within 0.1 s |
| `g12_game.exe` | 5,217,280 bytes | 5,408,768 bytes |
| The record of one script's run | none | about 150,000 bytes |
| A loop of 10 million iterations with three stores each | 0.13 s | 2.37 s |

The game's scripts spend their time on the screen and the keys, so the record calls do not show in their run
time. A loop that does nothing but store shows them: about 75 ns for each store, which made that loop 18 times
slower.

## In the test runner

`tools\legacy_tests\run_legacy_tests.py --trace-values <file>` builds each program of the corpus, verification or
compile suite with the option and appends each run's record to `<file>`. Each program must still give its
recorded output. The whole corpus and the verification families `v18` and `v24` were run this way once for the
OpenSpec change `value-trace` (its `tasks.md`, tasks 2.6 and 3.2). The run is not part of `just full`: the CLI
tests named `trace_…` in `crates\driver\tests\it\cli.rs` are the permanent check.
