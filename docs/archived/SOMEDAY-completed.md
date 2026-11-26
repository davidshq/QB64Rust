# Someday: the rows that were built

The rows of `docs\SOMEDAY.md` that a change built or made unneeded, under the headings they stood under there
(`CLAUDE.md` working rule 7). The open rows are in `docs\SOMEDAY.md`.

## VS Code parity left

| Priority | Group | Feature | What the old IDE does, and what it needs here | Source | Outcome |
|---|---|---|---|---|---|
| P2 | f1-help | Contextual F1 help: a link to the online wiki | F1 on a built-in or keyword opens its page of the QB64pe wiki in the browser, and the hover gives the same link. A link ships no wiki text, so it does not wait on the wiki's license | `study\archive\06` "Help system"; the panel, 2026-10-29 and 2026-11-08 | Not built as written: help is the project's own reference, which ships in the extension and works offline (OpenSpec change `editor-side-panel`). F1 stays VS Code's (user, 2026-11-25); help is "QB64: Help for This Word", the editor's right-click menu and the hover's link |
| P3 | f1-help | Contextual F1 help: help text of our own | A few lines per built-in and keyword, written from the built-in table and the specs and checked against the recordings, shown in the hover (today it shows only the signature) and by F1. It works offline and describes this compiler, divergences included. Each claim to be tied to a recording, so that the text does not drift from the compiler | The same | Built by OpenSpec change `editor-side-panel`: the hover gives the first sentence of the word's reference page, and the help tab shows the whole page with its recorded examples |
