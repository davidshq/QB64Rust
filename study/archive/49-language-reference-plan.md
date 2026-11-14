# 49. Plan: a language reference for QB64Rust

Written 2026-11-11, at the user's request: "a plan for building out a comprehensive language reference for
QB64Rust, something that can be handed off to Opus or Sonnet". Section 8 holds the questions for the user, each
with a default.

**Decided 2026-11-11** (user: "do it"): the plan and the defaults of all five questions (`DECISIONS.md`). Built as
the OpenSpec change `language-reference`. Where its `design.md` differs from this plan, the design holds: it
puts the examples in `tests\reference\examples\`, and a page's availability may also say "Rejected, as in QB64pe".

**For the model that takes this over:** read sections 1 to 4 once, then work from section 6 (the batch
procedure) and section 7 (the batches). Section 5 is built first, by one session, before any page is written.

## 1. What is being built

A reference that describes **this compiler**: every statement, function, metacommand, operator, type and
built-in constant a program may use, what it does, which errors it raises, and where it differs from QB64pe
4.7.0 and QuickBASIC 4.5.

The reader is a QB64 programmer, the same reader as `MANUAL.md` and `QUICKSTART.md`. They may come from
QuickBASIC 4.5 or from QB64pe. They want to look a word up and get a correct answer in a minute.

Today there is none. A hover shows a signature and the word "Built-in" (`crates\lsp\src\hover.rs`,
`builtin_text`). `study\47` names "no help text at all" as a gap for a new user. `SOMEDAY.md` has three rows in
the group `f1-help`; this plan builds the text that the P3 row ("help text of our own") needs.

It has three parts:

| Part | Content | Size |
|---|---|---|
| Chapters | One per area of the language: how the area works as a whole (types, scope, files, the screen) | About 30, one per spec under `openspec\specs\language\` |
| Entries | One page per word, in a fixed layout (section 4) | About 550: 404 built-in names, about 100 language keywords, about 40 metacommands and preprocessor words |
| Appendices | Tables, most of them generated: the auto-included constants, the color names, the error numbers, the reserved words, the OpenGL routines, what is not supported yet | About 8 |

## 2. The two risks, and the rule that answers both

**Risk 1: a wrong statement.** A model knows QB64 from its training. It will write what it remembers of the
QB64 wiki, and part of that is wrong for this compiler, or for any compiler.

**Risk 2: copied text.** The QB64pe wiki states no license, so no wiki text may be committed or shipped
(`DECISIONS.md` 2026-10-02). A model that writes from memory can reproduce wiki sentences and wiki examples
without meaning to. FreeBASIC's manual and Microsoft's QB 4.5 help are also not to be copied (`CLAUDE.md`
working rule 3).

**The rule: nothing is written from memory.** Every statement about behavior comes from one of the sources in
section 3 and names it. Every example is a program in this repository that was compiled and run, with its
output recorded. When the sources do not answer a question, the writer measures it with a new verification
program, or leaves the statement out. The tool of section 5 checks all of this mechanically, because the user
cannot review 550 pages by reading them.

## 3. The sources a page may be written from

| Source | What it gives | Size today |
|---|---|---|
| `tools\builtins\builtins.json` | Every built-in: name, statement or function, argument types, optional arguments, return type, the syntax line `hr_syntax`, the libqb routine | 455 entries, 404 names; 427 have `hr_syntax`; 28 are stubs |
| The same file, `auto_include` | The auto-included constants and their values | 600: 310 of `beforefirstline.bi`, 270 of `color32.bi`, 17 of `color0.bi` |
| `openspec\specs\language\*\spec.md` | The specified behavior, as requirements with scenarios. This is the first source for every description | 35 specs, about 9,800 lines |
| `openspec\specs\compiler\pipeline`, `fixes`, `lints` | The preprocessor, included files, the fixes switch, the warnings | |
| `verification\` with `verification\README.md` | Measured facts of `qb64pe.exe`, each a program with its recorded output | Families `v13` to `v70` |
| `tests\corpus\`, `tests\upstream\` | More recorded programs | |
| `tests\callsite\` | One one-statement program per built-in form and argument type. Compiling one says whether that form is supported | About 2,650 programs |
| `docs\DIVERGENCES-QB64PE.md`, `docs\DIVERGENCES-QB45.md` | The decided differences | 62 rows `D-0nn`, 36 rows `Q-0nn` |
| `docs\SOMEDAY.md`, "Deferred QB64pe features" | What is "not supported yet" on purpose | |
| `crates\format\src\words.rs`, `vscode\syntaxes\qb64rust.tmLanguage.json`, `crates\syntax\src\parser\` | The language's own keywords. There is no single keyword table: the lexer reads a keyword as a name, and the parser matches words | About 130 spellings in `words.rs` |
| `libqb\` and the compiler's sources | To read how something works when a spec is silent. A statement read from code and not measured says "read, not measured" in its source note | |
| The compiler itself | `qb64rust -z` and a build, to check an example | |

Not sources: the wiki cache in `tools\wiki\cache\`, `..\FreeBASIC`, `qb64pe-vscode`, QB 4.5's help, and the
model's memory. The wiki cache has one use, in the overlap check of section 5.

## 4. The layout of an entry page

One file per word, in `docs\reference\entries\`. A statement and a function of one name share a page (`SCREEN`,
`PLAY`, `MID$`). The file name is the word in lower case, with a leading `$` written `meta-` and a trailing `$`
written `-str`: `left-str.md`, `meta-include.md`, `_memcopy.md`, `input.md` beside `input-str.md`.

```markdown
# LEFT$

One or two sentences: what it does. This is also the text a hover will show.

## Syntax
<!-- generated: do not edit -->
(the forms, from the built-in table; written by hand for a language keyword)

## Arguments
## Result            (functions only)
## Behavior
Short paragraphs or a list, one rule each.
<!-- src: language/builtin-functions "<requirement heading>"; verification\<program> -->

## Errors
Compile errors and run-time error numbers, each with what raises it.
<!-- src: ... -->

## Differences
From QB64pe (the `D-0nn` row), from QuickBASIC 4.5 (the `Q-0nn` row), with a fix on (the fix's name),
on Linux. Left out when there is none.

## Availability
<!-- generated: do not edit -->
(supported, partly supported or not supported yet, from compiling the call-site programs)

## Example
<!-- example: left-str_1 -->
(the program's text and its recorded output, copied in by the tool)

## See also
```

Rules of the layout:

- Every paragraph and list under "Behavior" and "Errors" ends with a `src:` comment. It names a spec
  requirement by spec and heading, a recorded program by path, or a divergence row by id. The comment does not
  show when the page is rendered.
- The summary, "Syntax" and "Availability" are what a hover or F1 can show later without the rest.
- A page says what QB64Rust does. It does not say "QB64pe does" except under "Differences".
- An example is small, prints to the console where it can, and shows the common use, not an edge case. Edge
  cases are sentences with a `src:`, since their programs already exist in `verification\`.
- The chapters have no fixed layout, but the same `src:` rule and the same example rule.
- Writing follows `CLAUDE.md` working rules 9 and 10: American spelling, short sentences, the specifics kept.

## 5. The tool, built first (one session, Opus)

This is an OpenSpec change, `language-reference` (`CLAUDE.md` working rule 11: it adds a tool and a test kind
and spans many sessions). Its `tasks.md` holds the batches of section 7, so that each later session is
"apply the next batch". The session writes the proposal, design, spec and tasks, then builds this:

`tools\reference\reference.py`, with its own tests and a `README.md`, and these commands:

1. **`inventory`** writes `docs\reference\inventory.json`: every word that needs a page. Built-ins come from
   `builtins.json`. Keywords are the union of `words.rs`, the hand-written lists of the TextMate grammar and
   the words the parser matches; a word found in only one of the three is listed for a person to judge. Each
   word gets its kind, its area (the spec that covers it) and its batch.
2. **`evidence <WORD>`** prints everything the repository knows about one word: its table entries, the spec
   requirements that name it (with their text), the verification and corpus programs that use it (path and the
   family's line in `verification\README.md`), its call-site programs and whether each compiles, the divergence
   rows and `SOMEDAY.md` rows that name it. This is what makes the pages writable by Sonnet: the writer reads
   one pack, not the repository.
3. **`skeleton <WORD>`** writes the page with the generated blocks filled and the other headings empty.
4. **`render`** refreshes the generated blocks of every page, copies each example's text and output in, and
   writes the index (`docs\reference\README.md`: by letter and by area) and the generated appendices.
5. **`--check`**, added to the `tools` step of `check`, fails when:
   - a word of the inventory has no page, or a page has no word;
   - a generated block or an embedded example differs from what `render` would write;
   - a paragraph under "Behavior" or "Errors" has no `src:`, or a `src:` names a spec heading, a file or a row
     that does not exist;
   - a page says a form is supported and its call-site program does not compile, or the reverse;
   - a link points nowhere, or a heading of the layout is missing or out of order.
6. **`overlap`**, run locally only since the wiki cache is not in the repository: it fails when a page shares a
   run of 10 or more words, or an example shares 6 or more lines, with a page of `tools\wiki\cache\`. This
   reads the cache only to compare. It is the check against Risk 2.

**Examples are test programs.** They live in `tests\reference\` as `<page>_<n>.bas` with a recorded `.output`,
under the corpus's conventions (`tests\corpus\README.md`: `.output`, `.err`, `.norun`, `.stdin`, `.external`).
They are recorded with `qb64pe.exe` and compared with `qb64rust`, as corpus programs are, by a new suite of
`run_legacy_tests.py` and a new `check` step. A whole run of them is a large run and goes to the suites VM
(`CLAUDE.md` working rule 6). An example that needs a window, sound, a person or the network follows the rules
those programs already have, or is `.norun`.

The session ends with **six pilot pages**, chosen to cover the kinds: `LEFT$` (a plain function), `LINE` (a
statement with a special format), `SELECT CASE` (a block keyword), `$INCLUDE` (a metacommand), `_MEMCOPY` (a
QB64 word with errors to describe) and one word that is not supported yet. The user reads these six. What they
change is put into the layout before any batch starts.

## 6. The procedure for one batch

A batch is one area: its chapter and its entry pages. One batch is one session. If the area has more than
about 15 entries, the session stops after 15 and the next session continues.

1. Read `docs\STATUS.md`, this section, and the area's spec in full.
2. For each word of the batch: run `evidence`, read the pack, run `skeleton`, write the page from the pack.
3. When the pack does not answer something a reader will ask (what happens with a negative argument, which
   error number), write a verification program for it in the newest family, record it with `qb64pe.exe`, add
   it to `tests\verification\pass.list` or `known.list`, and add the family's line to `verification\README.md`.
   If that costs too much for the page, leave the statement out. Never fill the gap from memory.
4. Write one example per page, two when a statement and a function share it. Build each with `qb64rust` and
   read its output before recording it.
5. Write the chapter last, from the spec and the pages.
6. Run `reference.py render`, `reference.py --check`, `reference.py overlap`, `cargo test -p` on anything
   touched, and `just quick` in the background. Run the batch's new examples by name on the host if they are
   20 or fewer, in the VM otherwise.
7. Report: the pages written, the statements left out for lack of a source, the programs added, and **what
   the batch found**. Writing a reference finds bugs. A page where `qb64rust` and the recording disagree, or a
   spec that says the opposite of a recording, is not fixed in the batch. It becomes a `SOMEDAY.md` row and a
   line in the report, and the page describes what the compiler does today.

The rules of `CLAUDE.md` that a batch most easily breaks: nothing is staged; no subagents; text with a
backslash goes through the Edit and Write tools, never the shell; no program that opens a port runs on the
host; no full local path in a recorded output; a new folder gets its row in `docs\LAYOUT.md`.

## 7. The batches

The order puts first what a new user looks up first. The entry counts come from `inventory` (section 5); the
table below fixes only the order, the specs and the model.

| Batch | Area | Specs (`language/…`) | Model |
|---|---|---|---|
| 1 | Lines, names, comments, labels, `$INCLUDE`, the preprocessor | `compiler/pipeline` | Opus |
| 2 | Types, literals, operators, conversions, evaluation order | `numeric-semantics`, `evaluation-order`, `default-types` | Opus |
| 3 | Variables, scope, constants | `procedures` (the scope part), `constants` | Opus |
| 4 | Control flow | `control-flow` | Sonnet |
| 5 | Procedures | `procedures` | Opus |
| 6 | Arrays and `TYPE` | `arrays-and-types`, `unstable-features` (the largest spec: 1,963 lines) | Opus |
| 7 | Strings | `fixed-length-strings`, the string functions of `builtin-functions` | Sonnet |
| 8 | Math and number functions | `builtin-functions` | Sonnet |
| 9 | `PRINT`, `PRINT USING`, console and text output | `print-using`, `console-and-window` | Sonnet |
| 10 | Keyboard and console input | `keyboard`, `builtin-statements` | Sonnet |
| 11 | Errors | `error-handling`, `run-time-checks`, `asserts-and-debug` | Opus |
| 12 | Files | `file-io`, `record-files`, `data-read` | Sonnet |
| 13 | Screen modes and drawing | `screen-output` | Sonnet |
| 14 | Images and fonts | `images`, `fonts` | Sonnet |
| 15 | Mouse and devices | `mouse`, `device-input` | Sonnet |
| 16 | Sound | `sound` | Sonnet |
| 17 | Events and timers | `events` | Sonnet |
| 18 | Memory | `mem-blocks`, `conventional-memory` | Opus |
| 19 | Libraries | `external-libraries` | Opus |
| 20 | Window, clipboard, system, date and time, the rest of `builtin-statements` | `console-and-window`, `builtin-statements` | Sonnet |
| 21 | Data processing, embedded files, logging | `data-processing`, `embedded-files`, `logging` | Sonnet |
| 22 | Network | `network` | Sonnet |
| 23 | OpenGL: one chapter and one generated table, not a page per routine | `opengl` | Sonnet |
| 24 | Appendices, and a page "Coming from QB64pe" and "Coming from QuickBASIC 4.5" written from the two divergence registers | | Opus |
| 25 | The review of the whole (below) | | Opus or Fable |

Opus takes the areas where the rules are subtle and a spec must be read as a whole. Sonnet takes the areas that
are many separate words, where the evidence pack carries the page.

**Batch 25, the review.** One session reads the index and every chapter, and 60 entry pages picked by a fixed
random seed. It checks that pages of one area agree with each other, that "See also" leads somewhere useful,
and that 30 real questions find their answer: take them from `docs\PORTING-PDS.md` and the game ports' lessons.
It then links the reference from `README.md`, `MANUAL.md` and `QUICKSTART.md`, and adds it to the two
downloads if section 8's question 3 says so.

**Keeping it right afterwards.** `--check` runs in `check`'s `tools` step, so a new built-in without a page
fails, and a form that becomes supported fails until `render` is run. A change that alters a behavior edits the
page with the spec. That sentence goes into `CLAUDE.md` working rule 11 when batch 25 is done.

## 8. Questions for the user, with the panel's view

The panel: a pragmatic engineer, a QB64 engineer, a compiler engineer, a Rust engineer, a test engineer, and a
technical writer (added because the subject is a document for readers outside the project).

1. **Is the form right: Markdown pages under `docs\reference\`, one per word?**
   Default: yes. The other forms are one long file, or a site built with a tool such as mdBook.
   The writer and the QB64 engineer want one page per word: it is what F1 and a hover can point at, and what a
   search lands on. The pragmatic engineer notes the cost that stays: about 580 files. The panel recommends
   pages now, and a site as a `SOMEDAY.md` row, since Markdown pages can be fed to any site tool later.

2. **Are the examples recorded test programs?**
   Default: yes, in `tests\reference\`, about 600 small programs.
   The test engineer and the compiler engineer hold this to be the point of the plan: an example that is not
   run goes stale, and recording against `qb64pe.exe` catches a wrong example at once. The cost that stays is
   machine time: the corpus has 523 programs today, so this about doubles that step in a full run. The
   pragmatic engineer accepts it if examples stay console programs of a few lines. The panel recommends yes.

3. **Does the reference ship in the two downloads, and feed the hover?**
   Default: it is linked from `README.md` and the manual now; shipping it and showing the summary in the hover
   are a separate change after batch 25 (`SOMEDAY.md`, `f1-help` P3).
   The QB64 engineer wants the hover soon, as it is the gap `study\47` names. The Rust engineer points out that
   the layout already keeps the summary and syntax apart for it, so nothing is lost by waiting. The panel
   recommends the default.

4. **May the wiki cache be read for the overlap check only?**
   Default: yes, and for nothing else. The check runs locally and writes nothing from the wiki into the
   repository. The panel sees no risk in comparing, and a real risk in not comparing. The question to the
   QB64pe maintainers about the wiki's license (`STATUS.md`, "Queued for later milestones") is not needed for
   this plan, since no wiki text is used.

5. **Which behavior does a page describe when a fix changes it?**
   Default: the behavior with the fixes off, which is QB64pe's, with a line under "Differences" for each fix
   that changes it. Language extensions behind their switch (`DECISIONS.md` 2026-11-11) get pages when they
   exist, marked as extensions. The panel agrees: the default must match what an old program sees.

## 9. What it costs

In Claude's time: one session for the tool and the pilot, about 35 sessions for batches 1 to 24 at 15 entries
each, one or two for the review. In the user's time: the five answers above, reading six pilot pages, and
staging each batch. No batch needs the user to read its pages; the checks stand in for that, and the batch's
report names what was left out and what was found. In machine time: about 600 more programs in a full run, and
the recordings of the new verification programs.
