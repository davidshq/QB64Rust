# 50. QB64 library collections, and a better way to use libraries

Written 2026-11-15, in answer to the user's questions:
- "Is there anything in here we should consider implementing in QB64Rust?", about Steve McNeill's fork of the
  QB64-PE Libraries Pack;
- "any functionality in this library we might want to add to QB64Rust, or perhaps to a standard library?";
- "we'll want to implement a better way to do libraries than `$USELIBRARY`";
- "Take a look at these as well": Steve McNeill's QB64-Phoenix-Edition-Toolbox, grymmjack's QB64_GJ_LIB, the
  official Libraries Pack and a740g's Toolbox64.

It is a review, so it stays active until the user has answered §7 (`CLAUDE.md` working rule 7). Nothing here is
decided. The standard library (§4, §5 and questions 1–2) moved to `study\52` on 2026-11-16.

**Sources.**
- **The four collections**, read through the GitHub API on 2026-11-15:
  - the Libraries Pack: `QB64-Phoenix-Edition/QB64-PE-LibrariesPack` and its fork `SteveMcNeill/QB64-PE-LibrariesPack`;
  - `a740g/Toolbox64`;
  - `grymmjack/QB64_GJ_LIB`, also cloned at its latest commit `2086607` (2026-11-14);
  - `SteveMcNeill/QB64-Phoenix-Edition-Toolbox`.
- **Our results**: the community clones at their pinned commits (`python tools\community\community.py fetch`) and
  `target\community\results.json` of the community run of 2026-11-13.
- **The old compiler**: its library lookup in `..\QB64pe\source\qb64pe.bas` (lines 4488–5204), and `qb64pe.exe`
  at `16f629784e` run with `-z` on the programs §3 names, one at a time on the host.
- **Other languages**: what Python, Go, Rust and JavaScript do comes from Claude's general knowledge, as in
  `study\48`.

## 1. The Libraries Pack

The pack is an add-on of QB64pe 4.3.0 and later: a `libraries\` folder that the user drops into QB64pe's own
folder. A program uses one of its libraries with one line, `$USELIBRARY:'author/library'`. The compiler then reads
the descriptor `libraries\descriptors\author\library.ini` and includes the files it names from
`libraries\includes\author\library\`, in three places:
- `IncAtTop` before the program's first line;
- `IncAfterMain` after the main module's last line;
- `IncAtBottom` after everything else.

**Why three places.** The main module runs from its first line. So a library's main-module code that is only
reached by a jump has to come after the main program: `DATA`, error handlers and `GOSUB` routines. SUBs and
FUNCTIONs do not need the bottom place. QB64pe 4.7.0 accepts a FUNCTION before the main code, and so does
QB64Rust: a scratch program with `FUNCTION Date.Twice&` on its second line printed `twice: 42` built by both
(2026-11-15). The pack's rule that a `.bi` holds no SUBs is the pack's convention. Toolbox64 and Steve McNeill's
Toolbox both put SUBs in files included at the top.

The pack also has a browser for its libraries, `LibraryExplorer.bas`, built with InForm.

**Versions.** The official pack's latest release is v1.5.0 (2026-11-08), at commit `fd10ed46`. That is the commit
`tests\community\repos.toml` pins, so the community run already covers all of it. Steve McNeill's fork is 26
commits behind the official pack and one ahead. The commit it is ahead by adds his "Keyhit Library" (Windows only,
version 1.0 of 2026-01-15).

**Licenses.** Every one of the 26 descriptors names MIT, MIT-0 or public domain (the Unlicense). None names the GPL
or LGPL.

## 2. What QB64Rust already does with the pack

QB64Rust has everything the pack relies on: `$USELIBRARY` (the `v58` family), `$INCLUDEONCE`, `ON ERROR GOTO
_NEWHANDLER` and `_LASTHANDLER` (the `v50` family), and `$IF VERSION < 4.3.0 THEN $ERROR`.

**One bug, fixed on 2026-11-15.** A `DECLARE LIBRARY` header that stands beside a `$USELIBRARY` file was found only
when the compiler was started from the library root.
- The old compiler looks for a header in the folder of the file that holds the block (`qb64pe.bas` lines
  4906–4926). QB64Rust has the same fallback (`crates\driver\src\library.rs`).
- The bug: QB64Rust took that folder from the file's name in the source map. A `$USELIBRARY` file is named there by
  its path under the root (`libraries/includes/RhoSigma/Sha2-Hash/sha2.bi`), and that path was resolved against the
  working directory.
- The fix: the loader now gives the real folder of each file it read (`Loader::folders`, `Frontend.folders`).
- The new program `verification\v58_u27_header_beside` measures it. It uses the test library `test/header` of
  `v58_root`, whose `hd_beside.h` stands beside its top file.
- 9 programs of the pack compile since the fix and are on `tests\community\pass.list`:
  - RhoSigma's QB-StdArg (2 programs), QB-StdIo, QB-Time and Sha2-Hash examples;
  - `CheckAll.bas`;
  - Tony3068's VLC and VID examples (`SimplePlay.bas`, `VLCpromo.bas`, `VideoPlay.bas`), whose `qbvlc.h` stands
    beside `VLC-HANDLES.BI`.

**Now 78 of the pack's 79 programs compile** (community run of 2026-11-16 in the suites VM). The one left is not a
program: `libraries\includes\a740g\Catch\Catch.bas` is the `IncAfterMain` part of a library. Compiled alone it
fails with "cannot store a string in a number variable" at line 12. `Demo_05_moving_over_fullscreen_animated_background.bas`
timed out at 120 s in the run of 2026-11-13, compiled alone in 2 s, and passed in the run of 2026-11-16.

**The fork's Keyhit Library** is not worth adding to the corpus. Its example cannot compile with either compiler:
it includes `Keyboard Library.bi` with `$INCLUDE`, but that file is in another folder. The library works around key
presses that QB64pe's window (GLUT) does not report. If QB64Rust's runtime misses them too, the fix belongs in the
runtime. That has not been measured.

## 3. Three other collections

None of the three uses `$USELIBRARY`. Each is a folder of include files that a program names with `$INCLUDE`.

### 3.1 a740g's Toolbox64

- **What it is**: 73 `.bi`, `.bas` and `.h` files in 14 folders, besides `external\`, by Samuel Gomes, who also wrote the pack's QBDS and Catch.
  MIT license. Latest commit `32f2b70` (2026-10-25), which is the commit `tests\community\repos.toml` pins.
- **How it is used**: as a git submodule named `include`. A program names `'$INCLUDE:'include/DS/HashTable.bi'`
  at the top and the matching `.bas` at the bottom. Every file has an include guard.
- **What is in it** (the general-purpose parts):
  - `CLI/Args.bi`: command-line arguments by name (`Args_GetArgumentName$`, `Args_GetArgumentIndex&`);
  - `Config/Ini.bi`: INI files, a revision of Fellippe Heitor's INI Manager;
  - `Core/String.bi`: 51 string routines, including formatting of numbers and booleans;
  - `DS/HashTable.bi`: a hash map that wraps C++17's `unordered_map` through a header (`HashTable.h`);
  - `DS/MemFile.bi`, `DS/StringFile.bi`: file-like reading and writing of memory;
  - `FS/Pathname.bi`: path parts (file name, folder, extension, separators);
  - `IO/File.bi`: load and save a whole file, also from a URL (`File_LoadFromURL$`);
  - `Algo/Sort.bi`: QuickSort of string arrays;
  - `Math/Math.bi`: 65 routines (random numbers, NaN tests, clamping), and 2D vectors;
  - `Debug/Test.bi`: the test framework the pack ships as Catch.
  - Also audio (a MIDI player, an OPL3 synthesizer), graphics (fonts, a small GUI, ANSI) and GIF playback, with
    third-party C and C++ under `external\` (Catch2, RtMidi, stb, microui, nuklear, tiny-AES).
- **Our results**: 5 of its 9 programs compile. Of the other 4:
  - `Audio/MIDIPlayer.bas` fails with QB64pe too ("File ../IO/File.bas not found").
  - **`DS/HashTable.bas`, `tests/test_basic.bas` and `tests/test_opl3x.bas` are a QB64Rust bug.** QB64pe accepts
    all three (`-z`, 2026-11-15). Each holds a `DECLARE LIBRARY` block that declares a SUB and a FUNCTION of the
    same name: `SUB HashTable_Remove` and `FUNCTION HashTable_Remove%%` in `DS/HashTable.bi` lines 17–18, and
    `OPL3X_WriteChipRegister` in `Audio/OPL3X.bi` lines 16 and 18. QB64Rust rejects it, with an error that depends
    on the order. With the SUB first: "name already in use: `HashTable_Remove%%`". With the FUNCTION first, a call
    as a statement: "the library FUNCTION `OPL3X_WriteChipRegister` cannot be called as a statement". The bug
    blocks 12 programs of the corpus, counting the copies of Toolbox64 in the projects below (`GIFPlay.bas` of
    `mplayer64` among them). It is listed in `STATUS.md`'s known bugs.
- **Four other projects of the corpus use it, each at another commit**: `vga-font-editor` at `7133ae5`,
  `spaceshooter2k` at `04f1e36`, `particle-life` at `04eeaa9` and `mplayer64` at `21e26d0`. A submodule is how a
  QB64 project pins a library today.

### 3.2 grymmjack's QB64_GJ_LIB

- **What it is**: 26 libraries in its README's table, 394 files, for QB64pe 4 and later. MIT license. Its latest commit is `2086607`
  (2026-11-14), and the author commits often.
- **How it is used**: clone it and include `_GJ_LIB.BI` at the top and `_GJ_LIB.BM` at the bottom for everything,
  or one library's `.BI` and `.BM`. `$IF` guards keep each library usable alone. Names start with `GJ_` or the
  library's name, "to not clobber anything", as its README says.
- **What is in it**, in the areas of `study\52` §2: `ARR` (arrays of every type), `DICT` (a dictionary of `.key` and `.val`
  pairs in an array), `STRINGS` ("batteries" for finding, parsing and arrays), `DUMP` (a debug print of a value, in
  the manner of PHP's `print_r`), `CONSOLE`, `SYS`, `VECT2D`. The rest is mostly for graphics programs and
  editors: ANSI text, Aseprite and Photoshop files, color pickers, file and message dialogs, text input, a flexbox
  layout solver, GLSL shaders, pen pressure, and Steam achievements.
- **Our results**: it is in the corpus only as the submodule `includes/QB64_GJ_LIB` of grymmjack's DRAW, at commit
  `2651577`. 39 of its 128 programs compile there. Two errors account for most of the rest:
  - 27 programs stop at `IMGADJ.BI` line 83, `DECLARE FUNCTION GJ_IMGADJ_CloneImage (sourceImg AS LONG) AS LONG`.
    QB64Rust says "a FUNCTION's type is given by a suffix on its name, not by `AS`". **QB64pe gets past that line**
    and stops later, at `IMGADJ.BM` line 634, so this is a divergence. The `SOMEDAY.md` row of group `community`
    named it "not measured"; it is half measured now. At the latest commit the line is still there.
  - `QB64_GJ_SAFE_FREE_IMAGE` "is no statement and no SUB", in 5 programs. QB64pe fails at the same lines with a
    syntax error, both at DRAW's commit and at the latest one: the examples do not include `_GJ_LIB_COMMON.BI`,
    which defines the SUB. Not the compiler's.
- **At its latest commit, as a corpus entry of its own** (`qb64_gj_lib`, added 2026-11-16, §7 question 5): 40 of
  its 129 programs compile, in the community run of 2026-11-16 in the suites VM. Every other error is QB64pe's too,
  or the `AS LONG` divergence above (27 programs):
  - 47 programs under `ASEPRITE\_\` include `ASEPRITE.BI`, which is in the folder above them. QB64pe: "File
    ASEPRITE.BI not found" (`chunk_debug.bas`, `-z`).
  - 4 test programs reach `TI__is_word_sep% = TRUE` in `TEXT_INPUT\TI-INPUT.BM` line 1148, where `TRUE` is not
    declared. QB64pe: "Variable 'TRUE' (SINGLE) not defined" (`CP-TEST.BAS`, `-z`).
  - The 5 programs of `QB64_GJ_SAFE_FREE_IMAGE` above, 5 that include `_GJ_LIB.BI` by a path that is not there,
    and `ARR\ARR_TEMPLATE.BAS`, a template with `{Q}` placeholders that `ARR\generate.sh` fills in.

### 3.3 Steve McNeill's QB64-Phoenix-Edition-Toolbox

- **What it is**: four libraries in single files, `Date Library.QLB`, `Draw Library.QLB`, `Input Devices.QLB` and
  `Keyboard Library.QLB`, with one Markdown page per routine. Its last commit is `ccf06a7` (2026-02-03). GitHub
  shows no license, but `Documentation\LICENSE.md` is MIT (2023, SteveMcNeill).
- **`.QLB` here is BASIC source**, not QuickBASIC's compiled Quick Library. Each file starts with
  `$INCLUDEONCE` and holds only SUBs and FUNCTIONs, and a program includes it at the top with `$INCLUDE`.
- **Its names look like namespaces**: `Date.Day$`, `Date.ToUniDate$`, `Draw.Circle.Fill`, `Draw.Line.Thick`. QB64
  allows a `.` inside a name, so `Date.` is a prefix, not a namespace, but it reads like one.
- **What is in it**: date parts, formatting and Unix timestamps; filled and thick shapes (circles, ellipses,
  rounded rectangles, triangles, quads); mouse buttons and joysticks; extended keyboard input. An older part lists
  `$LET` tags (`INCLUDE_ALL`, `INCLUDE_CONVERT_BASE64`, `INCLUDE_TIMESTAMP` and others) that turn single routines on,
  so that a program carries only what it uses.
- **Our results** (`qb64-phoenix-edition-toolbox`, added 2026-11-16, §7 question 5): 9 of its 37 programs compile,
  among them the demos of the Date, Draw and Input Devices libraries and the four programs of `Archives\BAS code`.
  The other 28 include files that are not in the repository: 24 old samples name `..\Library Files\Toolbox.BI`, and
  others `..\Sound.BI`, `Keyboard Library.Lib`, `..\Draw Library.BM` or `..\Screen Library.BM`. QB64pe fails the
  same way ("File ..\Draw Library.BM not found", `Draw Library Demo.bas`, `-z`). QB64Rust reports the first call of
  a missing SUB (`Draw.Circle.Fill`, line 5) where QB64pe reports the missing file (line 35); both reject it.

## 4. The libraries, and which fill a gap

Moved to `study\52` §2 on 2026-11-16, with the standard library it leads to.

## 5. Three ways to provide a standard library

Moved to `study\52` §3 on 2026-11-16.

## 6. A better way to use libraries than `$USELIBRARY`

### 6.1 How QB64 code shares libraries today

The four collections show four ways, none of them good:
- **`$USELIBRARY`** (the pack): libraries dropped into the compiler's own folder.
- **A git submodule** (Toolbox64): pinned per project, but each user needs git's submodule commands, and the four
  corpus projects that use Toolbox64 pin four different commits.
- **"Clone it and point at it"** (QB64_GJ_LIB): an `$INCLUDE` path to wherever the user put it. Nothing pins the
  version.
- **Copy the file** (Steve McNeill's Toolbox): one `.QLB` per library beside the program.

And three conventions stand in for what the language lacks:
- **Prefixes in place of namespaces**: `SampleLib_`, `GJ_`, `HashTable_`, `Date.`.
- **A `__` prefix in place of private names**: Toolbox64's `__Args_ExtractName$` and `__Ini_Commit` are meant
  for the library alone.
- **`$LET` tags or include guards in place of leaving unused code out**: Steve McNeill's `INCLUDE_TIMESTAMP` and
  the guards of Toolbox64 and QB64_GJ_LIB.

### 6.2 What is wrong with `$USELIBRARY`

- **One place for every library: the compiler's own folder.** A project cannot have libraries of its own, two
  projects cannot use two versions of a library, and an update of the pack overwrites what a user dropped in. The
  pack's README says as much and advises against adding other libraries.
- **No fetching, pinning or lock.** A library is installed by copying folders by hand. Nothing records which version
  a program was built with.
- **Only the pack's layout.** Toolbox64, QB64_GJ_LIB and Steve McNeill's Toolbox have no descriptors, so
  `$USELIBRARY` cannot use them.
- **A library is up to three files, by the pack's rules.** The author must put declarations, main-module code and
  SUBs in separate files (`samplelib.bi`, `.bas`, `.bm`), and the comments of `SampleLib` spell out what may go
  where. §1 shows that SUBs could go at the top; only the main-module part needs a place of its own.
- **One global namespace.** Every name a library declares is the program's too, so libraries prefix everything
  (§6.1). The pack keeps `CheckAll.bas` only to find clashes between its own libraries.
- **A library named twice is included twice** unless its files start with `$INCLUDEONCE` (`SOMEDAY.md`, group
  `uselibrary`, `v58_u04`, `u05`).
- **The version check is by hand.** Every file of the pack repeats `$IF VERSION < 4.3.0 THEN $ERROR`.
- **The descriptor is loosely read.** The `v58` family measured its quirks: `$USELIBRARYX` is accepted, a word after
  the name is taken, and a missing value includes nothing without an error.
- **No documentation link for the editor.** Each descriptor names a document (`FullDocs`), but nothing in the
  editor shows it; LibraryExplorer is a separate program.

### 6.3 What a better way needs

- **`$USELIBRARY` and `$INCLUDE` keep working exactly as now.** A program QB64pe 4.7.0 accepts behaves the same
  (`DECISIONS.md` 2026-11-11).
- **Libraries per project, declared and pinned.** For instance a `[dependencies]` table in `qb64rust.toml`, as
  `study\48` §4.5 and `study\43` (Tools item 5) suggest:

  ```toml
  [dependencies]
  pack = { git = "https://github.com/QB64-Phoenix-Edition/QB64-PE-LibrariesPack", rev = "fd10ed46" }
  toolbox64 = { git = "https://github.com/a740g/Toolbox64", rev = "32f2b70" }
  mylib = { path = "../shared/mylib" }
  ```

  A lock file records the exact commits. `qb64rust fetch` puts them in a cache, as `toolchain install` and
  `community.py fetch` already fetch pinned files. This replaces the submodule (Toolbox64) and the unpinned clone
  (QB64_GJ_LIB).
- **Two kinds of dependency, found by name:**
  - **A folder of include files**, the kind of Toolbox64, QB64_GJ_LIB and Steve McNeill's Toolbox.
    `'$INCLUDE:'toolbox64/DS/HashTable.bi'` finds the file in the dependency named `toolbox64`. These projects
    then build from a fresh checkout with no submodule step.
  - **A library root in the pack's layout** (`libraries\descriptors\…`, `libraries\includes\…`).
    `$USELIBRARY` looks in the project's dependencies first, then beside the compiler. Every library of the pack is
    then a dependency as it is, with no change by its authors.
- **Optional, behind the extensions switch: one file per library.** The compiler already reads the whole program,
  so it could place a single library file's parts itself: everything at the top except main-module code reached by
  a jump, which goes after the main module. Library authors would no longer split files by the pack's rules.
- **Optional, later: private names.** For instance a `PRIVATE` SUB or variable that the program cannot see, which
  is what Toolbox64's `__` prefix means by convention. Full namespaces would remove prefixes altogether, but
  `study\48` §4.3 names them the riskiest item, since name resolution holds most of QB64pe's measured behaviors.
- **Optional: unused SUBs left out of the build.** The lint `unused-procedure` already finds them. Leaving them out
  of the C++ would make Steve McNeill's `$LET` tags unnecessary and shorten builds of large libraries such as
  QB64_GJ_LIB.
- **The editor knows libraries.** Completion of `$USELIBRARY:'…'` names and of dependency paths, hover showing a
  library's `ShortDesc` and `FullDocs`, and a command to add a dependency: the work of LibraryExplorer, inside VS
  Code.
- **A version check the compiler makes.** A descriptor or manifest key for the lowest compiler version, checked
  once, instead of `$IF VERSION` in every file.

### 6.4 The order to build it in

1. **`[dependencies]` with both kinds, a lock file and `qb64rust fetch`.** No new syntax, no extensions switch. It
   touches only programs whose `qb64rust.toml` declares dependencies. It needs an OpenSpec change (working rule 11:
   a new mechanism, across the driver, the settings and the extension).
2. **The editor's part**: completion, hover and an "add library" command.
3. **One file per library**, behind the extensions switch, and unused SUBs left out.
4. **Private names**, after the namespaces question of `study\48` is answered.

## 7. Questions for the user

1. **The standard library:** moved to `study\52` §4, question 1 (2026-11-16).
2. **Others' code:** moved to `study\52` §4, question 2 (2026-11-16).
3. **Libraries:** build §6.4 step 1 (dependencies per project, pinned, of both kinds) as the next OpenSpec change in
   this area?
4. **The syntax:** keep `$USELIBRARY` and `$INCLUDE` as the ways to name a library, or add a new statement under
   the extensions switch (for instance `IMPORT json`)? The recommendation is to keep them, since a new word adds a
   second way to do one thing, and to revisit this with namespaces.
5. **The corpus:** add QB64_GJ_LIB at its latest commit and Steve McNeill's Toolbox as community repositories of
   their own? **Answered** (user, 2026-11-16: "Yes"; `DECISIONS-SMALL.md` 2026-11-16). Both are in, with 40 and 9
   programs on the pass list (§3.2, §3.3).

## 8. The panel's view

- **Pragmatic engineer:** start with dependencies per project and with process capture. These are the two gaps
  people hit first, and neither changes the language. Fix the SUB-and-FUNCTION bug of §3.1 first; it is small and
  blocks 12 programs.
- **QB64 engineer:** support both kinds of dependency, so that every library people already wrote works unchanged.
  Our own BASIC library should compile with QB64pe too.
- **Compiler/languages engineer:** lists and maps belong in the language, where the checker knows their element
  types (`study\48`). The `DECLARE FUNCTION … AS LONG` divergence of §3.2 needs a verification program before it is
  changed, since QB64pe may skip such lines rather than check them.
- **Rust engineer:** the fetch, the cache and the lock file reuse what `toolchain install` and
  `tools\community\community.py` already do. Toolbox64's `HashTable` shows the shape of built-in maps: a thin layer
  over C++'s `unordered_map`.
- **Test engineer:** QB64pe is no oracle for the new parts, so each needs expected outputs written by hand. The
  pack's 78 programs that compile, and the Toolbox64 projects of the corpus, are a ready regression test for the
  dependency mechanism: build them with their libraries as dependencies instead of submodules and include roots.
- **A package-manager engineer**, added because §6 designs one: pin by commit from the start, and add version
  ranges only when there is a registry to resolve them. Keep the lock file next to `qb64rust.toml` and commit it.
  Four projects pinning four commits of Toolbox64 is normal; the lock file makes it visible.
- **A newcomer from Python or JavaScript**: copying folders into the compiler's install folder, or learning git
  submodules, is a surprise; `npm install` and `pip install` taught them otherwise.

**Recommendation:** answer §7. Then propose §6.4 step 1 as an OpenSpec change. The standard library's
recommendation is in `study\52` §5.
