# 48. Language features: Python, C/C++, Rust, Go, JavaScript and QB64Rust

Written 2026-11-11, at the user's request for a table of the features of leading languages beside QB64Rust's.
Sections 1 to 3 are a reference table. Sections 4 and 5 weigh adding each missing feature. Nothing here is
decided. The gaps it shows are discussed in `study\47` section 6,
and whether language extensions are allowed behind a switch is question 2 of `study\47` section 8.

**Sources.** The QB64Rust column comes from `STATUS.md`, the spec index in `crates\README.md` ("What the
compiler supports") and `study\47`. The other columns come from Claude's general knowledge, not from a search
or from reading the languages' documents. Check a cell before relying on it. Section 7, added 2026-11-16,
was checked against the sources it names, and so was the C/C++ cell of the "Metaprogramming" row.

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
| Metaprogramming | Decorators, metaclasses | Preprocessor; templates, `constexpr`; static reflection in the approved C++26 draft, which is not yet a published standard (section 7.4) | Macros, `const fn` | `go generate`, reflection | `Proxy`, `Reflect` | `$IF` / `$LET` precompiler |
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
checked against the code. Two rows were added to section 4.2 on 2026-11-16: defaults on `TYPE` fields and
fields by name, from the example in section 7.

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
| Defaults on `TYPE` fields | Removes a SUB that sets each field by hand: `tuning_defaults` in Civil War Strategy is about 135 lines (section 7). FreeBASIC allows `field As type = value`, and VB.NET allows it on class fields | VB.NET refuses it on a `Structure` (error BC31049), the nearest thing to a `TYPE`. Rules are needed for `DIM`, `REDIM`, `ERASE`, records read from files, and arrays of records | `name AS type = constant` in a `TYPE` block, set wherever a record of that type is created |
| Fields by name | Lets one loop read a settings file into a record. In Civil War Strategy, `tuning_set` and `tuning_changes$` are about 820 generated lines that this would replace (section 7) | No BASIC has it built in. VB.NET reaches it only through .NET reflection. A misspelled name is found only when the program runs | `_FIELDCOUNT`, `_FIELDNAME$`, and get or set a field by name. The compiler expands each into a `SELECT CASE` over the fields; nothing is looked up by reflection |

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
- **Macros that the user writes.** They defeat the formatter, the language server and line-level debugging.
  `$IF` and `$LET` are enough. This does not rule out code that the compiler writes for a narrow built-in,
  such as the fields by name of section 4.2 (section 7.3).

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

## 6. A worked example: one status message

Written 2026-11-14, at the user's request, to show what QB64Rust might add. The example is one line of the
Civil War Strategy port (`..\civil-war-strategy`, `ui\main_menu.bm` line 89). It clears the bottom line of the
screen and says which army the railroad is carrying and where to:

```basic
CALL clear_bottom_line
COLOR COLOR_LIGHT_CYAN
PRINT "Railroad is already carrying "; army_name$(train_army(current_side)); " to "; city_name$(army_order(train_army(current_side)));
```

The same three statements appear, with other text, after most of the port's 104 calls of `clear_bottom_line`.

### 6.1 The same message in five popular languages

Each one assumes the usual modern data model: an army is an object that holds its name and a reference to its
destination city, and `status` is a function that clears the line, sets the color and prints. The code comes
from Claude's general knowledge and was not compiled.

| Language | Line | Code |
|---|---|---|
| Python | 1 | `army = side.train_army` |
| Python | 2 | `status(f"Railroad is already carrying {army.name} to {army.destination.name}", CYAN)` |
| JavaScript | 1 | `const army = side.trainArmy;` |
| JavaScript | 2 | ``status(`Railroad is already carrying ${army.name} to ${army.destination.name}`, CYAN);`` |
| Java | 1 | `Army army = side.trainArmy();` |
| Java | 2 | `status("Railroad is already carrying %s to %s".formatted(army.name(), army.destination().name()), CYAN);` |
| C# | 1 | `var army = side.TrainArmy;` |
| C# | 2 | `Status($"Railroad is already carrying {army.Name} to {army.Destination.Name}", Cyan);` |
| C++ 20 | 1 | `auto& army = *side.train_army;` |
| C++ 20 | 2 | `status(std::format("Railroad is already carrying {} to {}", army.name, army.destination->name), CYAN);` |
| QB64 today | 1 | `a = train_army(current_side)` |
| QB64 today | 2 | `show_status "Railroad is already carrying " + army_name$(a) + " to " + city_name$(army_order(a)), COLOR_LIGHT_CYAN` |

The "QB64 today" rows use a local variable and a `show_status` SUB that the port does not have yet. They are
the shortest form the dialect allows now.

### 6.2 What makes the five short, and where QB64Rust stands

| Feature | Which of the five have it | In QB64Rust |
|---|---|---|
| String interpolation or formatting | Built into Python, JavaScript and C#. Java and C++ use a format function | Missing. Section 4.2 lists it as worth adding. The open point is how numbers print, because `STR$(5)` gives `" 5"` |
| Records with named fields (`army.name`) | All five | Already there: `TYPE`, with variable-length string members. The port keeps parallel arrays instead |
| References to records (`army.destination` is the city itself) | All five | Missing, and not among the seventeen features of section 4. The runtime has no heap lifetimes for it, the obstacle section 4.2 names for closures |
| Methods or properties on types (`side.train_army`) | All five | Missing. Section 4.2 proposes sugar for a SUB or FUNCTION whose first parameter is the record |
| Default parameters (leave the color out) | Python, JavaScript, C# and C++. Java uses overloads | Missing. Section 4.2 lists it with `BYVAL` |

### 6.3 What the line could become

With interpolation and default parameters (the syntax is made up for illustration):

```basic
a = train_army(current_side)
show_status $"Railroad is already carrying {army_name$(a)} to {city_name$(army_order(a))}"
```

Methods on records could shorten the lookups further, to something like `armies(a).DestinationName`. Without
references to records the index `a` stays.

### 6.4 What the example changes

- It supports the order of section 5. Interpolation removes most of this line's length, and it is front-end
  work only.
- It shows one feature section 4 did not weigh: references to records. Claude's view is to leave them out,
  because of the runtime cost above. Nothing is decided.
- A port that uses an extension no longer builds with QB64pe (section 4.1). Whether Civil War Strategy may use
  one is a decision for that port.

## 7. A second worked example: the tuning knobs

Written 2026-11-16, from the user's question about the length of Civil War Strategy's tuning files and what a
modern language would provide. Every claim about another language here was checked against the source
named beside it (`CLAUDE.md` working rule 17). Nothing here is decided.

### 7.1 The example

Civil War Strategy has about 135 tuning knobs: numbers a playtester can change in `TUNING.DAT` without
building the game again. The list is `tools\tuning\knobs.txt` in the game's repo, one line per knob: name,
default, least, most and a description. That file is as short as any language would make it.

`tools\tuning\make_knobs.py` writes BASIC from it. `io\tuning_knobs.bm` is 1,009 lines, because BASIC cannot
do three things:

| Generated part | Lines | What BASIC lacks |
|---|---|---|
| `SUB tuning_defaults`, one assignment per knob | about 135 | A default value in the declaration |
| `SUB tuning_set`, one `CASE` of 5 lines per knob | about 680 | A way from a name in a file to the variable it names |
| `FUNCTION tuning_changes$`, one line per knob | about 140 | A loop over a record's fields |

The generator works. Its costs stay: a second toolchain, a step to run after each edit, `tests\test_tuning.py`
to catch generated files that are out of date, and diffs of hundreds of lines when a knob is added.

### 7.2 How other languages do it

| Language | Defaults in the declaration | A field by its name | The loader for these files |
|---|---|---|---|
| QB64 and QB64Rust today | No | No | Generated by a Python script |
| Go | No; fill the struct with defaults, then read the file over it | Yes, at run time, through `reflect` and struct tags | One loop over the fields |
| Rust | Through the `smart-default` crate: `#[default(100)]` | At compile time, through derive macros such as `serde`'s | `#[derive(Deserialize)]` |
| C# and VB.NET | Class fields yes; VB.NET `Structure` fields no | Yes, at run time, through .NET reflection; a compile-time generator exists for C# | The configuration binder |
| Swift | Yes | The compiler writes `Codable` when every property is `Codable` | Synthesized by the compiler |
| Zig | Yes | At compile time: `@typeInfo` and `inline for` over the fields | Ordinary Zig code that runs while compiling |
| FreeBASIC | Yes, on `TYPE` fields | No | Macros, or a generator |
| True BASIC | No records at all | No | By hand, or parallel arrays filled by `DATA` |

Details:

- **Go.** The spec says struct tags "are made visible through a reflection interface … but are otherwise
  ignored". A misspelled tag key is ignored silently. `encoding/json` sets fields through `reflect`, ignores
  unknown keys unless `DisallowUnknownFields` is set, and leaves fields absent from the input as they were.
  That last point is relied on but not written in the package's documents (Go issue 27172). Code generators
  such as easyjson ("without the use of reflection", 4 to 5 times faster by its own measure) and sqlc
  exist beside it.
- **Rust.** Rust has no reflection at run time. A derive macro reads the struct while it compiles and writes
  the code, as `make_knobs.py` does outside the compiler. `#[serde(default)]` fills missing fields from
  `Default`; `#[serde(deny_unknown_fields)]` rejects unknown keys. The `validator` crate checks ranges with
  `#[validate(range(min = 0, max = 19999))]`. Typed fields give a check for free: tried with serde 1 and
  toml 0.8, `cost_recruit = 1.5` and even `cost_recruit = 1.0` are refused for an `i32` field ("invalid type:
  floating point `1.5`, expected i32").
- **C# and VB.NET.** `FieldInfo.SetValue` sets a field found by name; Microsoft's page has a VB example. The
  .NET configuration binder maps keys to properties by name, later sources override earlier ones (as
  `load_tuning` layers its files), and "the default binder is reflection-based". Its compile-time generator
  "uses C# interceptors", added because reflection does not work in ahead-of-time compiled programs.
  Reflection is still the default; the generator is the choice for those programs.
- **Swift.** SE-0166: "If all properties are Codable, protocol implementation is automatically generated by
  the compiler."
- **Zig.** The reference says `@typeInfo` "Provides type reflection", with fields "in the same order as
  appearance in the source file". Zig's own `std.meta.eql` loops `inline for (info.fields)` and reads each
  field with `@field`.
- **FreeBASIC** (its manual, read in `..\FreeBASIC\doc\manual`; nothing copied, rule 3). `TYPE` fields take
  `[= initializer]`. The only type information at run time is `Operator Is`, for types that extend `Object`.
  It has `#define` and `#macro` with the `#` and `##` operators, so a FreeBASIC program would likely use C's
  "X-macro" pattern. Whether its preprocessor allows that pattern was not tried: there is no `fbc` here.
- **True BASIC.** Its list of the statements in all versions has no `TYPE` or record statement; the only
  declarations of type are `DECLARE NUMERIC` and `DECLARE STRING`. It has `MODULE` with `PRIVATE`, `PUBLIC`
  and `SHARE`, which govern who sees a name.

### 7.3 What it suggests for QB64Rust

- Opinion has moved toward doing this work at compile time: Rust's derive, Swift's `Codable`, Zig's
  `@typeInfo`, C#'s generators beside reflection. The cases that moved name speed and ahead-of-time
  compilation as reasons.
- A narrow built-in fits BASIC better than either Go's or Rust's form. The user sees functions, as in Go:
  `_FIELDCOUNT`, `_FIELDNAME$`, a field by name. The compiler knows every `TYPE`'s fields, so it expands each
  call into the `SELECT CASE` that `make_knobs.py` writes today, as Rust's derive would. No macro language is
  added, so the formatter, language server and debugger need nothing new. These are the two rows added to
  section 4.2.
- Defaults on `TYPE` fields have BASIC precedents in FreeBASIC and VB.NET classes. Fields by name have none:
  no BASIC has it built in, and FreeBASIC's answer is macros, which section 4.4 leaves out.
- Both are extensions, so they go behind the switch decided on 2026-11-11 (`DECISIONS.md`).

### 7.4 Sources

Checked 2026-11-16:

- Go: [language spec](https://go.dev/ref/spec) ("Struct types", "Integer overflow");
  [`encoding/json`](https://pkg.go.dev/encoding/json) and its
  [`decode.go`](https://go.dev/src/encoding/json/decode.go); [issue 27172](https://golang.org/issue/27172);
  [easyjson](https://github.com/mailru/easyjson); [sqlc](https://docs.sqlc.dev/en/latest/);
  [generics in Go 1.18](https://go.dev/blog/intro-generics).
- Rust: [serde container attributes](https://serde.rs/container-attrs.html);
  [smart-default](https://github.com/idanarye/rust-smart-default);
  [validator](https://docs.rs/validator/latest/validator/);
  [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) (overflow checks on in `dev`, off
  in `release`); [rust-analyzer configuration](https://rust-analyzer.github.io/book/configuration.html) (it
  runs `proc-macro-srv` processes to expand procedural macros); [rustfmt issue
  6047](https://github.com/rust-lang/rustfmt/issues/6047) (rustfmt does not format the inside of macro calls
  written with braces unless `format_brace_macros` is set, and has broken a `macro_rules!` body);
  [serde on crates.io](https://crates.io/api/v1/crates/serde) (1.72 billion downloads).
- .NET: [`FieldInfo.SetValue`](https://learn.microsoft.com/en-us/dotnet/api/system.reflection.fieldinfo.setvalue);
  [configuration](https://learn.microsoft.com/en-us/dotnet/core/extensions/configuration);
  [configuration source generation](https://learn.microsoft.com/en-us/dotnet/core/extensions/configuration-generator);
  [System.Text.Json source generation](https://learn.microsoft.com/en-us/dotnet/standard/serialization/system-text-json/source-generation);
  [BC31049](https://learn.microsoft.com/en-us/previous-versions/visualstudio/visual-studio-2008/dk991yyb(v=vs.90)).
- Swift: [SE-0166](https://github.com/swiftlang/swift-evolution/blob/main/proposals/0166-swift-archival-serialization.md).
- Zig: [language reference](https://ziglang.org/documentation/master/) (`@typeInfo`);
  [`std/meta.zig`](https://github.com/ziglang/zig/blob/master/lib/std/meta.zig).
- C++: [Herb Sutter, June 2025](https://herbsutter.com/2025/06/) (reflection, P2996R13, voted into the C++26
  draft at Sofia); [Herb Sutter, March 2026](https://herbsutter.com/2026/03/) (C++26 approved, 114 to 12);
  [WG21 papers of 2026](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2026/) (P4238R1 and P4433R0 show
  the dispute over contracts). News sites report that the ISO ballot on the draft failed in October 2026;
  no primary source for that was found.
- True BASIC: [statements and functions](https://www.truebasic.com/s/TB_Commands.pdf).
- FreeBASIC: its manual in `..\FreeBASIC\doc\manual\cache`: `KeyPgType.wakka`, `KeyPgOpIs.wakka`,
  `KeyPgPpmacro.wakka`, `ProPgMacros.wakka`.

Not checked: that the JSON and configuration generators of .NET cannot be used from VB.NET (only a
third-party package says so), how rust-analyzer expands `macro_rules!`, FreeBASIC's X-macro pattern, and
the "Yes" for defaults in the declaration in the Swift and Zig rows.
