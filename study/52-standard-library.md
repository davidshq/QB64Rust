# 52. A standard library for QB64Rust

Written 2026-11-16, split out of `study\50` at the user's request ("lets break it out as a separate doc"). It
answers the user's question of 2026-11-15: "any functionality in this library we might want to add to QB64Rust, or
perhaps to a standard library? Most modern languages have standard libraries right?"

It lists what QB64 programs need and the language does not give them (§2), and the ways to provide it (§3).
`study\50` keeps the four library collections themselves (§1–§3 there) and the better way to use libraries
(§6 there). It is a review, so it stays active until the user has answered §4 (`CLAUDE.md` working rule 7).
Nothing here is decided.

**Sources.**
- **The four collections**, as `study\50` §1–§3 describes them: the QB64-PE Libraries Pack, a740g's Toolbox64,
  grymmjack's QB64_GJ_LIB and Steve McNeill's QB64-Phoenix-Edition-Toolbox. All four are in the community corpus
  (`tests\community\repos.toml`).
- **QB64Rust's words**: the language reference (`docs\reference\`).
- **Other languages**: what Python, Go and Rust ship (§2) was checked on 2026-11-16 in their documentation
  (`CLAUDE.md` working rule 17): Python's [library index](https://docs.python.org/3/library/index.html) and
  [built-in types](https://docs.python.org/3/library/stdtypes.html), Go's
  [standard packages](https://pkg.go.dev/std), Rust's [`std`](https://doc.rust-lang.org/std/index.html) (no JSON
  module, and `std::time` has no calendar dates), [`serde_json`](https://docs.rs/serde_json/latest/serde_json/)
  and [`chrono`](https://docs.rs/chrono/latest/chrono/).

## 1. Why a standard library

Most modern languages ship more than the language: a standard library of modules for common work. QB64's
equivalent is its several hundred built-in words. They are strong on graphics, sound and files, and have the gaps
§2 marks. Three authors of the four collections wrote INI files, maps and string helpers separately, which shows
the same gaps.

## 2. What is missing, and where people get it today

| Need | Where it is today | Does QB64Rust have it? | Verdict |
|---|---|---|---|
| Run a command and capture its stdout, stderr and exit code | Pack: SpriggsySpriggs/PipeCom (Windows and POSIX) | No. `SHELL` cannot capture output, and on Windows it opens a console window (`CLAUDE.md` working rule 6 has the cost of that) | **Standard** |
| Lists, maps, sets, queues, stacks | Pack: a740g/QBDS. Toolbox64: `DS/HashTable` (over C++ `unordered_map`). GJ_LIB: `ARR`, `DICT` | No. Arrays and `REDIM _PRESERVE` only. `study\48` §4.2 plans lists and maps in the language | **Standard**, as a language feature |
| JSON parse and write | Pack: MattKilgore/QB64pe-Json | No | **Standard** |
| Dates and times: parts, formatting, timestamps | Pack: RhoSigma/QB-Time (`strftime`). Steve's Toolbox: Date Library | `DATE$`, `TIME$` and `TIMER` only | **Standard** |
| Formatting numbers and text | Pack: RhoSigma/QB-StdIo (`printf`). Toolbox64: `Core/String` | `PRINT USING`. `study\48` §4.2 plans string interpolation | **Standard**, as formatting |
| String helpers: split, join, find, replace, trim | GJ_LIB: `STRINGS`. Toolbox64: `Core/String` | Partly: `INSTR`, `MID$`, `_TRIM$` | **Standard** |
| Command-line arguments by name | Toolbox64: `CLI/Args` | `COMMAND$` and `_COMMANDCOUNT`, with no option parsing | **Standard** |
| File paths: name, folder, extension | Toolbox64: `FS/Pathname` | No | **Standard** |
| Read or write a whole file at once | Toolbox64: `IO/File` | No | **Standard** |
| Sorting | Toolbox64: `Algo/Sort` | No | **Standard** |
| SHA-256 | Pack: RhoSigma/Sha2-Hash | `_MD5$`, `_CRC32` and `_ADLER32`, but no SHA | **Standard** |
| INI files | Pack: FellippeHeitor/IniManager. Toolbox64: `Config/Ini` | No | **Standard**, maybe |
| A debug print of any value | GJ_LIB: `DUMP` | The debugger (M5), not a print | Maybe |
| UTF-8 and code pages | Pack: RhoSigma/Charsets | Partly: `_UPRINTSTRING` and `_UCHARPOS` | Maybe, with `study\48` §4.3's UTF-8 helpers |
| Tests | Pack: a740g/Catch. Toolbox64: `Debug/Test` | Yes: `qb64rust test` and `qbtest.bi` (change `basic-unit-tests`) | Covered |
| C `va_list` for C's `printf` family | Pack: RhoSigma/QB-StdArg, the base of QB-StdIo and QB-Time | Not needed if formatting is built in | Not by itself |
| Filled and thick shapes | Steve's Toolbox: Draw Library | `LINE`, `CIRCLE`, `PAINT` | Too specialized |
| Dialogs, menus, widgets, layout | Pack: TerryRitchie/MenuLib. GJ_LIB: `FILE_DIALOG`, `MSG_BOX`, `TEXT_INPUT`, `LAYOUT`. Toolbox64: `Graphics/GUI` | No | Too specialized |
| Image and media formats, effects | Pack: Imageprocess, AnimManager, the Tony3068 video libraries. GJ_LIB: Aseprite, PSD, `IMGADJ`. Toolbox64: MIDI, OPL3, GIF | Partly: `_LOADIMAGE`, `_SNDOPEN` | Too specialized |
| Platforms: Steam, pen pressure | GJ_LIB: `STEAM`, `PRESSURE_DEVICE` | No | Too specialized |

**What other languages ship** (checked, see the sources). Python has `subprocess`, `json`, `datetime`,
`hashlib`, `configparser`, `argparse`, `pathlib` and built-in lists and dicts. Go has `os/exec`, `encoding/json`,
`time`, `flag`, `path/filepath`, `sort` and `crypto/sha256`. Rust ships its collections, `std::process` and
`std::path`, and leaves JSON and dates to packages such as `serde_json` and `chrono`.

## 3. Three ways to provide a standard library

1. **Built-in words, behind the extensions switch** (`DECISIONS.md` 2026-11-11). This suits what needs runtime
   code: capturing a process's output, lists and maps. It costs runtime C++, a reference page and tests with
   expected outputs written by hand, since QB64pe has no such words to measure.
2. **A library of our own, written in BASIC**, installed beside the compiler under a reserved author name, for
   instance `$USELIBRARY:'qb64rust/json'`. This suits JSON, INI files, dates, strings, paths, arguments and
   sorting. Written to compile with QB64pe 4.7.0 too, a program that uses it stays portable. It is shipped with
   each release, so its version always matches the compiler's.
3. **Ship or fetch others' libraries.** `SOMEDAY.md` has a row for the extension to download a pinned release of
   the pack. All four collections are MIT, MIT-0 or public domain (Toolbox64's `external\` folder holds third-party
   code under its own licenses). `CLAUDE.md` working rule 3 names none of them, so taking other authors' code in
   is the user's call.

These can be combined: option 1 for lists, maps and process capture; option 2 for the rest; option 3 for what
others have and we do not write. Option 3 would use the dependencies per project of `study\50` §6.

## 4. Questions for the user

1. **The standard library:** should QB64Rust have one, and in which of §3's forms? The recommendation is all three
   in combination: built-in words for process capture, lists and maps; our own BASIC library for JSON, INI files,
   dates, strings, paths, arguments and sorting; others' libraries fetched for the rest.
2. **Others' code:** may QB64Rust ship or fetch libraries of these collections (MIT, MIT-0 and public domain), or
   should everything in a standard library be written fresh? `CLAUDE.md` working rule 3 covers none of them.

## 5. The panel's view

- **Pragmatic engineer:** start with process capture. It is the gap people hit first, and it changes no syntax.
- **QB64 engineer:** our own BASIC library should compile with QB64pe too.
- **Compiler/languages engineer:** lists and maps belong in the language, where the checker knows their element
  types (`study\48`).
- **Rust engineer:** Toolbox64's `HashTable` shows the shape of built-in maps: a thin layer over C++'s
  `unordered_map`.
- **Test engineer:** QB64pe is no oracle for the new parts, so each needs expected outputs written by hand.
- **A newcomer from Python or JavaScript**, added because a standard library is mostly for them: JSON, maps,
  string helpers and process capture are what they look for in the first week.

**Recommendation:** answer §4. Then add one `SOMEDAY.md` row for each standard item of §2 (process capture first,
among the extension words).
