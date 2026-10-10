# Questions for the user

Questions met while applying this change (2026-10-10). Each was recorded here and the work went on with the
choice named under "Done meanwhile"; none blocks a task. Both are answered (2026-10-10).

## Q1. Should the emitter leave integer argument conversions to C++, as the old compiler does?

Found in task 2.2 (the call-site check). The old compiler writes an integer argument of a number slot as it is
(`sub_seek(*__LONG_L,*__INTEGER_I)`, `func_chr(qbr(x))`) and lets C++ convert it to the parameter's type. The new
compiler's IR states the conversion and the emitter writes it (`func_chr(((int32)(qbr(x))))`). Both are the same
C++ when the cast names the parameter's type.

- **Done meanwhile:** a normalisation rule in the tool drops an integer cast around a whole call argument
  (`tools\callsite\README.md`, rule 4). The rule names no built-in, but it makes the check blind to a cast that is
  narrower than the parameter (the one case met: `SEEK`'s position is `int64` in libqb while the table says LONG;
  it has a slot type of its own and a slice line with a position beyond 32 bits).
- **The alternative:** the emitter writes no integer cast on a call argument, as the old compiler. The check then
  needs no rule 4 and is stricter; about 40 `cpp` snapshots change once and tier 2 must be rerun.
- **Answered (user, 2026-10-10, `DECISIONS.md`):** no. The emitter keeps its casts and the tool keeps rule 4;
  about 40 snapshots and a tier 2 rerun are too much for a stricter trial tool.

## Q2. Array elements are left out of the call-site programs. Is that acceptable for a "kept" verdict?

The two compilers spell an element's index differently (`1` against `1ll`, an explicit `int64` cast before the
lower bound is subtracted). That is element addressing (`m2-arrays-and-types`), not the built-in's call, and it
would need two more normalisation rules (drop an `ll` suffix, drop an integer cast on an operand) that weaken the
check everywhere.

- **Done meanwhile:** the programs pass variables, literals, expressions and fixed-length strings; elements as
  arguments are covered by the slice programs only (`tools\callsite\README.md`, "What it does not compare").
- **Answered (user, 2026-10-10, `DECISIONS.md`):** yes, acceptable. Element addressing is covered by the slice
  programs; no further normalisation rules are added for it.
