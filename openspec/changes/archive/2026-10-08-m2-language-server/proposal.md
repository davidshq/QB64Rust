# Proposal

## Why

The M1 extension gets everything from the old compiler: diagnostics on save (`qb64pe -z`), formatting (`-y`),
build and run. The new compiler's front end now parses every program the old compiler accepts without a diagnostic
(`m2-parser-breadth`, 2026-10-07), and does so fast: the old compiler's own source with its 50 included files in
0.6 s, a normal program in a few milliseconds (`study\27` §2). That is enough for a thin language server, step 7
of `STATUS.md` "Next": live syntax errors, the document outline, folding ranges and go to definition for
procedures and labels, as decided in `study\23` §2.6 and detailed by `study\27` §6 (accepted 2026-10-08). The
server gives users something whatever `sema` supports, and makes the new compiler a part of the extension for the
first time.

## What Changes

- **A `qb64rust lsp` subcommand** speaking the Language Server Protocol over stdio, in the one binary; its logic
  in a new crate `lsp` (`driver -> lsp -> syntax`), on `lsp-server` and `lsp-types`: the first non-dev dependencies,
  so `cargo-deny` comes with them (`study\21` item 7).
- **Parser only.** The server publishes the parser's diagnostics (syntax errors) and nothing from `sema`; the
  outline and folding ranges come from the block nodes (`ProcDef`, `TypeBlock`, `IfBlock`, `ForBlock`, `DoBlock`,
  `WhileBlock`, `SelectBlock`) and labels; go to definition resolves a procedure or label name to its definition
  by a walk over the trees. A full reparse on every change, debounced and cancelled by a newer change (decided
  2026-10-04: no incremental analysis).
- **The encoding boundary.** The extension tells the server each document's encoding (CP437 by default); the
  server encodes the editor's text back to bytes before parsing and maps byte columns to the editor's UTF-16
  columns (the identity for single-byte encodings, a per-line table otherwise).
- **Included files** come from open documents first and the disk second, through the parser's `Loader` seam,
  with the include root from a setting (the old compiler's lookup: the including file's folder, then the root).
- **The extension** starts the server when `qb64rust.path` names a `qb64rust` binary (or one is found on `PATH`),
  shows its diagnostics live in their own collection with source `qb64rust`, beside the old compiler's on-save
  diagnostics with source `qb64pe`; and uses the server's outline, folding and definitions in place of the
  `language-configuration.json` folding markers where the server is running.
- **Tests:** protocol tests in Rust (`lsp-server` driven in-process), the extension's unit and integration suites
  extended for the server (started from a built binary), and the extension's CI workflow building the Rust binary
  first.

## Capabilities

### New Capabilities

- `editor/language-server`: the language server's protocol surface: syntax diagnostics, document symbols,
  folding ranges, go to definition for procedures and labels, the encoding and position mapping, how included
  files are found, reparse and cancellation.

### Modified Capabilities

- `compiler/cli`: the `lsp` subcommand.
- `editor/compiler-diagnostics`: two diagnostic sources (`qb64rust` live, `qb64pe` on save), neither clearing the
  other.
- `editor/language-support`: folding comes from the server when it runs; the static folding markers stay as the
  fallback.

## Impact

- New: `crates\lsp` (protocol handling, document store, encoding tables, the mapping of positions, the tree walks
  for symbols and definitions), `deny.toml`; `vscode\src\languageServer.ts` (client start, settings, encoding
  notifications), the dependency `vscode-languageclient`; tests under `crates\lsp\tests` and `vscode\test`.
- Changed: `crates\driver\src\main.rs` (the subcommand), `Cargo.toml` (dependencies, `cargo-deny` in `rust.yml`),
  `vscode\package.json` (settings `qb64rust.path`, `qb64rust.includeRoot`; the client), `vscode\src\diagnostics.ts`
  (the source label; clearing rules), `.github\workflows\vscode-extension.yml` (build the Rust binary; run the
  integration suite with it), `crates\README.md`, `vscode\README.md`, `vscode\DEVELOPMENT.md`, `STATUS.md`.
- No change to the parser, `sema`, the IR or the emitter; the server reads the trees through `ast.rs` as `sema`
  does. If a tree walk needs an accessor `ast.rs` lacks, it is added there.
- Not in this change: hover, completion, rename, references, go to definition for variables and constants
  (`sema`'s symbol table waits until most statements compile), semantic highlighting, the server shipped inside the
  `.vsix` (publishing work, `ROADMAP.md`), formatting by the new compiler (a later step), DAP (M5).
