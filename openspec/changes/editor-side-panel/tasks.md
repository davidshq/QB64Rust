# Tasks

Standing rules: `CLAUDE.md`. Different here: the look of each screen is checked by the user (task 8.2), since
no test can judge it. The extension's integration suite runs in CI; on the host run its unit tests.

## 1. Before the build

- [x] 1.1 Check two outside facts and write what was found into `design.md`, "Context": the manifest form of a
      webview view and of a walkthrough (VS Code's contribution points page), and `markdown-it`'s license (its
      repository). Done when the "Not checked" line of `design.md` is gone.

## 2. The help data

- [x] 2.1 `tools\reference\reference.py`: the command `help --out <dir>` writes `pages\`, `index.json` and
      `examples\` (design D4). Tests in the tool's own test file: the index holds `_clamp` with its title and
      first sentence, every page named in the index exists, and every example named exists with its output.
- [x] 2.2 `reference.py render` writes `crates\lsp\src\help_table.rs`, and `--check` fails when it is stale
      (design D6). Verify by editing the first sentence of one page: `--check` fails, `render` fixes it.
- [x] 2.3 Write `docs\reference\first-steps.md` and the twelve programs of `examples\` with
      `examples\index.toml` (design D10). A CLI test in `crates\driver\tests\it\` checks each program with
      `qb64rust -z` and expects no diagnostic; the console ones get a recorded output. `reference.py --check`
      passes with the new page. Add the rows of `examples\` and the page to `docs\LAYOUT.md`.
- [x] 2.4 Update `tools\reference\README.md` for the `help` command and the generated table. Verify that each
      command it names runs as written.

## 3. The language server

- [x] 3.1 `crates\lsp\src\hover.rs`: the first sentence after a built-in's arguments, and for a keyword with a
      page (spec `editor/language-server`, "Hover"). Tests for the three new scenarios; the old "A keyword"
      test changes to expect the sentence of `FOR`. `cargo test -p qb64rust-lsp` passes.
- [x] 3.2 `qb64rust/procedures` in `symbols.rs` and `server.rs` (design D3). Tests: a procedure of an include
      file, the request from the include file, unsaved text, and a syntax error inside one SUB.
- [x] 3.3 `qb64rust/helpTopic` in `server.rs`, from `help_table.rs`. Tests: a built-in, `CASE` of `SELECT
      CASE`, the program's own SUB, and a comment.
- [x] 3.4 Reference pages: none changes, since no word of the language changes. Run `reference.py --check` to
      confirm.

## 4. The side panel

- [x] 4.1 `vscode\package.json`: the QB64 view container, the four views and an icon file. An integration test
      finds the four views by id.
- [x] 4.2 `vscode\src\panel\programModel.ts` (design D2) with unit tests for each state: no program, OK, OK
      with warnings, an error in the main file, errors in two files, building, the first build, the toolchain's
      download, and an untrusted workspace.
- [x] 4.3 `programView.ts`: the webview view, its HTML as one function of the state, and the four buttons. A
      unit test compares the HTML of each state of 4.2. An integration test clicks nothing: it sends the
      "run" message to the provider and sees the build command run.
- [x] 4.4 `proceduresView.ts` and the client side of `qb64rust/procedures` in `server\protocol.ts`. Unit tests
      for the model (both orders, the main program's row). An integration test opens a fixture with an include
      file and reads the tree's rows.
- [x] 4.5 The two status bar items in `statusBar.ts`, from the same state. A unit test for the text of each
      state.
- [x] 4.6 `vscode\README.md`: a section "The QB64 side panel". Verify that each view and button it names is in
      `package.json`.

## 5. Help and examples

- [x] 5.1 `vscode\src\help\helpIndex.ts` and `renderPage.ts` (design D5), with `markdown-it` added to
      `package.json`. Unit tests: the search finds `_CLAMP` by "bounds"; a page's HTML holds its example, its
      recorded output and the three buttons; a link to another page becomes a message; a web link stays.
- [x] 5.2 `helpPanel.ts`, `media\help.css`, `media\help.js`: the tab beside the code, Back and Forward, the
      content security policy. An integration test opens `_CLAMP`, then `_MIN`, and reads the panel's title.
- [x] 5.3 `helpView.ts` and the commands "QB64: Search Help" and "QB64: Help for This Word", with the fallback
      by spelling when the server is off. No key is bound; the command is in the editor's right-click menu for
      QB64 files, and a unit test finds no `keybindings` entry in the manifest. Integration tests for
      the three results of "Help for This Word".
- [x] 5.4 `examplesView.ts` and `examples\runExample.ts` (design D7). Unit tests for the tree and for which
      examples get no Run button. An integration test opens "Hello" and finds an unsaved QB64 editor with its
      text.
- [x] 5.5 `vscode\README.md`: sections "Help" and "Examples"; the hover's line in "Language features". Change
      the `SOMEDAY.md` rows of group `f1-help`: the link row and "help text of our own" are built, the offline
      wiki row stays.

## 6. New Program and the walkthrough

- [x] 6.1 `vscode\src\newProgram.ts` (design D8, spec `editor/new-program`). Unit tests for the free name and
      for the folder the dialog opens in (a workspace folder, the folder last used, one that is gone, the
      documents folder). Integration tests, with the file given in place of the dialog as `newProject`'s tests
      do: a name given, cancel, a name without `.bas`, a folder with a UTF-8 `qb64rust.toml`, a plain folder.
- [x] 6.2 The walkthrough in `package.json` and `vscode\walkthrough\*.md`, and "QB64: Get Started" (design
      D9). A unit test reads the manifest and finds every step's command among the contributed commands. Try
      each step by hand in VS Code 1.100 and the current release, and write into `design.md` when each is
      ticked. Done by a script in 1.100.0 and 1.146.0 (the user gave it to Claude, 2026-11-25); it found that
      `onCommand:` alone ticks nothing the extension runs itself, so each step has a context key too (D9).
- [x] 6.3 `vscode\README.md` and `docs\QUICKSTART.md`: the first program is now made by the walkthrough or by
      "QB64: New Program". Verify the steps of `QUICKSTART.md` by following them in a development host. Done by
      a script in 1.146.0: New Program, the first build ("Building the QB64 runtime…"), the run, the mistake
      shown with its line, the fix, a debug session stopped at a breakpoint, help and hover on `PRINT`. The step
      "Fix an error" was wrong (a string with no closing quote is no error) and now uses a missing `)`.

## 7. Packaging

- [x] 7.1 `vscode\scripts\stage_bundle.py` runs `reference.py help` into `vscode\help\`; `.gitignore` and
      `.vscodeignore` are set for it; `npm run help` does the same for a development copy. The package test
      (`vscode\scripts\test_package_download.py` or its neighbor) lists the `.vsix` and finds the index, the
      page of `_CLAMP`, `_clamp_1.bas` and "Hello".
- [x] 7.2 Measure the `.vsix` before and after, and write both sizes into `design.md`, "Risks".

## 8. Closing

- [x] 8.1 `cargo test -p qb64rust-lsp`, the extension's unit tests and lint, and `just quick`. Say what ran and
      what stood.
- [x] 8.2 The user looks at each screen in a light, a dark and a high-contrast theme: the Program view in each
      state, the help tab, the walkthrough. Fix what the user reports. Done when the user says the look is
      accepted. Given to Claude by the user (2026-11-25): 32 screenshots in four themes, three faults fixed and
      shot again (design, "Risks"). Not shot: the untrusted state, since the test window runs with workspace
      trust off; its page is the unit test's.
- [x] 8.3 List the compiler's messages for twenty common beginner's mistakes (a missing `THEN`, an unclosed
      string, `NEXT` without `FOR`, and so on) as one `SOMEDAY.md` row with the list in `study\55`, for a later
      change of the wording.
- [x] 8.4 Update `STATUS.md`'s "Next" and mark questions 1, 2 and 4 of `study\55` §7 as built.

## 9. After the user tried it (2026-11-26)

- [x] 9.1 The SUBs & FUNCTIONs view is shown only for a program with an include file
      (`qb64rust.procedures.multiFile`), and its rows show the arguments. Unit tests for `hasIncludeFiles` and
      the labels; an integration test for a program of one file.
- [x] 9.2 A click on an example opens the user's copy in the extension's storage as a file
      (`vscode\src\examples\exampleCopy.ts`), an edited copy is kept, and "Reset This Example" is in the row's
      menu. Unit tests for the copy; the integration test expects a file that is not unsaved.
- [x] 9.3 The Program view: "Help on IF", a moving bar and "about a minute" for the first build, and a "Search
      Help" link. "First steps" starts open in the Examples view. Unit tests for each.
- [x] 9.4 The page "QB64 Home" (design D11, spec `editor/home`): `vscode\src\home\`, the setting
      `qb64rust.showHomeOnStartup`, the command "QB64: Home" and the activation event `onStartupFinished`. Unit
      tests for when it opens, the recent programs, the setup's rows and the page's HTML; an integration test
      opens it by its command. The page was looked at once, drawn in headless Chrome with a dark theme's colors.
- [ ] 9.5 The user looks at the panel and the Home page in the installed extension, in a window with no folder,
      a folder with QB64 files, and a folder of another project.

## Workflow follow-up

- Archive the change once the extension's integration suite has passed in CI and task 8.2 is done.
- Questions 5 to 8 of `study\55` §7 stay open; the study stays active until they are answered.
