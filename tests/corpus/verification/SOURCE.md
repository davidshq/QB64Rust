# Source of these programs

Copies of programs from this repo's `verification\` folder that the panel decided to keep as corpus tests.

| Program | Copied from | Why |
|---|---|---|
| `v11_wrap_o2.bas` | `verification\v11_wrap_o2.bas` (2026-10-03, byte-identical) | LONG and INTEGER overflow. The old compiler's default build wraps, its `-O2` build does not (`study\16` §8: LONG overflow wraps). Expected on the `-O2` list in `tests\corpus\README.md`. |

The comments in `v11_wrap_o2.bas` mention `run.sh` and `v11_wrap_o2.O2.out.txt`; those refer to the original in
`verification\`.
