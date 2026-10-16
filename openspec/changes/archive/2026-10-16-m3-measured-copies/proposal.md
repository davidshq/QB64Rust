# Proposal

## Why

The user's answers of 2026-10-16 (`DECISIONS.md`, items (2) and (3)) left four behaviours of owner types (types
with `_DYNAMIC` member arrays, below `$UNSTABLE:TYPEFIELDS`) to be measured and then done QB64pe's way unless that
way misuses memory, and narrowed D-022. Measured with `qb64pe.exe` before this proposal (`verification\v38_*`,
four programs, recorded 2026-10-16) and read in `qb64pe.bas` (`UDTFileArrayBE$` 17181,
`AppendUDTFileBoundsCheck` 17047, `UDTDynArrayDescGuard$`, `AppendDynUDTArrayAssign` 16855):

- **The unpack after a `GET` whose read raised an error** (`v38_a`, D-021's third part): the old compiler
  unpacks its buffer into the array after every `GET` of a whole array of an owner type, whatever the read did.
  The buffer belongs to the statement: a `GET` that read before brings back what it read last (`2. 2`: error 52,
  then the earlier record's values); one that never read unpacks a buffer just allocated and never written, which
  gave zeros on every run (`1.`, error 52; `3.`, error 59). This compiler unpacks nothing after a failed read.
- **The number of dimensions in the bounds check of a file's dynamic member** (`v38_b`): the old check compares
  slot 3 with the declared number and raises error 9, and then, like every other mismatch, copies the member's
  declared bytes anyway: `GET` writes them into the member's block, whose bounds stay (cases 1–3, 5, 6, also a
  member with other bounds of the same number); with a block of fewer elements it writes past its end (case 4,
  D-019). `PUT` raises error 9 and writes nothing, as here. This compiler leaves every such member as it was.
- **The number of dimensions of a dynamic member's descriptor in a whole-array copy** (`v38_c`): the old guard
  gives error 9 and a stand-in descriptor, so nothing is copied, in every case measured. This compiler already
  prints the same output: nothing to change but a comment that said otherwise.
- **D-022, arrays of an owner type with other bounds** (`v38_d`): the old compiler raises error 9 and copies as
  many elements as the target has, element by element in order, deep, the target keeping its bounds: a static or
  a `REDIM` target, fewer or as many elements, another shape of two dimensions (cases 1, 2, 4–6, 8). With more
  elements in the target it reads past the source's block (case 3); from a source that `ERASE` released it reads
  through a missing block (case 7). This compiler copies nothing in every case.

## What Changes

- **A `GET` of a whole array of an owner type unpacks its buffer after every read**, as the old compiler does: the
  buffer is the statement's own, so a failed read brings back what that statement read last. The bytes the
  statement never read are 0 (the buffer is cleared when it is allocated or grown), which is what the old program
  printed in every run; reading memory never written is the misuse left out. `DIVERGENCES.md` D-021 loses its
  third part.
- **A dynamic member with other bounds or another number of dimensions gets its declared bytes from a `GET`**,
  after error 9, when its block holds at least the declared number of elements; with fewer, nothing (D-019,
  narrowed to the case where the old program writes past the block). The bounds check of a dimension count is the
  old compiler's (measured, nothing to change). D-019 is narrowed by the rule the user set for these cases; for the
  user to confirm.
- **D-022 narrowed** (user, 2026-10-16): a whole-array assignment of arrays of an owner type with other bounds
  raises error 9 and copies the target's number of elements when the source has at least that many; nothing when
  it has fewer or is not allocated.
- **Tests**: the four `v38` programs; CLI tests of D-019, D-021 and D-022 rewritten to the new outputs; a slice
  program `s126_owner_copies` with the lines that now equal the old compiler's, recorded with `qb64pe.exe`.

Out of scope: the in-place copy of a `REDIM` target (the old compiler gives it a new block, with the same values;
accepted 2026-10-16); the run-time trace of divergences (its own change, next).

## Capabilities

### Modified Capabilities

- `language/record-files`: requirement "What a variable reads and writes" (the unpack after a failed read; a
  member with other bounds).
- `language/arrays-and-types`: requirement "Whole-array assignment of owner types" (other bounds).

## Impact

- `crates\codegen-cpp\src\owner.rs` (`owner_file`, `file_functions`, `file_conv`), `arrays.rs` (`copy_array` for
  array variables of an owner type), `lib.rs` (a counter for the statements' buffers).
- Tests: `crates\driver\tests\cli.rs`, `tests\frontend` `cpp` snapshots that hold a whole-array file statement or
  an owner copy, `tests\corpus\slice\s126_owner_copies`.
- Documents: `DIVERGENCES.md` (D-019, D-021, D-022), `study\00` §5 (`v38`), `verification\README.md`,
  `SOMEDAY.md`, `DECISIONS.md`, `STATUS.md`.
