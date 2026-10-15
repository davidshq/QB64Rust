# Proposal

## Why

The user took the `$UNSTABLE:TYPEFIELDS` member markers out of `SOMEDAY.md` ("Do 1, 2, 3", `DECISIONS.md`
2026-10-14), after `REDIM _RETAIN` (`m3-redim-retain`, done). It is the largest blocker left upstream: 109 of
the 118 programs on `tests\upstream\deferred.list` have `$UNSTABLE:TYPEFIELDS`, and 93 of them need nothing else
that is still deferred (76 the markers alone, 17 the markers and `_RETAIN`; the other 16 also need whole-array
assignment or `_ARRAYCOPY`). The user chose **one change for the whole family**, measured before the design
(2026-10-14, two answers before this proposal).

Read and measured before this proposal (`qb64pe.bas` and `utilities\type.bas` at the reference clone's commit,
`16f629784e`; `verification\v35_*`, 69 programs recorded with `qb64pe.exe`, 2026-10-14):

- **`$UNSTABLE:<flag>` is read in source order, before the program's first pass**: `TYPEFIELDS` allows the
  markers in the `TYPE` blocks *below* it, also when it stands after the first statement, and not in a `$IF`
  branch that is not taken; below the `TYPE` it is too late (`v35_x14`, `x15`, `x45`, `x47`). The flag may be in any letter case, written
  twice, and with a blank after the colon (`v35_h`, `x13`). `MIDI` and `HTTP` are accepted with the warning "No
  longer required, feature is stable now". Any other flag is "Unrecognized unstable flag"; `$UNSTABLE` alone is
  "Syntax error" (`x10`–`x12`). Without the flag a marker is "_Static/_Dynamic require $Unstable:TypeFields"
  (`v24_x204`, `x205`).
- **The markers**: `_DYNAMIC` or `_STATIC` before a member's name, after its name or bounds, or after its type,
  also in an `AS type name(…) _DYNAMIC, …` list (where it marks that name only); not between `AS` and the type
  (`x01`–`x05`, `x44`). Both markers, or one twice, are an error, as is a marker on a member that is no array
  (`x06`–`x09`). A `CONST` may name a bound; `v() _DYNAMIC` is "Array bounds missing"; `_BIT` is refused in a
  `TYPE` as before (`x16`, `x17`, `x25`).
- **`_STATIC` changes nothing**: the member stays inline, with today's rules (`v35_f`, `x34`, `x40`, `x53`).
- **A `_DYNAMIC` member is an array of its own, owned by the value**: the value holds a pointer to the member's
  descriptor; every variable, element and copy has its own. It starts with its declared bounds and zero or empty
  elements (under the `OPTION BASE` of the block); `REDIM` (plain, `_PRESERVE`, `_RETAIN`, another number of
  dimensions, `SHARED`, a list), `ERASE` (then every use is error 9 until a `REDIM`), `LBOUND`/`UBOUND`, an
  index outside (error 9), every kind of element (numbers, `_OFFSET`, `STRING * n`, `STRING`, user types with
  strings or with `_DYNAMIC` members of their own) (`v35_a`, `v35_c`, `v35_g`, `x01`–`x38`).
- **The value's life**: a whole-value copy is deep and its indexes are evaluated per part as the old compiler
  writes them (`x56`); `SWAP` exchanges; a parent array's `REDIM` (plain resets the members, `_PRESERVE` and
  `_RETAIN` keep them), `ERASE` of a dynamic parent frees them, `ERASE` of a static parent frees them and leaves
  the members unallocated until a `REDIM` (`v35_b`, `x51`); a local is new on each call, a `STATIC` local keeps
  its members, a parameter of the type and a member passed as an array argument are by reference and a `REDIM` in
  the callee is seen by the caller (`v35_d`, `x36`); `READ`, `INPUT`, `INPUT #`, `LINE INPUT #`, `SWAP` and the
  `MID$` statement take an element (`v35_e`, `x42`).
- **Memory layout**: a type with a `_DYNAMIC` member, in itself or in a nested user type, is laid out with each
  pointer and each nested such type at a multiple of 8 bytes and its size rounded up to 8 (`x52`: 24 and 40
  bytes); other types keep today's packed layout (`x53`). `_MEM` of such a value gives that layout (`x22`, `x23`).
- **Files**: `PUT` and `GET` of a whole array of such a type write and read the *declared* layout (the members at
  their declared bounds, packed); a member with other bounds raises error 9, and `PUT` then writes nothing
  (`v35_e`, `x55`). `PUT`/`GET` of one variable or element, `LEN` of the value, `_OFFSET` of the value and
  `COMMON` of such a variable are compile errors; with a `STRING` anywhere in the type `PUT`/`GET` is "UDT must
  have fixed size" (`x20`, `x39`, `x43`, `x46`, `x48`, `x50`, `x54`).
- **The parent's place is evaluated for every use of the member**, `LBOUND`/`UBOUND` included; an index outside
  raises error 9 and element 0 stands in (`x58`, `x59`).
- **Three behaviours of the old compiler that misuse memory** (measured): `GET` of an array whose member has
  other bounds raises error 9 and still writes the declared bytes into the smaller block (`v35_e`, 6.); a member's
  `REDIM` to more elements than memory holds succeeds with an overflowed size (`v35_g3`); a store through a
  parent index outside its bounds writes into the member of element 0 (`x58`). Bounds reversed at run time end
  the program with "Out of memory", as for any array (`v35_g2`).

## What Changes

- **`$UNSTABLE:TYPEFIELDS`, `$UNSTABLE:MIDI` and `$UNSTABLE:HTTP`** compile (the last two with the warning); the
  other forms of `$UNSTABLE` are compile errors. No longer "not supported yet".
- **The `_STATIC` and `_DYNAMIC` member markers** compile in every position the old compiler takes, with its
  errors for the others.
- **`_DYNAMIC` member arrays** compile with the behaviour above: their own descriptor in each value, set up,
  released, copied deep, exchanged and reset with the value; `REDIM`, `ERASE`, `LBOUND`, `UBOUND`, elements as
  places, the member as an array argument; every kind of element.
- **Types that own a `_DYNAMIC` member get the old compiler's aligned layout**; whole arrays of them go to and
  from a file in the declared layout; the forms the old compiler rejects are compile errors.
- **Three memory misuses are fixed** (user, 2026-10-14, the recommendation made): `GET` leaves a member with other
  bounds untouched; a member too large ends the program with "Out of memory" as an array does; a store through a
  parent index outside its bounds stores nothing (as `DIVERGENCES.md` D-004). `DIVERGENCES.md` D-019, D-020, and
  D-004 widened.
- **Tests and lists**: `verification\v35_*`; front-end tests (the metacommand, the markers, the errors, the IR,
  the C++ of each kind of element); console slice programs on `tests\corpus\slice.list`; the 93 programs leave
  `tests\upstream\deferred.list`, and those that pass join `pass.list` (about 90, so about 283 of 379 counted).

Out of scope: whole-array assignment `a() = b()` and `_ARRAYCOPY` (still deferred, with the 16 programs that need
them); `$UNSTABLE` flags other than the three.

## Capabilities

### New Capabilities

- `language/unstable-features`: the `$UNSTABLE` metacommand: its flags, where it takes effect, its errors and
  warnings.

### Modified Capabilities

- `language/arrays-and-types`: requirement "Member arrays" (the markers, `_STATIC`, the scenario "Marked member"
  replaced), new requirement "Dynamic member arrays" (the member, its life with the value, its statements,
  layout and limits), requirement "Whole user-type values" (deep copy and the evaluations of an owner type).
- `language/record-files`: requirement "What a variable reads and writes" (whole arrays of an owner type in the
  declared layout; the rejected forms).
- `compiler/pipeline`: requirement "Member arrays in the IR" (a `_DYNAMIC` member is allocated and released like
  a dynamic array; the life of an owner value is not spelled out in the IR).

## Impact

- `crates\syntax` (`$UNSTABLE` as a metacommand; the marker after the type and in `AS` lists), `crates\sema`
  (the flag in source order, `Member` gains its storage, the marker rules, places of a `_DYNAMIC` member, the
  rejected uses, `REDIM`/`ERASE` of it), `crates\ir` (allocation and release of a member array, its dump and
  validation), `crates\codegen-cpp` (`decl.rs`: the aligned layout and the owner parts; per-type set-up, release,
  copy and reset helpers; `arrays.rs`: `allocate_with` on a member's descriptor; `place.rs`: elements through
  the descriptor; `io.rs`: the declared layout for files; `mem.rs`).
- Tests: `tests\frontend\` (new programs and snapshots; `member_arrays_errors.bas` loses its marker lines),
  `tests\corpus\` (new slice programs), `tests\upstream\deferred.list`, `pass.list`, the shrink-only lists.
- Documents: `SOMEDAY.md` (the `TYPEFIELDS` row goes; new behaviours to review), `DIVERGENCES.md`, `DECISIONS.md`,
  `study\00` §5, `CLAUDE.md`'s layout table (`v35*`), `crates\README.md`, `tests\upstream\README.md`, `STATUS.md`,
  `ROADMAP.md`.
