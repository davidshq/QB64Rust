# 44. QB64 projects on GitHub: programs to test with, ideas to take, projects to watch

Written 2026-11-09, at the user's request: search GitHub for QB64 projects, and list the ones that are useful.
"Useful" means one of two things. A project may have functionality to build into QB64Rust or to offer as a
library or extension. Or it may be a program that makes a good test, as Civil War Strategy does. Nothing here is
decided. The recommendations and the panel's view are in the last section.

**Sources.** The searches were made with `gh search repos`. Each candidate's README, file tree, size and license
came from the GitHub API. Twelve of them were fetched as shallow sparse clones holding their BASIC sources and
include files, and for some of them their other small files. The clones went under `target\scratch\` (git-ignored) and were not kept. The release build of
`qb64rust` 0.3.0 on branch `after-pr26` checked them, on 2026-11-09. It used `--report unsupported` and `-z`,
which parse and check and write C++ but build and run nothing. So every "passes" below means "passes the front
end". No program was built or run, and no result here was compared with `qb64pe.exe`. Each section names the
commit of each clone that it used.

## Summary

The shortest useful list, in order of value:

| Project | Why | Section |
|---|---|---|
| `grymmjack/DRAW` | The largest QB64pe program found: 9 MB of BASIC in 390 files, MIT, changed daily. 68 of its 72 programs pass; the main program `DRAW.BAS` fails on two constructs QB64Rust does not support yet | 1, 2 |
| `jleonard2099/LHG_*` | Four Lance Haffner sports games (baseball, football, two kinds of basketball), DOS-era, ported to QB64pe with the author's permission. 1.5 to 2.7 MB of BASIC each, AGPL-3.0. Baseball's main program, 24,431 lines, passes | 1 |
| `a740g/QB64-PE-Museum` | 839 BASIC files by about 40 authors, each tested with QB64pe. 458 of its 519 programs pass | 1, 2 |
| `QB64-Phoenix-Edition/QB64-PE-LibrariesPack` | The library collection that QB64pe 4.3's `$USELIBRARY` was made for. 47 of its 71 examples pass; 16 more fail only on a `SUB` and a `FUNCTION` of the same name | 1, 2, 3 |
| `QB64-Phoenix-Edition/InForm-PE` | The GUI designer for QB64pe, MIT. All 42 examples fail on the same `SUB`/`FUNCTION` name rule | 1, 2 |
| `VB6Hobbyst7/HamCalc64` | 448 ham-radio calculator programs in GW-BASIC style, linked by `CHAIN` and `COMMON`, MIT. All 467 files pass | 1 |
| Gene Buckle's Crescent Software releases (`geneb/*`) | QuickPak Professional, QuickPak Scientific, XREF, QuickScreen and others: QB 4.5 and PDS 7.1 code, public domain. They show what a QuickPak compatibility library and reading `.MAK` projects would need | 1, 3 |
| `logiclrd/QBX` | A C# reimplementation of PDS 7.1 with 101 small behavior tests that our PDS 7.1 oracle can judge, LGPL-3.0 | 1, 4 |
| `mark-iid/qb45detok` | Reads QB 4.5's binary "fast load" `.BAS` files (they start with byte `0xFC`), Apache-2.0. QB64pe cannot read these files | 3 |

The front-end check found **two likely false errors and several constructs not yet supported** that QB64pe
programs use (section 2). Both likely false errors were read from the reference `qb64pe.bas`, not measured.
First, QB64Rust rejects a `SUB` and a `FUNCTION` of the same name, which `qb64pe.bas` allows. That blocks every
InForm program and 16 Libraries Pack examples. Second, QB64Rust does not take a line-continuation `_` that is
joined to the word before it (`or_` at the end of a line), which `qb64pe.bas` does.

## How the search was done

- `gh search repos` for `qb64` (200 results by stars), `qb64pe`, `qb64-pe`, `qb64 phoenix`, `qbasic`,
  `quickbasic`, `qbasic game`, `quickbasic game`, `gw-basic`, `qb45`, `visual basic for dos`, and for tools:
  `qb64 language server`, `lsp`, `linter`, `formatter`, `unit test`, `tree-sitter`, `debugger`, `wasm`, and
  `qbasic interpreter`/`compiler`. Also the GitHub topics `qb64`, `quickbasic` and `qbasic`, and the language
  "QuickBASIC" (150 results by stars).
- About 450 repositories were listed. About 70 were opened (README, tree, license), 12 were fetched and checked.
- Most of the 400-odd results are small: a few programs from someone's school days, a tutorial, a fork. The
  language search also returns many repositories that are not QuickBASIC at all. GitHub labels PicoCalc MMBasic
  code "QuickBASIC", and a search for `qbx` returns FiveM game-server scripts.
- No search for QB64 language servers, linters, formatters, test frameworks or tree-sitter grammars found a
  dedicated project. The only editor tooling is the VS Code extensions and one Zed extension (section 3).

## 1. Programs to test with

The corpus says "the same as QB64pe". It does not yet say "programs people wrote work" for anything but Civil War
Strategy (`SOMEDAY.md`, the `real-programs` rows). These are the candidates for that, grouped by kind.

### 1.1 Whole programs, like Civil War Strategy

| Project | What it is | BASIC source | License | Front end today |
|---|---|---|---|---|
| `jleonard2099/LHG_Baseball`, `LHG_Football`, `LHG_ProBB`, `LHG_CollegeBB` | Text-mode sports simulations from the 1980s, sold by mail order. Ported from DOS to QB64pe "with permission from Lance Haffner himself"; Crescent's QuickPak calls were rewritten in BASIC (`QPProEqu.bm`). Active (pushed 2026-11-09) | Baseball 2.0 MB in 71 files, its `HELLO.BAS` 747 KB and 24,431 lines; Football 2.7 MB; Pro basketball 1.5 MB; College basketball 1.9 MB | AGPL-3.0 | Baseball (`ae74a02`): `src\HELLO.BAS`, the whole game in one program, passes. Most other files are modules it includes and fail alone, as expected |
| `grymmjack/DRAW` | A pixel-art editor with layers, tools and image import and export. Includes the author's `QB64_GJ_LIB` as a git submodule | 9.1 MB in 390 files; the largest files `GUI\IMAGE-ADJ.BM` (584 KB), `GUI\COMMAND.BM` (330 KB) | MIT | `05770d7` with the library at the pinned `2651577`: 68 of 72 programs pass. `DRAW.BAS` fails (section 2, items 2 and 3) |
| `RZulu54/ChessbrainQB64` | A chess engine of about 2700 Elo, ported from VB.NET in 2026. A GUI mode and a UCI mode (`-uci`) that reads moves from standard input | 498 KB in 11 files | MIT | `e746764`: passes |
| `VB6Hobbyst7/HamCalc64` | George Murphy's HamCalc (VE3ERP): 448 engineering programs for radio amateurs, a menu that `CHAIN`s between them, data in `COMMON`. Line numbers, GW-BASIC style | 3.5 MB in 467 files | MIT (the port); HamCalc itself was freeware | `3d7b818`: all 467 pass |
| `QB64-Phoenix-Edition/InForm-PE` | The InForm GUI engine and its form designer, kept by the QB64pe team. 42 example programs and `tests\test_main.bas` | 2.0 MB in 83 files | MIT | `f353022`: 0 of 42 examples pass, all for section 2 item 1 |
| `geneb/XREF` | Don Malin's cross-reference program for BASIC sources (Crescent, 1989 to 1991). A real QB 4.5 application with menus and dialogs, in nine modules | 355 KB in 18 files | Public domain (statement in README) | `9155350`: 0 of 13 pass. It is a multi-module program that needs QuickPak Professional's `PRO.LIB` (section 3.2) |
| `geneb/QuickPakScientific` | Crescent's numerical library (1988 to 1993): matrices, eigenvalues, equations and more, with demos. Almost all BASIC: 204 `.BAS` files and one `.OBJ` | 743 KB in 204 files | Public domain | `e940eb0`: 102 pass. Nearly all of the other 102 are routines meant to be linked with a demo by one of the 60 `.MAK` files, so alone they fail with "no SUB of this name"; 3 use `DEF FN`, which QB64pe lacks |
| `RobertSundling/ADVGEN` | A text-adventure compiler and interpreter written in QuickBASIC in 1991 | 81 KB in 3 files | MIT | `8dee9b4`: fails on `FRE`, which QB64pe does not implement, on its second module `DRAWBOX.BAS`, and on routines of a library it does not include (`Busq`, `Exist`) |
| `bafetk/final-quest-2` | A QBasic RPG with a level editor and its own scripting language; needs `FQ2.BAS` and `FQ2MOD2.BAS` loaded as two modules | 172 KB in 6 files | MIT | Not checked |
| `fgr101/QB_WIld` | A text RPG in QBasic | 145 KB in 2 files | GPL-3.0 | Not checked |
| `Quest-Master/QuestForLubok` | A text adventure in QBasic | | GPL-3.0 | Not checked |
| `alvarogonzalezferrer/ktank` | A tank artillery game for up to 10 players, DOS (QBasic) and Win64 (QB64) | 29 KB, 1 main file | Not stated in the API (NOASSERTION) | Not checked |

Notes on these:

- **The Lance Haffner games are the closest match to Civil War Strategy.** They are commercial DOS games of the
  same era. Their interface is text of 80 columns by 30 rows (the constants `LAST_COL` and `LAST_ROW`), drawn on
  a QB64 image (`SCREEN menu&`). They read and write many data files: teams, schedules, leaders and statistics.
  They also use QB64pe 4.x additions: `_IIF` (19 uses in `HELLO.BAS`), `_ANDALSO` (10), `_CLAMP`, `$COLOR:32`,
  `_SAVEFILEDIALOG` and `_PRINTMODE` (30). A game is played from menus and typed numbers, so a keys file can
  drive it. The open questions are its file dialogs (`_SAVEFILEDIALOG`, and on Linux `zenity` and similar) and
  `RND`. AGPL-3.0 allows copying the sources into this repo as test data, kept in a folder with its license and
  an entry in `tools\licenses\inventory.toml`. Whether the project wants AGPL files in an MIT repo is a choice
  for the user (section 6).
- **DRAW is the size test.** It has more BASIC than any other program found. Civil War Strategy's copy in
  `tests\programs\civil-war-strategy` is one file of 160,698 bytes; DRAW is about 56 times that, and the
  Baseball game about 12 times. It is MIT, so it can be copied, but it changes every day. Pinning a
  commit is better than following it. It has its own visual test scripts (`QA\`, 387 files), from which
  `grymmjack/qa-harness` grew (section 3.7).
- **ChessbrainQB64 could be a deterministic benchmark.** In UCI mode the engine reads commands such as
  `position` and `go depth N` from standard input and prints its best move. At a fixed depth the answer should
  not change between runs, if the search uses no clock and no random numbers there; that is still to be checked.
  Then it is a console test with a `.stdin` file, and a measure of how fast the generated code runs.
- **HamCalc64's calculators are console tests.** Each one asks for numbers with `INPUT` and prints results, so a
  `.stdin` file drives it. Its menu is a heavy test of `CHAIN`. `CHAIN` to a program that exists runs
  `qb64pe.exe -c` beside the program (`DECISIONS.md` 2026-10-13), so the menu needs care, but the calculators
  can run one by one. The README says the port is "mostly" converted and the menus need fixed folder names.
- **OpenStrut was left out.** `gmunaro/openstrut-v1` (GPL-3.0) is SISTRUT 4.0, a 1991 QB 4.5 suite for
  reinforced-concrete design: 28 programs, 7.8 MB of BASIC, with regression tests. It was rebuilt by
  disassembling the original executables, "with no link to the authors of the original", and its license checks
  were removed. That provenance is too doubtful to test with.
- **`hsehestedt/WIM-Tools` was left out** (727 KB in one file, no license). It edits Windows images through DISM
  as an administrator. A front-end check would be safe, but no run would be.

### 1.2 Collections

| Collection | What it holds | Size | License | Front end today |
|---|---|---|---|---|
| `a740g/QB64-PE-Museum` | Programs from GW-BASIC, QBasic, QB 4.5, PDS 7.1, VB for DOS and QB64, "tested and adapted for compatibility with QB64-PE", one folder per author (Terry Ritchie, RhoSigma, Fellippe Heitor, Bplus, Microsoft, and about 35 others) | 839 BASIC files, 25.8 MB of BASIC | None for the collection; each program its author's | `34f800f`, without its images, sounds and data files: 519 programs, 458 pass, 17 fail only on "not supported yet", 44 on other errors |
| `QB64-Phoenix-Edition/QB64-PE-LibrariesPack` | 26 libraries by 9 authors: data structures (`a740g/QBDS-*`), a test framework (`a740g/Catch`), JSON (`MattKilgore/QB64pe-Json`), INI files, SHA-2, image processing, animation formats, a menu library; with 71 examples and the `LibraryExplorer` | 1.7 MB of BASIC | Each library its own (NOASSERTION for the pack) | `fd10ed4`, with the pack as `--include-root`: 47 of 71 pass |
| `QB64-Phoenix-Edition/Resources` and `QB64-Archives` | The QB64pe team's archive: the samples that QB64 once shipped, "Old QB45 and QB64 Samples" (148 BASIC files from qb45.com and Pete's QB site), sounds and fonts. The Museum overlaps much of it | 15.1 MB of BASIC; the repositories are 0.25 and 1 GB | MIT for the repositories; contents by many authors | Not checked |
| `qb40/abc-archive` | William Yu's All Basic Code archive, 1995 to 2003: packets of programs sent in by users | About 2,500 programs (196,482 bytes of index at 78 bytes an entry); 1,685 parsed by a quick script, 1,083 marked "QB, QBasic, PDS" and 162 "QB, PDS", about 350,000 lines | "Do not charge or bundle these files in commercial products"; each program its author's | Not checked. The packets need extracting: entries are separated by `CHR$(227)` with a fixed-width header (author, title, date, dialect, line count) |
| `qb40/all` | DOS-era QuickBASIC programs and utilities | 85 BASIC files, 545 KB | None | Not checked |
| `robhagemans/hoard-of-gwbasic` | GW-BASIC programs collected from old web sites and archives | 25 MB repository | None | Not checked. QB64pe accepts much GW-BASIC code; PC-BASIC (section 4) is the reference for what it should do |
| `InsaneJetman/classic-qbasic`, `pmachapman/basic-samples` | Microsoft's QBasic and QuickBASIC samples (`GORILLA.BAS`, `NIBBLES.BAS`, `MONEY.BAS`, ...) | Small | Microsoft's | Covered by the `SOMEDAY.md` row on Microsoft's samples: run from a local copy, keep only recordings |

Notes:

- **The Museum is the best ready-made set.** Every program was made to build with QB64pe by its curator, so a
  failure is more likely ours than the program's. It has no license, so it is run from a local clone pinned by
  commit and only the recordings are kept here. That is what the `SOMEDAY.md` row on `tests\qbasic_testcases`
  proposes for those 143 programs. Some of its programs are fan remakes of commercial games (Dragon Warrior,
  Galaga, Frostbite). Run inside the set they are just more test input; none should be singled out or copied.
- **The Libraries Pack is the set for `$USELIBRARY`.** QB64Rust supports `$USELIBRARY` (spec
  `language/external-libraries`, tests in `tests\frontend\uselibrary_*.bas` with a made-up pack). The real pack
  has never been checked. With `--include-root` at the pack's folder its descriptors are found, and two thirds of
  its examples pass.
- **The ABC archive is big but needs work.** Its programs are short pieces of code, not whole games, and many
  need assembly or a particular sound card. It is a later step than the Museum.

### 1.3 The repositories the user named (2026-11-09)

After reading the first version of this study, the user named 28 repositories that looked interesting. Twelve
are already above or in section 5: DRAW, InForm-PE, the Museum, `QB64_GJ_LIB`, `ktank`, `QuestForLubok`,
`raylib-64`, `QB64GuiTools`, `qub`, `vWATCH64`, `QB64-interpreter` and GX. `FellippeHeitor/InForm` is now the
same project as InForm-PE (its README describes InForm-PE), so only InForm-PE is checked. The others, by the
GitHub API (not compiled yet):

| Project | What it is | BASIC source | License | For |
|---|---|---|---|---|
| `boxgaming/gx` | The GX 2D game engine (scenes, sprites, tile maps, an editor) for QB64pe and QBJS. Its `tools\qb2js.bas` (143 KB) is QBJS's BASIC-to-JavaScript compiler written in QB64 | 452 KB in 32 files | MIT | A large QB64pe program (the compiler), and a library test |
| `FellippeHeitor/QB64-interpreter` | A QB64 interpreter written in QB64 (`basiccmdline.bas`, 91 KB) | 128 KB in 13 files | MIT | String handling and parsing, a console program |
| `oonap0oo/QB64-projects` | Graphics programs: a Mandelbrot viewer, clocks, simulations | 235 KB in 67 files | None | Screen tests, run from a clone |
| `loopy750/SRT-Stats-Monitor`, `NGINX-Stats-Monitor` | Programs that watch a streaming server over HTTP and switch scenes in OBS Studio; in daily use by streamers | SRT 377 KB in one file (plus `BIN2INCLUDE.bas`, 609 KB); NGINX 85 KB | GPL-3.0 | Front end only on the host: they open network connections, so any run goes to the suites VM with a `network:` sidecar (`CLAUDE.md` working rule 6) |
| `a740g/VGA-Font-Editor` | A VGA ROM font editor and font library | 48 KB in 2 files | MIT | A GUI program |
| `a740g/QB64-MOD-Player` | A MOD player written in BASIC, with a library (`MODPlayer.bi`, 114 KB) | 278 KB in 17 files | MIT | Sound: decodes in BASIC and plays through `_SNDRAW` |
| `AshishKingdom/p5js.bas` | A port of the p5.js drawing library to QB64, with 53 sample sketches | 152 KB in 54 files | MIT | A library and many small window programs. The Museum's copy has `Blobby.bas` with `END SUB` closing a FUNCTION |
| `AshishKingdom/QB64_OpenGL_Demos` | OpenGL demos through `SUB _GL` | 279 KB in 37 files | None | Tests of the OpenGL spec, run from a clone |
| `FellippeHeitor/INI-Manager` | Reading and writing INI files | 36 KB in 6 files | MIT | Also in the Libraries Pack |
| `FellippeHeitor/tui` | A text-mode user interface library in one file | 136 KB | MIT | A library |
| `aouwt/qb-wm` | A window manager drawn in a QB64 window | 22 KB in 1 file | None | In the Museum (section 2 item 8) |
| `AshishKingdom/GUI-Menu-Bar` | Windows-style menus for QB64 | 14 KB in 3 files | None | In the Museum |
| `BPOutlaws/QB64-Platformer-Test` | A platform game test with sprite sheets by itch.io artists | 29 KB in 6 files | None | A small game, run from a clone |
| `a740g/Alien-Alley` | A source port of the 1994 shooter from Dave Roberts' book *PC Game Programming Explorer*, by way of FreeBASIC | 47 KB in 1 file | MIT | A whole game |
| `w84death/qbasic` | Five small QBasic games and demos | 15 KB in 5 files | MIT | Small tests |
| `RetroNick2020/raster-master` | A sprite, map and sound editor written in Pascal that writes source code for many BASICs and other languages | Pascal; its `Examples\qb64` (Sokoban, raylib, RGBA demos) and `Examples\qbasic` folders hold BASIC that it generated | MIT | Its generated QB64 and QBasic programs are tests; the editor itself is not BASIC |

All of these go into the set for the front-end check (section 6, recommendation 3), the ones without a license
from a clone like the Museum.

### 1.5 The second list the user named (2026-11-09)

The user named 31 more. Three are already above: the Libraries Pack, `raylib-64` and `Toolbox64`.
`RhoSigma-QB64/QB64Library` (407 KB in 51 files), Haggarman's
`Software-3D-Perspective-Correct-Textured-Triangles` (1.9 MB in 24 files, the largest here) and
`mechatronic3000/fzxNGN` (a 2D physics engine, 271 KB in 35 files) were seen in the survey and are in the Museum
too; they go into the set from their own repositories, which may be newer. Two are left out:
`Nivet2006/Dictionary` publishes no BASIC source (a license, a README and a word list), and
`cirillistefano/Basic_code` holds one 1.7 KB file. The others, by the GitHub API (not compiled yet):

| Project | What it is | BASIC source | License | For |
|---|---|---|---|---|
| `grymmjack/QB64-samples` | A curated collection of QB64 samples, overlapping the Phoenix team's `Resources` | 22.3 MB in 1,104 files | None | The second-largest set after the Museum; from a clone. Many files are the same as `Resources`', so a later step could drop duplicates by hash |
| `a740g/SpaceShooter2K` | A source port of Space Shooter 2000, a Visual Basic 6 sample of Microsoft's DirectX 7 SDK | 196 KB in 1 file | MIT | A whole game in one file |
| `FellippeHeitor/Pendulum` | A game drawing on "Fire Rides" | 250 KB in 2 files | LGPL-3.0 | A whole game; also in the Museum |
| `AshishKingdom/Hunter-Revenge` | A shooting game, with Terry Ritchie's sprite library | 342 KB in 4 files | GPL-3.0 | A whole game; also in the Museum |
| `loudar/Vedit` | An image editor, with the `um` menu library | 274 KB in 12 files | None | A GUI program, from a clone |
| `AshishKingdom/3D-Grapher` | Plots `z = f(x, y)` with OpenGL; includes the `sxript` expression evaluator (100 KB) | 161 KB in 5 files | GPL-3.0 | OpenGL; its copy in the Museum has section 2 item 8's "string member in parentheses" |
| `AshishKingdom/aKFrameWork` | A library of dialogs (buttons, text boxes, check boxes) | 108 KB in 7 files | None | The library behind 7 of the Museum's 10 "array with a constant's name" failures (section 2 item 8) |
| `gAndy50/Qb64Wrappers` | Bindings of raylib and other C libraries through `DECLARE LIBRARY` | 114 KB in 8 files | None | External libraries; from a clone |
| `a740g/Particle-Life` | A particle-life simulation with a small GUI library | 74 KB in 1 file | MIT | A window program |
| `GaryLuck/QB64` | LuckBasic, a BASIC interpreter, ported to QB64 | 73 KB in 3 files | None | An interpreter: string handling and parsing |
| `TonyVallad/MSZP` | A Mandelbrot and Julia set explorer that writes BMP files | 102 KB in 4 files | MIT | Graphics and binary files |
| `AshishKingdom/QB64-OpenGL-Engine`, `OpenGL-OBJ-Loader` | An OpenGL rendering engine and a Wavefront `.OBJ` viewer | 60 KB and 53 KB | GPL-3.0; none | OpenGL |
| `creamcast/3D-Library-for-QB64`, `zandgall/qb64-3d-renderer`, `idleherb/basic-raytracing` | Software 3D: textured triangles with `_MAPTRIANGLE`, a depth-tested renderer, a ray tracer | 25 KB, 34 KB, 19 KB | MIT; none; Unlicense | Graphics; the ray tracer is pure arithmetic, a good numeric test |
| `kc8rwr/Schemat` | A DOS QuickBASIC schematic editor, updated for QB64 | 29 KB in 1 file | None | A QB 4.5 program; QB 4.5 and PDS 7.1 oracles can run its original form |
| `FellippeHeitor/tcpip-experiments` | Two QB64 programs talking over a local TCP connection | 34 KB in 6 files | None | Network: front end on the host, any run only in the suites VM (`CLAUDE.md` working rule 6) |
| `jakebullet70/QB64-SQLite` | A wrapper of SQLite's DLL with examples | 15 KB in 4 files | None | `DECLARE DYNAMIC LIBRARY`; needs `sqlite3.dll` to build |
| `a740g/MPlayer64` | An audio player built on InForm-PE and Toolbox64 | 15 KB in 1 file | MIT | Needs InForm-PE and Toolbox64 as include folders (a manifest option) |
| `caglartoklu/instantarchive` | Backs up files and folders into time-stamped 7-Zip archives | 15 KB in 1 file | MIT | Front end only: it runs `7z` through `SHELL`, so it is never run on the host |
| `FellippeHeitor/Localizer` | Pulls the strings out of a QB64 program into an `$INCLUDE` for translation | 9 KB in 1 file | MIT | A small tool |
| `guillaC/chess` | A chess game | 5 KB in 1 file | None | Small |

All of these but the two left out join the set of section 1.3.

### 1.4 Behavior tests from another implementation

`logiclrd/QBX` is a reimplementation of QuickBASIC PDS 7.1 in C#, active (pushed 2026-10-23), LGPL-3.0. Its
`QBX.Tests\Integration\Tests` folder has 101 small programs. Each one tests one behavior and writes `PASS` to a
file when the result is right: `CIRCLE`, `PAINT`, `DRAW`, `POINT`, `VIEW`, `WINDOW`, `DATA`, `COMMON`, `CHAIN` with
`DEF SEG`, `RANDOM` files, `ON ERROR`, `SHARED`, `STATIC`, aliasing of arguments. They are small, cover graphics
that the verification families cover less, and say what PDS 7.1 does. The PDS 7.1 oracle can confirm each one and
the QB 4.5 oracle most of them, and QB64pe can be compared with both. Of all 327 `.BAS` files in the repository (`667acff`), 259 pass. 16
use `CALL INTERRUPTX` (a `SOMEDAY.md` row) and 5 use `ON ERROR RESUME NEXT`, which QB64Rust does not support.

## 2. What the check found in QB64Rust

These came from the front-end check only; none was compared with `qb64pe.exe`. Each needs a verification
program before it is fixed (`CLAUDE.md` working rule 11: a direct change with its `DECISIONS.md` row and tests).

**Done, 2026-11-09** (the user: "yes", `DECISIONS.md` 2026-11-09). Family `verification\v70` measured every item
with `qb64pe.exe`, and QB64pe accepted every form below, so each is now fixed and all 23 programs pass. Item 8
turned out narrower than written: a call of a one-bit FUNCTION *with* its suffix is "Name already in use"
(`v70_h1`), and one without it compiles (`v70_h2`). A ninth item came up while checking item 1 against the
Libraries Pack.

**Item 9: a library with `$INCLUDEONCE` named twice.** Each `a740g/QBDS-*` library pulls in `a740g/QBDS` with its
own `$USELIBRARY`, and QBDS's files start with `$INCLUDEONCE`. QB64Rust read the library's files again each time
and reported "name already in use" for its `DECLARE LIBRARY` SUB. QB64pe reads them once (`v70_i1`, `i2`). Behind
it was a second difference: a `DECLARE CUSTOMTYPE LIBRARY` block that names no file wrote a C++ prototype
(`extern void memcpy(void);`) that clashes with the C headers' `memcpy`. QB64pe writes none.

1. **A `SUB` and a `FUNCTION` of the same name: rejected, probably wrongly.** InForm's `extensions\HMap64.bi`
   has `FUNCTION HMap64_Remove%%` (line 99) and `SUB HMap64_Remove` (line 117), which calls the function and
   ignores its result. `extensions\Sort.bi` does the same with `Sort_StringArray`, and so do the Libraries Pack's
   `a740g/QBDS-HMap` and others. QB64Rust says "name already in use: `HMap64_Remove`". Then it says "the
   FUNCTION `HMap64_Remove` cannot stand as a statement" where the SUB is called. The reference `qb64pe.bas`
   allows the pair. At lines 25880 to 25883, a new FUNCTION's name is checked against reserved words, constants
   and other FUNCTIONs. A new SUB's is checked against reserved words, constants and other SUBs. Neither is checked
   against the other kind. InForm-PE's CI builds with the latest QB64pe release, `v4.7.0-GLFW`. **Blocks:** all
   42 InForm-PE examples and 16 of the 71 Libraries Pack examples.
2. **A type suffix on a `STRING * n` member** (`THEME.FILL_ADJ_ICON_FILE$ = ""`, the member declared
   `FILL_ADJ_ICON_FILE AS STRING * 64`). QB64Rust marks it "not supported yet". The code says why:
   "A suffix on a fixed-length string member was not measured" (`crates\sema\src\check\places.rs` 1821). A suffix
   on other members (`THEME.FILL_ADJ_ROTATE_RADIUS% = 28`) already works. **Blocks:** DRAW's main program, with
   many uses in `ASSETS\THEMES\DEFAULT\THEME.BI`, and 3 Museum programs, where it is their only error (32 uses).
3. **`< =` with a space between the two characters**, read by QB64pe as `<=`:
   `IF LAYER_COUNT% < = 1 THEN EXIT SUB`. QB64Rust marks it "not supported yet: operator `=` (expected an
   expression)". **Blocks:** DRAW's main program (53 uses in `GUI\LAYERS.BM`).
4. **A stray comma in a `DECLARE FUNCTION` parameter list**:
   `DECLARE FUNCTION JsonTokenGetValueBool&(j AS Json, idx, AS LONG)` in the Libraries Pack's
   `MattKilgore/QB64pe-Json/json.bi` line 128. This is a typo in the library. QB64pe probably ignores it, since
   the library ships with the pack. QB64Rust marks it "not supported yet". **Blocks:** 3 pack examples.
5. **`ON ERROR GOTO __catch_error_handler` inside a `SUB`**, in the Libraries Pack's test framework
   (`a740g/Catch/Catch.bm` line 262). The label is in `Catch.bas`, included into the main module. QB64Rust marks
   the statement "not supported yet"; the cause has not been traced. `ON ERROR GOTO` from a SUB to a label of the
   main module is supported and measured (`crates\sema\src\check\flow.rs`), so it is something narrower.
   **Blocks:** 2 pack examples, and any program that uses Catch.
6. **A line-continuation `_` joined to the word before it: rejected, probably wrongly.** Two Museum programs end
   a line with a keyword and `_` written together:
   `is_subset(grid_img(), gridwidt, gridheigh) or_` (`Petr\3DMEaP\Editor.bas` line 3183) and
   `IF NOT leftClick _ANDALSO NOT rightClick _ANDALSO NOT _MOUSEMOVEMENTX _ANDALSO_`
   (`TeddyKlugs\Rock-Jockey\RockJockey.bas` line 759). The lexer takes `_` as a continuation only where a token
   starts (`crates\syntax\src\lexer.rs` 144 and 165), so `or_` is read as a name. The reference `qb64pe.bas`
   tests whether the laid-out line ends in `_` (line 25505), whatever stands before it, and removes the `_`. So
   QB64pe reads `or_` at a line end as `OR` and a continuation. QB64Rust then reports "expected `THEN` or `GOTO`"
   or "expected a statement". **Blocks:** those 2 programs.
7. **`FOR … NEXT` inside a single-line `IF … ELSE`**:
   `If dx > 0 Then For j = 0 To 10 Step diggerspeedx: DRAWIMG ...: Next Else For j = ...: Next`
   (`RETROQB45\QDigger\QDIGGER.BAS` line 2447, a QB 4.5 game). QB64Rust marks `FOR` there "not supported yet",
   then says "`NEXT` inside a single-line `IF` cannot close a block outside it". **Blocks:** QDigger (2 programs).
8. **Smaller items from the Museum's report**, each in one to three programs: a string element or member in
   parentheses as an argument (2 programs, the only error of 1); a `STATIC` variable beside a `DIM SHARED` one of
   the same name (3); a `TYPE` named like a keyword or built-in (`swap`, `time`); an array with the name of a
   constant (10, seven of them built on one framework, `aKFrameWork`); "name already in use:
   `mouseIsOver`" in `aouwt\QB-WM\wm.bas`, where the function is declared with the `_BIT` suffix
   (`` FUNCTION mouseIsOver` ``) and called without it. The cause of the last is not traced.

Items 1 to 3 together stood between QB64Rust and the two largest QB64pe applications found: DRAW and InForm.
Since the fixes, InForm-PE's example `21-game` passes the front end; the other programs are left to the
front-end check of pinned clones (section 6, recommendation 3).

### 2.1 The Museum's other errors

The tally ran `-z` on each of the Museum's 520 `.bas` files (the report counts 519 programs). 459 check clean.
Most of the rest are not programs of their own, or not compiler problems:

- 24 are modules that a main program includes, such as the parts of the physics engine in
  `mechatronic3000\QBPool\fzxNGN\`. Alone they fail with "included file not found" or "the type `TBODY` of a
  parameter is no type". Their main programs pass.
- 2 programs have a real error in them, such as `END SUB` closing a `FUNCTION` in
  `AshishKingdom\p5js.bas\Samples\Demos\Blobby.bas` line 30. QB64pe should reject those too; that would need
  checking with `qb64pe.exe`.
- `madscijr\Demos\qb64_sound_examples_2-00.bas` fails on "invalid bounds for `m_arrMenu`: 0 TO -1" and two type
  errors, `Enrique-Huerta\MADMIX\EDIMIX\module\EDIMIX.BAS` on passing a variable to a `VARPTR` parameter. Each
  is one program and needs a closer look.

## 3. Functionality to build in, or to offer as a library or an extension

### 3.1 The Libraries Pack and `$USELIBRARY`

QB64pe 4.3.0 added `$USELIBRARY:'author/library'`. It finds a library in the pack's folder layout
(`libraries\descriptors\<author>\<library>.ini`, `libraries\includes\...`) under the compiler's folder. The pack
is a separate download. QB64Rust implements the metacommand. What is missing:

- The real pack as a test set (section 1.2), once section 2 items 1, 4 and 5 are fixed.
- A way to get the pack. The extension could offer to download a pinned release of it beside the compiler, as it
  fetches the toolchain (`qb64rust toolchain install`). Its licenses are per library, so it is downloaded, not
  shipped.
- This is the nearest thing to "a way to share libraries" (`study\43`, Tools item 5). A `[dependencies]` table
  could build on the same layout rather than invent one.

### 3.2 A QuickPak compatibility library

Crescent Software's libraries were bought in 2018 by Gene Buckle, who released them to the public domain
(`geneb/QuickPak-Pro-DOS`, `QuickPakScientific`, `QuickScreen`, `Graphics-QuickScreen`, `GraphPak`, `PDQ`,
`PDQComm`, `LaserPak`, `QBPlus`, `XREF`, `QuickMenu`, `GraphicsWorkshop`, `CompressionWorkshop`, `QBase`). Many
DOS-era commercial programs were built on QuickPak Professional: the Lance Haffner games are one, XREF another.
QuickPak Professional has 202 BASIC files and 96 assembly files. The assembly cannot be built by QB64pe or
QB64Rust.

So a program that calls QuickPak routines fails here, and in QB64pe. XREF (`9155350`) calls, among others,
`QPLen`, `Exist`, `MaxInt`, `MinInt`, `ASCII`, `InCount`, `FindExact`, `BaseName$`, `NoPath$`, `ParsPath$`,
`QPValI` and `FileExtension$`. QuickPak Scientific's calculator `HANDHELD.BAS` calls `Compact$` and others.

A library of BASIC versions of the most-called routines, written here under the project's license, would make
these programs build with QB64Rust. It would work with QB64pe too. It would be an optional library, perhaps a
`$USELIBRARY` entry of our own. The Lance Haffner port did this for its own needs
(`src\QPProEqu.bm`, 50 KB). It is AGPL, so it shows which routines matter but is not copied. The QuickPak manual
(a PDF linked from the README) specifies each routine. Most are short string, file and array helpers. Routines
that read video memory or call DOS interrupts would stay out.

### 3.3 Reading a QuickBASIC `.MAK` project

A QB 4.5 or PDS 7.1 program of several modules is held together by a `.MAK` file: a list of the `.BAS` files to
load. QB64pe has no such thing; a port joins the modules with `$INCLUDE` by hand, as the game ports do
(`PORTING-PDS.md`, "Programs and files"). QuickPak Scientific has 60 `.MAK` files, and Final Quest 2, XREF and
ADVGEN are multi-module programs too. QB64Rust could build a `.MAK` file directly: take its first module as the
main program, append the others as if each were included at the end, and report a `COMMON` or `DECLARE` that does
not match between modules. A command that writes the joined program out would help a port. It would be a feature
of its own, outside the QB64pe dialect, so it needs a decision.

### 3.4 Reading QuickBASIC's binary source files

QB 4.5 and PDS 7.1 could save a program in a tokenized "fast load" format, which starts with byte `0xFC`. Much
code of the era survives only in that form. QB64pe cannot read it. `mark-iid/qb45detok` (Python, Apache-2.0,
2026-09) converts such a file back to text without DOS. QB64Rust could:

- detect a `0xFC` file and say what it is, instead of reporting syntax errors on binary bytes;
- convert it, with a command `qb64rust detok <file>`, or in the editor on opening.

This idea is already the `SOMEDAY.md` row `qb45-binary` ("Know a QuickBASIC 4.5 binary file"); `qb45detok` is a
reference for that row, not a new one. That tool's code shows the format. GW-BASIC's tokenized and "protected" files are a second format of the same
kind; `danvk/gwbasic-decoder` and PC-BASIC read them.

### 3.5 An MCP server

`grymmjack/qb64pe-mcp-server` (TypeScript, MIT) gives AI assistants 52 tools for QB64pe: search the wiki,
compile, explain errors, port QB 4.5 code. QB64Rust already has the parts such a server needs: the checker, the
formatter, hover and the build. A `qb64rust mcp` command next to `qb64rust lsp` would expose them. This is a new
idea, not a row of `SOMEDAY.md`.

### 3.6 Other editors

`Pterox64/qb64pe-zed` is a Zed extension with a tree-sitter grammar. Its README says it "was written by AI" and
its author has not reviewed it. Zed, Neovim and Helix can all use a language server. So `qb64rust lsp` could
serve them with only a small extension or a configuration snippet each. That would bring hover, diagnostics and
formatting to editors other than VS Code at little cost. No tree-sitter grammar for QB64 exists that is worth
taking.

### 3.7 A visual GUI test harness

`grymmjack/qa-harness` (MIT, 2026-11) drives a desktop program with real mouse and keyboard input, takes
screenshots and asserts on named regions of pixels. It has a `--calibrate` mode that outlines every region on the
live window, so that a person can check the regions before trusting a run. `tools\screen` already judges a
program by its screen state, from inside the runtime. The harness's ideas, regions with names and a calibration
view, could make the `.screen` comparisons easier to read when they fail. Nothing would be copied.

### 3.8 Smaller ideas

- **A view of the generated C++.** `parkertomatoes/basbolt` is a compiler explorer for QuickBASIC: it compiles
  as you type and shows the assembly, colored by source line. `qb64rust -z` already writes the C++. A side
  panel in the extension that shows the C++ of the current `SUB` would help with debugging and teaching.
- **QBJS.** `boxgaming/qbjs` (MIT, 86 stars, active) runs QB64-style BASIC in the browser, with the GX game
  engine (`boxgaming/gx`, MIT). `study\43` (Tools item 10) says a QBJS row is in `SOMEDAY.md`'s "VS Code parity
  left"; it is not there now. A web build, or a check that a program uses only what QBJS supports, would need a
  row of its own.
- **Bindings over `DECLARE LIBRARY`.** `a740g/raylib-64` (MIT) binds the raylib game library through
  `DECLARE LIBRARY`, and `a740g/Physac64` does the same for a physics library. Both are good tests of the
  external-libraries spec, beyond the corpus programs with a header.
- **A test framework.** `a740g/Catch`, in the Libraries Pack, is a small test framework for QB64pe programs
  (`TEST_CASE_BEGIN`, `TEST_CHECK`, ...). `qb64rust test` and the Test Explorer have their own form. Once Catch
  builds (section 2 item 5), the Test Explorer could also list Catch tests.

## 4. Other implementations to watch

Reference only: nothing is copied (as `CLAUDE.md` working rule 3 sets for other projects).

| Project | What it is | License | Why watch it |
|---|---|---|---|
| `logiclrd/QBX` | PDS 7.1 reimplemented in C#: interpreter, IDE, emulated hardware (AdLib, fonts, keyboard layouts) | LGPL-3.0 | Its behavior tests (section 1.4); how it emulates DOS details we skip |
| `flukiluke/qb64-lang-utils` | A new parser and formatter for QB64 in Python, by a QB64pe core developer, with a table of which commands it supports (`command-support.txt`) | None | It may become QB64pe's own tooling (a language server or formatter). If it does, its rules will be QB64pe's |
| `ngeor/rusty-basic` | A QBasic interpreter in Rust; runs `MONEY.BAS` | MIT | The only other Rust implementation found, apart from the user's QB64Fresh |
| `Hawkynt/PB-Compiler` | A PowerBASIC, QuickBASIC and GW-BASIC compiler in C# that emits 16-bit DOS executables, "driven against the original compilers until its programs behave identically" | LGPL-3.0 | The same method as ours, with real DOS compilers as oracles |
| `mojontwins/qblike` | A clean-room QBasic 1.1 for Windows and Linux on the `dos-like` library | None | What a small QBasic runtime needs |
| `robhagemans/pcbasic` | GW-BASIC, BASICA and PCjr BASIC, aiming at bug-for-bug compatibility; Microsoft Binary Format floats | GPL-3.0 | The reference for GW-BASIC behavior and old data files |
| `thirawat27/QBNex` | "A modern QBasic compiler", in C, pushed 2026-11-09 | MIT | Unclear yet whether it is new or a QB64 fork |
| `jichu4n/qbjc`, `elektito/qbee`, `microcai/llvm-qbasic` | QBasic to JavaScript, to a VM, and through LLVM | Apache-2.0, GPL-3.0, none | Other front ends; small |

## 5. Looked at and left out

- The old QB64 (`QB64Team/qb64`, `Galleondragon/qb64`), forks of QB64pe (`TeamQB64pe/QB64pe`,
  `SteveMcNeill/Steve64`, the `QB64Petr/*` feature branches), and `SteveMcNeill/QB64-SDL`. The reference clones
  cover QB64pe.
- `grymmjack/qb64pe-vscode`, the community extension: reference only (`CLAUDE.md` working rule 3).
- `FellippeHeitor/vWATCH64`, the debugger that became QB64pe's `$DEBUG`. Its successor is D-017 in
  `DIVERGENCES-QB64PE.md`, and QB64Rust has its own debugger.
- `FellippeHeitor/QB64-interpreter` (2023, no README), `jamonholmgren/qub` (a QB64 web framework, macOS and
  Linux only), `RhoSigma-QB64/QB64GuiTools` (a GUI framework, 2.4 MB of BASIC, MIT): possible tests later, not
  first.
- `Fundynamic/arrakis`, a Dune II-like strategy game in QuickBASIC: it needs DirectQB, an assembly library.
- `dmitrysmagin/wetspot2`: a C port of the QB 4.5 game, not BASIC.
- Remakes of commercial games (`rokcoder-qb64/galaga`, the Museum's Dragon Warrior 64): see section 1.2.
- `clintmoyer/oregon-trail` (124 stars): the listing printed in 1978, in a 71 KB repository. It is small, and
  its Unlicense was given by the uploader, not the authors.
- `davidshq-legacy/pyBASICTools`: the user's own 2020 tool that splits SUBs into files; the formatter and language
  server do more now.

## 6. Recommendations and the panel's view

The panel: a pragmatic engineer, a QB64 engineer, a compiler and languages engineer, a Rust engineer, a test
engineer, and a **license reviewer**. The last is added because half of these choices turn on what may be kept in
this repo.

**Recommended order:**

1. **Measure and fix the two likely false errors (section 2 items 1 and 6).** A verification program first, then
   a direct change with its `DECISIONS.md` row. The `SUB`/`FUNCTION` rule unblocks InForm and a quarter of the
   Libraries Pack. Claude's time: about an hour each with the measurement.
2. **Measure and build items 2 to 5 and 7 of section 2.** Each is one measured behavior. Items 2 and 3 unblock
   DRAW; items 4 and 5 the rest of the Libraries Pack. About an hour each.
3. **Done, 2026-11-09 (change `community-programs`: 66 repositories, 3018 of 3977 programs pass the front end,
   `tests\community\README.md`).** **Add a "real programs" check, front end first.** Pin the Museum, the Libraries Pack, DRAW, InForm-PE,
   HamCalc64 and QBX by commit. Then check them with `qb64rust -z`, from local clones, in the suites VM (this is
   over 20 programs: `CLAUDE.md` working rule 6). Keep in this repo only the list of commits and the list of
   programs that must pass, which can only grow. Building and running them comes later, a few at a time, with
   keys or `.stdin` files and recordings made with `qb64pe.exe`.
4. **Decide before copying whole programs in.** ChessbrainQB64, DRAW and HamCalc64 are MIT and can be copied
   with their license files. The Lance Haffner games are AGPL-3.0: either keep them in a clone, or copy them into
   a folder of their own with their license, or ask their maintainer.
5. **Later, as `SOMEDAY.md` rows:** the QuickPak compatibility library (3.2), building `.MAK` projects (3.3),
   reading `0xFC` files (3.4), `qb64rust mcp` (3.5), editor snippets for Zed, Neovim and Helix (3.6), the C++
   view (3.8), and the Libraries Pack download in the extension (3.1).

**The panel:**

- **Pragmatic engineer:** recommendations 1 and 2 cost little and unblock the two biggest QB64pe programs found.
  They should come before any new test set. Of the test sets, the Museum is the best value: it is already
  curated for QB64pe.
- **QB64 engineer:** InForm and the Libraries Pack are what QB64pe users actually use. Today every InForm
  program fails to build with QB64Rust. That matters more than any QB 4.5 archive.
- **Compiler and languages engineer:** treat every item of section 2 as a measurement to make, not a guess. Two
  of them may be cases of wider rules. `< =` may be one case of spaces inside any two-character operator (`< >`,
  `> =`). A `_` joined to a word may apply to any word, not just keywords. The verification programs should test
  each whole family. Reading `qb64pe.bas` found the two likely false errors; it does not replace running it.
- **Rust engineer:** the QuickPak library and `0xFC` reading are separate from the compiler. The first is BASIC
  in its own folder. The second is a small crate or a module of `driver` with fuzz tests. Neither should touch
  `sema`. A `.MAK` project is mostly `driver` work too: it is a list of files joined before the front end.
- **Test engineer:** front end first, because it is fast and needs no recordings. A program that passes the
  front end today and fails tomorrow is a regression we would otherwise not see. The Lance Haffner games and
  HamCalc64 are the best candidates for run-time tests after Civil War Strategy: text mode, typed input, data
  files.
- **License reviewer:** the Museum, the ABC archive, `hoard-of-gwbasic` and the Microsoft samples have no
  license that allows copying. They are run from clones, as `tools\qb45` runs QB 4.5. The Crescent releases are
  public domain by the owner's statement, though the files still carry copyright notices; keep the README's
  statement with any copy. AGPL-3.0 test data in an MIT repository is allowed if it stays in its own folder with
  its license, but it is a policy choice. Leave OpenStrut out.

**Questions for the user:**

1. Measure and fix section 2's items now, as direct changes? (The panel: yes; items 1, 6, 2 and 3 first.)
   **Answered "yes"; done 2026-11-09** (section 2).
2. Start the front-end check of pinned clones (Museum, Libraries Pack, DRAW, InForm-PE, HamCalc64, QBX)? (The
   panel: yes, in the suites VM.) **Answered "yes"**, with the repositories of section 1.3 added to the set.
3. May AGPL-3.0 programs, the Lance Haffner games, be copied into the repository as test data, or only run from
   a clone? (The panel: from a clone, unless the user wants them kept.) **Answered "run from a clone".**
4. Which of section 3's ideas become `SOMEDAY.md` rows? **Answered "yes" to all of them, 2026-11-09**: seven
   ideas and the four remaining gaps of §2 item 8 are rows now, each pointing back here; reading `0xFC` files
   was already the row `qb45-binary` (`DECISIONS-SMALL.md` 2026-11-09). (The panel: all seven of recommendation 5, at P3 or
   P4.)
