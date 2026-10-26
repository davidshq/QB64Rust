# Spec Delta

## Purpose
The debug symbol file: what a debug build writes beside the executable so that a debugger can show the running
program in BASIC terms (names, types, scopes, layouts), and what a reader of the file may rely on.

## ADDED Requirements

### Requirement: The debug symbol file
A debug build (`qb64rust --debug`, spec `compiler/cli`) SHALL write a file `<exe>.debug.json` beside the
executable, a JSON document with a top-level `version` (an integer, 1 for this format) and the sections below. The
file SHALL describe the program as built: every name it holds SHALL be one the executable's debug information or
memory holds, so that a reader needs the compiler for nothing else.

#### Scenario: Written beside the executable
- **WHEN** `qb64rust --debug -x p.bas -o out\p.exe` builds
- **THEN** `out\p.exe.debug.json` exists, parses as JSON, and its `version` is 1

### Requirement: No symbol file without a debug build
A build that is not a debug build SHALL write no debug symbol file. Every build SHALL remove one an earlier build
left at that path when it removes the earlier executable, so that a symbol file beside an executable is always
that build's (a reader trusts its C names and layouts; there is no `qb64pe` behaviour to match, the file being
this compiler's own).

#### Scenario: Not written otherwise
- **WHEN** `qb64rust -x p.bas -o out\p.exe` builds and `out\p.exe.debug.json` was left by an earlier build
- **THEN** the file is removed

#### Scenario: A failed build
- **WHEN** `qb64rust --debug -x p.bas -o out\p.exe` fails and `out\p.exe` and `out\p.exe.debug.json` were left by
  an earlier build
- **THEN** both are removed

### Requirement: Files
The file SHALL list the program's source files (`files`): each with its index, its path as the `#line` directives
name it, and the same hash of its text as a coverage record.

#### Scenario: An included file is listed
- **WHEN** the program includes `lib.bi`
- **THEN** `files` has an entry for `lib.bi` with the hash of its text

### Requirement: Procedures
The file SHALL list the program's procedures (`procedures`): the BASIC name without suffix, the kind `sub` or
`function`, the C function's name, the file and 1-based line of the header and of the closing `END SUB` or `END
FUNCTION` (`end_file`, `end_line`), the variables that are its parameters in order, and its result variable. The
main module SHALL be described as the C function `QBMAIN`.

#### Scenario: A SUB
- **WHEN** a program has `SUB show (n AS LONG)` on line 16 with `END SUB` on line 20
- **THEN** `procedures` has an entry with name `SHOW`, kind `sub`, C name `SUB_SHOW`, lines 16 and 20, and one
  parameter naming the variable `N` of that procedure

#### Scenario: A FUNCTION of an included file
- **WHEN** the program includes `lib.bi`, which defines a FUNCTION
- **THEN** the FUNCTION's entry names the file of `lib.bi`

#### Scenario: A SUB closed in an included file
- **WHEN** `SUB show` is opened in `p.bas` and its `END SUB` is in `end.bi`, which `p.bas` includes in the SUB
  (the old compiler reads included text so, `verification\v16_m7_sub_across`)
- **THEN** the SUB's `file` is that of `p.bas` and its `end_file` that of `end.bi`

### Requirement: Procedures with no body
Procedures of a `DECLARE LIBRARY` block and the OpenGL routines SHALL be left out of `procedures`, with their
parameters and results (they have no BASIC body).

#### Scenario: A library function
- **WHEN** a program declares `FUNCTION f& (BYVAL x AS LONG)` in a `DECLARE LIBRARY` block
- **THEN** `procedures` has no entry for `F`, and `variables` none for its parameter `X`

### Requirement: Variables
The file SHALL list every variable the program allocates (`variables`), each with its BASIC name in upper case,
its type as BASIC spells it, its storage (`main`, `static`, `local`, `param` or `result`, the last four with the
procedure's index), its shape (`scalar`, `static` with bounds, or `dynamic` with a number of dimensions), whether
it is in conventional memory, and its C name. Temporaries of the lowering SHALL be left out.

#### Scenario: A type as BASIC spells it
- **WHEN** a program has `DIM bits AS _BIT * 3` and `DIM u AS _UNSIGNED INTEGER`
- **THEN** their types are `_BIT * 3` and `_UNSIGNED INTEGER`

#### Scenario: Each storage
- **WHEN** a program has `DIM SHARED total AS LONG`, a `SUB` with `STATIC calls AS INTEGER`, a local `x`, a
  parameter `n AS LONG` and a FUNCTION `f&`
- **THEN** `variables` holds `TOTAL` once (`main`, `pointer`, C name `__LONG_TOTAL`), `CALLS` (`static` of the SUB),
  `X` (`local` of the SUB), `N` (`param` of the SUB, `param-pointer`) and `F` (`result` of the FUNCTION)

#### Scenario: A static array
- **WHEN** a program has `DIM a(1 TO 3, 0 TO 1) AS SINGLE`
- **THEN** `A` has type `SINGLE`, shape `static` with bounds `[[1, 3], [0, 1]]` and is reached by `descriptor`

#### Scenario: A dynamic array
- **WHEN** a program has `REDIM b(n) AS STRING`
- **THEN** `B` has shape `dynamic` with 1 dimension and type `STRING`

### Requirement: How a value is reached
Each variable SHALL say how its value is reached from its C name (`reach`): `pointer` (a C pointer variable to the
value, as `int32 *__LONG_TOTAL`), `descriptor` (a C pointer variable to an array descriptor, also for an array
parameter), `string` (a C pointer to a `qbs`), `bytes` (a C pointer to a user type's bytes), or `param-pointer` (a
parameter that is no array: a C pointer to the caller's value, or to a copy).

#### Scenario: Read through the reach
- **WHEN** a debug build stops in a SUB under the toolchain's `lldb`
- **THEN** a main-module variable, a `STATIC`, a local, a parameter, an element of a static array, a `STRING` and
  the members of a user type each read through the expression its `c_name`, `reach` and the layouts give

### Requirement: User types
The file SHALL list every user type (`types`): its name, its size in bytes, and its members in declaration order,
each with name, type as BASIC spells it, byte offset, size, bounds of a member array (empty for a single value)
and whether it is `_DYNAMIC`. The offsets and sizes SHALL be those the emitter lays the type out with, so that a
reader that adds the offset to the value's address finds the member.

#### Scenario: A type with a fixed string
- **WHEN** a program has `TYPE pt` with `x AS INTEGER` and `nm AS STRING * 4`
- **THEN** `types` has `PT` of size 6 with `X` at offset 0 size 2 and `NM` (`STRING * 4`) at offset 2 size 4

### Requirement: The array descriptor's layout
The file SHALL state, in `layouts.array`, the layout of an array descriptor the emitter relies on: the slot of the
data pointer and of the flags, and for each dimension the slots of the lower bound and of the count and the order
the dimensions are stored in.

#### Scenario: The descriptor's slots
- **WHEN** the file of any debug build is read
- **THEN** `layouts.array` says the data pointer is slot 0, the flags slot 2, and dimension `k` of `n` (0 first) has
  its lower bound at slot `4 * (n - 1 - k) + 4` and its count at the slot after it

### Requirement: Other fixed layouts
The file SHALL state the pointer size, the byte offsets of `chr` and of `len` in a `qbs`, the size of a `_FLOAT`
element in an array, and that a `STRING` member of a user type is a pointer to a `qbs` and a `STRING * n` member
is `n` bytes in place.

#### Scenario: The qbs layout
- **WHEN** the file of any debug build is read
- **THEN** `layouts.qbs` gives `chr` at 0 and `len` at 8, the offsets of the first two members of libqb's
  `struct qbs` in the copy

### Requirement: Debug-info dump
`qb64rust --dump debug-info p.bas` SHALL print the debug symbol file of the program without building it, with
exit status 0 when the program has no errors and 1 otherwise; its text SHALL equal what a debug build writes but
for the files' paths, which SHALL be as given to the compiler.

#### Scenario: Dump of a valid program
- **WHEN** `qb64rust --dump debug-info p.bas` runs on a program without errors
- **THEN** the JSON is printed and the exit status is 0
