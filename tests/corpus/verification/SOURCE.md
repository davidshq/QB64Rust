# Source of these programs

Copies of programs from this repo's `verification\` folder kept as corpus tests: `v11` by the panel's decision (`study\16` §8), `v12` to answer that decision's
open question.

| Program | Copied from | Why |
|---|---|---|
| `v11_wrap_o2.bas` | `verification\v11_wrap_o2.bas` (2026-10-03, byte-identical) | LONG and INTEGER overflow. The old compiler's default build wraps, its `-O2` build does not (`study\16` §8: LONG overflow wraps). Expected on the `-O2` list in `tests\corpus\README.md`. |
| `v12_wrap_int64.bas` | `verification\v12_wrap_int64.bas` (2026-10-03, byte-identical) | `_INTEGER64` overflow, the open question of `study\16` §8 (the `runtime_comparison` group has no `_INTEGER64` program). On the `-O2` list. |

The comments in `v11_wrap_o2.bas` and `v12_wrap_int64.bas` mention `run.sh` and the `.O2.out.txt` files; those refer to the originals in
`verification\`.
