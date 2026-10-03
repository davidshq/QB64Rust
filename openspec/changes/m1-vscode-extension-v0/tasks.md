# Tasks

## 1. Project setup

- [x] 1.1 Create `vscode\` as an npm project (`package.json` with name `qb64rust`, publisher `qb64rust`, engine
  `^1.85.0`, raised to `^1.100.0` on 2026-10-03 for mocha 12 and `TextDocument.encoding`, MIT), `tsconfig.json` (strict, `out\`), ESLint config; add `vscode/node_modules/` and `vscode/out/` to
  `.gitignore`. Verify: `npm ci` and `npx tsc -p .` succeed on an empty `src/extension.ts` with `activate`.
- [x] 1.2 Add dev dependencies (`typescript`, `@types/vscode`, `@types/node`, `mocha`, `@types/mocha`,
  `@vscode/test-electron`, `@vscode/vsce`, `vscode-tmgrammar-test`, ESLint packages) and the runtime dependency
  `iconv-lite`; scripts `compile`, `lint`, `test:unit`, `test:grammar`, `test:integration`, `package`. Verify:
  each script runs (empty test suites pass).
- [x] 1.3 Write `vscode\README.md` (what M1 does, requirements, settings, coexistence via `files.associations`)
  and a short `vscode\DEVELOPMENT.md` (build, test, record fixtures, package). Verify: the commands in
  `DEVELOPMENT.md` run as written.

## 2. Language support (`specs/editor/language-support`)

- [x] 2.1 Contribute language `qb64rust` (display name "QB64", `.bas .bi .bm`, no `BASIC`/`QBasic` aliases) and
  `language-configuration.json` (`'` comment, `()` brackets, `"` auto-close, folding markers for
  SUB/FUNCTION/TYPE). Verify: integration test opens a `.bas` and a `.bi` fixture and asserts `languageId ===
  'qb64rust'`.
- [x] 2.2 Write `syntaxes/qb64rust.tmLanguage.json` per design D8 (strings, comments, REM, metacommands in
  comments, numbers with suffixes and `&H/&O/&B`, labels/line numbers, keywords taken once from
  `tools\builtins\builtins.json`, plus `_TRUE`, `_FALSE` and the color constants by hand). Verify: `npm run test:grammar` passes annotated fixtures covering every grammar
  scenario in the spec (comment forms, `'$INCLUDE`, `"it's"`, keyword case).
- [x] 2.3 Add `configurationDefaults` for `[qb64rust]` (`files.encoding: cp437`, `files.autoGuessEncoding:
  false`). Verify: integration test opens a fixture holding bytes 128–255 in a string, saves it unchanged, and
  compares bytes with the original; a second test with a workspace override to `utf8` sees UTF-8 decoding.

## 3. Compiler output parser (`specs/editor/compiler-diagnostics`, design D2, D9)

- [x] 3.1 Write `vscode\scripts\record-fixtures.sh` and fixtures in `vscode\test-fixtures\compiler\`: error in
  main file, error in `$INCLUDE`, missing include, unused-variable warning, clean file, `-y` success, `-y` error,
  and a `-x` output with progress bar. Record `.out.txt` and `.exit` from `..\QB64pe\qb64pe.exe`, with absolute
  paths replaced by a placeholder. Verify: rerunning the script reproduces the fixtures with no diff.
- [x] 3.2 Implement `src/compiler/parseQb64peOutput.ts` (pure, no `vscode` import) per D2, including the
  unrecognised-output fallback. Verify: `npm run test:unit` covers every fixture and the spec scenarios (include
  location, `LINE n` of the `$INCLUDE` ignored, warning detail appended, progress bar ignored, CRLF and LF).

## 4. Compiler discovery and process handling (design D1, D4)

- [x] 4.1 Implement `config.ts` (including one function that returns a document's encoding, design D10) and `compiler/discovery.ts` (setting `qb64rust.compilerPath`, then `PATH`;
  re-run on setting change) and `statusBar.ts`. Verify: unit tests with a fake file system / `PATH`; integration
  test sets the setting and sees the language status text change without reload.
- [x] 4.2 Implement `compiler/runQueue.ts`: one process at a time, per-file cancellation of checks, process-tree
  kill, timeout (`qb64rust.checkTimeoutSeconds`). Verify: unit tests with a fake long-running process (a Node
  script) for ordering, cancellation and timeout.
- [x] 4.3 Implement `compiler/qb64pe.ts` as plain functions, no interface type (`check` with `-z -q -w`, `format`
  with `-y -q`, `build` with `-c -x -w`, `-o` into a per-session temp folder for check/format). Verify: integration tests
  against `..\QB64pe\qb64pe.exe` (skipped if missing) for a clean and an erroneous fixture.

## 5. Diagnostics (`specs/editor/compiler-diagnostics`)

- [x] 5.1 Implement `diagnostics.ts`: check on save (`qb64rust.checkOnSave`, default true), command
  `qb64rust.check`, range from first to last non-blank character, include-file targets, clear on edit and close,
  untrusted workspaces skipped. Verify: integration tests for each spec scenario (error on save, fixed on save,
  disabled, error in `.bi`, unused-variable warning, indented range, edit clears, rapid saves show only the last
  result).

## 6. Formatting (`specs/editor/formatting`, design D5)

- [x] 6.1 Implement `format.ts`: sibling temp file, CP437 via `iconv-lite` using the document's encoding, EOL
  conversion, one full-range edit, errors → message + diagnostics, temp file removed in `finally` and stale ones
  removed on activation. Verify: integration tests for the spec scenarios (indent/keyword case, unsaved edits,
  relative include, no temp file left, LF document stays LF, `╔══╗` survives, syntax error leaves the document
  unchanged, no reformat on save by default).

## 7. Build and run (`specs/editor/build-run`, design D6)

- [x] 7.1 Implement `build.ts`: commands `qb64rust.build`, `qb64rust.run`, `qb64rust.buildAndRun`, output
  channel "QB64", save-before-build, untitled prompt, errors to Problems, run in terminal "QB64" with
  `qb64rust.runArguments` and optional `qb64rust.noPrompt`, editor title run-menu entries, untrusted-workspace
  refusal. Verify: integration tests build a fixture and assert the exe exists; a failing fixture shows the error
  in Problems and does not run; Run without exe offers to build; manual check of arguments via a fixture that
  prints `COMMAND$`.
- [x] 7.2 Implement `tasks.ts`: task type `qb64rust` with `action: build | run` and optional `file`. Verify:
  integration test fetches tasks of type `qb64rust` and runs the build task on a fixture.

## 8. Integration and wrap-up

- [ ] 8.1 Package with `npm run package`, install the `.vsix` in VS Code, and walk through the scenarios by hand
  on a real program from `..\QB64pe\tests\` (open, highlight, check on save, format, build and run). Verify: a
  short record of the walkthrough in `verification\m1-extension.md`.
- [ ] 8.2 Do the open CP437 round-trip check from `study\09` (last section) with the installed extension and
  record the result there.
- [ ] 8.3 Update `STATUS.md` (M1 done, next steps), `study\00-synthesis.md`, and `CLAUDE.md`'s layout table
  (`vscode\`). Verify: `openspec validate m1-vscode-extension-v0 --strict` passes.
