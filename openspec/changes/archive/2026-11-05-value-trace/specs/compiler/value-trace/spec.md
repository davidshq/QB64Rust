## Purpose

A value-trace build records the values each BASIC variable of a program takes while it runs. A tool merges the
records of several runs and reports, for each variable, its distinct values, their counts and its range.

## ADDED Requirements

### Requirement: The traced variables
A value-trace build SHALL keep one entry per scalar variable, one per array (all its elements together) and one per
member path of a user type (`p.pos.x`, `ships().hp`). Variables of the same name in different procedures, and a
procedure's parameters, SHALL have entries of their own.

#### Scenario: Two procedures, one name
- **WHEN** `SUB a` and `SUB b` each store into a local `i`
- **THEN** the record has two entries named `I`, one for each procedure

#### Scenario: An array
- **WHEN** a program stores 5 into `x(1)` and 7 into `x(2)`
- **THEN** the record has one entry for `X()` with the values 5 and 7

#### Scenario: A member path
- **WHEN** `TYPE pt: x AS INTEGER: y AS INTEGER: END TYPE` and `DIM p AS pt`, and the program stores into `p.x`
- **THEN** the record has an entry for `P.X`, and none for `P.Y` until it is stored into

### Requirement: What an entry says
An entry SHALL give the variable's name in upper case without its type suffix, its type and its procedure (none
for the main module). A `_BIT` variable or array SHALL be traced as any integer. A variable's starting value
(zero or an empty string) is not a store.

#### Scenario: One name, two types
- **WHEN** a program stores into `a%` and into `a$`
- **THEN** the record has two entries named `A`, one of type `INTEGER` and one of type `STRING`

#### Scenario: A _BIT
- **WHEN** `DIM b AS _BIT * 3` and `b = 5` run
- **THEN** the entry for `B` holds the value -3, the value the variable holds after the store

### Requirement: The stores that are traced
A value-trace build SHALL record the value a variable holds after each statement form that stores into it by name:
assignment, `FOR` and `NEXT`, `INPUT`, `LINE INPUT`, `READ`, `SWAP`, `MID$` and `ASC` as statements, `LSET`,
`RSET`, `GET #` into a variable, `_MEMGET` into a variable, and a copy of a whole user type (each scalar member).
A store that a pending error skips SHALL NOT be recorded.

#### Scenario: A FOR loop
- **WHEN** `FOR i = 1 TO 3: NEXT` runs
- **THEN** the entry for `I` holds the values 1, 2, 3 and 4

#### Scenario: SWAP
- **WHEN** `a = 1: b = 2: SWAP a, b` runs
- **THEN** the entry for `A` holds 1 and 2, and so does the entry for `B`

#### Scenario: A store that an error skips
- **WHEN** `DIM a(3)` and `a(9) = 5` run, with an error handler that does `RESUME NEXT`
- **THEN** the entry for `A()` does not hold 5, and its number of stores is not raised

### Requirement: A variable passed by reference
The stores a `SUB` or `FUNCTION` makes into a parameter SHALL be recorded under the parameter's entry. A variable,
or a member with no index on the way, that a statement passes by reference SHALL be read after that statement,
and its value recorded when it differs from the entry's last recorded value. Such a record SHALL be counted as an
after-call record, not as a store.

#### Scenario: A SUB that changes its argument
- **WHEN** `SUB inc (n): n = n + 1: END SUB` is called as `CALL inc(k)` with `k = 4`
- **THEN** the entry for the parameter `N` of `INC` holds 5, and the entry for `K` holds 4 and 5, with one store
  and one after-call record

#### Scenario: A SUB that leaves its argument alone
- **WHEN** `SUB show (n): PRINT n: END SUB` is called 1000 times as `CALL show(k)` with `k = 4`
- **THEN** the entry for `K` has one store, no after-call record, and the value 4 with a count of 1

### Requirement: A variable of a user type passed by reference
For a variable of a user type passed by reference, each number and string in it SHALL be read after the statement
as a scalar variable is. Each member array in it SHALL get an alias to the parameter's member array.

#### Scenario: A SUB that changes a member
- **WHEN** `SUB move (q AS pt): q.x = 8: END SUB` is called as `move p`, where `pt` has a member `x` and a member
  array `v()`
- **THEN** the entry for `P.X` holds 8 as an after-call record, and the record has an alias from `P.V()` to `Q.V()`
  of `MOVE`

### Requirement: An array passed by reference
For an element or a whole array passed by reference, the record SHALL hold an alias: the statement's file and
line, the array, the procedure and parameter it was passed to, and the number of times the call ran. Nothing is
read after the call.

#### Scenario: An array passed to a SUB
- **WHEN** `SUB fill (a()): a(1) = 7: END SUB` is called as `fill grid()` on line 12
- **THEN** the entry for the parameter `A()` of `FILL` holds 7, and the record has an alias from `GRID()` to `A()`
  of `FILL` at line 12 with a count of 1

### Requirement: Stores at an address
A value-trace build SHALL count, per statement, the stores it cannot tie to a variable: `POKE`, `_MEMPUT`,
`_MEMCOPY`, `_MEMFILL`, `BLOAD`, `CALL ABSOLUTE`, `CLEAR`, `RUN`, a graphics `GET` into an array, and calls of
`DECLARE LIBRARY` procedures. The record SHALL list each such statement with its file, line, what it was and the
number of times it ran.

#### Scenario: A POKE
- **WHEN** a program runs `POKE 100, 7` twice on line 12
- **THEN** the record lists line 12 as an untraced store that ran 2 times

### Requirement: Bulk stores that are traced
A store that writes many elements at once and brings new values SHALL be recorded element by element, each
element as a store. They are `b() = a()`, `_ARRAYCOPY` of one dimension, `GET #` and `_MEMGET` into a whole array,
`_UCHARPOS` (the elements it fills), and a member array inside a copy of a whole user type.

#### Scenario: A whole-array copy
- **WHEN** `a()` holds 10, 11, 12 and 13, and a program runs `b() = a()` on line 8
- **THEN** the entry for `B()` has 4 stores and the values 10, 11, 12 and 13, each first recorded on line 8, and
  the record lists no untraced store for line 8

#### Scenario: A file read into a whole array
- **WHEN** a program runs `GET #1, 1, f()` on line 9, and `f()` has 4 elements
- **THEN** the entry for `F()` has 4 stores, with the values the file held

#### Scenario: A range copied
- **WHEN** a program runs `_ARRAYCOPY a(1 TO 2) TO c(4)`, where `c()` has 6 elements
- **THEN** the entry for `C()` has 2 stores, the two values copied, and none for its other elements

### Requirement: Bulk stores that are counted
A value-trace build SHALL count, per statement, the bulk stores it does not read, and list each as a store at an
address is listed, with the name of the array or variable written and its procedure. They are `ERASE`, `DIM` and
`REDIM` of a dynamic array, which write a variable's starting value; a store of a whole `_MEM` value; a `_DYNAMIC`
member array inside a copy of a whole user type; and a bulk store whose target cannot be read without evaluating
an index again.

#### Scenario: A REDIM
- **WHEN** a program runs `REDIM b(5)` once on line 8
- **THEN** the record lists line 8 as an untraced store into `B()` that ran 1 time, and the entry for `B()` gets no
  value from it

### Requirement: The strings of a FIELD
After `FIELD`, and after `GET #` into a file's record buffer, each string that a `FIELD` of the program names
SHALL be read again and recorded when its value changed, as a variable is after a call. A string reached through
an index, or one of another procedure, is not read; the statement SHALL then also be listed as a store at an
address.

#### Scenario: A record read into the buffer
- **WHEN** `FIELD #2, 4 AS fa$` runs, and later `GET #2, 1` reads a record that holds `abcd`
- **THEN** the entry for `FA` holds `abcd`, and the record lists no untraced store for the `GET`

### Requirement: The values kept
An entry SHALL keep its number of stores, its number of after-call records and its distinct values, each with how
often it was recorded and the file and line that first recorded it. A number's value SHALL be written as text that
reads back to the same value; two values are distinct when their stored bits differ. When an entry reaches the
limit of distinct values (256, or the number `QB64RUST_TRACE_LIMIT` gives), it SHALL keep counting stores and mark
itself as cut.

#### Scenario: The limit
- **WHEN** `FOR i = 1 TO 1000: NEXT` runs with `QB64RUST_TRACE_LIMIT=10`
- **THEN** the entry for `I` has 1001 stores, 10 distinct values and the mark that it was cut

#### Scenario: A SINGLE
- **WHEN** `s! = 0.1` is stored
- **THEN** the value's text reads back as the `SINGLE` 0.1, not as the `DOUBLE` 0.1

### Requirement: The text of a string value
A string's value SHALL be written so that no byte of it can break its line. A string longer than 200 bytes SHALL
be written as its first 200 bytes, with its full length and the FNV-1a 64 hash of all its bytes beside them. The
report SHALL tell apart two such strings whose hashes differ.

#### Scenario: A tab and a line end
- **WHEN** `t$ = "a" + CHR$(9) + "b" + CHR$(10)` is stored
- **THEN** the value is one line of the record, and its text reads back as those four bytes

#### Scenario: A long string
- **WHEN** a string of 300 bytes is stored
- **THEN** its value gives the first 200 bytes, the length 300 and the hash of the 300 bytes

#### Scenario: Two long strings that share their first 200 bytes
- **WHEN** two strings of 300 bytes that differ only in their last byte are stored into one variable
- **THEN** the record and the report give two values

### Requirement: The range of an entry
An entry of a number SHALL give the smallest and largest value recorded, counted over every store, including the
stores after the entry was cut. An entry of a string SHALL give the shortest and longest length recorded.

#### Scenario: A range past the limit
- **WHEN** `FOR i = 1 TO 1000: NEXT` runs with `QB64RUST_TRACE_LIMIT=10`
- **THEN** the entry for `I` gives the range 1 to 1001

### Requirement: The same program
A value-trace build SHALL behave as the build without the option: the same output, input, files and exit code,
and the same errors at the same lines. It SHALL evaluate each expression of a statement as often as the build
without the option does. Only its speed and memory use may differ.

#### Scenario: A runtime error
- **WHEN** a value-trace build stops at a runtime error on line 20
- **THEN** it reports the same error at line 20, and its record is still written

#### Scenario: A FUNCTION in an index
- **WHEN** `a(f(1)) = 5` runs, where the `FUNCTION f` prints a line each time it is called
- **THEN** the line is printed once, as in the build without the option

#### Scenario: The window closed during a loop
- **WHEN** the window of a value-trace build is closed while the program stores in a loop
- **THEN** the program ends as the build without the option does, and its record is whole, with its `end` line

### Requirement: The record
A record SHALL begin with a line naming its format and version, list the program's files each with the same hash of
its text as a coverage record, then give its entries, their values, the aliases and the untraced stores, and end
with a line of its own. The format SHALL be described in `tools\trace\README.md`.

#### Scenario: An included file
- **WHEN** a program `$INCLUDE`s `inc.bi`, which stores into a variable
- **THEN** the record lists `inc.bi` with its hash, and that value's first store names `inc.bi` and its line

### Requirement: The report
`tools\trace\value_trace.py` SHALL merge any number of records of one program and print, for each entry, its stores, its
after-call records, its range, whether it was cut, and its distinct values by count. Under an array it SHALL print
the array's aliases and bulk stores. It SHALL also print the stores at an address. It SHALL take `--json` for the
same report as JSON and `--var <name>` for the entries of one name.

#### Scenario: Two runs merged
- **WHEN** one run stores 1 into `a` and another stores 2
- **THEN** the merged report gives `A` the values 1 and 2, each with a count of 1

#### Scenario: An alias in the report
- **WHEN** a record has an alias from `GRID()` to `A()` of `FILL`
- **THEN** the report's block for `GRID()` names `A()` of `FILL` as the entry that holds the values stored through it

#### Scenario: Records of another program
- **WHEN** a record names a source whose text differs from the one the tool reads
- **THEN** the tool stops with a message naming the file, and reports nothing
