# Tasks

Standing rules: CLAUDE.md. What this change does differently: its own tests are the extension's (`npm run
test:unit`, `npm run lint`, and `npm run test:integration` with `QB64RUST_TEST_QB64RUST` set, `vscode\DEVELOPMENT.md`)
beside tier 1; the two facts it rests on that are not measured yet (task 1) are measured before the code that
relies on them, and written into `design.md` under a new section "Measured".

## 1. Measured first (design D6, "Risks")

- [x] 1.1 `crates\driver\tests\cli.rs` `dap_run_in_terminal` (`--ignored`): a `$CONSOLE:ONLY` program with
  `INPUT x` and a breakpoint after it, launched through `qb64rust dap` with `console` `integratedTerminal`; the
  client of `driver\tests\support\dap.rs` answers `runInTerminal` by starting the command it is given (its `cwd`,
  `env`), feeds a number on that process's stdin, and the program stops at the breakpoint with `X` holding it.
  Verify: the test passes alone; its result and D6's branch taken are in `design.md`, "Measured". If a branch
  other than "it works" is taken, the default and the spec's scenario "Console input" are amended as D6 says,
  and with the proxy's branch the user is asked before `crates\dap` is touched.
- [x] 1.2 The QB64pe release of the extension's CI (`qb64pe_win-x64-4.7.0-GLFW.7z`, `vscode-extension.yml`):
  whether its `internal\c\c_compiler\bin` holds `lldb-dap.exe`, `lldb-server.exe` and `liblldb.dll` as the clone
  does (the release's file list, downloaded into the scratchpad). Verify: the answer in `design.md`, "Measured";
  without them, D8's extra skip is built in task 4.2.

## 2. The build with `--debug` (D4)

- [x] 2.1 `compiler\qb64rust.ts`: `buildArgs(file, roots, debug)` and `build(…, debug)`; `qb64rust.test.ts`:
  `--debug` present only when asked. Verify: `npm run test:unit`.
- [x] 2.2 `buildPlan.ts` `planBuild(…, debug)`: the refusal for `qb64pe` naming `qb64rust.buildWith`, the plan's
  `debug`; `build.ts` `Builder.debugBuild` and no "Build with QB64pe" offer for a debug plan. `tasks.ts` and Build
  unchanged. Verify: `npm run compile`, `npm run lint`; the existing integration suites pass.

## 3. Debugging (D1–D3, D5)

- [x] 3.1 `debugConfig.ts`: `resolveLaunch` with the table of D3 and the defaults, `splitArgs`.
  `test\unit\debugConfig.test.ts`: each row of the table; each default; `splitArgs` on words, quoted words, `\"`,
  empty text (the cases where Run, which leaves the splitting to the program's C runtime, agrees by construction;
  design D3). Verify: `npm run test:unit`.
- [x] 3.2 `package.json` (D1): `breakpoints`, `debuggers` with the attributes, initial configuration and snippet,
  the command `qb64rust.debug` and its menus, the activation events. Verify: `npm run compile`; the extension
  activates in the integration host (`activation.test.ts`).
- [x] 3.3 `debug.ts` (`DebugSupport`): the provider's two steps, the refusals shown with "Open Setting", the
  adapter factory (D2), the Debug command (D5); registered in `extension.ts`, on `Api`. Verify: `npm run lint`,
  `npm run test:unit`.

## 4. Tests in VS Code (D8)

- [x] 4.1 Fixture `test-fixtures\workspace\debug_me.bas`; `test\integration\suite\debug.test.ts`: the breakpoint
  in the `SUB` hit through `qb64rust.debug`, the frame `SUB <name>` at its line, the parameter's value by BASIC
  name, `continue` to the session's end; F5's path (`startDebugging` with an empty configuration while the
  fixture is active) debugs it too; `buildWith` `qb64pe` refuses with no session; an untrusted workspace is not
  testable in the host (as for Build) and is covered by the unit test of 3.1. Verify: `QB64RUST_TEST_QB64RUST=…
  npm run test:integration` and `test:integration:min` pass, the new suite not skipped.
- [x] 4.2 Only if 1.2 found no `lldb-dap` in the release: the suite skips when the QB64pe folder has none, with the
  reason printed. Verify: the skip message, run against a folder without it. *Not needed: 1.2 found `lldb-dap.exe`
  and `lldb-server.exe` in the release (design, "Measured"); nothing built.*

## 5. Documentation

- [x] 5.1 `vscode\README.md` "Debugging" (F5, the Debug command, `launch.json` attributes, what it needs: QB64pe's
  toolchain with `lldb-dap` and the `qb64rust` binary; a debug build replaces the executable; the exception filter
  "BASIC runtime errors"); `DEVELOPMENT.md` (the new suite, its needs); `CHANGELOG.md` under "Unreleased".
  `crates\README.md` only for the new `--ignored` test's line, if it lists them. Verify: the sections exist;
  `tools\repo_check\check_repo.py --untracked` passes.

## 6. Close

- [x] 6.1 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then
  `tools\close\close.py --check` passes. Then `STATUS.md`'s "Next" says M5's items are built and M5 is ready to be
  declared done at the next merge into `main`, and `python tools\testtime\test_time.py`. Verify: `--check` passes.

## 7. After review (the pragmatic engineer's recommendations, user 2026-10-25)

- [x] 7.1 Design D9: `crates\dap\src\launch.rs` refuses an executable newer than its symbol file; the delta spec
  `editor/debug-adapter` ("Launch", scenario "Built again by QB64pe"); `crates\dap\tests\session.rs` tests it, and
  its fixtures write the executable before the symbol file. Verify: `cargo test -p qb64rust-dap`; the `dap_*` CLI
  tests (`--ignored dap_`), so that no real debug build is refused.
- [x] 7.2 `crates\driver\tests\cli.rs` `dap_args`: `args` `a`, `b c` and `d\` reach the program's `COMMAND$` as they
  are through `lldb-dap`. Verify: `--ignored dap_args`.
- [x] 7.3 `package.json`: `onDebugDynamicConfigurations` dropped (no dynamic provider; design D1).
- [x] 7.4 `debug.test.ts`: the refusal test does not need the first test's executable. (The suite still puts back
  `qb64rust.buildWith` `qb64pe`, the test workspace's value, as `buildQb64rust.test.ts` does: its `after` runs
  also when `before` skips the suite.) Verify: `npm run lint`; the debug suite passes.
- [x] 7.5 Reruns for these edits (working rule 9): tier 1 with fmt and clippy, the CLI tests (`--ignored`), the
  extension's unit tests and the debug suite; `close.py --check`, then `test_time.py`.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
- At the merge into `main`: `ROADMAP.md` M5's fourth item ticked and M5 declared done with the user, a
  `DECISIONS.md` row (working rule 11).
