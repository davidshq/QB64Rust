# Design

## Context

How other languages and tools record variable values is in `study\42-value-trace-prior-art.md`.

The `--coverage` build is the model (design D1 of `cws-whole-game`, `crates\codegen-cpp\src\coverage.rs`):
- The driver passes a `Coverage` value down to the emitter.
- The emitter's statement loop (`Emitter::body` in `procs.rs`) adds a counter before each source statement.
- A table, and a writer registered with `atexit` and `at_quick_exit`, go into the program's C++.
- At exit, the writer appends one record to the file `QB64RUST_COVERAGE` names.
- `tools\coverage\coverage.py` merges the records and checks each file's hash against the source.

The IR names every store by its `Place` (`Var`, `Element`, `Member`). The operations that store fall into four
groups (D5 gives the full list):
- a store into one place by name: `Op::Assign`, `AssignAll`, `Lset`, `Input`, `Read`, `MemGet` into a place, and
  `Op::Builtin` with a `StmtArg::Place` or `StmtArg::Bytes` argument (`SWAP`, `MID$`, `GET #`, and others);
- a store into every member of a value: `Op::CopyWhole`;
- a call, `Op::Call` and `ExprKind::CallProc`, whose `Arg::Ref` and `Arg::Array` arguments the procedure may
  change;
- a store into many elements at once, or at an address: `Op::CopyArray`, `ArrayCopy`, `Clear`, `Allocate`,
  `MemPut`, `POKE` and others.

`Var` carries the BASIC name, type, storage and shape. The debug symbol file (`debug.rs`) already writes the same
facts for the debugger.

The emitter writes most stores through one function, `Emitter::store` (`crates\codegen-cpp\src\stmt.rs`). Two
facts about it shape this design:
- An element's position is evaluated once, into `tmp_long`, before the store. The C++ of a place such as
  `a(f(i))` holds the call of `f` and libqb's `array_check` (`place.rs`, `flat_index_in`). Writing that C++ a
  second time would call `f` twice and raise error 9 twice.
- A store into an element is skipped while an error is pending: `if (!is_error_pending()) element=v;`.

## Goals / Non-Goals

**Goals:**
- Every store by name is seen once, at the IR's places, not by parsing the C++.
- A traced program evaluates nothing twice. Its output, errors and exit code are those of the normal build.
- A store that did not happen is not counted.
- The build without the option writes the same C++ as today.
- A traced program's memory stays bounded, whatever its run length.

**Non-Goals:**
- Tracing which element of an array was written, or the subscripts used.
- Recording reads, or the order of values over time. The record is a summary, not a log.
- The value a variable starts with (zero or an empty string). It is not a store. The report says so once.
- The members of a `_MEM` value. A store of a whole `_MEM` is listed as a bulk store (D5).
- A trace of `$CHECKING:OFF` statements that differs from that of checked ones. Both are traced alike.

## Decisions

### D1. Record where the store is made, with the address already found

The record call is `qbr_trc_<kind>(<entry>, <value>, <site>)`.
- `<entry>` is the entry's number.
- `<value>` reads the stored place through the address the store itself used.
- `<site>` is the number of the file and line, as the coverage table numbers them.

The call is written inside the store's own C++, not after the operation:
- **A variable** (`Place::Var`) or **a member with no index on the way** (`p.pos.x`): the call follows the store.
  The place's C++ holds no index, so reading it again evaluates nothing.
- **An element** (`a(i)`): the call goes under the store's own guard and reads the element at `tmp_long`:
  `if (!is_error_pending()) { element=v; qbr_trc_…(…, element, …); }`.
- **A place through a member with an index on the way** (`a(i).m`, `t.v(i)`): the call goes inside the block
  `store_through_member` writes, under its `mbr_ok` test, and reads through the address computed there.
- **`INPUT` and `READ`** store through `Emitter::store` (`io.rs`), so they are covered by the three cases above.
  A target that takes another path gets the same treatment in task 2.2.
- **A built-in that is given a place** (`SWAP`, `MID$` and `ASC` as statements, `GET #` into a variable, `LSET`,
  `RSET`, `_MEMGET` into a place): a place with no index is read again after the call. For a place with an index,
  a traced build takes the address once into a temporary, passes the temporary to the call, and reads through
  it after the call. The read is under `if (!is_error_pending())`.
- **`CopyWhole`**: each scalar member of the target is read through the `copy_to` address the copy holds. A
  member array inside the value is recorded element by element (D9).

A statement that an error handler retries with `RESUME` runs its store again. The store is then counted again,
because it happened again.

**Arguments passed by reference.** The callee's stores into a parameter are recorded under the parameter's own
entry. What the caller's variable gets depends on the argument:
- **A variable, or a member with no index on the way** (`CALL inc(k)`, `inc p.x`): after the statement, the value
  is read again. It is recorded only when it differs from the entry's last recorded value. Such a record raises
  the entry's `after-calls` count, not its `stores` count. So `CALL show(k)` run 1000 times adds nothing when
  `show` never changes `k`.
- **An element** (`inc a(i)`) or **a whole array** (`fill grid()`): nothing is read after the call. The record
  gets an `alias` line that links the caller's array to the callee's parameter, with the number of times the
  call ran (D6). The report shows the link under the caller's array.

*Why not read an element after the call:* its address would have to be held across the call, and the callee may
`REDIM` or `ERASE` the array. The traced build would then read freed memory, which the normal build never does.

*Why the last recorded value and not a copy made before the call:* it needs no extra storage per call. For a
local variable of a recursive procedure, or after a `POKE`, the last recorded value can be out of date. The set
of distinct values is still right. Only the `after-calls` count can be one too high.

*Why the IR and not the C++ text:* the IR is where every kind of store has one shape. A search of the C++ would
miss stores that a libqb call does, such as `SWAP` and `GET #`.

*Alternative considered:* write the place's C++ again after each operation. Rejected: it evaluates an index
twice (`a(f(i)) = 1` would call `f` twice), and it counts a store that a pending error skipped.

*Alternative considered:* a hardware watchpoint per variable through `lldb`. Rejected: about four at once, and a
stop at every store.

*Alternative considered:* record once per statement, for every place the statement names. Rejected: `SWAP a, b`
and `a = a + 1` would also record places that are only read.

### D2. One entry per variable, array or member path

The entries are numbered while the bodies are emitted, as `coverage::Counters` numbers its statements:
- a scalar `Var` is one entry;
- an array `Var` is one entry for all its elements;
- a member path is one entry, its key the variable and the member ids. For example, `p.pos.x` is `(p, [pos, x])`,
  and an element of a member array is that member's entry.

A `_BIT` variable is traced as any integer. A `_BIT` scalar lies in a 32- or 64-bit variable of its own, and a
`_BIT` array's element is read with `getbits`, as `element_at` already writes it.

An entry's name is the IR's: upper case, without a type suffix, as in the debug symbol file. The entry's type
tells `A%` from `A$`. Entries never stored into are not written. Code: a new module
`crates\codegen-cpp\src\trace.rs` with `Entries::entry(place) -> usize`.

### D3. The run-time part is C++ in the generated program, not in libqb

`trace.rs` adds a fixed block of C++ to a traced program, as `coverage.rs` adds its writer. The block is a file of
its own, `crates\codegen-cpp\src\trace_rt.h`, taken in with `include_str!`, so that it reads as C++ and not as
Rust format strings. The block holds:
- per entry, a hash map from a value's bytes to its count and first site;
- the number of stores, the number of after-call records, the last recorded value, a cut flag, and the range
  (minimum and maximum for a number, lengths for a string);
- the counters of the `alias` and `untraced` lines;
- the writer, registered with `atexit` and `at_quick_exit`.

The map uses `std::unordered_map` with a key of `std::string` that holds the value's bytes. A number is keyed by its
stored bits, so `-0` and `0` are distinct and every `NaN` pattern is its own value. A `NaN` does not change the
range. A string is keyed by its full characters. The record keeps the first 200 bytes of its text, with the full
length and the FNV-1a 64 hash of all its bytes written beside it.

*Why not libqb:* libqb is QB64pe's runtime, shared with the old compiler (`UPSTREAM.md`). The trace is ours, and the
build without the option must not change. The block is about 150 lines, and only a traced build includes it.

*Threads:* the stores run on the program's thread. libqb's exit waits for that thread to stop before it calls
`exit` (`exit_ok` in `libqb.cpp`), so the writer does not read a map while a store changes it. A CLI test closes
the window during a loop to check this.

*The text of a number* is formed by the writer at exit, not at each store:
- an integer of any width in decimal;
- a `SINGLE` with `%.9g`, a `DOUBLE` with `%.17g`, which both read back to the same value;
- a `_FLOAT` with `%.21Lg`;
- an `_OFFSET` in decimal.

Shortest texts, as the debugger shows them (D7 of `m5-dap-proxy`), are left to the report tool. It can shorten a
text in Python and check that it reads back.

### D4. The limit of distinct values

Each entry keeps at most 256 distinct values, or the number `QB64RUST_TRACE_LIMIT` gives, read once at the first
store. After that, a new value only adds to the store count and the range, and sets the cut flag. This bounds
memory to entries × limit × the size of a value. A value already kept still has its count raised.

### D5. Every operation is placed in one of four groups

One function decides what an operation stores: `Op::stores` in `crates\ir\src\lib.rs`, beside `Op::may_raise`. It
is a `match` over `Op` with no `_` arm, so a new operation does not compile until it is placed. It lives in the
IR, not in the emitter, because the answer is a fact about the operation, and a later static analysis needs the
same answer.

| Group | Operations | What the record gets |
|---|---|---|
| Stores by name | `Assign`, `AssignAll`, `Lset`, `Input`, `Read`, `MemGet` into a place, `CopyWhole` (its scalar members), a `Builtin` with a `StmtArg::Place` or `StmtArg::Bytes` argument | a value in the place's entry (D1) |
| Calls | `Call`, and `ExprKind::CallProc` in any expression | after-call values and `alias` lines (D1) |
| Bulk stores | `CopyArray` (`b() = a()`), `ArrayCopy`, `Clear` and `EraseParam` (`ERASE`), `Allocate` (`DIM`, `REDIM`), `MemGet` into a whole array, a member array inside a `CopyWhole`, a store of a whole `_MEM` value | each element's value where the store brings new values (D9); otherwise an `untraced` line with the array's or variable's name |
| Stores at an address | `MemPut`, `MemFill`, `CallAbsolute`, a call of a `DECLARE LIBRARY` procedure, `Run` (it clears every variable), and the built-ins `POKE`, `_MEMCOPY`, `BLOAD`, `CLEAR`, `GET #` into a `FIELD` buffer, and a graphics `GET` into an array (`StmtArg::Rest`, `StmtArg::Block`) | an `untraced` line with no name |

Every other operation stores nothing. `Release` frees an array's elements and stores nothing either.

The emitter gives a site counter to each statement that holds a bulk store or a store at an address. The line
says what the statement was (`REDIM`, `POKE`, `b() = a()`), and how often it ran.

*Which bulk stores are counted and which are traced:* D9. The first design counted them all, to save the time
of recording each element. That left an array filled by `b() = a()` or `GET #` with no values and no range.

*Why count stores at an address:* a byte written at an address is not tied to a variable at compile time. The
record says where the program did this, so that the report is not silently incomplete.

### D6. The record format, version 1

```text
trace 1
file     <index> <hash> <path>                           as a coverage record
entry    <n> <name> <type> <procedure or -> <stores> <after-calls> <distinct> <cut 0|1> <min> <max>
value    <n> <count> <file> <line> <text> [<length> <hash>]
alias    <file> <line> <count> <name> <procedure or -> <callee> <parameter>
untraced <file> <line> <count> <what> <name or -> <procedure or ->
end
```

Three fields were added while building (2026-11-05). A string value longer than 200 bytes has its full length as
`<length>`, and the FNV-1a 64 hash of all its bytes as `<hash>` (16 hex digits). Without the hash, the report
merged two strings that share their first 200 bytes and their length (found in review). An `untraced` line of a bulk store gives the procedure of its array, so that the report can put it
under the right array when two procedures have an array of one name.

The fields are tab-separated. A string's text is written as a JSON string, quotes included, so that a tab or line
end in it cannot break the line. A string entry's `<min>` and `<max>` are lengths. A value's `<count>` is the sum
of its stores and its after-call records.

An `alias` line says: the statement at this file and line passed the array `<name>` of `<procedure>`, or an
element of it, to `<parameter>` of `<callee>`, `<count>` times.

The format is written in `tools\trace\README.md`. The record goes to the file `QB64RUST_TRACE` names, opened for
appending. When the variable is unset or empty, nothing is written.

### D7. The driver and the option

The option is `--trace-values`, beside `--coverage`, in `crates\driver\src\main.rs`. `compile.rs` gets a field
`trace: bool`, and `lib.rs` builds the `Trace` value the emitter takes, as it builds `Coverage`, from the same file
table and hashes (`coverage_files`). If the driver's build cache keys on the C++ text, the option needs nothing more.
Task 1.1 checks this. If the cache keys on the options instead, the option is added to its key.

### D8. The report tool

`tools\trace\value_trace.py` reads the source files, checks their hashes as `coverage.py` does, and merges the records:
- the counts, store numbers and after-call numbers add up;
- the distinct values are the union, and a value's first site is the one from the earliest record;
- an entry is cut when any of its records is;
- the range is the widest of the records;
- `alias` and `untraced` lines of the same site add their counts.

It prints one block per entry, sorted by procedure and name, with the values by count, most stored first. Under an
array it prints its aliases ("also stored through `A()` of `FILL`, line 12, 3 calls") and its bulk stores. It
then prints the stores at an address. `--json` gives the same data. `--var NAME` keeps the entries of that name, in
any letter case. The tool imports the record's file lines and the hash check from `tools\coverage\coverage.py`,
so that the two tools cannot disagree on how a file's hash is computed.

### D9. Bulk stores that bring new values are traced (user, 2026-11-05: "do it")

The first design counted every bulk store. An array filled only by `b() = a()` or by `GET #` then had no values
and no range in the report, which is the tool's main use. So a bulk store that brings new values is recorded
element by element, each element as a store. The traced build pays about 75 ns per element; the normal build is
unchanged.

| Statement | What is recorded | Where the elements are read |
|---|---|---|
| `b() = a()` (`CopyArray`) | every element of the target | through the target's descriptor, inside the copy's own block, when it copied |
| `_ARRAYCOPY` of one dimension | the elements libqb wrote: the range's length from the destination's start | through the destination's descriptor, after the call |
| `GET #` into a whole array | every element | by the array's name, after the call |
| `_MEMGET` into a whole array | every element | by the array's name, or the member array's when no index leads to it |
| `_UCHARPOS` | as many elements as it returns, at most the array's | by the array's name, after the operation; the result is held in a temporary inside the expression |
| A member array inside a copy of a whole type, a `SWAP` or a `GET #` of one | every element | through the address the store holds |
| A copy of a type that owns `_DYNAMIC` member arrays | its numbers and strings | through the address the copy's last part found, kept in a temporary |

Each read is under `if (!is_error_pending())` and evaluates nothing: no index and no call is written a second
time. The last row is why. That copy finds its target once per part, so `o(f(i)) = one` calls `f` three times
in the normal build. The traced build must call it three times too, so it keeps the address the last part found.

Two details of the reads, found in review:
- `_FLOAT` elements of an array variable are 16 bytes apart (`long double`s side by side), though the type's size
  is 32. Inside a user type they are 32 bytes apart. The loop steps by the right one.
- A copy of an array of an owner type with other bounds raises error 9 and is still made
  (`QB64PE-DIVERGENCES.md` D-022). Its elements are recorded without the test for a pending error.

These stay counted, as `untraced` lines:
- `ERASE`, `DIM` and `REDIM` write zeros and empty strings. That is a variable's starting value, which is never
  listed.
- A store of a whole `_MEM` value, and a `_DYNAMIC` member array inside a copy of a whole type.
- A bulk store whose target cannot be read without an index: `_ARRAYCOPY` of several dimensions, `GET #` into an
  array from an element on (`GET #1, , a(5)`), and `_MEMGET` into a member array reached through an index.

**The strings of a `FIELD`.** `FIELD` makes each of its strings a part of a file's record buffer, and `GET #`
into that buffer changes them all. After each of the two statements, the traced build reads again every string
that a `FIELD` of the program names, and records it when it changed, as it does for a variable after a call
(D1). The compiler cannot know which file a `GET #` reads, so it reads the strings of every `FIELD`. A string
reached through an index, or a local string of another procedure, cannot be read there. When the program has
one, the statement is also listed as a store at an address.

## Found while building (2026-11-05)

These cases were not in the decisions above. Each was settled by the rule the decisions already give.

- **`FIELD`** makes each of its strings a part of the file's record buffer, so it changes them. First listed as
  a store at an address; now its strings are read again (D9).
- **`_UCHARPOS`** is a built-in function that fills an array it is given. First listed as a bulk store; now the
  elements it fills are recorded (D9).
- **`GET #` into a whole array** was first listed as a bulk store; now its elements are recorded (D9).
- **`ASC(s$, n) = c`** stores by name, as `MID$` as a statement does.
- **A variable of a user type passed by reference** (`move p`): each number and string in it is read after the
  statement, as a scalar variable is (D1). Each member array in it gets an `alias` line to the parameter's
  member array.
- **A copy of a whole user type that owns `_DYNAMIC` member arrays** finds the target anew for each part and
  holds its address in no one place. Reading the target afterwards by its place would evaluate an index once
  more, which D1 forbids. First such a copy through an index was listed as one bulk store; now the copy's last
  part leaves the address in a temporary and the target is read through it (D9).
- **`_MEMGET` into a variable-length string reached through an index** is not read back. The copy is given the
  string's characters, not the string.
- **`SWAP` on Linux.** The two places are held in temporaries inside the call's own arguments
  (`swap_32((t1=&a),(t2=&b))`), not before the call. So the order in which `g++` evaluates the arguments is
  the normal build's (`QB64PE-DIVERGENCES.md` D-054). The other built-ins that are given a place hold it the
  same way.
- **A jump inside a statement.** The read after a call is written before the jump of an `IF` whose condition
  made the call, since the lines after the jump would be skipped.
- **The `after-calls` of a variable with no record yet** compare with its starting value, 0 or an empty string,
  so a call that leaves a never-stored variable alone records nothing.

## Risks / Trade-offs

- [A store missed by the hook, for an op added later] → `Op::stores` is a `match` over `Op` with no `_` arm, so a
  new op does not compile until it is placed. A CLI test runs a program with every statement form of the spec and
  checks each entry.
- [A traced build that behaves differently] → D1 writes no place twice. Task 2.6 builds the corpus and the
  verification families `v18` and `v24` with the option and compares each program's output with its recording.
  Those families hold the programs with a `FUNCTION` in an index.
- [A traced program runs much slower] → only the traced build pays. A hash lookup per store is about tens of
  nanoseconds. The screen-state scripts' time limits are not used with this build.
- [Memory for programs with many arrays of strings] → the limit of D4. A string key is its full text, so a 1 MB
  string kept 256 times is 256 MB. The writer keeps only 200 bytes, but the key must be the full text to be
  correct. Mitigation: a string longer than 4096 bytes is keyed by its length and an FNV-1a 64 hash of its bytes.
  Two such strings that share both are taken as one value. The README says so.
- [An array changed only through a parameter shows no values of its own] → its `alias` lines name the parameter
  whose entry holds them, and the report prints the link. The values are not merged, because one parameter may
  stand for several arrays.
- [The record's view of a by-reference variable is the value at the end of the statement] → a `FUNCTION` that
  changes its argument twice in one expression shows only the last value under the caller's name. The callee's
  parameter entry has each value.
- [Entries for variables the lowering makes] → `Storage::Temp` variables are not traced. Only variables named in
  the source get an entry.
