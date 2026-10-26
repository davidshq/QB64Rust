# Design

## Context

How QB64pe formats (`study\01` §3.3; `source\qb64pe.bas`, `source\utilities\format.bas`):

- There is no separate formatter. Each statement handler of the compiler's main pass appends its pretty-printed
  text to `layout$`, with two separators: `sp` means "one space here" and `sp2` (`CHR$(10)`) means "no space here".
  Expressions get their layout from `fixoperationorder`, labels from `validlabel`, assignments from `assign`.
- Keywords go through `SCase$` or `SCase2$`. `SCase$` takes 194 literal spellings in `qb64pe.bas`, such as
  `"$End If"` and `" Then"`. `SCase2$` CamelCases a built-in's name, except `_AndAlso`, `_OrElse`, `_Dynamic` and
  `_Static`.
- The indent is `min(lhscontrollevel, controllevel)`. It is one level more inside `TYPE` and `DECLARE LIBRARY`.
  `ELSE`, `ELSEIF`, `CASE` and `$ELSE` lower `lhscontrollevel`. `IndentSubs` decides whether SUB and FUNCTION
  bodies count.
- A handler that does not set `layoutdone` leaves `layout$` empty, and the line is written exactly as read. So is
  a line joined by `_`, and so is every line between `'$FORMAT:OFF` and `'$FORMAT:ON`.
- `apply_layout_indent$` turns the separators into text and applies the indent. With `AutoLayout` off, it puts the
  layout's letter-case changes onto the user's own spacing, and it makes two corrections: `?` becomes `Print`, and
  a missing closing quote is added.
- `-y` writes each line into a buffer and copies the buffer to `-o` only after both passes succeed. Any compile
  error therefore means no output.
- With no `-f:` setting, the settings come from QB64pe's `config.ini`. Its built-in defaults are `IDE_AutoFormat`
  true, `IDE_AutoIndent` true, `IDE_IndentSUBs` true, `IDE_IndentSize` 4, and both keyword settings false
  (CamelCase).

Measured on 2026-10-25 (scratch program, the clone's `qb64pe.exe`, the five defaults given explicitly):

- `MYVAR` becomes `myVar` after `DIM myVar AS LONG`.
- `myfunc(2)` becomes `myFunc(2)` when it is written before `FUNCTION myFunc`.
- After an implicit `A$ = "x"`, a later `a$` becomes `A$`.
- `call MySub(5)` becomes `Call mySub(5)`.
- `len` and `_rgb32` become `Len` and `_RGB32`, the spelling of the built-in table that `tools\builtins\builtins.json`
  already holds.
- `?MYVAR` becomes `Print myVar`.
- A procedure header keeps a space before its parameter list (`Sub mySub (n)`), but a call has none
  (`myFunc(2)`).

What exists on our side:

- A lossless tree that prints back byte for byte (`crates\syntax`), with one parser module per statement family.
- `sema`'s symbol table (`crates\sema\src\symbols.rs`). It keeps the definition span of every variable, procedure,
  label and constant. The definition is the first name in source order, which is the old compiler's "first
  declaration" for implicit variables. It does not keep user type names or member names.
- The driver's `check_program`, which the CLI and the language server share through `check_for_server`.
- The language server's `Check` hook (`crates\lsp\src\lib.rs`). It lets the server run the whole front end
  without `qb64rust-lsp` depending on `sema`.
- The runner's `--suite format`, which runs QB64pe's `format_tests` with any compiler. The old compiler passes
  24 of 24 (`baselines\`).
- The extension's `Formatter` (`vscode\src\format.ts`). It writes a temporary file beside the document and runs
  `qb64pe -y -q` with no `-f:` setting.

## Goals / Non-Goals

**Goals:**

- `qb64rust -y` output equal to `qb64pe -y` for every program of the oracle (spec `testing/format-oracle`) and
  for all 24 of QB64pe's format tests.
- A formatter that cannot change a program. Each oracle program gives the same typed tree before and after, and
  formatting is idempotent.
- Formatting in the editor with no `qb64pe.exe` process and no temporary file, through the server, with
  settings that belong to the extension.

**Non-Goals:**

- Formatting on type, and the `?` expansion as the user types (the old IDE's per-line behaviour).
- Layout rules of our own, or a "better" layout. Where QB64pe's layout is odd, it is copied; where it would
  change the program, there is a `DIVERGENCES.md` row (D8).
- Recording the verification families (D6).
- Hover, references, rename and completion (`m5b-language-features`).

## Decisions

### D1. A new crate `format`, after `sema`

`crates\format` (package `qb64rust-format`) depends on `base`, `syntax`, `builtins` and `sema`. The `driver`
depends on it. Its entry point is `format::format_file(map, parsed, program, file, settings) ->
Vec<u8>`. It takes the program that `check_program` produced (for the symbol table) and lays out one file, the
main file, line by line.

It needs `sema` because the spelling of names is part of the output (Context, "Measured"). It is a separate crate,
not a module of `sema` or `syntax`, so that the layout tables do not grow the front end, and so that the
`lsp → syntax, base` boundary stays as it is: the server reaches the formatter through a hook (D5).

*Alternative considered:* a formatter over the tree alone, with names left as written. That is simpler, but the
oracle would fail on every program that spells a name two ways, and the editor would lose the old IDE's
best-known behaviour.

### D2. Layout by statement, over the tree, in QB64pe's own form

For each statement node, the formatter writes a line in QB64pe's intermediate form: elements joined by `sp`
("one space") or `sp2` ("no space"), with keywords already in the chosen case. Then a port of
`apply_layout_indent$` (D4) turns that into text. The rules are grouped by statement family, one module for each
family of the parser:

- `crates\format\src\stmt\` holds the statement families: control flow, declarations, I/O, graphics, and so on.
- `crates\format\src\expr.rs` holds expressions, which is `fixoperationorder`'s layout: operators, unary minus,
  commas, parentheses, members and the `#` of file numbers.
- `crates\format\src\walk.rs` holds the line pass: the indent level, labels, line numbers, `:` between statements,
  and the comment after the statements (planned as a `line.rs`; built as the walk over the tree, with the
  `stmt\mod.rs` rules for labels).

Each rule is taken from the handler in `qb64pe.bas` that builds that statement's layout, and each module's
comment names the handlers it follows. A statement kind whose handler sets no layout is written as read. That list
is `crates\format\src\stmt\mod.rs` `KEPT`, and it is filled from the oracle, not guessed.

The oracle decides every rule. A rule is written only when a recorded output shows it. Where the oracle shows
nothing for a form, the form follows the general rule (one space between elements, none inside parentheses or
before `,` and `;`), and the closing runs list the forms no recorded program reached (D6).

*Alternative considered:* a general pretty-printer with spacing chosen by token pairs alone. Rejected after the
measurement: the same token pair is spaced differently by different handlers (`Sub mySub (n)` but
`myFunc(2)`), so the statement has to be the context.

### D3. Keyword spelling from QB64pe's sources; names from the symbol table

- **Keywords and metacommands:** a table `crates\format\src\words.rs` maps each upper-case keyword form to its
  `SCase$` spelling (`END IF` to `End If`, `$ELSEIF` to `$ElseIf`). `tools\format\layout_words.py` writes the table
  from the `SCase$("…")` and `SCase2$` literals of the clone's `qb64pe.bas`, with `--check` and its own tests, in
  the same way as `tools\opengl\gl_table.py`. QB64pe is MIT, and these are short words.
- **Built-ins:** the name as `builtins.json` spells it, through the `builtins` crate. `SCase2$`'s rule and its four
  exceptions are applied for the CamelCase style. In the upper and lower styles, the name is upper-cased or
  lower-cased.
- **User names:** the bytes of the symbol's definition span, suffix included. This holds for a variable, array,
  constant, procedure or label, from the symbol at each reference (`Symbols::at`). `sema` gains two symbol kinds,
  `Type` (a user type's name, in `TYPE` and after `AS`) and `Member` (an element name, in its `TYPE` and in each
  `.member` reference), recorded in the same way in `crates\sema\src\symbols.rs` and its callers in `check\`.
  `m5b-language-features` needs both kinds too. If the oracle shows that QB64pe does not respell types or
  members, the formatter does not use the kinds, but they stay in the table.
- Letter case changes only ASCII letters, as `UCASE$` and `LCASE$` do, so CP437 bytes never change.

### D4. Settings and the final text: a port of `apply_layout_indent$`

`crates\format\src\apply.rs` ports `apply_layout_indent$` step by step. It covers:

- the indent counted from the layout's leading spaces;
- `sp` and `sp2` turned into text outside strings;
- the old indent kept when `AutoIndent` is off;
- the user's spacing kept, with only the letter-case changes carried over, when `AutoLayout` is off;
- the two corrections (`?` to `Print`, the closing quote).

Each step's comment names the lines of `format.bas` it follows. Settings are a struct
`qb64rust_base::layout::Settings`, which lives in `base` so that `lsp` can hold it. The struct carries
`auto_indent`, `indent_size`, `indent_subs`, `auto_layout` and a keyword style `Camel | Upper | Lower`. Its
`Default` is QB64pe's built-in default. The `-f:` names are parsed into it by one function,
`Settings::set(name, value)`, which the driver uses for `-y` and for a build (spec `compiler/cli`, where the
settings are checked and ignored).

Line endings: `-y` writes CRLF after every line, as `qb64pe` writes its buffer. The server keeps each line's own
line ending, because formatting never adds or removes a line. A line joined by `_` and the lines it joins stay
the same lines. The tasks measure what QB64pe does with a last line that has no line ending, and the formatter
copies it.

### D5. The server formats through a hook, like `Check`

`qb64rust-lsp` gets `pub type Format = fn(&SourceMap, &ParsedProgram, Option<&Path>, &Path, &Settings) ->
Result<Vec<u8>, Diagnostics>`. `serve` takes it beside `Check`. The driver passes `format_for_server`
(`crates\driver\src\lib.rs`), which runs `check_program` and then `format::format_file` on the main file. That
keeps the boundary of design D1 of `m2-language-server`.

The requests run on the worker that runs checks (`crates\lsp\src\server.rs`, `start_worker`), with the program
read as a check reads it. A formatting request is answered once the job finishes. A newer edit to the document
cancels it, as a check is cancelled, and the answer is then `ContentModified`.

- **Whole document:** the answer is one `TextEdit` that replaces the whole document, or none when the text is
  unchanged.
- **Range:** the whole program is formatted, and then one edit is made for each line of the range whose text
  changed. Formatting keeps the number of lines, so line *n* of the output is line *n* of the input.
- **Settings:** from `initializationOptions.format`, and `workspace/didChangeConfiguration` replaces them.
  `vscode\src\server\protocol.ts` `initializationOptions` gets the `format` object.
- **Failure:** the diagnostics are published as a check publishes them, a `window/showMessage` error names the
  first error, and the answer is an empty list.

### D6. The oracle: `tests\format\`, recorded by a runner suite, checked in tier 1

- **Recording:** `run_legacy_tests.py --suite format-corpus`, with `qb64pe`, writes `tests\format\<set>\<path>`
  for each accepted program of the corpus, upstream, snippets and Civil War Strategy. Each program is run in its
  own folder (for includes) with `-y -m` and the five defaults as `-f:` settings. The suite takes the clone's lock
  as the other recording suites do. With any other compiler the suite compares; a program passes when the
  outputs are equal with line endings taken as LF. The comparison rule comes with a test in
  `test_run_legacy_tests.py`.
- **Tier 1:** `crates\driver\tests\format_oracle.rs` walks `tests\format\` and finds each source. It loads the
  source with the driver's loader, using `tests\upstream\root` as the include root for upstream, as the other
  tests do. It runs the front end and the formatter in process and compares the result with the recording. On the
  output it runs the front end again and compares the typed dumps, with spans removed and names compared without
  regard to case. It also formats the output once more and expects no change. Programs that differ are listed in
  `tests\format\known_differences.list` with a reason each, and the list is shrink-only.
- **Verification families left out:** about 4,000 programs and 5 MB. Most repeat the same statement forms that
  the corpus and upstream already cover, so recording them would add a lasting disk cost and tier 1 time for
  little new layout. The closing task reports which statement kinds the oracle never reached, from a count the
  test prints. If that list is long, a family can be added then.
- **QB64pe's own format tests:** `--suite format --qb64 target\release\qb64rust.exe` must pass 24 of 24. It is
  part of this change's closing runs.

### D7. The extension: one formatting provider at a time

- **While the server runs:** the language client's provider formats. The client registers it from the server's
  capabilities, and the server sends its own error message (D5), so the extension adds nothing to that path.
- **While the server is off:** the extension registers its own `Formatter` (`vscode\src\format.ts`) and disposes
  it again when the server starts, so VS Code never sees two formatters for `qb64rust` and never asks the user to
  choose. `Formatter` runs `qb64rust -y` or `qb64pe -y` by the rule of spec `editor/formatting` ("Which
  formatter"), through `compiler\qb64rust.ts` `format` (new) or `compiler\qb64pe.ts` `format`, both given the
  `-f:` settings from `config.ts` `formatSettings()`. It keeps today's temporary file beside the document for
  includes. Format Selection keeps only the lines in the range, as the server does.
- **Settings:** `qb64rust.format.autoIndent`, `indentSize`, `indentSubs`, `autoLayout` and `keywordCase`, in
  `package.json`, default to QB64pe's built-in defaults. A change is sent to the server with
  `workspace/didChangeConfiguration`.

Passing the settings to `qb64pe -y` as well changes the result for a user whose `config.ini` differs from the
defaults. That is intended: the extension's settings decide the layout, whichever compiler formats. The change log
says so.

### D8. When QB64pe's layout would change the program

If the oracle finds a line whose `-y` layout gives a different typed tree (the "same program" check of D6 fails
on the recording itself), the formatter does not copy it. The line gets a row in `DIVERGENCES.md`, the program
gets a `.divergences` sidecar the comparison reads, and the bug gets a row in `UPSTREAM.md`. None is known yet.

### D9. Where each piece lives

| Piece | Place |
|---|---|
| Layout of a file | `crates\format\src\lib.rs` `format_file` |
| Statement rules | `crates\format\src\stmt\*.rs`; expressions `expr.rs`; the lines, their indent, labels, `:` and comments `walk.rs` and `stmt\mod.rs`; spelling of keywords and names `fmt.rs` and `lay.rs` |
| Keyword spellings | `crates\format\src\words.rs`, written by `tools\format\layout_words.py` |
| Settings and the final text | `crates\base\src\layout.rs` `Settings`; `crates\format\src\apply.rs` |
| Type and member symbols | `crates\sema\src\symbols.rs` (`SymbolKind::Type`, `Member`), recorded in `crates\sema\src\check\` |
| `-y` | `crates\driver\src\main.rs` (`-y`, the settings) and `lib.rs` `format_program` |
| Server | `crates\lsp\src\lib.rs` `Format`; `server.rs` requests and settings; `crates\driver\src\lib.rs` `format_for_server` |
| Oracle | `tests\format\`; `tools\legacy_tests\run_legacy_tests.py` `--suite format-corpus`; `crates\driver\tests\format_oracle.rs` |
| Extension | `vscode\src\format.ts`, `languageServer.ts`, `compiler\qb64rust.ts`, `config.ts`, `package.json` |

## Risks / Trade-offs

- **The long tail of handler rules.** QB64pe has a layout branch in almost every handler. The oracle covers the
  forms that real programs use, but a rare form can be laid out wrongly without any test knowing. Mitigation:
  the general rule for unmeasured forms, the "same program" check (a wrong layout can look odd but cannot change
  the program), and the count of statement kinds the oracle never reached (D6).
- **Tier 1 time.** About 1,300 programs, each run through the front end three times. If the test takes more than
  about 20 s in a debug build, it runs the typed-tree and idempotence checks on the corpus and Civil War Strategy
  only, and the equality check on all of them. The measured time is recorded in the test's comment.
- **`config.ini` no longer used.** A user who set the old IDE's layout options has to set them again in VS Code.
  The change log and `vscode\README.md` say so.
- **Respelling across includes.** The tasks measure whether a name declared in an included file respells the main
  file, as the spec says. If QB64pe does otherwise, the spec follows the measurement.
- **The race of a long format with typing.** A format answered after an edit is refused by VS Code. The server
  answers `ContentModified` itself when a newer version came in, so no stale edit is applied.

## Measured

Task 1.1, 2026-10-25, the clone's `qb64pe.exe` with `-y -m -f:autoindent=1 -f:autoindentsize=4 -f:indentsubs=1
-f:autolayout=1 -f:keywordcapitals=0 -f:keywordlowercase=0`, scratch programs in the session's scratchpad. The
program `main.bas` (CRLF lines, the last line `print 9` with no line ending) and the file it includes, `inc.bi`:

```
' inc.bi                               ' main.bas
DIM SHARED totalCount AS LONG          REM main
TYPE MyPoint                           '$include:'inc.bi'
    xPos AS INTEGER                    TOTALCOUNT = 1
    yPos AS INTEGER                    DIM p AS mypoint
END TYPE                               p.XPOS = 3
                                       print p.xpos, TotalCount
                                       ␠␠␠               (a line of three spaces)
                                       mylabel:
                                         10 print 1
                                       x = 1    ' a comment
                                       y = 2'tight
                                       z = 3:   rem   after
                                       sub foo
                                       $if win then
                                       print 1
                                       $else
                                       print 2
                                       $end if
                                       lab2:
                                       end sub
                                       print 9
```

gives

```
Rem main
'$Include:'inc.bi'
totalCount = 1
Dim p As MyPoint
p.xPos = 3
Print p.xPos, totalCount
␠␠␠
mylabel:
10 Print 1
x = 1 ' a comment
y = 2 'tight
z = 3: Rem   after
Sub foo
    $If WIN Then
        Print 1
    $Else
print 2
    $End If
    lab2:
End Sub
Print 9
```

- **A name declared in an included file respells the main file**: `TOTALCOUNT` and `TotalCount` become
  `totalCount`, as the spec says. The included file is not written.
- **User type names and member names are respelled** (D3): `mypoint` becomes `MyPoint` after `AS`, and `XPOS`
  and `xpos` become `xPos` after `.`. So the formatter uses the kinds `Type` and `Member`.
- **The last line with no line ending** gets CRLF like every other line. **A line of spaces only** is written as
  read (its three spaces stay): the handler sees an empty line and sets no layout.
- **Labels and line numbers are indented** as the statements around them: `lab2:` one level inside the SUB; in a
  second program, `for i=1 to 2` / `lab3:` / `20 print i` / `30` / `lab4: print 2` / `next` gives `For i = 1 To 2`
  / `    lab3:` / `    20 Print i` / `    30` / `    lab4: Print 2` / `Next`.
- **A comment after a statement** is written after one space, whatever the spacing before it (`x = 1 ' a comment`,
  `y = 2 'tight`). The text of the comment, and the text after `REM`, is kept (`Rem   after`).
- **`$IF` inside a SUB** is indented as a block: `$If` one level in, its body two, `$Else` and `$End If` one. The
  condition is written in capitals (`$If WIN Then`, from the upper-cased line). The lines of the branch that is
  not taken are written as read (`print 2`): the handler skips them and sets no layout.
- **An error found only in the second pass** (`goto nowhere`, "Label 'nowhere' not defined") gives exit status 1
  and no output file, as an error of the first pass does.
- **The command line**: without `-o`, `qb64pe -y` prints `Formatting requires specifying output file with -o
  option` and exits with 1. `-f:AutoIndentSize=65` prints `AutoIndentSize must be in range 1-64.` and the
  settings help, exit 1. A boolean setting takes `true`, `on`, `yes`, `1`, `-1`, `false`, `off`, `no` or `0` in
  any case (`ParseBooleanSetting&`); another value prints `Invalid boolean value for temporary (-f) setting:
  <token>`, exit 1. `AutoIndentSize` is read with `VAL`, so `-f:AutoIndentSize=4x` is 4.

Task 2.3, 2026-10-25: the "same program" check on the 1,180 recordings (`qb64rust --dump typed` of each source
and of its recording, each in a scratch folder with what it includes; an upstream program in a copy of the
clone's `tests\compile_tests` with a junction to QB64pe's sources). 1,178 give the same dump. One is skipped: the
snippet `call_absolute__call_absolute_basic`, whose source the new compiler rejects (`CALL ABSOLUTE`, not supported
yet). **One differs**, a case of D8: in `tests\format\corpus\slice\s131_arraycopy.bas` line 167, `-y` writes
`_ARRAYCOPY g(1.5 TO 3.5) TO gd(0.5)` as `_ArrayCopy g(1.5E+0,1.5 To 3.5E+0,3.5) To gd(0.5E+0,0.5)`, the internal
element of each number (`ArrayCopyRefCaseLayout$` lays out raw elements), which is another program. Rows added:
`DIVERGENCES.md` D-038, `UPSTREAM.md`, and the sidecar `tests\format\corpus\slice\s131_arraycopy.divergences`.

Also found while reading QB64pe's sources for group 3: the lines between `'$FORMAT:OFF` and `'$FORMAT:ON` are not
written exactly as read. `'$FORMAT:OFF` turns `IDEAutoIndent` and `IDEAutoLayout` off, so each line keeps its own
indent and spacing, and `apply_layout_indent$` still carries the layout's letter case over
(`format_tests\meta_format\meta-full-cc.out`: `DATA 0,   0,   0` becomes `Data 0,   0,   0`, ` one   AS LONG`
becomes ` one   As Long`). The spec `compiler/format` ("Lines left as written") is amended to say so.

At the review, 2026-10-25, the same `qb64pe.exe` with `-y -q` on `print 1`: a keyword setting given twice counts by
its last value, as QB64pe decides the style after reading every option. `-f:KeywordCapitals=1 -f:KeywordCapitals=0`
gives `Print 1`, as do `-f:KeywordLowercase=1 -f:KeywordLowercase=0` and `-f:KeywordCapitals=1
-f:KeywordLowercase=0 -f:KeywordCapitals=0`; `-f:KeywordCapitals=1` alone gives `PRINT 1`. `Settings` in
`crates\base\src\layout.rs` was changed to match (it had kept `Upper` for the first).

## Open Questions

None for the user. The forms the oracle never reaches are reported at the close (D6), and QB64pe behaviours worth
reviewing go to `SOMEDAY.md` (`CLAUDE.md` working rule 11).
