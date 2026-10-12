# Developing the extension

Commands run from `vscode\` unless stated otherwise. Node 20.19 or later.

`tsconfig.json` lists `compilerOptions.types` as `node` and `mocha`. Without `node`, TypeScript does not load `@types/node` (already a devDependency), so `import * as assert from "assert"` and other Node names fail in the editor. `mocha` keeps `describe` / `it` typed once `types` is set, because that field replaces automatic `@types` inclusion.

## Build

```sh
npm ci
npm run compile      # tsc → out\
npm run lint
```

To try it, open `vscode\` as a folder in VS Code and press F5 ("Run Extension" in `.vscode\launch.json`), or
package it (below).

## Tests

| Command | What | Needs |
|---|---|---|
| `npm run test:unit` | Output parser (against recorded fixtures), compiler discovery (`qb64pe` and `qb64rust`), run queue, the language server's initialization options and encoding notification | Node only |
| `npm run test:grammar` | Annotated `test\grammar\*.bas` against the TextMate grammar | Node only |
| `npm run test:integration` | Downloads a VS Code build into `.vscode-test\` and runs `test\integration\suite` in it, in a temporary copy of `test-fixtures\workspace` | A compiler for the compiler tests (skipped without one): `QB64RUST_TEST_QB64PE`, else `..\..\QB64pe\qb64pe.exe`; the `qb64rust` binary for the language server tests (skipped without it): `QB64RUST_TEST_QB64RUST` |
| `npm run test:integration:min` | The same on the oldest version `engines.vscode` accepts (1.100.0) | as above |
| `npm test` | All four | |

Integration options (environment variables): `QB64RUST_TEST_QB64PE` (path to `qb64pe.exe`; an error if it does
not exist, so CI cannot silently skip the compiler tests), `QB64RUST_TEST_QB64RUST` (path to `qb64rust.exe`, same
rule; without it `qb64rust.path` is set to a file that does not exist, so a `qb64rust` on `PATH` is not used and
the run is the extension without the server), `QB64RUST_TEST_VSCODE_VERSION` (default `stable`),
`QB64RUST_TEST_GREP` (run only tests whose name matches).

## The language server

`src\languageServer.ts` starts `qb64rust lsp` (`vscode-languageclient`, stdio) when `qb64rust.path` or `PATH`
names a binary and the workspace is trusted; `src\server\protocol.ts` holds the parts without VS Code (the
initialization options, the `qb64rust/documentEncoding` notification). The server itself is the Rust crate
`crates\lsp` (its tests: `cargo test -p qb64rust-lsp`). To run the extension against it:

```sh
cargo build --release                                     # from the repository root
QB64RUST_TEST_QB64RUST=../target/release/qb64rust.exe npm run test:integration
```

For F5, set `qb64rust.path` to `<repo>\target\release\qb64rust.exe` in the development host's settings. The
server writes nothing to stdout but the protocol; its own messages (a panic, a failed parse) go to stderr, which
VS Code shows in the "QB64 language server (qb64rust)" output channel.

`tsconfig.json` uses `"module": "node16"`: `vscode-languageclient` 10 publishes its entry points through the
`exports` map of its `package.json` (`vscode-languageclient/node`), which TypeScript resolves only with `node16`
module resolution. The output is still CommonJS (`package.json` has no `"type": "module"`).

The server's encoding tables (`crates\lsp\src\encoding_tables.rs`) are generated from this folder's `iconv-lite`,
the library VS Code reads and writes those encodings with: `node tools\encodings\gen_tables.js` from the repository
root, after `npm ci` here.

**CI.** `.github\workflows\vscode-extension.yml` (repo root) runs on `windows-latest` for changes under `vscode\`,
`crates\` and the Rust workspace's manifests: it builds `qb64rust` (release), then lint, unit, grammar,
integration on stable and on 1.100 with `QB64RUST_TEST_QB64RUST` set, then packages the `.vsix` as a build
artifact. It downloads the QB64pe `v4.7.0-GLFW` Windows release (the version the fixtures were recorded with) and
caches it, including the runtime library QB64pe compiles on its first build. Checked locally 2026-10-03: the
integration suite passes against that release freshly extracted (then 37 passing, none skipped, about a minute);
2026-10-08, with the language server: 42 passing, or 37 passing and 5 pending without `QB64RUST_TEST_QB64RUST`.
macOS and Linux are not in CI yet.

The integration runner clears `ELECTRON_RUN_AS_NODE`, which is set in terminals inside VS Code and would start
the test instance as plain Node.

**Why mocha 12 and VS Code 1.100.** The integration tests run inside VS Code's extension host, which loads the
runner with `require`. Mocha 12 is ESM-only; Node can `require` ESM from 20.19 / 22.12. Measured 2026-10-03: it fails
on VS Code 1.99 (Node 20.18, `ERR_REQUIRE_ESM`) and works on 1.100 (Node 20.19.0), so `engines.vscode` is
`^1.100.0` (decision 2026-10-03, `DECISIONS.md`). 1.100 also gives `TextDocument.encoding`, which `config.ts` uses.
Mocha is what VS Code's own test tooling (`@vscode/test-cli`) and most large extensions use; Jest and Vitest run
tests in their own workers where the `vscode` module does not exist, so they only suit unit tests with a fake
`vscode` (survey in `study\17-vscode-extension-testing.md`).

## Compiler output fixtures

`test-fixtures\compiler\` holds small programs and what `qb64pe` 4.7.0 printed for them (`<case>.out.txt`,
`<case>.exit`, and `<case>.formatted.bas` for `-y`). The parser unit tests read these, so they run without the
compiler. To record them again (Git Bash):

```sh
scripts/record-fixtures.sh
git diff --stat test-fixtures/compiler    # no diff expected with the same compiler
```

## Grammar

`syntaxes\qb64rust.tmLanguage.json` is maintained by hand. Its keyword lists were copied once (2026-10-03) from
`tools\builtins\builtins.json` (built-in functions and statements), from
`..\QB64pe\internal\support\include\beforefirstline.bi` (`_TRUE`, `_FALSE` and the other built-in constants) and
from `..\QB64pe\internal\support\color\color0.bi` and `color32.bi` (color constants). Generating them from
`builtins.json` at build time is planned for later. Pattern order matters where two patterns match at the same
column: labels come before numbers (`10 PRINT` is a line number), and built-in functions come before suffixed
identifiers (`CHR$` is a function, not a variable).

## Package

```sh
npm run package      # → qb64rust-<version>.vsix
code --install-extension qb64rust-0.1.0.vsix
```
