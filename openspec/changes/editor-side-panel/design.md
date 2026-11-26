# Design

## Context

See `proposal.md` for the reasons, and `study\55-mockups\index.html` for what the screens look like.

What is there today, read on 2026-11-25:

- `vscode\src\extension.ts` builds the parts (`Builder`, `DebugSupport`, `BrowserSupport`, `LanguageServer`,
  `CompilerStatus`, `ToolchainSupport`) and hands them to each other. None draws a view.
- The language server tells the client each document's program (`qb64rust/program`, kept by `ProgramTracker` in
  `vscode\src\server\protocol.ts`). Build, Run and Debug already act on that program.
- `RunQueue.onBusyChange` reports "check", "format" or "build". `build.ts` line 105 sets the text "building libqb
  (first build)…" itself.
- Every reference page starts with a title and a summary before its first heading; `reference.py --check`
  refuses a page without them. A page's file name comes from the word by one rule (`reference.py`, the page
  file of a word): `_CLAMP` is `_clamp.md`, `SELECT CASE` is `select-case.md`, `CHR$` is `chr-str.md`.
- The reference's 333 examples are `tests\reference\examples\<page>_<n>.bas` with a recorded `.output`. 30 have
  an `.external` sidecar (a clock, a file, a device), 5 a `.files` folder, 2 a `.stdin`.
- "QB64: New Project" (`vscode\src\newProject.ts`) asks for a folder and writes `main.bas`, `qb64rust.toml` and
  `.vscode\settings.json`.

Constraints from VS Code, with their sources (`CLAUDE.md` working rule 17):

- View containers go in the activity bar; a view is a tree or a webview
  ([Contribution Points](https://code.visualstudio.com/api/references/contribution-points)).
- A walkthrough step is ticked by a completion event such as `onCommand:`; without one it is ticked when its
  button is clicked (same page). Others report that `onCommand` works only when the step has a button
  ([discussion 500](https://github.com/microsoft/vscode-discussions/discussions/500)).
- An empty window is trusted by default (`security.workspace.trust.emptyWindow`,
  [Workspace Trust](https://code.visualstudio.com/docs/editing/workspaces/workspace-trust)).
- VS Code asks extensions to use webviews sparingly and to limit webview views
  ([Webviews](https://code.visualstudio.com/api/ux-guidelines/webviews),
  [Views](https://code.visualstudio.com/api/ux-guidelines/views)).
- VS Code's save dialog opens in the workspace folder, else by the last active file, else in the home folder or
  `files.dialog.defaultPath` ([release notes of 1.80](https://code.visualstudio.com/updates/v1_80),
  [pull request 182908](https://github.com/microsoft/vscode/pull/182908)).
- A view in `contributes.views` has `"type": "tree"` (the default) or `"type": "webview"`, and a webview view's
  provider is registered with `registerWebviewViewProvider` under the view's `id`. Read on 2026-11-25 in the
  manifest schema of VS Code 1.100 (its `nls.messages.json`: "Type of the view. This can either be `tree` … or
  `webview` …") and in `@types/vscode` 1.100.
- A walkthrough step has `id`, `title`, `description` (a command link on a line of its own is drawn as a
  button), `media` (`{ "markdown": "<path>" }`, "Path to the markdown document, relative to extension
  directory") and `completionEvents` (`onCommand:<id>` "Mark step done when the specified command is
  executed"). Without completion events a step is ticked when a button or link of it is clicked. Same schema.
- `markdown-it` is MIT. Its repository is [markdown-it/markdown-it](https://github.com/markdown-it/markdown-it),
  with a `LICENSE` file; npm and [licenses.dev](https://licenses.dev/npm/markdown-it/14.3.0) list release 14 as
  MIT. The installed copy's `package.json` is read again when it is added (task 5.1).

## Goals / Non-Goals

**Goals:**

- Every new view has a model with no `vscode` import, tested with mocha, as `buildPlan.ts` is.
- Native parts wherever they can do the job. Two webviews only: the Program view and the help tab.
- What ships as help is made from `docs\reference` by the tool that already checks it. No second copy to keep.

**Non-Goals:**

- No change to what Build, Run, Debug or Run in Browser do. The views call the existing commands.
- No new key for Run. F5 debugs (user, 2026-11-25).
- No rendering of the QB64pe wiki, and no fetch from the network.
- No rewording of the compiler's messages. The status line shows a message as the compiler wrote it.

## Decided by the user

Both questions were answered on 2026-11-25 (`DECISIONS.md`): "B, and no key for F1". The specs and tasks are
written to the answers. The options and the panel's view are kept below as the record.

**Question 1. How does "New Program" ask for its folder? Answer: B.** The user had asked what VS Code itself
does: a new file is unsaved, and a save dialog asks at the first save. B is the nearest to that which still
gives a file on disk at once. The user ruled on 2026-11-25 that the folder is
never set without asking, and that a default is fine. Three ways meet that:

- **A. Ask once, then remember.** The first time, a question shows the default's full path with "Use this
  folder" and "Choose another folder". The answer is kept in the setting `qb64rust.programsFolder`. Later
  programs ask only for a name. "QB64: Choose Programs Folder" changes it. Inside an open folder the program
  goes into that folder and the question is never put.
- **B. A save dialog every time**, opened in the default folder or in the folder last used. Nothing is
  remembered as a setting. It costs one more dialog for each program.
- **C. An unsaved editor.** New Program opens an untitled file. The first Run puts the save dialog. This is
  how Notepad and QBasic work. A check on save and a build need a file on disk, so the first Run always starts
  with a dialog, and an unsaved program has no `qb64rust.toml`, so it is built with the fixes off.

*The panel:* A, 7 to 1. The teacher: one question on the first day and none after is the least a beginner can
be asked while still being asked. The user-experience designer: A shows the full path before anything is
written, which is the control the user asked for. The VS Code extension engineer preferred B as the form VS
Code users know. The QB64 engineer accepted A because C builds a new program with the fixes off, against the
decision that a new project has them on. The user chose B, the VS Code extension engineer's view.

**Question 2. Does F1 open help in a QB64 file? Answer: B, no key.** F1 is VS Code's key for the Command Palette, as Ctrl+Shift+P
is. The QB64pe IDE and QBasic use F1 for help.

- **A. F1 is help in a QB64 file.** Ctrl+Shift+P still opens the Command Palette, and F1 does in every other
  file. The user can unbind it in Keyboard Shortcuts.
- **B. No key.** Help is reached by the hover's link, the editor's right-click menu and the command.
- **C. Another key**, such as Ctrl+F1.

*The panel:* A, 6 to 2. F1 for help is as old as BASIC on a PC, and the palette keeps its main key. The VS Code
extension engineer and the pragmatic engineer were for B, by the reasoning the user gave for F5: do not change
what a VS Code key does. The specs name only the command, so either answer leaves them as they are; the answer
changes one `keybindings` entry in `package.json` and one walkthrough sentence.

## Decisions

### D1. One webview view, three tree views

The Program view is a webview view, because a tree view cannot hold a large button and a status box. VS Code's
`viewsWelcome` draws buttons, but only in a tree that is empty. The other three views are tree views. Their
search is a title button that opens a quick pick, since a tree view has no text box.

*Where:* `vscode\src\panel\programView.ts` (the provider) and `programModel.ts` (the model: program, state,
status line, with no `vscode` import). `proceduresView.ts`, `examplesView.ts`, `helpView.ts`, each with a model
beside it. `package.json` gets `viewsContainers.activitybar`, `views` and `menus` for the title buttons.

*Alternative:* all four as webviews, drawn as the mockup. Refused: three more webviews to theme and to test by
eye, for a look the tree already gives.

### D2. The Program view's state has one source

`programModel.ts` takes four inputs and gives one state. The inputs: the program (`ProgramTracker`, else the
active `.bas` file), the busy kind (`RunQueue.onBusyChange`, and the builder's own "first build" and the
toolchain's download, which get an event each), the diagnostics of the program's files
(`vscode.languages.getDiagnostics`), and workspace trust. The status bar item (`statusBar.ts`) reads the same
state, so the two never disagree.

The first error is the lowest by (main file first, then file name, then line). A file belongs to the program
when it is the main file, when the `qb64rust/procedures` answer lists it among the program's files, or when the
server named that program for it. The list is needed for a program of many files: the server names a program
only for an open document, but its parse of the program publishes errors for every included file. Civil War
Strategy's `cws.bas` includes 47 files in folders, and an error in a closed one would otherwise leave the line at
"OK" (the user asked how a program of many files is handled, 2026-11-25).

The webview gets the state as one message and sends back only the name of a button. It holds no logic. Its HTML
is one function of the state, so a unit test compares the string for each state.

### D3. The procedures come from a request of our own

`qb64rust/procedures { uri }` gives `{ main, files, procedures: [{ name, kind, params, uri, range }] }`, where
`files` is every file of the program in the order it reads them, open or not (D2 counts their errors). The client
asks again, 300 ms after the last change, when a document of the program changes or the program notification
arrives.

The server answers from the text as edited. So while a parse or a check of the program is due, the request
waits for it, and the view keeps its old rows meanwhile. A check after a save can take a while: `qb64rust -q -z`
on Civil War Strategy's `CWSTRAT.BAS` (5,075 lines) took 1.2 to 1.7 s on the host (2026-11-25). That wait was
kept, since an answer from the parse before the save could list procedures the user has just removed.

*Where:* `crates\lsp\src\symbols.rs` (a function over the program's `Analysis`, beside `document_symbols_in`),
`crates\lsp\src\server.rs` (the request), `vscode\src\server\protocol.ts` (its types).

*Alternative:* `workspace/symbol`. Refused: it answers a text query over every file the server knows, and this
view wants one program's procedures in order.

### D4. The help is made by `reference.py help`

A new command writes a folder:

- `pages\<name>.md`: each entry, chapter and appendix page, as it is in `docs\reference`.
- `index.json`: for each page its name, title, first sentence, kind (keyword, chapter, appendix), and its
  examples by name. Also the tree of the Help view and the groups of the Examples view.
- `examples\`: the reference's examples (`.bas` and `.output`) and the programs of `examples\`.

`vscode\scripts\stage_bundle.py` runs it into `vscode\help\`, which is not committed (`.gitignore`) and is in
the package. `npm run help` runs it for a development copy, and the integration suite runs it first.

*Alternative:* commit the folder. Refused: 3.3 MB that repeats `docs\reference`, and a generated copy that a
page edit makes stale.

### D5. A help page is drawn by `markdown-it` in the extension host

The help tab is a webview panel. The extension host turns the page's Markdown into HTML with `markdown-it`
(MIT, to be confirmed in task 1.1) and sends it to the webview. The webview has a content security policy that
allows no remote content and one script of ours. Links between pages are rewritten to messages. Colors are VS
Code's theme variables. A fenced `qb64` block is colored by a small tokenizer of our own (comments, strings,
numbers, and the words of the keyword pages' titles), not by the TextMate grammar, which a webview cannot reach.
As built, the tokenizer runs in the extension host, in `renderPage.ts`, not in the webview: the webview then
holds no logic, as D2 asks of the Program view, and the coloring is unit-tested with the rest of the page.

*Where:* `vscode\src\help\helpPanel.ts`, `helpIndex.ts` (reads `index.json`, the search, no `vscode` import),
`renderPage.ts` (Markdown to HTML, the example buttons, no `vscode` import), `vscode\media\help.css` and
`help.js`.

*Alternatives:* VS Code's own Markdown preview of the bundled file. Refused: it has no place for the example's
buttons, and Back and Forward would not be ours. A converter of our own in `reference.py`. Refused: a Markdown
converter is code to keep, for what a library does.

### D6. The first sentences are a generated table in the language server

`reference.py render` also writes `crates\lsp\src\help_table.rs`: for each word of a page, the page's name and
its first sentence. `reference.py --check` fails when it is stale, as for the other generated blocks. `hover.rs`
appends the sentence. `qb64rust/helpTopic` answers from the same table, so the server and the extension agree
on page names without sharing the naming rule.

*Why in the server:* it knows what the token is (a built-in, a keyword, the program's own name), and any other
editor that speaks to `qb64rust lsp` gets the richer hover too.

*Without the server* the command falls back to the word's spelling against `index.json` (spec `editor/help`,
"Help for the word under the cursor").

*Cost:* about 400 sentences in the binary, some tens of kilobytes.

### D7. An example runs from the extension's storage

An example has one copy for the user: the program, and the contents of its `.files` folder when it has one, in
`<globalStorage>\examples\<name>\`. A click in the Examples view opens that copy as a file, and "Run this
example" and the view's "Run" call the builder on it. The build's output stays there.

The first build opened an example as an untitled document. The user refused that (2026-11-26): closing an
untouched example asked to save it, and Run asked to save first. A real file closes without a question and
runs at once. Beside the copy, `.original` holds the shipped text it was made from. A copy equal to it was not
edited and is written again when the shipped example changes; any other copy is the user's and is kept.
"Reset This Example" writes the copy again. The folder is never emptied, since a program started from it may
still run. An
example with an `.external` sidecar or a `.stdin` runs like any other; the sidecars are for the test runner.
Network examples (a sidecar that starts with `network:`) get no Run button, since they make Windows ask about
the firewall.

*Where:* `vscode\src\examples\exampleCopy.ts` (the copy's rules, free of VS Code) and `runExample.ts`.

### D8. New Program

`vscode\src\newProgram.ts`, beside `newProject.ts`, whose `qb64rust.toml` and settings file it writes
nothing of: the command writes one file. It calls `vscode.window.showSaveDialog` with a `.bas` filter and a
`defaultUri` of folder and free name (spec `editor/new-program`, "Where the save dialog opens"). The documents
folder is `Documents` under the user's home; when the home has none, the home itself. The folder last written to
is kept in the extension's `globalState`, not in a setting. The dialog itself asks before a file is replaced.
The file is opened as UTF-8 when its folder's `qb64rust.toml` says so, as `newProject.ts` does, because a
folder's `.vscode\settings.json` applies only when the folder is the workspace. The command does not open the
file's folder as the workspace: that reloads the window and would end the walkthrough.

A consequence of B: a program saved into a folder with no `qb64rust.toml` builds with the fixes off and is
saved as code page 437, as an existing program is. No source line turns the fixes on; only `qb64rust.toml` and
the command line do (spec `compiler/fixes`). "QB64: New Project" stays the way to a folder with the fixes on.

### D9. The walkthrough

One `contributes.walkthroughs` entry. Each step has a button (a command link) and an `onCommand:` completion
event. The media of each step is a Markdown file in `vscode\walkthrough\`, with no image, so it follows every
theme. The step "Run it" has two texts chosen by `when` (`isWindows`, `isLinux`). "QB64: Get Started" calls
`workbench.action.openWalkthrough` with the walkthrough's id.

The step "Find and fix a mistake" opens the "Hello" example with one line broken and points at the Program view's
status line. As built, the mistake is a `(` never closed on line 3, not a misspelled `PRINT`: `PRNT "x"` is
reported only by the check on save ("`PRNT` is no statement and no SUB", from `sema`), so an unsaved editor would
show nothing, while the missing `)` is a parser error shown as the user types ("expected `)`").

As built, the first step is ticked by `onContext:qb64rust.walkthrough.madeProgram`, set once New Program has
written a file, so that a cancelled dialog does not tick it (spec: "ticked once it has made a program").

Measured in task 6.2 (2026-11-25, VS Code 1.100.0 and 1.146.0, a test instance driven by a script): with only
`onCommand:` events, steps 2 to 5 stayed unticked although their commands ran. The script ran them as the
extension does when its own buttons are used (the Program view's Run, the status bar, the help's links), and
such a call stays in the extension host, so the event never fires. My reading of why; VS Code's source was not
read. So each step also has a context key, `onContext:qb64rust.walkthrough.<step>`, set by the code that does the
step's work: the program started in its terminal (`ran`), a help page shown (`lookedUp`), an example opened or
run (`triedExample`), the mistake opened (`openedMistake`), a page opened in the browser (`ranInBrowser`). With
them, steps 1 to 5 were ticked in both versions. A click on a step's own button was not tried: the script
cannot click.

### D10. The new examples and the "First steps" page

`examples\<group>\<name>.bas`, with `examples\index.toml` for titles and order. About twelve programs: three
first steps (Hello, Guess the number, Draw with the mouse), three graphics, two sound, four small games. Each is
written here, in a genre and not after a named game. A CLI test (`crates\driver\tests\it\`) checks each with
`qb64rust -z` and expects no diagnostic. The console ones also get a recorded output.

`docs\reference\first-steps.md` is a page of about 150 lines: a program, a variable, `INPUT`, `IF`, a loop, a
SUB, a drawing. Every code block is one of the new examples, so `reference.py` checks it as it checks the
others.

### D11. QB64 Home

Added on 2026-11-26 after the user tried the side panel (user: "what about opening the main window 'starter'
screen? And have something like a 'hide this' checkmark"; then "build it, panel's choice, include recent
programs"). It is mockup 4 of `study\55`, which the first plan left out.

One webview panel, "QB64 Home", in the editor area. As the Program view, its state is one function of its
inputs and its HTML one function of the state, so both are unit-tested without VS Code.

- **When it opens by itself.** The setting `qb64rust.showHomeOnStartup` is on, no file is open, and the window
  has no folder or a folder with a `.bas`, `.bi` or `.bm` file (`shouldOpenHome`). A tab that is not a file,
  such as VS Code's Welcome page, does not count as open. "QB64: Home" opens it at any time.
- **The checkbox** writes that setting for the user (the global target), so it holds in every window.
- **Recent programs** are 8 paths in the extension's `globalState`, newest first. A program is noted when the
  Program view names it, so an include file notes its main file. A copy of an example in the storage is not
  noted, and a file that is gone is not shown.
- **The setup's rows** come from what the extension already looks up: the binary's discovery, the toolchain's
  lookup and the Emscripten SDK's.

*The cost.* To open at startup the extension has the activation event `onStartupFinished`. So it now starts
in every window, a window of another project too, and with it the language server's process. No activation
event names "a window with no folder", so a narrower event would lose that case. Starting the language server
only in a QB64 window is a `SOMEDAY.md` row: it changes when builds find the binary, and the integration tests
that would show a mistake run only in CI.

*Not measured:* whether VS Code opens the walkthrough by itself after an install. If it does, the first start
shows both, the Home page in front.

*Where:* `vscode\src\home\homeModel.ts` (the rules), `homeHtml.ts` (the page), `homePanel.ts` (the panel),
`vscode\src\config.ts` (the setting).

## Risks / Trade-offs

- [The look cannot be tested.] → Each webview's HTML is compared as a string, and the user looks at each screen
  in a light, a dark and a high-contrast theme before the change is archived (task 8.2). As done (the user gave
  the task to Claude, 2026-11-25): screenshots of a VS Code 1.146 test window in Default Light Modern, Default
  Dark Modern, Default High Contrast and Default High Contrast Light, read one by one. They found three faults,
  fixed and shot again: VS Code shows a view's name in title case ("SUBs And FUNCTIONs", now "SUBs &
  FUNCTIONs"); the Program view's pane was too short for an error, which scrolled (now `initialSize` 3 to the
  others' 2, and a tighter page); and a message's backticks showed as text (now drawn as code).
- [A walkthrough step is ticked too early or never.] → Community reports disagree with the documents. Task 6.2
  tries each step by hand and records what VS Code 1.100 and the current release do.
- [The compiler's messages are not written for a beginner.] → Out of scope here. Task 8.3 lists the messages of
  twenty common mistakes for a later change.
- [Two webviews stay to be maintained.] → Both hold no logic: each draws a state it is sent.
- [`help_table.rs` goes stale.] → `reference.py --check` is a step of `just quick`.
- [The package grows by about 3.3 MB.] → Text, before compression. Measured in task 7.2 (2026-11-25, `vsce
  package --target win32-x64` on this branch): 10,400,568 bytes without `help\` and 11,103,798 bytes with it, so
  the help adds 703,230 bytes (0.7 MB, 7 %) and 1,134 files. `help\` itself is 3.2 MB on disk. Both packages
  hold `markdown-it` and its six dependencies, 1,185,000 bytes before compression. For comparison, the release
  `vscode-v0.3.0` package was 9,786,762 bytes.
- [Help has no key, so a newcomer may not find it.] → The hover names the command, the editor's right-click
  menu has "QB64: Help for This Word", and the walkthrough's third step shows it. A user can bind a key.
- [A new program outside a project builds with the fixes off.] → D8 says why. The walkthrough says nothing of
  fixes; `vscode\README.md` names "QB64: New Project" for them.

## Open Questions

- A key for Run. F5 debugs, and Ctrl+F5 ("Run Without Debugging") reaches the debug adapter with `noDebug`,
  which the extension does not handle today. Making Ctrl+F5 run the program is a small change of its own.
