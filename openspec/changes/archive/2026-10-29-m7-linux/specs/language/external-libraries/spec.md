## MODIFIED Requirements

### Requirement: Finding a library
On Windows, the compiler SHALL look for each name as the old compiler does there, after dropping a version after `.` or
`:`: a `.lib`, `.a`, `.o` (or, `DYNAMIC`, a `.dll`, also in System32) by the written path from the old compiler's
folder, `./` alone meaning the source's folder; then a header `.h` or `.hpp`, also from the source's or the
included file's folder. On Linux it SHALL look as the old compiler does on Linux (`qb64pe.bas` 4639 onwards): by the
written path, then under `/usr/lib64/`, then `/usr/lib/`, then by the bare name, for `lib<name>.so.<version>`,
`lib<name>.so` (neither for `STATIC`), `lib<name>.a` and `lib<name>.o`; then a header as on Windows.

#### Scenario: A library beside the program
- **WHEN** `v37c.a` lies beside the program and the block names `"./v37c"`
- **THEN** the program links it; with `"v37c"` compiling fails with "LIBRARY not found"

#### Scenario: A header from an included file's folder
- **WHEN** an included file in `v37_inc\` names `"../v37_lib/v37h"`
- **THEN** the header `v37_lib\v37h.h` is found and its function is called

#### Scenario: A DLL of the system
- **WHEN** a block names `DECLARE DYNAMIC LIBRARY "kernel32"` and declares `FUNCTION lstrlenA& (s AS STRING)`
- **THEN** `lstrlenA&("seven!!")` is 7

#### Scenario: A shared object beside the program on Linux
- **WHEN** `libv37s.so` lies beside the program on Linux and a plain block names `"./v37s"`
- **THEN** the program links it with an `rpath` to its folder, and runs from another working folder

#### Scenario: A versioned shared object
- **WHEN** a `DYNAMIC` block names `"./v37s.1"` on Linux and `libv37s.so.1` lies beside the program
- **THEN** that file is the one the program loads

#### Scenario: STATIC skips shared objects
- **WHEN** a `STATIC` block names `"./v37b"` on Linux and both `libv37b.so` and `libv37b.a` lie beside the program
- **THEN** the program links `libv37b.a`

#### Scenario: Only in a multiarch folder
- **WHEN** a block names `"z"` on Ubuntu, where the library is only `/usr/lib/x86_64-linux-gnu/libz.so`, and no
  header `z.h` is found by the rules above
- **THEN** compiling fails with "LIBRARY not found", as with the old compiler on Linux

### Requirement: DLLs at the start of the program
A `DYNAMIC` library SHALL be loaded, and each of its names found, when the program starts, before its first
statement: a library that does not load SHALL be run-time error 259 and a name the DLL does not export error
260, neither of which `ON ERROR` catches. On Linux the library SHALL be loaded with `dlopen` (`RTLD_LAZY`) by the
name the lookup found, and each name found with `dlsym`, as the old compiler's Linux build does.

#### Scenario: A name the DLL does not export
- **WHEN** a program sets `ON ERROR GOTO handler` and declares `d_none&` from `"./v37dll"`, which does not export
  it
- **THEN** it stops before its first `PRINT` with "Sub/Function does not exist in dynamic library"

#### Scenario: A system library loaded at run time on Linux
- **WHEN** a program on Linux declares `DECLARE DYNAMIC LIBRARY "/usr/lib/x86_64-linux-gnu/c:6"` with
  `FUNCTION strlen~%& (s AS STRING)`, or names a library found under `/usr/lib/`
- **THEN** the library `/usr/lib/x86_64-linux-gnu/libc.so.6` loads and `strlen~%&("seven!!" + CHR$(0))` is 7

#### Scenario: A file name written whole on Linux
- **WHEN** a `DYNAMIC` block on Linux names `"/usr/lib/x86_64-linux-gnu/libc.so.6"`
- **THEN** compiling fails with "DYNAMIC LIBRARY not found", as with the old compiler on Linux, which looks for
  `liblibc.so.so.6` there (measured 2026-10-28)

#### Scenario: A shared object that does not load
- **WHEN** a `DYNAMIC` block on Linux names a `.so` that was found when compiling and deleted before the run
- **THEN** the program stops before its first statement with run-time error 259
