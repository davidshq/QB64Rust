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

An index job, and the job that checks a rename's edit, carry a flag the server clears when their result would be
dropped (a change of the text, a cancelled rename); the worker skips a job whose flag is cleared before it starts
it. Completion starts an index job after each edit without waiting for it, so without the flag a run of edits and
completion requests on a large program would queue front-end runs ahead of the parses that give diagnostics (found
in review). A cancelled request that waits for an index is answered `RequestCancelled`; the index job goes on.

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
file and offset. A FUNCTION and its result variable (`Entry::twin`, found by task 4.3) give the uses of both, with
the header as the definition, so that the references show what a rename edits (found in review). Definition (`definition.rs`) tries the parser's answer first for procedures and labels, as now,
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
4. **A use lies under the compiler root**: the root's files are QB64pe's. A file in the main file's folder is the
   user's even when that folder is under the root (QB64 users often keep their programs in the QB64pe folder, and
   with the plain rule every rename of such a program would be refused); a file under the root's `internal` folder
   is always QB64pe's (found while building task 4.2's tests).
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
- [The front end on a large program, such as Civil War Strategy at 5,075 lines, may be slow enough to feel
  on a hover] → task 1.1 measures it on Civil War Strategy in a debug and a release build. Above about 300 ms in
  release, the index is built after each parse, after a pause longer than the debounce. That would be a
  `DECISIONS.md` row. Measured below: well under it, so the index stays on demand.
- [A rename of a variable that is declared implicitly in two procedures renames only the one under the cursor]
  → this is right, since they are two variables. The hover says "local to", so the user can see it.
- [The symbol table misses a use, for example a name inside a form `sema` reads in its own way] → D5's check of
  the edit refuses the rename, so it is never silently wrong. Tier 1 renames each symbol of the corpus's slice
  programs to look for such misses (task 4.3).

## Measured

### Task 1.1: the front end's time (2026-10-25)

`check_program` with the disk checks, over a parse with the files loaded from disk, five runs each, on this machine
(a temporary example binary, deleted after; the dev profile is `opt-level = 1`, the release one adds overflow
checks). "Again" is a second `check_program` over the same parse, as an index job after a check of the same text
would run it.

| Program | Build | Parse | Front end | Again |
|---|---|---|---|---|
| Civil War Strategy (`CWSTRAT.BAS`, 5,075 lines, no error) | dev | 100–104 ms | 72–104 ms | 75–86 ms |
| Civil War Strategy | release | 118–155 ms | 43–60 ms | 44–55 ms |
| `slice/s143_const_funcs` (611 lines, the largest slice program) | dev | 1.1–1.5 ms | 14–16 ms | 13–15 ms |
| `slice/s143_const_funcs` | release | 0.9–1.1 ms | 10–12 ms | 10–12 ms |

The parse is the larger part on Civil War Strategy, and the server makes it for every request job anyway. The
front end stays far under the 300 ms of "Risks" in either build, so **the index is built on demand** (D2, no
`DECISIONS.md` row). A second run after a check costs as much as the first, about 50 ms in release on the largest
real program: too little to give the check hook an output, so `Check` stays as it is.

### Task 1.2: what the index needs beside `Program`

| Need | Where it comes from |
|---|---|
| A symbol's scope | a variable's `Storage` (`Static`, `Local`, `Param`, `Result` name the procedure; `Main` is the main module), a label's and a constant's `proc`; types and members have none |
| Which main-module names a procedure sees | **added to `sema`**: `Program::shared`, the variables declared `DIM SHARED` or `COMMON SHARED` (the checker's `dim_shared`, kept until now only while checking). A name a procedure takes with a `SHARED` statement is seen there by its uses inside that body, which the symbol's `refs` already hold |
| The bodies, for completion's scope | `Proc`'s `file`, `line`, `end_file`, `end_line`, turned into a span by the driver (a body that ends in another file runs to the end of its first file) |
| The suffix within a name's span | not needed from `sema`: the span covers the name token with its suffix, and the server cuts the suffix from the text as the lexer does (D5) |
| A procedure's header as text | made by the driver from `Proc`'s kind, name and parameters (each a variable with its type and shape), as `dump_symbols` makes its lines |
| The first error, for a rename | the diagnostics `check_program` returns |
| Every use of a procedure's name | **added to `sema`** (found with task 2.1's snapshot of `proc_scopes`): the name of a `DECLARE SUB` or `DECLARE FUNCTION` is recorded as a use of the procedure of that name and kind, if there is one. The checker still ignores the statement otherwise, as the old compiler does; without the use, a rename would leave the `DECLARE` naming the old name |

No behaviour changes: `Program::shared` is read by nothing in the compiler, and `cargo test -p qb64rust-sema` passes
with the `typed` snapshots unchanged.

### Task 3.2: the parser's definition and the index's

Over the 3,270 uses of procedures and labels in the slice programs' main files, the parser's definition differed
from the index's at ten, all a line that is a procedure's name and a colon (`twice:`, `h_tick:`): with no jump to
the name anywhere it is a call of the procedure (`verification\v28_c_labels`), which only the whole program can
tell. The parser now gives no answer on a label named like a procedure header, and the index answers. Where the
parser finds nothing (a procedure of a `DECLARE LIBRARY` block, the target of `RESTORE` in a procedure or of
`ON TIMER … GOSUB`), the index answers too.

### Task 4.3: every symbol of the slice programs renamed

Each symbol renamed to a fresh name keeping its first letter, then the program with every rename at once checked
for errors and its typed dump compared, names aside. What it found, each fixed here:

| Found | Fix |
|---|---|
| A FUNCTION and its result variable are two symbols of one name: renaming one left `f = …` in the body naming the old name, a new local, and the check of the edit (D5) passed, as the counts of uses did not change | the index links the two (`Entry::twin`); a rename edits both |
| A new name's first letter may have another `DEFtype`, changing an implicit variable's type unseen by D5's check | the index keeps each symbol's type (`Entry::ty`); D5's check compares it |
| `m.OFFSET` of a `_MEM`, whose members are no symbols, was taken whole as the variable's name; `p.x` likewise | a use's name ends at the dot after the defined name, or before a member's use inside the token |
| `p . r(1)`, the array `p.r` written with blanks, one token: only `p` was renamed | the name is matched blanks aside |
| `ON TIMER(n) show` and the other event handlers: the SUB's name was no use of it | **added to `sema`**: recorded as a use |
| `_MEMGET(m, o, t)`: the type's name was no use of it | **added to `sema`**: recorded as a use |
| QB64pe's own constants (`_TRUE`) and the names of `afterlastline.bm` were offered for renaming | refused (a constant of the auto-included file; a file under the root's `internal`) |
| A new name with one leading `_` is QB64's own; names such as `__a` are the program's | `check_name` takes leading `_`s, and refuses exactly one |
| A procedure of a `DECLARE LIBRARY` block without `ALIAS` is named as its C function | refused |

The test takes about 3 s in a debug build; no symbol is on its list of known misses.
