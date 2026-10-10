# The QB64Rust compiler crates

The new compiler, `qb64rust`. One crate per pipeline stage, so the layering is enforced by the dependency graph
(design of the OpenSpec change `m2-workspace-and-slice`, D1). Dependencies point only downward.

| Crate | Package | What it holds |
|---|---|---|
| `base` | `qb64rust-base` | `FileId`, byte `Span`, `SourceMap` with the line index (CR LF, LF, lone CR), `Diagnostic`, the 100-error cap |
| `syntax` | `qb64rust-syntax` | Byte lexer, lossless tree (green nodes + cursor; printing it gives back the file byte for byte), parser with one module per statement family (`parser\keywords.rs`: the reserved words; `parser\template.rs`: built-in statements read by their `specialformat` template; `parser\meta.rs`: metacommands, the preprocessor and included files), typed accessors over the tree (`ast.rs`: one wrapper per node kind, every child an `Option` or an iterator), the old compiler's rule for metacommands in comments (`meta.rs`), the preprocessor (`pp.rs`: `$IF`/`$LET` evaluated as the old compiler does); the program's trees (`program.rs`: one `Tree` per file per inclusion, `ParsedProgram`, the `Loader` for included files; a node's `key()` is its tree and offset) |
| `builtins` | `qb64rust-builtins` | The built-in table, generated at build time from `tools\builtins\builtins.json`; the grammar of the `specialformat` templates (`template.rs`, checked for every template by `build.rs`); the names of QB64pe's auto-included files |
| `sema` | `qb64rust-sema` | Procedure table, scopes (main, procedure, `STATIC`, `SHARED`), variables (a name plus a type), arrays, user types, places (variable, element, member), labels, constants (`CONST`, `consteval.rs`), literal typing, computation types, explicit conversions, by-reference or by-value arguments, integer constant folding; the typed tree; the symbol table (`symbols.rs`: definition and references of every variable, procedure, label and constant, `Symbols::at` for a position, `dump_symbols`) |
| `ir` | `qb64rust-ir` | The ABI-neutral IR (no libqb names, no C types: procedures, storage classes, places and their store rules, handlers and `RESUME` as statement-level rules; values, places and arguments are `sema`'s typed tree) and its lowering from the typed tree; `validate` checks its structure |
| `codegen-cpp` | `qb64rust-codegen-cpp` | IR to the fragments `qbx.cpp` includes (`global.txt`, `main0.txt`, ...): `lib.rs` the entry points and the fragment set, one module per concern adding the emitter's methods (`names.rs`: C types, identifiers, labels, literals, `#line`; `decl.rs`: sizes, declarations, allocation, temporaries; `procs.rs`: procedures, bodies, labels, `retK.txt`; `stmt.rs`: operations and stores; `place.rs`: places; `value.rs`: values and procedure arguments; `builtins.rs`: built-in calls) |
| `lsp` | `qb64rust-lsp` | The language server (`qb64rust lsp`, OpenSpec change `m2-language-server`): the message loop with debounce, a parse worker and cancellation (`server.rs`), one program's parse with included files from open documents first (`analysis.rs`), the encoding boundary (`encoding.rs`: tables generated from `iconv-lite` by `tools\encodings\gen_tables.js`, UTF-16 columns), `file:` URIs (`uri.rs`), and the features from the trees alone: diagnostics, document symbols (`symbols.rs`), folding ranges (`folding.rs`), go to definition for procedures and labels (`definition.rs`) |
| `driver` | `qb64rust-driver` | The `qb64rust` binary: command line, pipeline, the file loader for included files (`FileLoader`), build through the reference clone's `Makefile`; `qb64rust lsp` starts the language server |
| `difftest` | `qb64rust-difftest` | Not part of the compiler: the generator of the differential programs in `tests\differential` (binary `qb64rust-difftest`, `gen [--check]`; design D2 of `m2-numeric-types`, `tests\differential\README.md`): the numeric type list with each type's `AS` name, literal suffix and value slots (extremes, -1, 0, 1, two seeded random values from a fixed-seed xorshift), the operators and the exclusions; `for_sema` maps every `sema::Ty` exhaustively, so a new numeric type fails its build until the generator covers it |

```
driver -> codegen-cpp -> ir -> sema -> syntax -> base
   \                 \      \--> builtins <--/   /
    \--> lsp ------------------------------------/   (lsp -> syntax, base: never sema)
difftest -> sema (the type list only)
```

`syntax -> builtins` (data only) came with the template statements (design D6 of `m2-parser-breadth`). `lsp`
reads the parser's trees only, so the server cannot report anything `sema` decides (design D1 of
`m2-language-server`).

Source is bytes throughout (`study\15` §1): files are never converted to `str`; positions are byte offsets;
columns in diagnostics are byte columns.

## Lints

`cargo clippy --workspace --all-targets -- -D warnings` must be clean (CI runs it). Beyond the defaults
(`study\21`):

- `sema`, `ir`, `codegen-cpp`: no `_ =>` arm on an enum (`wildcard_enum_match_arm`). List the variants; where a
  default is right, say why in `#[expect(clippy::wildcard_enum_match_arm, reason = "...")]`. The lint sees only
  `match`: write a type predicate or a per-type choice as a `match`, not as `==` or `if … else`.
- `clippy.toml` disallows converting bytes to `str` (`from_utf8`, `from_utf8_lossy`) and reading text
  (`read_to_string`, `read_line`, `BufRead::lines`). Show source bytes with `show_bytes`.
- Casts that can lose or reinterpret bits are linted. Lengths, offsets and ids use `base::to_u32` (source files are
  at most `MAX_SOURCE_LEN`); a deliberate wrap keeps its `as` with `#[expect]` and the reason.
- Release builds keep overflow checks: arithmetic BASIC defines as wrapping is written `wrapping_*`.

## Building and running

```
cargo build --release
target\release\qb64rust.exe -x prog.bas -o prog.exe        # compile (needs ..\QB64pe, see below)
target\release\qb64rust.exe -z prog.bas                    # write the C++ fragments only, print their folder (kept)
target\release\qb64rust.exe --dump typed prog.bas          # tokens | tree | typed | ir | cpp
target\release\qb64rust.exe lsp                            # the language server, over stdin and stdout
```

`qb64rust lsp` (only that word, no other argument) speaks the Language Server Protocol to an editor; the VS Code
extension starts it when `qb64rust.path` names the binary (`vscode\README.md`). It parses each open program after
every change (100 ms debounce) and gives syntax errors, the outline, folding ranges and go to definition for
procedures and labels; positions are UTF-16 columns, the document's text is parsed as the bytes of its encoding
(CP437 unless the editor says otherwise). It exits with 0 after `shutdown` and `exit`, with 1 otherwise.

Included files (`'$INCLUDE:'file'`) are looked up in the including file's folder, then relative to the compiler
root: the folder of `qb64rust.exe`, or `--include-root <dir>` (the old compiler's rule; never the working
directory). The tier-2 runner passes `--include-root tests\upstream\root`.

Building an executable uses the QB64pe reference clone for `qbx.cpp`, libqb and the toolchain: `--qb64pe-root
<dir>`, else the environment variable `QB64RUST_QB64PE_ROOT`, else `..\QB64pe` next to this repository. The
fragments, the copy of `qbx.cpp`, `qbx.o` and the `.sym` file go into `<exe>.qb64rust\` next to the executable
(characters other than `A-Z a-z 0-9 . _ -` in that folder name become `_`, because `make` cannot take them; the
executable is linked there and moved into place; the folder is deleted after a successful build unless
`--keep-build`); libqb objects are built into the clone's git-ignored folders if missing. No tracked file of the
clone changes. `-z` keeps that folder: run over many files (to collect diagnostics), it leaves a
`<name>.exe.qb64rust\temp\` next to every program without errors, so give `-o` a scratch folder outside the repo and
the clone, or use `--dump tree`. The QB64pe Windows release works as the root too (CI uses it). `-f:OptimizeCppProgram=true` builds
with `-O2` as `qb64pe` does; `-f:StripDebugSymbols=...` is ignored; other `-f:` settings are "not supported yet".

What the compiler supports so far:

- the first slice (`m2-workspace-and-slice`): `$CONSOLE:ONLY`, comments, `:`, `DIM` of scalars (`INTEGER`,
  `LONG`, `_INTEGER64`, `SINGLE`, `DOUBLE`, `_FLOAT`, `STRING`), `[LET] v = e`, implicit variables with suffixes,
  `PRINT` with `;`, `,` and the auto-semicolon, `END`, numeric and string literals, unary `-`, `+ - * /`,
  parentheses, string `+`, and `INSTR`;
- procedures (`m2-procedures-and-errors`): `SUB` and `FUNCTION` with parameters, calls with and without `CALL`
  (by reference or by value as in QB64pe), function calls in expressions, `EXIT SUB`/`EXIT FUNCTION`, `DECLARE`
  (ignored, as QB64pe does), local and implicit variables, `STATIC`, `SHARED`, `DIM SHARED`, reserved names;
  `SYSTEM` without an exit code;
- error handling: labels in the main module, `ON ERROR GOTO label` and `ON ERROR GOTO 0` (also inside a
  procedure), `RESUME`, `RESUME NEXT`, `RESUME label`, `ERROR n`, `ERR`, `ERL`, `CHR$`;
- every operator (`m2-control-flow-slice`): comparisons (`=`, `<>`, `<`, `>`, `<=`, `>=`, numbers and strings),
  `NOT`, `AND`, `OR`, `XOR`, `EQV`, `IMP`, `_ANDALSO`, `_ORELSE`, `_NEGATE`, `\`, `MOD`, `^`, typed as QB64pe types
  them (`sema\src\check\ops.rs`); an `IMP` whose left operand is an `IMP` is "not supported yet" (QB64pe
  computes `a IMP b IMP c` as `a OR b OR c`);
- `CONST` (`m2-control-flow-slice` task 4.2): the old compiler's constant evaluator (`sema\src\consteval.rs`, the
  walk in `sema\src\check\constants.rs`), its typing (`_INTEGER64`, DOUBLE, or the suffix's type), main and
  procedure scopes as measured; uses become literal nodes. Floats are computed in `f64` and checked for being the
  value the old compiler's `_FLOAT` path gives; where that cannot be shown, and for the evaluator's functions,
  the constant is "not supported yet";
- `OPTION _EXPLICIT` and `_EXPLICITARRAY`, program-wide wherever they stand, as measured;
- control flow (`m2-control-flow-slice`): `IF` in both forms with `ELSEIF`/`ELSE`, `IF c GOTO label`, `FOR …
  NEXT` (with `STEP`, `NEXT j, i`, the old compiler's temporaries and their widths), `DO … LOOP` in its four
  forms, `WHILE … WEND`, `EXIT FOR`/`DO`/`WHILE`, `GOTO`, `GOSUB`, `RETURN` and `RETURN label`, labels in every
  body (main and procedures, also inside blocks). The IR is flat with explicit jumps (`ir\src\lower.rs`); a
  raising block header behaves as in QB64pe (the pending-error rule, `ir\src\lib.rs`);
- arrays and `TYPE` (`m2-arrays-and-types`, `sema\src\check\places.rs`): static arrays of the main module (`DIM`
  and `DIM SHARED`, one or more dimensions, `lower TO upper`, bounds that are constant expressions) of the numeric
  types, `STRING` and user types; element read and write (indexes rounded half to even, error 9 outside the
  bounds), elements passed by reference, `LBOUND`/`UBOUND`; `TYPE` blocks of the main module with numeric and
  nested members, used anywhere in the file; `TYPE` variables in every storage class; member read and write
  (`p.x`, `p.a.b`, `a(i).m`), members passed by reference, dotted plain names (`a.b` without a `TYPE` variable
  `a`). Each place has its own store rule (`ir\src\lib.rs`); a member store into an element with a bad index
  stores nothing (`DIVERGENCES.md` D-004). The IR shares `sema`'s value tree (`Expr`, `Place`, `Arg`);
- the preprocessor and included files (`m2-parser-breadth`): `$IF`/`$ELSEIF`/`$ELSE`/`$END IF`/`$LET`/`$ERROR`
  evaluated as the old compiler does (Windows 64-bit, version 4.7.0; a branch not taken is kept as
  `InactiveCode`), comment `$INCLUDE` with the old compiler's lookup, `$INCLUDEONCE`; the program is checked and
  compiled across its files, and a runtime error in an included file names it as QB64pe does;
- procedures named like built-ins as measured (`verification\v19_proc_names`): a SUB may take a built-in
  function's name, a FUNCTION a built-in statement's;
- 70 built-in functions (`m2-core-builtins`; the file functions, `RND`, `TIMER`, `SHELL`, `COMMAND$`, `ENVIRON$` and
  14 added by demand with `m2-builtin-statements`), checked by one table-driven checker: `sema\src\builtins.rs` lists
  them, each with its `Rule` (one per kind of special-casing in the old compiler's `evaluatefunc`, not per
  function), and gives each call a held type (the C++ type of the libqb call) and a believed type (the old
  compiler's); `sema\src\check\builtins.rs` checks arity, argument kinds and the conversion of each argument to
  its slot (LONG as a store, DOUBLE exactly, `_FLOAT` as it is, any-numeric cast to its believed type);
  `codegen-cpp\src\builtins.rs` writes each rule as the old compiler does. String: `LEN`, `LEFT$`, `RIGHT$`,
  `MID$`, `ASC` (one and two arguments), `CHR$`, `STR$`, `VAL` (also with a type), `STRING$`, `SPACE$`, `LTRIM$`,
  `RTRIM$`, `_TRIM$`, `UCASE$`, `LCASE$`, `HEX$`, `OCT$`, `_BIN$`, `_TOSTR$`, `INSTR`; math: `ABS`, `SGN`, `INT`,
  `FIX`, `SQR`, `SIN`, `COS`, `TAN`, `ATN`, `LOG`, `EXP`, `CINT`, `CLNG`, `CSNG`, `CDBL`, `_ROUND`, `_PI` (also
  bare), `_ATAN2`, `_HYPOT`; files: `EOF`, `LOF`, `LOC`, `SEEK` (the last three held `_INTEGER64`, as libqb
  returns them, and believed LONG), `FREEFILE` and `_CWD$` (bare only), `_FILEEXISTS`, `_DIREXISTS`; `RND` and
  `TIMER` (bare or with their argument; `TIMER` held DOUBLE and believed SINGLE); the `SHELL` function (an
  `_INTEGER64`, the command's exit code), `COMMAND$` (bare or with an index), `ENVIRON$` (a name or an index: two
  libqb entries, the rule `StrOrIndex`), `_STARTDIR$` (bare only); `_ACOS`, `_ASIN`, `_SINH`, `_COSH`, `_TANH`,
  `_CEIL` (`std::` functions, held as C++ overloading gives for the argument), `_COT`, `_CSC`, `_SEC`, `_D2R`,
  `_R2D` (held DOUBLE), all believed `_FLOAT`; `_STRCMP`, `_STRICMP`; and `LBOUND`, `UBOUND`, `ERR`,
  `ERL`. A function that needs an argument, named without one, is an error, as in QB64pe. Adding one of the
  remaining built-ins is a row plus tests where an existing rule fits, and a program in `tests\callsite` (the
  call-site check, below); tier 1 requires each listed one in a `slice.list` program and a `typed` test. Left
  marked on purpose: `CSRLIN` and `POS` (their value is the screen's), `_ERRORLINE`, `MKL$`/`CVL` and the other
  functions the old compiler writes itself, `_MIN`, `_MAX`, `_CLAMP`, `_IIF` and the shifts (typed by their
  arguments; no rule yet);
- `SELECT CASE` and `SELECT EVERYCASE` (lists, `TO`, `IS`, `CASE ELSE`; the selector copied once into a static
  hidden variable unless it is a plain variable, as QB64pe; `DIVERGENCES-QB45.md` Q-002) and `ON n GOTO`/`ON n
  GOSUB` to labels (`n` above 255 falls through, Q-001), lowered to the IR's branches and jumps
  (`ir\src\lower.rs`);
- the full numeric type set (`m2-numeric-types`): `sema::Ty` has 17 numeric variants (`_BYTE`, `INTEGER`, `LONG`,
  `_INTEGER64`, each also `_UNSIGNED`; `_OFFSET` and `_UNSIGNED _OFFSET` as their own variants; `_BIT * n` and
  `_UNSIGNED _BIT * n` for n from 1 to 64; `SINGLE`, `DOUBLE`, `_FLOAT`), an enum with an explicit rank and no
  derived order (design D3). Every `AS` spelling and suffix, in every place: scalars of every storage class, static
  arrays, `TYPE` members, parameters (by reference also across signedness, as measured), FUNCTION results,
  `CONST`, `FOR`, `SELECT CASE` and the special-cased built-ins. Values compute as the old compiler's C++ does,
  tracking each value's [held and believed type](../GLOSSARY.md#held-type--believed-type) (`sema\src\check\ops.rs`
  `held`, `sema\src\literal.rs`); `conversion` (`sema\src\lib.rs`) gives each store's rule, including the `_BIT`
  store mask and sign extension. Checked against `qb64pe.exe` by the differential programs
  (`tests\differential`, made by `difftest`);
- fixed-length strings (`m2-numeric-types` group 7, design D6): `STRING * n` (n a number or an integer constant,
  read in 32 bits) and `name$n` variables of every storage class, static arrays of them and `TYPE` members, as the
  old compiler builds them: a fixed `qbs` over n NUL bytes per variable, a temporary one over an element's or
  member's bytes, stores by `qbs_set` (which cuts and pads); their values are plain `STRING`s (`Ty::held_value`),
  and one passed to a `STRING` parameter goes by reference, also in parentheses;
- built-in statements (`m2-builtin-statements`): a statement that is one call of the runtime is a row in
  `sema\src\builtins.rs` (`STATEMENTS`, the twin of the function list) and `Op::Builtin` in the IR: the table entry
  of the form written and one slot per argument or choice of its template (a value, a place, the word chosen, or
  absent). `sema\src\check\builtins.rs` matches the statement against its forms as the parser did and converts each
  argument to its slot; `codegen-cpp\src\builtins.rs` writes the call by the old compiler's template rule
  (`builtins\src\passing.rs`, a port of `seperateargs`' pass rules: which parts are C arguments, `NULL` for an
  absent one, the bits of the `passed` mask). The arguments are evaluated in order and the call is made also
  after a raising one, as measured. So far: `KILL`, `MKDIR`, `RMDIR`, `CHDIR`, `NAME`, `ENVIRON` (also `CALL
  KILL(…)`), `OPEN` in both forms with every mode, access and lock word, `SEEK`, `RANDOMIZE` (with a seed, with
  `USING`, and without one: the runtime then asks for it), `SHELL` in its three forms (plain, `_HIDE` first,
  `_DONTWAIT` first; with or without a command), and three with a rule of their own: `CLOSE` (one call per file number), `SWAP` (two places of one
  type but for signedness, any two strings, one `TYPE`; never a `_BIT`) and the `MID$` statement. Tier 1 requires each listed statement in a `slice.list` program and an `ir` test;
- sequential file I/O and the program's data, as operations of their own in the IR because they have items or
  targets (`sema\src\check\io.rs`, `codegen-cpp\src\io.rs`): `PRINT #` (not `USING`, `TAB`, `SPC`), `WRITE` to a
  file and to the console, `INPUT #` and `LINE INPUT #` into variables, elements and members (a pending-error
  check after each item or target, as the old compiler); `DATA` (the items of the whole program in file order,
  procedures and included files where they stand), `READ` (no check between its targets, as measured) and
  `RESTORE` to the start or to a label of any body; console `INPUT` and `LINE INPUT` (the prompt forms, the `;`
  before the prompt, one `,` after the last target): the compiler hands the runtime the type and address of each
  target, and the runtime reads, converts and stores (it never asks again; `study\00` §5). A corpus program that
  reads the console has its answers in `<name>.stdin` and ends with `SYSTEM` (`tests\corpus\README.md`).

Anything else gets a "not supported yet" error, never wrong code. **Every form the old compiler accepts parses**
(`m2-parser-breadth`: `tests\known_parse_gaps.list` is empty): parsed into typed nodes but still marked by `sema`
are line numbers and jumps to them (also as `ON … GOTO` and `RESTORE` targets), `EXIT SELECT`/`EXIT
CASE`, the other built-in functions, `DECLARE LIBRARY`, `OPTION BASE`, event handlers and switches, `STOP`, `RUN`,
`END`/`SYSTEM` with an exit code, the other I/O statements
(`PRINT USING`, `LPRINT`, `FIELD`, `LSET`/`RSET`),
`READ`, `INPUT`, `INPUT #` or `SWAP` with a whole array or a member of an element, every other
built-in statement read by its template (`LINE`, `SCREEN`, `GET`/`PUT`, `TIME$ =`, ...), the declaration
forms (`REDIM`, `COMMON`, `ERASE`, `DEFxxx`, `_DEFINE`, the type before the names, arrays named `name$n`, array
parameters, `STATIC` after a header, `_MEMPUT`/`_MEMFILL … AS type`, `_ARRAYCOPY`), type names as arguments, the
names of QB64pe's auto-included files and its precompiler flags (`_CONSOLE_`, ...); of arrays and `TYPE`: dynamic
and implicit arrays, arrays in procedures, whole arrays (`x()`), `TYPE` parameters, `STRING` and array members,
member arrays, whole-`TYPE` assignment; `DEF FN` is an error, as in QB64pe. After a declaration
marked "not supported yet", later real errors are dropped (the follow-on rule, design D10 of `m2-parser-breadth`):
the program is rejected by the mark, and no error is reported that the declaration might have prevented.

A panic is reported as `qb64rust: internal compiler error: <message> at <source location>`, with the input file;
the executable is removed and the exit code is 3 (every other failure exits with 1). `QB64RUST_TEST_PANIC=1`
panics on purpose, for the CLI test only.

## Tests

| Tier (`study\19`) | Command | What |
|---|---|---|
| 1 | `cargo test` | Unit tests; lexer and parser snapshots; symbol-table snapshots; `tests\frontend\` by mode line (included files under `tests\frontend\inc\`); every input set through the front end with its included files (`inputs.rs`: corpus, `tests\upstream`, snippets, `tests\differential`, the real programs of `tests\programs`, and from the clone `qbasic_testcases` and the old compiler's sources, `qb64pe.bas` through all its includes; no panic, exact round trip of every tree, the copy equals the clone, the three lists of `tests\upstream\README.md`); the programs of `tests\corpus\slice.list` without diagnostics; every program with an `.err` file rejected; reserved names of variables and procedures against the measured lists (`names.rs`); the seeded mutation test (`mutate.rs`); the command line; the differential programs equal to what the generator writes, each with its recording (`difftest\tests\fresh.rs`) |
| 1 | `cargo test -p qb64rust-lsp` | The language server: encoding tables against `iconv-lite`'s bytes, positions, URIs (unit); symbols and folding snapshots and every go-to-definition form on `lsp\tests\fixtures\outline.bas` (`features.rs`); the protocol scenarios of spec `editor/language-server` through an in-process connection (`server.rs`); the walks over every corpus and upstream program (`walk.rs`) |
| 1, by hand | `cargo test -p qb64rust-driver --test cli -- --ignored` | The command-line scenarios that build an executable |
| 2 | `python tools\legacy_tests\run_legacy_tests.py --suite corpus --qb64 target\release\qb64rust.exe --list tests\corpus\slice.list` | The listed corpus programs end to end against the output recorded from `qb64pe.exe` |
| 2 | `python tools\legacy_tests\run_legacy_tests.py --suite compile --qb64 target\release\qb64rust.exe --list tests\upstream\pass.list` | The upstream programs of the pass list end to end (`tests\upstream\README.md`); also in CI (`rust.yml`, job `tier2`) |
| 2 | `python tools\legacy_tests\run_legacy_tests.py --suite corpus --corpus-root tests\differential --qb64 target\release\qb64rust.exe --list tests\differential\pass.list` | The differential programs of the pass list end to end against their recordings (`tests\differential\README.md`); also in CI |
| 2 | `python tools\callsite\callsite.py` | The call-site check (`tools\callsite\README.md`): for each program of `tests\callsite`, the libqb call the new compiler writes for one built-in statement or function equals the old compiler's (`-z`, nothing built or run; about a minute). Needs `qb64pe.exe` in the clone; also in CI. A new plain built-in comes in with a program here |

Tier 2 locally: add `--jobs 8 --build-cache target\build-cache` to each command. `--jobs` runs programs in
parallel; `--build-cache` (environment variable `QB64RUST_BUILD_CACHE`, `driver\src\build.rs`) skips `make` for a
program whose C++ build has the same inputs as an earlier one's (the fragments, the clone's `qbx.cpp`, `Makefile`
and every file of its `internal\c` by size and time, the options), so after an edit only the programs whose C++
changed are rebuilt; every program is still compiled by `qb64rust`, run and compared. CI uses neither.
The full corpus with the old compiler also takes `--jobs 8` (private copies of `qb64pe`,
`tools\legacy_tests\README.md`): about 4 minutes instead of 12.

**"Not supported yet."** A diagnostic either reports an error in the program or is marked "not supported yet"
(`Diagnostic::unsupported`, printed `error: not supported yet: <message>`; the summary says how many). `sema` and
the parser mark every construct they do not handle; the parser also marks a generic syntax error ("expected ...")
at a BASIC word or operator (`syntax_error`). The three lists in `tests\` hold the programs where this does not
yet match the old compiler's verdict (since `m2-parser-breadth`, `known_false_errors.list` and
`known_parse_gaps.list` are empty); after a change, regenerate them with `QB64RUST_UPDATE_LISTS=1 cargo test -p
qb64rust-driver --test inputs` and review the diff (entries may only go away).

**Mutation test.** `QB64RUST_MUTATE_SEED` and `QB64RUST_MUTATE_COUNT` (mutants per corpus program, default 20)
change the run; a failure prints the seed to rerun with and writes the mutant to `target\mutate-failure.bas`.

Time of tier 1, measured 2026-10-07 after `m2-parser-breadth` groups 7 and 8 (debug build already built, 16
threads, clone present): `cargo test` takes 9.5–9.9 s over three runs, `inputs.rs` 6.1 s of it, most of it
`qb64pe.bas` parsed through all its included files (after `m2-control-flow-slice`: 4.9–5.4 s, `inputs.rs` 2.4 s). The budget is a minute (design of `m2-upstream-tests`); past
it, the clone sets would run in release only.

`tests\frontend\*.bas` start with a mode line, `' TEST: <mode>`:

| Mode | Checks |
|---|---|
| `parse-ok` | no syntax errors; the tree prints back to the file |
| `check-ok` | no errors after `sema` |
| `check-fail` | at least one error; the diagnostics are a snapshot |
| `typed`, `ir`, `cpp` | no errors; the typed tree, the IR or the C++ fragments are a snapshot |

The mode line is a comment, so `qb64pe.exe` ignores it.

### Snapshots

Snapshot tests use [`insta`](https://insta.rs). The `.snap` files are committed next to the tests
(`crates\syntax\tests\snapshots\`, `crates\sema\tests\snapshots\`, `crates\driver\tests\snapshots\`, the last named `<file>.<mode>.snap`). A
changed output fails `cargo test` and writes a `.snap.new` file. Review with `cargo insta review` (install with
`cargo install cargo-insta`) or read the `.snap.new` file and rename it over the `.snap` file to accept it. To
write all new snapshots in one run: `INSTA_FORCE_PASS=1 INSTA_UPDATE=new cargo test`, then review every
`.snap.new`.

`QB64RUST_NO_FOLD=1` turns integer constant folding off; it is for checking that folding changes no result (run
tier 2 with it set), not for normal use.

## Dependencies

The compiler itself has none. The language server (`lsp`) has `lsp-server` (the transport and message loop,
synchronous) and `lsp-types` (the protocol types), with `serde_json` for their messages; they bring `serde`,
`crossbeam-channel`, `fluent-uri` and a few more. `serde_json` also reads the built-in table at build time, and
`insta` and `tempfile` serve the tests. `cargo deny check` (`deny.toml`; CI job `deny` in `rust.yml`; install with
`cargo install cargo-deny --locked`) keeps licences (MIT, Apache-2.0, Unicode-3.0), advisories and sources in view.
The policy (decided 2026-10-07):

- Take a crate for a protocol or for test infrastructure when the need arrives: `lsp-types` and `lsp-server` for
  the language server, their DAP equivalents for the debugger, `insta` for snapshots, `tempfile` for scratch
  folders that are deleted when a test fails.
- No crate that takes source as `str` (`rowan`, `logos`, `ariadne`, `codespan-reporting`): source is bytes
  (`study\15` §1). No crate that replaces a pipeline stage or the analysis model (`salsa`; `DECISIONS.md`,
  2026-10-04).
- Performance crates (`memchr`, `rustc-hash`, `indexmap`, `rayon`) only after a measurement shows the need.
- Not needed at this size: `clap` (the flags copy `qb64pe`'s `-x`, `-z`, `-f:name=value`, about 80 lines by hand),
  `thiserror`/`anyhow` (driver errors are messages), per-file test runners (`libtest-mimic`: the harnesses already
  name each failing file and share one run over all inputs).
- `cargo-deny` comes with the first dependency that is not build- or dev-only (`study\21` item 7; added with the
  language server, 2026-10-08); it also enforces rule 5 of `CLAUDE.md` (no GPL code by way of a crate).
