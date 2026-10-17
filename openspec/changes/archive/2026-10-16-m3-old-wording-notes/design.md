# Design

## Decisions

### D1. A generated table, looked up by an exact key

The pairs are made by running this compiler, so the table follows its messages as they are; the script writes the
table and the word list it used (`old_wording_words.txt`: the parser's reserved words and the built-in table's
names). A message's key: a backtick span whose words are all such words (or a metacommand, a string, signs) stays
as written; any other span becomes `` `{}` ``; a number outside the spans `{#}` (a digit run joined to a letter, a
`_` or a dot is no number). `qb64rust_base::old_wording` makes the key the same way and looks it up exactly, so a
pattern the pairs disagreed on is simply absent and never falls back to a wider one.

*Alternative:* a note written by hand at each of the ~300 places that raise an error, from the measured message in
the comment above it. Rejected: 91 places have such a comment, it does not say how a name is shown in the old text,
and nothing would check the notes against recorded output.

### D2. What is kept

A pattern is kept when all its pairs give one old template, the template differs from the message (case and
backticks aside), names no file, and holds no number and no word in capitals that is neither a keyword nor in the
message (such text belongs to one program: "1st function argument", "Cannot convert type (FOO)"). Names in the old
text are found as written, in capitals, without `()`, without a type suffix. A pair often fits several of these
forms (`count` is itself without a suffix), so each placeholder keeps every form all its pairs allow (`{1:=|S}`),
and the note is left out where those forms give the current name different texts: `count&` gets none, since no
recording showed whether the old compiler keeps the suffix (it does not: measured with `qb64pe.exe`, "Variable
'count' (LONG) not defined").

### D3. Where the note is shown

`Diagnostic::render` adds a second line `... note: QB64pe: <wording>` with the error's position; the summary and
the exit status are unchanged, and the runner reads only the last line. The language server appends
`\nQB64pe: <wording>` to the message (no related-information entry: it would need a location).

### D4. Keeping it current

A changed message loses its note until the table is regenerated (its key is gone); nothing wrong is shown. The
script and its `--check` run at a change's close when messages changed (`crates\README.md`, tier 2).

## Risks / Trade-offs

- An error no recorded program reaches gets no note (the narrow form chosen).
- A pairing on the same line can still pair two different errors of that line; the agreement rule drops a pattern
  whose pairs differ, and `--check` holds the kept ones to the recorded messages.
