# Design

## Context

See `proposal.md` for why, and `study\49` for the plan this change builds. What shapes the approach:

- **The writers are other models**, working one batch per session, from a brief. Their memory of QB64 is the wiki
  (partly wrong here, and not licensed to copy). A page must be writable from what one command prints, and
  checkable without a person reading it.
- **The repository already holds the facts.** The merged specs under `openspec\specs\language\` say what the
  compiler does, as requirements with scenarios. `verification\` and `tests\corpus\` hold thousands of programs
  recorded with `qb64pe.exe`. `tools\builtins\builtins.json` lists every built-in with its arguments and syntax
  line. `tests\callsite\` holds one program per built-in form.
- **There is no single keyword table.** The lexer reads a keyword as a name. The words come from four places: the
  built-in table, the parser's reserved words (`keywords.rs`), the formatter's spellings (`words.rs`) and the VS
  Code grammar.
- **The test runner already runs another corpus root** (`--corpus-root`, as the `differential` step does).

## Goals / Non-Goals

**Goals:**
- A page's every rule is tied to a source a tool can find; a missing or renamed source fails a check.
- An example cannot go stale: it is recorded from `qb64pe.exe` and compared with `qb64rust`.
- What is "supported" on a page is computed, not written.
- A batch is one session's work, started from `tasks.md` and this tool's README.

**Non-Goals:**
- Judging whether a page is well written or complete. The review batch (task group 26) reads samples.
- Checking that a `src:` source says what the sentence says. The check finds the source; the writer and the
  review read it.

## Decisions

### D1. Pages in Markdown, one per word, in `docs\reference\entries\`

One file per word is what a hover or F1 can point at later, and what a search lands on. A word covered on another
word's page (`THEN` on `if`) is a row of `words.toml`'s `[page]`. *Instead of:* one long file (no address per
word), or a site generator now (a build the user has not asked for; Markdown can feed one later).

### D2. The inventory is computed, with a hand-kept file for what the sources cannot say

`inventory` unions the four sources and writes `docs\reference\inventory.json`. `words.toml` holds the batches,
which spec maps to which batch, prefix rules, single-word overrides, pages that cover several words, compounds
(`LINE INPUT`), words no source lists, and words to skip. A word's batch is the batch of the spec that names it
most, a mention in a requirement's heading counting most. That guess is rough for the built-in function spec,
which names nearly every function, so prefixes and overrides correct it; each batch's session corrects the rest.

### D3. Every rule names its source in an HTML comment

`<!-- src: … -->` ends the paragraph or list item it belongs to, so it does not show when the page is rendered
and stays beside its sentence when the page is edited. Four forms, each resolved by `--check`: a spec
requirement by spec and heading, a file (a recorded program without `.bas`), `read:` a source file with lines,
and a divergence row. A sentence that ends with a colon introduces a list and needs none. *Instead of:* footnotes
(visible clutter for a reader) or a separate table of sources (drifts from the sentences).

### D4. Specs are matched by words in capitals inside code spans

`evidence` finds a word in a spec only inside backticks and only written in capitals, because specs write
BASIC's words so and write placeholders in lower case (`RUN line`). Only `language/*` specs and three compiler
specs (`pipeline`, `fixes`, `lints`) are read; the others are about tools and tests. Requirements are ordered
most specific first: the word in the heading, then in the text, then by how few other words the requirement
names. Programs are matched by words in code, comments and strings left out, except a metacommand, which is
looked for in comments too.

### D5. Availability is computed by compiling, and probes may be rejected

For each page, `render --compiler` compiles with `qb64rust -z --no-settings`: every call-site program whose
statement under test (its last line before `END`) names one of the page's words, every example, and every
`probe:` line. The counts and the messages go to `tests\reference\availability.json`, and the page's block says
"Supported", "Partly supported" or "Not supported yet" with the compiler's messages, or "Rejected, as in QB64pe"
for a form QB64pe rejects too (`FRE`). A call-site program or example that does not compile is a problem; a probe
may be rejected, since showing a rejection is what it is for. `--check` without a compiler compares the pages
with the stored file; the `reference` step passes the compiler, so a change to the compiler that makes a form
compile fails the step until `render` is run.

Found while building the pilot: no word of the inventory is wholly "not supported yet" today. The forms left so
are corners (`SOMEDAY.md`, "Deferred QB64pe features"), so the sixth pilot page is `FRE`, the rejected case.

### D6. Examples are a corpus of their own

`tests\reference\examples\<page>_<n>.bas` with `.output`, under the corpus's conventions, run by the runner's
corpus suite with `--corpus-root tests\reference`. Each starts with `$CONSOLE:ONLY` so that its output can be
recorded; the page leaves that line out and keeps the `END` prompt. An example that needs a window draws on an
image from `_NEWIMAGE` and prints its pixels (`line_1`). A file the example includes is in its `.files` folder
and is shown on the page after the program. *Instead of:* examples written in the page and extracted (the page
would then be the source of a test, and the runner would need a new input kind).

### D7. Two steps of `check`

`reference`, in the quick layer, runs `--check --compiler`. Its inputs are the files that decide its result: the
release build's sources, the pages, the examples, the call-site programs, the specs, the two divergence
registers, the tool and the inventory's sources. A recording a page cites is not an input, so that a new
recording plans only its own programs (the cost tests of `tools\check\test_check.py`); a removed or renamed one
is found at the step's next run. `reference-examples`, in the full layer, runs the examples' pass list, in the
suites VM when asked, as `differential` does.

### D8. The overlap check stays local

`overlap` reads `tools\wiki\cache\pages` only to compare: 10 words in a row of prose, or 6 lines of code. The
cache is not in the repository, so the check cannot run in CI or the VM. Each batch runs it before handing over.

## Risks / Trade-offs

- **A source that exists but does not say what the sentence says.** The check cannot see it. → The writer reads
  each source (the README's procedure), and the review batch reads samples.
- **The batch guess puts a word in the wrong batch.** → It only orders work; the batch's session moves it.
- **About 580 files to keep.** → The generated parts are refreshed by `render`, and `--check` fails when a new
  built-in or a newly supported form leaves a page out of date.
