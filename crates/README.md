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
| `codegen-cpp` | `qb64rust-codegen-cpp` | IR to the fragments `qbx.cpp` includes (`global.txt`, `main0.txt`, ...): `lib.rs` the entry points and the fragment set, one module per concern adding the emitter's methods (`names.rs`: C types, identifiers, labels, literals, `#line`; `coverage.rs`: the counters of `--coverage`; `decl.rs`: sizes, declarations, allocation, temporaries; `procs.rs`: procedures, bodies, labels, `retK.txt`; `stmt.rs`: operations and stores; `place.rs`: places; `value.rs`: values and procedure arguments; `builtins.rs`: built-in calls; `owner.rs`: values that own dynamic member arrays) |
| `lsp` | `qb64rust-lsp` | The language server (`qb64rust lsp`, OpenSpec change `m2-language-server`): the message loop with debounce, a parse worker and cancellation (`server.rs`), one program's parse with included files from open documents first (`analysis.rs`), the encoding boundary (`encoding.rs`: tables generated from `iconv-lite` by `tools\encodings\gen_tables.js`, UTF-16 columns), `file:` URIs (`uri.rs`), the check (`qb64rust/check`: the whole front end, a function the driver passes to `serve`), and the features from the trees alone: diagnostics, document symbols (`symbols.rs`), folding ranges (`folding.rs`), go to definition for procedures and labels (`definition.rs`) |
| `driver` | `qb64rust-driver` | The `qb64rust` binary: command line, pipeline, the file loader for included files (`FileLoader`), build through the copy of libqb's `Makefile` (`libqb\`) with the clone's toolchain; `qb64rust lsp` starts the language server, passing it `check_for_server` (the front end after the parse, `check_program`, the CLI's own) |
| `difftest` | `qb64rust-difftest` | Not part of the compiler: the generator of the differential programs in `tests\differential` (binary `qb64rust-difftest`, `gen [--check]`; design D2 of `m2-numeric-types`, `tests\differential\README.md`): the numeric type list with each type's `AS` name, literal suffix and value slots (extremes, -1, 0, 1, two seeded random values from a fixed-seed xorshift), the operators and the exclusions; `for_sema` maps every `sema::Ty` exhaustively, so a new numeric type fails its build until the generator covers it |

```
driver -> codegen-cpp -> ir -> sema -> syntax -> base
   \                 \      \--> builtins <--/   /
    \--> lsp ------------------------------------/   (lsp -> syntax, base: never sema; the check's front end from driver)
difftest -> sema (the type list only)
```

`syntax -> builtins` (data only) came with the template statements (design D6 of `m2-parser-breadth`). `lsp`
reads the parser's trees only, so the live parse cannot report anything `sema` decides (design D1 of
`m2-language-server`); a check runs the rest of the front end through the function the driver hands the server
(`qb64rust_lsp::Check`, design D2 of `extension-diagnostics`), so `lsp` still does not depend on `sema`.

Source is bytes throughout (`study\archive\15` §1): files are never converted to `str`; positions are byte offsets;
columns in diagnostics are byte columns. A real error whose wording by the old compiler is known from recorded
output is followed by `note: QB64pe: <its wording>` (`base\src\old_wording.rs`, the table written by
`tools\diagnostics\old_wording.py`; `m3-old-wording-notes`). A program built here writes the rows of
`DIVERGENCES.md` whose paths it takes to the file `QB64RUST_DIVERGENCES` names (`m3-divergence-trace`).

## Lints

`cargo clippy --workspace --all-targets -- -D warnings` must be clean (CI runs it). Beyond the defaults
(`study\archive\21`):

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
target\release\qb64rust.exe -x --coverage prog.bas         # count each statement run (QB64RUST_COVERAGE=<file>)
target\release\qb64rust.exe lsp                            # the language server, over stdin and stdout
target\release\qb64rust.exe --report unsupported tests\corpus tests\programs   # what these programs still need
```

`--report unsupported <file or folder>... [--include-root <dir>]` (`driver\src\report.rs`) runs the front end over
every program given (of a folder every `.bas` file below it) and prints one line per "not supported yet" message:
the programs it occurs in, the programs it alone holds back (its only message, and no other error), and its uses,
the most wanted first. The 100-error cap does not apply; a program with an `.err` file beside it is left out (the
old compiler rejects it); a program with an error that is not marked is named after the table. For choosing what
comes in next, by hand; no test depends on its numbers. For the upstream copy add `--include-root
tests\upstream\root`; the ten programs of its `qb64pe\` folder include the old compiler's sources and show as
"other errors" unless the clone's `tests\compile_tests` is given instead.

`qb64rust lsp` (only that word, no other argument) speaks the Language Server Protocol to an editor; the VS Code
extension starts it when `qb64rust.path` names the binary (`vscode\README.md`). It parses each open program after
every change (100 ms debounce) and gives syntax errors, the outline, folding ranges and go to definition for
procedures and labels; positions are UTF-16 columns, the document's text is parsed as the bytes of its encoding
(CP437 unless the editor says otherwise). It exits with 0 after `shutdown` and `exit`, with 1 otherwise.

Included files (`'$INCLUDE:'file'`) are looked up in the including file's folder, then relative to the compiler
root: the folder of `qb64rust.exe`, or `--include-root <dir>` (the old compiler's rule; never the working
directory). The tier-2 runner passes `--include-root tests\upstream\root`.

Building an executable uses this repo's copy of libqb (`libqb\`: QB64pe's `Makefile`, `qbx.cpp` and libqb at the
reference commit, `libqb\SOURCE.md`; written and checked by `tools\libqb\copy_libqb.py`) with the QB64pe reference
clone's toolchain. The copy: `--libqb-root <dir>`, else the environment variable `QB64RUST_LIBQB_ROOT`, else
`libqb\` of this repository. The clone (its toolchain, and the folder `DECLARE LIBRARY` names are looked for in):
`--qb64pe-root <dir>`, else the environment variable `QB64RUST_QB64PE_ROOT`, else `..\QB64pe` next to this
repository. The fragments, the copy of `qbx.cpp`, `qbx.o` and the `.sym` file go into `<exe>.qb64rust\` next to the
executable (characters other than `A-Z a-z 0-9 . _ -` in that folder name become `_`, because `make` cannot take
them; the executable is linked there and moved into place; the folder is deleted after a successful build unless
`--keep-build`); libqb objects are built into the copy's git-ignored places if missing (the first build after a
checkout builds those its flags need, about 40 s for a console program). Nothing is written into the clone, and no
tracked file of the copy changes. `-z` keeps that folder: run over many files (to collect diagnostics), it leaves a
`<name>.exe.qb64rust\temp\` next to every program without errors, so give `-o` a scratch folder outside the repo and
the clone, or use `--dump tree`. The QB64pe Windows release works as the root too (CI uses it). `-f:OptimizeCppProgram=true` builds
with `-O2` as `qb64pe` does; `-f:ExtraLinkerFlags=<value>` adds the value to the end of the link command as `qb64pe`
does (the test runner gives `tools\screen\screen_state.cpp` this way; the build cache's key takes the value and the
contents of a file it names; a relative name is the clone's file, as `qb64pe` links in its own folder);
`-f:GenerateLicenseFile=true` has the `Makefile` write the licences of the
parts the program links, moved beside the executable as `<exe>.license.txt` (`build.rs`, `license_path`; the build
cache keeps the file with its executable, and an entry without it is no hit for a build that asks for it);
`-f:StripDebugSymbols=...` is ignored; other `-f:` settings are "not supported yet".

`--coverage` (`codegen-cpp\src\coverage.rs`, `cws-whole-game`) builds a program that counts each source statement
it runs; at exit, when `QB64RUST_COVERAGE` names a file, it appends one record to it: each source file with its path
relative to the main file's folder and the FNV-1a 64 hash of its text, then each statement as file, line, column
and count (unrun ones included). Statements at one position share an entry; the jumps the lowering adds for a
block (`Stmt::own`) have none. Without the option the C++ is unchanged; with it the fragments differ, so the build
cache keeps the two builds apart. `--dump cpp --coverage` shows that C++. The records are read by
`tools\coverage\coverage.py`.

## What the compiler supports

What a program may use, and how it behaves, is in the merged specs under `openspec\specs\` (one home per
behaviour, `study\archive\31` item 2). How and where a change built it is in that change's `design.md` and `tasks.md`
(`openspec\changes\archive\`); what it left "not supported yet" on purpose is in `SOMEDAY.md` ("Left marked by
…"); what the corpus and upstream programs still need is the demand report (`--report unsupported`, above). The
catalogue this section held until 2026-10-16, with the code each feature lives in, is
[`study\archive\support-record.md`](../study/archive/support-record.md) (closed: a new change adds nothing to it or here).

| Area | Spec (`openspec\specs\…\spec.md`) |
|---|---|
| Stages, preprocessor (`$IF`, `$LET`), included files, the build, the build cache | `compiler/pipeline` |
| Command line, `-f:` settings, warnings, the demand report | `compiler/cli` |
| Console and window, `$CONSOLE`, `$SCREENHIDE`, `$EXEICON`, window built-ins | `language/console-and-window` |
| Procedures, parameters, `SHARED`, `STATIC`, `COMMON` | `language/procedures` |
| `ON ERROR`, `RESUME`, `ERROR`, `ERR`, `ERL` | `language/error-handling` |
| Control flow, `SELECT CASE`, `ON … GOTO`/`GOSUB` | `language/control-flow` |
| `CONST`, `OPTION _EXPLICIT`, the auto-included constants | `language/constants` |
| Numeric types, operators, literals, conversions | `language/numeric-semantics` |
| `DEFINT` … `DEFSTR`, `_DEFINE` | `language/default-types` |
| Arrays, `TYPE`, member arrays, `_DYNAMIC` members, whole-array assignment | `language/arrays-and-types`, `language/unstable-features` |
| `STRING * n` | `language/fixed-length-strings` |
| Built-in functions and statements in general | `language/builtin-functions`, `language/builtin-statements` |
| Files by number; `GET`/`PUT`, `FIELD`; `PRINT USING`; `DATA`/`READ` | `language/file-io`, `language/record-files`, `language/print-using`, `language/data-read` |
| Screen, images, fonts, mouse | `language/screen-output`, `language/images`, `language/fonts`, `language/mouse` |
| `SOUND`, `PLAY`, `BEEP`, the sound handles | `language/sound` |
| `_MEM`; `DEF SEG`, `VARPTR`, `PEEK`/`POKE` | `language/mem-blocks`, `language/conventional-memory` |
| `$ASSERTS`, `$DEBUG`; checksums, compression, base64 | `language/asserts-and-debug`, `language/data-processing` |
| `DECLARE LIBRARY` | `language/external-libraries` |
| `ON TIMER`, `ON KEY`, the event switches, `KEY ON`/`OFF`/`LIST` | `language/events` |
| The network functions, `_SOCKETS_`, `_ENCODEURL$`/`_DECODEURL$` | `language/network` |
| `$EMBED`, `_EMBEDDED$` | `language/embedded-files` |
| `SUB _GL`, the OpenGL constants and routines | `language/opengl` |
| The language server | `editor/language-server` |

Not listed, as they are not about what the compiler accepts: the VS Code extension's other specs
(`editor/build-run`, `editor/compiler-diagnostics`, `editor/formatting`, `editor/language-support`) and the test
sets' (`testing/…`).

Anything else gets a "not supported yet" error, never wrong code. **Every form the old compiler accepts parses**
(`m2-parser-breadth`: `tests\known_parse_gaps.list` is empty): parsed into typed nodes but still marked by `sema`
are line numbers and jumps to them (also as `ON … GOTO` and `RESTORE` targets), the other built-in functions, event handlers and switches, `STOP`, `RUN`,
`READ`, `INPUT`, `INPUT #` or `SWAP` with a whole array, every other
built-in statement read by its template (`TIME$ =`, ...), the declaration
forms (arrays named `name$n`,
`_MEMPUT`/`_MEMFILL … AS type`), `CHAIN`, type names as arguments, the
constants of QB64pe's auto-included file that are not computed yet and the names of its other auto-included
files; of arrays and `TYPE`:
whole arrays (`x()`) where neither an array parameter nor a whole-array assignment takes them (`PRINT a()`),
whole-array assignment of names with a type suffix (`a%() = b%()`), member arrays not marked `_DYNAMIC` with a bound that
names a `CONST`;
`DEF FN` is an error, as in QB64pe. After a declaration
marked "not supported yet", later real errors are dropped (the follow-on rule, design D10 of `m2-parser-breadth`):
the program is rejected by the mark, and no error is reported that the declaration might have prevented.

A panic is reported as `qb64rust: internal compiler error: <message> at <source location>`, with the input file;
the executable is removed and the exit code is 3 (every other failure exits with 1). `QB64RUST_TEST_PANIC=1`
panics on purpose, for the CLI test only.

## Tests

| Tier (`study\archive\19`) | Command | What |
|---|---|---|
| 1 | `cargo test` | Unit tests; lexer and parser snapshots; symbol-table snapshots; `tests\frontend\` by mode line (included files under `tests\frontend\inc\`); every input set through the front end with its included files (`inputs.rs`: corpus, `tests\upstream`, snippets, `tests\differential`, the real programs of `tests\programs`, and from the clone `qbasic_testcases` and the old compiler's sources, `qb64pe.bas` through all its includes; no panic, exact round trip of every tree, the copy equals the clone, the three lists of `tests\upstream\README.md`); the programs of `tests\corpus\slice.list` without diagnostics; every program with an `.err` file rejected; reserved names of variables and procedures against the measured lists (`names.rs`); the seeded mutation test (`mutate.rs`); the command line; the differential programs equal to what the generator writes, each with its recording (`difftest\tests\fresh.rs`) |
| 1 | `cargo test -p qb64rust-lsp` | The language server: encoding tables against `iconv-lite`'s bytes, positions, URIs (unit); symbols and folding snapshots and every go-to-definition form on `lsp\tests\fixtures\outline.bas` (`features.rs`); the protocol scenarios of spec `editor/language-server` through an in-process connection (`server.rs`, the check with a stand-in front end; with the driver's, `driver\tests\check.rs`: every `check-fail` program of `tests\frontend` published as the CLI reports it); the walks over every corpus and upstream program (`walk.rs`) |
| 1, by hand | `cargo test -p qb64rust-driver --test cli -- --ignored` | The command-line scenarios that build an executable; about 30 s with the copy's libqb objects built. Unattended (`driver\tests\support`): every child has a deadline, a compiler or toolchain run 900 s (room for a first build of the libqb objects), a built program 60 s, after which its process tree is killed and the test fails naming the phase; programs run with `QB64PE_NOPROMPT=y`, so no runtime-error dialog waits for a person; no lock of the clone is taken (they run nothing there), and every compiler they start is given the objects' lock (`tools\legacy_tests\README.md`, "Locks"). `--nocapture` shows a `cli-time <test> <phase> <seconds>` line per child |
| 1 | `python -m unittest discover -s tools/legacy_tests -p "test_*.py"` | The test runner's own rules (`tools\legacy_tests\README.md`, "Tests of the runner"): what "pass" means and what gets recorded, without a compiler; also in CI (`rust.yml`, job `test`) |
| 2 | `python tools\legacy_tests\run_legacy_tests.py --suite corpus --qb64 target\release\qb64rust.exe --list tests\corpus\slice.list` | The listed corpus programs end to end against the output recorded from `qb64pe.exe` |
| 2 | `python tools\legacy_tests\run_legacy_tests.py --suite compile --qb64 target\release\qb64rust.exe --list tests\upstream\pass.list` | The upstream programs of the pass list end to end (`tests\upstream\README.md`); also in CI (`rust.yml`, job `tier2`) |
| 2 | `python tools\legacy_tests\run_legacy_tests.py --suite corpus --corpus-root tests\differential --qb64 target\release\qb64rust.exe --list tests\differential\pass.list` | The differential programs of the pass list end to end against their recordings (`tests\differential\README.md`); also in CI |
| 2 | `python tools\callsite\callsite.py` | The call-site check (`tools\callsite\README.md`): for each program of `tests\callsite`, the libqb call the new compiler writes for one built-in statement or function equals the old compiler's (`-z`, nothing built or run). The old compiler's C++ is kept per program in `target\callsite-cache`: about 40 s for the 663 programs when it is filled, about 6 min without it. Needs `qb64pe.exe` in the clone; also in CI. A new plain built-in comes in with a program here |
| 2, when messages changed | `python tools\diagnostics\old_wording.py`, then after a release build `--check` | The table of the old compiler's wording from recorded rejections, and every note on them equal to the recorded message (`tools\diagnostics\README.md`) |

**When to run what** (`CLAUDE.md` working rule 9, `DECISIONS.md` 2026-10-12 and 2026-10-17): tier 1 while
working; tier 2 once per change, at its close, and before that only the programs the work touches (`--glob`,
`--category`, a short `--list`). Add `--pass-cache target\pass-cache` to a corpus or compile run: a program that
passed with the same executable and expected files is not run again (`tools\legacy_tests\README.md`). At a
change's close the slice list and the upstream pass list run with it, and the run without folding only when the
change touched `sema`'s constants or a numeric conversion. At each merge into `main` the two full suites run
without it, and the slice list and the upstream pass list are judged from their results instead of being run
again, by the runner with `--suite corpus --check-list tests\corpus\slice.list` and `--suite compile --check-list
tests\upstream\pass.list` after them. Runs of the new compiler may run side by side (`tools\legacy_tests\README.md`,
"Locks"). In CI the `tier2` job runs when started by hand and once a week, not on a push.

Tier 2 locally: add `--jobs 8 --build-cache target\build-cache` to each command. `--jobs` runs programs in
parallel; `--build-cache` (environment variable `QB64RUST_BUILD_CACHE`, `driver\src\build.rs`) skips `make` for a
program whose C++ build has the same inputs as an earlier one's (the fragments, the copy's `qbx.cpp`, `Makefile`
and every file of its `internal\c` that no build writes, by size and time, the clone's toolchain, the options), so
after an edit only the programs whose C++ changed are rebuilt, and a compile with `qb64pe.exe` in the clone throws
nothing away; every program is still compiled by `qb64rust`, run and compared. CI uses neither.
A run of the runner or of `callsite.py` compiles with a copy of `qb64rust.exe` made at its start
(`target\test-bin\<hash>`), so the compiler may be built again, and tier 1 run, while a tier 2 run goes on
(`tools\legacy_tests\README.md`, "The compiler a run tests").
The slice list holds screen programs (`screen/<name>`, checked by the state of the screen they leave,
`tools\screen\README.md`): each shows its window on the desktop for about 0.3 s, several at a time with `--jobs`,
and a key typed just then can reach one (`SOMEDAY.md`: a desktop of their own).
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
| `coverage` | no errors; the C++ fragments of a `--coverage` build are a snapshot |

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

The compiler itself has one: `sema` has `astro-float-num` (pure Rust, MIT; with `itertools` and `lazy_static`),
the high-precision reference behind the `CONST` evaluator's functions and its exact decimal conversions
(`consteval.rs`, `f80.rs`, `certify.rs`; `DECISIONS.md` 2026-10-17). The language server (`lsp`) has `lsp-server` (the transport and message loop,
synchronous) and `lsp-types` (the protocol types), with `serde_json` for their messages; they bring `serde`,
`crossbeam-channel`, `fluent-uri` and a few more. `serde_json` also reads the built-in table at build time, and
`insta` and `tempfile` serve the tests. `cargo deny check` (`deny.toml`; CI job `deny` in `rust.yml`; install with
`cargo install cargo-deny --locked`) keeps licences (MIT, Apache-2.0, Unicode-3.0), advisories and sources in view.
The policy (decided 2026-10-07):

- Take a crate for a protocol or for test infrastructure when the need arrives: `lsp-types` and `lsp-server` for
  the language server, their DAP equivalents for the debugger, `insta` for snapshots, `tempfile` for scratch
  folders that are deleted when a test fails.
- No crate that takes source as `str` (`rowan`, `logos`, `ariadne`, `codespan-reporting`): source is bytes
  (`study\archive\15` §1). No crate that replaces a pipeline stage or the analysis model (`salsa`; `DECISIONS.md`,
  2026-10-04).
- Performance crates (`memchr`, `rustc-hash`, `indexmap`, `rayon`) only after a measurement shows the need.
- A numeric crate where exact or high-precision arithmetic cannot reasonably be written here, guarded so that a
  fault in it cannot become a wrong value in a program (`astro-float-num`, 2026-10-17: its results are checked at
  two precisions and against fixed values, and anything it cannot settle is "not supported yet").
- Not needed at this size: `clap` (the flags copy `qb64pe`'s `-x`, `-z`, `-f:name=value`, about 80 lines by hand),
  `thiserror`/`anyhow` (driver errors are messages), per-file test runners (`libtest-mimic`: the harnesses already
  name each failing file and share one run over all inputs).
- `cargo-deny` comes with the first dependency that is not build- or dev-only (`study\archive\21` item 7; added with the
  language server, 2026-10-08); it also enforces rule 5 of `CLAUDE.md` (no GPL code by way of a crate).
