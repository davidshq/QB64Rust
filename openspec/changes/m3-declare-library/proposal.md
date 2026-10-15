# Proposal

## Why

The user chose `DECLARE LIBRARY` as the next step of M3 after `m3-whole-array-assign` ("do declare library",
2026-10-15): it is the first line of the upstream demand report (9 programs), and real QB64 programs use it to
call C code, headers and Windows DLLs. Of the 9 upstream programs, 8 need nothing else; `on_error/errtest` also
needs `ON ERROR GOTO _NEWHANDLER`/`_LASTHANDLER` and stays out (user, answering Claude's question).

Read and measured before this proposal (`qb64pe.bas` at the reference clone's commit; `verification\v37_*`, 76
programs recorded with `qb64pe.exe`, 2026-10-15; every fact with its program in `study\00` §5, "`DECLARE
LIBRARY`"):

- **Four forms**, `DECLARE [STATIC|DYNAMIC|CUSTOMTYPE] LIBRARY ["name", …]` … `END DECLARE`, holding `SUB` and
  `FUNCTION` headers with `ALIAS` (a C or C++ name: `"h_max<int32_t>"`, `"hns::twice"`) and `BYVAL`
  parameters. A block may stand in a SUB, an included file or a `$IF` branch; its names are global.
- **Finding the library**: a `.lib`, `.a` or `.o` (`DYNAMIC`: a `.dll`, also in System32) by the written path
  from the old compiler's own folder, where a path of exactly `./` means the source's folder; then a header
  `.h`/`.hpp`, which alone is also looked for from the source's (or included file's) folder. Not found: "LIBRARY
  not found", "DYNAMIC LIBRARY not found".
- **What it writes**: a library on the link command; a header `#include`d; without a header a prototype per
  declaration whose linkage (C or C++) the old compiler takes from `nm` on the library ("Could not find
  sub/function … in …" when it is not there); `DYNAMIC`: `LoadLibrary` and `GetProcAddress` when the program
  starts (errors 259 and 260, before the first statement, not trappable); `CUSTOMTYPE`: a pointer of the
  declared type to the C function.
- **The call**: arguments as for a program's SUB (a place of the parameter's type by address, anything else
  through a copy), `BYVAL` by value with an argument whose C++ text is `0` as `NULL` (`(0)` and a `CONST` 0
  too, `1 - 1` not), a `STRING` as `char*` to its bytes, a TYPE as `void*`; a result is cast to the declared
  type (a C `double` into a `LONG` FUNCTION is truncated); a `STRING` result is a `char*` copied into a string.
- **Three behaviours that fail to compile** in the old compiler: a header named by two blocks is included twice
  (a header without a guard fails in C++); one header in a block writes no prototype for the block's other
  libraries; with several libraries in one block every name is looked up in the last one only.

## What Changes

- **The four forms compile and run on Windows**, with the old compiler's lookup, rejections and messages (each
  measured form), its C++ (prototypes, `#include`, `nm` linkage, the DLL loading and its errors 259 and 260, the
  `CUSTOMTYPE` pointers) and its calls and results.
- **The driver finds the libraries** (between `sema` and the C++), taking the reference clone as "the old
  compiler's folder", and runs the clone's `nm` for a library without a header; the libraries go on the link
  command and their bytes, with the headers', into the build cache's key.
- **A header is included once**, however many blocks name it (the old compiler includes it once per block, and
  a header without a guard then fails in C++): `DIVERGENCES.md` D-023. For the user to confirm.
- **With several libraries in one block each name is looked up in all of them** (the old compiler looks only in
  the last and rejects the program): `DIVERGENCES.md` D-024. For the user to confirm.
- **A header in a block still writes no prototype for the block** (followed; listed in `SOMEDAY.md` "QB64pe
  behaviours to review"): a prototype could disagree with what a header declares.
- **Tests and lists**: `verification\v37_*` with `v37_lib\build_libs.py`; front-end tests (each rejected form,
  the C++ of each form); command-line tests that build a C library, a C++ library and a DLL from source and run
  a program against them; console slice programs on `tests\corpus\slice.list` with a header in their
  `<name>.files\`, the C runtime and `kernel32`; the 8 upstream programs join `pass.list` (310 to 318 of 399); CI's `tier2` job runs the pass list from
  the clone's `tests\compile_tests` instead of the copy, which lacks the two Windows binaries two of them need
  (`lib-windows.a`, `lib.dll`; design D4).

Out of scope: `on_error/errtest` (`_NEWHANDLER`, `_LASTHANDLER`); Linux and macOS lookup (`.so`, `.dylib`,
`/usr/lib`, `dlopen`), as the rest of the project is Windows only; the IDE's folder in the lookup.

## Capabilities

### New Capabilities

- `language/external-libraries`: `DECLARE LIBRARY` in its four forms: the declarations, finding the library,
  the calls, arguments and results, the start-up errors of a DLL, and the forms that are rejected.

### Modified Capabilities

- `compiler/pipeline`: requirement "The build follows the program" (the libraries a program names are linked,
  and their bytes and the headers' are part of the build cache's key).

## Impact

- `crates\sema` (`check\blocks.rs`: the block no longer marked; `check\proc.rs`: library procedures entered in
  pass 1, their parameters, `ALIAS`, the rejections; calls to them; `lib.rs`: the library and its procedures in
  `Program`), `crates\ir` (a call to a library procedure, the program's libraries, dump and validation),
  `crates\codegen-cpp` (a new `library.rs`: `regsf.txt`, `maindata.txt`, `global.txt`, the call),
  `crates\driver` (a new `library.rs`: the lookup, `nm`, the link command, the cache key), `crates\lsp` (the
  lookup's errors, if it reaches the driver's function).
- Tests: `tests\frontend\`, `crates\driver\tests\cli.rs`, `tests\corpus\` (new slice programs),
  `tests\upstream\pass.list`, the shrink-only lists; `.github\workflows\rust.yml` (the `tier2` job's upstream
  step) and the runner's `--compile-tests` help text.
- Documents: `DIVERGENCES.md` (D-023, D-024), `SOMEDAY.md`, `DECISIONS.md`, `study\00` §5, `CLAUDE.md`'s layout
  table (`v37*`), `crates\README.md`, `tests\upstream\README.md`, `STATUS.md`, `ROADMAP.md`.
