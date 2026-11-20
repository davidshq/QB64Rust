# The wasm slice list

`slice.list` names 22 console programs that run for the wasm target (OpenSpec change `wasm-console`; spec
`testing/golden-corpus`, "The wasm slice list"). They are the short run on the host after a change that can touch
the wasm build: the compiler's crates, libqb's copy and its edits, the runner. The whole family, every
`$CONSOLE:ONLY` program of `tests\verification`, runs in the suites VM (`check wasm-verification --where vm`).

Each line is `verification/<name>` or `corpus/<group>/<name>`. Every program on the list must pass. The kinds the
spec names, and the programs that cover each:

| Kind | Programs |
|---|---|
| Numbers of each type | `v20_h_select`, `v20_b_result_types`, `v30_g_iif_min_max`, `v33_c12_system_float` (a `_FLOAT` to `SYSTEM`), corpus `slice/s01_literals` |
| Strings | `v20_c_string_edges`, `v30_b_using_strings`, `v22_b_funcs` |
| A file read and written | `v22_b_open`, `v38_b_file_dims`, `v30_j_field` |
| Standard input | `v22_e_input`, `v22_e_redo`, `v63_a6_line_input_crlf` |
| `ON ERROR` | `v63_e1_error_11_handler`, `v14_error_256` |
| A run-time error's exit status | `v02c_no_handler`, `v17_b_untrapped`; `SYSTEM 3`: `v33_c01_system_3` |
| `TIMER` | `v22_d_random`, `v59_a5_console_only` |
| `ON TIMER` | corpus `slice/s134_events_timer` |

None opens a window or a network port: the target builds console programs only and rejects the network words.

## Running it

```text
set QB64RUST_EMSDK_ROOT=..\emsdk
python tools\legacy_tests\wasm_slice.py --qb64 target\debug\qb64rust.exe --jobs 4
```

`wasm_slice.py` splits the list by suite and runs `run_legacy_tests.py --platform wasm` once for each
(`tools\legacy_tests\README.md`, "The wasm target").
