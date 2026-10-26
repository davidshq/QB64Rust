# Design

## Context

- **The server** (`crates\lsp`) parses each program on a worker after every change (`server.rs`, `Job`). It answers
  the outline, folding and definition from the trees (`symbols.rs`, `folding.rs`, `definition.rs`).
- **The two hooks**: the whole front end runs only through the hooks the driver hands it (`lib.rs`, `Hooks`):
  `check` (`extension-diagnostics`, D2 and D3: a job on the same worker, results numbered, an older one answered
  with `ContentModified`) and `format` (`m5b-formatter`, D5). `qb64rust-lsp` depends on `syntax` and `base`, never
  on `sema`. It cannot depend on `driver`, which depends on it.
- **The symbol table** (`crates\sema\src\symbols.rs`) has, for each symbol, its kind (`Var`, `Proc`, `Label`,
  `Const`, `Type`, `Member`), the span of its definition and of every reference (the name token, suffix included),
  and a lookup by position. The `Program` gives each kind's details: a variable's type, `Storage` and shape; a
  procedure's kind and parameters; a constant's value; a type's members. `dump_symbols` already writes most of
  what a hover needs.
- **The front end with errors**: `check_program` returns a `Program` together with the diagnostics, even when there
  are errors. With an error, some names may be unresolved.

## Goals / Non-Goals

**Goals:** each feature answers from the same table the compiler uses, so that a reference the server shows is one
the compiler resolved. A rename never leaves a program that means something else. The live parse stays parser-only
and as fast as it is now.

**Non-Goals:** a front end that is incremental or kept in memory between changes; features over code that is not
compiled.

## Decisions

### D1. A third hook, `index`, returning plain data

`crates\lsp\src\lib.rs` gets `pub type Index = fn(&SourceMap, &ParsedProgram, Option<&Path>, &Path) -> SymbolIndex`
and a field `index` in `Hooks`. `SymbolIndex` is defined in a new module `crates\lsp\src\index.rs` and holds data
only:
- the list of `Entry { kind, name, detail, scope, def: Span, refs: Vec<Span> }`, where `detail` is the hover's
  Markdown, made by the driver, and `scope` is the procedure the symbol belongs to, or none for the main module and
  shared names;
- the procedure bodies as spans, for completion's scope;
- whether the front end reported an error, with the first one's file and line;
- a lookup by position, the same `partition_point` as `Symbols::at`.

`crates\driver\src\lib.rs` gets `index_for_server`, which calls `check_program` and turns `Program.symbols` into a
`SymbolIndex`. Its hover texts are made in a new module `crates\driver\src\hover.rs`, written from the same facts as
`dump_symbols`.

**Alternative:** the hook returns `sema::Program` behind a trait object. Rejected: the server would then need
`sema`'s types to read it. **Alternative:** `lsp` depends on `sema`. Rejected for the reason of
`extension-diagnostics` D2: the driver's hook is what keeps the server's view equal to the CLI's.

### D2. The index is a job on the parse worker, built once per text

`Job` gets an `index` request kind beside `check`. The worker builds it with the same `analyze` and
`catch_unwind`, and the result comes back as `Event::Indexed { key, seq, index }`. `Server` keeps
`indexes: HashMap<String, (u64, Arc<SymbolIndex>)>` keyed by program and the seq of the text. A request that needs
an index for the current seq is answered at once. Otherwise it is queued on `pending_index` and the job is sent,
unless one for that seq is already running. When the result comes, the queued requests are answered. A change to the
program answers the queued ones with `ContentModified`, as a formatting result for an old text is now. The index of
the old seq is kept for completion (spec "Completion": the last index).

A check also builds the index, because the check's run already produced the `Program`. `check_for_server` stays as
it is, and the `Check` hook gains no output. Instead, a check job and an index job for the same seq share the
worker's order. Task 1.1 measures whether a second run of the front end after a check matters. If it does, the
check hook returns the index too.

**Alternative:** run the front end after every parse, in the background. Rejected: that costs the front end's
time on every keystroke for features asked for now and then, and the live parse was kept parser-only on purpose
(`DECISIONS.md` 2026-10-08, 2026-10-19).

### D3. Hover

A name with a symbol gives its `detail`. A name with no symbol is looked up in `qb64rust-builtins`
(`find_any`): a built-in gives `NAME(arg types) AS ret`, or the statement's `specialformat` for a statement, with
optional arguments in brackets, from `arg_types`, `optional` and `ret`. Anything else gives no hover. No text is
taken from the wiki: the cache under `tools\wiki\` has no licence and is a local reference only (CLAUDE.md layout
table). The text is made in `crates\lsp\src\hover.rs` for built-ins, and in the driver (D1) for symbols.

### D4. References and definition

References are `def` and `refs` of the symbol at the position, mapped to uris as diagnostics are, and sorted by
file and offset. Definition (`definition.rs`) tries the parser's answer first for procedures and labels, as now,
so it stays instant and needs no index for them. For any other name it takes the index's `def`. When the
parser's answer and the index's differ for a procedure (they must not), a tier 1 test fails (task 3.2).

### D5. Rename: the checks

`crates\lsp\src\rename.rs`. `prepareRename` gives the span of the name token without the suffix. The suffix is any
trailing `$ % & ! # ~ \``, and the type suffixes of more than one character (`~%%`, `##`, `&&`, `~&`, `%&`), cut
as the lexer cuts them. The rename is refused with `RequestFailed` and a message in each of these cases:

1. **The program has an error** (`SymbolIndex`'s first error): the message names its file and line.
2. **The new name is not a valid name**: it must match the lexer's name rule, have no suffix, contain no `.`
   (a dotted name is a different symbol in QB64), not be a reserved word (the parser's keyword table), and not be a
   built-in name (`find_any`).
3. **The new name clashes**: an entry with the new name (case-insensitive, with the same suffix as one of the uses)
   is visible at any of the symbol's uses. Visible means the same scope or the main module's shared names. A
   procedure or user type clashes with any entry of that name. This is cautious: some renames the compiler would
   accept are refused. Being too cautious costs the user a second try, while being too loose could change what the
   program means.
4. **A use lies under the compiler root**: the root's files are QB64pe's.
5. **The cursor is on no symbol, or on a line number** (a label whose name is digits).

Each case has a test in the driver's protocol tests (task 4.2). The edit is a `WorkspaceEdit` with `changes` keyed
by uri: one `TextEdit` per use, over the name without its suffix.

**Before the answer, the edit is checked**: the server applies it to the texts in memory and builds the index of
the renamed program through the hook. The rename is refused ("the rename would change the program") unless the new
index has the same symbols, with the same number of uses each, and the renamed symbol now has the new name. In
QB64, a use the table missed would silently become a new implicit variable. This check catches that case, and also
catches a clash that case 3 let through. It costs one more run of the front end per rename.

### D6. Completion

`crates\lsp\src\completion.rs`. The scope at the cursor is the procedure body (from `SymbolIndex`'s bodies) that
contains it, or the main module. The items are:
- the entries of that scope, and the main module's shared ones;
- every procedure, constant, user type, and the labels of that body;
- the built-ins (one item per name, spelled as the table spells it);
- the keywords (the syntax crate's keyword table, written in the formatter's default CamelCase only if that list
  can be reached without depending on `format`, otherwise in capitals).

The client filters, so the server sends the full list once per request, and `isIncomplete` is false. With no index
for the current text, it uses the last one, or only built-ins and keywords if there is none.

### D7. Members after `.`

After `x.`, the type of `x` is known only from the index's entry for `x` (its `detail` names the type, and the
members are the type's entries). If the entry for `x` has a user type, completion after the dot offers only that
type's members. Otherwise, after a dot, it offers nothing. This covers `a.b` for a scalar variable, but not
`a(i).b` or `a.b.c`, which would need the expression's type. Those are left out.

### D8. `DECISIONS.md`

One row: the language server runs the front end for hover, references, rename, completion and the wider definition,
built once per text through a hook of the driver, as for a check.

## Risks / Trade-offs

- [Code in a `$IF` branch that is not compiled has no symbols: a rename leaves it unchanged, and references miss
  it] → the rename's message says so when the program has inactive `$IF` regions (the trees mark them,
  `InactiveCode`). `README.md` names the limit. The alternative, a text search, could rename the wrong names.
- [The front end on a large program, such as Civil War Strategy at about 9,000 lines, may be slow enough to feel
  on a hover] → task 1.1 measures it on Civil War Strategy in a debug and a release build. Above about 300 ms in
  release, the index is built after each parse, after a pause longer than the debounce. That would be a
  `DECISIONS.md` row.
- [A rename of a variable that is declared implicitly in two procedures renames only the one under the cursor]
  → this is right, since they are two variables. The hover says "local to", so the user can see it.
- [The symbol table misses a use, for example a name inside a form `sema` reads in its own way] → D5's check of
  the edit refuses the rename, so it is never silently wrong. Tier 1 renames each symbol of the corpus's slice
  programs to look for such misses (task 4.3).
