# Tasks

Standing rules: CLAUDE.md. What this change does differently: the facts it relies on beyond the measurement of
`m5-debug-build-symbols` are measured against the real `lldb-dap` by `#[ignore]`d CLI tests (task 3) before the
rewrite that uses each; what they show is written into `design.md` under a new section "Measured".

## 1. The `next` hang (design D1)

- [x] 1.1 Reproduce: `debug_next_steps_by_line` in a loop beside the slice list at `--jobs 8`, with `lldb`'s
  step log written when `QB64RUST_LLDB_LOG` names a file (`support::lldb`), and the stuck program's stacks taken
  after the deadline (`thread backtrace all` from the clone's `lldb` attached to it). Verify: either a hang caught
  with its log and stacks, or the bounded try (one hour of machine time) ended with none.
- [x] 1.2 Act on it as design D1 says: the stepping setting in the proxy's `launch` and the CLI tests, or another
  remedy, or the "Still to watch" line of `STATUS.md` kept with what was tried. A decision goes to `DECISIONS.md`
  if one is taken. Verify: the finding written into `design.md`, "Measured"; with a remedy, the loop of 1.1 runs
  without a hang for as long as it took to hang before.

## 2. The crate and the passthrough (D2, D12)

- [x] 2.1 `crates\dap` (`qb64rust-dap`, workspace member and dependency; `base`, `serde_json`): `wire.rs` with
  unit tests (one message, two in one buffer, a body split over reads, a bad header). Verify: `cargo test -p
  qb64rust-dap`.
- [x] 2.2 `proxy.rs`: `serve` over generic readers and writers, the two reader threads and the channel, `seq`
  renumbering both ways with reverse requests, `Proxy::call` with the queue; `run_stdio(lldb_dap)` starting the
  backend, and the `initialize` error when it cannot. Tier 1 `dap\tests\proxy.rs` with a scripted backend: a
  request and its response pass through with `seq`s mapped; an event passes; a reverse request and its answer
  map back; `initialize` adds the filter and `supportsGotoTargetsRequest`; a missing `lldb-dap` gives the error
  naming its path. Verify: `cargo test -p qb64rust-dap`.
- [x] 2.3 `driver\src\main.rs` `dap` (only `--qb64pe-root`; `dap.bas` still compiles); usage text. CLI tests:
  `dap_rejects_other_options` (tier 1), `dap_initialize_and_disconnect` (`--ignored`: the real `lldb-dap`
  answers, exit code 0) with the DAP client of `driver\tests\support\dap.rs`. Verify: both pass.

## 3. Measured against `lldb-dap` (design "Risks")

- [x] 3.1 Ignored CLI tests through the passthrough of 2.2, on debug builds of the programs of
  `m5-debug-build-symbols`'s tests: `readMemory` at an address from `evaluate` of `(unsigned long long)<c_name>`;
  `writeMemory` of a `LONG` read back by the program; `dataBreakpointInfo` of an `lldb-dap` variable gives the
  `dataId` form D8 assumes, and an address `dataId` of ours stops on a write; `` `thread jump `` through a `repl`
  `evaluate` moves the next line; a function breakpoint on `QBMAIN` stops before the first statement; a `pause` of
  a busy loop, and on which thread; `qbs_set` called through `evaluate` changes a `STRING`. Verify: each test
  passes, and its result is written into `design.md`, "Measured"; where a result differs from D4, D7 or D8, the
  decision is amended there before task 4 uses it.

## 4. The rewrites (D3–D11)

- [x] 4.1 `symbols.rs`: `Symbols::read`, the indices; the tier 1 test that parses every `debug_info_*` snapshot
  of `crates\driver\tests\snapshots\`; errors for version 2 and a missing field. Verify: `cargo test -p
  qb64rust-dap`.
- [x] 4.2 `launch.rs` and the own breakpoints of `breakpoints.rs` (D4, D9): the refusal without a symbol file, the
  backend's `launch` arguments, stop on entry, the union of function breakpoints and the filtering of responses
  and events. Scripted-backend tests per rule. Verify: `cargo test -p qb64rust-dap`.
- [x] 4.3 `stack.rs` (D5, D11): BASIC frames, names, paging, the program's thread, `stopped` on that thread.
  Scripted tests: a SUB under the main module; a handler above `evnt` above the raising line; a pause on another
  thread. Verify: `cargo test -p qb64rust-dap`.
- [x] 4.4 `step.rs` (D6): the table of rules and the bound of 200. Scripted tests: the stop sequences of the
  measurement (step into a SUB, into a FUNCTION of an included file through `data1.txt`, `stepOut`, `next` past
  `RESUME`) each give one client `stopped` at the spec's line; a breakpoint during the steps ends them. Verify:
  `cargo test -p qb64rust-dap`.
- [x] 4.5 `vars.rs` and `format.rs` (D7): scopes, each reach, strings with `CHR$`, arrays paged, user types,
  "not allocated yet", `_BIT`/`_MEM` "not shown yet". Unit tests of `format.rs` per type (`_FLOAT` from its bytes,
  `_UNSIGNED`, `_OFFSET`); scripted tests of `variables`. Verify: `cargo test -p qb64rust-dap`.
- [x] 4.6 `eval.rs` (D8): the parser, lookup order and suffixes, display suffixes and `format.hex`, `repl`
  passthrough; set value with ranges and `STRING * n`, `STRING` as task 3.1 found; data breakpoints;
  `gotoTargets`/`goto`. Unit tests of the parser; scripted tests per request. Verify: `cargo test -p qb64rust-dap`.
- [x] 4.7 `errors.rs` and the error stop (D10): the table, its tier 1 test against
  `libqb\internal\c\libqb\src\error_handle.cpp`, the filter, `exceptionInfo`. Verify: `cargo test -p qb64rust-dap`.

## 5. End to end

- [x] 5.1 Ignored CLI tests through `qb64rust dap` and the real `lldb-dap`, one per requirement of
  `editor/debug-adapter` that runs a program: stop on entry; a breakpoint in a SUB with the stack and both scopes
  (each kind of variable of the spec's scenario); `stepIn`/`stepOut`/`next` on the measured programs; evaluate
  with `,h`; set a `LONG` and an out-of-range `INTEGER`; a data breakpoint; `goto`; the error stop with and
  without the filter; `pause` of `DO: x = x + 1: LOOP`; `disconnect` ends the program. Verify: the tests pass,
  run alone and in the whole `--ignored` run.
- [x] 5.2 `crates\README.md`: the crate map row for `dap`, `qb64rust dap` under "Building and running", the tests.
  `SOMEDAY.md`: rows for values formatted as `PRINT` would, `_BIT` and `_MEM` values, `_FLOAT` shown through
  `f64`, a BASIC expression evaluator, and a `STRING` set value if 3.1 left it out. Verify: the rows exist.

## 6. Close

- [x] 6.1 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then
  `tools\close\close.py --check` passes. Then `STATUS.md`'s "Next" names the next change of M5 (the extension's
  `debuggers` contribution), and `python tools\testtime\test_time.py`. Verify: `--check` passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
