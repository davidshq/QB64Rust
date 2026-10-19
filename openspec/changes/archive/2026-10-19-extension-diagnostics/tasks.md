# Tasks

Standing rules: `CLAUDE.md`.

## 1. Paperwork first

- [x] 1.1 `DECISIONS.md` row (design D6): the server runs the whole front end for a check, given by the driver; the
  live parse stays parser only; `qb64pe -z` on save only when the server does not run. Checked by reading it.
- [x] 1.2 The two stale lines of `libqb-copy` (user's request): `STATUS.md` "Next" item 1 names `libqb-copy` as
  done and archived (`89ec865`) and this change as the one under way; `ROADMAP.md` "After M3" ticks the copy of
  `internal\c`. Checked by reading both.

## 2. The front end as one function (design D2)

- [x] 2.1 `check_program` in `crates\driver\src\lib.rs`; `check_exe_icon` and `check_embedded_files` take
  `&Program, &mut Diagnostics`; `frontend_with` calls `check_program` with no path; `main.rs` passes the input path
  instead of calling the two checks. Checked by tier 1 (`cargo test`): the CLI tests of the missing icon and
  `$EMBED` file and the front-end tests pass unchanged.
- [x] 2.2 `check_for_server` in the driver (drops the `Program`); `qb64rust_lsp::Check`, taken by `run_stdio` and
  `serve`; `main.rs`'s `lsp` passes it. `crates\lsp\Cargo.toml` unchanged (no `sema`). Checked by
  `cargo tree -p qb64rust-lsp` naming no `qb64rust-sema`.

## 3. The check in the server (design D3, D4)

- [x] 3.1 `Analysis::lsp_diagnostics` over a list, used by the parse path (its snapshots in `crates\lsp\tests`
  unchanged) and the check path; the per-file publishing of `Server::parsed` factored into one method.
- [x] 3.2 The request `qb64rust/check`: `Job.check`, `Event::Checked`, `Server::checks`, `ContentModified` for a
  superseded check, `RequestCancelled` on cancel, `InvalidParams` for a document that is not open; the answer
  `{ errors, warnings }`. Protocol tests in `crates\lsp\tests\server.rs` with a stand-in `Check` (an error at each
  `BAD` word, a warning at each `WARN`): the spec's scenarios "Not an open document", "Error in an included file",
  "Edit after a check", "Two checks in a row", a cancelled check publishing nothing, and the answer's counts.
- [x] 3.3 The real front end through the server: a test in `crates\driver\tests` that serves an in-process
  connection with `check_for_server`, opens each `check-fail` program of `tests\frontend` (with its `inc\` files
  on disk), checks it, and compares the published diagnostics (file, line, column, message, severity) with
  `frontend_with` plus `check_program`'s rendering with warnings. Plus the spec's "Type error on check",
  "Several errors in one run", "Warning" (`$DEBUG`) and "Missing icon".
- [x] 3.4 Protocol scenario "Syntax diagnostics" / "Unsupported statement is no diagnostic" still holds: a
  document with `x$ = 1` gets no diagnostic from the live parse (test in `server.rs` with the stand-in, which is
  not called on a change).
- [x] 3.5 Time of a check of Civil War Strategy (`tests\programs\civil-war-strategy`) through the server, release
  build, measured once; if over about a second, a line in `vscode\README.md` (design, Risks). Measured
  2026-10-19 with a test written for it and removed after: 71–84 ms a check over three runs, a new server each
  (no diagnostic); no line needed.
- [x] 3.6 Main spec `openspec\specs\editor\language-server\spec.md` Purpose ("from the new compiler's parser
  alone") and `crates\lsp\src\lib.rs`'s doc comment say the check runs the whole front end; `crates\README.md`'s
  crate-map row of `lsp` and its dependency line (`lsp -> syntax, base: never sema`) say the front end comes from
  the driver.
- [x] 3.7 From the review of 2026-10-19 (pragmatic engineer): a check takes the place of a pending parse, so a
  check that is cancelled or fails makes that parse instead (`Server::checked`, the cancel; after a cancel, once
  the debounce is over, so that a newer check sent at once takes its place and the check's diagnostics do not
  flicker), and a request waits for a check under way as for a parse. Tests in `crates\lsp\tests\server.rs`:
  "cancelled check after an edit parses the edit", "failed check parses and answers the waiting requests" (both
  fail without the fix); "cancelled check publishes nothing", renamed "cancelled check publishes the parse
  only", now sees the parser's (empty) list. In the extension, a check that fails other than by cancellation is shown for
  the command `qb64rust.check` only, not on save (`LanguageServer.check` throws, `Diagnostics.check` shows it). No
  integration test for it (it would need a front end that panics on purpose in the binary). Rerun after it:
  `cargo test -p qb64rust-lsp --test server`, the driver's `check.rs`, fmt, clippy, `npm run lint`,
  `test:unit` and `test:integration` with `QB64RUST_TEST_QB64RUST`; not the slice, upstream or call-site
  runs (no program's executable changes), so `close.py --check` names the closing runs as of the earlier build.

## 4. The extension (design D5)

- [x] 4.1 `protocol.ts`: `CHECK`, `CheckParams`, `CheckResult`; `LanguageServer.check` with the timeout and a newer
  check cancelling the older. Unit test in `serverProtocol.test.ts` of whatever has no VS Code import (the
  params).
- [x] 4.2 `Diagnostics` takes the `LanguageServer` (`extension.ts`): on save and in `qb64rust.check`, the server's
  check when it runs, `qb64pe -z` otherwise; `lastCheck` for both. `npm run lint` and `npm run test:unit` pass.
- [x] 4.3 Integration tests in `languageServer.test.ts` (with `QB64RUST_TEST_QB64RUST`): the spec's "Error on save
  from the new compiler" (two errors with columns, no `qb64pe` entry, and the fake QB64pe not run), "Disabled",
  "Command without changes", "Fixed on save". The existing `diagnostics.test.ts` (no server) passes unchanged:
  "Error on save" through `qb64pe -z`. `npm test` passes, with `QB64RUST_TEST_QB64RUST` set to
  `target\release\qb64rust.exe` (so no test is pending for want of it).
- [x] 4.4 `vscode\README.md` "Problems on save": from `qb64rust` when it runs, `qb64pe -z` otherwise; the settings
  table's `checkOnSave` and `checkTimeoutSeconds` lines say both. `vscode\DEVELOPMENT.md` if it describes the
  check.

## 5. Close

- [x] 5.1 The closing runs: `tools\close\close.py --check` passes. No program's executable changes (nothing in
  `sema`'s rules, the IR or the emitter), so the pass cache skips them; no run without folding.
- [x] 5.2 `python tools\testtime\test_time.py`; `STATUS.md` "Next" (the second of the three "After M3" changes
  built; the `.vsix` next).
