# Proposal

## Why

The compiler finds almost no mistakes beyond outright errors. Its few warnings are about `CONST` values and
metacommands. It also lacks four warnings that QB64pe's `-w` gives, the best known being "Unused variable", so a
user moving from QB64pe loses them. A misspelled variable name in a program without `OPTION _EXPLICIT` silently becomes a new variable
that holds 0 or `""`. Dead code, unused routines and unused labels go unnoticed. `study\43` ranks lint warnings
as the most useful tool still missing. The symbol table in `sema` already records every definition and every use,
so four of the five checks are lookups in data we have.

## What Changes

- Five new warnings, printed with `-w` as every warning is, and shown in the editor as warnings:
  - **`used-once`**: a variable that no `DIM` or other declaration made, named only once in the program.
  - **`unused-procedure`**: a SUB or FUNCTION that nothing calls.
  - **`unused-label`**: a named label that no jump, `RESTORE`, `RESUME`, `ON ERROR` or event names. Line numbers
    are never reported.
  - **`hides-shared`**: a `DIM` or `REDIM` in a SUB or FUNCTION that makes a local variable with the name and
    type of a main-module `DIM SHARED` variable.
  - **`unreachable`**: statements after `GOTO`, `END`, `SYSTEM`, `STOP`, `EXIT`, `RETURN`, `RESUME` or `RUN` in
    the same statement list, up to the next label or line number. One warning per stretch of dead code.
- The warnings of QB64pe's own `-w` that we do not give yet, in its wording and at its lines:
  - **"Unused variable"**, followed by the name and type: a variable that is created and never named again.
  - **"Empty SELECT CASE block"**: a `SELECT CASE` with no `CASE`.
  - **"Duplicate constant definition"**, followed by `name = value`: a `CONST` that repeats an existing one with
    the same value.
  - **"Feature incompatible with $DEBUG mode"**, followed by `CHAIN` or `RUN`, if `qb64pe -w` gives it outside
    the IDE. The new family `verification\v68_*` measures this and the exact rules of the other three.
- Our own lint warnings are given only for code in the file being compiled, not for code from its `$INCLUDE`
  files. A toolbox such as `MOUSE.BAS` would otherwise give one warning for each routine a program does not use.
  QB64pe's warnings follow QB64pe, which gives them in included files too.
- Lints run only when the program has no errors.
- Each lint warning carries its name as a diagnostic code. The editor shows it beside the message.
- A new report, `qb64rust --report warnings <file or folder>...`, runs the front end over many programs and
  counts the warnings by code. Nothing is built or run. It measures how noisy each lint is on the corpus.

Out of scope, each a `SOMEDAY.md` row:
- A setting to turn single lints off, or to turn them on for included files.
- Dead code after an `IF` whose every branch jumps away, and a routine that only unused routines call.

## Capabilities

### New Capabilities
- `compiler/lints`: the lint warnings, ours and QB64pe's. What each one reports, what it never reports, its
  message and code, the main-file rule, and when lints run.

### Modified Capabilities
- `compiler/cli`: a new requirement for `--report warnings`, beside "Demand report".
- `editor/language-server`: "Check diagnostics" publishes a lint warning's code.

## Impact

- `crates/base`: `Diagnostic` gets an optional `code`.
- `crates/sema`: a new module `lints.rs`, run at the end of the check. `Var` gets an `implicit` flag. The
  shadowing case in `check/decl.rs` gives its warning there.
- `crates/driver`: `--report warnings`, and `-w` output has more lines. CLI tests that compare `-w` output exactly
  may need updating.
- `crates/lsp`: the diagnostic code is passed to the client.
- `verification\v68_*`: a new family, recorded with `qb64pe -w`, for QB64pe's warnings.
- `tests/frontend`: a new mode, `warnings`, whose snapshot is the warnings. Programs where each lint fires and
  programs where it must stay quiet.
- No change to the C++ written or to any program's behavior.
