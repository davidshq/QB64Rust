# Proposal: extension-diagnostics

## Why

On save the extension checks a program with `qb64pe -z`: one error per run, a line and no column, and the old
compiler's wording. The new compiler now accepts every corpus program the old one accepts (M3 done,
`DECISIONS.md` 2026-10-18), so its whole front end can do this check, and do it better: every error in one run,
each at its column, in the new wording with the old one as a note (`DECISIONS.md` 2026-10-02, "new error
messages, with columns, multiple errors"). This is the second of the three changes of `ROADMAP.md` "After M3".
The language server was left at the parser in `m2-language-server` until "most statements compile"; that point
is past.

## What Changes

- The language server gets a request, `qb64rust/check`: it runs the program's whole front end (parse, `sema`, and
  the two disk checks the CLI makes, the `$EXEICON` file and the `$EMBED` files) over the open documents and the
  disk, as the live parse reads them, and publishes every error and warning it finds, with columns, source
  `qb64rust`. The diagnostics are the CLI's for the same files: one function of the driver makes both.
- The result of a check stands until the program's next parse after an edit, which publishes the parser's
  diagnostics alone again (as the `qb64pe` ones are cleared on edit today).
- The extension sends `qb64rust/check` on save (when `qb64rust.checkOnSave` is true) and for the command
  `qb64rust.check`, when the language server runs. Then `qb64pe -z` is not run on save.
- When the language server does not run (no `qb64rust` binary, an untrusted workspace, a failed start), the check
  on save stays `qb64pe -z` as now, so a user with only QB64pe loses nothing.
- Build, Run and Format still use `qb64pe.exe`; a build's errors still go to the `qb64pe` collection.
- Two stale lines from `libqb-copy`: `STATUS.md`'s "Next" item 1 still says it is to be archived, and
  `ROADMAP.md`'s box for the copy of `internal\c` is not ticked.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `editor/language-server`: a fourth kind of diagnostics run, the check: the request `qb64rust/check`, what it
  publishes (`sema`'s errors and warnings and the disk checks', with columns) and how long its result stands. The
  requirement "Syntax diagnostics" keeps the live parse parser-only.
- `editor/compiler-diagnostics`: the check on save and the command go to the language server when it runs, and
  to `qb64pe -z` only when it does not; the scenario "Two sources" applies to a build's errors, not to a save.

## Impact

- `crates\driver\src\lib.rs`: the front end's checks after the parse become one function over a source map and a
  parse (`sema` and the two disk checks), used by `frontend_with` and passed to the server.
- `crates\lsp`: the check request and its job on the worker thread; the server is given the check function by
  `main.rs`, so `qb64rust-lsp` still depends on `syntax` and `base` only.
- `crates\driver\src\main.rs`: `lsp` passes the function to `run_stdio`.
- `vscode\src\diagnostics.ts`, `languageServer.ts`, `server\protocol.ts`: the request, the choice between the
  server and `qb64pe -z`; their tests (unit, integration with `QB64RUST_TEST_QB64RUST`).
- `vscode\README.md`: "Problems on save".
- No change to the parser, `sema`'s rules, the IR, the emitter or any program's behaviour. No new dependency.
