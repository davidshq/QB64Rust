# Proposal

## Why

M5 takes Path A of `study\36` (user, `DECISIONS.md` 2026-10-24). Its first two changes are archived: the debug
build and its symbol file (`m5-debug-build-symbols`) and the debug adapter `qb64rust dap` over the toolchain's
`lldb-dap` (`m5-dap-proxy`, spec `editor/debug-adapter`). Neither is reachable from VS Code yet: the extension
contributes no debugger, so a user cannot press F5 on a `.bas` file, set a breakpoint in it, or start the adapter
without writing a launch configuration for an extension that does not exist. This change is ROADMAP M5's fourth
and last item, the one that lets a user actually debug, and so the step that meets M5's "Done when".

## What Changes

- **A debugger of type `qb64rust`** in the extension's `package.json` (`debuggers`, `breakpoints` for the
  `qb64rust` language): launch attributes `program` (a `.bas` file, by default the active one, or a debug build's
  executable), `args`, `cwd`, `stopOnEntry`, `console` and `noPrompt`, an initial configuration and a snippet for
  `launch.json`.
- **Build before launch**: a configuration whose `program` is a `.bas` file is built with `qb64rust --debug`
  through the extension's own build path (the same compiler choice, roots, output channel, Problems and run
  queue as Build), and launched only when the build succeeds; its `program` is then the executable. A build that
  is not by `qb64rust` (`qb64rust.buildWith` is `qb64pe`, or the binary is not found) cannot be debugged: the
  launch is refused with a message saying why, with no fallback to QB64pe.
- **The adapter** is `qb64rust dap --qb64pe-root <QB64pe folder>`, run from the binary the language server and
  builds use (spec `editor/packaging`), with the QB64pe folder the build uses.
- **F5 with no `launch.json`** on a `.bas` file debugs it; a **"Debug" command** (`qb64rust.debug`) beside Build
  and Run, in the command palette and the editor's run menu, does the same.
- **Defaults** that suit both kinds of program: `console` is the integrated terminal (a `$CONSOLE` program reads
  its input there), `noPrompt` follows `qb64rust.noPrompt`, `args` follows `qb64rust.runArguments`.
- **Workspace trust**: debugging is unavailable in an untrusted workspace, as building is.
- **Tests**: unit tests of the configuration's resolution (defaults, a `.bas` versus an executable, refusals);
  an integration test that debugs a fixture program in VS Code (a breakpoint hit, a variable read by its BASIC
  name, the session ended), skipped without QB64pe and the `qb64rust` binary as the other `qb64rust` tests are.

Out of this change: attach to a running program; debugging a build by QB64pe (its `$DEBUG` debugger is the old
IDE's, D-017); a separate output path for debug builds (a debug build replaces the program's executable, which
behaves the same, spec `compiler/cli` "A debug build is the same program"); anything in `qb64rust dap` itself.

## Capabilities

### New Capabilities

- `editor/debug-launch`: how the extension starts a debug session: the debugger contribution and its launch
  attributes, the debug build before launch, how the adapter is found and started, the defaults, F5 without a
  configuration, the Debug command and workspace trust.

### Modified Capabilities

- `editor/debug-adapter`: "Launch" also refuses an executable modified later than its symbol file (another build
  wrote it again), added after the change's review with the user's approval (design D9).

`editor/build-run` keeps its requirements (Build, Run and the tasks are unchanged; the debug build shares their
path but is specified in `editor/debug-launch`).

## Impact

- `vscode\package.json`: `debuggers`, `breakpoints`, the command `qb64rust.debug` and its menu entries, activation
  on `onDebugResolve:qb64rust`.
- `vscode\src\`: a new `debug.ts` (configuration provider, adapter descriptor factory, the Debug command);
  `buildPlan.ts` and `compiler\qb64rust.ts` take a debug flag (`--debug` in the arguments); `build.ts` exposes a
  debug build that returns the executable or a refusal; `extension.ts` registers it all.
- `vscode\test\`: unit tests of the resolution; an integration suite `debug.test.ts` and a fixture program.
- `vscode\README.md`, `DEVELOPMENT.md`, `CHANGELOG.md` (unreleased): debugging, what it needs (QB64pe's toolchain
  with `lldb-dap`, the `qb64rust` binary).
- Whether the QB64pe release the extension's CI tests against (4.7.0) carries `lldb-dap.exe` and `lldb-server.exe`
  as the clone does is checked first; the adapter already refuses `initialize` with the path it looked for.
- No change to the compiler or the symbol file. One to the adapter, after review (design D9): `launch` refuses an
  executable newer than its symbol file (`crates\dap\src\launch.rs`). M5's "Done when" is met when this change is built;
  marking M5 done in `ROADMAP.md` and `DECISIONS.md` comes at the merge into `main` (`CLAUDE.md` working rule 11).
