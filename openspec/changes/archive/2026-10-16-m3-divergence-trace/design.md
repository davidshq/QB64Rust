# Design

## Decisions

### D1. A function in `global.txt`, a file named by the environment

`qbr_diverged(int32 n)`, first in `global.txt`: on its first call it reads `QB64RUST_DIVERGENCES`; when that
names a file, the first call for each row opens it for appending and writes `D-0nn` and a line end. Not stdout or
stderr: the corpus compares the merged output, and a screen program's window has neither. Once per row and run: a
loop that takes the path a million times writes one line. The old compiler's programs never write the file, so
the runner can set the variable for every run with either compiler.

*Alternative:* a line on stderr, filtered out by the runner. Rejected: every comparison would need the filter,
and a program's own stderr output could be taken for a trace.

### D2. Where each row is traced

| Row | Where | When |
|---|---|---|
| D-003 | `stmt.rs`, `RETURN label` | no `GOSUB` pending |
| D-004 | `stmt.rs` `store_through_member`, `member_path`; `arrays.rs` `copy_array` | an index outside its bounds; an index that raised with nothing pending; a guarded copy of strings or user types after an error |
| D-006 | `qb_safe_idiv`, `qb_safe_mod` | the smallest value of a signed type with -1 |
| D-016 | `value.rs`, `_MEM` | `ExprKind::MemOf::lock_slot0` (set by `sema` where no fact about the parameter's number of dimensions was recorded before it, in the order the program is checked) |
| D-018 | `arrays.rs` `allocate_with` | the array's block or allocated flag changed while its bounds were evaluated |
| D-019 | `owner.rs` `file_conv` (unpack) | a member that keeps its elements |
| D-020 | `arrays.rs` `allocate_with` | an overflow check of a dynamic member's `REDIM` |
| D-021 | `qbr_dyn_unset` (`_MEM` of a whole dynamic member), `_file`, `_commit` | a member or array not allocated; a failed read that unpacks bytes the statement never read |
| D-022 | `copy_owner_array` | a source with fewer elements, or not allocated beside a target that is |

### D3. The runner's sidecar

`<name>.divergences` beside a corpus or compile-suite program: one `D-0nn` per line, `#` comments. A row the
trace names and the sidecar does not is a failure; a listed row not taken is not (a list is what a program may
take, as folding can remove a path: `QB64RUST_NO_FOLD=1`). The pass cache's key holds the sidecar.

## Risks / Trade-offs

- Every `cpp` snapshot changes once (the helper); each of the 47 was checked by undoing the trace's additions and
  comparing with the old snapshot: no other change.
- The call-site check compares libqb calls; the trace is no libqb call (1677 of 1677 equal).
- D-016's flag reads the checker's facts in checking order; a fact recorded later (a call from a procedure below)
  leaves the flag set, as the old compiler's one pass does.
