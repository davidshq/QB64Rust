# 48. Language features: Python, C/C++, Rust, Go, JavaScript and QB64Rust

Written 2026-11-11, at the user's request for a table of the features of leading languages beside QB64Rust's.
Sections 1 to 3 are a reference table. Sections 4 and 5 weigh adding each missing feature. Nothing here is
decided. The gaps it shows are discussed in `study\47` section 6,
and whether language extensions are allowed behind a switch is question 2 of `study\47` section 8.

**Sources.** The QB64Rust column comes from `STATUS.md`, the spec index in `crates\README.md` ("What the
compiler supports") and `study\47`. The other columns come from Claude's general knowledge, not from a search
or from reading the languages' documents. Check a cell before relying on it.

QB64Rust implements the QB64pe 4.7.0 dialect. So its column is also QB64pe's for the language table, and
differs from QB64pe's in the tooling table.

## 1. Language

| Feature | Python | C / C++ | Rust | Go | JavaScript | QB64Rust |
|---|---|---|---|---|---|---|
| Typing | Dynamic, optional hints | Static | Static, inferred | Static, inferred | Dynamic | Static; type suffixes (`%`, `&`, `$`); variables implicit unless `OPTION _EXPLICIT` |
| Execution | Bytecode interpreter | Compiled to native | Compiled to native | Compiled to native | JIT | Compiled to native, through C++ and libqb |
| Memory | Reference counts and GC | Manual; RAII in C++ | Ownership and borrowing | GC | GC | Runtime-managed strings and arrays; manual `_MEM` blocks; no GC |
| Null / memory safety | Safe; `None` errors at run time | Unsafe | Safe by default, `Option` | Safe; nil panics | Safe; `undefined` errors | No references or null; arrays and `_MEM` blocks are checked at run time ("Memory region out of range") unless `$CHECKING:OFF` |
| Error handling | `try`/`except` | Return codes; exceptions in C++ | `Result`, `?`, panics | Error values, `panic`/`recover` | `try`/`catch` | `ON ERROR GOTO`, `RESUME`, `ERR`, `ERL` |
| Records | Classes, dataclasses | `struct`; classes in C++ | `struct` | `struct` | Objects, classes | `TYPE`, no methods |
| Methods / OOP | Classes, inheritance | C++ only | Traits and `impl`, no inheritance | Methods, interfaces, embedding | Prototypes, classes | None |
| Enums / sum types | `Enum`; unions by hints | `enum`; `std::variant` | `enum` with data | `iota` constants | None | `CONST` only |
| Generics | Duck typing, generic hints | Templates (C++) | Generics with trait bounds | Generics since 1.18 | Not needed (dynamic) | None |
| Functions as values, closures | Yes | Function pointers; lambdas in C++ | Yes | Yes | Yes | No |
| Default / optional parameters | Yes, and keyword arguments | C++ only | No | No | Yes | No; arguments pass by reference by default |
| Operator overloading | Yes | C++ only | Through traits | No | No | No |
| Pattern matching | `match` since 3.10 | `switch` | `match`, exhaustive | `switch`, type switch | `switch` | `SELECT CASE` with ranges and `IS` |
| Built-in collections | List, dict, set, tuple | Arrays; STL in C++ | `Vec`, `HashMap` in std | Slices, maps | Array, Map, Set | Arrays only: multi-dimensional, any lower bound, `REDIM _PRESERVE` |
| Strings | Unicode | Bytes; `std::string` | UTF-8 | UTF-8 bytes | UTF-16 | Bytes in CP437; `STRING * n` fixed length |
| String interpolation | f-strings | `std::format` (C++20) | `format!` | `Sprintf` | Template literals | No; `PRINT USING` and `+` |
| Modules / namespaces | Modules, packages | Headers; namespaces and modules in C++ | Crates, modules | Packages | ES modules | `$INCLUDE` only; one global namespace, one compilation unit |
| Concurrency | Threads, `asyncio` | Threads; coroutines in C++20 | Threads, `async`, checked by `Send`/`Sync` | Goroutines, channels | Event loop, `async`, workers | None; `ON TIMER` and `ON KEY` events |
| Metaprogramming | Decorators, metaclasses | Preprocessor; templates, `constexpr` | Macros, `const fn` | `go generate`, reflection | `Proxy`, `Reflect` | `$IF` / `$LET` precompiler |
| C interop | `ctypes`, C API | Native | `extern "C"` | cgo | Native addons, WebAssembly | `DECLARE LIBRARY` |
| `GOTO` | No | Yes | No | Yes | No | Yes, with `GOSUB` and line numbers |
| Graphics, sound, input in the language | No (libraries) | No | No | No | Browser APIs | Yes: `SCREEN`, `LINE`, `_LOADIMAGE`, `PLAY`, `_SNDOPEN`, mouse, fonts, OpenGL |

## 2. Tooling

| Feature | Python | C / C++ | Rust | Go | JavaScript | QB64Rust |
|---|---|---|---|---|---|---|
| Package manager | pip | None standard (vcpkg, Conan) | Cargo | Go modules | npm | None |
| Formatter | Black, Ruff | clang-format | rustfmt | gofmt | Prettier | Built in (`-y`, and in the editor) |
| Language server | Pyright | clangd | rust-analyzer | gopls | tsserver | Built in |
| Debugger | pdb, debugpy | gdb, lldb | gdb, lldb | Delve | Browser and Node tools | By BASIC line, in VS Code |
| Unit tests | `unittest`, pytest | Libraries | Built in | Built in | Libraries; `node:test` | Built in (`qb64rust test`) |
| Coverage | coverage.py | gcov, llvm-cov | llvm-cov | Built in | c8, Istanbul | Built in (`--coverage`) |
| Lints | Ruff, Pylint | clang-tidy | Clippy | `go vet` | ESLint | Five lints |
| Platforms | Nearly all | Nearly all | Many | Many | Browser and server | Windows x64, Linux x64 |
| Web target | Pyodide | Emscripten | WebAssembly | WebAssembly | Native | None |

## 3. What stands out

- **Biggest language gaps:** closures, methods on types, growable lists and maps, modules, `TRY`/`CATCH`,
  default parameters, string interpolation and Unicode strings. Every one is an extension that QB64pe would
  reject, so a program that used one would no longer build with QB64pe.
- **Where QB64Rust is ahead:** graphics, sound and input are part of the language. The formatter, language
  server, debugger, tests, coverage and lints ship with the compiler, as they do with Go and Rust.
- **Tooling gaps:** no package manager, no web target, and no ARM or macOS build.

## 4. The missing features, one by one

Written 2026-11-11, in answer to the user's "walk me through each feature we don't have and the pros/cons of
adding it". Fifteen language features and three tooling items are missing. The view here: seven are worth
adding in a small form, three wait on other work, and five are not worth it. None of the design claims was
checked against the code.

### 4.1 Costs every language feature shares

- **It breaks QB64pe compatibility.** A program that uses the feature no longer builds with QB64pe. So each
  one needs the switch that question 2 of `study\47` section 8 asks about.
- **A new keyword can break old programs.** A program with a variable named `TRY` or `LIST` stops compiling.
  QB64pe's answer is the underscore prefix (`_TRY`), which is safe but ugly.
- **Every tool must learn it.** The parser, checker, emitter, formatter, language server, debugger, lints and
  coverage each need the new form, and that cost stays.

Writing the code is minutes to hours of Claude's time for most of these. The real costs are the three above,
and the user's time to decide and review.

### 4.2 Worth adding, in a small form

| Feature | For | Against | Small form |
|---|---|---|---|
| Enums | Replaces runs of `CONST`; the checker can catch a wrong value. No runtime work | Little, beyond the new keyword | `ENUM … END ENUM` that compiles to constants |
| Default and `BYVAL` parameters | Removes the hand copies that ports write today. `BYVAL` is already a `SOMEDAY.md` row (`pds-switch`) from the PDS 7.1 ports | Call-site checking is an area with many measured QB64pe quirks | Both, for BASIC procedures |
| String interpolation | Much easier to read than `+` and `STR$`. Pure front-end work | `STR$(5)` gives `" 5"` with a leading space, so the number format needs a ruling | A marked literal that expands to concatenation |
| `TRY`/`CATCH` | `ON ERROR GOTO` is global and `RESUME` is hard to follow. A local handler is what every newcomer expects | It must coexist with `ON ERROR`, `RESUME NEXT` and `GOSUB` in the same program. This has the most correctness risk in this group | Catch runtime error codes only; no user exception types |
| Methods on types | `player.Move 3` reads better than `MovePlayer player, 3`, and editor completion after the dot becomes useful | Invites requests for inheritance and interfaces, which cost far more | Sugar for a SUB whose first parameter is the record. No inheritance |
| Lists and maps | The biggest practical gap. Programs today hand-roll `REDIM _PRESERVE` and linear searches | Without generics, each needs a typed built-in form. Rules are needed for `TYPE` members, parameters and file I/O of these values | Two built-in containers with a declared element type |
| Procedures as values | Allows callbacks, a sort with a comparison function, and tables of handlers | Full closures capture variables, which needs heap lifetimes that the runtime does not have | References to named SUBs and FUNCTIONs only; no captured variables |

### 4.3 Worth it, but gated on other work

- **Namespaces or modules.**
  - For: every name is global today, so libraries prefix everything. A package manager depends on this.
  - Against: name resolution is where most of the measured QB64pe behaviors live, so the regression risk is
    the highest of any item. It needs its own OpenSpec design.
- **Unicode strings.**
  - For: file names, text files and the web are all UTF-8 now.
  - Against: strings are bytes everywhere: `LEN`, `MID$`, `ASC`, fixed-length strings, record files. Changing
    the string type breaks old programs.
  - Small form: keep byte strings and add UTF-8 helper functions and a UTF-8 source file option.
- **Web, ARM and macOS targets.**
  - For: a web build is how hobby games get shared today.
  - Against: all three wait on the runtime redesign row in `SOMEDAY.md`, because the runtime ABI relies on
    x86 memory order. macOS is already postponed (`DECISIONS.md` 2026-11-07).

### 4.4 Not worth adding

- **Generics.** Only library authors need them, and built-in lists and maps cover the common case. They add
  the most checker complexity of anything here.
- **Operator overloading.** The value for games and tools is small, and it makes code harder to read for the
  beginners this dialect serves.
- **Destructuring pattern matching.** It needs sum types first. `SELECT CASE` with ranges already covers what
  BASIC programs do.
- **Threads or `async`.** libqb is not built for BASIC code that runs on several threads. Data races would be
  a new class of bug that the debugger and tests cannot show well.
- **Macros.** They defeat the formatter, the language server and line-level debugging. `$IF` and `$LET` are
  enough.

### 4.5 Tooling

- **Package manager.**
  - For: shared `.bi` and `.bm` libraries exist; the `community-programs` change pinned 66 repositories.
  - Against: it is a staying cost (a manifest format, a fetch tool, version rules), and it is weak without
    namespaces.
  - Small form: a `[dependencies]` table in `qb64rust.toml` that fetches a pinned git commit.

## 5. The panel's view

- **Pragmatic engineer:** enums, default parameters and interpolation first. They are front-end only and
  carry little risk.
- **QB64 engineer:** everything goes behind one switch that a port leaves off. Use plain keywords under the
  switch, not underscore names.
- **Compiler engineer:** decide the switch policy once, then make one OpenSpec change per feature.
  `TRY`/`CATCH` and namespaces need real designs; the rest are small.
- **Rust engineer:** lists and maps are the only item with real runtime code. Keep it a thin layer over C++
  containers.
- **Test engineer:** there is no QB64pe oracle for an extension, so each feature needs its expected outputs
  written by hand, as the fixes switch does today.
- **A newcomer from Python or JavaScript**, added because these features are aimed at them: lists, maps and
  interpolation are missed in the first hour; the rest much later.

**Recommendation:** answer question 2 of `study\47` section 8 first (answered the same day: extensions go
behind a switch, `DECISIONS.md` 2026-11-11). Then build in this order: enums, default and `BYVAL` parameters, interpolation, lists and maps, methods, procedure references,
`TRY`/`CATCH`.
