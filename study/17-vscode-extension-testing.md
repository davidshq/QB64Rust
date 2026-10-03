# 17. How VS Code extensions are tested, and where M1 stands

Researched 2026-10-03 for the M1 extension (`vscode\`): which test runner to use, and which minimum VS Code
version. Facts marked *measured* were run here; the rest come from the sources listed.

## 1. The constraint

Integration tests run inside a real VS Code instance (the Extension Development Host), the only place where the
`vscode` module exists. `@vscode/test-electron` starts that instance and loads a test entry file with `require`,
inside VS Code's own Node (Electron). So the runner must be loadable there, and its Node version is VS Code's, not
the developer's.

| VS Code | Electron | Node in the extension host |
|---|---|---|
| 1.85 | 25.9.7 | 18.15.0 |
| 1.99 | 34.3.2 | 20.18.3 |
| 1.100 | 34.5.1 | 20.19.0 |
| 1.101 | 35.5.1 | 22.15.1 |
| 1.140 (stable, 2026-10) | | 24.21.0 (*measured*) |

Node can `require` an ES module by default from 20.19.0 and 22.12.0. Mocha 12 is ESM-only (npm: `"type":
"module"`, engines `^20.19.0 || >=22.12.0`); mocha 10 and 11 are CommonJS (11 asks for Node 18.18+).

*Measured* with the M1 suite: mocha 12 fails on VS Code 1.99 with `ERR_REQUIRE_ESM` and runs on 1.100, 1.101 and
1.140. Mocha 11 runs the full suite on 1.85 despite its Node 18.18 floor. `npm audit`: mocha 10 has 4 high findings
(`braces`, `serialize-javascript`), mocha 11 has 3 (1 high), mocha 12 has none. All are development-only.

## 2. Runners

- **Mocha.** The VS Code docs' testing guide and `@vscode/test-cli` (the official runner; "exclusively uses Mocha
  under the hood") are built on it.
- **Jest.** Runs tests in its own workers with its own module system, so `import * as vscode` fails ("Cannot
  find module 'vscode'"). Extensions that use it mock `vscode` by hand (`__mocks__/vscode.ts`). That is fine
  for logic, but it cannot check real Problems entries, edits, terminals or tasks.
- **Vitest.** The same limit. Where it is used for extension code, it runs unit tests against a small fake of
  `vscode` aliased in `vitest.config.ts`. Telling detail: the Vitest team's own VS Code extension
  (`vitest-dev/vscode`) runs its extension tests with **Mocha via `@vscode/test-cli`**, and uses Vitest only for
  Playwright end-to-end tests.
- **`node:test`** (built into Node). No dependency. Good for unit tests that do not import `vscode`.

## 3. What popular extensions do (package.json, 2026-10-03)

| Extension | engines.vscode | Extension-host tests | Unit tests |
|---|---|---|---|
| Python (`microsoft/vscode-python`) | ^1.95.0 | `@vscode/test-electron` + mocha 11 | mocha, chai, sinon |
| Go (`golang/vscode-go`) | ^1.90.0 | `@vscode/test-electron` + mocha 11.7 | mocha (`-u tdd`), sinon |
| Prettier (`prettier/prettier-vscode`) | ^1.101.0 | `@vscode/test-cli` (mocha 10), also `@vscode/test-web` | – |
| Vitest (`vitest-dev/vscode`) | ^1.88.0 | `@vscode/test-cli` + mocha, chai | Vitest + Playwright for e2e |
| ESLint (`microsoft/vscode-eslint`, client) | ^1.91.0 | – | `node --test` (no `vscode` import) |
| rust-analyzer (`editors/code`) | ^1.93.0 | `@vscode/test-electron` with a small hand-written runner | same runner |
| GitLens | ^1.101.0 | not determined (manifest too large to read) | |

Patterns:
1. Every one that tests inside VS Code uses `@vscode/test-electron` (directly or through `@vscode/test-cli`), and
   nearly all of them use mocha. None uses Jest or Vitest inside the extension host.
2. rust-analyzer runs the suite twice: on the minimum `engines.vscode` version and on stable.
3. Floors between 1.88 and 1.101 are normal; two of the seven need 1.101.

## 4. How M1 lines up

| Practice | M1 |
|---|---|
| `@vscode/test-electron` + mocha in the extension host | Same (mocha 12, programmatic, `test\integration\suite\index.ts`) |
| Pure unit tests outside VS Code | Same, with mocha (parser, discovery, run queue); `node:test` would also do |
| Test on the minimum version and stable (rust-analyzer) | Adopted: `test:integration` (stable) and `test:integration:min` (1.100.0) |
| `@vscode/test-cli` config file instead of a hand-written launcher | Not used: the launcher copies a fixture workspace to a temp folder, writes its settings and points it at `..\QB64pe\qb64pe.exe`, which the CLI's config does not do directly. Could be revisited. |
| Grammar tests | `vscode-tmgrammar-test` with annotated fixtures (common in language extensions) |
| Mocks (`sinon`, fake `vscode`) | None: the logic that needs no VS Code is in modules that do not import it, and the rest is tested for real |

Decision (user, 2026-10-03): minimum VS Code 1.100 (released April 2025), mocha 12. Recorded in `CLAUDE.md`.

## Sources

- Node.js release notes: [20.19.0](https://nodejs.org/en/blog/release/v20.19.0), [22.12.0](https://nodejs.org/en/blog/release/v22.12.0)
- VS Code / Electron / Node versions: [ewanharris/vscode-versions](https://github.com/ewanharris/vscode-versions); [VS Code 1.101 notes](https://code.visualstudio.com/updates/v1_101)
- [VS Code docs: Testing Extensions](https://code.visualstudio.com/api/working-with-extensions/testing-extension); [@vscode/test-cli](https://www.npmjs.com/@vscode/test-cli)
- Jest and `vscode`: [microsoft/vscode-test#37](https://github.com/microsoft/vscode-test/issues/37); [Microsoft ISE blog](https://devblogs.microsoft.com/ise/testing-vscode-extensions-with-typescript/)
- Vitest with a fake `vscode`: [t-hamano/mark-bricks#105](https://github.com/t-hamano/mark-bricks/pull/105)
- package.json files: [vitest-dev/vscode](https://github.com/vitest-dev/vscode), [microsoft/vscode-python](https://github.com/microsoft/vscode-python), [microsoft/vscode-eslint](https://github.com/microsoft/vscode-eslint) (client), [golang/vscode-go](https://github.com/golang/vscode-go), [prettier/prettier-vscode](https://github.com/prettier/prettier-vscode), [rust-lang/rust-analyzer](https://github.com/rust-lang/rust-analyzer) (`editors/code`), [gitkraken/vscode-gitlens](https://github.com/gitkraken/vscode-gitlens)
