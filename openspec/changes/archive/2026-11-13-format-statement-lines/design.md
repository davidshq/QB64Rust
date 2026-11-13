# Design

## Context

Motivation and scope: `proposal.md`. The rules a user sees: the delta of spec `compiler/format`.

How the formatter works today (`crates\format`):

- `walk.rs`, `Fmt::plan`, makes one `Plan` for each source line. It sorts the tree's statements, colons and
  `ELSE` words into `events[line]`, then walks the lines in order with a `Pass` that carries the block depth.
  For each line it builds one layout (`Lay`) and one indent.
- `lib.rs`, `format_file`, writes one output line for each source line.
- `apply.rs`, `apply_layout_indent`, is a port of QB64pe's function. With `AutoLayout` or `AutoIndent` off it
  reads the source line to keep its spacing or its indent.
- In an incomplete program, `format_file` compares each output line with its source line (`reading`) and
  falls back when they read as other tokens.

So "one source line, one output line" is assumed in four places: the plan, `format_file`, the server's range
formatting (`crates\lsp\src\server.rs`, `formatted`) and the extension's server-off path
(`vscode\src\formatPlan.ts`, `selectionEdits`).

Measured on 2026-11-12 with `target\release\qb64rust.exe`, on two hand-written pairs of programs (packed, and
the same program split and with block `IF`s): `--dump typed`, `--dump ir` and `--dump cpp` are equal once line
numbers are masked (`line N`, `#line N "file"`, `evnt(N)`). The forms were: `a: b`, a single-line `IF` with
`ELSE` and colons, `THEN 100`, `IF y GOTO 100`, `FOR … : NEXT`, `x = 1: IF a THEN` followed by a block, nested
single-line `IF`s, an empty `THEN` branch, `SELECT CASE` on one line, a statement after `ELSEIF … THEN` and
after a block's `ELSE`, a label before two statements, and `DO: … : LOOP UNTIL`. A single-line `IF` and a
block `IF` are the same node in the typed tree. Nothing was measured with `qb64pe.exe` yet; D9 does that
first.

## Goals / Non-Goals

**Goals:**
- With both settings off, not one byte of output changes. The format oracle proves it.
- A rewritten program is the same program, and a test over 1,180 programs says so.
- The rules are few. Where a case is doubtful, the line is left as it is.

**Non-Goals:**
- Matching any other formatter's style. `qb64pe -y` has no such feature, so there is no oracle for the text.
- Rewriting lines the formatter already leaves as read.
- Any change to the parser, `sema` or code generation.

## Decisions

### D1. The settings are two more layout settings

`one_statement_per_line` and `block_if` are fields of `Settings` and `Overrides`
(`crates\base\src\layout.rs`). `Settings::set` reads `OneStatementPerLine` and `BlockIf` with `boolean`, so
`-f:` and `--format-default` take them with no change in `crates\driver\src\main.rs`. `Settings::apply`
copies them from the `[format]` table. `format_table` and `FORMAT_KEYS` (`crates\driver\src\settings.rs`)
read `one-statement-per-line` and `block-if`. The server's `format_settings` (`crates\lsp\src\server.rs`)
reads `oneStatementPerLine` and `blockIf`. The extension adds the two to `package.json`, `config.ts` and
`DEFAULT_FORMAT`.

*Why not a separate command (`qb64rust --rewrite`)?* The rewrites need the formatter's indent and layout for
the new lines. A second tool would call the formatter anyway and would need its own settings, its own server
request and its own tests. As settings they get the settings file, the command line, the server and format on
save with no new mechanism.

*Why `-f:` names that QB64pe does not have?* `--format-default` takes the names of the `-f:` layout settings,
and the extension passes its settings that way when the server is off. The extension never passes the two to
`qb64pe -y` (`vscode\src\format.ts`).

### D2. A source line plans a list of output lines

`Plan::Layout` holds a list of `(layout, Apply)` pairs, not one. `Fmt::plan` builds the list. For a line that
is not rewritten the list has one pair and the code path is today's, so the output cannot change.

For a line that is rewritten, `plan` cuts `events[line]` into segments and runs today's loop body once for
each segment: its own `Lay`, its own `lhs`, and its own indent `min(lhs, pass.level)` taken when the segment
ends. `Pass` carries from segment to segment as it carries from line to line. So `For i = 1 To 3` is laid out
at the old level, `Print i` one level in, and `Next` back out, by the code that already does this for three
source lines. This body moves to a function `Fmt::segment` in `walk.rs`.

`format_file` writes each pair followed by CRLF. A new `format_lines` in `crates\format\src\lib.rs` returns
the output of each source line as a list of lines; `format_file` is `format_lines` joined. The server uses
`format_lines` (D7).

*Alternative: rewrite the source text first, then format it.* That needs a second parse and check of the
whole program, and a text rewrite that knows about strings, comments and `DATA`. The tree already has every
statement's span, and the plan already walks them in order.

### D3. Where a line is cut

`Fmt::collect` records more than it does now, so that `plan` can cut without looking at the tree again:

- `Event::Colon` gets a flag `in_line_if`: the colon's parent is a `LineBranch`.
- A new `Event::LineIfEnd` is pushed after the children of an `IfStmt`.
- `Event::Stmt` of an `IfHeader` already carries its container (`IfStmt`, `IfBranch` or `ElseIfBranch`).

With `one_statement_per_line`, a cut is made:
- at a colon that is not `in_line_if`, when a statement stands before it and after it on the line; the colon
  is dropped;
- after the header of a `CaseClause`, after `Event::Else { block: true }`, and after the `IfHeader` of an
  `ElseIfBranch`, when a statement follows on the line.

With `block_if`, the events of an `IfStmt` are cut as D4 says. Inside an `IfStmt` that is turned into a
block, a colon counts as not `in_line_if` for the first rule, so that with both settings each statement of a
branch gets its line.

No cut is made after the first statement-like event of a line when it is a `LineNumber` or a `LabelDef`. The
parser reads a SUB's name and a colon as a `LabelDef` too, and the formatter writes it through `label_def`
(`crates\format\src\stmt\mod.rs`), so this covers `DrawBoard: PRINT`. *Why keep them together?* Whether
`name:` is a call or a label depends on every jump of the program (spec `language/control-flow`, "A SUB's name
and a colon"). Leaving the text alone keeps this change out of that question. A label on a line of its own is
a `SOMEDAY.md` row.

One `LabelDef` is cut: a built-in statement's name and a colon (`CLS: PRINT 1`, `BEEP: x = 1`). The parser
reads it as a label and `sema` as the statement, always, whatever the jumps (spec `language/control-flow`,
"A line that is a built-in statement's name and a colon"). `label_symbol` tells a label from the rest but
not a built-in from a SUB call, so the test is new: the name is a built-in SUB in the built-ins table
(`qb64rust_builtins::find_any`, `Kind::Sub`) and `label_symbol` is false. When the line is cut after it, the
statement is written without its colon. Lines such as `CLS: LOCATE 1, 1` are common in old programs, so
leaving them packed would be the first thing a user sees.

A line is rewritten only when at least one cut is made. Then a colon at the start or the end of a segment is
dropped. This covers a colon between two segments, one at the end of the line, and one right after `THEN` or
`ELSE`: today's formatter writes `IF x THEN: x = 4` as `If x Then: x = 4` and `IF x THEN ELSE: x = 3` as
`If x Then Else: x = 3` (measured 2026-11-12), and both are single-line `IF`s in the tree.

### D4. A single-line `IF` as a block

When `block_if` is on, `plan` treats the events of an `IfStmt` as the events of an `IfBlock`:

- The `IfHeader` ends a segment. Its text is the header's layout as today. For `IF c GOTO x` the header has
  no `THEN`; `Fmt::statement` writes ` Then` after it in this mode. `pass.level` goes up by one after the
  segment, as `block_effect` does for the header of an `IfBranch` (`crates\format\src\stmt\mod.rs`).
- The statements of the first `LineBranch` are one segment, or one each with `one_statement_per_line`. An
  empty branch gives no segment.
- `Event::Else { block: false }` is a segment of its own with `lhs` one less, as a block's `ELSE`.
- An `ImplicitGoto` (`THEN 100`) is laid out as `GoTo 100`: the keyword from `self.kw("GOTO")`, then the
  number as it is written today.
- `Event::LineIfEnd` gives a segment `End If` (`self.kw("END IF")`) and takes `pass.level` down by one.

A nested `IfStmt` in a branch goes through the same steps one level in.

With `block_if` on and `one_statement_per_line` off, statements before the `IF` on the line stay on the
header's line (`x = 1: If a Then`). `qb64rust` reads that as a block (measured, Context). D9 measures it with
`qb64pe.exe` before this is built. If the old compiler does not read it as a block, the rule becomes: a cut is
made before the `IF` too, and the spec scenario "A statement before the IF, BlockIf alone" changes to match.

### D5. The comment goes above

When a line is cut and has a comment, the comment is the first output line: its layout from `Fmt::comment`,
at the indent of the first segment. *Why above, and not at the end of the first or the last line?*

- At the end of the last line it reads as a comment on that statement only.
- At the end of the `IF … THEN` line, a `REM` comment would change the program: the parser reads
  `IF x THEN REM note` as a single-line `IF` (`block_if_ahead`, `crates\syntax\src\parser\blocks.rs`), so the
  `END IF` after it would be an error. Above, `'` and `REM` need no separate rule.

A line whose comment is a `MetaComment` (`'$DYNAMIC`, `'$INCLUDE`) is not rewritten at all (D6), because such
a comment acts at its place in the program.

### D6. Lines that are never cut

`plan` makes no cut on a line that is `as_read` (joined by `_`, or stale after one), `inactive`, has
`pass.format_off` set, has a `MetaStmt` event or a `MetaComment`, or where `settings.auto_indent` or
`settings.auto_layout` is false. The last one is because `apply_layout_indent` reads the source line in
those two modes, and a new line has no source line. All of these are tests on values `plan` already has.

A rewrite setting that is on while `auto_indent` or `auto_layout` is off does nothing in the whole file, and
would do so without a word. So it gets a warning (user, 2026-11-12): "`OneStatementPerLine` has no effect while
`AutoLayout` is off", naming the settings that apply. `format` in `crates\driver\src\main.rs` prints it to
standard error once for a run; the output is still written and the exit status is 0. The server sends it as a
`window/showMessage` warning from `formatted` (`crates\lsp\src\server.rs`), once until the client's settings
or the settings file change, so that format on save does not repeat it. The test is on the settings after
all their layers. `'$FORMAT:OFF` is not a setting and gets no warning: the author put it there.

### D7. Only a complete program is rewritten

`format_file` passes `complete` to `Fmt`; `plan` makes no cut when it is false. *Why:* in an incomplete
program `format_file` checks each output line against its source line with `reading` and falls back when
the layout misread a statement. That check is one line against one line. A check for several lines would be
new code guarding a case (rewriting a program that does not compile) nobody has asked for.

The cost: `qb64rust -y lib.bm` alone does not rewrite, since an include file alone is incomplete. In the
editor the server formats an include file inside its program, which is complete when the program has no
error, so it is rewritten there. The command line needs `--program` (`SOMEDAY.md`, "`qb64rust -y` of an
include file inside a named program"); this change does not build it.

### D8. The server: range formatting and code actions

`crates\driver\src\lib.rs` gets `format_lines_for_server`, as `format_for_server` but returning
`format_lines`' list. `formatted` in `crates\lsp\src\server.rs` uses it for a range: for each source line the
range touches whose output differs from it, one edit that replaces the line with its output lines, each with
the document's line end. Whole-document formatting stays one edit.

Code actions (`textDocument/codeAction`, new in `server.rs`; `code_action_provider` with `resolve_provider`):

- The list of actions comes from the last parse of the document, with no check and no formatting. "Put each
  statement on its own line" is listed when a line of the range would be cut by D3's rules; "Convert to block
  IF" when a line of the range holds an `IfStmt`. Neither is listed when D6 rules the line out or the parse
  has an error. The test is a function `rewritable` in `crates\format` that takes the tree alone; `plan` and
  the server both call it, so the two cannot disagree. For the one case that needs `sema` (a built-in
  statement's name and a colon, D3), `rewritable` takes the name from the built-ins table.
- The edit is made on `codeAction/resolve`: a format job as for range formatting, with that one rewrite
  forced on and the other forced off, and edits only for the lines that become more than one line. When the
  program is incomplete (D7), the job gives no edit and the server sends a `window/showMessage` that names the
  first error. When `AutoIndent` or `AutoLayout` is off for the program (D6), it gives no edit and the message
  says that the rewrite needs both. The settings file can turn them off, and it is read only by the job, so
  the list cannot know.

*Why resolve?* VS Code asks for code actions each time the cursor moves. Formatting the whole program for
each request would run the check of a large program many times a second.

The extension needs no code for the actions; the language client shows them.

For the server-off path, `vscode\src\format.ts` runs `qb64rust -y` for Format Selection with
`-f:OneStatementPerLine=false -f:BlockIf=false`. `-f:` goes over the settings file, so `selectionEdits` keeps
its one-to-one lines. Format Document with the server off passes the two settings as `--format-default`.

### D9. How it is checked

1. **With the old compiler, first.** A family `verification\v72_*`, recorded with `qb64pe.exe`. The programs
   come in pairs: `v72_<form>.bas` holds a packed form, and `v72_<form>_r.bas` is the same program as the
   formatter is to write it with both settings (`_b` with `BlockIf` alone, where that differs). Each prints
   what the form does over the same inputs. The two recordings of a pair must be equal. The forms are those
   of Context, plus a colon right after `THEN` and after `ELSE`, and a comment moved above a line with a line
   number that a `GOTO` names. This comes before the formatter is changed, because D4's rule for
   `x = 1: If a Then` rests on it. Once the formatter is built, a test in `format_oracle.rs` formats each
   packed program and requires the text of its `_r` or `_b` file. So what `qb64pe.exe` was shown is what the
   formatter writes, not a form written by hand that may drift from it. The family's line in
   `verification\README.md` says what it shows.
2. **Tier 1, every oracle program.** `crates\driver\tests\it\format_oracle.rs`, in `run`: for each default
   recording, format the source with the first setting, the second, and both. For each output: the typed
   dump and the C++ equal the source's with line numbers masked, the comments are the same texts in the same
   order (comments are not in the typed tree, so nothing else would notice one lost), the second formatting
   changes nothing, and a walk of the output's tree finds no `IfStmt` (with `block_if`) and no line that
   `rewritable` would still cut. An incomplete program is skipped for the rewrites, as D7 leaves it alone.
3. **Rule tests.** `crates\format\tests\it\rules.rs`, one test for each scenario of the delta spec.
4. **Server tests.** `crates\lsp\tests\it\server.rs` and `crates\driver\tests\it\format_server.rs`: range
   formatting of a packed line, both code actions, and their absence.
5. **Extension.** `vscode` unit tests for the settings and for the server-off arguments.
6. **Once, the whole corpus with the old compiler** (user, 2026-11-12). Items 1 to 5 prove that our compiler
   reads the rewritten text as it reads the source. Only the `v72` pairs prove that `qb64pe.exe` does. So
   before the merge into `main`, every corpus program that has a recording is formatted with both settings
   into a scratch folder, built with `qb64pe.exe` and run, and its output is compared with the recording of
   the source. This is a large run: it goes in the suites VM and the user starts it. It is done once and is
   not a step of `check`: a standing step would cost VM time at every merge and a tool to maintain, and
   after this run items 1 to 5 hold the formatter to what it showed. The script that writes the scratch
   folder is not committed. The command, the counts, and each program that differs with its reason go under
   "Measured" in this file. A program that prints `_ERRORLINE` or an error's line number is expected to
   differ.

The C++ comparison is the costly part of item 2: three more compiles to C++ of 1,180 programs. The time of
`formatter_equals_the_old_compilers_layout` is measured before and after. If the test grows by more than 60
seconds, the C++ comparison goes to a test of its own that `just full` runs and `just quick` does not, and
tier 1 keeps the typed tree. The typed tree and the C++ agreed on every form measured so far.

Measured 2026-11-13 (tasks 5.1 and 5.2), debug build on all cores: `formatter_equals_the_old_compilers_layout` took
1.6 s before the rewrites were checked, 375 s with all of item 2, and 2.7 s without the C++ comparison. So the C++
comparison is the test `rewrites_give_the_same_cpp` in `format_oracle.rs`, marked `#[ignore]`: the `cli` step of
`just full` runs it with `--ignored`, and `just quick` does not. It passed for every program in the 375 s run.

### D10. Documents

`docs\MANUAL.md` and `vscode\README.md` gain the two keys in their `[format]` examples, with one paragraph on
what changes and the note about `_ERRORLINE`. `DECISIONS.md` gets one row. `SOMEDAY.md` gets the four rows of
the proposal's "Out of scope". No `DIVERGENCES-QB64PE.md` row: with the defaults nothing differs from
QB64pe.

## Measured

**The `v72` family with `qb64pe.exe`** (2026-11-13, task 1.1). 16 packed programs, 16 `_r` forms and 2 `_b`
forms, recorded on the host with `python tools\legacy_tests\run_legacy_tests.py --suite verification --glob
"v72_*.bas"`: 34 compiled, and each `_r` and `_b` recording equals its packed program's. So the old compiler
reads every form of D9 item 1 as its packed form. That includes `x = 1: If a Then`, then `b = 1: x = 2` and
`End If` (`v72_f_colon_then_if_b`): QB64pe takes it as a block, as `qb64rust` does. D4's rule stands, and the
spec scenario "A statement before the IF, BlockIf alone" is unchanged. The other forms: statements split at
colons with the comment above (`a`), both branches as a block (`b`), `GoTo` for `THEN 100 ELSE 200` and
`IF a GOTO 100` (`c`, `d`), `FOR` and `DO` on one line (`e`, `n`), the header of a block `IF` after a colon
(`g`), a nested single-line `IF` (`h`), an empty `THEN` (`i`), a colon right after `THEN` and `ELSE` (`j`),
`SELECT CASE` on one line (`k`), statements after `ELSEIF … THEN` and a block's `ELSE` (`l`), `CLS:`, a label
and a SUB's name before a colon (`m`), a `REM` comment above an `IF` (`o`), and a comment above a numbered line
that a `GOTO` names (`p`). With `qb64rust`, all 34 pass in the suites VM (`just check verification --where
vm`, 2026-11-13: the whole list of 4,383 programs passed in 1,912 s). With `qb64rust`, each pair's `--dump typed` and `--dump cpp` are equal once line
numbers and the file name are masked.

**The whole corpus with `qb64pe.exe`** (2026-11-13, task 8.3, D9 item 6). Each of the 490 programs of
`tests\corpus` that has an `.output` was formatted with `qb64rust -y -f:OneStatementPerLine=true
-f:BlockIf=true --no-settings` into a scratch corpus root. A program with a `<name>.files` folder was formatted
with that folder's files beside it, as the runner runs it. All 490 changed: besides the rewrites, the default
layout sets keyword case and indents. The 26 programs with `.err` or `.norun` were left as written. The suites
VM then ran `python tools\legacy_tests\run_legacy_tests.py --suite corpus --corpus-root <scratch> --qb64
..\QB64pe\qb64pe.exe --jobs 8`, through `suites.ps1` and `step.ps1` with a one-off job that copied the
scratch root in as a tar. Result: 505 passed, 4 known failures (`PRINT` with a comma never ends when stdout is
redirected), and 16 differed from their recordings. Eight programs of the first batch (`114_select_multiple`
to `120_redim_no_preserve` and `11_math_backslash`) timed out while the eight `qb64pe` copies were new, and
passed when run again alone. The 16 that differ:

- 12 differ only in line numbers, as expected: `s93_mem_error_302` to `s99_mem_error_308`,
  `s104_mem_error_313`, `s106_asserts` and `s19_header_errors` print "Runtime error: Line: n", and
  `s136_errorline` and `s139_net_errors` print `_ERRORLINE`. With every digit masked, each output equals the
  recording.
- 4 are not `qb64pe.exe`'s output even unformatted: `s114_dyn_members` and `s117_dyn_member_kinds` (D-004),
  `s118_dyn_member_files` (D-019) and `s162_get_put_index_once` (D-061) were recorded from edited programs.
  Their unformatted sources, built with `qb64pe.exe` on the host, fail the same way, and each formatted
  program's output equals its source's byte for byte.

So no difference is a bug of the formatter. The script that wrote the scratch root is not committed.

## Found while building

- **A colon in a `FOR` inside a branch of a single-line `IF` has a `ForBlock` as its parent, not a `LineBranch`**
  (`IF 1 THEN FOR i = 1 TO 3: PRINT i;: NEXT`, `verification\v70_g1`). So a flag on `Event::Colon` that tests the
  parent, as D3 says, would cut such a line with `OneStatementPerLine` alone, and break the `IF`. `segments` in
  `crates\format\src\walk.rs` counts instead how many single-line `IF`s each event is inside. The `IfHeader` of an
  `IfStmt` adds one and `Event::LineIfEnd` takes it off. Without `BlockIf` nothing inside one is cut. The same
  count keeps a `CASE` inside a branch whole. The rule test `split_keeps_a_single_line_if_whole` holds it.
- **A statement with no layout** (a `DECLARE` line, a statement the formatter writes as read) on a line that would
  be cut: the line is laid out as without the settings, and the pass is put back as it was before the line.
- Every program of the oracle passed the checks of D9 item 2 at the first run, so no rule was changed for one
  (task 5.3).
- **The server reaches `rewritable` through its hooks.** `crates\lsp` depends on no `sema` (design D1 of
  `m2-language-server`), and `crates\format` does. So `qb64rust_lsp::Hooks` has a fifth hook, `rewritable`, which
  the driver fills with `qb64rust_format::rewritable`. The type `Rewritable` is in `qb64rust_base::layout`.
- **One server formatter, not two.** D8 named a new `format_lines_for_server` beside `format_for_server`. Instead
  `format_for_server` returns a `qb64rust_lsp::Formatted`: the output lines of each source line, the settings after
  the settings file, and the check's diagnostics. The server needs all three for one answer: the edits, the warning
  of D6, and the reason a code action gives no edit.
- **A code action request waits for the first parse**, as document symbols do. Asked right after the document
  opens, it would otherwise find no parse and offer nothing.
- **"Server start" lists the capabilities exactly**, so the delta of `editor/language-server` modifies it to add
  code actions with resolve.

## Risks / Trade-offs

- **The old compiler may read a rewritten form otherwise than we do** → the `v72` family is recorded first,
  and D4 names the fallback for the one form that is in doubt.
- **A rewrite that changes a program in a form nobody thought of** → every oracle program is compared by its
  C++, and a line is cut only by the short list of D3.
- **`_ERRORLINE` and error messages name new line numbers** → inherent in moving statements. Said in the
  manual. `ERL` is not affected.
- **A comment moved above a numbered line is no longer on the line a `GOTO` reaches** → a comment does
  nothing when run; the family checks one such program.
- **More lines per file.** A program of dense one-line `IF`s grows a lot. The settings are separate so that a
  project can take one and not the other.
- **A code action is listed from the parse alone** → for a program with an error of a later stage the action
  is listed and then does nothing but show a message. Listing only after a check would run the check on each
  cursor move.
- **`fuzz-front-end` is open on this branch and touches `crates\format\src\lib.rs` and the driver's tests** →
  build on top of it; its fuzz targets format with the default settings and need no change.

## Open Questions

- Whether the fuzz target `server` should also format with the two settings on. It can wait: the `SOMEDAY.md`
  row on formatting twice as a fuzz property covers the same ground.
