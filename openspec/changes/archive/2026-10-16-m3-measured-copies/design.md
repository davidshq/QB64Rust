# Design

## Context

What was measured and why: `proposal.md` (`verification\v38_*`). Where the code stands (2026-10-16):
`codegen-cpp\src\owner.rs` `owner_file` writes `PUT`/`GET` of a whole array of an owner type through per-type
functions in `global.txt` (`qbr_udtN_pack`, `_unpack`, `_fcount`, `_file`, `_commit`) with one buffer per type
(`qbr_udtN_fbuf`); `_commit` runs only when no error is pending. `file_conv` moves nothing for a dynamic member
whose bounds or number of dimensions differ from the declared ones. `arrays.rs` `copy_array` writes every
whole-array assignment as the old compiler's `AppendWholeArrayAssign`: other bounds stop the copy.

## Decisions

### D1. A buffer per `GET` statement, cleared when it grows, unpacked after every read

`owner_file` gives each `GET` (and `PUT`) its own buffer, `static uint8 *qbr_fbufN` and `static uint64
qbr_fcapN` in `global.txt` (a counter of the emitter), passed to `_file` and `_commit` by address. `_file` grows it
with `realloc` and clears the new bytes with `memset`. `_commit` runs after `sub_get` whatever it raised; it
unpacks nothing when the array is not allocated (D-021's `GET` into a released array: error 9 once, from
`_file`, and nothing read) or when the buffer is smaller than the array.

*Why per statement:* the old compiler's `UDTFileArrayBE$` makes one buffer per statement (`uniquenumber`), and
what a failed read brings back is that statement's last read (`v38_a` 2.). *Why cleared:* the old buffer is
`realloc` memory that nobody wrote; it read as zeros in every run measured. Reading it is the misuse; zeros are
what the program printed.

### D2. A member with other bounds: its declared bytes when its block holds them

In `file_conv`, for the unpack, a dynamic member whose descriptor is allocated but whose bounds or number of
dimensions differ raises error 9 (as now) and then copies the declared elements into its block when
`qbr_dyn_count(d)` is at least the declared count; its bounds stay. With fewer elements, or a descriptor not
allocated, nothing is moved (D-019 narrowed). The pack for `PUT` is unchanged: error 9, nothing packed for that
member, and libqb writes nothing with an error pending, as the old program does.

*Alternative:* keep D-019 as it was (nothing moved for any mismatch). Rejected: the user's rule of 2026-10-16
for the dimension count ("QB64pe's way unless it misuses memory") is the same check as for other bounds; the old
check does not tell them apart, and only a block with fewer elements is written past.

### D3. D-022 narrowed: the target's number of elements when the source has them

`copy_array` for two array variables of an owner type: both allocated, else error 9 and nothing (as now). Other
bounds or shape raise error 9 once; then the target's number of elements is copied, element `i` of the target from
element `i` of the source, by `copy_owner`, when the source has at least that many elements; else nothing. The
target keeps its bounds. Member arrays of an owner type keep `AppendWholeArrayAssign`'s rule (other bounds copy
nothing), as the old compiler does for them.

### D4. The dimension count in a whole-array copy: no change

`v38_c` prints the same with both compilers. The comment on `copy_array` that said the old compiler "reads the
slots of another number" is corrected: its guard gives error 9 and a stand-in descriptor, so nothing is copied.

## Risks / Trade-offs

- A buffer per statement costs a static pointer per `GET`/`PUT` of an owner array, as in the old compiler.
- `cpp` snapshots with a whole-array file statement or an owner copy change; each is read before it is accepted.
