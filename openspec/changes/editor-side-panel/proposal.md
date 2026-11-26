# Proposal

## Why

The extension can check, build, run and debug a program, but it shows none of that. It adds no icon, no view and
no page: its nine commands are in the Command Palette, and one small run button sits at the top right of the
editor. A hover on `_CLAMP` gives its form and no word of what it does, and nothing opens help. The project has
401 keyword pages, 39 chapters and 333 example programs of its own, and none of them ships in the extension.

The user asked for this on 2026-11-25: "Part of BASIC is how easy and intuitive it is to use, and VSC doesn't
feel 100% easy and intuitive." The panel review is `study\55-editor-experience.md`; its mockups are
`study\55-mockups\index.html`. The user chose mockups 1 to 3, in the one existing extension, with F5 left as it
is (`DECISIONS.md` 2026-11-25).

## What Changes

- **A QB64 side panel.** One icon in the activity bar opens four views:
  - **Program**: the name of the program that Run builds, the buttons Run, Debug, Run in Browser and Build, and
    one status line. The line says "OK", or the program's first error with a link to its line and to help, or
    what a build is doing.
  - **SUBs and FUNCTIONs**: every procedure of the whole program, include files too. A click goes to it.
  - **Examples**: programs to open as a copy and run.
  - **Help**: the language reference as a tree, with a search.
- **A status bar item** with a Run button and the same "OK" or error, for when the side panel is closed.
- **Help beside the code.** A help tab shows a reference page to the right of the code. Its examples have "Run
  this example" and "Open as a new program". A hover on a built-in or keyword gains the first sentence of its
  page. Help gets no key: F1 stays VS Code's (user, 2026-11-25). It is a command, and it is in the editor's
  right-click menu.
- **The reference and the examples ship in the extension.** They work offline. No QB64pe wiki text is used.
- **A walkthrough**, "Get Started with QB64", in VS Code's own Welcome page: write a program, run it, look a word
  up, try an example, fix a mistake, put it on the web.
- **"QB64: New Program"** makes one `.bas` file, ready to run. It asks for the folder and the name in a save
  dialog each time, as VS Code asks for any new file, and writes nothing else (user, 2026-11-25).
- **New example programs**: a short set for first steps, graphics, sound and small games, written here.
- **A page "First steps"** in the reference, for someone who has never written BASIC.

- **A page "QB64 Home"** (mockup 4, added on 2026-11-26 at the user's request): New Program, Open a Program,
  Examples and Help, the recent programs and the setup's state, in the editor area. It opens when a QB64 window
  starts with no file open, and a checkbox on it stops that.

Not in this change: F5 keeps starting the debugger. No second extension. The program running beside the code
(mockup 5). A VS Code profile. The ASCII chart, snippets, signature help and a
click on an `$INCLUDE` path stay rows of `SOMEDAY.md`.

**Different from the mockups.** A native tree view cannot hold a text box, so Help's search is a button in the
view's title that opens VS Code's quick pick. The Run button shows no "F5", since F5 debugs.

## Capabilities

### New Capabilities
- `editor/side-panel`: the QB64 view container, the Program view, the SUBs and FUNCTIONs view, and the status
  bar item.
- `editor/help`: the Help view, the help tab, the search, and help for the word under the cursor.
- `editor/examples`: the Examples view, opening an example as a copy, and running one.
- `editor/walkthrough`: the "Get Started with QB64" walkthrough and the command that opens it.
- `editor/new-program`: the command "QB64: New Program" and its save dialog.
- `editor/home`: the page "QB64 Home", when it opens by itself, its recent programs and its setup rows.

### Modified Capabilities
- `editor/language-server`: "Hover" gains the reference's first sentence for a built-in and a keyword. Added:
  a request for the procedures of a document's program, and a request for the help topic at a position.
- `editor/packaging`: added, the help pages and the examples in each package.

## Impact

- `vscode\`: `package.json` (a view container, four views, a walkthrough, four commands, no setting and no
  key); new sources for the views, the help tab, the examples and New Program; unit and integration tests;
  `README.md`; `scripts\stage_bundle.py`. One new dependency, `markdown-it`, to draw a help page.
- `crates\lsp`: `hover.rs`, `server.rs`, a new generated table of each word's page and first sentence.
- `tools\reference\reference.py`: a `help` command that writes the pages, an index and the examples for the
  extension, and the generated table for the language server. `--check` covers the table.
- `examples\`, new: the new example programs, with a test that each one checks with no diagnostic.
- `docs\reference\first-steps.md`, new. `docs\LAYOUT.md`, `docs\QUICKSTART.md`, `DECISIONS.md`, `SOMEDAY.md`
  (the rows "F1 help: a link" and "help text of our own" change).
- The package grows by about 3.3 MB before compression: `docs\reference` is 2.2 MB and
  `tests\reference\examples` is 1.1 MB.
- Machine time: `cargo test -p qb64rust-lsp`, the extension's unit tests and `just quick` on the host. The
  extension's integration suite runs in CI, as today.
- The user's time: the two design questions are answered (design, "Decided by the user"). After the build, a look at
  each screen in a light, a dark and a high-contrast theme, which no test can judge.
