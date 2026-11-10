# 43. Features and tools of modern languages that we lack

Written 2026-11-04, in answer to the user's question "What common features or tools of modern languages are we
missing?". It compares the project with the tool chains of Rust, Go, Python and TypeScript. Nothing here is
decided; it is a list to choose from.

**Sources.** The state of the project comes from `STATUS.md`, `SOMEDAY.md`, `crates\README.md` and the merged specs
under `openspec\specs\`. What other languages offer comes from Claude's general knowledge, not from a search.

## What we already have

Most of a modern tool chain is built:

- In the editor: diagnostics as you type, hover, completion, find references, rename, go to definition, the
  outline, folding and a formatter (spec `editor/language-server`).
- A source-level debugger (spec `editor/debug-adapter`).
- Statement coverage (`--coverage`, spec `testing/coverage`).
- Unit tests with `qb64rust test` and the Test Explorer (specs `compiler/unit-tests`, `editor/test-explorer`).
- A settings file, `qb64rust.toml`, for the fixes switch (spec `compiler/fixes`).
- Value tracing, `--trace-values`, is in progress (the OpenSpec change `value-trace`, `study\42`).

## Tools

These are listed most useful first.

1. **Lint warnings.** The compiler has almost no warnings of its own. The ones there are cover a chained `^` in a
   `CONST` and math constants that depend on the CPU (`DECISIONS.md` 2026-10-08 and 2026-10-18). No check for
   unused variables was found, which QB64pe's `-w` has. The highest-value lint for BASIC is a name used only once
   in a program without `OPTION _EXPLICIT`, which is almost always a typo. Others:
   - unreachable code after `GOTO`, `END` or `EXIT`;
   - a label nothing jumps to;
   - a local that hides a `SHARED` name;
   - a `SUB` or `FUNCTION` nobody calls.

   `sema`'s symbol table (`crates\sema\src\symbols.rs`) already holds what these need.
2. **Documentation from comments.** Hover shows only a procedure's signature. A `'` comment block directly above a
   `SUB` or `FUNCTION` could appear in hover, completion and signature help, as JSDoc and rustdoc comments do.
   For user code, this does not wait on the wiki's license (`STATUS.md`, "Queued for later milestones").
3. **Signature help, semantic highlighting, workspace symbol search and clickable `$INCLUDE` links.** All four are
   rows of `SOMEDAY.md`, "VS Code parity left". Any modern language server has them, and the data for them
   already exists.
4. **Quick fixes in the editor.** None exist. Examples:
   - "declare this variable" when `OPTION _EXPLICIT` is on;
   - "add the `_` prefix" for a `$NOPREFIX` program;
   - renumbering or removing line numbers;
   - fixes attached to diagnostics.
5. **A way to share libraries.** QB64 users copy `.bi` and `.bm` files by hand. Even a small manifest helps: a
   `[dependencies]` table in `qb64rust.toml` naming a git repository or a folder, with the include path resolved
   from it. QB64pe has nothing like this.
6. **Run a selection or a scratch file**, in place of a REPL. A true REPL fits badly with a compiler that goes
   through C++. "Run this selection" or "run this one-line expression" in a temporary build comes close, and the
   build cache makes it fast.
7. **A timing profiler.** `--coverage` counts statements. Recording time per `SUB` (or sampling) on top of the same
   instrumentation would show where a game spends its frames.
8. **Inlay hints.** They could show:
   - the type of an undeclared variable (`a` is `SINGLE` because of `DEFSNG`);
   - parameter names at call sites;
   - the values of `CONST`s.

   Implicit typing is the classic BASIC trap, so this fits the language well.
9. **Watch mode.** `qb64rust run --watch` would build and run the program again on every save.
10. **Already rows of `SOMEDAY.md`:** fuzzing the parser, a web build (the QBJS row of "VS Code parity left"),
    Marketplace publishing, and a one-step installer.

## Language features

The project's goal is the QB64pe 4.7.0 dialect (`ROADMAP.md`, "Purpose"). So each item here would be a language
extension. A program that uses one would no longer build with QB64pe. Each needs a decision and a switch of its
own, as for names of more than 40 characters (`SOMEDAY.md`, "Other ideas raised during the study"). The usual
missing ones:

- **Modules or namespaces.** Every name is global. `$INCLUDE` pastes text; it is not an import.
- **Structured errors.** There is `ON ERROR GOTO` but no `TRY`/`CATCH`. `SOMEDAY.md` names a `TRY` error model as
  one of the conditions that would bring back the runtime redesign.
- **Fixed arrays inside a `TYPE`.** QB64 users often ask for these, and BASIC PDS 7.1 had them (`SOMEDAY.md`,
  "Popular BASIC PDS 7.1 features").
- **`BYVAL` for BASIC procedures.** Today it is allowed only in `DECLARE LIBRARY`.
- **Optional or default parameters.**
- **Procedures as values:** function pointers, callbacks or closures.
- **Collections:** dictionaries or maps, and lists that grow without `REDIM _PRESERVE`.
- **String interpolation.** The row on an expression as the `INPUT` prompt (`SOMEDAY.md`) is a related small case.

## The panel's view

- **Pragmatic engineer:** do tools 1 to 4 first. Every user benefits, nothing in the language changes, and the data
  already exists. Leave the library manager until there are users who want to share code.
- **QB64 engineer:** the typo lint (tool 1) and the type hints (tool 8) catch the bugs BASIC programmers actually
  hit. Language extensions split the community unless QB64pe takes them too.
- **Compiler engineer:** doc comments and the lints are both small passes over the symbol table, and the "used
  once" lint is worth more than the rest together. Each language extension touches the parser, `sema`, the IR and
  the code generator, so each would be an OpenSpec change.
- **Rust engineer:** quick fixes need text edits that keep the lossless tree intact, and the tree already allows
  that.
- **Test engineer:** each lint needs programs where it must fire and corpus programs where it must stay quiet.
  Running the lints over the 3,890 programs of `tests\verification\pass.list` would measure how often they flag
  correct code.
- **Extra role, a QB64 user porting an old program** (added because the World War 2 port of 2026-11-04 is the
  newest real use): BASIC PDS's `DIR$`, `ON LOCAL ERROR` and `CALL INTERRUPT` matter more to this user than modern
  features. They are already rows in `SOMEDAY.md`.

**Recommendation:** start with the lints, mainly the "name used once" typo check. Then doc comments in hover with
signature help, then quick fixes. Language extensions are a separate decision, for later.
