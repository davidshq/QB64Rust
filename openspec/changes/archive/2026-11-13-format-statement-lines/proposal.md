# Proposal

## Why

Old BASIC programs pack several statements on a line with `:`, and put whole branches in a single-line `IF`.
Such lines are hard to read, and they hide a common mistake: in `IF hp <= 0 THEN PRINT "Dead": lives = lives - 1`
both statements are conditional. The formatter lays a program out as `qb64pe -y` does and never moves a
statement to another line, so a port of an old program has to be unpacked by hand.

The idea was named once, as out of scope for the change `project-settings`
(`openspec\changes\archive\2026-11-11-project-settings\proposal.md`, line 38). It has no `SOMEDAY.md` row and
no design. The user asked for it on 2026-11-12.

## What Changes

- **Two new layout settings, both off by default.** With both off, the output is `qb64pe -y`'s, as today.
  - **One statement per line.** `x = 0: y = 0` becomes two lines. A statement after `CASE 1:`, after a block's
    `ELSE`, or after `ELSEIF … THEN` goes to its own line too.
  - **Block `IF`.** `IF a THEN x = 1 ELSE x = 2` becomes `IF a THEN`, `x = 1`, `ELSE`, `x = 2`, `END IF`.
    `IF a THEN 100` and `IF a GOTO 100` get an explicit `GOTO 100` inside the block.
- **The two are independent.** With only the first on, a single-line `IF` stays whole and is never split at the
  colons inside it. With only the second on, the statements of a branch stay together on one line.
- **Where they are set**: `one-statement-per-line` and `block-if` in the `[format]` table of `qb64rust.toml`,
  `-f:OneStatementPerLine=` and `-f:BlockIf=` on the command line (and with `--format-default`), and
  `qb64rust.format.oneStatementPerLine` and `qb64rust.format.blockIf` in VS Code.
- **Lines that are left as they are**: a line joined by `_`, the lines between `'$FORMAT:OFF` and
  `'$FORMAT:ON`, a line with a metacommand, a line of a `$IF` branch that is not taken, and any line where
  `AutoIndent` or `AutoLayout` is off.
- **A label, a line number or a SUB's name and a colon at the start of a line stays with the first
  statement.** A built-in statement's name and a colon (`CLS: LOCATE 1, 1`) is split like any statement.
- **The comment at the end of a rewritten line goes to a line of its own above it.**
- **Only a program without errors is rewritten.** An include file is rewritten when it is formatted inside its
  program, as the language server does. `qb64rust -y lib.bm` alone lays it out as today.
- **Two code actions in the editor**, "Put each statement on its own line" and "Convert to block IF". They
  rewrite the line under the cursor or the lines of the selection, whatever the settings say.
- **Range formatting and Format Selection** give several lines for one line when a setting is on.
- **Checks.** Each of the 1,180 programs of the format oracle is also formatted with the first setting, the
  second, and both. The result must compile to the same typed tree and the same C++ as the input, apart from
  line numbers, and formatting it again must change nothing. A new verification family records with
  `qb64pe.exe` that each rewritten form runs as its packed form does. Once, before the merge, the whole
  rewritten corpus is built with `qb64pe.exe` in the suites VM and compared with its recordings.
- **A warning** when a rewrite setting is on while `AutoIndent` or `AutoLayout` is off, since it then does
  nothing.

One thing a rewrite does change: the number of the line a statement is on. A program that prints `_ERRORLINE`,
or an error message that names a line, shows the new number. `ERL` and line-number labels do not change.

Out of scope, each a `SOMEDAY.md` row written by this change:
- A lint that warns about a packed line or a single-line `IF` with several statements.
- Turning `ELSE` followed by a nested `IF` block into `ELSEIF`.
- Putting a label on a line of its own.
- Rewriting an include file from the command line. It needs the row "`qb64rust -y` of an include file inside a
  named program", which stays as it is.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `compiler/format`: the two rewrites and their rules are added. The `[format]` table and its error message
  name seven keys, not five. "Output equal to the old compiler's" holds with both rewrites off.
- `editor/language-server`: the server reads the two settings from the client, range formatting may give
  several lines for one, and the server offers the two code actions.
- `editor/formatting`: the extension has the two settings and passes them to the server and to `qb64rust -y`,
  never to `qb64pe -y`. Format Selection with the server off does not rewrite.
- `testing/format-oracle`: tier 1 checks the three rewrite variants of each recorded program.

## Impact

- `crates\format`: `walk.rs` plans several output lines for one source line; `lib.rs` writes them and gives
  the server the output of each source line.
- `crates\base\src\layout.rs`: two fields in `Settings` and `Overrides`, two `-f:` names.
- `crates\driver`: `settings.rs` reads the two keys; `tests\it\format_oracle.rs` runs the variants.
- `crates\lsp\src\server.rs`: the two settings, range formatting over line groups, `textDocument/codeAction`.
- `vscode`: two settings in `package.json` and `config.ts`; `format.ts` and `formatPlan.ts` for the server-off
  path.
- `verification\`: family `v72_*`, recorded with `qb64pe.exe`.
- Documents: `docs\MANUAL.md` and `vscode\README.md` (the `[format]` table), `DECISIONS.md`, `SOMEDAY.md`.
- No new dependency. Nothing changes for a program or project that does not turn a setting on.
