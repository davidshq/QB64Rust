# Questions for the user

Questions met while applying this change (2026-10-10). Each was recorded here and the work went on with the
choice named under "Done meanwhile"; none blocks a task. All three are answered (2026-10-10).

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

## Q3. `SWAP` with an element whose index is out of range exchanges with the array's first element. Keep it?

Found in task 5.1 (`verification\v22_d_swap`, the last two lines; `tests\corpus\slice\s40_swap_mid`). The old
compiler writes `swap_32(&l,&((int32*)(arr[0]))[array_check(…)]);` with no test after `array_check`, which raises
error 9 and gives position 0. So `SWAP l, arr(9)` on a `DIM arr(3)` raises 9 **and** exchanges `l` with `arr(0)`.
Nothing outside the array is touched and nothing crashes, so by the rule of `DECISIONS.md` 2026-10-08 it is a
questionable behaviour, not wrong code. It is close to `DIVERGENCES.md` D-004 though (a store into a member of an
element with a bad index, where the old compiler writes the first element and the decision was to skip the store),
which is why it is asked.

- **Done meanwhile:** QB64pe's way. The new compiler emits the same call, `s40_swap_mid` records the old
  compiler's result (`0  2  1`) and passes, and the behaviour is listed for `SOMEDAY.md` "QB64pe behaviours to
  review" (task 8.4).
- **The alternative:** skip the exchange when an operand's index raised, as D-004 does for the member store: a
  row in `DIVERGENCES.md`, an index check before the call in the emitter, the two `s40` lines recorded as a known
  difference, and one more thing the call-site check cannot compare.
- **Not affected:** the `MID$` statement with such a target changes nothing (`sub_mid` returns while an error is
  pending), and `READ`, `INPUT #` and an assignment skip their store.
- **Answered (user, 2026-10-10, `DECISIONS.md`):** keep QB64pe's way. No row in `DIVERGENCES.md`; the behaviour
  stays listed in `SOMEDAY.md` "QB64pe behaviours to review".
