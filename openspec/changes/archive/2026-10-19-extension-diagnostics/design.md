# Design: extension-diagnostics

## Context

See `proposal.md` for why. What is there now:

- `vscode\src\diagnostics.ts` checks a saved document with `qb64pe -z` through the run queue (`compiler\qb64pe.ts`,
  `parseQb64peOutput.ts`) into the collection `qb64pe`, cleared on edit and close. `build.ts` publishes a build's
  messages through the same class.
- `vscode\src\languageServer.ts` starts `qb64rust lsp` when the binary is found and the workspace is trusted; the
  language client owns the collection `qb64rust`.
- The server (`crates\lsp\src\server.rs`) keeps the open documents, parses a changed program after a 100 ms
  debounce on one worker thread (`start_worker`, the compiler's stack size, a panic caught per job), and publishes
  per file (`Server::parsed`, with `published` recording which program owns each file's diagnostics).
  `analysis::analyze` reads the program from the open documents first, then the disk, in their encodings, and
  maps byte columns to UTF-16 columns (`FileInfo::columns`).
- `crates\lsp` depends on `syntax` and `base` only (D1 of `m2-language-server`), so that the server could not
  reach `sema` by accident while `sema` rejected most programs.
- The CLI's front end is `qb64rust_driver::frontend_with` (parse, `sema::check_with`) followed in `main.rs` by the
  two checks that read the disk, `check_exe_icon` and `check_embedded_files`; it prints with `Frontend::render`
  (sorted by file and offset, `-w` for warnings).

## Goals / Non-Goals

**Goals:**

- One function makes the CLI's diagnostics and the check's, so they cannot drift apart.
- The live parse stays as it is: parser only, after every change.
- A user without `qb64rust` keeps today's check on save.

**Non-Goals:**

- `DECLARE LIBRARY`'s files: they are looked for from the clone, as for a build (`library::resolve`). The check
  does not look for them; a build reports them as now.
- Build, Run and Format through the new compiler; a build's errors stay `qb64pe`'s.
- A check after every change (`sema` as the user types). The request is there for it later; this change sends it
  on save only.
- A machine-readable output of the CLI.

## Decisions

### D1. The language server runs the check, not a CLI run from the extension

On save the extension sends `qb64rust/check { uri }`; the server runs the front end on its worker and publishes.
**Alternative:** run `qb64rust <file> -w --dump typed` (or a new `--message-format json`) from the extension, as
for `qb64pe -z`. Rejected: the server already reads included files from the open documents in their encodings
and maps byte columns to the editor's; a CLI run would read only the disk and would need that mapping again in
TypeScript, for each encoding the server knows. The server is also already running, so no process starts per save.

### D2. The driver hands the server its front end; `qb64rust-lsp` still does not depend on `sema`

`crates\driver\src\lib.rs` gets `check_program(map: &SourceMap, parsed: &ParsedProgram, main_path: Option<&Path>)
-> (Program, Diagnostics)`: `sema::check_with` (folding as `frontend_with` decides it), then the `$EXEICON` and
`$EMBED` checks when `main_path` is given. The parse diagnostics come first, as in `frontend_with`.
`frontend_with` calls it with no path (as now: the disk checks are the CLI's step), and `main.rs` no longer calls
`check_exe_icon` and `check_embedded_files` after it but passes the input path. The two checks take
`&Program, &mut Diagnostics` instead of `&mut Frontend`.

`crates\lsp` gets a type `pub type Check = fn(&SourceMap, &ParsedProgram, Option<&Path>) -> Diagnostics;`,
taken by `run_stdio` and `serve` and kept by the server. `main.rs`'s `lsp` passes a function of the driver,
`check_for_server`, that calls `check_program` and drops the `Program`. **Alternative:** `qb64rust-lsp` depends on
`sema` (and the disk checks move into it or into `sema`). Rejected: the disk checks are the driver's (`sema`
reads no file), and the injected function makes it certain that the server's list is the CLI's. **Alternative:**
`lsp` depends on `driver`. Not possible: `driver` depends on `lsp` for the subcommand.

### D3. A check is a job on the parse worker

`server.rs`'s `Job` gets a field `check: Option<RequestId>`. For a check, the worker parses with `analyze` and
then runs the `Check` function over the analysis's map and parse, with the main file's path (`uri::to_path`; none
for an `untitled:` document, so no disk check), inside the same `catch_unwind`. Its result comes back as
`Event::Checked { key, seq, id, result: Result<(Analysis, Diagnostics), String> }`. One worker keeps one program
at a time: a check and a parse never run side by side, and jobs end in the order they were sent, so a parse
after an edit always publishes after the check before it.

`Server` keeps `checks: HashMap<String, (u64, RequestId)>`, the last check sent for each program. A new request
for the same program answers the older one at once with `ContentModified` and replaces it; when a result comes
whose number is not the last, it is dropped. A `$/cancelRequest` for a check removes it and answers
`RequestCancelled`; its result is dropped when it comes. The worker cannot stop a check under way; the extension's
timeout only stops waiting for it (Risks).

### D4. Publishing a check

`Analysis::diagnostics` becomes `Analysis::lsp_diagnostics(&self, list: &[Diagnostic])`, which both paths call:
a parse with the trees' diagnostics, a check with the check's whole list (sorted by file and offset, as
`Frontend::render`; the "not supported yet" marker and the `QB64pe:` line as now; warnings with severity
warning). Same per-file publishing and ownership as `Server::parsed` (factored into one method), so files the
program no longer covers are released. The check's analysis replaces the program's in `analyses`, as a parse's
would. The answer is `{ "errors": n, "warnings": n }`, counted over the list as `Diagnostics::error_count` counts
(the cap included: the list is the CLI's).

A check's diagnostics need no clearing of their own: the next parse of the program, after a change, publishes
the parser's alone for every file it covers (spec, "Check result until the next parse").

### D5. The extension chooses the server when it runs

`protocol.ts` gets `CHECK = "qb64rust/check"` and the types `CheckParams`, `CheckResult`. `LanguageServer`
gets `check(uri, timeoutMs): Promise<CheckResult | undefined>`: `sendRequest` with a `CancellationTokenSource`
cancelled after `qb64rust.checkTimeoutSeconds` or by a newer check of the same uri; `undefined` when it is
cancelled, superseded or the server stops. `Diagnostics` is given the `LanguageServer`:

- On save (`checkOnSave`, trusted workspace): the server's check if `server.running`, otherwise `qb64pe -z` when
  QB64pe is found, as now. The save condition no longer requires QB64pe to be found when the server runs.
- `qb64rust.check`: the same choice; only the `qb64pe -z` path asks for the compiler (`status.require()`).
  A clean document has no save event, so the command sends the request itself.
- `lastCheck` keeps the promise of either path, for the tests.

**Alternative:** the server checks on `didSave` itself (`textDocumentSync.save`). Rejected: `checkOnSave` is
the extension's setting per resource, and the command needs a request anyway; one path is simpler.

### D6. `DECISIONS.md` row

The server's boundary of D1 of `m2-language-server` ("parser only") is revised: the live parse stays parser only,
and the server runs the whole front end for a check, given by the driver. A row in `DECISIONS.md` (task 1.1).

## Risks / Trade-offs

- [A long check holds the worker, so a parse after an edit waits for it] → the front end of Civil War Strategy, the
  largest program in hand, is measured in task 2.5; if a check takes more than about a second, it is noted in
  `vscode\README.md`. A second worker for checks is left out: two front ends at once would need the open
  documents twice and gain little at save frequency.
- [A check the extension stopped waiting for still runs to its end] → its result is dropped by number; the next
  job waits for it. A front end that does not end would be a bug of the compiler (as for the CLI).
- [Check and CLI differ in the way a file is read]: the server reads open documents in their encodings and
  included files from the disk in the default encoding, the CLI reads bytes → for a saved CP437 file the bytes are
  the same; the equality test (task 2.4) runs over the front-end tests' `check-fail` programs and the disk checks.
- [The server's check is shown where `qb64pe -z` would have found more, or other, errors] → by the corpus, the
  new compiler rejects what the old one rejects (`known_false_errors.list` empty); the wording note gives the old
  text. Build still runs `qb64pe.exe` and shows its errors.

## Migration Plan

None: the extension's settings stay, and without `qb64rust` nothing changes.
