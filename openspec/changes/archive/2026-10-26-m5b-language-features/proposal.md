# Proposal

## Why

M5b, "the editor without QB64pe" (`DECISIONS.md` 2026-10-25; `ROADMAP.md` M5b), has two items. The first, the
formatter (`m5b-formatter`), is built. The second is the language-server features that `sema`'s symbol table
already supports: hover, find references, rename and completion. Today the server answers from the parser alone,
apart from the check and formatting. It goes to a definition only for procedures and labels, and has nothing for a
variable, a constant, a type or a member. `sema` already records, for every resolved name, its definition, every
reference and its type (`crates\sema\src\symbols.rs`, kept for this since `m2-procedures-and-errors`). This change
puts that table in front of the user.

## What Changes

- **Hover** (`textDocument/hover`) on a name gives what `sema` knows about it:
  - a variable: its type, whether it is an array, and where it lives (main module, `SHARED`, `STATIC`, local to
    or a parameter of a procedure);
  - a constant: its type and value;
  - a procedure: its header;
  - a user type: its members;
  - a member: its type;
  - a label: the body it stands in.

  On a built-in, hover shows the built-in table's form: its arguments with their types and its result. It shows no
  help text, since the QB64pe wiki has no licence.
- **Find references** (`textDocument/references`) gives every use of the symbol under the cursor, across the
  program's included files, with or without its definition as the client asks.
- **Rename** (`textDocument/prepareRename`, `textDocument/rename`) renames every use of a variable, constant,
  procedure, user type, member or label, and keeps the type suffix written at each use. It is refused, with a
  message, in these cases:
  - the program has an error, because the uses found may then be incomplete;
  - the new name is not a valid name, is a reserved word or the name of a built-in, or clashes with another name
    that a use can see;
  - a use lies in a file under the compiler root (QB64pe's own included files);
  - the symbol is a line number.
- **Go to definition** goes on to variables, constants, user types and members, beside procedures and labels.
- **Completion** (`textDocument/completion`) offers, at the cursor:
  - the names that are visible there (the main module's shared names, and the procedure's own locals and
    parameters);
  - the procedures and constants;
  - the built-ins, with the old table's spelling;
  - the keywords.
- **How the server gets the table**: the driver hands the server a third hook, beside the check and the formatter.
  It runs the front end and returns a plain index of the symbols, so `qb64rust-lsp` still does not depend on
  `sema`. The server builds the index on the first request that needs it after a change, and keeps it until the
  next change.
- **The extension** gets the features from the server alone (the language client wires them). It gets integration
  tests and a section in `README.md`.

Out of this change:
- signature help;
- semantic highlighting;
- workspace-wide symbol search;
- references and renames inside `$IF` blocks that are not compiled (the front end does not see them, `design.md`
  "Risks");
- completion of members after `.`, if the index cannot tell the type in front of the dot without a full check
  (decided in design D7).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `editor/language-server`: "Server start" advertises hover, references, rename (with prepare) and completion. "Go
  to definition" covers variables, constants, types and members. New requirements: "Hover", "Find references",
  "Rename", "Rename refused", "Completion", and "Semantic index until the next change".

## Impact

- `crates\lsp`: the `Index` hook and its plain data types; the four requests and the wider definition; a module per
  feature beside `definition.rs`. A dependency on `qb64rust-builtins` (a table, not a stage) for hover and
  completion of built-ins.
- `crates\driver`: `index_for_server`, which builds the index from `sema`'s `Program`; the hook in `server_hooks`;
  the tests over the protocol, beside `check.rs` and `format_server.rs`.
- `crates\sema`: whatever the index needs that the symbol table does not keep (for example the procedure a label or
  a use stands in, or the span of the suffix), if anything.
- `vscode\`: integration tests, and `README.md` and `CHANGELOG.md` (unreleased).
- `DECISIONS.md`: a row recording that the server runs the front end for these requests, as it does for a check.
- No change to code generation, libqb or the formatter.
