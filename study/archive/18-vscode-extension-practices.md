# 18. VS Code extension practices: where M1 stands

Researched 2026-10-03. Compares the M1 extension (`vscode\`) with the official guidance (VS Code API docs, UX
guidelines, the `yo code` templates) and with five large extensions: rust-analyzer, Go, Python, Prettier, ESLint.
Testing is covered separately in `study\17`. §3 lists proposals; A–D were done on 2026-10-03.

## 1. What the big extensions do

| Practice | rust-analyzer | Go | Python | Prettier | ESLint | M1 |
|---|---|---|---|---|---|---|
| Bundled (single JS file) | esbuild | esbuild | webpack | esbuild | webpack | no: `tsc` to `out\`, 47 files |
| `vscode:prepublish` script | yes (minified build) | yes | gulp `package` | own build step | yes (webpack prod) | yes (since A) |
| Icon, repository, bugs, LICENSE | yes | yes | yes | yes | yes | none (`--allow-missing-repository --skip-license`) |
| `untrustedWorkspaces` | false | false | false | limited | false | limited, with description |
| `virtualWorkspaces` | (default) | false | limited | limited | false | limited (since B) |
| `extensionKind` | `["workspace"]` | (default) | (default) | `["workspace"]` | (default) | (default = workspace) |
| Activation | `workspaceContains` | `onLanguage` etc. | 20+ events | `onStartupFinished` | `onStartupFinished` | `[]` (implicit `onLanguage`, `onCommand`, task) |
| Language status item | – | yes (warnings) | – | yes | yes | yes (since C) |
| Log output channel (`{log: true}`) | yes | | | | | no: plain channel for compiler output |
| Localised strings (`%key%`, `l10n`) | no | no | yes | yes | no | no |
| Prettier for TS sources | yes | (gts) | yes | yes | | no (ESLint only) |
| CI on several OSes | yes | yes | yes | yes | yes | Windows only (since G); no GitHub remote yet |

Sources: each repository's `package.json` (links below) and, for language status, `client/src/client.ts`
(ESLint), `extension/src/goStatus.ts` (Go), `src/StatusBar.ts` (Prettier); rust-analyzer's log channel is in
`editors/code/src/util.ts`.

## 2. Official guidance and how M1 follows it

Followed:
- **Activation.** Contributed languages, commands and tasks need no explicit activation events since 1.74/1.76;
  `activationEvents: []` is the recommended form.
- **Workspace trust.** "limited" with a description, commands disabled by `enablement: isWorkspaceTrusted`. Finer
  than most of the five, which just say "not supported".
- **Types match the floor.** `@types/vscode ~1.100.0` = `engines.vscode`; `@types/node` 20 = the extension host's
  Node at 1.100 (`study\17`).
- **Commands.** One category ("QB64"), hidden from the palette outside QB64 files, run actions in
  `editor/title/run` (command palette guideline).
- **Disposables** all go through `context.subscriptions`. **No telemetry**, so nothing to gate on
  `isTelemetryEnabled`.
- **`.vscodeignore`** keeps sources, tests and maps out of the package.

Not followed (before A–D):
1. **No `vscode:prepublish`.** `vsce` runs it before packaging; without it `npm run package` packs whatever is in
   `out\`, stale or not. Every one of the five and the `yo code` templates have it.
2. **`virtualWorkspaces` undeclared.** The default is "supported", but every feature except highlighting needs a
   `file:` URI and a local process. Docs: extensions that spawn processes should declare `false` or `"limited"`
   with a description. The code mostly checks `uri.scheme` already; `tasks.ts` (`folder.uri.fsPath`) and
   `removeStaleTempFiles` do not.
3. **Status bar.** The "compiler found / not found / checking…" item is language-specific information, which
   `languages.createLanguageStatusItem(id, selector)` is made for: it is shown only for matching editors, and has
   `busy` and `severity`. M1 uses a plain `StatusBarItem`, shows and hides it by hand, and sets the warning
   background, which the status bar guideline reserves "as a last resort" for blocking problems. ESLint, Prettier
   and Go use language status items.
4. **Notifications.** "Only send notifications when absolutely necessary." M1 shows "QB64: built X.exe" after every
   successful build, including Build and Run, where the program starting is the confirmation. Errors and the
   trust/save prompts are user-initiated and fine.
5. **Bundling.** Recommended for load time ("loading 100 small files is much slower than loading one large file")
   and required for the web. All five bundle; the `yo code` esbuild template does too. For M1 the gain is small:
   11 own files plus `iconv-lite` (about 30 files, 229 KB in total). If `iconv-lite` is replaced by
   `workspace.encode`/`decode` (possible since 1.100), there are no runtime dependencies left and bundling buys
   even less.
6. **Marketplace metadata.** Icon (PNG, at least 128 px), `repository`, `bugs`, a LICENSE file, CHANGELOG. Needed
   before publishing to the Marketplace or Open VSX; not needed while the `.vsix` is installed by hand.
7. **CI.** The docs recommend a GitHub Actions matrix (Windows, macOS, Linux with `xvfb-run`). The posix branches
   of `shellCommand` and discovery's `:` PATH split have only been unit-tested, never run on Linux or macOS.
   The compiler tests need `qb64pe`, which CI would have to build or skip.

Considered and not needed:
- **Log output channel.** The "QB64" channel shows the compiler's output as a transcript; timestamps and levels
  on every line would get in the way. rust-analyzer's log channel is for the extension's own diagnostics, which M1
  barely has.
- **Localisation.** Two of the five do it; English-only is common. Revisit if users ask.
- **`extensionKind`.** The default ("workspace", run where the files are) is right; declaring it only documents it.
- **Problem matcher for the build task.** The build task (`ProcessExecution`) shows errors in the terminal but not
  in Problems; the Build command does both. A problem matcher would need a multi-line pattern for the
  `LINE n:` block and the `\x01` include suffix. M2's language server makes this moot.

## 3. Proposals

| # | Change | State |
|---|---|---|
| A | Add `"vscode:prepublish": "npm run compile"` | done 2026-10-03; `npm run package` now compiles first |
| B | Declare `virtualWorkspaces: {supported: "limited", description: …}`; guard the two `fsPath` uses | done 2026-10-03; commands are also hidden from the palette and run menu when `virtualWorkspace` is set |
| C | Replace the status bar item with a `LanguageStatusItem` (severity instead of colour, `busy` instead of the spin icon) | done 2026-10-03; text `qb64pe` / `qb64pe: checking…` / `compiler not found` (error severity, "Open Setting"), path in `detail` |
| D | Drop the success notification after Build and Run | done 2026-10-03; Build alone still shows "built X" |
| E | Bundle with esbuild | open; low value now |
| F | Icon, repository, LICENSE, CHANGELOG | open; before publishing |
| G | CI on three OSes | Windows done 2026-10-03: `.github\workflows\vscode-extension.yml` (GitHub Actions, `windows-latest`, QB64pe release download); runs once the repo is on GitHub. macOS and Linux open |

After A–D: unit 31, grammar 4, integration 37 (new: language status while busy) on VS Code stable and 1.100.

## Sources

- [UX guidelines: status bar](https://code.visualstudio.com/api/ux-guidelines/status-bar),
  [notifications](https://code.visualstudio.com/api/ux-guidelines/notifications)
- [Bundling extensions](https://code.visualstudio.com/api/working-with-extensions/bundling-extension),
  [Publishing](https://code.visualstudio.com/api/working-with-extensions/publishing-extension),
  [Continuous integration](https://code.visualstudio.com/api/working-with-extensions/continuous-integration)
- [Activation events](https://code.visualstudio.com/api/references/activation-events),
  [Virtual workspaces](https://code.visualstudio.com/api/extension-guides/virtual-workspaces),
  [API reference](https://code.visualstudio.com/api/references/vscode-api) (`createLanguageStatusItem`, `LogOutputChannel`)
- [`yo code` esbuild template](https://github.com/microsoft/vscode-generator-code/tree/main/generators/app/templates/ext-command-ts/vscode-esbuild)
- package.json: [rust-analyzer](https://github.com/rust-lang/rust-analyzer/blob/master/editors/code/package.json),
  [Go](https://github.com/golang/vscode-go/blob/master/extension/package.json),
  [Python](https://github.com/microsoft/vscode-python/blob/main/package.json),
  [Prettier](https://github.com/prettier/prettier-vscode/blob/main/package.json),
  [ESLint](https://github.com/microsoft/vscode-eslint/blob/main/package.json)
