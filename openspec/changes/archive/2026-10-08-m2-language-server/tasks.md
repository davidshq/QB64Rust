# Tasks

Order: 1 (facts and dependencies), 2 (the server's core), 3 (features), 4 (the extension), 5 (CI and documents).
Each group ends with `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`
and, from group 4, `npm run lint`, `npm run test:unit`, `npm run test:integration`; nothing is committed with a red
step (`CLAUDE.md` rule 1: the user stages and commits).

## 1. Facts and dependencies (design D1, D3)

- [x] 1.1 Check what VS Code's language client negotiates for `positionEncoding` (the `vscode-languageclient`
  version to be used, LSP 3.17 `general.positionEncodings`) by reading its source or running a probe server;
  record the answer in `design.md` D3 and in the `editor/language-server` spec ("Positions").
  Done 2026-10-08: `vscode-languageclient` 10.1.2 offers `['utf-16']` only and throws on any other answer; the
  server speaks UTF-16 (D3, spec "Positions").
- [x] 1.2 Add `lsp-server` and `lsp-types` to the workspace dependencies and `deny.toml` to the root; `cargo deny
  check` clean locally; the step added to `rust.yml`. Verify: `cargo build` and `cargo deny check` succeed;
  `Cargo.lock` updated; `crates\README.md` "Dependencies" names the two crates and `cargo-deny`.
  Done 2026-10-08: `lsp-server` 0.10, `lsp-types` 0.97 (built and tested on Rust 1.88, and again on 1.99.0 after
  the toolchain bump of the same day); `deny.toml` allows MIT,
  Apache-2.0 and Unicode-3.0 (`unicode-ident`), ignores the workspace's own unlicensed crates; `cargo deny check`:
  advisories, bans, licenses, sources ok. `rust.yml` has a job `deny` (`EmbarkStudios/cargo-deny-action@v2`).
- [x] 1.3 Measure once more with the release build what the server will do per change: `parse` only (no `sema`)
  of `qb64pe.bas` with includes, of the largest corpus program, and of a 1,000-line program with a syntax error on
  every tenth line; record in `study\00` §5 (one line) so that D5's debounce has a number behind it.
  Done 2026-10-08: 111 ms, 0.2 ms, 1.8 ms (best of 5; `study\00` §5).

## 2. The server's core (design D1, D2, D3, D5)

- [x] 2.1 Create `crates\lsp` (`qb64rust-lsp`, depends on `base`, `syntax`, `lsp-server`, `lsp-types`): the
  message loop, `initialize` with the capabilities (text sync full, document symbols, folding ranges, definition),
  `shutdown`/`exit`; `qb64rust lsp` in `main.rs` dispatching to it. Verify: a test through
  `Connection::memory()` initializes and shuts down; `qb64rust lsp < /dev/null` exits cleanly.
  Done 2026-10-08: `server.rs`, `lib.rs`; tests `initialize_and_exit`, `exit_without_shutdown_is_an_error`, and the
  CLI tests `language_server` (real pipes, exit code 0) and `lsp_named_program`. With stdin closed at once the
  binary ends without a hang or panic, with exit code 1 and "disconnected channel" on stderr (no `exit` after
  `shutdown`, so 1 by the protocol). The crate also depends on `serde_json`.
- [x] 2.2 The document store: open documents with text, version and encoding; the `qb64rust/documentEncoding`
  notification; `initializationOptions` (default encoding, include root). Verify: unit tests.
  Done 2026-10-08 (`server.rs`): tests `utf8_column_after_a_multi_byte_character` (notification before open),
  `encoding_change_reparses` (after open, `encoding` option), `include_root_from_the_options`.
- [x] 2.3 Encoding tables and the position map: the single-byte tables (`cp437` first, then the encodings VS Code
  lists that QB64 users meet), UTF-8 as is, `?` for an unencodable character; `Span` to `Range` and `Position` to
  byte offset, identity for single-byte encodings, a per-line table otherwise. Verify: unit tests with a CP437
  document holding bytes 128–255 and a UTF-8 document with a multi-byte character before an error; the table for
  `cp437` checked against the bytes the extension's `iconv` writes for the same text (a recorded fixture).
  Done 2026-10-08: ten tables generated from `iconv-lite` 0.7.3 (`tools\encodings\gen_tables.js` →
  `encoding_tables.rs`, fixtures `cp437_all_bytes_utf8.bin` and `cp437_unencodable.bin`). Measured: `iconv-lite`
  writes one `?` per UTF-16 unit (an emoji gives `??`), so the server does too (spec and D3 updated). Unit tests in
  `encoding.rs`; protocol tests `cp437_bytes_and_columns`, `utf8_column_after_a_multi_byte_character`.
- [x] 2.4 `DocLoader` implementing `Loader` (open document first, then the disk by the old compiler's lookup; same
  canonical path, same `FileId`); the program model (which open documents a parse covered); publish diagnostics per
  file from the parser's `Diagnostics`; reparse the including programs on a change. Verify: tests with an include
  on disk, an include shadowed by an open document, an include edited on its own; diagnostics under the include's
  own uri.
  Done 2026-10-08 (`analysis.rs`; files keyed by their normalised absolute path, D "As built"): tests
  `error_in_an_included_file_on_disk`, `unsaved_include_and_its_includer`, `closing_clears_the_diagnostics`,
  `definition_across_included_files`. The include name rule is now `syntax::include_name`, shared with the driver.
- [x] 2.5 Debounce and the worker thread (`driver::STACK_SIZE`); `$/cancelRequest` answered with
  `RequestCancelled`. Verify: a test that sends three changes within the debounce and sees one parse; a cancelled
  request gets the error reply.
  Done 2026-10-08: `STACK_SIZE` moved to `base` (re-exported by `driver`); tests `burst_of_edits_parses_once`,
  `cancelled_request`, `request_for_a_document_not_open`.

## 3. Features (design D4)

- [x] 3.1 Document symbols: `ProcDef`, `TypeBlock` with fields, `LabelDef` (top level and inside procedures).
  Verify: snapshot test on a fixture with every kind; tier-1 walk over every corpus and upstream program (no panic,
  ranges nested).
  Done 2026-10-08 (`symbols.rs`): snapshot `symbols_of_every_kind` on `tests\fixtures\outline.bas`,
  `outline_of_spec_scenario`; `walk.rs` over 696 programs (1.6 s).
- [x] 3.2 Folding ranges: every block kind, `CaseClause`, `$IF` regions, comment runs. Verify: snapshot test;
  the tier-1 walk.
  Done 2026-10-08 (`folding.rs`; also `DECLARE LIBRARY` and `DEF FN`): snapshot `folding_of_every_kind`,
  `do_loop_folds_to_the_line_before_loop`, the walk.
- [x] 3.3 Go to definition for procedures (calls with and without `CALL`, function calls, `DECLARE` headers point
  to the definition) and labels (`GOTO`, `GOSUB`, `ON … GOTO/GOSUB`, `ON ERROR GOTO`, `RESUME`, `RETURN label`),
  within the right body, across included files. Verify: tests for each form, a call before the definition, a
  label of the same name in main and in a procedure, a name with no definition (empty result).
  Done 2026-10-08 (`definition.rs`; also line numbers and `RESTORE`): `definition_of_each_form`,
  `definition_at_the_end_of_a_name`, `on_error_in_a_procedure_names_the_main_module_label`,
  `definition_across_included_files`, protocol test `requests_on_a_program`; the walk asks at every name of the
  programs under 50 KB.
- [x] 3.4 Add to `ast.rs` any accessor the walks needed (listed here when done), with its unit test.
  Done 2026-10-08: none was needed (the walks use `ProcDef`, `ProcHeader`, `TypeBlock`, `TypeField`, `FieldName`,
  `LabelDef`, `LineNumber`, the jump statements' `target()`, `OnJumpStmt::targets`, `ImplicitGoto::number`,
  `MetaStmt::token` with `pp::directive`, and each block's closer). `base::SourceFile` gained `line_content`
  (unit test `line_content_without_line_end`).

## 4. The extension (design D6, D7)

- [x] 4.1 `vscode-languageclient` dependency; `languageServer.ts`: find the binary (`qb64rust.path`, else
  `PATH`), start the client on stdio with `initializationOptions`, send `qb64rust/documentEncoding` on open and
  on an encoding change, stop on deactivate; settings `qb64rust.path` and `qb64rust.includeRoot` in
  `package.json`; the language status item shows `qb64rust` found/not found. Verify: unit tests for discovery and
  the notification payload.
  Done 2026-10-08: `vscode-languageclient` ^10.1.2 (`tsconfig.json` `module` `node16` for its `exports` map);
  `src\languageServer.ts`, `src\server\protocol.ts`; `findCompiler` takes the program to find (`QB64PE`,
  `QB64RUST`). Unit tests: 46 (discovery of `qb64rust`, `serverProtocol.test.ts`).
- [x] 4.2 Diagnostics: the client's collection with source `qb64rust`; the M1 collection labelled `qb64pe`;
  neither clears the other. Verify: integration test (edit without save: a `qb64rust` error appears; save: the
  `qb64pe` one appears beside it; edit: the `qb64pe` one goes, the `qb64rust` one follows the fix).
  Done 2026-10-08: integration test "syntax errors as you type, beside the old compiler's on save"; M1's tests read
  source `qb64pe` only (`qb64peDiagnostics`).
- [x] 4.3 Integration tests for the outline, a `DO` loop folding range and go to definition on `CALL` and `GOTO`,
  with `QB64RUST_TEST_QB64RUST` naming the binary; the suite skips these when it is unset and says so.
  Done 2026-10-08 (`languageServer.test.ts`): 42 passing with the binary on VS Code 1.141 and on 1.100; without it
  37 passing, 5 pending, with "skipped: QB64RUST_TEST_QB64RUST is not set".
- [x] 4.4 `vscode\README.md` (what the server gives, the two settings, the two diagnostic sources) and
  `DEVELOPMENT.md` (building the binary, running the suite with it).
  Done 2026-10-08.

## 5. CI and records

- [ ] 5.1 `vscode-extension.yml`: build the Rust binary (release, with `Swatinem/rust-cache`) and pass it to both
  integration runs; the workflow's `paths` gain `crates/**` and `Cargo.*`. Verify: green on GitHub.
  Workflow changed 2026-10-08 (also `rust-toolchain.toml` in `paths`); not yet run on GitHub. Still open when the
  change was archived (2026-10-08, at the user's request); the first push checks it (`STATUS.md`).
- [x] 5.2 `crates\README.md` (the `lsp` crate, the subcommand, the dependency section), `STATUS.md` (step 7 done;
  what the server does and does not do), `ROADMAP.md` (the language server ticked), `study\00` §5 (the
  measurements of 1.3). Archive the change.
  Documents done 2026-10-08; archived 2026-10-08 with 5.1 still open.

## 6. Review fixes (2026-10-08)

- [x] 6.1 Closing one includer (or a program no longer including a file) cleared an include's diagnostics that
  another open program still covers: they are now left to that program, which is parsed again at once
  (`Server::release`). Test `closing_one_includer_keeps_the_others_diagnostics` (fails without the fix).
- [x] 6.2 After a parse that panicked, a request on that document waited for a parse that never came: it now waits
  only while a parse is pending or running, else gets an empty answer. Unit test
  `request_after_a_failed_parse_is_answered` (`server.rs`).
- [x] 6.3 The extension: a restart overtaken by a later one (or by the dispose) no longer sets the state or keeps
  a client; a relative `qb64rust.includeRoot` without a workspace folder is ignored (unit test). The disk not
  being watched is recorded (design "As built", `SOMEDAY.md`).
