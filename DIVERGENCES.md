# Divergence register

Every intentional difference between QB64Rust and the observed behaviour of the old compiler (QB64pe 4.7.0,
`16f629784e`, Windows). A row is added only when a difference has been **decided**; open choices stay in
`study\00` §6 until then. The numeric rules themselves are the spec `openspec\specs\language\numeric-semantics`.

"Default" is the old compiler's normal build; "`-O2`" is its build with "optimize C++" on
(`-f:OptimizeCppProgram=true`). Where they behave the same only one value is given.

| Id | Behaviour | Old compiler | QB64Rust | Reason | Decided | Pinned by |
|---|---|---|---|---|---|---|
| D-001 | LONG (and INTEGER, computed in 32 bits) overflow in `+ - *` | Default: wraps (`2147483647 + 1` = `-2147483648`, `x + 1 > x` false). `-O2`: undefined behaviour in the generated C++; `x + 1 > x` is **true** and `PRINT x + 1` prints ` 2147483648 ` (`verification\v11_wrap_o2.out.txt`, `.O2.out.txt`) | Always wraps in two's complement, in generated code (`-fwrapv`) and in constant folding | Equals the default build; makes `-O2` irrelevant to results | 2026-10-03, panel (`study\16` §8) | `tests\corpus\verification\v11_wrap_o2`, `tests\corpus\slice\s02_integer_wrap` |
| D-002 | `_INTEGER64` overflow in `+ - *` | Default: wraps (`x + 1 > x` false). `-O2`: `x + 1 > x` is **true**; the printed value still wraps (`verification\v12_wrap_int64.out.txt`, `.O2.out.txt`) | Always wraps, like D-001 | Same exposure as LONG; same reason | 2026-10-03, user (`CLAUDE.md`, `study\16` §8) | `tests\corpus\verification\v12_wrap_int64`, `tests\corpus\slice\s02_integer_wrap` |

Both rows differ from the old compiler only in its `-O2` build; the corpus is recorded with the default build, so
these programs pass unchanged.
