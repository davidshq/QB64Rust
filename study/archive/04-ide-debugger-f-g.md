# 04 — IDE, IDE/compiler coupling, and the `$DEBUG` debugger

Study of QB64 Phoenix Edition (`..\QB64pe`) for the ground-up rewrite. Read-only study; nothing in the repo was modified and nothing was executed — every behavioural statement comes from reading source.

## How to use this document

The report is organised in parts. Section numbers restart inside each part, so cross-references are written "Part B §2.4".

| Part | Content | Primary sources |
|---|---|---|
| **Summary** | Answers to the seven study questions in brief, with pointers into the parts | — |
| **A** | Compiler side of the IDE interface (the `ide(0)` coroutine, message codes, error path, shared globals, C++ build launch) | `source\qb64pe.bas` |
| **B** | `ide` / `ide2`: entry, protocol (IDE side), as-you-type checking, auto-layout, main loop, keys, mouse, menus, run flow, undo, recovery | `source\ide\ide_methods.bas` 1-6975 |
| **C** | Text buffer, file I/O, dialog/control toolkit, input layer, editor renderer and syntax highlighting, search/replace | `ide_methods.bas` 10849-13700, 14929-15772, 16969-17126, 18805-18935, 20655-21424; `ide_global.bas` |
| **D** | Every dialog and menu builder, help-view renderer, history/recent/bookmark files | `ide_methods.bas` 13701-14928, 15773-16968, 17127-18804, 18936-20654 |
| **E** | Help/wiki, config (ini) with full key table, export, converters, support includes, command line | `source\ide\wiki\`, `source\ide\config\`, `ide_export.bas`, `ide_converters.bas`, `internal\support\`, `source\global\` |
| **F** | Debugger: instrumentation, TCP protocol, debuggee state machine, `DebugMode`, watch list, watchpoints, call stack | `internal\support\vwatch\`, `ide_methods.bas` 6976-10850, `qb64pe.bas` (targeted) |
| **G** | Assessment for the rewrite | — |
| **H** | Coverage (what was read fully vs sampled) and open questions | — |

**Parts A–D and Part E §1–§4 moved on 2026-10-18** to `study\archive\04-ide-debugger.md`, unchanged and under
the same headings: the text-mode IDE's internals (the compiler side of the IDE, `ide2`, the text buffer, the
dialog toolkit, every dialog and menu, help, configuration, export, converters), which are not ported
(`DECISIONS.md` 2026-10-02). They are read only when needed (`CLAUDE.md` working rule 10). This file keeps the
summary, Part E §5–§7 (auto-includes, the command line), the debugger (F), the assessment (G) and coverage (H).

Line references: `qb64pe.bas:N` is `source\qb64pe.bas`; `L123`, `M:123` or `ide_methods:123` is `source\ide\ide_methods.bas`. The compiler-side control flow (labels `ideprepass`, `ide3`, `compileline`, `ide5`, `ide6`, `ideret1..6`) is also described in `01-compiler-frontend.md`; Part A here covers only what the IDE depends on.

Size of the subject: `ide_methods.bas` is 21,424 lines (885 KB) holding about 115 SUB/FUNCTIONs, of which one, `FUNCTION ide2`, is 6,880 lines. `ide_global.bas` 242 lines, wiki 1,523, config 737, export 696, converters 142, `vwatch.bm` 885.

---

# Summary: answers to the study questions

## 1. Architecture

- The IDE is not a separate program or thread. The compiler main program calls `C = ide(0)` in a loop (`qb64pe.bas:853-858`); the IDE runs its event loop inside that call and *returns* when it wants the compiler to do something. It is a hand-rolled coroutine: the compiler's request goes in the global `idecommand$` (one code byte plus payload), the IDE's answer is the function result plus the global `idereturn$`. All IDE state must be `DIM SHARED` or `STATIC` to survive a yield (Part B §1.1).
- **Compiler → IDE codes** (`idecommand$`): `""` start; 1 open file from command line; 3 line done, send next; 6 ready/OK; 7 repass from line 1; 8 error (message + `MKL$(line)`); 10 include-line passback; 11 EXE created; 12 EXE name changed; 13 `$NOPREFIX` conversion offer; 100 continuation-line fetch (called from inside `lineformat`, `qb64pe.bas:25525`); 254 launch debugger; 255 runtime error inside IDE code. **IDE → compiler return values**: 2 begin (first line in `idereturn$`); 4 next line; 5 end of program; 9 build and run (EXE base name); 14 `$NOPREFIX` refused; 0 leave IDE mode (never actually returned — the IDE exits with `SYSTEM`). Tables: Part A §2, Part B §1.3-1.4.
- **As-you-type checking**: every edit sets `idechangemade`; on the next loop iteration the IDE writes an undo snapshot and returns 2, which makes the compiler restart from scratch. The compiler then asks for one line at a time (3 → 4), runs a prepass over all lines, gets 5, sends 7, runs the main pass, gets 5 again, and sends 6 or 8. The IDE polls input once per line, so typing interrupts a pass simply by causing another "2". There is no cancel message and no debounce (Part B §2.2-2.3). A fast path in `FUNCTION ide` avoids entering `ide2` for off-screen lines (Part B §1.2).
- **Errors, warnings, status**: one error at a time arrives as code 8 and is shown in the 3-row status area with a clickable "on line N" link; the error line is highlighted through `idefocusline`. Warnings are not messages: the compiler fills shared arrays `warning$()`, `warningLines()`, `warningIncLines()`, `warningIncFiles()` (`qb64pe.bas:28668-28720`) which the IDE reads directly (Part D §1.2). Progress is "..." or a percentage (Part B §2.5).
- **Auto-layout**: after compiling each line the compiler leaves the formatted text in the global `layout$` (leading spaces = indent level; `qb64pe.bas:12662-12680`). On code 3 the IDE converts it and writes it into the buffer with `idesetline`, which deliberately bypasses undo, the dirty flag and recompilation; the line under the cursor is deferred until the cursor leaves it (Part B §2.4).

## 2. Text buffer, undo, files

- The whole document is one string `idet$`; each line is `MKL$(len) + text + MKL$(len)`. A cached position (`idel`, `ideli`) makes seeking relative; every insert/delete/replace copies the whole string (Part C §1).
- Undo/redo is a per-instance on-disk ring of deflate-compressed whole-buffer snapshots (`undo3<n>.bin`), with typing runs coalesced. The same file is the crash-recovery backup; an empty flag file `autosave<n>.bin` marks an unclean exit. There is no autosave timer (Part B §8.1-8.2).
- Bookmarks are an array of (line, column), shifted on line insert/delete and persisted per file in `bookmarks.bin`. Selection is an anchor plus the cursor; multi-line selections are whole-line granular. Clipboard uses `_CLIPBOARD$`; multi-line paste always inserts whole lines above the current line (Part B §4.5-4.6, §8.6).
- Files: loader accepts LF/CR/CRLF/LFCR, expands tabs to 4-column stops, strips a trailing `CHR$(26)`; saving writes OS-native line endings. There is no BOM, UTF-8 or encoding detection — the buffer is bytes shown through a selectable code page mapped with `_MAPUNICODE`. QB4.5 binary files are detected and converted by an external tool (Part C §2, Part E §4).

## 3. UI framework

- Text mode drawn to page 3 and copied to page 0. Dialogs use two record types, `idedbptype` (window) and `idedbotype` (control; five control types), a 1000-entry string pool (`idenewtxt`), `idedrawobj`/`ideobjupdate`, and a ~100-line event loop repeated in each dialog (Part C §3, Part D §0.1). Input comes through `GetInput` into globals (`K$`, `KB`, `mX`, `mY`, `mCLICK`, …) (Part C §4).
- Menus are string arrays `menu$(m, i)`; handlers dispatch by comparing the literal item text (Part B §6). Full menu table with definition and handler lines: Part B §6.3. All dialogs with implementing routine: Part D §9.
- Main loop phases with line ranges: Part B §3.

## 4. Features

Syntax highlighting (Part C §5), quick navigation (Part B §8.7), SUBs dialog (Part D §1.1), search/replace (Part C §6), help/wiki download, cache and markup renderer (Part E §1, Part D §5), contextual F1 help (Part E §1.7), export to HTML/RTF/Discord/forum/wiki (Part E §3), converters (Part E §4), compiler settings (Part D §3.2), colour schemes (Part D §3.7), config ini with every key (Part E §2), recent files and history (Part D §6). Two items in the brief do not exist in this tree: there is **no auto-includes UI** (auto-including is compiler-side; the only UI hook is launching an optional external Library Explorer, Part E §5.3) and **no external-editor mode** (the nearest equivalents are the command-line switches `-x`, `-c`, `-z`, `-y`, Part E §6).

## 5. Debugger

`$DEBUG` makes the compiler append a BASIC SUB (`vwatch.bm`) to the user program and call it before every statement of the main file. That SUB is a TCP client; the IDE is the host on port `BaseTCPPort + instance` passed through the environment variable `QB64DEBUGPORT`. Frames are a 4-byte length plus `command:payload`. Breakpoints, stepping and watchpoints live in the debuggee; variable metadata stays in the IDE's copy of the compiler's `usedVariableList()`. Complete command tables, state machine and limitations: Part F.

## 6. Run/build flow

F5 returns code 9 with the EXE base name once a check has completed; side-channel globals (`iderunmode`, `NoExeSaved`, `idecompiled`, `ModifyCOMMAND$`, `SaveExeWithSource`) carry the details. The compiler runs `make` through a hidden `SHELL`, redirecting output to `internal\temp<n>\compilelog.txt`, and judges success only by whether the EXE file exists; failure comes back as an ordinary code-8 error whose text contains a clickable link that opens the log in the OS default viewer (Part A §5, Part B §7).

## 7. Assessment

See Part G.

---

# Part E — Help/wiki, config, export, converters, support files, command line

*(Section numbers below are local to Part E. §1–§4, help/wiki, configuration, export and converters, are in
`study\archive\04-ide-debugger.md` with this part's scope paragraph.)*

## 5. Support includes and the auto-include mechanism

### 5.1 Auto-include manager (compiler side, `source\qb64pe.bas:1725-1799`)

`autoIncludeManager` is a GOSUB routine invoked from the per-line loops of both passes (prepass qb64pe.bas:1812-1818, main pass 3369-3374) whenever one of three position triggers is armed: `firstLine = 1` (before the first user line; set at 1242/3257 and 911), `mainEndLine = 1` (end of the main module; also forced when `lastLine = 1` arrives first, 1812), `lastLine = 1` (after the last line; 890 in IDE mode, 3230/12699 otherwise). Each trigger has states 1 = pending, 2 = in progress, 3 = done (1793-1796). The routine writes a list of file names into a temporary s-buffer `tmpdir$ + "autoinc.txt"` (1726) and then drives the normal `$INCLUDE` machinery (`autoInclude_prepass` at 3101, `autoInclude` at 12516) once per listed file. The real current line is saved/restored around it (1815-1818).

Injection order (paths are written with backslashes on every platform and corrected later, see `source\utilities\s-buffer\sb_qb64pe_extension.bm:222`):

1. **At top** (1728-1752):
   - `internal\support\include\beforefirstline.bi` (unless that very file is the program being edited);
   - `internal\support\color\color0.bi` if `$COLOR:0` (`ColorSet` = 1) or `internal\support\color\color32.bi` if `$COLOR:32` (`ColorSet` = 2) (1734-1738; the flags are set at 1903-1913);
   - `IncAtTop` files of all `$USELIBRARY` libraries, in **reverse** order of registration (1740-1744);
   - `internal\support\vwatch\vwatch.bi` if `$DEBUG` is on (1747-1751).
2. **After main** (1753-1764): `internal\support\include\aftermain.bas`, then `IncAfterMain` library files in order of appearance.
3. **At bottom** (1765-1787): a `-----` marker line (tells the include driver that the AfterMain group is finished when both triggers fire together, handled at 3104/12519), then `vwatch.bm` (with `$DEBUG`) or `vwatch_stub.bm` (without), then `IncAtBottom` library files, then `internal\support\include\afterlastline.bm`.

Also: `setPrecompFlags` defines the pre-compiler variable `_SOCKETS_` from the sockets dependency state (1722) so `afterlastline.bm` can conditionally compile network helpers. `autoIncludingFile` is 1 for ordinary auto-includes and -1 for `beforefirstline.bi` / `afterlastline.bm` (12520-12523); -1 relaxes the "no single leading underscore in user names" check for those two files (qb64pe.bas:28269-28285). Errors raised inside vwatch/library auto-includes are rewritten to less confusing messages (14313-14318). Auto-included files are flagged internal (`incIsInternal`, 471) for debug-path resolution.

### 5.2 `$USELIBRARY` (qb64pe.bas:1915-1975; layout handling 3821-3825)

Syntax `$USELIBRARY:'author/library'`. Requires `libraries/descriptors/<author>/<library>.ini` with section `[LIBRARY INCLUDES]` and optional keys `IncAtTop`, `IncAfterMain`, `IncAtBottom` (file names relative to `libraries/includes/<author>/<library>/`), read with the same INI manager (`ReadSetting$`, 1929-1939). Registered in `useLibList$(4, n)` with row constants `ullName=0, ullNeedy=1, ullTop=2, ullMain=3, ullBottom=4` (qb64pe.bas:162-163); `ullNeedy` stores "file + line" of the referrer for de-duplication (1945-1960). A new registration forces a recompile (`GOTO do_recompile`, 1975).

### 5.3 "Auto-includes UI"

There is **no IDE dialog or setting that manages automatically included files or libraries**; the mechanism is compiler-side only. Case-insensitive greps of `source\` for `autoinclude`, `auto-include`, `useLibList`, `$USELIBRARY`, `LibExplorer`, `LibraryExplorer` found only:
- the compiler-side manager and metacommand described above;
- `$USELIBRARY` in the highlighter keyword list (`syntax_highlighter_list.bas:5`);
- a launcher for an **external** "Library Explorer": at startup, if a `libraries` folder exists, the program looks for `LibraryExplorer.exe`, `./LibraryExplorer`, `libraries\LibraryExplorer.exe`, `./libraries/LibraryExplorer` (qb64pe.bas:359-367; `LibExplorer$` declared at `ide_global.bas:179`). If found, the Tools menu gains "Library Explorer...  Ctrl+L" (ide_methods.bas:432-435), which only runs `SHELL _HIDE _DONTWAIT` on that executable (5605-5609; Ctrl+L at 3486). The explorer is part of a separate "Libraries Pack add-on"; neither it nor a `libraries` folder is present in this checkout.
- a special edit mode: opening `beforefirstline.bi` or `afterlastline.bm` in the IDE shows an "Attention" box and forces OPTION _EXPLICIT (`ForceOptExpl`) and underscore-prefixed names until another file is loaded or File->New is used (ide_methods.bas:6670-6685, 6571).

### 5.4 What the support files contain

**`internal\support\include\beforefirstline.bi`** (270 lines; `$INCLUDEONCE`): language-level constants only, all underscore-prefixed:
- `_TRUE`, `_FALSE`; `_LESS`, `_EQUAL`, `_GREATER`; failure/state values `_HOST_FAILED`, `_CLIENT_FAILED`, `_CONNECTION_FAILED`, `_INVALID_IMAGE`, `_LOADFONT_FAILED`, `_SNDOPEN_FAILED` (35-40);
- strings `_STR_EMPTY`, `_STR_CRLF`, `_STR_LF`, `_STR_NAT_EOL` (OS-conditional via `$IF WIN`) (42-48); `_E##` (50);
- `_LOG_TRACE` .. `_LOG_NONE` (53-57);
- time constants `_SECS_IN_MIN` ... `_WEEKS_IN_LEAPYEAR` (60-77); size factors `_ONE_KB` .. `_ONE_EB` (80-85);
- `_SIZE_OF_*` for all elementary types incl. `_SIZE_OF_OFFSET` (`$IF 32BIT`) (88-101); `*_MIN` / `*_MAX` limits for all integer, float and offset types (104-128);
- `_ASC_*` / `_CHR_*` pairs for control characters and punctuation (131-197);
- `_KEY_*` codes for `_KEYHIT`/`_KEYDOWN` (200-211);
- `_ERR_*` runtime error codes (214-269).

**`internal\support\include\aftermain.bas`** (21 lines): `$INCLUDEONCE` and a single `END` statement: an implicit program end so execution cannot fall through into library "after main" code.

**`internal\support\include\afterlastline.bm`** (167 lines; `$INCLUDEONCE`, `$CHECKING:OFF`): BASIC-implemented built-ins.
- `FUNCTION _IKW_EncodeURL$` (59-93) and `FUNCTION _IKW_DecodeURL$` (97-117). The `_IKW_` prefix ("internal keyword") tells the registration logic to register the routine as the keyword without the prefix (`_ENCODEURL$`, `_DECODEURL$`); such names must also be added by hand to `syntax_highlighter_list.bas` (header comment 40-54).
- Inside `$IF _SOCKETS_`: helper `FUNCTION _WhatIsMyIP$` (125-166) used by `_CONNECTIONADDRESS$`; it fetches `http://qb64phoenix.com/qb64_files/ip.php` with `_OPENCLIENT`.

**`internal\support\color\color0.bi`** (23 lines): 17 `CONST name~%%` text-mode colours (Black..BrightWhite = 0..15, Blink = 16), without underscore prefix. **`color32.bi`** (276 lines): 270 `CONST name~&` 32-bit colour values based on HTML/crayon colour names (AliceBlue, Almond, ...). Both use `$INCLUDEONCE`. The two files share names (e.g. `Black`), so only one is included per program.

**`internal\support\vwatch\`** (`vwatch.bi`, `vwatch.bm`, `vwatch_stub.bm`): debugger runtime, covered by another section.

### 5.5 Implications for a rewrite

- Part of the "language" (all `_TRUE` / `_KEY_*` / `_ERR_*` / ... constants, `_ENCODEURL$`, `_DECODEURL$`) is not in the compiler core but in BASIC source silently compiled into **every** program. The compiler/IDE source itself depends on them (`_TRUE`, `_FALSE`, `_EQUAL`, `_KEY_F1`, `_STR_CRLF`, `_CHR_CR`, ... are used throughout, e.g. `source\global\constants.bas:9-16`).
- These files need special name-validation rules (leading underscore allowed/enforced), must be OPTION _EXPLICIT clean, and must not disturb user-visible line numbers or debugger stepping.
- The three-position injection model (top / after main / bottom) is also the extension point for `$USELIBRARY`, `$COLOR` and `$DEBUG`; the ordering rules (reverse order at top, `aftermain.bas` END first, `afterlastline.bm` last) are semantically significant.
- A `vwatch_stub.bm` is included whenever `$DEBUG` is off (details in the vwatch section).
- Every compile re-parses these files; a rewrite could pre-register them as built-ins but must keep identical observable names/values and identical clash behaviour with user identifiers.

### 5.6 `source\global\settings.bas` and `constants.bas`

- `settings.bas` (3 lines): `CONST Debug = 0`, the compiler self-debug switch (enables `debug.txt` output and readable separators).
- `constants.bas` (28 lines): token separators `sp` (CR), `sp2` (LF), `sp3` (SUB, 0x1A) plus `sp_asc`, `sp2_asc`, `sp3_asc` (replaced by CHR$(250)/(249)/(179) when `Debug`); `CHR_QUOTE`, `CHR_TAB`, `CRLF`; `NATIVE_LINEENDING`; `OS_BITS` (64, or 32 if `_OS$` contains `[32BIT]`); `TARGET_BITS` (defaults to `OS_BITS`, overridable with `-f:TargetBits`).

### 5.7 `source\subs_functions\syntax_highlighter_list.bas` (164 lines)

Declares `DIM SHARED listOfKeywords$, listOfCustomKeywords$, customKeywordsLength AS LONG` (line 1) and builds `listOfKeywords$` as one long string of upper-case names separated and enclosed by `@` (`"@NAME1@NAME2@...@"`). Organisation: one block for metacommands (3-7), then one block per initial letter A..Z (9-163), each made of three string lines: QB64 keywords, QB4.5 keywords, OpenGL (`_GL*`) keywords. Names with a string suffix include the `$`. Lookup everywhere is `INSTR(listOfKeywords$, "@" + UCASE$(word$) + "@")` (e.g. ide_export.bas:388). It is a pure highlighting/export list, maintained by hand and independent of the compiler's real keyword registration.

---

## 6. Command line and no-IDE modes

### 6.1 Parser

`FUNCTION ParseCMDLineArgs$ ()` at `source\qb64pe.bas:14354-14611`, called at 411 (after `ReadInitialConfig`, so switches override config values). Tokens are `COMMAND$(i)`; only the **first two characters, lower-cased**, select the switch (14362), so e.g. `-compile` behaves like `-c`. `/?`, `/h`, `/help`, `--help` are aliases of `-?` (14361). The first token that is not a recognised switch becomes the source file (14583-14584); later unknown tokens are ignored. The return value is the source file name. Afterwards `_STARTDIR$` is prefixed to the source file if it exists there, and to the `-o` path if its folder exists there (413-418).

| Switch | Effect | Flags / variables | Line |
|---|---|---|---|
| `-?`, `-h`, `--help`, `/?`, `/h`, `/help` | Print usage (incl. `-s` and `-f` help) to the console, exit | | 14363-14414 |
| `-v` | Print `QB64-PE Compiler V<version>`, exit | | 14416-14419 |
| `<file>` | Source file: loaded into the IDE, or compiled/formatted in the no-IDE modes; `.bas` appended if it has no extension | return value -> `CMDLineSrcFile$` | 14583 |
| `-o <file>` | Output file. Compile: overrides EXE name/location (a trailing `.exe` is stripped, 13505-13516). Mandatory for `-y`. | `CMDLineOutFile$` | 14421 |
| `-l:<n>` | IDE only: after loading, move the cursor to line n (consumed at ide_methods.bas:624-628) | `IDEStartAtLine` | 14425 |
| `-c` | Compile without IDE; progress shown in the program's own window | `NoIDEMode` | 14429 |
| `-x` | Compile without IDE; all output to the console, no window | `NoIDEMode`, `ConsoleMode` | 14433 |
| `-y` | Format mode: write the auto-formatted source to the `-o` file; nothing is compiled to an executable | `FormatMode`, `ConsoleMode`, `NoIDEMode`, `QuietMode` | 14438 |
| `-z` | Generate C++ only (into `internal\temp`), skip the C++ compiler; usable as a syntax check | `NoCCompileMode`, `ConsoleMode`, `NoIDEMode` | 14445 |
| `-p` | Purge pre-compiled content (`PurgeTemporaryBuildFiles`), executed immediately during parsing | | 14451 |
| `-e` | Force OPTION _EXPLICIT for this compilation (cleared again if the IDE is started, 14598) | `ForceOptExpl` | 14455 |
| `-s` | Show the three persistent settings and exit | `SettingsMode` | 14459-14472 |
| `-s:DebugInfo[=bool]` | Show / set and save `[COMPILER SETTINGS] IncludeDebugInfo` (setting it also purges build files) | `IncludeDebugInfo` | 14475-14481 |
| `-s:ExeWithSource[=bool]` | Show / set and save `[GENERAL SETTINGS] SaveExeWithSource` | `SaveExeWithSource` | 14482-14487 |
| `-s:ExeDefaultDir[=path]` | Show / set and save `DefaultExeSaveFolder` (must exist; relative to the start dir allowed) | `DefaultExeSaveFolder$` | 14488-14502 |
| `-f` | Print the list of temporary settings, exit | | 14516-14520 |
| `-f:OptimizeCppProgram=bool` | Temporary override (not saved) | `OptimizeCppProgram` | 14523 |
| `-f:StripDebugSymbols=bool` | | `StripDebugSymbols` | 14525 |
| `-f:AbsoluteDebugPaths=bool` | | `AbsoluteDebugPaths` | 14527 |
| `-f:ExtraCppFlags=string` | | `ExtraCppFlags$` | 14529 |
| `-f:ExtraLinkerFlags=string` | | `ExtraLinkerFlags$` | 14531 |
| `-f:MaxCompilerProcesses=n` | 1..128 | `MaxParallelProcesses` | 14533 |
| `-f:TargetBits=32|64` | Target pointer width | `TARGET_BITS` | 14536 |
| `-f:GenerateLicenseFile=bool` | | `GenerateLicenseFile` | 14539 |
| `-f:UseSystemCompiler=bool` | Windows only in effect | `UseSystemMinGW` | 14541 |
| `-f:AutoIndent=bool` | Layout (for `-y`; also affects an IDE session started with it) | `IDEAutoIndent`, `DEFAutoIndent` | 14543 |
| `-f:AutoIndentSize=n` | 1..64 | `IDEAutoIndentSize` | 14546 |
| `-f:IndentSubs=bool` | | `IDEIndentSubs` | 14549 |
| `-f:AutoLayout=bool` | | `IDEAutoLayout`, `DEFAutoLayout` | 14551 |
| `-f:KeywordCapitals=bool`, `-f:KeywordLowercase=bool` | Keyword case; both given with equal state = CaMeL | `IDEAutoLayoutKwStyle` | 14554-14557, 14588-14596 |
| `-w` | Show warnings in CLI output | `ShowWarnings` | 14562 |
| `-q` | Quiet: no banner/progress, only warnings/errors | `QuietMode` | 14566 |
| `-m` | Monochrome console output | `MonochromeLoggingMode` | 14570 |
| `-u` | Hidden (CI): update all help pages from the wiki with console output; exit 0, or exit 1 with "Help update incomplete, ..." | `Help_Recaching = 2` | 14574-14581 |

Boolean values accepted by `-s`/`-f`: `true|on|yes|1|-1` and `false|off|no|0` (`ParseBooleanSetting&`, 14686-14707). Errors print a message plus the relevant help and `SYSTEM 1` (`CMDLineSettingsError`, 14613-14635). `-s:x=value` continues parsing (so it can be combined with a compile); a bare `-s:x` query exits (14506). If only settings switches were given and no file, the program exits (14609). `-y` without `-o` prints "Formatting requires specifying output file with -o option" and exits 1 (14600-14604).

Not present in this version: no `-n`, `-d`, `-g`, `-b` switches. **No "external editor mode" exists**: a grep of `source\` for "external editor" returned nothing, and no setting or switch makes the IDE watch or defer to an external editor. The closest features are `-l:<n>` (open a file at a line) and the headless compile/format modes below, which is what external editors would call.

### 6.2 Mode flags and flow

Declared at qb64pe.bas:126-128: `NoIDEMode, ConsoleMode, FormatMode, NoCCompileMode, QuietMode` (`_BYTE`), `MonochromeLoggingMode, ShowWarnings, ForceOptExpl`, `CMDLineSrcFile$, CMDLineOutFile$`. (`SettingsMode`, `CMDLineSwitch`, `PassedFileName$` are used only inside the parser; their declarations were not looked up.)

- The executable is built with `$CONSOLE` and `$SCREENHIDE` (qb64pe.bas:8, 11). After parsing: `ConsoleMode` -> `_DEST _CONSOLE`; otherwise `_CONSOLE OFF: _SCREENSHOW: _ICON` (420-426).
- `IF NoIDEMode THEN IDEAutoPosition = _FALSE: GOTO noide` (845) skips the entire IDE; otherwise `idemode = 1` (850) and the compiler is driven by the IDE through the message protocol documented at the top of ide_methods.bas.
- `noide:` (1109-1157): prints the version banner (unless quiet or already printed), prompts `COMPILE (.bas)>` if no file was given, appends `.bas`, resolves the source path and EXE target folder (`DefaultExeSaveFolder$`, or the source folder when `SaveExeWithSource`), then compiles. Progress: `[....] 100%` style in console mode, block characters in window mode (13457-13464).
- Exit codes: missing source -> "Cannot locate source file" and `SYSTEM 1` (console) / `END 1` (window) (1134-1138, 1683-1686). End of run (14228-14230): in window mode `END 1` if compilation failed or warnings were issued (keeps the window open); otherwise `SYSTEM 1` on C++ failure and `SYSTEM 0` on success. C++ failure prints "ERROR: C++ compilation failed." and "Check <compilelog> for details." (14209-14211); success prints `Output: <path>` unless quiet (14213-14217).
- `FormatMode`: formatted lines are collected during the main pass (12684-12690), end up in `tmpdir$ + "format.out"` and are copied to the `-o` path (13646-13651), then control jumps to `NoCCompile`.
- `NoCCompileMode`: skips the make/C++ stage (13494, 13935, 14166, 14194).
- `QuietMode` gates the banner and progress prints (1110, 1161, 3386, 13457, 13495, 14213).
- `ShowWarnings` / `MonochromeLoggingMode` are used by the CLI warning printer (28672-28695).
- `$NOPREFIX` in no-IDE mode is a hard error (2026).
- The IDE itself relies on these modes to build helper tools (`qb64pe -x ... -o ...`, section 4), so `-x`/`-o` are part of the product's internal contract.

---

## 7. Not determined / not verified

- Server-side meaning of `&qbide=1`, and whether a raw-wikitext endpoint would be accepted by the wiki (not visible in the repo).
- Exact trigger condition for writing `IDE_TopPosition` / `IDE_LeftPosition` (ide_methods.bas:1439-1440 seen; surrounding logic not read).
- Complete list of legal `LogMinLevel` / `LogScopes` / `LogHandlers` values (logging dialog near ide_methods.bas:16810 not read).
- Dialog names in table 2.5 are inferred from the keys written, not from reading each dialog.
- Internals of `idef1box$` and `iderestore$` beyond their prompts; the layout of `idet$` inside the undo file (owned by the ide_methods section).
- `QB45BIN.bas` decoding internals and `AddPREFIX.bas` tokenizer internals were only skimmed (headers and main flow), as requested.
- `qbjs-build.bas` lines 300-446 (download helper, dependency list) seen by signature only.
- The `libraries/` add-on (descriptors, LibraryExplorer) is not in this checkout; the descriptor format is inferred solely from the compiler code.
- `ini.bm` value parsing details (quotes/comments, lines 200-258 and `IniGetSection$`) not read in full.
- The fall-through observation in `BinaryFormatCheck%` (4.1) and the `ErrorColor` default oddity (2.4) come from reading only, not from running the program.
- `findWorkingTerminal$` (Linux terminal detection) not read.
- How `_IKW_` routines are registered inside the compiler was not traced; only the comments in `afterlastline.bm` were used.
- Whether `Help_IgnoreCache = 0` in `-u` mode really skips already cached pages was inferred from `Wiki$` logic, not run.

---

# Part F — Debugger (`$DEBUG` / vWatch)

*(Section numbers below are local to Part F.)*

Scope: how QB64-PE implements source-level debugging. All paths are relative to `..\QB64pe`. Abbreviations used for file references:

| Abbrev | File |
|---|---|
| `qb64pe.bas` | `source\qb64pe.bas` (compiler) |
| `ide_methods` | `source\ide\ide_methods.bas` (IDE) |
| `ide_global` | `source\ide\ide_global.bas` |
| `vwatch.bm` / `.bi` / `_stub.bm` | `internal\support\vwatch\...` (debuggee-side BASIC) |
| `libqb.cpp`, `qbx.cpp` | `internal\c\libqb.cpp`, `internal\c\qbx.cpp` (runtime) |

Line numbers were read from the working tree on the study date. Items I could not verify are flagged **[UNVERIFIED]** and collected in section 9.

Architecture in one paragraph: the debugger is an **in-process agent written in BASIC**. With `$DEBUG` active the compiler appends `vwatch.bm` (one 880-line `SUB vwatch`) to the user program and emits a call to it before every instrumented statement. That SUB is a TCP **client** which connects to the IDE (the TCP **host**) on localhost. All state (breakpoints, skip lines, stepping mode, watchpoints) lives in the debuggee; the IDE holds mirror copies and the variable metadata (names, types, UDT layouts) that the compiler left in memory in `usedVariableList()`. There is no symbol file: variable metadata never leaves the IDE process; the debuggee only gets "index N in the pointer table, add offset X, read S bytes, type string T".

---

## 1. Instrumentation (compiler side)

### 1.1 Turning `$DEBUG` on, and the recompile mechanism

The brief mentions `vWatchDesiredState` / `vWatchRecompileAttempts`. **These identifiers do not exist in this tree** (case-insensitive grep over `source\` returns nothing). They have been replaced by a generic "recompile-state variable" type:

- `TYPE RCStateVar` with fields `wanted`, `actual`, `locked`, `forced` — `source\utilities\statevars.bi:7-12`.
- `DIM SHARED vWatchOn AS RCStateVar` — `qb64pe.bas:176`.
- `SetRCStateVar` (`source\utilities\statevars.bas:16-21`): stores `wanted`; if `actual <> wanted` and not `locked`, sets the global `recompile = 1`.
- `ExecuteRCStateVar` (`statevars.bas:32-38`): at the start of each (re)compile copies `wanted` to `actual`; once non-zero it sets `locked`, so the value cannot flip back within the same full compile (this is what bounds the number of recompiles, replacing the old "attempts" counter).
- `ClearRCStateVar vWatchOn` at `fullrecompile:` — `qb64pe.bas:1222`; `ExecuteRCStateVar vWatchOn` right after `recompile:` — `qb64pe.bas:1235`, immediately followed by `vWatchVariable "", -1` (reset variable lists) at `:1236`.

Flow: the prepass meets the metacommand at `qb64pe.bas:2004-2008` (`IF temp$ = "$DEBUG" THEN SetRCStateVar vWatchOn, 1 : SetPreLET "_DEBUG_", "1"`). Because `actual` was 0 this sets `recompile = 1`. The flag is consumed at the top of the main compile loop (`qb64pe.bas:3340`, `IF recompile GOTO do_recompile`) and at `do_recompile:` (`qb64pe.bas:13001-13007`), which jumps back to `recompile:`; in IDE mode `iderecompile = 1` is also set so the IDE restarts feeding lines. On the second pass `vWatchOn.actual = 1` from line 1, so `vwatch.bi` can be auto-included *before* the first user line. Consequence: **every `$DEBUG` program is compiled at least twice** (prepass abort + restart).

The precompiler constant `_DEBUG_` is exposed for `$IF` (`qb64pe.bas:1718`, `:2006`).

The main pass only recognises the keyword for layout: `qb64pe.bas:3553-3559` (prints as `$Debug`; when compiling from the command line, `NoIDEMode`, it adds the warning "$DEBUG features only work from the IDE").

When compilation finishes the compiler tells the IDE which mode it is in: `qb64pe.bas:1076-1080` sends `CHR$(254)` ("launch debug interface") instead of `CHR$(6)` ("ready") after the program has been started. The IDE reacts at `ide_methods:797-836`.

A C global is always emitted so the runtime knows: `int32 vwatch=-1;` when on, `int32 vwatch=0;` when off — `qb64pe.bas:13094-13098` (see 2.4 for how the runtime uses it).

### 1.2 Auto-include of the support files

`autoIncludeManager:` (`qb64pe.bas:1725-1799`) writes a list of files to `internal\temp\autoinc.txt` and includes them:

| When | Condition | File | Line |
|---|---|---|---|
| before first user line | `vWatchOn = 1` (or editing `vwatch.bm` itself), and the program being edited is not `vwatch.bi` | `internal\support\vwatch\vwatch.bi` | `:1747-1751` |
| after last user line ("AtBottom") | `vWatchOn <> 0` and program name does not start with `vwatch` | `internal\support\vwatch\vwatch.bm` | `:1770-1771` |
| after last user line | `vWatchOn = 0` | `internal\support\vwatch\vwatch_stub.bm` | `:1772-1773` |

- `vwatch.bi` (19 lines) declares the shared state under `$CHECKING:OFF`: `vwatch_linenumber`, `vwatch_sublevel`, `vwatch_goto` (LONG); `vwatch_subname`, `vwatch_internalsubname`, `vwatch_callstack` (STRING); `REDIM SHARED vwatch_breakpoints(0) AS _BYTE`, `vwatch_skiplines(0) AS _BYTE`, `vwatch_stack(1000) AS STRING` (`vwatch.bi:3-7`). Lines 9-17 are dummy assignments to silence "unused variable" warnings.
- `vwatch_stub.bm` (6 lines) defines an empty `SUB vwatch ()` so that the name `vwatch` is *always* taken, in every program, debug or not. A user program defining its own `vwatch` gets "Name already in use (vwatch)", which `qb64pe.bas:14313-14318` rewrites into a plain "Syntax Error".
- A direct call is rejected: `qb64pe.bas:10867-10869` ("Cannot call SUB VWATCH directly").

The compiler refers to these BASIC variables by their **mangled C names**, hard-coded as string literals: `__LONG_VWATCH_LINENUMBER`, `__LONG_VWATCH_SUBLEVEL`, `__LONG_VWATCH_GOTO`, `__STRING_VWATCH_SUBNAME`, `__STRING_VWATCH_INTERNALSUBNAME`, `__STRING_VWATCH_CALLSTACK`, `__ARRAY_BYTE_VWATCH_BREAKPOINTS`, `__ARRAY_BYTE_VWATCH_SKIPLINES`, `__ARRAY_STRING_VWATCH_STACK`, and the SUB as `SUB_VWATCH`. The list `vWatchVariableExclusions$` (`qb64pe.bas:44-46`) keeps them (and all `_SUB_VWATCH_*` locals) out of the watchable-variable table (`:17465`) and out of the `CLEAR` statement's reset code (`:12753`, `:12778`, `:12792`) so that `CLEAR` does not wipe debugger state.

### 1.3 The per-statement line hook

The canonical hook text (emitted from ~14 places) is:

```c
*__LONG_VWATCH_LINENUMBER= <N>; SUB_VWATCH((ptrszint*)vwatch_global_vars,(ptrszint*)vwatch_local_vars); if (*__LONG_VWATCH_GOTO>0) goto VWATCH_SETNEXTLINE; if (*__LONG_VWATCH_GOTO<0) goto VWATCH_SKIPLINE;
```

`<N>` is the IDE source line (`linenumber`). Each emission is preceded by `vWatchAddLabel linenumber, 0`, which emits the C labels (see 1.4). Common condition: `vWatchOn = 1 AND inclinenumber(inclevel) = 0` (i.e. **only lines of the main file; `$INCLUDE`d code is never line-instrumented**) and normally `CheckingOn = 1`.

| Statement kind | Emission site (`qb64pe.bas`) | Notes |
|---|---|---|
| Generic statement | `:7226-7232` | emitted as `do{<hook>` ; the block is closed at `:12481-12488` by the event check `if(!qbevent)break;<vWatchErrorCall>evnt(N);}while(r);` |
| `IF` (block) | `:6689-6691` | after `S_n:;` label |
| `ELSEIF` | `:6647-6649` | |
| `END IF` | `:6772-6774` | |
| `SELECT CASE` | `:6793-6795` | |
| `CASE` | `:7038-7040` | |
| `END SELECT` | `:6919-6921` | |
| `WHILE` | `:6297-6299` | |
| `DO WHILE/UNTIL` | `:6357-6359` | hook is inside the C `while(...){` |
| plain `DO` | `:6364-6366` | `do{<hook>` |
| `LOOP WHILE/UNTIL` and plain `LOOP` | `:6399-6401`, `:6407-6409` | after `dl_continue_N:;` |
| `FOR` | `:6561-6563` | |
| `NEXT` | `:6247-6250` | after `fornext_continue_N:;`, inside the loop body |
| `END SUB/FUNCTION` | `:5964-5969` | at `exit_subfunc:;`, then `vWatchAddLabel 0,-1` |

Special (negative / zero) "line numbers" used as event codes; these are passed through the same SUB:

| Value | Meaning | Emitted at | Handled in `vwatch.bm` |
|---|---|---|---|
| `0` | program is ending | `xend` helper `:27251-27256` (used by `END` and by `closemain` `:17570`); `SYSTEM` `:9891-9896`; and inside `vWatchErrorCall$` when `stop_program` is set `:43` | `:115-122` |
| `-1` | runtime error pending on the last line | `vWatchErrorCall$` `:43` | `:123-129` |
| `-2` | SUB/FUNCTION entered | `:5818` | `:130-136` |
| `-3` | `STOP` statement: pause instead of quitting | `:9917-9919` (only when checking is on and in main file; otherwise real `end()` `:9921-9922`) | `:137-141` |
| `-4` | about to block for input (`INPUT`/`LINE INPUT` `:11226-11228`; `SLEEP` `:12391-12395`; `INPUT$()` function `:20238-20242`) | | `:142-146` |
| `-5` | input finished (`:11231-11233`, `:12400-12404`, `:12465-12470`) | | `:147-151` |

`vWatchErrorCall$` (`qb64pe.bas:43`) is spliced into the per-statement event check (`:3988-3989`, `:4047`, `:9860-9861`, `:9884-9885`, `:12482-12487`):

```c
if (stop_program) {*__LONG_VWATCH_LINENUMBER=0; SUB_VWATCH(...);};
if(new_error){bkp_new_error=new_error;new_error=0;*__LONG_VWATCH_LINENUMBER=-1; SUB_VWATCH(...);new_error=bkp_new_error;};
```

So the error notification runs with the error temporarily cleared (`bkp_new_error` is a runtime global, `qbx.cpp:321`), then the error is restored and the normal `evnt()` path proceeds (ON ERROR handler or the fatal error dialog). Note it is only reached `if(qbevent)`.

### 1.4 Labels, set-next-line and skip-line dispatch

`SUB vWatchAddLabel (this, lastLine)` — `qb64pe.bas:17517-17549`:

- For line `N` it emits `VWATCH_LABEL_N:;` *before* the hook (once per line; `prevLabel` guards duplicates for multi-statement lines) and records it in the byte-map string `vWatchUsedLabels` (`ASC(vWatchUsedLabels, N) = 1`).
- When moving to a *new* line it first emits `VWATCH_SKIPLABEL_<prev>:;` — i.e. the skip label of line P is placed at the end of line P's code, just before line N's label — and records it in `vWatchUsedSkipLabels`.
- `lastLine = -1` just flushes the pending skip label (end of scope).
- `firstLineNumberLabelvWatch` / `lastLineNumberLabelvWatch` bound the lines seen in the current scope (`:671-672`, reset per scope `:1578-1580`).

At the end of each SUB/FUNCTION (`qb64pe.bas:5972-6005`) the compiler emits two `switch` dispatchers, jumped over in normal flow:

```c
goto VWATCH_SKIPSETNEXTLINE;
VWATCH_SETNEXTLINE:;
switch (*__LONG_VWATCH_GOTO) {
    case N: goto VWATCH_LABEL_N; break;   // one per labelled line in this procedure
    default: *__LONG_VWATCH_GOTO=*__LONG_VWATCH_LINENUMBER; goto VWATCH_SETNEXTLINE;
}
VWATCH_SKIPLINE:;
switch (*__LONG_VWATCH_GOTO) {
    case -N: goto VWATCH_SKIPLABEL_N; break;
}
VWATCH_SKIPSETNEXTLINE:;
```

For the main module the same pair is emitted by `closemain` (`:17568-17585`), but the `case` lists come from two side files written to `internal\temp` (`tmpdir$`): `vw_main_dispatch.txt` and `vw_main_skip.txt` (opened `:3274-3275`, filled by `closeMainVwatchSection` `:17551-17566`, which is called when the first SUB starts `:5424-5428`), pulled in with `#include`.

Semantics:
- **Set next line**: `vwatch` sets `vwatch_goto = N` (`vwatch.bm:666`) and returns; the hook does `goto VWATCH_SETNEXTLINE`, the switch jumps to `VWATCH_LABEL_N`, which *re-runs the hook for line N* (so the debuggee pauses there again; `vw_setNextLine` defeats the "same line" early-out, `vwatch.bm:154-155`). Target must be in the **same C function**; an unknown target falls to `default`, which re-targets the current line (effectively a no-op). It is a raw C `goto`, so jumping into/out of loops or blocks is unchecked.
- **Skip line**: `vwatch` sets `vwatch_goto = -N` (`vwatch.bm:195`); the hook jumps to `VWATCH_SKIPLABEL_N`, i.e. the end of that line's code. If no skip label exists for the line (last line of a scope before flush, etc.), the `switch` has no `default` and simply falls through to `VWATCH_SKIPSETNEXTLINE` — in procedure scope that label is located after the exit-hook, so execution continues at procedure exit **[UNVERIFIED: behaviour for un-labelled skip targets was inferred from the emitted C, not tested]**.

### 1.5 SUB/FUNCTION entry and exit hooks (call stack)

Entry — `qb64pe.bas:5804-5820` (emitted for every procedure, including those in include files, and including `SUB_VWATCH` itself for the level counter):

```c
*__LONG_VWATCH_SUBLEVEL=*__LONG_VWATCH_SUBLEVEL+ 1 ;
qbs_set(__STRING_VWATCH_SUBNAME,qbs_new_txt_len("<display name>",len));
qbs_set(__STRING_VWATCH_INTERNALSUBNAME,qbs_new_txt_len("<SUB_xxx / FUNC_xxx>",len));
*__LONG_VWATCH_LINENUMBER=-2; SUB_VWATCH(...);
```

The display name is `subfuncoriginalname$` (e.g. `SUB MySub`), prefixed by `(file.bi, line) ` for procedures defined in an include (`:5807-5812`). The last three statements are skipped for `SUB_VWATCH`.

Exit — `:5970`: `*__LONG_VWATCH_SUBLEVEL=*__LONG_VWATCH_SUBLEVEL- 1 ;` after the `exit_subfunc:` hook.

Because `SUB_VWATCH` increments on entry and decrements on exit like every procedure, **inside `vwatch` the level is one higher than the caller's real depth**: main-module code calls it at level 1, a first-level SUB at level 2. That is why `SendCallStack` reports `vwatch_sublevel - 1` entries starting from `vwatch_stack(2)` (`vwatch.bm:719-732`).

### 1.6 Variable tables

**Compile-time metadata (stays in the IDE process).** `TYPE usedVarList` — `qb64pe.bas:130-138`:

| Field | Meaning | Set at |
|---|---|---|
| `id` | index into compiler `ids()` (`currentid`) | `:28591` |
| `linenumber`, `includeLevel`, `includedLine`, `includedFile` | where declared | `:28596-28607` |
| `scope` | `subfuncn` (numeric procedure index; main = 0) | `:28608` |
| `subfunc` | internal C procedure name (`""` for main, else `SUB_X`/`FUNC_X`) — this is the string the debuggee compares against | `:28609` |
| `localIndex` | slot number in `vwatch_global_vars[]` / `vwatch_local_vars[]` | `:28612` |
| `varType` | full type name from `id2fulltypename$` (e.g. `_UNSIGNED LONG`, `STRING * 10`, a UDT name) | `:28610` |
| `cname` | mangled C name (array subscript stripped) | `:28611` |
| `name` | BASIC name with type suffix; arrays get `()` appended | `:28619-28632` |
| `used` | set when the variable is referenced (drives "unused variable" warnings `:13466-13488`) | `:28663` |
| `watch` | selected in the Watch List | IDE |
| `isarray` | `id.arrayelements > 0` | `:28627-28632` |
| `displayFormat` | 0=DEC 1=HEX 2=BIN 3=OCT | IDE |
| `watchRange`, `indexes` | array index selection (text form / expanded form) | IDE |
| `elements`, `elementTypes`, `elementOffset` | UDT member selection (names / type names / byte offsets) | IDE |
| `arrayElementSize` | byte size of one element for UDT arrays | IDE |
| `storage` | packed `MKL$` slot numbers into `vWatchReceivedData$()` | IDE |
| `hashNext` | chain link for the djb2 hash on `cname` (`varListHashTable`, 2^20 buckets `:145-147`, `:28546-28556`) | `:28588` |

`manageVariableList` (`qb64pe.bas:28558-28666`): action 0 adds an entry; other actions mark it used. When adding it also looks the `cname` up in `backupVariableWatchList$` and restores the previous watch selection from `backupUsedVariableList()` — this is how watch selections survive a recompile within one IDE session. UDT element selections are restored only if `typeDefinitions$` (a packed record of all TYPE definitions, built at `:2306`, `:2487`, `:2493`) is byte-identical to `backupTypeDefinitions$` (`:28648-28658`).

**Run-time pointer tables (in the debuggee).** `SUB vWatchVariable (this$, action)` — `qb64pe.bas:17454-17515`, called from every variable-creation path in `dim2` (`:17757` … `:18917`, 16 call sites):

- action 0: skip excluded names; append `vwatch_global_vars[i] = &<cname>;` (main module) or `vwatch_local_vars[i] = &<cname>;` (procedure) to a pending string, and register in `usedVariableList` with that index. The pointer-table strings are only built when `$DEBUG` is on; `usedVariableList` is always built (comment `:17469-17471`).
- action 1 (at end of each procedure `:5957-5959` and end of main `:13108-13110`): writes to the per-scope data buffer `DataTxtBuf` (`internal\temp\maindata.txt` / per-procedure data file) the declaration `void *vwatch_local_vars[N];` plus the assignment lines; for the main module writes `void *vwatch_global_vars[N];` to `GlobTxtBuf` (global declarations) and the assignments to main's data section, plus a zero-length `vwatch_local_vars[0]` so main also has a (dummy) local table.
- action -1: reset.

Key consequences:
- Each table entry is the **address of the C variable**, and nearly all QB64 C variables are themselves pointers (`int32 *__LONG_X`, `qbs *__STRING_X`, `ptrszint *__ARRAY_...`). Hence the debuggee does *two* dereferences (`vwatch.bm:315-316`): table slot -> address of pointer variable -> data address / qbs struct / array descriptor.
- "Global" means "declared in the main module", not `SHARED`. "Local" means declared in a procedure (incl. STATICs and parameters **[UNVERIFIED: which parameter kinds pass through `vWatchVariable` was not traced]**).
- `vwatch_local_vars` is a C local array of each procedure, so only the **currently executing** procedure's locals are reachable; the hook passes the current function's array. The debuggee enforces this by comparing the requested scope name with the top of its call stack (`vwatch.bm:307-312`).

### 1.7 Restrictions and warnings

| Restriction | Where |
|---|---|
| `$CHECKING:OFF` blocks get no line hooks (`CheckingOn = 0` suppresses almost every emission); warning "$DEBUG features won't work in these blocks" (main file only) | `qb64pe.bas:3561-3567` |
| Code in `$INCLUDE` files is never line-instrumented (`inclinenumber(inclevel) = 0` test in every hook); only procedure entry is recorded | all hook sites |
| `CHAIN` and `RUN`: warning "Feature incompatible with $DEBUG mode" (compile continues) | `:9758-9767` |
| Compiling with `-x`/`-c` (no IDE): warning "$DEBUG features only work from the IDE"; instrumentation is still generated | `:3555-3557` |
| `SUB vwatch` may not be called or redefined | `:10867`, `:14316` |
| ON ERROR: no explicit restriction found; the error hook (`-1`) fires before the handler runs, so the IDE reports the error even if the program handles it **[UNVERIFIED by running]** | `:43` |
| Console programs: no compiler-side restriction found; `_WINDOWHANDLE` may be 0 so the `hwnd` message may never be sent (`vwatch.bm:48-52`, `:107-113`) **[UNVERIFIED]** | |
| Requires the sockets runtime: `_OPENCLIENT` in `vwatch.bm` pulls in the network dependency (the `_SOCKETS_` / `DEPENDENCY_SOCKETS` logic at `qb64pe.bas:1719-1722`) **[UNVERIFIED: exact trigger not traced]** | |
| "Run without saving EXE" path (`NoExeSaved`): the compiler runs the program with a blocking `SHELL` and then replies `CHR$(6)` instead of `CHR$(254)`, so no debug session is started for that run mode | `qb64pe.bas:1038-1061` |

---

## 2. Connection setup

### 2.1 Roles

- **IDE = TCP host (listener). Debuggee = TCP client.** Transport: QB64's own `_OPENHOST` / `_OPENCONNECTION` / `_OPENCLIENT`, with `GET #` / `PUT #` on the handle (non-blocking reads).
- One debuggee per IDE instance; one listening socket per IDE instance, kept open for the life of the IDE once created.

### 2.2 Port selection

- Setting `idebaseTcpPort` (`source\ide\config\cfg_global.bas:30`), read from the `[DEBUG SETTINGS]` section, key `BaseTCPPort`, default 9000 (`source\ide\config\cfg_methods.bas:312-314`).
- Actual port: `hostport$ = _TOSTR$(idebaseTcpPort + tempfolderindex)` — `ide_methods:778`. `tempfolderindex` is the IDE instance number (the same index that selects `internal\temp`, `internal\temp2`, ...), so concurrent IDE instances use 9001, 9002, ...
- The listener is opened lazily the first time the IDE sees a compiled program with `$DEBUG` active (`ide_methods:775-788`): `ENVIRON "QB64DEBUGPORT=" + hostport$` then `host& = _OPENHOST("TCP/IP:" + hostport$)`. A static `attemptToHost` flag prevents retrying on every pass. If it failed, `DebugMode` retries once when a session starts (`ide_methods:7109-7119`) and otherwise reports "Failed to initiate debug session. / Cannot receive connections on port N. Check your firewall permissions." (the message prints `idebaseTcpPort`, not the real port).
- Debug menu "Set Base TCP/IP Port Number..." (`ide_methods:360`) calls `ideSetTCPPortBox` (`ide_methods:16277-16285`: digits-only input box; 0 becomes 9000; saved immediately). The menu handler (`ide_methods:6480-6492`) closes the old `host&`, clears `attemptToHost`, sets `changingTcpPort` (so a failure is reported in a message box, `:783-786`) and forces a recompile (`idechangemade = 1`).

### 2.3 How the port reaches the debuggee

Through the **environment variable `QB64DEBUGPORT`** set in the IDE process (`ide_methods:779`) and inherited by the child started with `SHELL _DONTWAIT ExecuteLine$` (`qb64pe.bas:1063`). The port is *not* compiled into the executable. In the debuggee: `vw_ideport$ = ENVIRON$("QB64DEBUGPORT")` (`vwatch.bm:686`).

### 2.4 Debuggee connect and the special `QB64IDE:` protocol name

`Connect:` (`vwatch.bm:684-696`) runs on the **first** call of `vwatch` (the first instrumented line or first SUB entry):

- If the env var is empty: `vw_bypass = _TRUE` and return. From then on every hook returns at `vwatch.bm:35`. This is what happens when a `$DEBUG` executable is **run outside the IDE**: it runs normally, with only the hook-call overhead and no delay.
- Otherwise loop `_OPENCLIENT("QB64IDE:" + port + ":localhost")` at `_LIMIT 30` until success, ESC pressed in the debuggee (`_KEYHIT = 27`), or `vw_timeout` = 10 s (`vwatch.bm:40`). Failure sets `vw_bypass`. (A `$DEBUG` exe started from a shell that inherited the variable, with no IDE listening, therefore stalls up to 10 s at startup.)

`QB64IDE` is a reserved pseudo-protocol in the runtime: `libqb.cpp:21573-21577` accepts it in place of `TCP/IP` only while the C global `vwatch == -1` (a `$DEBUG` build that has not yet connected), then stores the resulting handle in `vwatch` (`libqb.cpp:21618-21619`). The runtime uses that to **exempt the debug socket from the close-all form of `CLOSE`**: `libqb.cpp:13273-13279`. `unlockvWatchHandle` (`libqb.cpp:5883-5886`) resets `vwatch` to -1 so `vwatch.bm` can really close it. `vWatchHandle()` (`libqb.cpp:5888`) is declared in `qbx.cpp:42`; its other uses were not traced **[UNVERIFIED]**.

### 2.5 Handshake

IDE side `ide_methods:7105-7238`; debuggee side `vwatch.bm:39-105`.

1. IDE shows "Entering $DEBUG mode (ESC to abort)..." and polls `_OPENCONNECTION(host&)` at `_LIMIT 100` for up to `timeout# = 10` s (`:6987`, `:7121-7141`). ESC or timeout gives "Debug session aborted." (plus "Connection timeout.") and returns.
2. Debuggee connects and sends `me:<COMMAND$(0)>` and, if it already has a window, `hwnd:<_OFFSET>` (`vwatch.bm:46-52`).
3. IDE ("Handshaking...", another 10 s window) waits for `me`; it strips the path from both the received name and `lastBinaryGenerated$` and compares the bare file names (`:7164-7183`). On mismatch it sends `vwatch:file mismatch` and closes; the debuggee then closes and goes to bypass (`vwatch.bm:57-62`). This guards against some *other* `$DEBUG` program connecting to the port.
4. IDE sends, in order: `vwatch:ok`, `hwnd:<IDE window handle>`, `line count:<MKL$(iden)>`, optionally `breakpoint count:` + `breakpoint list:`, optionally `skip count:` + `skip list:`, and finally `break` (Start Paused) or `run` (`:7186-7233`).
5. Debuggee processes these in a **busy loop without `_LIMIT`** (`vwatch.bm:54-104`) until `run` or `break`. `line count` sizes the two byte arrays. A list whose length does not match the announced count produces `quit:Communication error.` `run` returns immediately unless the current line already has a breakpoint.

There is no protocol version negotiation and no debuggee-side timeout after connecting (if the IDE never sends `run`/`break`, the debuggee spins at 100% CPU).

---

## 3. Wire protocol

### 3.1 Framing

Identical in both directions: a 4-byte little-endian LONG holding `LEN(body)`, followed by the body `<command>[":"<payload>]`.

- Sender: `cmd$ = MKL$(LEN(cmd$)) + cmd$ : PUT #h, , cmd$` (IDE `ide_methods:8405-8417`; debuggee `vwatch.bm:734-738`).
- Receiver: non-blocking `GET #h, , temp$` appended to a persistent buffer; if the buffer holds a complete message, pop it and split at the **first** `:` into `cmd$` / `value$`; otherwise return `cmd$ = ""` (IDE `ide_methods:8377-8403`; debuggee `vwatch.bm:698-717`). At most **one message is consumed per call**.
- Command names are plain ASCII text containing spaces. Payloads are raw binary built with `MKL$` (4-byte LONG), `MKI$` (2-byte INTEGER), `_MK$(_BYTE,..)`, `_MK$(_OFFSET,..)` (4 or 8 bytes depending on bitness) and raw strings. No escaping is needed because of the length prefix. No checksums, sequence numbers or request ids (replies are correlated through the `storage` slot number that is echoed back).
- **Bug in both receivers**: the completeness test is `LEN(buffer$) >= cmdsize`, but a full frame needs `cmdsize + 4` bytes. If a read ends 1-4 bytes short of a frame boundary, the message is extracted truncated and the stream desynchronises (`ide_methods:8390`, `vwatch.bm:703`). Localhost delivery of small frames hides this in practice.

### 3.2 The variable descriptor

Four command families share a positional binary descriptor, parsed by `GetBytes` (`vwatch.bm:740-744`):

| # | Field | Encoding | `get * var` | `set * address` | `set/clear * watchpoint` |
|---|---|---|---|---|---|
| 1 | index into IDE `usedVariableList` (opaque to debuggee, echoed back) | LONG | yes | yes | yes |
| 2 | isArray | BYTE | yes | yes | yes |
| 3 | declaration line of the variable (arrays are not read until execution is past it) | LONG | yes | sent as 0 | yes |
| 4 | `localIndex` (slot in pointer table) | LONG | yes | yes | yes |
| 5 | byte length of field 6 (4 x dimensions) | LONG | yes | yes | yes |
| 6 | array indexes, one LONG per dimension | bytes | yes | yes | yes |
| 7 | `arrayElementSize` (0 = use varSize) | LONG | yes | yes | yes |
| 8 | isUDT flag | LONG | **absent** | yes | yes |
| 9 | UDT element ordinal (0 = none) | LONG | yes | sent as 0 | yes |
| 10 | element byte offset | LONG | yes | yes | yes |
| 11 | varSize in bytes (0 = ignore the request) | LONG | yes | yes | yes |
| 12 | storage slot (index into IDE `vWatchReceivedData$()`) | LONG | yes | yes | yes |
| 13 | scope: INTEGER length + internal sub name (`SUB_X`); empty for main | I+str | yes | yes | yes |
| 14 | type name: INTEGER length + string | I+str | yes | yes | yes |
| 15 | new value (set) / condition expression (watchpoint): INTEGER length + string | I+str | - | yes | yes |

Parsed at `vwatch.bm:276-302` (get; field 8 is read only when re-parsing a stored watchpoint, `:284-288`), `:448-469` (set), `:604-628` (watchpoint). Built at `ide_methods:8247-8265` (get), `:7869-7883` (set), `:7939-7953` (watchpoint).

### 3.3 Commands, IDE -> debuggee

"Init" = handled in the handshake loop `vwatch.bm:54-104`; "Poll" = handled in the per-line poll while running, `vwatch.bm:158-179`; "Paused" = handled in the paused loop `vwatch.bm:226-678`. "Sent at" lines are in `ide_methods`.

| Command | Payload | Semantics | Sent at | Handled at (`vwatch.bm`) |
|---|---|---|---|---|
| `vwatch` | `ok` or `file mismatch` | handshake verdict; anything but `ok` closes and bypasses | `:7186`, `:7176` | Init `:57-62` |
| `hwnd` | `_OFFSET` | IDE window handle (for `SetForegroundWindow`) | `:7188` | Init `:80-81` |
| `line count` | LONG | number of source lines; sizes breakpoint/skip arrays | `:7190` | Init `:63-65` |
| `breakpoint count` | LONG | number of entries that follow | `:7202` | Init `:66-67` |
| `breakpoint list` | n x LONG | line numbers | `:7204` | Init `:68-79` |
| `skip count` | LONG | | `:7217` | Init `:82-83` |
| `skip list` | n x LONG | | `:7219` | Init `:84-95` |
| `run` | none | resume free-running | `:7229`, `:7983` (F5) | Init `:96-99`; Paused `:229-233` |
| `break` | none | pause at the next instrumented line | `:7225`, `:7750` (F4 while running), `:8012` (F7 while running), `:8040` (F8 while running, and before the context menu) | Init `:100-102`; Poll `:160-164` |
| `step` | none | step into (run to next hook) | `:8016` | Paused `:240-243` |
| `step over` | none | | `:8032` | Paused `:244-249` |
| `step out` | none | | `:7996` | Paused `:250-255` |
| `run to line` | LONG | run until that line's hook | `:8131` | Paused `:234-239` |
| `set next line` | LONG | jump to line, stay paused | `:8144` | Paused `:662-667` |
| `set breakpoint` | LONG | also clears the skip flag of that line | `:7590`, `:8054` | Poll `:165-167`; Paused `:262-264` |
| `clear breakpoint` | LONG | | `:7586`, `:8056` | Poll `:168-169`; Paused `:265-266` |
| `set skip line` | LONG | also clears the breakpoint of that line | `:7581`, `:8157` | Poll `:170-172`; Paused `:668-670` |
| `clear skip line` | LONG | | `:7577`, `:8158` | Poll `:173-174`; Paused `:671-672` |
| `clear all breakpoints` | none | | `:8072` | Poll `:175-176`; Paused `:267-268` |
| `clear all skips` | none | | `:8067` | Poll `:177-178`; Paused `:269-270` |
| `call stack` | none | request the stack | `:8080` | Paused `:271-273` |
| `current sub` | none | request the current scope name | **never sent by the IDE** (dead handler) | Paused `:659-661` |
| `get global var` / `get local var` | descriptor without fields 8 and 15 | read a value; reply is `address read` | `:8241-8265` | Paused `:274-446` |
| `set global address` / `set local address` | full descriptor + value bytes | write a value; no reply | `:7785-7883` | Paused `:447-592` |
| `set global watchpoint` / `set local watchpoint` | full descriptor + expression | add or replace a watchpoint | `:7916-7953` | Paused `:602-658` |
| `clear global watchpoint` / `clear local watchpoint` | same | remove it | same | same |
| `clear last watchpoint` | none | remove the watchpoint that fired last | `:8197` | Paused `:593-601` |
| `free` | none | detach: close socket, bypass forever, program keeps running | `:7706` (ESC, "Exit $DEBUG mode", IDE window close) | Paused `:256-261` only |

Important asymmetry: **most commands are honoured only while the debuggee is paused.** While it is free-running, the per-line poll understands only `break` and the breakpoint/skip edits; anything else popped there (for example `free`, `run to line`, `call stack`) is silently discarded. In particular, ESC in the IDE while the program runs sends `free`, the IDE closes its end and leaves debug mode, and the debuggee drops the command and keeps polling a dead socket for the rest of its life **[inferred from code; UNVERIFIED by running]**.

### 3.4 Commands, debuggee -> IDE

The IDE's main dispatch is `ide_methods:8166-8367`. "Sent at" lines are in `vwatch.bm`.

| Command | Payload | Semantics | Sent at | Handled at (`ide_methods`) |
|---|---|---|---|---|
| `me` | `COMMAND$(0)` (exe path) | identify | `:46` | handshake `:7164-7182` |
| `hwnd` | `_OFFSET` | debuggee window handle; sent at connect, or later once a window exists | `:50`, `:110` | `:8271-8272` |
| `line number` | LONG line | paused before this line | `:215-220` | `:8169-8270` |
| `breakpoint` | LONG line | paused at a breakpoint | `:216-220` | same |
| `watchpoint` | LONG varIndex, LONG len, index bytes, LONG elementOffset, INTEGER len, expression, then the LONG line appended | paused because a watchpoint matched | `:189`, `:219` | same (`:8171-8200`; the line is taken as `RIGHT$(value$, 4)`) |
| `current sub` | internal name of the current procedure (empty in main) | always follows the three above; also the reply to `current sub` | `:221-222`, `:660` | `:8299-8305` |
| `address read` | LONG varIndex, LONG arrayIndex, LONG element, LONG storage, raw bytes | reply to `get * var` | `:380`, `:445` | `:8273-8298` |
| `call stack size` | LONG count | precedes `call stack` | `:728` | `:8092-8095` (requested), `:8348-8366` (unsolicited) |
| `call stack` | entries separated by `CHR$(0)` | | `:730` | `:8099-8101`, `:8360-8363` |
| `error` | LONG line | runtime error on the last executed line (unsolicited, preceded by the call stack) | `:126` | `:8317-8327` |
| `enter input` | LONG line | about to block in INPUT / LINE INPUT / SLEEP / INPUT$ | `:144` | `:8328-8338` |
| `leave input` | none | | `:149` | `:8339-8347` |
| `quit` | reason text (`Program ended.`, `Communication error.`) | session over | `:117`, `:70`, `:86` | `:8306-8316` |

Note on `address read`: its second field is the debuggee's `vw_arrayIndex` scratch variable (the last dimension's scaled index), not a meaningful value; the IDE parses but ignores it (`:8275`). Only `storage` matters.

---

## 4. Debuggee-side state machine (`vwatch.bm`)

Everything is one SUB with STATIC state (`vwatch.bm:4-8`):

| Variable | Role |
|---|---|
| `vw_bypass` | permanent "debugger off" switch (no env var, connect failure, mismatch, `free`, program end); checked first (`:35`) |
| `vw_ideHost` | socket handle; 0 = not yet connected |
| `vw_pauseMode` | "pause at the next hook" |
| `vw_stepOver`, `vw_startLevel` | step over/out: keep running while `vwatch_sublevel > vw_startLevel` |
| `vw_runToLine` | target line, 0 = none |
| `vw_setNextLine` | allows re-entry on the same line after a set-next-line jump |
| `vw_lastLine` | last line seen (dedupe for multi-statement lines; also the line reported for errors, input and call-stack entries) |
| `vw_buffer$` | receive buffer |
| `vw_globalWatchpoints$`, `vw_localWatchpoints$`, `vw_lastWatchpoint$` | watchpoint stores |
| `vwatch_breakpoints()`, `vwatch_skiplines()` | `_BYTE` arrays indexed by line number (shared, `vwatch.bi:5-6`) |
| `vwatch_stack()` | call-stack strings indexed by sublevel |

### 4.1 Per-hook control flow while running

For every instrumented statement (`vwatch.bm:35-213`):

1. `vw_bypass` set: return.
2. `vwatch_goto = 0`.
3. First call: connect and handshake (section 2).
4. Late `hwnd` send if the window appeared after connecting (`:107-113`).
5. Event codes 0 and -1 .. -5 are handled and return (`:115-152`).
6. **Same line as last time: return** (`:154`). Granularity is therefore the *line*: `a = 1: b = 2` stops once. Side effect: a loop written entirely on one line never reaches the poll and cannot be paused or broken into.
7. **One socket poll**: `GOSUB GetCommand` = one non-blocking `GET #`, a string concatenation and parsing (`:158`, `:698-717`); at most one pending command is handled.
8. If any watchpoint exists, **all** are evaluated now (`:181-192`), each one re-parsing its descriptor and reading memory. A hit pauses with a `watchpoint:` message.
9. Skip-line check: `vwatch_goto = -line` and return (`:195`). This precedes the breakpoint test; setting one flag clears the other, so a line is never both.
10. Step over/out filter (`:197-202`): while deeper than `vw_startLevel`, keep running (unless the line has a breakpoint); when back at exactly the start level, convert to pause. Step out uses `vw_startLevel = sublevel - 1`. The comparison is `=` rather than `<=`.
11. Run-to-line filter (`:204-209`). While a run-to-line is active, breakpoints on other lines are **ignored** (it returns before the breakpoint test).
12. No breakpoint on this line and not `vw_pauseMode`: return (`:211-213`).
13. Otherwise enter the paused state.

**Cost when free-running**: every source line performs a C call into a large BASIC SUB (about 40 locals to set up), a non-blocking socket read with a string append, a string `SELECT CASE`, three array lookups and several flag tests. There is no throttling (no "every N lines" or time-based poll); the poll happens once per line change. With watchpoints set, add a full variable fetch per watchpoint per line.

### 4.2 Paused state

Entry (`:215-224`): stop timers (`vwatch_stoptimers` is the runtime's `stop_timers`, which sets `ontimerthread_lock = 1` and **spins until the timer thread acknowledges**, `qbx.cpp:1244-1248`), send `line number:` / `breakpoint:` / `watchpoint:` plus the line, send `current sub:`, and bring the IDE window to the front with `set_fg vw_ideHwnd` (`SetForegroundWindow`; **Windows only**, `libqb.cpp:12801-12805` is an empty function elsewhere).

Loop (`:226-678`): dispatch the pending command, then `GetCommand`, then `_LIMIT 100` (so at most about 100 commands per second; fetching N watched values takes at least N/100 s). Exits:

| Command | New state | Extra |
|---|---|---|
| `run` | pause off, stepOver off | `_KEYCLEAR`, restart timers |
| `run to line` | pause off, target set | same |
| `step` | pause on | returns immediately; **timers are not restarted** for single steps |
| `step over` | pause on, stepOver on, startLevel = current | `_KEYCLEAR`, restart timers |
| `step out` | same, with startLevel = current - 1 | same |
| `set next line` | pause on, `vw_setNextLine`, `vwatch_goto = N` | returns into the C dispatcher |
| `free` | close, bypass | restart timers |
| runtime `stop_program` flag set (`stop_program_state&`, `libqb.cpp:5915`; e.g. the user closed the debuggee window) | bypass, leave loop (`:227`), `_KEYCLEAR`, restart timers (`:680-682`) | no `quit` message on this path |

While paused the program's main thread is blocked inside `vwatch`; `_KEYCLEAR` on resume discards keystrokes typed in the meantime.

### 4.3 Program end, errors, STOP, input, SUB entry

- **END / SYSTEM / end of main / close request** (`vwatch_linenumber = 0`, `:115-122`): sends the call stack and `quit:Program ended.`, closes the socket and sets bypass.
- **Runtime error** (`-1`, `:123-129`): sends the call stack and `error:<last line>`, raises the IDE window and returns. It does **not** pause; the runtime's normal error handling continues (ON ERROR handler, or the unhandled-error dialog). The IDE marks itself paused (`ide_methods:8327`) although the debuggee is not in its paused loop, so value requests are not answered until a real pause occurs.
- **STOP** (`-3`, `:137-141`): sets pause mode so that the *next* line breaks; the program is not terminated.
- **INPUT / LINE INPUT / SLEEP / INPUT$** (`-4` / `-5`): bracketing notifications so the IDE can explain why a requested pause has not happened yet and hand focus to the debuggee.
- **SUB entry** (`-2`, `:130-136`): stores `internalname,displayname, line N` in `vwatch_stack(vwatch_sublevel)` (grown by 1000 when needed). No socket I/O.

### 4.4 Reading and writing memory

Read (`get * var`, `vwatch.bm:274-446`):

1. `address = table + pointerSize * localIndex`, then two `_MEMGET(..., _OFFSET)` dereferences (`:304-316`). For locals the scope name must equal the internal name at the top of the call stack, otherwise the request is silently dropped (no reply; the IDE prints `<out of scope>` from its own knowledge).
2. Arrays (`:318-391`): refuse if execution is not yet past the DIM line; for each dimension call the runtime's `check_lbound` / `check_ubound` (`qbx.cpp:436-452`; wrappers around `func_lbound/ubound` that clear any error), refuse if out of range, and accumulate a linear index (first dimension varies fastest); read the first pointer-sized word of the array descriptor as the data base address; element stride is `arrayElementSize` if non-zero, else `varSize` (fixed strings: N; variable strings: pointer size; `_FLOAT`: forced to 16). `_BIT` arrays are read with `call_getbits` / `call_getubits` (`qbx.cpp:454-460`) and returned as an 8-byte integer.
3. Add `elementOffset` (UDT member) and copy `varSize` bytes with `_MEM` / `_MEMCOPY` (`:393-398`).
4. Strings (`:400-438`): fixed-length strings inside arrays or UDTs are inline. Everything else is a `qbs`: for array elements and UDT members first load the `qbs*`, then read `pointerSize + 4` bytes of the struct — this **assumes `qbs` begins with `{ char *chr; int32 len; ... }`** — and finally copy `len` bytes from `chr`. Scalar strings arrive already pointing at the struct because the IDE asked for `pointerSize + 4` bytes (`ide_methods:8478`).
5. `_FLOAT`: the buffer is padded with 16 zero bytes (`:439-441`).
6. Reply with `address read:` (`:445`).

Write (`set * address`, `:447-592`) mirrors this. Variable strings are resized through the runtime's `set_qbs_size` (`internal\c\libqb\include\qbs.h:123-125`, i.e. `qbs_set(target, func_space(len))`) and then overwritten in place; fixed strings are space-padded or truncated; bit arrays use `call_setbits` (`qbx.cpp:462-464`). No scope check is made for `set local address` (`:471-475`); it trusts the IDE.

All of this runs under `$CHECKING:OFF`, so a wrong descriptor crashes the debuggee instead of raising a BASIC error.

---

## 5. IDE side: `SUB DebugMode` (`ide_methods:6976-8512`)

### 5.1 Entry, exit and the `IdeDebugMode` re-entry codes

The IDE main routine (`ide2`) enters debugging when the compiler returns `CHR$(254)` (`ide_methods:797-836`): it sets `IdeDebugMode = 1`, empties `vWatchReceivedData$()` (`:801`), closes the help pane, redraws, disables window resizing (`_RESIZE OFF`, `:817`) and calls `DebugMode`. `DebugMode` is a *modal* loop: the normal IDE event loop does not run during a session, so the normal menu bar is replaced by the text "$DEBUG MODE ACTIVE" (`:6992-6994`), and editing is impossible.

To reuse IDE facilities (the contextual menu, dialogs), `DebugMode` returns to `ide2` and is later re-entered; the shared LONG `IdeDebugMode` (`ide_global:23`) carries the reason. STATIC locals (`PauseMode`, `buffer$`, `currentSub$`, `debuggeehwnd`, `panelActive`, `vWatchPanel`, `:6977-7002`) preserve the session across re-entries. Dispatch is the `SELECT EVERYCASE` at `:7033-7103`; values above 1 first refresh the status area and then `GOTO` straight to a label inside the main loop.

| Value | Meaning | Set at | Re-entry target |
|---|---|---|---|
| 0 | not debugging | `:824` | |
| 1 | start a new session (on entry) / session finished cleanly (on return) | `:799`; reset by every re-entry case | full init `:7034-7073`, then connect |
| 2 | returned to `ide2` to show the right-click contextual menu; also re-entered with 2 when the menu is dismissed | `:7300`; `:4820`, `:4833`, `:4849` | `returnFromContextMenu` `:7303` |
| 3 | Call Stack | `:6337` | `requestCallStack` |
| 4 | Continue | `:6349` | `requestContinue` |
| 5 | Step Out | `:6354` | `requestStepOut` |
| 6 | Step Over | `:6364` | `requestStepOver` |
| 7 | Step Into | `:6359` | `requestStepInto` |
| 8 | Run To This Line (uses `idecy`) | `:6369` | `requestRunToThisLine` |
| 9 | Exit $DEBUG mode (also when the IDE window is closed while the menu is open, `:4810-4812`) | `:6374` | `requestQuit` |
| 10 | Toggle Breakpoint | `:6380` | `requestToggleBreakpoint` |
| 11 | Clear All Breakpoints | `:6419` | `requestClearBreakpoints` |
| 12 | Toggle Skip Line (uses `idecy`) | `:6431` | `requestToggleSkipLine` |
| 13 | Set Next Line (uses `idecy`) | `:6495` | `requestSetNextLine` |
| 14 | SUBs dialog | `:5807` | `requestSubsDialog` |
| 15 | Unskip All Lines | `:6470` | `requestUnskipAllLines` |
| 16 | Watch List | `:6285` | `requestVariableWatch` |

After `DebugMode` returns (`ExitDebugMode:`, `:819-836`): `_CONSOLE OFF` if the console watch output is enabled; value 1 means the session is over (reset `IdeDebugMode`, `idefocusline`, `debugnextline`); value 2 builds the debug contextual menu (`IdeMakeContextualMenu`, `:19473-19505`) and jumps to the menu loop. If `ide2` is re-entered while `IdeDebugMode <> 0` (an internal IDE error during debugging), it jumps to `ExitDebugMode` (`:790-795`).

The contextual menu while debugging offers: Continue F5, Step Out F6, Step Into F7, Step Over F8, Set Next Line Ctrl+G, Run To This Line Ctrl+Shift+G, Toggle Breakpoint F9, Clear All Breakpoints F10, Toggle Skip Line Ctrl+P, Unskip All Lines Ctrl+F10, SUBs... F2, Watch List... F4, Call Stack... F12, Exit $DEBUG mode ESC (`:19473-19505`). A right-click while the program is running first sends `break` (`:7301`).

### 5.2 Event loop

`DO ... _LIMIT 100 ... LOOP` (`:7240-8371`). Each iteration:

1. `_EXIT` (window close): set `ideexit = 1` and quit the session (`:7241`).
2. Drain mouse input; the wheel scrolls the watch panel if the pointer is over it, else the code view by 3 lines (`:7245-7261`).
3. Right button: on the watch panel opens the Watch List dialog; on the code area moves the cursor line and returns with code 2 for the contextual menu (`:7275-7309`).
4. Left button: watch-panel hit testing (close button, scrollbars, resize corner, drag, double-click), code scrollbars, the button row, the line-number gutter (toggle breakpoint, or with Shift toggle skip line, `:7567-7596`), click in text to move the cursor line, click on the Find box (`:7311-7637`).
5. Focus status: when the IDE window is not focused the status area says "Set focus to the IDE to control execution" (`:7640-7655`).
6. One `_KEYHIT` (`:7658-8164`).
7. **One** `GetCommand` and dispatch of at most one incoming message (`:8166-8367`).
8. Redraw the button row if focused.

Because only one message is consumed per 10 ms iteration, a pause with N watched values takes at least N x 10 ms on the IDE side as well.

### 5.3 Key bindings inside DebugMode

| Key | Action | Line |
|---|---|---|
| Up / Down (Ctrl = scroll view) | move cursor line | `:7660-7683` |
| PgUp / PgDn | page | `:7684-7695` |
| Ctrl+Home / Ctrl+End | first / last line | `:7696-7703` |
| ESC | send `free`, close socket, "Debug session aborted." | `:7704-7714` |
| F2 | SUBs dialog (navigation only) | `:7715-7722` |
| Ctrl+F, Ctrl+F3 | Find dialog; F3 = find again (Shift = backwards) | `:7723-7746` |
| F4 | Watch List dialog. If running: send `break` first and open the dialog when `current sub` arrives (`estabilishingScope`) | `:7747-7977`, `:8299-8305` |
| F5 | `run`; dims the IDE colours (`DarkenFGBG(1)`); gives focus to the debuggee window | `:7978-7989` |
| F6 | `step out` (only when paused and `currentSub$` is not empty, else "Not inside a sub/function.") | `:7990-8007` |
| F7 | paused: `step`; running: `break` | `:8008-8027` |
| F8 | paused: `step over`; running: `break` | `:8028-8046` |
| F9 | toggle breakpoint on the cursor line (**only when paused**) | `:8047-8061` |
| F10 | clear all breakpoints; Ctrl+F10 = unskip all lines | `:8062-8075` |
| F12 | call stack (only when paused) | `:8076-8120` |
| Ctrl+G | "Set Next Line" line-number box, then `set next line` | `:8139-8146` |
| Ctrl+Shift+G | "Run To Line" box, then `run to line` | `:8124-8137` |
| Ctrl+P | "Skip Line" box, then toggle skip | `:8149-8163` |

The status area shows eight clickable pseudo-buttons, `<F4 = Add Watch>` ... `<F12 = Call Stack>` (`:7008-7031`, click handling `:7554-7565`).

Outside a session the same keys act on the IDE's stored state (`ide_methods:1697-1739`): F4 opens the Watch List, F7/F8 = Start Paused, F9 toggles a breakpoint, F10 clears breakpoints, Ctrl+F10 unskips, F12 shows the call stack captured in the last run; clicking the line-number column toggles a breakpoint (Shift-click: skip line) (`:3231-3245`). If the program has no `$DEBUG`, these actions offer to insert `$Debug` as line 1 ("Insert $DEBUG metacommand?" Yes / No / Don't show this again), governed by the `AutoAddDebugCommand` setting (`:6246-6280`, `:6289-6313`, `:6385-6408`, `:6436-6459`; setting in `cfg_methods.bas:325-335`; menu toggle `:5405-5416`). "Start Paused" sets `startPaused`, which makes the handshake end with `break` instead of `run` (`:7224-7233`); `startPausedPending` (`ide_global:15`) carries the request across the recompile that inserting `$Debug` requires (`:1401-1402`, `:6267`).

Debug menu items are defined at `ide_methods:330-362`.

### 5.4 Current-line display

On `line number` / `breakpoint` / `watchpoint` (`:8169-8221`): `idecy = l`, `debugnextline = l`, `idecentercurrentline`, status "Paused." / "Breakpoint reached on line N" / "Watchpoint condition met (...)". Rendering is in the shared editor painter:

- next line to execute: line-number cell in colour 13 on 5 and a green `CHR$(16)` arrow in the separator column (`:13646-13649`, `:13667-13677`, `:13692-13694`);
- breakpoint: red background on the line number and a `CHR$(7)` bullet (`:13650`, `:13658-13659`);
- skip line: yellow number and `!` (`:13651`, `:13660-13661`);
- error line (`idefocusline`): red background on the whole line (`:13188-13189`).

`debugnextline` is cleared on F5 / run-to-line (`:7982`, `:8130`). While the program runs the IDE is drawn with darkened colours.

### 5.5 Breakpoints and skip lines

IDE-side storage: `REDIM SHARED IdeBreakpoints(1) AS _BYTE`, `IdeSkipLines(1) AS _BYTE` indexed by line (`ide_global:46-47`). They are shifted when lines are inserted or deleted (`:11411-11424`, `:12263-12273`), saved per file in the debug settings file with the bookmarks (`total breakpoints`, `breakpoint N`, `total skips`, `skip N`; `:19210-19227`, `:19248-19270`), sent in bulk in the handshake, and updated live with the single-line commands. The two flags are mutually exclusive.

There is **no validation that a breakpoint is on an executable line**: a breakpoint on a comment, blank line, `DIM`, a line inside `$CHECKING:OFF`, or an included file simply never fires.

### 5.6 Detecting the end of a session

| Event | Detection | Line |
|---|---|---|
| normal end / END / SYSTEM | `quit:Program ended.` -> "Debug session aborted." + reason | `:8306-8316` |
| communication error | `quit:Communication error.` | same |
| runtime error | `error:<line>`: red status "Error occurred on line N", line highlighted, `PauseMode = _TRUE`, `BypassRequestCallStack` (the call stack was already pushed) | `:8317-8327` |
| debuggee crashed or was killed | `_CONNECTED(debugClient&) = 0` checked after every GET and PUT, **only when `os$ = "WIN"`** -> "Disconnected." | `:8379-8386`, `:8408-8415` |
| user abort | ESC or window close -> `free` | `:7704-7714`, `:7241` |
| connect/handshake timeout (10 s) or ESC | | `:7128-7138`, `:7150-7160` |

On Linux/macOS there is no disconnect detection in this code: if the debuggee dies without sending `quit`, the IDE stays in DebugMode until ESC **[inferred from the `os$ = "WIN"` guard; UNVERIFIED by running]**.

---

## 6. Variable watch

### 6.1 The Watch List dialog — `idevariablewatchbox$(currentScope$, filter$, selectVar, returnAction)` (`ide_methods:8771-10027`)

Title "Add Watch - Variable List (N[, filtered])". Controls: a filter text box, a list box, and buttons `Add All | Remove All | Close`, plus `Send Value | Add Watchpoint` while debugging (`:8855-8859`). It can be opened without a running session (after a successful compile; otherwise the status line says "Variable List will be available after syntax checking is done...", `:6315-6323`).

**Source of the list**: `usedVariableList(1 .. totalVariablesCreated)`, the compiler's in-memory table. Variables declared in `$INCLUDE` files are skipped everywhere (`.includedLine <> 0`; `:8804`, `:9405`, `:9826`, `:9889`). Each row (`buildList`, `:9853-10026`) shows a `+` marker when watched, the name (arrays with their watched index range), a red bullet if a watchpoint exists, the type name, the scope (`GLOBAL` or the internal procedure name such as `SUB_FOO`), and while debugging the last value: `= value`, `= {v1,v2,...}` for native arrays, `<multiple values>` for UDTs, or `<out of scope>`. Colours are embedded in the list text as control bytes (`CHR$(16)`/`CHR$(17)` + colour) and patched in place when toggling.

**Filtering**: `multiSearch` (`:8746-8769`) — case-insensitive substring match, terms joined with `+` must all match, against name + type + scope (+ current value for simple variables while debugging, `:9891-9902`). Typing while the list has focus jumps to the filter box (`:9777-9785`).

**Selecting**: Enter or double-click toggles `.watch` (`:9544-9770`). "Add All" skips arrays without a stored range and UDT variables (`:8967-8982`).

**Return actions** (`returnAction`): 0 = closed; 1 = "send value" descriptor returned; 2 = set watchpoint; 3 = clear watchpoint; -1 = redraw and reopen. `selectVar = -1` is a non-interactive mode used at the start of every debug session to rebuild the list after edits ("Analyzing Variable List..." progress box, `:7052`, `:8818`, `:9385-9403`).

**Report export**: Ctrl+C in the list copies a plain-text "QB64(PE) - Variable List Report: <file>" (name, type, scope per line, honouring the filter) to the clipboard (`:9772-9775`, `copyList` `:9812-9851`). No file is written.

### 6.2 Arrays and index ranges

Watching an array prompts "Watch Array / Indexes" (`:9559-9606`). Syntax: dimensions separated by `,`; inside a dimension, single values, ranges `a-b` or `a TO b`, and alternatives separated by `;` (e.g. `1-3;7, 0-1`). The number of dimensions must equal `ids(id).arrayelements` (`:9573-9577`).

- `parseRange$` (`:10600-10675`): turns one dimension's text into a sorted, de-duplicated list of `MKL$` values using a bitmap string (initially 1000 long, grown on demand). **Only non-negative indexes** are possible (`-` is the range operator).
- `formatRange$` (`:10540-10567`): the inverse, for display (`1-3;7`).
- Stored as `usedVariableList().indexes` = for each dimension `MKL$(byteLength) + values`, and `.watchRange` = display text.
- `expandArray$` (`:10569-10598`): recursive cartesian product over the dimensions (uses STATIC recursion state); result is a sequence of `MKL$(len) + (one MKL$ per dimension)`.

Every expanded combination becomes one watch row with its own storage slot and its own `get var` round trip, so a range such as `0-99,0-99` creates 10 000 rows.

### 6.3 UDT members — `ideelementwatchbox$` (`ide_methods:10029-10513`)

A whole UDT cannot be watched ("Cannot add full UDT to Watch List", `:9730`); the user picks members. The dialog lists the members of the type by walking the compiler tables `udtxnext()` / `udtenext()` (`:9618-9628`), showing name and type (`id2fulltypename$`, `:10373-10393`). Selecting a member that is itself a UDT recurses (`level + 1`, `:10311-10348`). Members that are static arrays prompt for one element index per dimension, validated against the member's descriptor with `ParseNextUDTArrayDescriptorDim` (`getStaticArrayIndex`, `:10420-10511`). Two modes: multi-select ("Add UDT Elements": Add All / Remove All / Close) and single-select ("Choose UDT Element": OK / Cancel / Up One Level) used for Send Value and watchpoints. The result is a list of member paths (`.a`, `.b.c`, `.arr(3)`).

For each chosen path the IDE calls the **compiler's own** `udtreference$("", path, typ)` to obtain the member's type and offset expression (`:9667-9672`), maps the type code to a type-name string (`:9687-9733`), and evaluates the offset with `idewatchoffset&` (`:10515-10538`), which runs the text after the last `sp3` separator through `Evaluate_Expression$` and accepts only a constant in 0..2^31-1. Otherwise: "The debugger can currently watch only TYPE members with a static byte offset." (`:9740`). Results are stored in `.elements` (first 4 bytes = longest name, then names separated by `sp`), `.elementTypes`, `.elementOffset` (one `MKL$` each). For arrays of UDTs `.arrayElementSize = udtxsize(typ) \ 8` (`:9640-9650`).

### 6.4 `variableWatchList$` and storage slots

Built on dialog close (`generateVariableWatchList`, `:9375-9514`):

```
MKL$(longestVarName) MKL$(totalVisibleVariables)
repeat: MKL$(-1) MKL$(varIndex) MKL$(indexBytes) [indexes] MKL$(elementOrdinal) MKL$(elementOffset) MKL$(storageSlot)
```

One record per displayed row (scalar; each array element; each UDT member; each member of each array element). `storageSlot` indexes `vWatchReceivedData$()` (`ide_global:14`, initially `1 TO 1000`, grown by 999 as required, `:9443-9445`), and each variable's slots are also listed in `.storage`. At the same time `backupVariableWatchList$` (cname -> ordinal) and `backupUsedVariableList()` are filled so selections survive recompiles (section 1.6).

### 6.5 Fetching and decoding values

On every pause notification, and after the dialog closes, the IDE walks `variableWatchList$` and sends one `get global var` / `get local var` per row (`requestVariableValues`, `:8223-8270`) — for **all** rows, including locals of other procedures (the debuggee silently ignores those). The request size comes from the type name (`GetVarSize`, `:8451-8488`):

| Type name | Bytes requested | Decoded as (`:8280-8296`) |
|---|---|---|
| `_BYTE` / `_UNSIGNED _BYTE` | 1 | `_CV` of the same type, `STR$` |
| `INTEGER` / unsigned | 2 | same |
| `LONG` / unsigned | 4 | same |
| `_INTEGER64` / unsigned | 8 | same |
| `SINGLE`, `DOUBLE` | 4, 8 | same |
| `_FLOAT` | `LEN(dummy##)` | `_TOSTR$(_CV(_FLOAT, ...))` |
| `_OFFSET` / unsigned | pointer size | same |
| `_BIT`, `_BIT * n` (n <= 32) | treated as (unsigned) `LONG`, 4 | |
| `_BIT * n` (n > 32) | treated as (unsigned) `_INTEGER64`, 8 | |
| `STRING` (variable) | pointer size + 4 (the head of the `qbs` struct); the debuggee replies with the characters | none |
| `STRING * n` scalar | as `STRING` (scalar fixed strings are also `qbs` objects) | none |
| `STRING * n` as UDT member | n | none |
| UDT | size of the selected member's type (recursive lookup in `.elementTypes`) | per member type |

The decoded value is stored **as decimal text** in `vWatchReceivedData$(slot)`. Display formats (HEX/BIN/OCT, per variable, cycled with the small `CHR$(29)` button) are applied at paint time through `VAL()` then `HEX$` / `_BIN$` / `OCT$` (`:8634-8639`), so 64-bit and floating values lose precision or meaning in those formats. NUL bytes in strings are shown as spaces. Values are not invalidated when the program resumes; the panel only repaints while paused (`:8426`).

### 6.6 The watch panel — `showvWatchPanel` (`ide_methods:8518-8744`)

A floating text-mode window drawn over the editor after each repaint (`UpdateDisplay`, `:8419-8434`) when paused, the list is non-empty and the panel has not been closed. Title "Watch List - <current scope>". One row per record: `name[(indexes)][.member] = value` (strings quoted), or `<out of scope>` in dim colour when the variable's `subfunc` differs from `currentSub$` and is not global (`:8619-8646`). Features: shadowed box, red ` x ` close button, `CHR$(254)` resize handle in the bottom-right corner, vertical and horizontal scrollbars with draggable thumbs, a hover button to cycle the display format (`:8655-8672`), a bullet in column 1 for rows with a watchpoint (white if it is the one that just fired) and a hover tooltip showing the condition (`:8674-8707`).

Mouse handling lives in `DebugMode` (`:7311-7548`): drag anywhere inside to move, drag the corner to resize (minimum 40 x 5; clamped to the editor area, `checkvWatchPanelSize` `:8500-8511`), wheel scrolls 3 rows, double-click or right-click opens the Watch List dialog, close button asks "Keep Variables / Clear List". Geometry is persisted per IDE instance in the debug settings file, section `VWATCH PANEL <tempfolderindex>` (`cfg_global.bas:79`; read `:7061-7071`, written `:7489-7495`).

### 6.7 `WatchListToConsole`

Setting in `[DEBUG SETTINGS]` (`cfg_methods.bas:316-323`, `cfg_global.bas:24`), toggled by Debug > "Output Watch List to Console" (`ide_methods:5391-5403`). When on, `UpdateDisplay` turns the IDE's console on (`_CONSOLE ON`, `:8427`) and `showvWatchPanel` writes the title and **all** rows with `_ECHO` instead of drawing the panel (`:8543`, `:8574`, `:8599-8601`, `:8709`); the console is closed when leaving debug mode (`:820`). Since this runs on every `UpdateDisplay`, the whole list is re-echoed on each repaint (every arriving value, cursor movement, etc.).

### 6.8 Send Value (setting a variable)

Button "Send Value", or a double-click on the value column (`:9519-9521`). Only allowed for globals or variables of the current scope ("Variable is out of scope.", `:9019`, `:9362`). Arrays prompt for a single element index (`:9042-9075`); UDTs open the single-select member chooser (`:9081-9202`). The current value pre-fills an input box. The dialog returns a descriptor with `returnAction = 1`; `DebugMode` (`:7763-7893`) converts the typed text according to the type — numbers through `VAL()` then `_MK$` of the right width (so `&H` literals work), strings verbatim, fixed strings truncated — sends `set global address` / `set local address`, and writes the new text straight into the local cache slot (no read-back from the debuggee).

### 6.9 Watchpoints

- **Defined in the IDE, evaluated in the debuggee.** Button "Add Watchpoint" (allowed for any scope). The condition is `<op> <literal>` with op in `=`, `<>`, `>`, `>=`, `<`, `<=` (`=<`, `=>` are normalised). For numeric types the literal must survive `_TOSTR$(VAL(x))` unchanged, so no scientific notation, hex or leading zeros (`:9252-9312`). An input shorter than 2 characters clears the watchpoint (action 3).
- IDE copy: `watchpointList$` (`ide_global:13`), records `MKL$(-1) MKL$(len) [MKL$(varIndex) indexes MKL$(elementOffset) expression MKI$(len)]` (`:9332-9353`); reset at the start of every session (`:7055`), so **watchpoints do not persist across runs**. One watchpoint per variable/index/member.
- Debuggee copy: `vw_globalWatchpoints$` / `vw_localWatchpoints$`, each record being the full descriptor (`vwatch.bm:602-658`).
- Evaluation: `CheckWatchpoints` (`vwatch.bm:746-829`) on every line change while running or stepping: re-reads the variable through the `get var` code path with `vw_checkingWatchpoints` set, converts to text (`GetV2`, `:841-882`) and compares with `VAL()` for numbers or string comparison for strings (a quoted literal is compared exactly; an unquoted one is compared after trimming both sides). Local watchpoints are skipped unless execution is in that procedure.
- **Operand-order inconsistency**: numeric tests are `value op literal` (`vwatch.bm:813-825`) but string tests are `literal op value` (`:789-801`), so `<`, `<=`, `>`, `>=` on strings behave reversed relative to numbers.
- On a hit the debuggee pauses and sends `watchpoint:`; the IDE shows "Watchpoint condition met" with `OK | Clear Watchpoint` (`:8171-8199`); "Clear" sends `clear last watchpoint`. A level-triggered condition that remains true fires again on the next line.

---

## 7. Call stack

- **Tracking** is entirely in the debuggee: level counter `vwatch_sublevel` maintained by compiler-emitted increments/decrements (section 1.5) and `vwatch_stack(level)` written at every procedure entry with `internalName,displayName, line N` where N is `vw_lastLine`, i.e. the line of the call site (`vwatch.bm:130-136`). Entries are overwritten per level; nothing is popped. The first field also provides the "current sub" name used for scope checks.
- **Transmission**: `SendCallStack` (`vwatch.bm:719-732`) joins the display parts of levels 2 .. current with `CHR$(0)` and sends `call stack size:` then `call stack:`. Sent on request (`call stack` command, paused only), and unsolicited just before `quit:Program ended.` and before `error:`.
- **IDE**: F12 while paused requests it and waits up to 10 s in a nested loop that discards any other message arriving meanwhile (`ide_methods:8076-8120`); unsolicited stacks are stored by the `call stack size` handler (`:8348-8366`). The data are kept in the globals `callstacklist$` / `callStackLength` (`ide_global:23-24`) so the **last run's stack stays viewable after the program ends** (F12 or Debug > Call Stack in the normal IDE, `:1727-1735`, `:6335-6346`; the menu item is greyed when empty, `:4687-4691`). They are cleared when a file is loaded or a new program started (`:526`, `:623`, `:6563`).
- **Dialog** `idecallstackbox` (`:10677-10847`): list box titled "Call Stack" with the caption "Most recent sub/function calls in your program:", last entry preselected; buttons `Go To Line | Close | Copy`. Go To Line (also Enter or double-click) parses the number after the last space of the selected entry and moves the cursor there (`:10814-10826`); Copy (or Ctrl+C) puts the list on the clipboard with line feeds (`:10836-10838`).
- The main module is not an entry; with an empty stack the IDE reports "No call stack log available."

---

## 8. Limitations and hazards

Design-level:

1. **The debugger is compiled into the debuggee and written in the debugged language.** It shares the heap, error state, timers, keyboard buffer and string system with the user program. `vwatch` allocates BASIC strings on every hook; `_KEYCLEAR` on resume eats user keystrokes; `CLEAR`, `CLOSE`, `RUN` and `CHAIN` needed special cases or are unsupported.
2. **Hard-wired mangled names and memory layouts.** The compiler emits literal C identifiers for BASIC variables defined in `vwatch.bi` (`__LONG_VWATCH_LINENUMBER` etc.), and `vwatch.bm` assumes: table slot -> pointer variable -> data (two dereferences); array descriptor word 0 = data pointer; `qbs` = `{pointer, int32 len, ...}`; `_FLOAT` occupies 16 bytes in arrays; pointer size from `LEN(_OFFSET)`. Any change in code generation or runtime structs silently breaks the debugger.
3. **Type handling by type-name strings** (`"STRING *"`, `"_BIT *"`, `"_UNSIGNED"` via `INSTR`) duplicated in three places (IDE size table, IDE decoder, debuggee) that must stay in sync.
4. **Metadata lives only in the IDE's compiler state** (`usedVariableList`, `ids()`, `udt*` arrays). No symbol file, so no attach to an already running program, no debugging from another IDE instance, and the watch dialog calls compiler internals (`udtreference$`, `Evaluate_Expression$`, `getid`) from UI code, temporarily mutating compiler globals such as `CheckingOn` and `Error_Happened` (`ide_methods:9668-9673`).
5. **Per-line cost with no throttle**: a function call, socket poll and string work on every line change, plus all watchpoints. Tight loops slow down markedly **[magnitude not measured]**.
6. **Line granularity only.** Multiple statements on a line are one step; single-line loops cannot be interrupted; breakpoints on non-instrumented lines never fire and are not flagged.

Coverage gaps:

7. No instrumentation inside `$CHECKING:OFF` blocks or `$INCLUDE` files; variables declared in include files are hidden from the watch list.
8. Locals are readable only for the currently executing procedure (no frame selection; callers' locals are unreachable even though the call stack is known).
9. Whole UDTs, UDT members without a constant byte offset (dynamic member layouts), negative array indexes, and expressions cannot be watched. `_MEM` and other non-native types fall under "UDT" handling **[not individually verified]**.
10. Scalar `_BIT` variables are fetched as 4 or 8 raw bytes from the variable's address; correctness for bit widths not stored in a 32/64-bit cell was not verified **[UNVERIFIED]**.
11. Watchpoints: only `variable op literal`; string ordering comparisons reversed; not persisted; evaluated only at instrumented line changes.
12. Set Next Line works only within the current procedure and is an unchecked C `goto` (can jump into loops/blocks, skipping initialisation).
13. Non-Windows: no foreground-window switching, and no disconnect detection.
14. Error stop is notification-only: the debuggee does not pause on a runtime error, so variables cannot be inspected at the error point through the protocol.

Protocol and robustness:

15. Frame-completeness off-by-4 bug in both receivers (section 3.1).
16. Most commands are ignored while running; `free` while running leaves the debuggee polling a closed socket; no acknowledgements, so IDE mirrors (`PauseMode`, breakpoints) can drift from debuggee state.
17. Handshake loop in the debuggee has no `_LIMIT` and no timeout; the 10 s timeouts are fixed constants (`ide_methods:6987`, `vwatch.bm:40`).
18. Nested wait loops in the IDE (call stack request) discard unrelated messages.
19. One message per 10 ms on each side: value refresh time grows linearly with watch rows; expanded array ranges multiply rows.
20. Authentication is only an executable-file-name comparison; the listener port is predictable (base + instance index). Whether `_OPENHOST` binds to loopback only was **not checked**.
21. Stale/uninitialised fields in the dialog-built descriptor: `tempElement&` is never assigned in the Send Value / watchpoint path, the first storage-slot lookup uses `tempElementOffset$` before it is set, and the descriptor is sent with the early `tempStorage&` rather than the recomputed `storageSlot&` (`ide_methods:9024-9041`, `:9203-9217`, `:9322-9325`). Effects were not tested.
22. Buffers and counts: `vWatchReceivedData$(1 TO 1000)` and `backupUsedVariableList(1000)` are initial sizes that grow by 999; `vwatch_stack(1000)` grows by 1000; dialogs use the IDE's fixed `o(1 TO 100)` object array; `parseRange$` bitmap grows to the largest index; names in the wire descriptor are limited to 32 767 bytes (INTEGER length), values in `set address` likewise.
23. `$DEBUG` forces at least one extra compile pass and makes every program depend on the name `vwatch` (even without `$DEBUG`, via the stub).

---

## 9. Not determined / not verified

- `vWatchDesiredState` and `vWatchRecompileAttempts` do not exist in this tree; the equivalent mechanism is `RCStateVar` (section 1.1). Whether older behaviour differed was not investigated.
- Nothing was executed; all behavioural statements are from reading source. Items marked **[UNVERIFIED]** above in particular: behaviour of skip-line when no skip label exists; ON ERROR interaction; console-only programs (`$CONSOLE:ONLY`) and the `hwnd` message; what pulls in the sockets dependency for `$DEBUG` builds; other callers of `vWatchHandle()`; scalar `_BIT` reads; which procedure parameters are registered in `vwatch_local_vars`; `free` while running; disconnect handling on Linux/macOS; loopback binding of `_OPENHOST`.
- The exact unit/format of the `udtreference$` result consumed by `idewatchoffset&` was taken at face value (the code treats it as a byte offset).
- The IDE's `ide2` main loop outside the listed line ranges was only sampled through greps; other minor debug-related touch points may exist.
- `internal\c` was searched only in `*.cpp`/`*.h` directly under `internal\c`, `internal\c\libqb\include` and `internal\c\libqb\src`; deeper runtime folders were not searched.

---

# Part G — Assessment for the rewrite

## G.1 What couples the IDE to compiler internals

1. **One compilation unit, one namespace.** The IDE is `$INCLUDE`d into `qb64pe.bas` (Part A §A.1). Nothing enforces a boundary; any IDE routine can read or write any compiler global, and several do.
2. **Coroutine by function call and GOTO.** The compiler calls `ide(0)`; the IDE returns to make the compiler work. Consequences: all IDE state must be global or STATIC; non-STATIC locals of `ide2` are silently reset on every yield (Part B §1.1); a nested `ide(0)` call exists inside `lineformat` for line continuations (code 100); a runtime error anywhere in IDE code is recovered by `RESUME sendcommand`, re-entering the IDE from the top (Part A §A.3).
3. **Line-at-a-time pull model.** The compiler never sees the document; it pulls lines through the IDE, including lines of `$INCLUDE` files that it read itself (code 10). The editor's line number is the compiler's `linenumber`. Any edit restarts compilation from line 1 (two passes); there is no incremental reuse.
4. **Results delivered through shared memory, not messages**: `layout$`, the warning arrays, `usedVariableList()` and the UDT tables, `InvalidLine()`, `listOfCustomKeywords$`, `linefragment`, `ExtDepBuf`, `lastBinaryGenerated$`, `compfailed`, `idecompiled` (table in Part A §A.4). The compiler in turn reads `ideprogname$`, `idepath$`, `iderunmode`, `ModifyCOMMAND$`, `IDEAutoIndent`/`IDEAutoLayout`.
5. **Run requests are a return code plus side-channel globals** (`iderunmode`, `NoExeSaved`, `startPaused`, `SaveExeWithSource`, `DefaultExeSaveFolder$`), and the compiler itself spawns the user program and calls the IDE routine `DarkenFGBG`.
6. **The formatter is the compiler.** Indentation, spacing and keyword case are a by-product of the main compile pass (`layout$`), so there is no formatting without compiling and none for lines the compiler did not reach (after the first error) or that use `_` continuation.
7. **The debugger depends on code-generation details**: mangled C names emitted as string literals, pointer-table layout, `qbs` and array descriptor layouts, type names as strings, and compiler routines (`udtreference$`, `Evaluate_Expression$`, `getid`) called from dialog code with compiler globals temporarily mutated (Part F §8).
8. **Shared infrastructure**: config globals are read by both sides; the IDE builds its helper tools by running itself with `-x … -o` (Part E §4, §6.2); part of the language surface (`_TRUE`, `_KEY_*`, …) lives in auto-included BASIC files that the IDE special-cases by file name (Part E §5).

The IDE's own SUB/FUNCTION list (SUBs dialog, contextual menu "Go to SUB") is *not* coupled: it re-scans the edit buffer by line prefix (Part D §1.1), which is a second, simpler parser that can disagree with the compiler.

## G.2 What a clean interface would need to expose

A language-server-style boundary (in-process API or a separate process) that covers today's behaviour needs the following. Items marked (new) do not exist today but fall out naturally.

| Capability | Replaces | Notes |
|---|---|---|
| Document sync: open/change/close with full text or ranges, plus include-file resolution relative to the document path | line pull (2/4/5/7/10/100), `ideprogname$`/`idepath$` | The service should own reading `$INCLUDE` files; unsaved buffer content must override disk. |
| Cancellable, restartable analysis with progress | "return 2 again", percentage in `IdeInfo` | Needs a real cancel; ideally incremental or at least debounced. |
| Diagnostics: list of {file, line, (column), severity, message, "caused by" fragment, include chain} | code 8, `warning$()` arrays, `linefragment`, `incerror$` | Today: first error only, line granularity, warnings grouped under a header string. Parity needs at least that; multiple errors is an improvement (new). |
| Formatting: per-line formatted text or edits, with options (indent on/off, indent size, spacing on/off, keyword case), able to exclude the line being typed | `layout$`, `apply_layout_indent$`, `IDEAutoIndent*`, `IDEAutoLayout` | Must not create undo steps or mark the file dirty if parity is wanted — or deliberately change that. Also used headless by `-y` format mode. |
| Inactive-region info (lines excluded by `$IF`) | `InvalidLine()` | |
| Semantic tokens or symbol names for highlighting user SUB/FUNCTION names; keyword list | `listOfCustomKeywords$`, `listOfKeywords$` | |
| Document symbols: SUBs/FUNCTIONs with kind, line, parameters, line count, external/library flag; labels | buffer scans in `idesubs$` and `IdeMakeContextualMenu`, `Labels()` | |
| Hover/help id for the word at cursor | `findHelpTopic$` + `links.bin` | Help content can stay in the IDE. |
| Expression evaluation for the Math Evaluator menu item | `lineformat` + `Evaluate_Expression$` | |
| Build: request {target path, run/exe-only/no-exe, flags, args}; streamed log; structured result {success, output path, log path} | return 9 + globals, `compilelog$`, file-exists test | Must be asynchronous; today the UI blocks during `make`. |
| Dependency fingerprints (external files and MD5) to decide rebuild vs relink | `ExtDepBuf`, `embedFileList$()` | |
| Program launch (console wrapper, terminal template `$$`/`$@`, logging environment variables, temp-EXE delete) | `qb64pe.bas:967-1081` | Belongs in the IDE or a runner, not the compiler. |
| Debug information: variables {name, type, scope, declaration site, storage location}, UDT layouts with member offsets, line table, procedure table | `usedVariableList()`, `udt*` arrays, `vwatch_*_vars` tables | Emit as a symbol file so the debugger no longer needs the compiler's memory; enables attach and out-of-process UIs (new). |
| "Is this program a debug build" and "does it use console" flags | `GetRCStateVar(vWatchOn)`, `ConsoleOn` | |
| Settings passed explicitly | shared config globals | |

For the debugger itself, a defined wire protocol with request ids, acknowledgements and versioning (or DAP) should replace the current text commands; see Part F §3 and §8.

## G.3 Features users will expect at parity

- **Editor**: the key and mouse map in Part B §4-5 (including virtual-space cursor, Home toggle, Alt+Shift+Up/Down line move, Ctrl+D duplicate, comment/uncomment/toggle, block indent, auto-close brackets, double-click word select with all-occurrence highlight, bracket matching, bookmarks, quick-navigation back, overwrite mode, Alt+numpad entry, Ctrl+K key-code insert, Shift+Enter RGB mixer, ASCII chart).
- **Live feedback**: status-area "OK"/error with click-to-jump, error-line highlight, warnings dialog (including unused variables), automatic indent/spacing/keyword-case as you type, manual-check mode (Shift+F9), `$IF`-excluded lines shown uncoloured.
- **Navigation**: SUBs dialog (F2) with sorting and line counts, go to line, go to SUB/label from the contextual menu, double-click on an `$INCLUDE` line to open it in another instance.
- **Search**: quick search bar with history, Find/Change dialogs with match case, whole word, backwards and the comment/string scope filters, change-with-verify and change-all (Part C §6).
- **Help**: offline cached wiki pages rendered in a split pane with links, back history, in-page link search, contextual F1 (including generated help for user SUBs), "update current page"/"update all pages", "view on wiki", code examples (Part E §1, Part D §5).
- **Run**: F5, F11 make EXE only, run-only without keeping the EXE, modify `COMMAND$`, output to source folder or default folder, EXE-location link, compile log link, logging options, terminal choice on Linux, licence file generation, purge build files.
- **Debug**: start paused, breakpoints and skip-lines (gutter click, F9, Ctrl+P), step into/over/out, run to line, set next line, watch list with arrays/ranges/UDT members and display formats, watch panel, send value, watchpoints, call stack (viewable after the run), console watch output.
- **Options**: display (window size, font 8/16, custom TTF, code page), colours with 14 presets and user schemes, code layout, compiler settings, language, undo/history limits, GUI vs text file dialogs, multi-instance behaviour with per-instance window/colour/watch-panel sections.
- **Files**: recent files, crash recovery prompt, export as HTML/RTF/Discord/forum/wiki, QB4.5 binary import, `$NOPREFIX` conversion offer, drag-and-drop open, QBJS web build.
- **Command line**: all switches in Part E §6, since scripts and the IDE itself depend on `-x`, `-c`, `-o`, `-z`, `-y`, `-l:`, `-s`.

Candidates to drop or redesign deliberately rather than port: the text-mode UI toolkit itself, whole-line-only multi-line paste and selection, the 608-column horizontal limit, CP437-only text, and the wiki edit-page scraping.

## G.4 Hazards

**Behavioural traps to preserve or consciously change**

- Auto-layout writes bypass undo and the dirty flag, and are deferred for the cursor line (Part B §2.4). A rewrite that routes formatting through the normal edit path will change undo granularity, mark files modified after merely opening them, and can trigger recompile loops.
- Keyword-case correction, `?`→`PRINT` expansion and closing-quote completion are applied even when "auto layout" is off (Part B §2.4 step 5).
- Every edit restarts a two-pass compile with no debounce; responsiveness on large programs depends on the one-input-poll-per-line design and the off-screen fast path. A background compile needs equivalent cancellation.
- Undo is whole-buffer snapshots in a size-limited disk ring shared across files in a session ("Undo through previous program content?"), and doubles as crash recovery. A recovered buffer loses its file name. Behaviour of redo after undo-then-edit is unclear from the code (Part B §8.1).
- `ideunsaved` has three states (0, 1, −1) and −1 also triggers the save prompt (Part B §8.3).
- Line numbers shown to the user exclude auto-included and `$INCLUDE`d lines only because the compiler decrements `linenumber` for them; errors in includes carry a separate include chain.
- The reserved name `vwatch` exists in every program (stub), and the auto-include files define part of the language (Part E §5, Part F §1.2).

**Encoding**

- The buffer is raw bytes; display goes through one of 27 code-page tables via `_MAPUNICODE`. No UTF-8 or BOM handling on load or save. Export and the wiki renderer each have their own CP437↔UTF-8 mapping. A Unicode-native rewrite must decide how to load legacy CP437 sources whose string literals contain box-drawing bytes that programs depend on at run time.
- Tabs are expanded to spaces on load and paste and never written back.

**Defects found by reading (decide whether to reproduce)**

- Debug protocol frame-completeness test is short by 4 bytes on both sides; string watchpoint comparisons are reversed; stale fields in the Send Value/watchpoint descriptor; disconnect detection only on Windows; a runtime error does not pause the debuggee; `free` is ignored while running (Part F §3, §8).
- `idedelline` shifts skip-lines the wrong way (`ide_methods:11422`); breakpoints and skip-lines are only shifted on insert/delete when `$DEBUG` is active; `DarkenFGBG` reads a misspelt colour variable (`:20832`); a comment/string-filtered match stops further matches on that line in Change All and Find Again; a second click on a text box at the cursor position clears the field (`:14950`; the source comment marks this as the intended double-click behaviour, but it is not time-limited) (Part C).
- `ErrorColor` falls back to the numbers colour in `cfg_methods.bas:585`; `BinaryFormatCheck%` return value after an on-demand converter build (Part E).
- `idesubs$` persists its check boxes on Cancel (Part D §1.1).

**Structural**

- Menu handlers match the literal menu text; several features are implemented twice (key path and menu path) and three copies each exist of the file loader and the search matcher. Porting "by behaviour" must pick one copy as the reference.
- The syntax highlighter has a 1-second watchdog that permanently disables highlighting when rendering is slow (Part C §5).
- Config is written immediately on every change and re-read on access; per-instance sections are keyed by the temp-folder instance index, which is also the debugger port offset and the temp build folder (Part E §2, Part A §A.5). Multi-instance behaviour depends on that single index.
- Fixed limits: 12 menus × 20 items, 1000-entry dialog text pool, `o(1 TO 100)` controls, 608 columns, 9-line message boxes without word wrap.
- The help cache file naming and `links.bin` generation must be reproduced if existing `internal\help` content is to stay usable (Part E §1.9).
- The C++ build is a blocking shell call whose only result is "does the EXE exist"; there is no mapping from C++ diagnostics to BASIC lines.

---

# Part H — Coverage and open questions

## H.1 How this study was done, and what was read

Part A and the cross-reference table in §A.4 were done directly on `source\qb64pe.bas` by targeted reading (lines 126-160, 283-420, 820-1170, 1565-1590, 1670-1830, 3196-3226, 3328-3420, 12496-12700, 12996-13008, 13430-13530, 13700-13945, 14160-14330, 25498-25535, 25895-25906, 28660-28720) plus a script over `DIM SHARED` declarations. The rest of `qb64pe.bas` was not read for this report.

Parts B to F were each produced by a dedicated reading pass over the stated ranges and merged here without re-verifying every line reference; a sample of references was spot-checked during assembly.

| Area | Coverage |
|---|---|
| `ide_methods.bas` 1-6975 (`ide`, `ide2`) | read in full |
| `ide_methods.bas` 6976-10850 (debugger, watch dialogs, call stack) | read in full |
| `ide_methods.bas` 10849-13700, 14929-15772, 16969-17126, 18805-18935, 20655-21424 | read in full, except the inline quick-search editor and Find-and-Verify loop inside `ide2`, read only for their hooks |
| `ide_methods.bas` 13701-14928, 15773-16968, 17127-18804, 18936-20654 (dialogs) | read in full |
| `ide_global.bas` | lines 1-75 and 131-242 read; 76-130 (27 code-page hex tables) not read |
| `wiki_global.bas`, `wiki_methods.bas`, `cfg_global.bas`, `cfg_methods.bas`, `ide_export.bas`, `ide_converters.bas`, `global\settings.bas`, `global\constants.bas` | read in full |
| `vwatch.bi`, `vwatch.bm`, `vwatch_stub.bm` | read in full |
| `internal\support\include\*`, `internal\support\color\*` | read |
| `internal\support\converter\*` (QB45BIN, AddPREFIX, qbjs-build) | sampled: headers and main flow only |
| `syntax_highlighter_list.bas` | sampled |
| `utilities\ini-manager` | sampled (API only) |
| `internal\c` runtime hooks for vwatch | grep only, top level and `libqb\include`, `libqb\src` |

## H.2 Could not determine / not verified

Each part ends with its own list (Part B §11, Part C §8, Part D §11, Part E §7, Part F §9). The most significant:

- Nothing was executed. Behaviour described for edge cases (undo-then-edit and redo records, compile state after an in-IDE runtime error, skip-line with no skip label, `free` while the program runs, debugger on Linux/macOS, console-only programs under `$DEBUG`) is inferred from code.
- Where the screen mode and palette are first set before `ide()` is called.
- The server-side meaning of the wiki's `&qbide=1` parameter, and whether a raw-wikitext endpoint is usable.
- Whether `_OPENHOST` binds the debug listener to loopback only.
- The per-line performance cost of `$DEBUG` instrumentation (not measured).
- Practical limits on file size and line length (no explicit limit found).
- Internals of the three converter programs and the on-disk layout of `DebugFile$` beyond its key names.
- The `libraries` add-on and Library Explorer are not in this checkout; `$USELIBRARY` is described from compiler code only.
- Ownership of some shared variables in Part B §9 is inferred from names; §A.4 lists the ones confirmed by declaration.
- The brief's assumptions that turned out not to match this tree: no `vWatchDesiredState`/`vWatchRecompileAttempts` (replaced by `RCStateVar`), no "Start detached" menu item, no triple-click, no word wrap in `idemessagebox`, no auto-includes UI, no external-editor mode.
