# 27. Sixth review: architecture after the core built-ins (2026-10-08)

Asked by the user: a panel review of where the codebase and the plans are, and whether there are big changes or
major bugs to consider; no nits, no unnecessary churn. The panel (`CLAUDE.md` rule 6, role-played inline): a
pragmatic engineer, a QB64 engineer, a compiler/languages engineer, a Rust engineer, a test engineer, and an
**editor-tooling engineer** (added because the next step is the language server, and its design points are about
the editor boundary, not the compiler).

Evidence: `STATUS.md`, `ROADMAP.md`, `DECISIONS.md`, `study\23`–`26`, `crates\README.md`, the IR, `sema`, emitter and
driver sources, the extension's `diagnostics.ts` and `config.ts`, and the committed tree (`8efb975`, nothing in the
working tree but the untracked `ROADMAP.md`). Measured for this review with the release build:

- `cargo test --workspace`: green (tier 1, about 10 s);
- every corpus program with a recorded output not on `slice.list` (74) and every upstream program neither passing
  nor deferred (208), run through `--dump typed`, its "not supported yet" messages grouped by kind with the names
  kept (the fifth review's table collapsed statement names);
- the front end on the old compiler's own source: `--dump tree` of `qb64pe.bas` with its 50 included files (28,828
  lines in the main file) in 0.6 s, `--dump typed` (parse, check to the 100-error cap) in 0.43 s; a small corpus
  program in 40 ms.

## 1. Verdict

The architecture holds and nothing found needs rework. No bug was found: tier 1 is green, the two shrink-only
lists of false errors and parse gaps are empty, the full corpus and upstream runs of session 24 produced no wrong
executable, and the "clean implies listed" guard of the fifth review is in place. The layering (seven crates,
dependencies downward, source as bytes, the IR sharing `sema`'s value tree) has taken four slices without strain,
and the lints of `study\21` are still respected.

What the panel found is in the plan, not the code: **the corpus is now blocked by built-in statements, file I/O
and `DATA`, which no step of "Next" names and the IR has no operation for** (§3); **step 9 as written ("the
remaining 249 plain built-ins") would move neither yardstick much** (§4); **"`Ty` as a type table" rests on a
premise that is gone** (§5); and the language server, the next step, has **five design points its proposal must
settle at the editor boundary** (§6). The measurements also confirm an earlier decision: a full reparse per edit
is fast enough that no incremental analysis is needed.

## 2. Where the code is

- 21,700 lines of Rust in seven crates (`syntax` 8,300, `sema` 7,300, `driver` 2,100 of which 1,500 are tests,
  `ir` 1,800, `codegen-cpp` 1,500); the largest file is the generated-looking but hand-written `syntax\src\ast.rs`
  (1,700 lines of typed accessors), the next `parser\blocks.rs` at 1,100. Three dependencies (`insta`,
  `serde_json`, `tempfile`), all for tests.
- Front end complete in breadth: every program the old compiler accepts parses without a diagnostic and gets no
  real error; every statement form is a node. Compiled in depth: scalars, procedures, error handling, every
  operator, `CONST`, control flow, `SELECT CASE`, `ON … GOTO/GOSUB`, static arrays and `TYPE` of the main
  module, 43 built-in functions. Everything else is "not supported yet" (28 marker sites in `sema`).
- The IR's operations: `Assign`, `AssignAll`, `Print`, `Call` (of a procedure), `Jump`, `Branch`, `Gosub`,
  `Return`, `SetHandler`, `Raise`, `Resume`, `Exit`, `End`, `System`, `SelectConsole`. Built-in functions are
  expression nodes (`ExprKind::Call`); there is no built-in **statement** in the IR.
- Tests: tier 1 runs about 1,000 inputs through the front end with round trip and lists, 104 `tests\frontend`
  programs by mode line, snapshots, the mutation test, the built-in coverage check; tier 2 192 slice programs and
  42 of 279 upstream, green on GitHub against the 4.7.0 release.
- The extension (M1, 1,200 lines of TypeScript) runs the old compiler for diagnostics on save and for formatting;
  it already assumes one byte per character for column positions (`diagnostics.ts`, the Latin-1 remark), which is
  exact for CP437, the default encoding.

## 3. Finding: the corpus is blocked by statements the plan does not name

The 74 corpus programs with a recorded output that are not on `slice.list`, by the kinds that block them
(programs mentioning each; 20 are blocked by exactly one kind, each a different one):

| Kind | Programs | Kind | Programs |
|---|---|---|---|
| `KILL` | 23 | `RESTORE`, `REDIM`, `INPUT`, `WRITE`, `MKDIR`, `RMDIR`, `RANDOMIZE`, `RND` | 3 each |
| `OPEN`, `CLOSE` | 22 each | `GET`, `EOF`, `LOC`, `LOF`, `FREEFILE`, `ERASE`, `ENVIRON$`, dynamic arrays, `TYPE` members | 2 each |
| `PRINT #` | 16 | `SWAP`, `TIMER`, `SHELL`, `COMMAND$`, `CHAIN`, `DEFINT`, `OPTION BASE`, `WIDTH`, `TAB`, `SPC`, `PRINT USING`, `EXIT SELECT`, `_IIF`, `_CLAMP`, `_CEIL`, the hyperbolic and `_SHL`/`_SHR` functions | 1 each |
| `READ`, `DATA` | 10 each | | |
| `LINE INPUT` | 9 | | |

So the corpus's next unlock is **sequential file I/O** (`OPEN`, `CLOSE`, `PRINT #`, `WRITE #`, `LINE INPUT #`,
`INPUT #`, `EOF`, `LOF`, `LOC`, `FREEFILE`, `KILL`, `MKDIR`, `RMDIR`: about 25 programs, most of them needing all of
it), then **`DATA`/`READ`/`RESTORE`** (10; parsed since parser breadth group 5, not compiled), then one-offs. None
of these is in steps 7–11 of `STATUS.md` "Next": step 9 says "the remaining plain built-ins", and in the built-in
table "plain" means a plain **call form** (210 functions and 44 subs), while `OPEN`, `PRINT #`, `LINE INPUT`,
`INPUT`, `LOCATE`, `COLOR` and 92 more are **special-format statements** read by their `specialformat` template
(98 subs and 74 functions). `DATA` and `READ` are statements of the language, not built-ins at all.

What it takes (compiler engineer): an `Op::Builtin { id, args }` in the IR, the statement twin of
`ExprKind::Call`, with the same slot model (`Option` per slot in table order) and the same `Rule` mechanism in
`sema\src\builtins.rs`; the emitter's `builtins.rs` already writes a call by its rule. File numbers and channels
are libqb's (`sub_open`, `sub_close`, `qbs_print #` and friends): the IR stays ABI-neutral by naming the built-in,
not the libqb function. `DATA` needs the old compiler's data segment (one `data.txt`-style fragment and a
`READ`/`RESTORE` cursor; `study\02`). Everything measured first, as usual: the corpus runner records stdout, so
the oracle for file I/O is a program that writes a file and reads it back; `LINE INPUT` under the runner's
redirected stdin and `KILL` of a file that does not exist need their old-compiler behaviour pinned (`verification\`
style).

**Recommendation:** make this **step 9**, in place of "the remaining 249 plain built-ins": *built-in statements
and functions by demand*, starting with the built-in statement operation in the IR, sequential file I/O,
`DATA`/`READ`/`RESTORE`, `SWAP`, `RANDOMIZE`/`RND`/`TIMER`, `INPUT`/`LINE INPUT` from the console, `SHELL`,
`COMMAND$`, `ENVIRON$`. The remaining plain functions come in as the corpus, upstream or a user names them; the
table-driven checker makes each one a row and a test. The pragmatic engineer adds: this is the step after which
the corpus number stops being "208 of 263 with 55 to go" and becomes "nearly all"; it is also what a QB64
programmer tries first after `PRINT`.

## 4. Finding: upstream is walled by types and arrays, and step 9 as written would not move it

The 208 upstream programs neither passing nor deferred, by kind (programs mentioning each; only 22 are blocked by
one kind):

| Kind | Programs | Where in the plan |
|---|---|---|
| a name used without `()` that is no FUNCTION (arrays in procedures, implicit arrays, array parameters) | 73 | step 11 |
| arrays as `TYPE` members | 57 | step 11 |
| `REDIM` | 51 | step 11 |
| a type the slice lacks (`_UNSIGNED`, `_BYTE`, `_BIT`, `_OFFSET`, `_MEM`), fixed-length strings, type suffixes | 48, 16, 13 | step 8 |
| `_DEST`, `$CONSOLE` (the metacommand count, 35) | 43 | step 10 |
| `_MEMFREE`, `_MEMGET`, `_MEM` and the image functions (`_NEWIMAGE`, `_FREEIMAGE`, `_LOADIMAGE`, `_PUTIMAGE`, `_WIDTH`, `_HEIGHT`, `_SOURCE`…) | 10–19 each | nowhere (M3: programs that draw) |
| `CHDIR`, `ERASE`, `OPEN`/`CLOSE`, `FREEFILE`, `PUT`, `COMMAND$`, `MKL$` | 10–18 each | §3 |

The first three rows are step 11, the last step of "Next": the largest unlock by far (`study\24` said so of the
model; the data now says so of the yardstick), where the model already holds (`Place`, store rules, `Ty::User`). The
panel does **not** ask to move it: step 8 (the type set) must come first (`_UNSIGNED` and `_OFFSET` run through
`REDIM`'s and `_MEM`'s arguments), and step 10 was decided yesterday with reasons. But two things follow for the
plan:

- "The remaining 249 plain built-ins" as one tranche is the wrong unit. Of the 210 plain functions, about a third are
  image, sound, window and input functions (`_RGB32`, `_MEMIMAGE`, `_SNDOPEN`, `_MOUSEX`…) with no stdout oracle
  until M3's screen-state dump, and many of the rest need `_UNSIGNED`, `_OFFSET` or `_MEM` arguments from step 8
  (`_MK$`, `_CV`, `_CAST`, `_ROL`, `_SETBIT`, `VARPTR`, `PEEK`); generating tests for them now would pin nothing. Hence
  §3's "by demand".
- Upstream's "x of 279" will stay near 42 until step 11; the corpus is the signal for steps 7–10, and after §3 it
  will be close to its ceiling. The panel suggests saying so in `STATUS.md` once, so that the number is not read
  as stalling.

## 5. Finding: drop "`Ty` as a type table"

Step 8 says "`Ty` as a type table, unsigned types". The idea comes from `study\20` §3.8: "`Ty` must become an
interned type table before unsigned types, fixed-length strings and user types". User types have since landed as
`Ty::User(TypeId)`, an index into `Program::types`, without a table of types and without strain (`study\26` §8:
"places went in without strain"). What is left to add has no identity to intern either:

- the fixed-width numeric types are finite variants (`_BYTE`, `_UNSIGNED _BYTE`, `_UNSIGNED INTEGER`, `_UNSIGNED
  LONG`, `_UNSIGNED _INTEGER64`, `_OFFSET`, `_UNSIGNED _OFFSET`);
- `_BIT * n` and `_UNSIGNED _BIT * n` carry a width, `STRING * n` a length: `Ty::Bit(u8, Sign)`, `Ty::FixedStr(u32)`
  (two values are the same type exactly when their variants and numbers are);
- `_MEM` is one fixed layout.

The cost of adding variants is the 520 matches on `Ty::` in 24 files that `wildcard_enum_match_arm` forces to list
every variant. That is the lint doing its job (every conversion, size and store rule must say what it does with
`_UNSIGNED LONG`), and it is paid once; a table would hide the same decisions behind predicates and lose
exhaustiveness. Two things do change with the new variants (Rust engineer): the derived `PartialOrd` on `Ty`, which
the conversions use for "wider than", stops meaning anything once unsigned and signed widths interleave, so it goes
and an explicit rank function takes its place (`study\21` already said so); and `Ty` should stay `Copy`, which the
variants above allow. **Recommendation:** step 8 reads "the full numeric type set and fixed-length strings, with
an explicit rank, as measured by the differential tester"; no type table.

## 6. The language server: design points for its proposal

Decided in `study\23` §2.6: syntax errors, the document outline and folding ranges from the block nodes, go to
definition for procedures and labels; the extension's other diagnostics stay on the old compiler. The editor-tooling
engineer lists what the `m2-language-server` proposal must settle; each has a recommended answer.

1. **Where it lives.** A subcommand `qb64rust lsp` over stdio in the one binary, its logic in a new `lsp` crate
   (`driver -> lsp -> syntax`), on `lsp-server` and `lsp-types` (the dependency policy of 2026-10-07 names them;
   this is the first non-dev dependency, so `cargo-deny` arrives with it, `study\21` item 7). The extension gets
   one setting for the path of `qb64rust.exe` next to the existing `compilerPath` for `qb64pe.exe` and starts the
   server only when the binary is found; the binary is not shipped inside the `.vsix` yet (that is the publishing
   work in `ROADMAP.md`, "not tied to a milestone"). Not a separate binary: one artifact to find and version.
2. **Parser only, no `sema`.** The parser's diagnostics are exactly the syntax errors; procedure definitions and
   labels are tree nodes (`ast::ProcDef`, label nodes) and go to definition is a name match over them (upper case,
   file order, first definition wins, as the old compiler resolves). `sema` would add nothing the decision asks
   for and would add "not supported yet" for most statements of a real program, which must never reach the editor
   as a squiggle. The full parse of `qb64pe.bas` and its includes takes 0.6 s in release and a normal program a few
   milliseconds, so a full reparse per change with a short debounce and cancellation is the design, as decided on
   2026-10-04 (no salsa); the server runs it on a `with_stack` thread as the driver does.
3. **The encoding boundary.** The compiler takes bytes and reports byte columns; LSP delivers the document as text
   and takes positions in UTF-16 code units (VS Code's client; whether it negotiates `positionEncoding: utf-8`
   under LSP 3.17 is to be checked in the proposal, not assumed). The server must encode the text back to the
   document's encoding before parsing (CP437 by default, `files.encoding` per document; the extension passes the
   encoding, for example in `initializationOptions` with a per-document override on open, since LSP's `didOpen` has
   no field for it) and map byte columns back to UTF-16 columns. For a single-byte encoding both maps are the
   identity, which is what the extension assumes today for the old compiler's columns; for a UTF-8 document the
   server keeps a per-line table. A character the encoding cannot hold (an emoji in a CP437 file) becomes one
   byte (`?`) so that columns stay aligned; the proposal says so. The encoding table itself is 256 entries of
   public data, written in the `lsp` crate, not a dependency.
4. **Included files.** The `Loader` trait is the right seam: the server's loader serves an open document's current
   text first and reads the disk second, with the include root from a setting (the old compiler's rule: the
   including file's folder, then the compiler root, never the workspace). The main file is the program; an included
   `.bi`/`.bm` edited on its own is parsed as a program of its own for its syntax errors (its `$IF` state is the
   defaults then), and the programs that include it are reparsed on its change. A program's diagnostics are
   published per file, as the extension already does for the old compiler (`owned` in `diagnostics.ts`).
5. **Two diagnostic sources.** The server's diagnostics are live and syntax-only; the old compiler's arrive on
   save and cover everything. They live in two collections with the sources "qb64rust" and "qb64pe", neither
   clearing the other (today the old compiler's are cleared on edit; keep that). The same syntax error may show
   twice until the first edit; that is accepted rather than a merge rule.
6. **Tests.** Protocol tests in Rust (`lsp-server` can be driven in-process: open, change, request outline and
   definition, check diagnostics), the extension's integration suite extended with the server started from a built
   binary, and `vscode-extension.yml` building the Rust binary first (today it tests against `qb64pe.exe` only).

What the server must **not** do yet (test engineer): hover, completion, rename, references, variable definitions,
or anything from `sema`; each of those is a later change with its own measured facts (hover needs the wiki licence
question answered, `STATUS.md` "Queued").

## 7. Looked at and left alone

- **`sema`'s `Checker`**: 35 fields of state, `check\mod.rs` at 721 lines. The fields are named and each has a
  comment; the scope model (`main`, `local`, `param_scopes`, `Scope` with name-and-type keys) matches what was
  measured. Watch it at step 11 (`REDIM`, arrays in procedures, `DEFxxx` add scope state); no change now.
- **The "keep" list**: 17 old-compiler behaviours implemented as measured, awaiting step 8 as planned
  (`study\26` §8 said "not later than the built-ins"; it is before them, §3).
- **`Facts::may_raise` is conservative** (every built-in call and string operation may raise): extra
  `is_error_pending` checks in the generated C++, never a missing one. Fine until a built-in carries a "cannot
  raise" flag (M3).
- **The libqb pin**: building through `..\QB64pe` with the clone untouched, and CI against the 4.7.0 release,
  still holds; the M3 copy at the pinned commit stays as decided. Upstream moves (`_ARRAYCOPY` and the member-array
  markers are in `SOMEDAY.md`); the corpus is recorded against 4.7.0, so nothing drifts under the tests.
- **`STATUS.md`** at 286 lines carries session narrative (the M1 walkthrough, the upstream-tests session) beside
  the one-line "Next"; it is the entry point and still readable. Moving the closed sections to `study\00` is
  tidying, not this review's business.
- **`ERR` typed LONG** until unsigned types, `tests\upstream\root` as the compiler root, the 64 MiB thread, the
  100-error cap: all fine.

## 8. Decisions for the user

**All four accepted by the user on 2026-10-08** (`DECISIONS.md`; steps 8 and 9 reworded in `STATUS.md`).

1. §3 and §4: **step 9 becomes "built-in statements and functions by demand"**: the built-in statement operation
   in the IR, then sequential file I/O, `DATA`/`READ`/`RESTORE`, `SWAP`, `RANDOMIZE`/`RND`/`TIMER`, console
   `INPUT`/`LINE INPUT`, `SHELL`/`COMMAND$`/`ENVIRON$`, each measured first; the remaining plain functions as the
   corpus, upstream or a user names them. (Replaces "the remaining 249 plain built-ins, table-driven, with
   generated tests".)
2. §5: **no type table**: `Ty` stays a `Copy` enum and gains the fixed-width variants, `Bit(width, sign)`,
   `FixedStr(len)` and `Mem`, with an explicit rank in place of the derived order. (Rewords step 8.)
3. §6: the `m2-language-server` proposal follows points 1–6: a `qb64rust lsp` subcommand with an `lsp` crate on
   `lsp-server`/`lsp-types` (first non-dev dependency, `cargo-deny` with it); parser only; the encoding passed by
   the extension and columns mapped by the server; a loader that serves open documents first; two labelled
   diagnostic sources; protocol tests in Rust and the integration suite with the built binary.
4. §4: one sentence in `STATUS.md` that upstream's number waits for steps 8, 10 and 11 while the corpus is the
   signal until then. (Documentation only.)

Steps 7, 8 (apart from the wording), 10 and 11 and their order are unchanged.
