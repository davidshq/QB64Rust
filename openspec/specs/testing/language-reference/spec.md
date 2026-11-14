# testing/language-reference Specification

## Purpose
A reference of the language QB64Rust compiles, one page per word, whose every rule names a source in this
repository and whose every example is a recorded program, checked by a tool so that no person needs to read every
page to trust it.

## Requirements

### Requirement: The inventory of words
`tools\reference\reference.py inventory` SHALL write `docs\reference\inventory.json`: every word of the built-in
table, of the parser's reserved words, of the formatter's spellings and of the VS Code grammar's hand-written
lists, with the words of `tools\reference\words.toml`'s `[extra]` and without those of its `[skip]`. Each word
SHALL have its kinds, its sources, its page and its batch. A word that only one source other than the built-in
table names, and that no `[page]` row covers, SHALL be listed in `one_source`.

#### Scenario: A helper word
- **WHEN** the inventory is written
- **THEN** `THEN` is on the page `if` and has the batch of `IF`

#### Scenario: A compound
- **WHEN** a recorded program has `LINE INPUT a$`
- **THEN** it is a use of `LINE INPUT`, not of `LINE`

### Requirement: The evidence of a word
`reference.py evidence <WORD>` SHALL print the word's entries in the built-in table; every requirement of the
`language/*` specs and of `compiler/pipeline`, `compiler/fixes` and `compiler/lints` that names it in capitals in
a code span, the most specific first; the recorded programs under `verification\`, `tests\corpus\`,
`tests\differential\`, `tests\upstream\` and `tests\sandbox\` that use it, with their first comment and their family's line of
`verification\README.md`; its call-site programs with their statement under test; and the rows of
`DIVERGENCES-QB64PE.md`, `DIVERGENCES-QB45.md` and `SOMEDAY.md` that name it.

#### Scenario: A metacommand in a comment
- **WHEN** `evidence $INCLUDE` is run
- **THEN** the programs that hold `'$INCLUDE: '…'` in a comment are listed

### Requirement: The page check
`reference.py --check` SHALL fail, naming the page and the cause, when: a generated block, an embedded example or
a generated file differs from what `render` writes; a paragraph or list item under "Behavior" or "Errors" that
does not end with a colon has no `src:` comment; a source names a spec requirement, a file or a divergence row
that does not exist; a required heading is missing, a heading is unknown, or the headings are out of order or
repeated; a page holds `TODO`; a hand-written "Syntax" of a page with a built-in leaves out a word of the
built-in table's forms that starts with `_`, or `STEP`; a relative link leads nowhere; a page belongs to no word
of the inventory; an example has no page or is not shown on it; or a page of a batch named in `words.toml`'s
`done`, or a pilot page, is missing.

#### Scenario: A hand-written syntax that leaves out an option
- **WHEN** the page `_putimage` writes its syntax by hand without `_SMOOTH`, which the built-in table's form has
- **THEN** the check fails, naming the page and `_SMOOTH`

#### Scenario: A rule without a source
- **WHEN** a paragraph under "Behavior" has no `src:` comment
- **THEN** the check fails, naming the page and the start of the paragraph

#### Scenario: A source that was renamed
- **WHEN** a `src:` names `verification\v20_c_nothing`, which does not exist
- **THEN** the check fails with "no file"

### Requirement: What render writes on a page
`reference.py render` SHALL write the generated syntax of a page's built-ins from the built-in table, with the
table's keywords in capitals and one form per line where the table joins two with " or ". It SHALL make each word
written as code under "See also" a link to its page, when the word, or a page's title, names a page other than
this one; other code stays as it is.

#### Scenario: A form joined with "or"
- **WHEN** the table's form of `STICK` is `STICK(direction%) or STICK(direction%, axis_number%)`
- **THEN** the page `stick` shows `STICK(direction%)` and `STICK(direction%, axis_number%)` on two lines

#### Scenario: A word under See also
- **WHEN** the page `put` lists `` `GET` `` under "See also"
- **THEN** `render` writes it as a link to `get.md`, and a word on the page's own page stays code

### Requirement: The availability of a page
`reference.py render --compiler <qb64rust>` SHALL compile, with `-z --no-settings`, every call-site program whose
statement under test names a word of the page, every example of the page, and every `probe:` line of the page,
and SHALL write for each page the number of programs, the number that compile, and the compiler's messages for
the others, split into "not supported yet" and other errors, to `tests\reference\availability.json`. A call-site
program or an example that fails with an error other than "not supported yet" SHALL be a problem; a probe SHALL
NOT. `--check --compiler` SHALL compute the same and fail when it differs from the stored file.

#### Scenario: A rejected built-in
- **WHEN** the page `fre` has the probe `PRINT FRE(-1)`
- **THEN** its availability says "Rejected, as in QB64pe" with the message "`FRE` is not implemented in QB64pe"

#### Scenario: A form that becomes supported
- **WHEN** the compiler changes so that a form that was "not supported yet" compiles
- **THEN** `--check --compiler` fails until `render --compiler` is run

### Requirement: The examples are recorded programs
Each example SHALL be `tests\reference\examples\<page>_<n>.bas`, recorded with `qb64pe.exe` under the golden
corpus's conventions and held to `tests\reference\pass.list` by the runner's corpus suite with `--corpus-root
tests\reference`. The page SHALL show the program without its first line `$CONSOLE:ONLY`, the source files of its
`.files` folder, and its recorded output.

#### Scenario: An example's output changes
- **WHEN** `qb64rust` prints something other than an example's `.output`
- **THEN** the step `reference-examples` fails for that example

### Requirement: The overlap with the wiki
`reference.py overlap` SHALL read the wiki cache only to compare, and SHALL fail when a page's prose, generated
blocks and comments left out, shares a run of 10 words with a wiki page's prose, or an example shares 6 lines of
code, blank and comment lines left out, with a wiki page's code. It SHALL write nothing.

#### Scenario: A copied sentence
- **WHEN** a page holds a sentence of 10 or more words taken from a wiki page
- **THEN** `overlap` fails, naming the page, the wiki page and the words
