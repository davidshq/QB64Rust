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
| `codegen-cpp` | `qb64rust-codegen-cpp` | IR to the fragments `qbx.cpp` includes (`global.txt`, `main0.txt`, ...): `lib.rs` the entry points and the fragment set, one module per concern adding the emitter's methods (`names.rs`: C types, identifiers, labels, literals, `#line`; `coverage.rs`: the counters of `--coverage`; `debug.rs`: the debug symbol file of `--debug`; `decl.rs`: sizes, declarations, allocation, temporaries; `procs.rs`: procedures, bodies, labels, `retK.txt`; `stmt.rs`: operations and stores; `place.rs`: places; `value.rs`: values and procedure arguments; `builtins.rs`: built-in calls; `owner.rs`: values that own dynamic member arrays) |
| `format` | `qb64rust-format` | The layout of a parsed and checked program, `qb64rust -y` (OpenSpec change `m5b-formatter`; spec `compiler/format`): the line pass with the indent level, labels, `:` and comments (`walk.rs`), the statements by family (`stmt\`: control flow, declarations, I/O, metacommands, SUB calls and built-in statements by their templates), expressions (`expr.rs`, `fixoperationorder$`'s layout and its sign rules), the spelling of keywords (`words.rs`, written by `tools\format\layout_words.py`), built-ins and names as the symbol table's definitions spell them (`fmt.rs`), and the last step, a port of `apply_layout_indent$` (`apply.rs`); the layout settings are `base`'s (`layout.rs`) |
| `lsp` | `qb64rust-lsp` | The language server (`qb64rust lsp`, OpenSpec change `m2-language-server`): the message loop with debounce, a parse worker and cancellation (`server.rs`), one program's parse with included files from open documents first (`analysis.rs`), the encoding boundary (`encoding.rs`: tables generated from `iconv-lite` by `tools\encodings\gen_tables.js`, UTF-16 columns), `file:` URIs (`uri.rs`), the check (`qb64rust/check`: the whole front end, a function the driver passes to `serve`), formatting of a document or a range (the driver's formatter, passed beside the check as `Hooks`; settings from `initializationOptions.format` and `workspace/didChangeConfiguration`), and the features from the trees alone: diagnostics, document symbols (`symbols.rs`), folding ranges (`folding.rs`), go to definition for procedures and labels (`definition.rs`) |
| `dap` | `qb64rust-dap` | The debug adapter (`qb64rust dap`, OpenSpec change `m5-dap-proxy`): a proxy in front of the toolchain's `lldb-dap` that reads the debug symbol file and speaks BASIC; the framing (`wire.rs`), the loop with its `seq` maps and blocking internal requests (`proxy.rs`), the symbol file (`symbols.rs`), `launch` and stop on entry (`launch.rs`), its own breakpoints and the error stop (`breakpoints.rs`, texts in `errors.rs`), BASIC frames and the one thread (`stack.rs`), stepping by BASIC line (`step.rs`), variables read from memory (`vars.rs`, shown by `format.rs`), evaluate, set value, data breakpoints and set next line (`eval.rs`) |
| `driver` | `qb64rust-driver` | The `qb64rust` binary: command line, pipeline, the file loader for included files (`FileLoader`), build through the copy of libqb's `Makefile` (`libqb\`) with the C++ toolchain `toolchain.rs` finds, and `qb64rust toolchain install` that installs it; `qb64rust lsp` starts the language server, passing it `check_for_server` (the front end after the parse, `check_program`, the CLI's own); `qb64rust dap` the debug adapter, with the toolchain's `lldb-dap` |
| `difftest` | `qb64rust-difftest` | Not part of the compiler: the generator of the differential programs in `tests\differential` (binary `qb64rust-difftest`, `gen [--check]`; design D2 of `m2-numeric-types`, `tests\differential\README.md`): the numeric type list with each type's `AS` name, literal suffix and value slots (extremes, -1, 0, 1, two seeded random values from a fixed-seed xorshift), the operators and the exclusions; `for_sema` maps every `sema::Ty` exhaustively, so a new numeric type fails its build until the generator covers it |

```
driver -> codegen-cpp -> ir -> sema -> syntax -> base
   \                 \      \--> builtins <--/   /
    \--> lsp ------------------------------------/   (lsp -> syntax, base: never sema; the check's front end from driver)
     \--> dap -------------------------------------> base   (the symbol file, not the compiler's trees)
      \--> format -> sema, syntax, builtins, base   (the spelling of names is the symbol table's)
difftest -> sema (the type list only)
```

`format` comes after `sema` because the spelling of names is part of `-y`'s output (design D1 of `m5b-formatter`);
a crate of its own so that the layout tables do not grow the front end, and so that `lsp` keeps away from `sema`.

`syntax -> builtins` (data only) came with the template statements (design D6 of `m2-parser-breadth`). `lsp`
reads the parser's trees only, so the live parse cannot report anything `sema` decides (design D1 of
`m2-language-server`); a check runs the rest of the front end through the function the driver hands the server
(`qb64rust_lsp::Check`, design D2 of `extension-diagnostics`), so `lsp` still does not depend on `sema`.

Source is bytes throughout (`study\archive\15` §1): files are never converted to `str`; positions are byte offsets;
columns in diagnostics are byte columns. A real error whose wording by the old compiler is known from recorded
output is followed by `note: QB64pe: <its wording>` (`base\src\old_wording.rs`, the table written by
`tools\diagnostics\old_wording.py`; `m3-old-wording-notes`). A program built here writes the rows of
`QB64PE-DIVERGENCES.md` whose paths it takes to the file `QB64RUST_DIVERGENCES` names (`m3-divergence-trace`).

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
target\release\qb64rust.exe --dump typed prog.bas          # tokens | tree | typed | ir | cpp | debug-info
target\release\qb64rust.exe -x --coverage prog.bas         # count each statement run (QB64RUST_COVERAGE=<file>)
target\release\qb64rust.exe -x --debug prog.bas            # a build for a debugger, with prog.exe.debug.json
target\release\qb64rust.exe -y prog.bas -o out.bas         # lay the file out as qb64pe -y does (-f:AutoIndent=... etc.)
target\release\qb64rust.exe lsp                            # the language server, over stdin and stdout
target\release\qb64rust.exe dap                            # the debug adapter, over stdin and stdout
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
procedures and labels, and formats a document or a range as `-y` does, with the client's layout settings; positions are UTF-16 columns, the document's text is parsed as the bytes of its encoding
(CP437 unless the editor says otherwise). It exits with 0 after `shutdown` and `exit`, with 1 otherwise.

`qb64rust dap [--toolchain-root <dir>] [--qb64pe-root <dir>]` speaks the Debug Adapter Protocol (spec
`editor/debug-adapter`). It starts the toolchain's `bin\lldb-dap.exe` (the toolchain found as a build finds it) with
`LLDB_USE_LLDB_SERVER=1`,
so that `lldb` debugs through `lldb-server` (its own Windows plugin can lose a step's stop under load: `DECISIONS.md`
2026-10-25), and passes every message through but those it rewrites. `launch` takes a debug build's executable
(`program`; `args`, `cwd`, `env`, `noPrompt`, `stopOnEntry`, `console`) and refuses one without its
`<exe>.debug.json`. The program then shows as one thread, "Program", its stack as `SUB show`, `FUNCTION twice&` and
"Main module", stepping by BASIC line, its variables by BASIC name in "Locals" and "Main module", `evaluate` of a
name, element or member (`,h`, `,b`, `,o`), set value, data breakpoints of up to 8 bytes, set next line, and the
exception filter "BASIC runtime errors"; anything typed in the Debug Console that names no variable goes to `lldb`.
Without a toolchain it still answers, every request with the error. It exits with 0 when the client or the backend
ends.

Included files (`'$INCLUDE:'file'`) are looked up in the including file's folder, then relative to the compiler
root: the folder of `qb64rust.exe`, or `--include-root <dir>` (the old compiler's rule; never the working
directory). The tier-2 runner passes `--include-root tests\upstream\root`.

Building an executable uses this repo's copy of libqb (`libqb\`: QB64pe's `Makefile`, `qbx.cpp` and libqb at the
reference commit, `libqb\SOURCE.md`; written and checked by `tools\libqb\copy_libqb.py`) with a C++ toolchain. The
copy: `--libqb-root <dir>`, else the environment variable `QB64RUST_LIBQB_ROOT`, else `libqb\` beside the
executable (as the compiler download unpacks it, `MANUAL.md`), else `libqb\` of this repository.
The toolchain (`driver\src\toolchain.rs`, spec `compiler/toolchain`): `--toolchain-root <dir>`, else
`QB64RUST_TOOLCHAIN_ROOT`, else the `internal\c\c_compiler` of a QB64pe folder given by `--qb64pe-root` or
`QB64RUST_QB64PE_ROOT`, else the one `qb64rust toolchain install` put in `%LOCALAPPDATA%\qb64rust\toolchain`
(`QB64RUST_TOOLCHAIN_HOME` moves it), else the clone `..\QB64pe` next to this repository; `qb64rust toolchain path`
says which. With none the build fails naming `qb64rust toolchain install`, which downloads the pinned llvm-mingw
release (the clone's toolchain byte for byte) and is the only thing that downloads. The QB64pe folder (where
`DECLARE LIBRARY` names and relative words of the extra link flags are looked for): `--qb64pe-root <dir>`, else
`QB64RUST_QB64PE_ROOT` (empty: none), else the clone; without one, the include root. The fragments, the copy of `qbx.cpp`, `qbx.o` and the `.sym` file go into `<exe>.qb64rust\` next to the
executable (characters other than `A-Z a-z 0-9 . _ -` in that folder name become `_`, because `make` cannot take
them; the executable is linked there and moved into place; the folder is deleted after a successful build unless
`--keep-build`); libqb objects are built into the copy's git-ignored places if missing (the first build after a
checkout builds those its flags need, about 40 s for a console program). Nothing is written into the clone, and no
tracked file of the copy changes. `-z` keeps that folder: run over many files (to collect diagnostics), it leaves a
`<name>.exe.qb64rust\temp\` next to every program without errors, so give `-o` a scratch folder outside the repo and
the clone, or use `--dump tree`. The QB64pe Windows release works as the root too (CI uses it). `-f:OptimizeCppProgram=true` builds
with `-O2` as `qb64pe` does; `-f:ExtraLinkerFlags=<value>` adds the value to the end of the link command as `qb64pe`
does (the test runner gives `tools\screen\screen_state.cpp` this way; the build cache's key takes the value and the
contents of a file it names; a relative name is the QB64pe folder's file, as `qb64pe` links in its own folder);
`-f:GenerateLicenseFile=true` has the `Makefile` write the licences of the
parts the program links, moved beside the executable as `<exe>.license.txt` (`build.rs`, `license_path`; the build
cache keeps the file with its executable, and an entry without it is no hit for a build that asks for it);
`-f:StripDebugSymbols=false` leaves the executable unstripped (`STRIP_SYMBOLS=n`, as `qb64pe` passes it; the
`Makefile`'s `objcopy --strip-unneeded` otherwise removes the local symbols and the debug information, and keeps
the global ones); other `-f:` settings are "not supported yet".

`--debug` (`m5-debug-build-symbols`) builds a program for a debugger: its own objects with `-g` and never `-O2`
(`OptimizeCppProgram=true` is left out, with a line on the console unless `-q`), the executable unstripped, libqb's
objects as for any build. The C++ is the same as without it; the `#line` directives the emitter writes before every
C++ line become the line table, so the toolchain's `lldb` stops at a `.bas` line. Beside the executable it writes
the debug symbol file `<exe>.debug.json` (`codegen-cpp\src\debug.rs`, spec `compiler/debug-info`): the files,
procedures, variables (C name and how the value is reached from it), user types (members at their offsets) and the
fixed layouts (the array descriptor's slots, `qbs`), version 1; every build removes one an earlier build left, with
the earlier executable, and a build without `--debug` writes none. `--dump debug-info` prints it without building. The build cache keeps debug, unstripped and
plain builds apart.

`--coverage` (`codegen-cpp\src\coverage.rs`, `cws-whole-game`) builds a program that counts each source statement
it runs; at exit, when `QB64RUST_COVERAGE` names a file, it appends one record to it: each source file with its path
relative to the main file's folder and the FNV-1a 64 hash of its text, then each statement as file, line, column
and count (unrun ones included). Statements at one position share an entry; the jumps the lowering adds for a
block (`Stmt::own`) have none. Without the option the C++ is unchanged; with it the fragments differ, so the build
cache keeps the two builds apart. `--dump cpp --coverage` shows that C++. The records are read by
`tools\coverage\coverage.py`.

## Building on Linux

On Linux x64 (glibc) the compiler builds and runs as on Windows (OpenSpec change `m7-linux`, spec
`compiler/platforms`); what differs is the C++ toolchain, which is the system's (`DECISIONS.md` 2026-10-28):
`g++`, `make` and `objcopy` from `PATH` (or a folder with `bin/g++` and `bin/make` given by `--toolchain-root` or
`QB64RUST_TOOLCHAIN_ROOT`), the distribution's development packages for OpenGL, GLU, X11 and ALSA, and `lldb-dap`
for the debugger. Nothing is downloaded: `qb64rust toolchain install` checks them and prints the packages to
install for the distribution it detects. On Ubuntu 24.04:

```
sudo apt-get install build-essential libglu1-mesa-dev libasound2-dev libx11-dev libxrandr-dev lldb
cargo build --release
target/release/qb64rust -x prog.bas            # writes prog, no extension, as qb64pe does on Linux
```

Rust comes from `rustup`; `rust-toolchain.toml` pins the version, which `rustup` installs at the first `cargo` in
the folder.

**From this Windows machine, in WSL.** Development stays on Windows; the Linux runs happen in WSL's Ubuntu 24.04,
in a copy of the working tree, never in `/mnt/c` (a Linux build would overwrite libqb's Windows objects, which are
built inside `libqb\`, and `/mnt/c` is slow for twenty thousand files; design D7 of `m7-linux`).
`tools/linux/sync.sh [dest]`, run from WSL in the Windows tree, copies the files of `git ls-files -co
--exclude-standard` into `~/qb64rust` (uncommitted edits included, nothing staged) and deletes there the files that
are gone on Windows; what a Linux build writes there is ignored by git and kept. A sync takes about two minutes
(`rsync --checksum` reads every file through `/mnt/c`). Set-up, once:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none --profile minimal
sudo apt-get install ...                       # the packages above (by the user: sudo)
```

then each time:

```
cd /mnt/c/<the repository> && bash tools/linux/sync.sh
cd ~/qb64rust && cargo build --release
```

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
| The fixes switch: `qb64rust.toml`, `--fixes`, `--fix`, `--list-fixes` | `compiler/fixes` |
| `--debug` builds, the debug symbol file `<exe>.debug.json` | `compiler/debug-info` |
| `-y`, the layout settings, the format oracle | `compiler/format`, `testing/format-oracle` |
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
(`editor/build-run`, `editor/compiler-diagnostics`, `editor/debug-launch`, `editor/formatting`,
`editor/language-support`, `editor/new-project`) and the test
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
| 1 | `cargo test -p qb64rust-format`, `cargo test -p qb64rust-driver --test format_oracle` | The formatter: `apply_layout_indent$`'s steps (unit), each layout rule with `qb64pe -y`'s output for its input (`format\tests\rules.rs`); every recording of the format oracle (`tests\format\README.md`) formatted in process, equal to the recording, the same typed tree, and the same again when formatted twice (`format_oracle.rs`, about 1 s; `QB64RUST_FORMAT_DIFFS=<file>` writes every line that differs); it prints the statement forms no recording reaches |
| 2 | `python tools\legacy_tests\run_legacy_tests.py --suite format --qb64 target\release\qb64rust.exe`, `--suite format-corpus --qb64 target\release\qb64rust.exe --jobs 8` | QB64pe's own `format_tests` (24 variants, every setting) and the format oracle through the command line |
| 1 | `cargo test -p qb64rust-lsp` | The language server: encoding tables against `iconv-lite`'s bytes, positions, URIs (unit); symbols and folding snapshots and every go-to-definition form on `lsp\tests\fixtures\outline.bas` (`features.rs`); the protocol scenarios of spec `editor/language-server` through an in-process connection (`server.rs`, the check with a stand-in front end; with the driver's, `driver\tests\check.rs`: every `check-fail` program of `tests\frontend` published as the CLI reports it); the walks over every corpus and upstream program (`walk.rs`) |
| 1, by hand | `cargo test -p qb64rust-driver --test cli -- --ignored` | The command-line scenarios that build an executable; about 30 s with the copy's libqb objects built. Unattended (`driver\tests\support`): every child has a deadline, a compiler or toolchain run 900 s (room for a first build of the libqb objects), a built program 60 s, after which its process tree is killed and the test fails naming the phase; programs run with `QB64PE_NOPROMPT=y`, so no runtime-error dialog waits for a person; no lock of the clone is taken (they run nothing there), and every compiler they start is given the objects' lock (`tools\legacy_tests\README.md`, "Locks"). `--nocapture` shows a `cli-time <test> <phase> <seconds>` line per child. The `debug_*` tests build with `--debug` and drive the clone's `lldb.exe -b` (`support::lldb`, 120 s): a breakpoint by `.bas` line, `next`, `step` and `finish`, a `RESUME` retry, each storage read through the symbol file's names, a stop at libqb's `error`; `--nocapture` also shows each run's `lldb stops: [...]` (what they measured: `openspec\changes\archive\2026-10-25-m5-debug-build-symbols\design.md`, "Measured"); with `QB64RUST_LLDB_LOG=<file>` `lldb` writes its log of stepping, threads and the process there. The `dap_*` tests drive `qb64rust dap` and the real `lldb-dap` through a small client (`support::dap`, 60 s a wait), one per requirement of spec `editor/debug-adapter` that runs a program, `dap_run_in_terminal` the extension's default console (the client answers `runInTerminal` and types the program's input; `openspec\changes\archive\2026-10-25-m5-debug-launch\design.md`, "Measured") and `dap_args` the arguments as `lldb-dap` passes them; `lldb_dap_measured_*` drive the clone's `lldb-dap` alone, for the facts the proxy rests on (`openspec\changes\archive\2026-10-25-m5-dap-proxy\design.md`, "Measured") |
| 1 | `cargo test -p qb64rust-driver --test unit_tests`; by hand `-- --ignored` | `qb64rust test`, the unit tests of BASIC code (spec `compiler/unit-tests`; `driver\src\unit_tests\`): one test per scenario on the fixtures of `tests\unit_tests\` (its `README.md`). Listing and the usage errors always run; the scenarios that build a test file, and the check that `qb64pe.exe` builds the same entry point and library and writes the same result lines (`the_old_compiler_builds_the_same_protocol`, the clone's lock taken alone), are `#[ignore]`d. Each run gets a temp folder of its own, so nothing a failing fixture keeps is left behind. Unit tests of discovery, the entry point, the loader, the judge and the output: `cargo test -p qb64rust-driver --lib unit_tests` |
| 1 | `cargo test -p qb64rust-dap` | The debug adapter: framing, the symbol file read from every `debug_info_*` snapshot of the driver's tests, the value formats, the parser of `evaluate`, the error texts held to libqb's `error_handle.cpp` (unit and `symbols.rs`); the passthrough and `seq` maps (`proxy.rs`); and every rewrite against a scripted stand-in for `lldb-dap` on the fixture `tests\fixtures\kinds.bas` with the symbol file the compiler writes for it (`session.rs`: launch, its own breakpoints, the stack, stepping, the error stop; `values.rs`: variables, evaluate, set value, data breakpoints, set next line) |
| 1 | `python -m unittest discover -s tools/legacy_tests -p "test_*.py"` | The test runner's own rules (`tools\legacy_tests\README.md`, "Tests of the runner"): what "pass" means and what gets recorded, without a compiler; also in CI (`rust.yml`, job `test`) |
| 2 | `python tools\legacy_tests\run_legacy_tests.py --suite corpus --qb64 target\release\qb64rust.exe --list tests\corpus\slice.list` | The listed corpus programs end to end against the output recorded from `qb64pe.exe` |
| 2 | `python tools\legacy_tests\run_legacy_tests.py --suite compile --qb64 target\release\qb64rust.exe --list tests\upstream\pass.list` | The upstream programs of the pass list end to end (`tests\upstream\README.md`); also in CI (`rust.yml`, job `tier2`) |
| 2 | `python tools\legacy_tests\run_legacy_tests.py --suite corpus --corpus-root tests\differential --qb64 target\release\qb64rust.exe --list tests\differential\pass.list` | The differential programs of the pass list end to end against their recordings (`tests\differential\README.md`); also in CI |
| 2 | `python tools\callsite\callsite.py` | The call-site check (`tools\callsite\README.md`): for each program of `tests\callsite`, the libqb call the new compiler writes for one built-in statement or function equals the old compiler's (`-z`, nothing built or run). The old compiler's C++ is kept per program in `target\callsite-cache`: about 40 s for the 663 programs when it is filled, about 6 min without it. Needs `qb64pe.exe` in the clone; also in CI. A new plain built-in comes in with a program here |
| 2, at a merge into `main` | `python tools\diagnostics\old_wording.py`, then after a release build `--check` | The table of the old compiler's wording from recorded rejections, and every note on them equal to the recorded message (`tools\diagnostics\README.md`) |

**When to run what** (`CLAUDE.md` working rule 6, `DECISIONS.md` 2026-10-12 and 2026-10-17): tier 1 while
working; tier 2 once per change, at its close, and before that only the programs the work touches (`--glob`,
`--category`, a short `--list`). Add `--pass-cache target\pass-cache` to a corpus or compile run: a program that
passed with the same executable and expected files is not run again (`tools\legacy_tests\README.md`). At a
change's close the slice list and the upstream pass list run with it, and the run without folding only when the
change touched `sema`'s constants or a numeric conversion. At each merge into `main` the two full suites run
without it, and the slice list and the upstream pass list are judged from their results instead of being run
again, by the runner with `--suite corpus --check-list tests\corpus\slice.list` and `--suite compile --check-list
tests\upstream\pass.list` after them. Runs of the new compiler may run side by side (`tools\legacy_tests\README.md`,
"Locks"). In CI the `tier2` job runs only when started by hand, not on a push.

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

**Mutation test.** `QB64RUST_MUTATE_SEED` and `QB64RUST_MUTATE_COUNT` (mutants of each kind, byte-level and
line-level, per corpus program, default 10) change the run; a mutant the front end accepts also goes through lowering
and C++ generation; a failure prints the seed to rerun with and writes the mutant to `target\mutate-failure.bas`.

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
| `debug-info` | no errors; the debug symbol file of a `--debug` build is a snapshot, the files' paths relative to the test folder |

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
  language server, 2026-10-08); it also enforces working rule 3 of `CLAUDE.md` (no GPL code by way of a crate).
