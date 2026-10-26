# Tasks

Standing rules: CLAUDE.md. What this change does differently:

- Beside tier 1, it runs the extension's tests: `npm run test:unit`, `npm run lint` and `npm run test:integration`
  with `QB64RUST_TEST_QB64RUST` set (`vscode\DEVELOPMENT.md`).
- Facts measured in group 1 go into `design.md` under a new section "Measured".

## 1. Measured first (design "Risks", D1)

- [x] 1.1 Time `check_program` on Civil War Strategy (`tests\programs\civil-war-strategy\`) and on a slice program,
  in a debug and a release build. Measure the front end alone, and the front end after a check of the same text.
  Write the numbers into "Measured" and decide on the index's timing as the "Risks" line says. If the decision is
  to build the index after each parse, add a `DECISIONS.md` row.
  Verify: the numbers and the decision are in `design.md`.
- [x] 1.2 List what the index needs that `Program` does not give. For each entry this covers: its scope (the
  procedure a variable, constant or label belongs to, from `Storage` and `proc`); the bodies' spans; the suffix's
  length within a name's span; and a procedure's header as text. Add what is missing to `sema` (no change of
  behaviour; the `typed` snapshots prove it).
  Verify: `cargo test -p qb64rust-sema`; the list is in "Measured".

## 2. The index and its job (design D1, D2; spec "Semantic index until the next change")

- [x] 2.1 Add `crates\lsp\src\index.rs` (`SymbolIndex`, `Entry`, the lookup by position) and the `Index` hook in
  `Hooks`. Add `index_for_server` and `crates\driver\src\hover.rs` in the driver, wired into `server_hooks`. Add a
  `frontend` test mode or a snapshot of the index's dump over a few programs of `tests\frontend\`, to cover the
  details and scopes.
  Verify: `cargo test -p qb64rust-driver`; the snapshots read right.
- [x] 2.2 The index job in `server.rs`: one run per text, queued requests, `ContentModified` on a change, and the
  last index kept for completion. Add protocol tests in `crates\driver\tests\` (a new `language_features.rs`, with
  the fixture style of `check.rs`). They check two requests with one run (a hook that counts runs) and an edit
  while a request waits.
  Verify: `cargo test -p qb64rust-driver --test language_features`.

## 3. Hover, references and definition (design D3, D4; spec "Hover", "Find references", "Go to definition")

- [x] 3.1 Hover: symbols from `detail`, built-ins from `qb64rust-builtins` (`crates\lsp\src\hover.rs`), nothing
  otherwise; `hover_provider` in `capabilities()`. Add a test for each scenario of the spec.
  Verify: the hover tests pass.
- [x] 3.2 References, and definition from the index for variables, constants, types and members;
  `references_provider` in the capabilities. Add a test for each scenario of the spec. Add a tier 1 test that, for
  every procedure and label use in the corpus's slice programs, the parser's definition equals the index's.
  Verify: the tests pass; `cargo test` passes.

## 4. Rename (design D5; spec "Rename", "Rename refused")

- [x] 4.1 `crates\lsp\src\rename.rs`: `prepareRename`, the edit with the suffix kept, the five refusals, and the check
  of the edit through a second index. Add `rename_provider` with `prepare_provider` in the capabilities.
  Verify: `cargo test -p qb64rust-lsp`.
- [x] 4.2 Protocol tests: each scenario of the two requirements, a rename across an include, a rename refused by
  the check of the edit (a fixture where a use is not in the table, if one exists, or a test hook that drops a use),
  and the message about inactive `$IF` regions.
  Verify: `cargo test -p qb64rust-driver --test language_features`.
- [x] 4.3 A tier 1 test: for each slice program, rename every symbol that is not under the compiler root to a fresh
  name, and require each rename to be accepted (no error program) and to leave the same typed dump, names aside.
  Write its time in its header comment.
  Verify: `cargo test`; any symbol that fails is a bug fixed here, or the test's list of known misses names it with
  its reason.

## 5. Completion (design D6, D7; spec "Completion")

- [x] 5.1 `crates\lsp\src\completion.rs`: the scope at the cursor, the names, built-ins and keywords, members after
  `x.` for a variable of a user type, and the last index when none is ready; `completion_provider` in the
  capabilities. Add a test for each scenario, and one for members after a dot.
  Verify: the completion tests pass.

## 6. The extension (spec "Server start")

- [x] 6.1 Integration tests in `vscode\test\` with the server running: hover on a variable and a built-in,
  references across an include, a rename and a refused one, and completion inside a SUB. Add a "Language features"
  section to `vscode\README.md`, naming the `$IF` limit, and an entry to `CHANGELOG.md` under "Unreleased".
  Verify: `npm run test:integration` with `QB64RUST_TEST_QB64RUST` set; `npm run lint`.

## 7. Close

- [x] 7.1 Record the `DECISIONS.md` row of design D8.
  Verify: the row names `design.md` D1, D2 and D5.
- [x] 7.2 Do the closing runs: `python tools\close\close.py --plan`, running each command on its own, then
  `tools\close\close.py --check` passes. Then update `STATUS.md`'s "Next" (M5b's items are built; next is M6), and
  run `python tools\testtime\test_time.py`.
  Verify: `--check` passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`). At the archive, change the Purpose of
  `openspec\specs\editor\language-server\spec.md`: the server answers hover, references, rename, completion and
  definition from the whole front end, built once per text.
- Tell the user which files to stage, and suggest a commit message.
- At the merge into `main`: tick `ROADMAP.md` M5b's second item (working rule 11).
