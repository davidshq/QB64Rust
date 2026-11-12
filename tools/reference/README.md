# The language reference's tool

`reference.py` lists the words the language reference needs, gathers what the repository knows about each one,
fills the parts of a page that come from data, and checks the rest. The plan is `study\49`; the OpenSpec change
is `language-reference`, whose `tasks.md` holds the batches.

The pages are in `docs\reference\entries\`, one per word. The examples are in `tests\reference\examples\`, one
program each, recorded with `qb64pe.exe` (`tests\reference\README.md`).

```
python tools\reference\reference.py inventory          # write docs\reference\inventory.json
python tools\reference\reference.py evidence LEFT$     # what the repository knows about one word
python tools\reference\reference.py skeleton LEFT$     # write a new page with its generated blocks
python tools\reference\reference.py render --compiler target\release\qb64rust.exe   # refresh what is generated
python tools\reference\reference.py --check --compiler target\release\qb64rust.exe  # what check's step runs
python tools\reference\reference.py overlap            # compare with the wiki cache (local only)
```

## The one rule

**Nothing on a page is written from memory.** Every rule under "Behavior" and "Errors" names its source in a
`<!-- src: … -->` comment, and every example is a program that was compiled, run and recorded. When the sources
do not answer a question, measure it with a new verification program, or leave the sentence out. *Why:* a
model's memory of QB64 is the QB64 wiki, which is partly wrong for this compiler and has no license, so its text
must not be copied (`DECISIONS.md` 2026-10-02).

## The words

`inventory` takes the words from four sources:

- the built-in table, `tools\builtins\builtins.json` (statements and functions);
- the parser's reserved words, `crates\syntax\src\parser\keywords.rs`;
- the formatter's spellings, `crates\format\src\words.rs` (the metacommands come from here);
- the hand-written word lists of the VS Code grammar, `vscode\syntaxes\qb64rust.tmLanguage.json`.

`words.toml` holds what those cannot say:

| Table | What it holds |
|---|---|
| `done` | The batches whose pages must all exist. `--check` fails on a missing page of a batch named here |
| `pilot` | Pages required whatever `done` says |
| `compounds` | Two words that are one statement (`LINE INPUT`). The first word alone does not match them |
| `batches` | The name of each batch of `study\49` §7 |
| `spec_batch` | The batch of each spec. A word goes to the batch of the spec that names it most |
| `prefix` | The batch of every word that starts so (`_SND`, `_MOUSE`) |
| `area` | The batch of one word, where neither a prefix nor the specs give it rightly |
| `page` | A word covered on another word's page (`THEN` on `if`) |
| `extra` | Words no source lists (`$NOPREFIX`, the compounds) |
| `skip` | Words that get no page, each with why |

The batch a word gets is a first guess. A batch's session moves a word with `[area]` when its page belongs
elsewhere, and adds `[page]` rows for helper words. `inventory.json`'s `one_source` lists the words only one
source names: a person decides whether each needs a page.

A page's file name is the word in lower case, a leading `$` written `meta-` and a trailing `$` written `-str`:
`left-str.md`, `meta-include.md`, `_memcopy.md`, `line-input.md`.

## Writing a page

1. Run `evidence <WORD>` for each word of the page and read the whole pack. It holds the word's entries in the
   built-in table, every requirement of the language specs that names it (the most specific first), the recorded
   programs that use it with each family's line from `verification\README.md`, its call-site programs, and the
   rows of the two divergence registers and of `SOMEDAY.md` that name it.
2. Open the recorded programs the page will rest on: the `.bas`, its `.out.txt` or `.output`, and for a
   rejection its `.compile.txt`. Compile a rejected program with `qb64rust -z --no-settings <file> -o <scratch>`
   to get QB64Rust's own message: the page gives that one, not QB64pe's.
3. Run `skeleton <WORD>`, then write the page. Its layout:

   | Heading | Required | What goes there |
   |---|---|---|
   | `# WORD` and a summary | yes | One or two sentences: what it does. A hover will show this |
   | `## Syntax` | yes | Generated for a built-in, or written by hand where the table's form reads badly; written by hand for a keyword |
   | `## Arguments` | | Each argument: what it is, its type |
   | `## Result` | | A function's result |
   | `## Behavior` | yes | One rule per paragraph or list item, each with its `src:` |
   | `## Errors` | | Compile errors with QB64Rust's message, run-time errors with their numbers, each with its `src:` |
   | `## Differences` | | From QB64pe (a `D-0nn` row), QB 4.5 (a `Q-0nn` row), with a fix on, on Linux |
   | `## The OpenGL routines` | | On `_gl.md` only: the block `opengl`, generated from `crates\sema\src\gl_table.rs` |
   | `## Availability` | yes | Generated |
   | `## Example` | | Generated from the example programs |
   | `## See also` | yes | Related words |

4. Write the example as `tests\reference\examples\<page>_<n>.bas`: `$CONSOLE:ONLY` on its first line (the page
   leaves it out), the common use, a few lines, ending with `END`. A file it includes goes in
   `<page>_<n>.files\`, and the page shows it. Build and run it with `qb64rust` first and read its output.
   An example that needs a window can draw on an image and print its pixels (`line_1.bas`).
5. Record the examples with `qb64pe.exe`, add them to `tests\reference\pass.list`, and compare them with
   `qb64rust` (`tests\reference\README.md`).
6. Run `render --compiler …`, `--check --compiler …` and `overlap`.

### The `src:` comment

It ends the paragraph or list item it belongs to, with no blank line between. It names one source or several,
separated by `;`:

| Form | Example | Checked |
|---|---|---|
| A spec requirement | `language/builtin-functions "Names and arity"` | The spec has a requirement of that heading |
| A recorded program, without `.bas` | `verification\v20_c_string_edges` | The file exists |
| Code read, not measured | `read: libqb\internal\c\libqb.cpp:8701-8766` | The file exists |
| A divergence row | `D-012`, `Q-037` | The row exists |

Only `language/*` specs and `compiler/pipeline`, `compiler/fixes` and `compiler/lints` are about the language;
`evidence` reads only those. A sentence that ends with a colon and introduces a list needs no source. A source
that is only code read (`read:`) says so: the page should not present it as measured.

### A probe

`<!-- probe: PRINT FRE(-1) -->` on a page is one more line that `render --compiler` compiles for the
availability (`" : "` separates statements). It may be rejected: the availability then says so. A call-site
program or an example must compile; one that does not is a problem.

## Writing a chapter

A chapter, `docs\reference\chapters\<spec>.md`, tells how one area of the language works as a whole. It is
named after its spec under `openspec\specs\language\`, or `pipeline`, `fixes` or `lints` for the compiler specs
`evidence` reads, and written from that spec and the area's pages. It has a
title and a summary, then headings of its own choosing. Every paragraph, list item and table under a heading
ends with its `src:` comment, as on a page. It links to the pages (`../entries/<page>.md`) rather than
repeating them. It may embed an existing example with `<!-- begin generated: example:<id> -->` and
`<!-- end generated -->`; it has no other generated block. The index lists the chapters.

## What is generated

`render` writes the blocks between `<!-- begin generated: … -->` and `<!-- end generated -->` (`syntax`,
`availability`, `example:<id>`, and `opengl`, the table of the OpenGL routines), the index
`docs\reference\README.md`, `inventory.json` and the appendices of `docs\reference\`: `appendix-constants.md`
(from `builtins.json`), `appendix-errors.md` (from libqb's `error_handle.cpp`), `appendix-reserved.md` (from
`keywords.rs`), `appendix-metacommands.md` (from the inventory), `appendix-unsupported.md` (from `SOMEDAY.md`,
"Deferred QB64pe features") and `coming-from-qb64pe.md` and `coming-from-qb45.md` (from the two divergence
registers). An edit of one of those sources needs a `render`, or `--check` fails. With `--compiler` it
also writes `tests\reference\availability.json`: for each page, how many of the programs that use its words
compile, and the messages of the others. Never edit a generated block by hand: `--check` fails on it.

`--check` fails when a generated block, an embedded example or a generated file is out of date; when a rule has no
source or a source does not exist; when a required heading is missing, the headings are out of order or a page
holds `TODO`; when a hand-written "Syntax" of a built-in leaves out a word of the built-in table's form (a word
that starts with `_`, such as `_SMOOTH`, or `STEP`); when a link leads nowhere; when a page has no word or an example has no page; and when a page of a
`done` batch or a pilot page is missing. It checks a chapter in the same way: its sources, its links, its
embedded examples, a `TODO`, and that a spec of its name exists. With `--compiler` it computes the availability
again and fails when it changed.

## The overlap check

`overlap` reads the wiki cache (`tools\wiki\cache\pages`, fetched by `tools\wiki\fetch_wiki.py`, not in the
repository) only to compare. It fails when a page shares a run of 10 words of prose with a wiki page, or an
example shares 6 lines of code with a wiki example. Run it before handing over a batch. It writes nothing.

## What a batch reports

The pages written; each sentence left out for want of a source; the programs added; and what writing the pages
found. A recording that `qb64rust` does not match, a spec that says the opposite of a recording, or a measured
difference that no divergence row records is not fixed in the batch: it is a line in the report, a `SOMEDAY.md`
row where it is a compiler matter, and the page says what the compiler does today.

## Tests

`test_reference.py`, in the `tools` step of `check`. The page check and the overlap check run on pages written
under `target\scratch`.
