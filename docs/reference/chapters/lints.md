# Lints

A lint is a warning about code that compiles but is probably a mistake: a misspelled name, a routine or label
nothing uses, a local variable that hides a shared one, and code that can never run.

The pages of this area: [SHARED](../entries/shared.md), [SELECT CASE](../entries/select-case.md),
[$IF](../entries/meta-if.md), [OPTION](../entries/option.md).

## When lints are given

The lints run after the whole program has been checked, and only when it has no errors. A lint is an ordinary
warning: the compiler prints it only with `-w`, and the editor shows it. It never counts as an error, never changes
the exit status, and never changes the program that is built.
<!-- src: compiler/lints "When lints run" -->

Each lint has a code. QB64Rust's own five warn only about the file being compiled, not about a file it includes with
`$INCLUDE`. QB64pe's four are given wherever QB64pe gives them, included files too.
<!-- src: compiler/lints "Codes"; compiler/lints "Only the file being compiled" -->

## The lints

| Code | The warning |
|---|---|
| `used-once` | "`x` is used only once: is it misspelled?", for a variable no declaration made whose name occurs once. Not for a `FOR` counter or a target of `INPUT`, `LINE INPUT` or `READ`, and never with `OPTION _EXPLICIT` |
| `unused-procedure` | "SUB `s` is never called", for a SUB or FUNCTION no statement calls or names. A `DECLARE` line is no call. Not for `SUB _GL`, a routine of a `DECLARE LIBRARY` block, or a test SUB |
| `unused-label` | "label `done` is never used", for a named label nothing names. Not for a line number |
| `hides-shared` | "this `n` is a new local variable, which hides the `DIM SHARED` one of the main module", for a `DIM` or `REDIM` in a procedure |
| `unreachable` | "this code is never run: no label follows the GOTO on line 1", once, at the first statement after `GOTO`, `END`, `SYSTEM`, `STOP`, an `EXIT`, `RETURN`, `RESUME` or `RUN`, with no label between |
| `unused-variable`, `empty-select`, `duplicate-const`, `debug-incompatible` | QB64pe's own warnings: "Unused variable", "Empty SELECT CASE block", "Duplicate constant definition" and "Feature incompatible with $DEBUG mode", where `qb64pe -w` gives them |
<!-- src: compiler/lints "Codes"; compiler/lints "Name used once"; compiler/lints "Unused procedure"; compiler/lints "Unused label"; compiler/lints "Local hiding a shared variable"; compiler/lints "Unreachable code"; compiler/lints "QB64pe's warnings" -->

A variable gets at most one of `used-once` and "Unused variable". Where QB64pe's rule reports it, the warning is
"Unused variable".
<!-- src: compiler/lints "One warning per variable" -->

A `$IF` line between a leaving statement and the next ends an `unreachable` stretch, as a label does, and a name used
in a `$IF` branch the compile skips counts as used. `RUN file$` starts no `unreachable` stretch, since a handler can go
on after it.
<!-- src: compiler/lints "Code that may run after all"; compiler/lints "Names used in a skipped branch" -->

## Turning lints off

Every lint is on unless the program's settings file says otherwise. The `[lints]` table of `qb64rust.toml`, the file
that also holds the fixes ([Fixes](fixes.md)), holds `all = true` or `false` and `<code> = true` or `false`, which
overrides `all`. A lint that is off gives no warning, with `-w` or in the editor:
<!-- src: compiler/lints "Lints table of the settings file" -->

```toml
[lints]
used-once = false
```

On the command line, `--lints on` or `--lints off` replaces the file's whole set, and each `--lint <code>=on` or
`--lint <code>=off` then sets one lint. An unknown code or value, on the command line or in the file, is an error
that names the known codes.
<!-- src: compiler/lints "Command-line lints"; compiler/lints "A bad lints table" -->

`qb64rust --report warnings` counts every lint's warnings, whatever the `[lints]` table says.
<!-- src: compiler/lints "The warning report counts every lint" -->
