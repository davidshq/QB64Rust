# Design

## Context

See `proposal.md` for motivation and the spec deltas for requirements. Background: the thin server's scope
(`study\23` §2.6), the design points and their reasons (`study\27` §6), the dependency policy (`crates\README.md`
"Dependencies"; `DECISIONS.md` 2026-10-07), the extension's design for the old compiler's diagnostics (the M1
change's `design.md`: one program owns the diagnostics of its include files, cleared on edit), and the measured
front-end speed (`study\27` §2).

Where the code stands (2026-10-08):
- `qb64rust_syntax::parse(&mut map, file, loader)` gives a `ParsedProgram` (one `Tree` per file per inclusion, each
  with its `Diagnostics`); `Loader::load(map, from, path)` is how included files arrive; `NoLoader` finds none;
  the driver's `FileLoader` reads the disk with the old compiler's lookup.
- `ast.rs` has typed accessors for every block (`ProcDef` with `header()`, `body()`, `end()`; `ProcHeader::name()`;
  `TypeBlock`, `IfBlock`, `ForBlock`, `DoBlock`, `WhileBlock`, `SelectBlock`, each with `header()` and `end()`),
  and `LabelDef::name()`.
- `SourceFile::line_col(offset)` gives a byte line and column; the extension maps the old compiler's byte
  columns to editor columns as the identity (`diagnostics.ts`, exact for single-byte encodings).
- The driver runs every stage on a 64 MiB thread (`with_stack`).

## Goals / Non-Goals

**Goals:**
- Syntax errors as the user types, from the same parser the compiler uses, at the right line and column.
- Outline, folding and go to definition for procedures and labels from the tree, with no `sema` involvement.
- Byte-exact parsing of what the editor shows, whatever the document's encoding.
- One binary, one setting, no new build step for a user who has `qb64rust.exe`.

**Non-Goals:**
- Anything `sema` knows (variables, types, "not supported yet"): a later change, when most statements compile.
- Incremental parsing (decided against) or a persistent index.
- Shipping the server with the extension (publishing).
- Protocol breadth beyond the four features: no completion, hover, rename, references, semantic tokens.

## Decisions

### D1. One binary, a subcommand, an `lsp` crate
`qb64rust lsp` starts the server on stdio; `main.rs` dispatches on the first argument and everything else stays
as it is. The protocol code lives in `crates\lsp` (package `qb64rust-lsp`), which depends on `syntax` and `base`
only, so that the server cannot reach `sema` by accident; `driver` depends on `lsp` for the subcommand. Dependencies
`lsp-server` (the transport and the message loop, synchronous, no async runtime) and `lsp-types` (the protocol
types); both are the policy's named choices. They are the workspace's first non-dev dependencies, so `cargo-deny`
is added (`deny.toml`: licences allowed MIT, Apache-2.0, and what `lsp-types`' tree needs; advisories) and runs in
`rust.yml`.

*Alternatives:* a separate binary (two artifacts to find and version: no); `tower-lsp` (async, a runtime for a
server that does one parse at a time: no); writing the JSON-RPC layer by hand (the policy allows a protocol crate
precisely to avoid this).

### D2. The document store and the loader
The server keeps each open document's current text and encoding (`didOpen`, `didChange` with full-text sync,
`didClose`). A `DocLoader` implements `Loader`: for an include path it applies the old compiler's lookup (the
including file's folder, then the include root; an absolute path as written) to find a file name, serves the open
document of that name if there is one (encoded to bytes, D3), and reads the disk otherwise; the same canonical path
gives the same `FileId`, so `$INCLUDEONCE` works. The include root comes from the client's `initializationOptions`
(`qb64rust.includeRoot`; empty means the folder of the `qb64rust` binary, the compiler's own rule).

Each open `.bas`, `.bi` or `.bm` document is parsed as a program of its own when it changes; a `.bi`/`.bm` parsed on
its own sees the preprocessor's defaults. A change to a document also reparses every open document whose last parse
included it (the server remembers each program's set of files). Diagnostics are published per file: a program's
parse gives diagnostics for its main file and its included files, and a file's diagnostics are those of the most
recent parse that covered it.

### D3. The encoding boundary
LSP delivers text (UTF-8 on the wire, a Rust `String` in the server) and takes positions in UTF-16 code units.
Checked (task 1.1, 2026-10-08, by reading `vscode-languageclient` 10.1.2, `lib\common\client.js`): the client
sends `general.positionEncodings: ['utf-16']` only, and throws "Unsupported position encoding" when the server's
`initialize` result names any other. So the server always speaks UTF-16 (it sends no `positionEncoding`, which
means UTF-16), and the per-line table below carries the UTF-8 case too. The server encodes a document's text to
bytes in its encoding before parsing, with its own 256-entry tables for the single-byte encodings VS Code offers
that QB64 programs use, and UTF-8 as the text's own bytes. The tables (`cp437`, `cp850`, `cp852`, `cp865`,
`cp866`, `windows1250`–`1252`, `iso88591`, `iso885915`) are generated from `iconv-lite`, the library VS Code and
the extension read and write these encodings with (`tools\encodings\gen_tables.js`, task 2.3). Each UTF-16 unit of
a character the encoding cannot hold becomes one `?`, as `iconv-lite` writes it on save (measured: an emoji gives
`??`); so in a single-byte encoding one UTF-16 unit is always one byte. The encoding is told by the extension: the default in
`initializationOptions`, and a notification `qb64rust/documentEncoding { uri, encoding }` on open and when the
document's encoding changes (VS Code's `TextDocument.encoding`, which the extension already reads for formatting).

Positions: the server keeps, per document, the line starts of the bytes (`SourceFile`'s line index) and, for a
document whose encoding is not single-byte, a per-line table from byte offset to UTF-16 offset. A byte `Span` maps
to an LSP `Range` through it; an LSP position maps back to a byte offset the same way. For single-byte encodings
the table is not built and the maps are the identity, as the extension already assumes for the old compiler; for
UTF-8 it is built only for lines with a byte above 127. An included file read from disk is mapped by the default
encoding (for UTF-8, by decoding its bytes; an invalid byte is one unit, as VS Code shows it as U+FFFD).

*Alternative:* taking a crate for encodings (`encoding_rs`): the policy's "no crate that takes source as `str`"
does not cover it (it would only produce the bytes), but the tables needed are small public data and the crate
would bring a large one; not taken.

### D4. Parser only, and what each feature reads
- **Diagnostics:** every `Tree`'s `Diagnostics` of the program, all of them syntax errors from the parser (the
  preprocessor's `$ERROR` among them); never `sema`'s. Severity error; source `qb64rust`. The parser marks a few
  of its own "not supported yet" (a generic syntax error at a BASIC word, `syntax_error`; an expression or block
  nested past its limit); since `m2-parser-breadth` every program the old compiler accepts parses without one, so
  they are published like the rest, the message starting "not supported yet: " as the compiler prints it. A file
  included twice reports each error once.
- **Document symbols:** a hierarchy of `ProcDef` (kind Function or Method, name from the header, range the whole
  node, selection range the name token), `TypeBlock` (Struct, with its fields as Field children), `LabelDef`
  (kind Key, at top level or inside the procedure it stands in). `DeclareStmt` headers are left out (they declare
  nothing in QB64pe).
- **Folding ranges:** one per `ProcDef`, `TypeBlock`, `IfBlock` (multi-line), `ForBlock`, `DoBlock`,
  `WhileBlock`, `SelectBlock` and `CaseClause`, from the header's line to the line before the block end's; a
  `$IF` region of the preprocessor likewise (`MetaStmt` from `$IF` to `$END IF`). A run of three or more comment
  lines folds as a comment region.
- **Go to definition:** at a position inside a name token, the server finds the token's enclosing statement. For a
  `CallStmt` name, a `GotoStmt`/`GosubStmt`/`OnJumpStmt`/`OnErrorStmt`/`ResumeStmt`/`ReturnStmt` target, a
  function-call `NameRef`, or a `ProcHeader` name, it resolves by name (upper case, suffix dropped for procedures,
  kept for labels as the old compiler keeps line numbers and names apart) over the program's trees: procedures to
  the first `ProcHeader` of that name in file order, labels to the `LabelDef` of that name in the same body
  (main module or the enclosing `ProcDef`). A name that resolves to nothing, or a variable name, gives no result
  (no error). If a `NameRef` cannot be told from a variable without `sema` (`x = f(1)` with no `SUB`/`FUNCTION`
  `f`), the server answers only when a procedure of that name exists.

### D5. Reparse and cancellation
Each `didChange` schedules a parse of the program after a short debounce (100 ms); a newer change for the same
document drops the pending one. Parsing runs on a worker thread with `driver::STACK_SIZE` of stack (the parser's
depth limits are tuned for it), one program at a time; requests (symbols, folding, definition) are answered from
the last completed parse of that document, or wait for the pending one when none has completed yet. The main loop
stays responsive to `$/cancelRequest` by answering a cancelled request with the `RequestCancelled` error. No
parse is interrupted midway: the measured worst case (0.6 s for `qb64pe.bas`) is below what a user notices for an
outline, and a normal program takes milliseconds.

### D6. The extension side
A new module `languageServer.ts` starts a `LanguageClient` (`vscode-languageclient`, stdio) when a `qb64rust`
binary is found: the setting `qb64rust.path` (full path; empty means `qb64rust` on `PATH`), checked as
`compilerPath` is for `qb64pe` (discovery, version, status). Without a binary nothing changes from M1 (highlighting,
static folding markers, old-compiler checks). The client's `initializationOptions` carry the default encoding
(the `[qb64rust]` `files.encoding`) and `qb64rust.includeRoot`; on `didOpen` and on a change of a document's
encoding the extension sends `qb64rust/documentEncoding`. Diagnostics from the server go into the client's own
collection (source `qb64rust`); the M1 `Diagnostics` class keeps its collection (source `qb64pe`, cleared on edit
as today). The language status item shows "qb64rust" beside "qb64pe" with the same found/not-found states.

Folding: when the client is running, VS Code merges the server's folding ranges with the static markers of
`language-configuration.json`; both agree on `SUB`/`FUNCTION`/`TYPE`, and the markers stay for the no-server
case. The outline view and go to definition come only from the server.

### D7. Tests
- **Rust, `crates\lsp\tests`:** the server driven in-process through `lsp_server::Connection::memory()`: initialize;
  open a document with an error and check the published diagnostic's range; change it and check the diagnostic
  goes; open a program with an include (through a loader over a temp folder, and over an open document that shadows
  the disk) and check the include's diagnostics are published under its own uri; symbols, folding and definition
  on a fixture that has every block kind, labels in main and in a procedure, a call before the definition; a CP437
  document with bytes above 127 and a UTF-8 document with a multi-byte character before the error, checking the
  columns; cancellation of a pending request.
- **Rust, tier 1:** the symbol and folding walks run over every corpus and upstream program (`inputs.rs` style):
  no panic, every range inside its file, every nested range inside its parent.
- **Extension, unit:** the encoding notification and the settings; **integration:** with `QB64RUST_TEST_QB64RUST`
  pointing at a built binary, open a fixture, see a `qb64rust` diagnostic appear on edit before any save and the
  `qb64pe` one on save, the outline symbols, a folding range for a `DO` loop (not in the static markers), and go
  to definition on a `CALL` and a `GOTO`.
- **CI:** `vscode-extension.yml` gains a `cargo build --release -p qb64rust-driver` step (with the Rust cache) and
  passes the binary to the integration runs; `rust.yml` gains `cargo deny check`.

### D8. What is deliberately not done
No `sema` in the server (see §Non-Goals); no persistent index; no workspace-wide symbol search (one program at a
time, the same model as compiling); no semantic highlighting; no formatting through the server (the old compiler's
`-y` stays; a new-compiler formatter is a later step).

## Risks / Trade-offs

- **The client's position encoding.** If VS Code's client speaks UTF-16 only, D3's per-line table carries the
  UTF-8 case; checked in task 1.1 before the mapping is written.
- **An include parsed alone.** A `.bi` edited on its own shows syntax errors under the preprocessor's defaults; a
  `$IF` in it may select a branch the main program would not. Accepted: the main program's parse, which the
  server also runs when the main file is open, publishes the right diagnostics for the include too, and the last
  parse that covers a file wins.
- **Two diagnostics for one syntax error** (the server's and, after a save, the old compiler's) until the next
  edit clears the old compiler's. Accepted (`study\27` §6 point 5); the sources tell them apart.
- **First dependencies.** `lsp-types` brings `serde`, `fluent-uri` and a few more; `cargo-deny` keeps the licences and
  advisories in view; `Cargo.lock` is committed already.

## As built (2026-10-08)

Details settled while applying, beyond the decisions above:
- **Shared pieces.** `STACK_SIZE` moved from `driver` to `base` (the server's worker needs it and `lsp` cannot depend
  on `driver`); the include name rule (`.\` dropped, UTF-8) is `syntax::include_name`, used by both loaders. The
  server's loader keys files by their absolute path with `.`/`..` resolved (lower case on Windows), not the
  canonical path: open documents may not exist on disk.
- **Encodings.** An encoding without a table (`utf16le`, `shiftjis`, ...) is read as UTF-8: the parse may see other
  bytes than the file, but positions stay right. `utf8bom` is UTF-8 (the editor's text has no BOM).
- **Order of parses.** When an include changes, its own parse is sent before its includers', so that the
  includer's parse, which sees the include as the program does, publishes the include's diagnostics last.
- **Closing a document** removes the diagnostics its parse published, except for a file still open (whose own
  parse publishes them again) or one another open program includes (that program is parsed again at once and
  publishes them); programs that included it are parsed again from disk. The same holds for a file a program
  stops including.
- **The disk is not watched.** An included file that is not open is read at each parse of its includer; a change
  on disk (another editor, a checkout) shows at the includer's next edit. Watching is in `SOMEDAY.md`.
- **Go to definition** also resolves line numbers (from `GOTO 10`, `GOSUB 10`, `IF … THEN 10`) and `RESTORE`
  targets (main module), and answers at the end of a name as well as inside it. `RESUME label` looks in its own
  body, `ON ERROR GOTO` in the main module (measured, `v14_on_error_sub_to_main`).
- **Folding** also covers `DECLARE LIBRARY` and `DEF FN` blocks; a `FOR` closed by an inner `NEXT j, i` ends on the
  line before it.
- **The extension** sends `qb64rust/documentEncoding` from its own open and change listeners (an encoding change
  can come as a change event without content changes, which the client's own sync skips); the server accepts it
  before or after `didOpen`. The server runs only in a trusted workspace. "qb64rust: not found" is shown with
  severity Information: the server is optional. `vscode-languageclient` 10 needs `"module": "node16"` in
  `tsconfig.json` (its `exports` map); the output stays CommonJS. The M1 collection is named `qb64pe`, the
  client's `qb64rust`; M1's integration tests read only source `qb64pe`. A restart overtaken by a later one (two
  settings changes in a row) leaves the state to the later one. A relative `qb64rust.includeRoot` is resolved
  against the workspace folder; without one it is ignored (the server would resolve it against VS Code's working
  directory).
- **`qb64rust lsp`** takes no other argument; stdout carries only the protocol, so the compile run's panic hook
  (which prints there) is not installed and a panic goes to stderr. A parse that panics is reported on stderr and
  the server goes on; requests waiting for that parse get an internal error, later ones an empty answer until the
  next change.

## Open Questions

- Whether the extension should offer to download a `qb64rust` binary when none is found: no, not before
  publishing; the status item says "qb64rust: not found" with the setting's name, as for `qb64pe`.
