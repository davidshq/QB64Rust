# Design

## Context

For the motivation see `proposal.md` ("Why"), `study\36` §3.1 ("Extension") and §4.2 ("DX"); for the behaviour,
the delta spec `editor/debug-launch`. How the parts stand (2026-10-25, branch `m5` after `m5-dap-proxy`):

- **The adapter** (spec `editor/debug-adapter`, `crates\dap`, `driver\src\main.rs` `dap`): `qb64rust dap
  [--qb64pe-root <dir>]` over stdin and stdout; `launch` takes an executable with `<exe>.debug.json` beside it,
  `args`, `cwd`, `stopOnEntry`, `console` (`integratedTerminal` and `externalTerminal` become `lldb-dap`'s
  `runInTerminal: true`, `dap\src\launch.rs`) and `noPrompt`. Its end-to-end tests (`driver\tests\cli.rs`
  `dap_*`) all launch with the default console; **`runInTerminal` through the proxy and `lldb-server` has not
  been run against the real `lldb-dap`** (only its reverse-request mapping, in tier 1).
- **The extension's build path** (`vscode\src\`): `buildPlan.ts` `planBuild` chooses the compiler
  (`chosenCompiler`: `qb64rust.buildWith`, else whether the binary is found), prepares libqb's copy and gives the
  command line; `compiler\qb64rust.ts` `buildArgs(file, roots)` is `-x -w <file> -o <exe> --qb64pe-root …
  --libqb-root … --include-root …`; `build.ts` `Builder.build` saves the document, runs the plan through the run
  queue, writes the "QB64" output, publishes Problems and offers "Build with QB64pe" after a failure whose errors
  are all marked. `tasks.ts` shares `planBuild`.
- **The binary** is found by `languageServer.ts` (`LanguageServer.binary`: `qb64rust.path`, the bundled one,
  `PATH`), and QB64pe by `statusBar.ts` (`CompilerStatus.current`). A build with `qb64rust` passes QB64pe's
  folder as `--qb64pe-root`; that folder's `internal\c\c_compiler\bin` holds `lldb-dap.exe` in the clone
  (4.7.0-GLFW). The extension's CI tests against the release `qb64pe_win-x64-4.7.0-GLFW.7z`
  (`.github\workflows\vscode-extension.yml`); whether that release carries `lldb-dap.exe` and `lldb-server.exe`
  is not yet looked at.
- **Tests**: unit tests run under mocha outside VS Code, so they import only modules that do not import `vscode`
  (`compiler\*.ts`); the integration suites (`test\integration\suite\`) run in a downloaded VS Code and skip the
  `qb64rust` suites without `QB64RUST_TEST_QB64RUST` and QB64pe (`buildQb64rust.test.ts`).

## Goals / Non-Goals

**Goals:**

- F5 on a `.bas` file debugs it, with nothing to configure, when Build with `qb64rust` works.
- One build path: the debug build is the Build command's with `--debug`, so roots, Problems, the output and the
  queue cannot differ.
- The configuration's resolution is a pure function with unit tests; one integration test debugs a real program.

**Non-Goals:**

- Attach, restart, several programs per session (the adapter has none of them).
- Debugging a build by QB64pe, or a fallback to it.
- A debug build in another folder or under another name.
- Any change to `qb64rust dap`, the symbol file or the compiler. If the measurement of D6 shows the adapter must
  change, that is said to the user before it is done (it widens the change's scope). One adapter change was made
  after the change's review, with the user's approval (2026-10-25): D9.

## Decisions

### D1. The contribution

`vscode\package.json`:

- `breakpoints`: `[{ "language": "qb64rust" }]`.
- `debuggers`: one, `type` `qb64rust`, `label` "QB64", `languages` `["qb64rust"]`; `configurationAttributes.launch`
  with `program` (string: the program's main `.bas` file, or an executable), `args` (array of strings), `cwd`,
  `stopOnEntry` (boolean), `console` (enum of the three), `noPrompt` (boolean); `initialConfigurations` one
  configuration named "Debug QB64 program" with `program` `${file}`; `configurationSnippets` the same.
- `commands`: `qb64rust.debug`, title "Debug", category "QB64", icon `$(debug-alt)`, `enablement`
  `isWorkspaceTrusted`; in `commandPalette` and `editor/title/run` (group `navigation@1`, the run menu's order
  becomes Build and Run, Debug, Run, Build) under the same `when` as Build.
- `activationEvents`: `onDebugResolve:qb64rust` (the commands and the language already activate the extension;
  this covers F5 from a `launch.json` before a `.bas` file is open). `onDebugDynamicConfigurations` is not used:
  it fires only for a provider registered as dynamic, and the provider here gives the initial configuration.

No `program` default in the schema: the resolution (D3) supplies it, so a `launch.json` written by hand without it
works.

### D2. The adapter: an executable descriptor

`debug.ts` `AdapterFactory implements vscode.DebugAdapterDescriptorFactory`: `createDebugAdapterDescriptor`
returns `new vscode.DebugAdapterExecutable(binary, ["dap", "--qb64pe-root", qb64peFolder])` with the binary of
`LanguageServer.binary` and the folder of `CompilerStatus.current` (as `planBuild` takes them). When either is not
found it throws an `Error` with the reason, which VS Code shows; the resolution (D3) has normally refused before.
*Alternative:* an inline adapter (`DebugAdapterInlineImplementation`) that spawns the binary. Rejected: nothing to
add between VS Code and the adapter; the executable form is VS Code's own process handling.

### D3. Resolution: pure rules, then the build

Two steps, as VS Code's `DebugConfigurationProvider` gives them:

1. `resolveDebugConfiguration` (before variables are substituted): an empty configuration (F5 with no
   `launch.json`) becomes the initial one when the active editor is a `qb64rust` file; otherwise it is returned
   unchanged, so VS Code's own choice of debugger is unaffected (spec, "Another file active"). `provideDebugConfigurations` gives the initial configuration.
2. `resolveDebugConfigurationWithSubstitutedVariables` (after `${file}` is a path): the pure function
   `debugConfig.ts` `resolveLaunch(config, facts) -> Launch` decides, and `debug.ts` acts.

`debugConfig.ts` imports no `vscode` (unit-tested under mocha as `compiler\*.ts`). `facts` are the trust, the
active file, the settings `qb64rust.runArguments` and `qb64rust.noPrompt` and the compiler choice. `Launch` is one
of:

| Case | Result |
|---|---|
| untrusted workspace | refuse: "debugging requires a trusted workspace" |
| `program` missing and no active `qb64rust` file | refuse: "open a QB64 program first" |
| `program` ends in `.exe` (case-insensitive) | launch it as it is |
| `program` is a `.bas` (any other extension: a source) and the chosen compiler is `qb64rust` | build, then launch the executable |
| a source, compiler `qb64pe` | refuse, naming `qb64rust.buildWith` |
| a source, binary not found | refuse, naming `qb64rust.path` |

With the defaults filled: `cwd` the program's folder, `console` `integratedTerminal` (D6), `noPrompt` and `args`
from the settings, `stopOnEntry` false. A refusal is shown by `debug.ts` (`showErrorMessage`, with a button
"Open Setting" when it names one) and the provider returns `undefined`, which ends the launch without opening
`launch.json`.

`args` from `qb64rust.runArguments`: `splitArgs` in `debugConfig.ts`, Windows' rules for a command line as the
C runtime reads it (spaces separate, double quotes group, `\"` is a quote), since the program's `COMMAND$` is read
from what the C runtime parsed. Run passes the text to a shell as typed (`build.ts` `shellCommand`), so there the
program's C runtime splits it; the two agree for the ordinary cases (words, double-quoted words) by construction,
and the unit tests check `splitArgs` on them (a test against `shellCommand` would compare `splitArgs` with
itself). That `lldb-dap` hands the split arguments to the program as they are is checked end to end by
`crates\driver\tests\cli.rs` `dap_args`.

### D4. The debug build is Build with one flag

- `compiler\qb64rust.ts`: `buildArgs(file, roots, debug = false)` puts `--debug` first when true; `build(…, debug)`
  passes it.
- `buildPlan.ts`: `planBuild(file, status, server, bundle, interactive, debug = false)`. With `debug`, a choice of
  `qb64pe` is a `NoBuild` whose reason names `qb64rust.buildWith`, shown with "Open Setting" when interactive;
  the missing binary's refusal is the existing one. The plan carries `debug`.
- `build.ts`: `Builder.debugBuild(uri): Promise<string | undefined>` runs `prepare` and `planBuild` with `debug`,
  then `buildWithQb64rust` with `notify` false. In `buildWithQb64rust`, the "Build with QB64pe" offer is skipped
  when `plan.debug` (spec, "Only unsupported forms"): the failure message is shown plain.
- `debug.ts` calls `Builder.debugBuild` from step 2 of D3 and sets `program` to the executable it returns; an
  `undefined` ends the launch (the failure is already in Problems and the output).

The debug build writes the program's usual executable (`executableFor`, beside the source) and its symbol file;
the next Build without `--debug` removes the symbol file (spec `compiler/debug-info`). Run after a debug session
runs the debug build, which behaves the same (spec `compiler/cli`).

### D5. The Debug command

`debug.ts` registers `qb64rust.debug(uri?)`: the file of `uri` or of the active editor, then
`vscode.debug.startDebugging(workspaceFolder, { type: "qb64rust", request: "launch", name: "Debug <base name>",
program: <file> })`; the resolution of D3 does the rest. A `launch.json` configuration is not consulted: the
command debugs the file, as Build and Run runs it.

### D6. `console`: the integrated terminal by default, measured first

A `$CONSOLE` program reads `INPUT` from its console; with `internalConsole`, `lldb-dap` gives the program no
input (the Debug Console's text goes to the debugger), so a program that asks for input cannot be debugged. A
window program (no `$CONSOLE`) writes nothing to stdout, so the terminal only stays idle. Hence
`integratedTerminal` by default for every program; `study\36` §4.2 wanted it only for `$CONSOLE` programs, which
needs the source scanned for a metacommand (an `$INCLUDE`d one too) for the saving of an idle terminal.

`runInTerminal` through the proxy, with `lldb-dap` under `LLDB_USE_LLDB_SERVER=1`, has not been run end to end
(Context). Task 1 runs it first (an ignored CLI test of `crates\driver`: launch with `console`
`integratedTerminal`, the test client answering the reverse request by starting the command it is given, a
breakpoint hit, `INPUT` fed through the started process's stdin). Then:

- It works → D6 as written.
- It fails in a way the proxy can fix → said to the user before the proxy is touched (Non-Goals); until then the
  default is `internalConsole`.
- It fails in `lldb-dap` → the default is `internalConsole`, the spec's scenario "Console input" is replaced by
  one saying that a program's input is not available in the Debug Console, a row goes to `SOMEDAY.md`, and the
  choice to `DECISIONS.md`.

### D7. Where each piece lives

| Piece | Where |
|---|---|
| Contribution, command, menus, activation | `vscode\package.json` |
| Resolution rules, defaults, `splitArgs` | `vscode\src\debugConfig.ts` (no `vscode` import) |
| Provider, adapter factory, Debug command, showing refusals | `vscode\src\debug.ts` (`DebugSupport`, registered in `extension.ts`; `Api` gains it for the tests) |
| `--debug` in the command line | `vscode\src\compiler\qb64rust.ts` `buildArgs` |
| The debug plan and its refusal | `vscode\src\buildPlan.ts` `planBuild` |
| The debug build, no QB64pe offer | `vscode\src\build.ts` `Builder.debugBuild`, `buildWithQb64rust` |
| `runInTerminal` measured | `crates\driver\tests\cli.rs` `dap_run_in_terminal` (`--ignored`), with the client of `driver\tests\support\dap.rs` |
| Arguments through `lldb-dap` | `crates\driver\tests\cli.rs` `dap_args` (`--ignored`) |
| An executable newer than its symbol file refused (D9) | `crates\dap\src\launch.rs`; `crates\dap\tests\session.rs` |
| Unit tests | `vscode\test\unit\debugConfig.test.ts`, `qb64rust.test.ts` (the `--debug` argument) |
| Integration test | `vscode\test\integration\suite\debug.test.ts`, fixture `vscode\test-fixtures\workspace\debug_me.bas` |
| Documentation | `vscode\README.md` ("Debugging"), `DEVELOPMENT.md` (the new suite and what it needs), `CHANGELOG.md` |

### D8. The integration test

`debug.test.ts`, skipped as `buildQb64rust.test.ts` is (no QB64pe or no `QB64RUST_TEST_QB64RUST`). The fixture
has a `SUB` with a `LONG` parameter and a main-module variable. The test registers a
`DebugAdapterTrackerFactory` for `qb64rust` that records the messages from the adapter, adds a
`SourceBreakpoint` in the `SUB`, runs `qb64rust.debug` on the fixture, waits for a `stopped` event, then asks the
session (`customRequest`) for `stackTrace`, `scopes` and `variables` and checks the frame name `SUB <name>`, the
line, and the parameter's value by its BASIC name; then `continue` and the session's end
(`onDidTerminateDebugSession`). A second test: `qb64rust.buildWith` `qb64pe` refuses with no session. The default
`console` is used, so the test covers D6's outcome as built.

If the QB64pe release of CI lacks `lldb-dap.exe` (task 1), the suite also skips when the QB64pe folder has none,
with the reason printed, and `DEVELOPMENT.md` says so.

### D9. An executable newer than its symbol file is refused (added after review)

A `qb64rust` build removes an earlier `<exe>.debug.json` with the executable (spec `compiler/debug-info`), but a
QB64pe build of the same program (`qb64rust.buildWith` `qb64pe`, the "Build with QB64pe" offer, or QB64pe's own
IDE) overwrites the executable and leaves the symbol file. A `launch.json` naming that `.exe` (D3: launched as it
is, whatever the compiler setting) would then start a session with another build's symbols. The adapter's
`launch` (`crates\dap\src\launch.rs`) refuses an executable whose modification time is later than its symbol
file's, with a message that says to build with `--debug` (spec `editor/debug-adapter`, "Launch"). A build writes
the symbol file after the executable is in place, and a build cache hit copies an executable with its older time,
so a debug build is never refused. Done in the adapter rather than the extension's QB64pe build, so that a build
outside VS Code is caught too. Tested in `crates\dap\tests\session.rs`.

## Risks / Trade-offs

- [`runInTerminal` does not work through the proxy] → D6's measurement first, with a fallback that keeps the rest
  of the change.
- [The QB64pe release lacks `lldb-dap`] → the adapter already answers `initialize` with the path it looked for,
  which VS Code shows (spec "QB64pe without lldb-dap"); README says what is needed.
- [A debug build replaces the program's optimised executable] → a debug build is never optimised, and
  `-f:OptimizeCppProgram=true` users get an unoptimised executable after a debug session until the next Build;
  said in README. A second output name was rejected (Non-Goals): the symbol file and the build cache would need
  a second path everywhere for little gain.
- [The first debug build after installing also builds libqb (about 40 s)] → the existing message of the first
  build is shown in the output, as for Build.
- [A breakpoint in a file that is not part of the program] → `lldb-dap` leaves it unverified, as it should.
- [A debug build's two files copied elsewhere by a tool that gives each copy the time of copying, the executable
  last] → D9 refuses it; building again with `--debug` there fixes it. Windows' copy keeps the times.

## Measured

- **`runInTerminal` through the proxy** (task 1.1, `driver\tests\cli.rs` `dap_run_in_terminal`, 2026-10-25, passed
  4 runs of 4): `lldb-dap` 23.1.2 sends the reverse request while it handles `launch`, **before** `initialized`,
  asking the client to run `lldb-dap.exe --comm-file \\.\pipe\lldb-dap-run-in-terminal-comm-<n> --launch-target
  <exe>` in the program's folder with `QB64PE_NOPROMPT` in its environment; once answered with the process id, it
  attaches to the program (`startMethod` `attach`) through `lldb-server`, and the proxy's `QBMAIN` breakpoint, the
  client's breakpoints and `evaluate` work as in a launch. A number written to the started command's stdin was
  read by the program's `INPUT` (`X` = 7 at the breakpoint after it). *Extension:* D6's first branch: `console`
  `integratedTerminal` by default, and VS Code answers the reverse request itself.
- **The QB64pe release** (task 1.2, the file list of `qb64pe_win-x64-4.7.0-GLFW.7z`, 2026-10-25): its
  `internal\c\c_compiler\bin` holds `lldb-dap.exe`, `lldb-server.exe` and `liblldb.dll` (and `lldb.exe`,
  `lldb-argdumper.exe`), each of the same size as the clone's. *Extension:* a user with the release can debug, and
  D8's extra skip is not built (task 4.2 not needed).

## Open Questions

None.
