# 53. Libraries of other languages: Python first, and which others

Written 2026-11-17. It answers the user's questions of that day: "what would it take for QB64Rust to be able to
integrate Python libraries? e.g., if someone wanted to use Pandas in BASIC?", and then "look at what it would take
to do other languages as well and which would be most valuable to do".

It is a review, so it stays active until the user has answered §8 (`CLAUDE.md` working rule 7). Nothing here is
decided, and nothing here was built or tested. Every statement about what "would work" is a reading of the
documents named, not a measurement.

The companion note is `..\language-history\notes\basicifying-an-existing-language.md`. It asks the opposite
question: whether to drop BASIC and restrict an existing language. Its answer is to keep BASIC and borrow other
languages' libraries, which is what this study looks at.

**Sources.** All read on 2026-11-17 through a fetch tool's extract (`CLAUDE.md` working rule 17). §9 lists what
was not checked.

- **QB64Rust**: `docs\reference\entries\declare-library.md`, `docs\reference\chapters\external-libraries.md`,
  `study\48` §4, `study\50`, `study\51`, `study\52`.
- **Python**: [Embedding Python in Another Application](https://docs.python.org/3/extending/embedding.html),
  [C API Stability](https://docs.python.org/3/c-api/stable.html),
  [Buffer Protocol](https://docs.python.org/3/c-api/buffer.html),
  [Using Python on Windows](https://docs.python.org/3/using/windows.html) (the embeddable package).
- **How other languages reach Python**: [reticulate](https://rstudio.github.io/reticulate/) (R),
  [PythonCall](https://juliapy.github.io/PythonCall.jl/stable/pythoncall/) (Julia),
  [PythonKit](https://github.com/pvieito/PythonKit) (Swift),
  [libpython-clj](https://github.com/clj-python/libpython-clj) (Clojure).
- **Other languages**: Rust's [linkage](https://doc.rust-lang.org/reference/linkage.html), Go's
  [build modes](https://pkg.go.dev/cmd/go), [Lua](https://www.lua.org/about.html),
  [QuickJS](https://bellard.org/quickjs/), Node.js's [embedder API](https://nodejs.org/api/embedding.html),
  Emscripten's [calls to JavaScript](https://emscripten.org/docs/porting/connecting_cpp_and_javascript/Interacting-with-code.html),
  .NET's [runtime host](https://learn.microsoft.com/en-us/dotnet/core/tutorials/netcore-hosting), Java's
  [Invocation API](https://docs.oracle.com/en/java/javase/21/docs/specs/jni/invocation.html), the
  [WebAssembly Component Model](https://component-model.bytecodealliance.org/).
- **Numbers**: the [Stack Overflow 2025 survey](https://survey.stackoverflow.co/2025/technology),
  [PyPI](https://pypi.org/), [crates.io](https://crates.io/api/v1/summary), npm's
  [registry](https://replicate.npmjs.com/).

## 1. Short answer

- **Pandas in BASIC is possible, in three stages.** Stage 1 is a library over Python's C API and needs no
  language change. Stage 2 fetches and ships Python per project. Stage 3 is language support for foreign objects,
  and is the real cost (§4).
- **Foreign libraries come in three kinds** (§3). A language that compiles to C's calling convention needs no
  bridge. A language with its own runtime needs that runtime embedded. JavaScript on the web target is the host
  itself.
- **Most valuable, in order** (§6): stronger C interop first, because every bridge is built on it. Then Python.
  Then JavaScript, on the web target only. The others are not worth building now.

## 2. What QB64Rust can call today

A `DECLARE LIBRARY` block calls C and C++ functions from a header, a static library or a DLL
(`docs\reference\entries\declare-library.md`).

| It can pass | It cannot pass |
|---|---|
| Numbers, by value or by address | An array: an array parameter is a compile error |
| A `STRING`, as a pointer to its bytes | A procedure for C to call back: BASIC has no procedures as values (`study\48` §4.2) |
| A `TYPE`, as a pointer to the value | A value with a lifetime: nothing runs when a handle goes out of scope |
| A `char*` result, copied into a new string | |

Whether a single array element passed by address can stand in for the array, as QB64 programs commonly do, was
not checked here.

## 3. Three kinds of foreign library

### 3.1 Languages that compile to C's calling convention

No runtime is embedded. The library is a `.a`, `.lib`, `.dll` or `.so`, and `DECLARE LIBRARY` already calls it.

| Language | How it offers a C interface | Source |
|---|---|---|
| C, C++ | Directly. C++ names go through `ALIAS`, such as `"hns::twice"` | The reference page |
| Rust | Crate type `staticlib`, "recommended for use in situations such as linking Rust code into an existing non-Rust application", or `cdylib`, "a dynamic library to be loaded from another language" | Rust Reference |
| Go | `-buildmode=c-archive` or `c-shared`. "The only callable symbols will be those functions exported using a cgo //export comment" | `go` command |

**The limit.** Only functions written for C are callable. An ordinary Rust crate or Go package offers generics,
traits, slices and closures, and none of that crosses. So "a Rust library in BASIC" means a C-style wrapper that
someone writes in Rust. QB64Rust needs nothing new for it beyond what §2 lacks.

### 3.2 Languages with a runtime to embed

The program starts the other language's runtime inside itself and calls it through that runtime's C or C++
interface.

| Language | The interface | What the documents say about cost |
|---|---|---|
| Python | The C API: `Py_Initialize()`, `PyImport_Import`, `PyObject_GetAttrString`, `PyObject_CallObject` | Linking flags are "not necessarily trivial", because Python loads C extension modules linked against it |
| Lua | "A simple and well documented API" | The Lua library "takes 484K" on 64-bit Linux |
| JavaScript, QuickJS | "A small and embeddable Javascript engine", MIT, ES2025 | "367 KiB of x86 code for a simple hello world program". Its page says nothing about npm packages |
| JavaScript, Node.js | C++ embedder API, with V8's | Breaking changes "may occur on each semver-major release without prior warning" |
| .NET | `nethost` and `hostfxr` | "Only support framework-dependent deployments", so .NET must be installed. "Only one runtime can be loaded inside a single process" |
| Java | The Invocation API: `JNI_CreateJavaVM()` | "Creation of multiple VMs in a single process is not supported" |

Every one of these needs the same five things from QB64Rust:

1. **The runtime found or shipped** beside the program.
2. **A handle type** for the other language's objects.
3. **Calls by name at run time**: get an attribute, call a method, pass named arguments.
4. **Conversions** of numbers, strings and arrays in both directions.
5. **Errors**: the other language's exception becomes a BASIC error.

### 3.3 The host itself, and one later route

- **JavaScript on the web target.** A program built with Emscripten (`study\51`, route A) runs inside a
  JavaScript engine. C++ calls JavaScript with `EM_JS`, `EM_ASM` or `emscripten_run_script()`. Nothing is
  embedded and nothing is shipped.
- **WebAssembly components.** The Component Model is "a broad-reaching architecture for building interoperable
  WebAssembly libraries". Its documentation lists C/C++, C#, Go, JavaScript, Python, Rust and MoonBit as able to
  build components. It could become one bridge to many languages. It was not examined further.

## 4. Python in detail

R, Julia, Swift and Clojure all reach Python the same way: they embed CPython and wrap its objects.

| Bridge | How a call looks | How it gets Python |
|---|---|---|
| reticulate (R) | `os <- import("os")`, then `os$listdir(".")`. A data frame converts to a pandas DataFrame | A virtual environment named `r-reticulate`, or one the user names |
| PythonCall (Julia) | `re = pyimport("re")`. "All Python objects are of type `Py`". `PyArray` "directly wraps the underlying data buffer" | CondaPkg creates an environment for the project, from a `CondaPkg.toml` file |
| PythonKit (Swift) | `let sys = Python.import("sys")`, then `sys.version` | It searches the system; `PYTHON_LIBRARY` names one |
| libpython-clj (Clojure) | `(require-python '[numpy :as np])`, then `(np/expand_dims img :axis 0)` | It runs `python3` to ask where the library is |

### Stage 1: a library, with no language change

A C++ header over the C API and a `.bi` file of `DECLARE LIBRARY` lines. Python objects are held as handles in
`_OFFSET` variables. A sketch, not a design:

```qb64
pd = PyImport("pandas")
df = PyCall1(PyGet(pd, "read_csv"), PyStr("sales.csv"))
PRINT PyToString$(PyCallMethod0(df, "describe"))
PyRelease df: PyRelease pd
```

- **Link against `python3.dll`.** Python's stable ABI means an extension built for 3.x "is ABI-compatible with
  Python 3.x and above", and on Windows it links `python3.dll`, not a version-specific DLL.
- **Arrays without copying.** The buffer protocol gives a pointer to an object's memory, "directly and without
  intermediate copying". `PyObject_GetBuffer` is in the stable ABI since 3.11. So a NumPy array's numbers can be
  read into, or shared with, a BASIC array.
- **It may build with QB64pe too**, as a `$USELIBRARY` library, since it uses only `DECLARE LIBRARY`.
- **What it proves:** that the llvm-mingw toolchain links against python.org's DLL, that pandas loads inside a
  QB64Rust program, and what the program's folder must hold. All three are unknown today.
- **What it lacks:** the caller releases every handle by hand, and a Python exception is an error code to test.

### Stage 2: Python per project

- **The runtime.** The Windows embeddable package is "a ZIP file containing a minimal Python environment",
  "intended for acting as part of another application". Pip is not supported in it; packages "should be
  installed by the application installer alongside the embedded distribution".
- **The settings.** A `[python]` table in `qb64rust.toml` names the Python version and the pinned packages. The
  compiler fetches them, as it fetches its C++ toolchain, and copies them beside the built program. This is the
  dependencies per project of `study\50` §6, for a second kind of dependency.
- **The cost.** A built program is an executable plus a Python folder. It is no longer one file. The folder's
  size with pandas and NumPy was not measured.
- **Linux.** The embeddable package is a Windows download. What to ship on Linux was not read.

### Stage 3: language support, behind the extension switch

Pandas is objects, methods, attributes, keyword arguments, `df["column"]` and exceptions. BASIC has none of
these. Each bridge in the table above leans on its own language's dynamic features. BASIC would need:

- **One foreign-object type.** A variable of this type holds a handle, and the runtime releases it when the
  variable dies. This is new: no BASIC value has a lifetime today.
- **Dot calls resolved at run time**, on that type only: `df.describe()`.
- **Named arguments**: `pd.read_csv("sales.csv", sep := ";")` or a form like it.
- **Conversions** to and from numbers, strings, arrays, and the lists and maps of `study\48` §4.2.
- **`TRY`/`CATCH`** (`study\48` §4.2), so that a Python exception can be handled where it happens.

A sketch of the same program, with no syntax decided:

```qb64
IMPORT PYTHON "pandas" AS pd
DIM df AS PYOBJECT
df = pd.read_csv("sales.csv")
PRINT df.describe()
```

Three of these five items are already on `study\48`'s list for their own sake. So stage 3 comes after methods,
lists and maps, and `TRY`/`CATCH`, not before them.

### Costs that stay

- **Nothing is checked at compile time.** A misspelled method name fails when the program runs. This cuts
  against "a wrong program fails to compile" (`..\language-history\chapters\15-languages-and-ai-agents.md`).
- **The language server knows nothing** about a Python object's members unless it asks a Python process.
- **Tests need a pinned Python** in the suites VM, and there is no QB64pe oracle.
- **The web target** would need Python in the browser. [Pyodide](https://pyodide.org/en/stable/) is "a port of
  CPython to WebAssembly", so it may be possible. It was not examined.
- **Errors and documentation are Python's.** The user reads a Python traceback and pandas' manual.

## 5. What each language would bring

Users are from the Stack Overflow 2025 survey: the share of all respondents who used the language that year.

| Language | Users | Public packages | What a BASIC program would gain |
|---|---|---|---|
| JavaScript | 66% | npm: 4,617,697 registry documents | The browser's own interfaces, on the web target |
| Python | 57.9% | PyPI: 944,523 projects | Data tables, numerics, plotting, machine learning, web clients |
| Java | 29.4% | Not found | Business libraries; little that a game or a beginner needs |
| C# (.NET) | 27.8% | Not found | Windows and business libraries; a link to Visual Basic .NET (4.4%) |
| C++ and C | 23.5%, 22% | No single archive | What QB64 libraries already wrap: images, sound, compression, SQLite |
| Go | 16.4% | Not found | Network servers; reachable only through C exports |
| Rust | 14.8% | crates.io: 380,324 crates | Reachable only through C exports |
| Lua | 9.2% | Not found | Little. Lua is usually the embedded side, not the library side |

The npm figure is the registry database's document count, which may count more than packages.

## 6. Which are most valuable

This ranking is Claude's judgment from §3 to §5. Effort is counted in Claude's time (`CLAUDE.md` working rule 4),
so what weighs is the cost that stays, the risk, and the user's time.

1. **Stronger C interop.** Every bridge in §3.2 is a C library, and every language in §3.1 is reached through
   C. The gaps of §2 limit all of them: arrays, callbacks and handle lifetimes. A second part is a tool that
   reads a C header and writes the `DECLARE LIBRARY` block, so that nobody copies declarations by hand. Whether
   such a tool exists for QB64 was not checked.
2. **Python.** It has the largest gain that BASIC cannot get any other way. Nobody will rewrite pandas or NumPy
   in BASIC, and Python holds the classroom
   (`..\language-history\chapters\08-scripting-languages.md`). Four other languages have built this bridge, so
   the design is known. Stage 1 is the cheap probe.
3. **JavaScript, on the web target only.** There it costs almost nothing, since the program already runs inside
   a JavaScript engine. It belongs to `study\51`. On the desktop it is worth little: QuickJS brings the language
   but not Node's packages, and embedding Node itself is an interface that breaks at each major release.
4. **Rust and Go.** Nothing to build. A library with C exports already works. A `qb64rust.toml` build step that
   runs `cargo` or `go build` first would be a convenience, with a staying cost of two more toolchains to find.
5. **.NET and Java.** Not now. Each needs a large installed runtime, allows one runtime per process, and brings
   libraries that the audiences of `..\language-history\QUESTIONS.md` rarely need. .NET has one special reason
   to come back later: Visual Basic .NET's users are the largest group of BASIC writers
   (`..\language-history\chapters\13-why-languages-win-or-die.md`).
6. **Lua.** Not as a library source. The reverse use, Lua or BASIC scripts inside a BASIC game for modding, is a
   different question.

**One design to share.** The foreign-object type of stage 3 need not be Python's alone. A handle, calls by name
and conversions are what JavaScript, Lua, .NET and Java objects need too. If stage 3 is ever built, it should be
designed once for all of §3.2 and built for Python first.

## 7. The panel's view

- **Pragmatic engineer:** build stage 1 as a probe before any design. It settles the linking and shipping
  questions, and it is useful by itself for a script that wants one pandas call.
- **QB64 engineer:** stage 1 as a `$USELIBRARY` library changes nothing for existing programs and may build with
  QB64pe. Stage 3 goes behind the extension switch (`DECISIONS.md` 2026-11-11).
- **Compiler/languages engineer:** stage 3 puts a dynamically typed island in a statically typed language. Keep
  it to one type, so that the checker's rules for everything else do not change. Build it after methods, maps and
  `TRY`/`CATCH`.
- **Rust engineer:** stage 1 touches no compiler crate. Stage 2 is settings and fetch code of the kind
  `toolchain install` already has. The handle lifetime of stage 3 is the first value in the runtime that needs
  code run when it dies, and that reaches code generation.
- **Test engineer:** expected outputs are written by hand. Python and every package must be pinned, or recorded
  outputs drift when pandas changes its formatting. The suites VM needs Python, as it now has Node.js
  (`DECISIONS.md` 2026-11-17).
- **A data-analysis user**, added because pandas is their tool: the function-call form of stage 1 is tolerable
  in a script, but nobody would choose it over Python. Only stage 3 makes BASIC attractive for this work.
- **A library author**, added because C interop is their daily tool: arrays and callbacks in `DECLARE LIBRARY`
  would help every existing QB64 library, not only the new bridges.

**Recommendation:** answer §8. Then build stage 1 as a probe, and add `SOMEDAY.md` rows for the C interop gaps
of §2 and for stages 2 and 3.

## 8. Questions for the user

1. **A Python folder beside the program:** is a built program that needs one acceptable? It sits beside the
   ruling of 2026-11-16, "a BASIC program should not need another language to be written" (`DECISIONS.md`).
   Claude reads that ruling as being about writing code, not about using libraries. The reading is the user's to
   confirm.
2. **The probe:** should stage 1 be built now, as a library in this repository?
3. **C interop first:** are arrays, callbacks and a header-reading tool for `DECLARE LIBRARY` wanted, ahead of
   stage 3?
4. **Stage 3:** does a foreign-object type join the extension order of `study\48` §5, after methods, lists and
   maps, and `TRY`/`CATCH`?

## 9. Not checked

- Nothing was built. Whether a program linked by llvm-mingw loads `python3.dll` and then pandas is the first
  thing stage 1 would show.
- The size of an embeddable Python with pandas and NumPy.
- What to ship on Linux in place of the Windows embeddable package.
- How a BASIC array's memory is offered to NumPy. The buffer page covers reading an object's memory; the
  function that wraps existing memory (`PyMemoryView_FromMemory`) is on a page that was not read.
- Package counts for NuGet, Maven Central and LuaRocks. Their pages did not show a total.
- Whether QuickJS can load npm packages. Its page does not say.
- COM on Windows, which Visual Basic used to call objects by name. It is general knowledge and was not read.
- Free-threaded Python builds, which use a second stable ABI (`abi3t`, from Python 3.15).
- License terms for shipping Python and packages beside a program.
- The Component Model beyond its introduction page.
