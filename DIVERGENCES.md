# Divergence register

Every intentional difference between QB64Rust and the observed behaviour of the old compiler (QB64pe 4.7.0,
`16f629784e`, Windows). A row is added only when a difference has been **decided**; open choices stay in
`study\00` §6 until then. The numeric rules themselves are the spec `openspec\specs\language\numeric-semantics`.

"Default" is the old compiler's normal build; "`-O2`" is its build with "optimize C++" on
(`-f:OptimizeCppProgram=true`). Where they behave the same only one value is given.

| Id | Behaviour | Old compiler | QB64Rust | Reason | Decided | Pinned by |
|---|---|---|---|---|---|---|
| D-001 | LONG (and INTEGER, computed in 32 bits) overflow in `+ - *` | Default: wraps (`2147483647 + 1` = `-2147483648`, `x + 1 > x` false). `-O2`: undefined behaviour in the generated C++; `x + 1 > x` is **true** and `PRINT x + 1` prints ` 2147483648 ` (`verification\v11_wrap_o2.out.txt`, `.O2.out.txt`) | Always wraps in two's complement, in generated code (`-fwrapv`) and in constant folding | Equals the default build; makes `-O2` irrelevant to results | 2026-10-03, panel (`study\16` §8) | `tests\corpus\verification\v11_wrap_o2`, `tests\corpus\slice\s02_integer_wrap` |
| D-002 | `_INTEGER64` overflow in `+ - *` | Default: wraps (`x + 1 > x` false). `-O2`: `x + 1 > x` is **true**; the printed value still wraps (`verification\v12_wrap_int64.out.txt`, `.O2.out.txt`) | Always wraps, like D-001 | Same exposure as LONG; same reason | 2026-10-03, user (`DECISIONS.md`, `study\16` §8) | `tests\corpus\verification\v12_wrap_int64`, `tests\corpus\slice\s02_integer_wrap` |
| D-003 | `RETURN label` with no `GOSUB` pending | Raises error 3, then decrements the `GOSUB` counter anyway (`qb64pe.bas` 9948), so it wraps below zero and the next `GOSUB` crashes the program (exit code 139; `verification\v17_d_return_label_none.out.txt`) | Raises error 3 and leaves the counter at 0; a later `GOSUB` and `RETURN` work | The old result is memory corruption, not behaviour a program can rely on | 2026-10-06, user (`m2-control-flow-slice` design D8) | `cpp` snapshot `tests\frontend\emit_gosub_cpp.bas`; CLI test `return_label_with_nothing_pending` (builds and runs; `#[ignore]`d, needs the clone) |
| D-004 | A store into a member of an array element whose index is out of range (`a(9).m = 5` for `DIM a(3) AS t`), with the program going on (`RESUME NEXT`, or `QB64PE_NOPROMPT=continue`) | Raises error 9 and **writes the value into element 0** (`a(0).m` becomes 5; also for `STRING` members): `array_check` returns 0 after raising and member stores are not guarded (`verification\v18_b_member_store.out.txt`) | Raises error 9 and stores nothing. The value is still evaluated before the index and the first error still wins, as measured (`a(9).m = ASC("")` reports 5). The same when an index raises with no error pending before it (`a(ASC("")).m = 6`: the old compiler writes `a(0).m`, `verification\v18_b_index_raises`), as an element store does. An index that raises while the value's error is pending cannot be told apart (the first error wins) and stores as the old compiler does | Writing an element the program did not name is wrong code; an element store `x(9) = 5` already writes nothing | 2026-10-07, user (`m2-arrays-and-types` design D6) | `cpp` and `ir` snapshots and a slice program (task 6.2 of `m2-arrays-and-types`) |

D-001 and D-002 differ from the old compiler only in its `-O2` build; the corpus is recorded with the default build,
so these programs pass unchanged. D-003 differs only after a program would have crashed. D-004 differs only after an
error 9 the program chose to ignore.
