# Design

## Context

The parser already reads the block (`crates\syntax\src\parser\proc.rs`, `declare_library_block`: the header with
its names, `ProcHeader`s with `ALIAS` and `BYVAL`, the error for a statement inside); `sema` marks the whole block
"not supported yet" (`check\blocks.rs` 553) and the follow-on rule then marks every use of its names. Program
procedures are `Proc` entries (`sema\src\lib.rs` 766) entered in pass 1 (`check\proc.rs`, `declare_proc`),
called by `StmtKind::Call` and `ExprKind::CallProc` with one `Arg` per parameter (`Ref` a place, `Temp` a copy,
`Array`), and emitted by `codegen-cpp` (`value.rs`, `args`: a place's pointer or `&(passN=…)`).

The old compiler's behaviour, measured in `verification\v37_*` (76 programs) and read in `qb64pe.bas`, is in
`study\00` §5 ("`DECLARE LIBRARY`"); the proposal summarises it. The libraries of the measurements are built from
`verification\v37_lib\` by `build_libs.py` with the clone's toolchain.

The closest precedent for a file the program names is `$EXEICON` (design D4 of `m3-sound`): `sema` records the
name as written and reads no file; the driver checks and copies the file and puts its bytes into the build
cache's key.

## Goals / Non-Goals

**Goals:**
- The four forms on Windows with the old compiler's lookup order, messages, C++ and call conventions, so that the
  8 upstream programs pass and `verification\v37_*` gives the same output (apart from D-023 and D-024).
- No new runtime code: everything is generated C++ and the link command.

**Non-Goals:**
- `on_error/errtest` (`_NEWHANDLER`, `_LASTHANDLER`).
- Linux and macOS (`.so`, `.dylib`, `/usr/lib`, `/usr/include`, `dlopen`), the IDE's folder in the lookup.
- Checking a header's declarations against the block's (the C++ compiler does, as for the old compiler).

## Decisions

### D1. Library procedures are `Proc`s with an external part

A declaration becomes a `Proc` with no statements and a new field `external: Option<External>`:
`External { library: LibraryId, c_name: String, byval: Vec<bool> }` (`c_name` the `ALIAS` text, else the name
as written, in its letter case: `x31`). Its parameters are variables as a program procedure's are, so the call
checks (number of arguments, conversion, a place of the parameter's type by reference) are shared. A `BYVAL`
parameter takes a new `Arg::Value(Expr)`: the argument converted to the parameter's type, written as an
expression (no `passN`). `Program` gets `libraries: Vec<Library>`:
`Library { kind: Plain | Static | Dynamic | Customtype, names: Vec<LibraryName>, file: FileId, span }`, a
`LibraryName` the written text and its span, `file` the file the block stands in (the fallback folder of a
header). A block with no name, or `""`, has no names (`x27`, `v37_a`).

Rejected alternative: a separate table of external procedures. The call checks, the follow-on rule, the symbol
kinds, the LSP's hover and rename all key on `ProcId`; a second table would duplicate each.

**The block and its rejections** (`check\blocks.rs`, `check\proc.rs`): pass 1 enters each declaration where it
stands (in a SUB too, `v37_i`); the measured messages of `x03`–`x12`, `x15`, `x25`, `x26`, `x28`, `x29`, `x32`,
`x40`, `x44`, `x46`–`x48` come from the declaration, those of `x14`, `x33`, `x34`, `x41`, `x50`, `x54` from the
use (a library FUNCTION with no parameters takes no parentheses: `x14`, `x54`). `ALIAS` and `BYVAL` outside a
library keep their present handling (`x08` is accepted and ignored, `x09` is an error).

### D2. The driver finds the libraries

A new `driver\src\library.rs`, run after `sema` and before the C++: for each library and each name, the lookup
of `qb64pe.bas` 4518–5204 for Windows, with **the reference clone (`--qb64pe-root`) as the old compiler's
folder** and the folder of the main source file for `./`:
- drop a version after `.` or `:` (`v37_i`), turn `/` into `\`, split the folder from the name;
- not `DYNAMIC`: `<folder>name.lib`, `.a`, `.o`, then `name.lib`, `.a`, `.o` from the clone; then a header
  `<folder>name.h`, `.hpp`, `name.h`, `.hpp`, then from the folder of `Library::file` joined with the written
  folder;
- `DYNAMIC`: `<folder>name.dll`, System32 + `<folder>name.dll`, `name.dll` from the clone or the source's
  folder, System32 + `name.dll`; the C++ names the DLL as the old compiler does (the full path when `./` gave
  one, else the written name with `.dll`).
The result, `Resolved { libraries: Vec<ResolvedLibrary> }` with `header: Option<PathBuf>`, `files:
Vec<PathBuf>`, `dll: Option<String>` per library and a `Linkage` (C or C++) per declaration without a header, is
an input of `codegen-cpp::emit`. Errors are diagnostics at the block's line with the old compiler's text
("LIBRARY not found", "DYNAMIC LIBRARY not found"). The LSP runs the same lookup if it can reach the function
(the `lsp` crate's dependencies decide; else it reports nothing for it, noted in `crates\README.md`).

**Linkage by `nm`** (`qb64pe.bas` 13805–13933): for a declaration with no header, `nm --demangle -g` of the
clone's toolchain on each file of its library (then `-D`), a line holding ` name(` is C++, a line ending in
` name` C; none in any file of the block: "Could not find sub/function 'name' in '<file>'" (D-024: all files of
the block are searched, the old compiler searches the last), two lines in one file, or a match in two files of
the block: "Unable to resolve multiple instances …" (the old compiler's rule for one file, applied to the
block). Both errors stand at the block's line (the old compiler gives "LINE 0", `x13`, `x24`).
`nm` runs once per file per compile, also with `-z`.

### D3. The C++

A new `codegen-cpp\src\library.rs`, writing what `qb64pe.bas` writes (`study\00` §5):
- **`regsf.txt`**, at the block's place among the prototypes: `#include "<header path>"` **once per header for
  the program** (D-023, the old compiler once per block); a block with a header writes no prototype (followed;
  `x23`). Without a header, per declaration: `extern "C" ` (C linkage) or nothing (C++), then
  `<ret> <c_name>(<args>);` with `typ2ctyp`'s types (`int32`, `char*` for `STRING`, `void*` for a TYPE,
  `ptrszint` for both `_OFFSET`s, `long double`), a by-reference parameter a pointer. The old compiler's
  `externtypeN.txt` indirection is not needed: the linkage is known when the C++ is written.
- **`DYNAMIC`**: `HINSTANCE DLL_<c name of the file>=NULL;` per library and `typedef <ret> (CALLBACK*
  DLLCALL_<X>)(<args>); DLLCALL_<X> <X>=NULL;` per declaration in `regsf.txt`, `X` the procedure's C name in the
  old compiler's form (`FUNC_D_ADD`); in `maindata.txt` the `LoadLibrary` and `GetProcAddress` lines with
  `error(259)` and `error(260)`, for a block in a SUB too.
- **`CUSTOMTYPE`**: `typedef <ret> CUSTOMCALL_<X>(<args>); CUSTOMCALL_<X> *<X>=NULL;` and in `maindata.txt`
  `<X>=(CUSTOMCALL_<X>*)&<c_name>;`; without a header, `extern "C"{ extern void <c_name>(void); }` in
  `global.txt` for a C name.
- **The call**: `(  <ret>  )<callee>(<args>)` as an expression (the cast removed where the old compiler removes
  it), a SUB as a statement; `callee` the C name, or `X` for `DYNAMIC` and `CUSTOMTYPE`. Arguments: `Arg::Ref` a
  place's pointer as today, `Arg::Temp` `&(passN=…)` as today, a string `(char*)(<qbs>)->chr`, a TYPE the
  value's address, `Arg::Value` the converted expression **with no cast**, and `NULL` for an argument whose C++
  text is exactly `0` (`qb64pe.bas` 12307, 21954): `0`, `(0)`, `0&` and a `CONST` 0 are, `1 - 1` is not (the old
  compiler does not write it `0`, and its C++ fails for a `void*`, `x56`–`x59`), so the test is on the text the
  old compiler would write, not on a folded value: this compiler's folding must not make `1 - 1` a `NULL`, which
  the normal run (folding on) checks; the run with `QB64RUST_NO_FOLD=1` checks that `0`, `(0)`, `0&` and the
  `CONST` still are. *As built (task 2.3):* `sema` decides it from the argument as written and passes
  `Arg::Null`, since its folding makes `1 - 1` a literal 0 the emitter could not tell from `0`; the emitter writes
  `NULL` for it. The converted expression must have the C++ type the old compiler's has
  (an integer literal stays an integer literal for a `DOUBLE` parameter, a `DOUBLE` to an integer is `qbr(x)`),
  which decides overloads (`v37_e`, `x60`); the front-end `cpp` tests pin each form.
- **The result's type**: `(  <ret>  )` casts the C result to the declared type, so a FUNCTION without a suffix
  under `DEFLNG` truncates a C `double` (2.7 gives 2, not 3, `x63`), as the old compiler does (followed; listed in
  `SOMEDAY.md` "QB64pe behaviours to review").
- **The result**: a `STRING` FUNCTION `qbs_new_txt((char*)…)` (libqb gives "" for `NULL`, `x39`).

### D4. The build

The driver adds each `ResolvedLibrary::files` path to `CXXLIBS_EXTRA`, after `ExtraLinkerFlags`' value in the old
compiler's order (libraries first, `qb64pe.bas` 13741), and puts the bytes of every library file and header into
the build cache's key (as the icon's). A header is included by its absolute path, so the build folder may be
anywhere. Only the files the block names go into the key, not the files a header itself `#include`s (the C++
compiler would have to be asked for them); a change to such a file alone needs a build without the cache.

**Upstream binaries**: `declare_library_static/test` links `./lib-windows.a` and `declare_library_dynamic/test`
loads `./lib.dll`, which `tests\upstream\compile_tests` lacks (the copy takes text files only), and CI's `tier2`
job runs `pass.list` from that copy. The job's step "Upstream pass list" drops `--compile-tests
tests/upstream/compile_tests`, so it runs the list from the clone's `tests\compile_tests`, as a local run does
by default (the job checks the clone out at the reference commit, `rust.yml` "Get QB64pe"; the copy has been
passed since `m2-upstream-tests`, `e9a5702`). The copy stays the input of tier 1. Rejected:
copying the two binaries into the repo (980 bytes and 47 KB that stay, for files CI already has); building each
`lib.c` in the runner (writes into the copy, which is never edited, and the DLL would differ from the one the
old compiler was measured with); a local-only list (the two programs would never run in CI).

### D5. Tests

- `tests\frontend\`: `library_errors.bas` (`check-fail`: every rejected form that `sema` sees), `library_cpp.bas`
  (`cpp`: each form's prototypes, a `DYNAMIC` and a `CUSTOMTYPE` block, each kind of argument, a string result;
  the lookup is given a fixed `Resolved` so the test needs no file), `library_ir.bas` (`ir`).
- `crates\driver\tests\cli.rs` (`--ignored`, they build): a C `.a` without a header, a C++ `.a`, a `.o`, a DLL
  beside the program and error 260, each library built from `verification\v37_lib\`'s sources by the clone's
  toolchain into a temporary folder; the lookup errors (`x01`, `x02`, `x13`, `x18`, `x52`, `x53`); D-023 and
  D-024 with this compiler's output.
- `tests\corpus\`: console slice programs with a header in their `<name>.files\` (numbers by value and by
  reference, strings, a TYPE, `ALIAS` with C++ names, `CUSTOMTYPE`), the C runtime with no library name, and
  `kernel32` with `DYNAMIC`, recorded with `qb64pe.exe`.
- `verification\v37_*` built and run with this compiler: the same output, apart from D-023 and D-024.
- Upstream: the 8 programs join `pass.list`.

## Risks / Trade-offs

- **[The lookup reads the file system]** → it runs in the driver only, as for `$EXEICON`; front-end tests get a
  fixed result. A compile in the LSP may not report a missing library (D2).
- **[A header's own includes are not in the cache key]** → stated in D4 and in `crates\README.md`; the
  libraries of the tests have no such includes.
- **[`nm` output differs between toolchains]** → the clone's `nm` is the one the old compiler runs; the
  command-line tests run it on a C and a C++ library.
- **[The C++ type of a `BYVAL` argument]** decides an overload with a header → follow the old compiler's text,
  pinned by `cpp` tests and the slice program of `ALIAS`.
- **[Two divergences]** → both turn an old compiler's failed compile into a working program (the 2026-10-08 rule:
  a failed compile is fixed); listed in `DIVERGENCES.md` and put to the user with the change.
