# Developing the extension

Commands run from `vscode\` unless stated otherwise. Node 20.19 or later; the repo's `mise.toml` and CI use
Node 24 (`mise install` at the repo root gives it).

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
| `npm run test:unit` | Output parser (against recorded fixtures), compiler discovery (`qb64pe` and `qb64rust`), run queue, the language server's initialization options and encoding notification, the debug launch's resolution (`debugConfig.ts`), the format arguments, layout settings, choice of formatter and Format Selection's lines (`format.test.ts`) | Node only |
| `npm run test:grammar` | Annotated `test\grammar\*.bas` against the TextMate grammar | Node only |
| `npm run test:integration` | Downloads a VS Code build into `.vscode-test\` and runs `test\integration\suite` in it, in a temporary copy of `test-fixtures\workspace` | A compiler for the compiler tests (skipped without one): `QB64RUST_TEST_QB64PE`, else `..\..\QB64pe\qb64pe.exe`; the `qb64rust` binary for the language server tests (skipped without it): `QB64RUST_TEST_QB64RUST` |
| `npm run test:integration:min` | The same on the oldest version `engines.vscode` accepts (1.100.0) | as above |
| `npm test` | All four | |

Integration options (environment variables): `QB64RUST_TEST_QB64PE` (path to `qb64pe.exe`; an error if it does
not exist, so CI cannot silently skip the compiler tests), `QB64RUST_TEST_QB64RUST` (path to `qb64rust.exe`, same
rule; without it `qb64rust.path` is set to a file that does not exist, so a `qb64rust` on `PATH` is not used and
the run is the extension without the server), `QB64RUST_TEST_VSCODE_VERSION` (default `stable`),
`QB64RUST_TEST_GREP` (run only tests whose name matches). The test workspace sets `qb64rust.buildWith` to `qb64pe`,
so the build tests of QB64pe test it; `buildQb64rust.test.ts` sets `auto` and needs both variables' programs (QB64pe
gives the toolchain). With `bundle\` staged it also checks the copy of libqb prepared in the test profile's global
storage; the first such build there builds libqb's objects. Without `bundle\` the binary uses the repo's `libqb\`.
`debug.test.ts` needs the same two and debugs `test-fixtures\workspace\debug_me.bas` through `qb64rust dap` and
the `lldb-dap` of QB64pe's toolchain (the clone and the 4.7.0 release both carry it): the Debug command, F5
without a `launch.json`, a failed build, and the refusal under `qb64rust.buildWith` `qb64pe`. The binary must have
the `dap` subcommand of this tree (`cargo build --release` after a change under `crates\dap`).
`scripts\record-qb64rust-fixtures.sh` records the output fixtures of `test-fixtures\qb64rust\` (the parser's unit
tests) with `target\release\qb64rust.exe`.

## The language server

`src\languageServer.ts` starts `qb64rust lsp` (`vscode-languageclient`, stdio) when `qb64rust.path`, the bundled
binary (`bundle\bin\qb64rust.exe`, present only once `npm run stage` has run) or `PATH` names a binary and the
workspace is trusted; a build with `qb64rust` uses the same binary (`src\buildPlan.ts`); `src\server\protocol.ts` holds the parts without VS Code (the
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

Run `python tools/licenses/check.py` from the repository root to check the copied-content inventory.
`tools/licenses/inventory.toml` records provenance and required notices; add an entry when importing code,
tests or assets, after reviewing their terms. New `SOURCE.md` records and content in common vendor directories
require inventory coverage. Imports elsewhere still require human review: this is not a code similarity or
legal compatibility scanner. `cargo deny check` separately checks Rust dependency licenses.
The license workflow runs the repository check on pushes and pull requests. The VSIX checker also runs it
against the archive, requiring every inventoried runtime notice to match the repository text (ignoring CRLF).
Tests: `python -m unittest discover -s tools/licenses -p "test_*.py"` from the repository root.

The `.vsix` is a platform package for Windows x64 that carries the new compiler (`openspec\specs\editor\packaging`):

```sh
cargo build --release                       # from the repository root: target\release\qb64rust.exe
npm run package                             # stage, then vsce → qb64rust-win32-x64-<version>.vsix
python scripts/check_vsix.py qb64rust-win32-x64-0.2.0.vsix
code --install-extension qb64rust-win32-x64-0.2.0.vsix
```

`npm run stage` (`scripts\stage_bundle.py`, run by `package`) writes the git-ignored `bundle\`: `bin\qb64rust.exe`
(`--binary` for another), `libqb\` (the tracked files of the repo's `libqb\`, so no object a build left there
ships), `libqb.id` (a hash of those files, which names the extension's prepared copy so that a package with other
libqb files never reuses an older copy), and `THIRD-PARTY-NOTICES.txt`, the licence texts of the crates linked
into the binary as `cargo tree` resolves them for the Windows target. It also copies the repository's `LICENSE` here, where `vsce` looks for it.
A crate without a licence file of its own takes `scripts\licenses\<name>.txt` (now `astro-float-num`, its text
taken from its repository); a crate with neither stops the staging, naming it. `scripts\check_vsix.py` holds a
package to the spec's list: the target, the required files, no `.ts`, `.o`, `.a` or `.map` file and no
`c_compiler`; with `--tag vscode-v<version>` it also checks the tag against `package.json`. The scripts' tests:
`python -m unittest discover -s scripts -p "test_*.py"`.

Measured 2026-10-19 (0.2.0): 8.2 MB, 1,512 files; uncompressed the bundle is the binary (4.8 MB), libqb (25.6 MB,
1,233 files) and the notice (244 KB). Without the bundle the package was under 1 MB.

**Release.** Push a tag `vscode-v<version>` equal to `package.json`'s version (and a matching section in
`CHANGELOG.md`): `.github\workflows\release-vsix.yml` builds, packages, checks and attaches the `.vsix` to a GitHub
release of the tag.
