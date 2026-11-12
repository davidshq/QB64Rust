# Design

## Context

See `proposal.md` for the motivation. What the code does today:

- **The formatter uses the checked program for one thing.** `Fmt::name` (`crates\format\src\fmt.rs`) writes a
  name with the spelling of its symbol's definition. A name with no symbol is already written as read. Nothing
  else in `crates\format` reads the `Program`: indentation, spacing and keyword case come from the tree.
- **Two places refuse to format on any error.** `format_for_server` (`crates\driver\src\lib.rs`) returns the
  diagnostics when `diagnostics.has_errors()`. `format` (`crates\driver\src\main.rs`) returns exit 1 when
  `error_count() > 0`.
- **`sema` goes on after an error.** `qb64rust -c turn.bm` reports 108 errors over the whole file, so the
  symbol table of a program with errors covers the whole file.
- **A symbol does not say whether a statement declared it.** `Symbol` (`crates\sema\src\symbols.rs`) has `def`,
  which is the name in a declaration, or the first use of a variable nothing declares. `Symbols::declaring`
  records this for procedures only (the `lints` change).
- **In the server, a program is an open document.** `Server` (`crates\lsp\src\server.rs`) keys `docs`,
  `analyses`, `indexes`, `pending`, `checks` and `published` by the document's key, and every job's `main` is
  `open_file(doc)`. The server already knows which open programs cover a file (`changed`, `release`), and uses
  it to parse includers again and to decide whose diagnostics stand. It does not read the workspace folders.
- **The settings file** is found by `find_settings` (`crates\driver\src\fixes.rs`): the nearest
  `qb64rust.toml` at or above the main file. `parse_settings` reads `[fixes]` and ignores other tables.
- **The extension checks on save** (`vscode\src\diagnostics.ts`), so saving `turn.bm` shows the 108 errors.

What other tools do (searched 2026-11-11): LaTeX Workshop takes the root file from a comment that names it, then
the open file, then a scan of the workspace. clangd lends a header the compile command of an open file that
includes it, then guesses from paths, and its documentation says the guess can be wrong. typst-lsp has a setting
for the main file. Ada's `gnatpp` and PowerShell's `PSUseCorrectCasing` write a name in the case of its
declaration. VBA's editor recases every identifier of a project to the latest declaration whatever its scope,
and one author counts 27 changed objects in version control from one such edit.

## Goals / Non-Goals

**Goals:**

- Format Document works on every `.bi` and `.bm` file.
- Formatting a file alone never gives a letter case that formatting it inside its program would not give.
- An include file's diagnostics and navigation are those of its program.
- A program without errors formats byte for byte as before.

**Non-Goals:**

- Formatting a file that has a syntax error.
- Build, Run and Debug from an include file (they still act on the active document).
- A command line option that formats an include file inside a named program.
- A user interface to choose among several programs. The `main` entry is the way to choose.
- Watching files on disk that are not open (`SOMEDAY.md`, the `language-server` row of 2026-10-08).

## Decisions

### D1. Only the parser's errors stop formatting

The test becomes "the parse has an error", read from `ParsedProgram::diagnostics()`, in `format_for_server`
(`crates\driver\src\lib.rs`) and in `format` (`crates\driver\src\main.rs`). A missing `$INCLUDE` file is the
parser's error and still stops it. A bad `qb64rust.toml` is added after the parse and does not.

`-y` prints the later stages' errors, writes the file and exits 0. *Why exit 0:* the file was formatted, which
is what `-y` was asked for; gofmt and rustfmt do the same, since they never see such errors. `qb64pe -y` writes
nothing here, so this is a `DIVERGENCES-QB64PE.md` row. No program QB64pe formats is affected.

*Instead of:* an option that turns the new behavior on. An option would keep the editor's dead end as the
default.

### D2. `sema` records which symbols a statement declares

`Symbols` (`crates\sema\src\symbols.rs`) gets a set of the symbols that a statement declares, filled where each
definition is made: procedure headers and `DECLARE` (beside `declaring`), `DIM`, `REDIM`, `COMMON`, `SHARED`,
`STATIC`, `CONST`, parameters, `TYPE` blocks with their members, and labels. An implicit variable is not in it.
`Symbols::is_declared(id)` reads it.

*Instead of:* working it out in the formatter from the tree around `def`. `sema` already knows at the point
where it creates the symbol.

### D3. The incomplete rule lives in `Fmt::name`

`format_file` (`crates\format\src\lib.rs`) takes a new argument, `complete: bool`. `Fmt::name`
(`crates\format\src\fmt.rs`) skips a symbol when the program is incomplete and `is_declared` is false, so the
name is written as read. Built-ins keep their table spelling.

The callers decide `complete`: there is no error of a later stage, and the main file's name ends in `.bas`
(any letter case) or the document is not a file. The driver computes it once in a helper used by both `format`
and `format_for_server`.

*Why the file name counts:* an include file checked alone can pass the check. In `turn.bm`, `atkfac` is then a
local variable of each SUB, spelled like its first use there. Inside `CWSTRAT.BAS` it is the shared `ATKFAC` of
`CWS14.BI`. Without the name rule the two results would differ.

*Added during the build (task 2.6):* in an incomplete program, a line whose layout the lexer would read otherwise
than the line is laid out as with `AutoLayout` off, and written as read if that still reads otherwise
(`reading` in `crates\format\src\lib.rs`: letter case and blanks aside, but a blank between two characters of a
name counts). The layout of a statement the check rejected can misread it. Before this, 26 of the 2078 programs of
the task 2.6 test changed, for example `KILL ("a", "b")` laid out `KILL ("a"), "b"`, `PRINT - + s$` laid out
`PRINT - s$`, and `DIM a() AS LONG` under `OPTION _EXPLICITARRAY` laid out `Dim a As Long`. A complete program is
not affected.

*Measured (task 2.6, 2026-11-11):* `verification\` holds 2539 programs recorded `(not compiled)`. 461 of them have
a parse error and are not formatted. The other 2078 format without a panic, and each one's tokens are unchanged
but for letter case and blanks (`format_oracle::programs_with_errors_format_without_a_panic`, 1.5 s).

### D4. The format hook takes the file to lay out

`qb64rust_lsp::Format` (`crates\lsp\src\lib.rs`) gets a `FileId` argument. The worker (`start_worker`,
`crates\lsp\src\server.rs`) finds it after `analyze` by the document's key in `Analysis::files`.
`format_for_server` passes it to `format_file` in place of `parsed.main().file`. `formatted` checks the
version and builds the edits against the document, not against the program's main file.

An answer from a program that is no longer the document's, or whose parse no longer includes it, is
`ContentModified`, and the document is not changed. This happens when a lookup ends while the job runs, for
instance on a Format Document right after an include file opens: VS Code shows nothing, and the next request
formats it inside its program. *Instead of:* laying out the main file in its place, which replaced the include
file's text with the main file's.

### D5. A program is a main file, open or not

Today "program" and "open document" are one key. The server gets a map from each open document to its program:

```
struct Owner { main: String /* key */, uri: Uri, source: Source }   // Source: Settings | Open | Disk | None
owners: HashMap<String, Owner>                                      // by document key; every open file, .bas too
```

A job's `main` is the open document when the program's main file is open, else an `OpenFile` built from the
disk in the default encoding (a new `Server::program_main` beside `open_file`). `analyses`, `indexes`,
`pending`, `checks` and `latest_job` stay keyed by the program's key, which may now be a file that is not open.

Requests name a document. A new `program_of(key)` gives the program's key, and each request path goes through
it: `changed`, `dispatch_due`, `check`, `format_request`, `answer_parsed`, `need_index`, `start_index` and the
rename path. The answers already work per file: `Analysis::files` and the index hold every file of the
program, and references and rename already cross files. Document symbols and folding ranges read the tree of
the document's file from `parsed.trees`.

A program that is not open is dropped (its analysis, index and published diagnostics) when its last owned
document closes.

*Instead of:* keeping the document as its own program and borrowing only the symbol table. Then two parses of
the same text would disagree, and the false errors would stay.

### D6. Publishing for a program that is not open

A program whose main file is not open publishes diagnostics only under the uris of open documents. A check's
answer still counts every error of the program. *Why:* errors under files the user has not opened would fill
the Problems panel with a program they did not ask about.

`published` keeps its rule that a program which includes a file publishes for it last. An owned document never
publishes as a program of its own, so the order question goes away for it.

### D7. The lookup, in `crates\lsp\src\owner.rs` (new)

`find_owner(doc, open, analyses, root)` runs when a document that is a file opens, whatever its extension, and
again when one of these happens: a parse of an open `.bas` program covers the document while it is its own program
(unless `main` names it) or belongs to a program on disk, a parse shows that the owner no longer covers the
document, the owner's main file closes, or `qb64rust.toml` changes (the client already reports it, "Settings file
changed"). Only that last event looks up every open document. The result is
kept in `owners` between those events. When it changes, the old program's diagnostics for the document are
released (`release`) and the new one is parsed.

1. **Settings.** The driver gives the server a new hook, `main_of(doc_path) -> Option<PathBuf>`
   (`crates\driver\src\fixes.rs`, beside `find_settings`): the `[project]` table's `main` of the nearest
   `qb64rust.toml`, joined to that file's folder. It returns `None` for no table, a value that is not a string,
   or a file that does not exist. The server takes it when that program's analysis covers the document. When
   it names the document itself, the lookup ends and the document is its own program.
2. **Open.** The open `.bas` documents that are their own program and whose analysis covers the document, the
   one opened first. `Doc` gets an `opened: u64` from a counter. An open `.bas` document that is itself a part
   of another program is passed over, so a chain of includes gives its top file. The lookup of an includer
   runs before the lookup of what it includes.
3. **Disk.** Folder by folder from the document's folder upward. The upper limit is the folder of the nearest
   `qb64rust.toml`, else the workspace folder that holds the document (the server reads `workspaceFolders`
   from `initialize`, which it does not do today), else the document's own folder. In each folder:
   - Read each file with a BASIC extension (`.bas`, `.bi`, `.bm`) and keep those whose bytes contain the
     document's file name without regard to case. A kept `.bas` file is a candidate. Every kept file, a
     `.bas` file too, adds its own name to the names searched for, so that an include of an include is
     followed up to the top file.
   - Order the candidates as the spec says and `analyze` them in that order until one covers the document.
   - Stop after 500 files read or 20 candidates analyzed, and log it to stderr.

   The byte search is only a filter. `analyze` decides, with the compiler's own include lookup.

*Why settings first:* it is the one explicit statement, and it settles two includers (`CWSTRAT.BAS` and
`tests\rules_test.bas` both include Civil War Strategy's files). *Why a `.bas` file is looked up like any
other:* the extension does not say whether a file is a whole program, and a `.bas` file may be included by
another (user, 2026-11-11). The extension decides two things only. A program found on disk or among the open
documents is a `.bas` file. And a file that nothing includes is checked in full when it is a `.bas` file, as
today, and for the parser's errors only when it is not (D9).

The lookup runs on the worker thread, as a job, since it reads the disk and parses.

### D8. `qb64rust/program`

The server sends `{ uri, program, source }` (`crates\lsp\src\server.rs`; the name in
`vscode\src\server\protocol.ts`) after each lookup that changes a document's owner, and once when the document
opens. The extension keeps the last value per uri and shows it for the active editor in a third language
status item, `qb64rust.program`, beside the two in `vscode\src\languageServer.ts` and `vscode\src\statusBar.ts`.
It is hidden for a `.bas` document that is its own program.

### D9. A lone include file's check

For a document with `Source::None` whose name does not end in `.bas`, the check job publishes and counts the
parser's diagnostics and skips the `check` hook. Hover and the other index requests still run the front end on
the file alone, as today; an unknown name gives no answer.

## Risks / Trade-offs

- [The symbol table of a program with errors holds a wrong symbol, and a name gets a wrong case] → Under D3 only
  declared symbols change case, and a declaration is in the file as written. The agreement test of task 2.5
  formats each include file of a fixture alone and inside its program and compares.
- [The formatter panics on a program with errors, a state it has not seen] → The worker already catches a panic
  and reports it. Task 2.6 formats every program of `tests\verification` that is recorded `(not compiled)` and
  parses, and asserts no panic and an unchanged token sequence.
- [The parse of a file alone differs from its parse inside the program] → The parser does not use the names of
  SUBs (`turn.bm` alone parses without an error; the missing SUBs are `sema`'s errors). The agreement test
  would show a difference.
- [The disk scan is slow in a large folder, such as a corpus of thousands of programs] → The byte filter reads
  files without parsing, the limits of D7 bound it, it runs on the worker at the events of D7 only, and its
  result is kept. Opening a `.bas` file looks up that file alone; only a change of `qb64rust.toml` looks up
  every open document.
- [The scan picks the wrong program] → The status item shows the choice, and `main` overrides it.
- [A `.bas` program that another `.bas` file includes, such as a test that includes the whole program, is
  checked as a part of that file] → The status item shows it, and a `main` entry that names the program makes
  it its own program again.
- [Every `.bas` file that opens now starts a disk scan, also in a folder of thousands of programs] → The same
  limits bound it. Task 5.6 measures it in `tests\verification`.
  *Measured (task 5.6, 2026-11-11, release build on the host, the repository as the workspace folder):* opening
  `verification\v01_numeric.bas`, in a folder of 4416 programs, the lookup stops at the limit of 500 files read
  and logs it; its notification comes 0.30 s after the open. The document's first diagnostics come with it,
  since the parse waits behind the lookup on the worker. An edit after that gives its diagnostics in 0.11 s.
- [A program not open is parsed after every edit of its include file] → This already happens when both are
  open. Task 5.6 measures the parse of Civil War Strategy's program from one of its include files.
  *Measured (task 5.6, same run, `..\civil-war-strategy` as the workspace folder):* opening `rules\turn.bm` alone
  finds `CWSTRAT.BAS` on disk in 0.045 s, and its first diagnostics (none) come 0.083 s after the open. Each of
  three edits gives its diagnostics 0.137 s after the change, of which 0.1 s is the debounce. Format Document on
  `turn.bm` through the protocol gives one edit and no message.
  *Checked (tasks 5.6 and 6.2, 2026-11-11, on this repository's `tests\programs\civil-war-strategy`, at the
  user's word):* that copy is one file, so a scratch copy under `target\scratch\` was split as the port is: its
  `COMMON SHARED` lines into `cws14.bi`, `SUB iterate` and `SUB movefrom` into `rules\turn.bm` with every name in
  capitals; the split program checks with no error. The release server, driven over stdio, finds `CWSTRAT.BAS` on
  disk for `turn.bm`; its check counts 0 errors and publishes none; Format Document gives one edit and no message,
  with the 31 shared names and SUBs of the program respelled as declared (`ARMYLOC` as `armyloc`) and the SUB's
  own locals as typed. Alone in a folder with no program, `turn.bm` gets `none`, a check of 0 errors, and a layout
  equal but for letter case, every difference a name left as typed. The extension's `programStatus` gives
  `CWSTRAT.BAS` and "no program found" for the two notifications. The status item was not looked at in VS Code
  itself.
- [`-y` now exits 0 where a script expected 1 for a program with a type error] → `-c` is the check. The
  `DIVERGENCES-QB64PE.md` row and `MANUAL.md` say so.

## Open Questions

- Should `[project]` `main` also let `qb64rust` build with no file named? It does not change this design; a
  `SOMEDAY.md` row holds it.
