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

# Part A — Compiler side of the IDE interface (`source\qb64pe.bas`)

## A.1 Where the IDE is linked in

The IDE is `$INCLUDE`d into the compiler's single compilation unit: `ide\ide_global.bas` at `qb64pe.bas:38`, `ide\config\cfg_global.bas` at `:408` (followed immediately by `ReadInitialConfig`), `ide\config\cfg_methods.bas` and `ide\ide_methods.bas` at `:28826-28827`. `ide_global.bas:6` in turn includes `wiki\wiki_global.bas`. Everything shares one global namespace.

The shared protocol variables are declared at `qb64pe.bas:392-406`: `idecommand`, `idereturn`, `ideerror`, `idecompiled`, `idemode`, `ideerrorline`, `idemessage`, `IDEStartAtLine`, `errorLineInInclude`, `warningInInclude`, `warningInIncludeLine`, `compilelog$`.

## A.2 The dispatch loop (`qb64pe.bas:845-1106`)

```
IF NoIDEMode THEN ... GOTO noide                    ' :845
idemode = 1                                         ' :850
sendc$ = "" | CHR$(1) + CMDLineSrcFile$             ' :851-852
sendcommand:                                        ' :853
  idecommand$ = sendc$ : C = ide(0) : ideerror = 0  ' :854-856
  IF C = 0 THEN idemode = 0: GOTO noide             ' :857
  c$ = idereturn$                                   ' :858
```

| IDE returns `C` | Compiler action | Lines | Next message to IDE |
|---|---|---|---|
| 2 begin | `idepass = 1`, `GOTO fullrecompile` (re-initialises all compiler state), arrives at `ideret1`, treats `c$` as the first line, runs the prepass on it (`ideprepass`), returns via `ideret2` | 860-871 | 3 |
| 4 next line | pass 1: `wholeline$ = c$`, `GOTO ideprepass`; pass 2: `a3$ = c$`, `GOTO compileline`, back at `ideret4` | 873-887 | 3 |
| 5 end of program | feeds an empty last line with `lastLine = 1`. After pass 1: `idepass = 2`, `GOTO ide3` (inter-pass work), `ideret3`, then sends 7. After pass 2: `GOTO ide5` (finalise), `ideret5`, sends 6 with `idecompiled = 0` | 889-921 | 7, then 6 |
| 9 run | if `idecompiled = 0`: choose EXE path (`DefaultExeSaveFolder$` or source folder when `SaveExeWithSource`), delete an existing EXE or pick `name(2)`… and tell the IDE with 12; `GOTO ide6` (C++ build), `ideret6`, `idecompiled = 1`. Then `iderunmode = 2` → 11; else build `ExecuteLine$` per OS (console programs are wrapped in `cmd /c` or the configured terminal), set logging environment variables, and `SHELL _DONTWAIT` (or blocking `SHELL` plus delete for `NoExeSaved`) | 923-1082 | 12, 11, 6 or 254 |
| 14 | raise "$NOPREFIX is a deprecated feature…" | 1084-1087 | 8 |

Other places that send: 7 after an internal recompile request (`iderecompile`, `:1700-1705`, set at `:13004`); 13 when the prepass meets `$NOPREFIX` (`:2020-2024`); 10 + line for each line read from an `$INCLUDE` or auto-include file, in the prepass (`:3208`) and main pass (`:12620`) — the compiler reads the include file itself and merely bounces each line through the IDE so the UI stays responsive, with `linenumber` decremented so include lines do not consume editor line numbers; 100 from inside `lineformat` to fetch the next physical line of a `_` continuation (`:25524-25529`, a nested `ide(0)` call, not via `sendcommand`).

Note the control flow is all `GOTO` between labels in the main program; `ideret1..6` are the points where non-IDE mode would simply fall through.

## A.3 Error path

- `errmes:` (`qb64pe.bas:14308-14325`): any compile error sets `a$`, clears `layout$`, appends include information as `CHR$(1) + incerror$`, and in IDE mode sets `ideerrorline = linenumber + erldiff`, `idemessage$ = a$`, `GOTO ideerror`. Errors caused inside auto-included support files are rewritten to be less confusing (`:14313-14318`).
- `ideerror:` (`:1091-1106`): sends `CHR$(8) + idemessage$ + MKL$(ideerrorline)`.
- Runtime errors in the compiler's own code (`ON ERROR GOTO qberror`): in IDE mode converted to "Compiler error (check for syntax errors) (…)" and `RESUME ideerror` (`:14284-14294`).
- Runtime errors **inside IDE code**: the same handler tests `ideerror` (non-zero while IDE code runs), logs to `internal\temp\ideerror.txt`, and `RESUME sendcommand` with `CHR$(255)` (`:14263-14275`), i.e. it re-enters the IDE from the top. Part B §1.6 documents the IDE half.
- Only **one** error is reported per pass: compilation stops at the first error.

## A.4 Data the IDE reads straight out of compiler memory

Produced by a script that cross-references `DIM SHARED` names declared in `qb64pe.bas` and `utilities\type.bi` against `ide_methods.bas` (counts are textual occurrences in `ide_methods.bas`):

| Compiler global | Declared | Uses in IDE | Purpose in the IDE |
|---|---|---|---|
| `usedVariableList()` (TYPE `usedVarList`, `qb64pe.bas:130-138`) | 1571 | 238 | watch list, variable dialog, debugger value decoding |
| `backupUsedVariableList()`, `backupTypeDefinitions$`, `typeDefinitions$` | 140, 149 | 9, 7, 1 | keep watch selections across recompiles |
| `totalVariablesCreated`, `totalMainVariablesCreated` | 150 | 11, 1 | variable list size |
| `warning$()`, `warningLines()`, `warningIncLines()`, `warningIncFiles()`, `totalWarnings`, `warningListItems`, `maxLineNumber` | 1581-1584, 151, 153 | 17, 6, 5, 4, 12, 6, 2 | warnings dialog and status line |
| `layout$` (`:478` "passed to IDE") | 478 | ~44 (name also used locally) | auto-format result |
| `InvalidLine()` | 55 | 9 | lines excluded by `$IF` are drawn uncoloured (`ide_methods:13367`) and skipped by multi-highlight (`:13044`) |
| `vWatchOn` (via `GetRCStateVar`) | 176 | 15 | is `$DEBUG` active |
| `CheckingOn`, `Error_Happened` | 179 | 6 | temporarily mutated by the watch dialog when it calls compiler routines |
| `udtxcname`, `udtxsize`, `udtxnext`, `udtecname`, `udtesize`, `udtetype`, `udtetypesize`, `udtearrayelements`, `udtenext`, … and type flag constants `ISSTRING`, `ISFIXEDLENGTH`, `ISUDT`, `ISINCONVENTIONALMEMORY`, `BYTETYPE` … `UOFFSETTYPE` | `utilities\type.bi` | 1-8 each | UDT member browsing in the watch dialog |
| `ids()`, `Labels()` | 622, 442 | 4, 3 | symbol lookups (labels for "Go to label") |
| `listOfCustomKeywords$` (appended by the compiler at `:25900-25904` when a user SUB/FUNCTION is registered) | `syntax_highlighter_list.bas:1` | — | highlight user procedure names |
| `linefragment`, `prepass`, `sp`, `sp2`, `sp3`, `fix046$` | 673, 453, constants | 2, 1, 20, 1, 10, 3 | "Caused by" text, progress, token separators |
| `ExtDepBuf`, `embedFileList$()` | 1662, 158 | 13, 4 | detect changed external dependencies before F5 |
| `lastBinaryGenerated$`, `path.exe$`, `extension$`, `compilelog$`, `compfailed`, `idecompiled`, `NoExeSaved`, `recompile` | 263, 262, 406, 380, 397, 40, 679 | 6, 4, 5, 3, 2, 8, 2, 3 | build results |
| `ForceOptExpl`, `UserDefineList`, `nativeDataTypes`, `lineinput3buffer`, `TempList`, `WindowTitle`, `tempfolderindex(str)`, `os$`, `MacOSX`, `pathsep$` | various | 1-26 | misc |

Compiler routines called from IDE code (occurrence counts in `ide_methods.bas`): `lineformat` 6, `getid` 2, `id2fulltypename` 1, `validname` 1, `HashFind` 3, `getelement` 7, `numelements` 1, `SCase` 4, `GetRCStateVar` 15, `Evaluate_Expression` 2, `apply_layout_indent` 1, `lineinput3load` 3, `StrReplace` 25, buffer API (`ReadBufLine` 8, `WriteBufLine` 4), `ReadSetting` 8, `WriteConfigSetting` 97, `ReadConfigSetting` 6, `WikiParse` 12, `FileHasExtension` 3.

In the other direction the compiler reads IDE globals: `ideprogname$` and `idepath$` (include resolution `qb64pe.bas:3129`, `:12544`; auto-include special cases `:1729-1783`; EXE path `:933`), `iderunmode`, `ModifyCOMMAND$`, `IDEAutoIndent`/`IDEAutoLayout` (saved and restored around includes, `:12507-12508`, `:12638-12639`, reset at `:3378`), `idecompiledline$` (`:3376`, `:12506`, `:12637`), and it calls `DarkenFGBG` (an IDE routine) around the C++ build (`:1073`, `:13940`, `:14170`).

## A.5 C++ build launch and compilelog

- `compilelog$ = tmpdir$ + "compilelog.txt"`, truncated at the end of code generation (`qb64pe.bas:13453-13455`). `tmpdir$` is `internal\temp\` for the first running instance and `internal\temp<N>\` for others; the instance index `tempfolderindex` is found by PID bookkeeping in `tempfoldersearch.bin` when `_OS$` contains `LINUX`, otherwise by trying to lock `temp.bin` (`:284-343`). Instances above 1 get their own `qbx<N>.cpp` (`:344-355`).
- `ide6:` builds one `make` command line (`:13700-13800`): `GetMakeExecutable$` + `DEP_*=y` flags from the dependency table + `EXE=<escaped path>` + `CXXFLAGS_EXTRA`/`CFLAGS_EXTRA` (user flags plus `-O2`, or `-Og -g`/`-g` with debug info) + `CXXLIBS_EXTRA` + `-j<MaxParallelProcesses>` + `BITS=` + optional `STRIP_SYMBOLS=n`, `GENERATE_LICENSE=y`, `USE_SYSTEM_MINGW=y`, `TEMP_ID=<instance>`. `qbx.o` is deleted first to force a rebuild (`:13787-13794`).
- Launch: Windows `SHELL _HIDE "cmd /c " + makeline$ + " 1>> " + compilelog$ + " 2>&1"` (`:13936`); elsewhere the same without `cmd /c` (`:14167`). The call is **blocking**: the IDE is frozen (screen dimmed by `DarkenFGBG`) for the duration of the C++ build. Helper scripts `debug_win.bat` / `debug_lnx.sh` are also written to the temp folder.
- Success test: does `path.exe$ + file$ + extension$` exist (`:14196-14201`)? If not, `idemessage$ = "C++ Compilation failed " + CHR$(0) + "(Check " + compilelog$ + ")"`, `GOTO ideerror` (`:14203-14206`). The compiler's exit status and the log contents are never parsed; C++ errors are never mapped back to BASIC lines.

---

# Part B — `ide` / `ide2`: entry point, protocol, main loop, menus, run flow, undo

*(Section numbers below are local to Part B.)*

Source: `..\QB64pe\source\ide\ide_methods.bas` lines 1-6975 (all refs below are to this file unless prefixed), plus `source\ide\ide_global.bas` 131-242 (shared declarations).

Conventions: `L123` = line 123 of `ide_methods.bas`. "Verified" means the line was read; anything not verified is marked **(unverified)**.

---

## 1. Entry, return, and the compiler<->IDE protocol

### 1.1 Shape of the interaction

The compiler (`qb64pe.bas`) and the IDE are co-routines implemented with a single function call. The compiler sets the global `idecommand$` (command byte + payload) and calls `ide(0)`. The IDE runs its event loop for as long as it likes, and *returns* a status byte as the function value, with extra data in the global `idereturn$`. All IDE state survives between calls because it lives in `DIM SHARED` globals (`ide_global.bas`) or `STATIC` locals of `ide2` (L83-95, L775, L1416, L1997, L2222, L2396 ...). Non-STATIC locals of `ide2` (`ready`, `failed`, `sendnextline`, `skipdisplay`, `passback`, `change`, `showexecreated`, `mustdisplay`...; `compfailed` is NOT local, it is `DIM SHARED` at qb64pe.bas:380) are re-zeroed on each entry — the code relies on this (e.g. `failed`/`ready` are set from the command byte at the top of every entry).

The protocol header comment is at L1-33.

### 1.2 `FUNCTION ide` (L35-80) — fast path

`ide` is a thin wrapper that avoids entering the large `ide2` for the most common message (3 = "give me the next line"):

- L39: `cmd = ASC(idecommand$)`.
- L40-77: if `cmd = 3` and there are lines left (`idecompiledline < iden`, L41) and the line just compiled is **off-screen** (`idecompiledline < idesy OR > idesy + (idewy - 9)`, L42), and no exit request (`_EXIT AND 1` sets `ideexit = 1`, L43), it calls `GetInput` (L45). If there is no input (`iCHANGED = 0 AND mB = 0`, L46):
  - L47-48: applies only the *indent* part of auto-layout: `indented$ = apply_layout_indent$(idecompiledline$)`; if different, `idesetline idecompiledline, indented$`. (Full layout – keyword capitalisation – is not applied on this fast path; see §2.)
  - L50-53: advances `idecompiledline`, fetches `idecompiledline$ = idegetline(idecompiledline)`, returns **4** with `idereturn$` = the line.
  - L56-65: if a run was requested (`ideautorun`) or a manual check (`idemanualcheck`), computes a percentage progress string. `prepass` (compiler global) selects first half (0-50%) vs second half (50-100%); result stored as `IdeInfo = CHR$(0) + status.progress$` and painted by `UpdateIdeInfo` (L66).
  - L68 `EXIT FUNCTION`.
  - L70: if input *did* arrive, `iCHECKLATER = 1` so that `ide2`'s `GetInput` re-processes it.
- L73-75: at the last line, clears `IdeInfo` (unless it holds a "Selection length = " message while in help window).
- L79: everything else falls to `ide = ide2(0)`.

### 1.3 Commands handled by `ide2` (compiler -> IDE)

`c$ = idecommand$` at L101. Dispatch is a linear series of `IF`s between L104 and L880, *before* the main loop at L962.

| Code | Payload | Handled at | What the IDE does |
|---|---|---|---|
| `""` (empty) | – | L203-652 | First call. `idelaunched = 0` triggers first-run initialisation (§1.7); then falls into main loop. |
| 1 | file name | L575-636 | Only honoured inside first-run init (inside `IF idelaunched = 0`), and only if no autosave was restored (`ideunsaved <> 1`, L574). Loads the file (inline copy of `ideopen` logic). |
| 3 | – | fast path L40-77; full path L654-766 | "Previous line OK; send the next". Sets `skipdisplay = _TRUE`, `sendnextline = 1` (L655-656), then consumes the compiler global `layout$` to auto-format the just-compiled line (§2.4). The next line is actually returned later, from the idle branch of the loop (L1617-1629, return 4) – only if no input is pending. |
| 6 | – | L768-773 | Compile finished OK. `idecompiling = 0`, `ideFirstCompileFromDisk = 0`, `ready = 1`. If `ideautorun` was set (user asked to run while still checking): clear it and `GOTO idemrunspecial` (L772 -> L1746) to issue the run request immediately. Otherwise "OK" is painted at L893-906. |
| 7 | – | L846-850 | Repass (compiler needs to recompile from line 1): `skipdisplay`, `idecompiledline = 0`, `sendnextline = 1`. Next idle iteration returns 4 with line 1. |
| 8 | message + `MKL$(line)` | L866-871 (flags) and L1066-1134 (display) | `idecompiling = 0`, `failed = 1`, `ideautorun = 0`. Display: message = `MID$(c$, 2, LEN(c$) - 5)`; line = `CVL(RIGHT$(c$, 4))` (L1080-1082); see §2.5/§7.4. |
| 10 | a line of code | L873-880 | Include passback. `sendnextline = 1`, `idecompiledline = idecompiledline - 1`, `passback = 1`, `passback$ =` payload. The idle branch (L1622-1624) then returns 4 with `idereturn$ = passback$` rather than a buffer line (and re-increments `idecompiledline`). Net effect: the IDE echoes the compiler's own line back, keeping the "IDE yields between lines" behaviour during `$INCLUDE` processing so the UI stays responsive. |
| 11 | – | L838-844 | EXE created (after a "make exe only" run request). `idecompiling = 0`, `ready = 1`, `ideautorun = 0`, `showexecreated = 1` -> status shows ".EXE file created" + clickable "Location: ..." (L907-926). |
| 12 | exe name (no extension) | L170-189 | Compiler tells the IDE the final exe name. IDE repaints status "Creating .EXE file named "name.exe"..." and **immediately returns 9** with `idereturn$ = f$` (L187-188). Handled before first-run init, so does not enter the loop. |
| 13 | – | L852-864 | `$NOPREFIX` found. If this is the first compile of a file loaded from disk (`ideFirstCompileFromDisk`) and `OfferNoprefixConversion%(path)` returns true (file was converted & reloaded) -> reset state (`ideunsaved = -1: idechangemade = 1: idelayoutallow = 2: ideundobase = 0: QuickNavTotal = 0: ModifyCOMMAND$ = "": ...`, L856) and `GOTO ideloop` (new compile starts). Otherwise **returns 14** (L861) so the compiler raises an error. |
| 100 | – | L191-201 | Line-continuation fetch. Increments `idecompiledline`; `idereturn$ =` that line or `""` if past the end. `EXIT FUNCTION` with **no return value assigned** (function result is 0/ignored by caller). No UI processing at all. |
| 254 | – | L797-836 | Compile finished OK and `$DEBUG` run launched: `IdeDebugMode = 1`, close help window if open (L804-811), `GOSUB redrawItAll`, `_RESIZE OFF`, call `DebugMode` (L818; SUB is at L6976+, other section). On return (`ExitDebugMode:` L819): `IdeDebugMode` 1 = clean exit -> reset; 2 = right-click inside debugger -> build contextual menu and `GOTO showmenu` (L827-831). Then `GOTO ideloop`. |
| 255 | – | L104-165 | "QB error inside IDE". The command byte is not tested; instead `ideerror <> 0` on entry triggers the error report (§1.6). |

Also on every entry (not command-specific):
- L102 `debugnextline = 0`.
- L776-788: if the compiler's `$DEBUG` flag is on (`GetRCStateVar(vWatchOn) = 1`) and hosting was not yet attempted, open TCP host: `host& = _OPENHOST("TCP/IP:" + hostport$)` with port `idebaseTcpPort + tempfolderindex`, exported via `ENVIRON "QB64DEBUGPORT="`.
- L790-795: if `IdeDebugMode` is still non-zero on entry (i.e. `DebugMode` itself raised an error and we re-entered via 255), redraw and jump to `ExitDebugMode`.

### 1.4 Return values (IDE -> compiler)

| Value | `idereturn$` | Where set | Meaning |
|---|---|---|---|
| 0 | – | L191-201 (cmd 100, no assignment); quit path does not return at all — see below | (Compiler treats 0 as "no IDE"). |
| 2 | first line of code | `beginCompile:` L1383-1389 | Begin new compilation. Sets `idecompiling = 1`, `idecompiledline = 1`, `idecompiledline$ = idegetline(1)`. |
| 4 | next line (or passback line) | L52 (fast path); L1621-1629 (idle branch) | Next line. |
| 5 | – | L1633-1637 | No more lines. |
| 9 | exe base name (no path/extension; `"untitled" + tempfolderindexstr$` if unnamed) | L187 (after cmd 12); L1884 (run request) | "C++ compile if necessary and run". What actually happens (run / exe only / detached / debug) is carried by **globals**, chiefly `iderunmode` (1 = run, 2 = make exe only; no other value is ever assigned in ide2), `NoExeSaved` ("Run only"), `idecompiled`, `startPaused` — see §7. |
| 14 | – | L861 | `$NOPREFIX` not converted; compiler should raise an error. |

Quit: the IDE never "returns 0" to quit. The exit path (`quickexit` / File > Exit, see §6/§8) ends the process with `SYSTEM` from inside `ide2` (see §8.6 for line refs).

### 1.5 Re-entry labels

`ide2` is one enormous body with ~100 labels. The important control-flow labels:

| Label | Line | Role |
|---|---|---|
| `IDEerrorMessage:` | L104 | Error report entry; also jumped to explicitly (e.g. L579, L583) after setting `ideerror` to a specific code. |
| `errorReportDone:` | L164 | Skip the message box. |
| `reInitIDE:` | L203 | Re-run first-run init (used after importing config, L1219; also from Display options, see §6). |
| `redraweverything:` / `redraweverything2:` | L531 / L538 | Reset cursor/view (+ redraw). |
| `skipload:` | L640 | End of init. |
| `EnterDebugMode:` / `ExitDebugMode:` | L803 / L819 | Debugger hand-off. |
| `ideloop:` | L963 | Top of main loop (full redraw unless `skipdisplay`). |
| `beginCompile:` | L1383 | Start syntax check -> return 2. |
| `waitforinput:` | L1400 | Idle/poll point (no redraw). |
| `idemexe:` / `idemrun:` / `idemrunspecial:` | L1722 / L1743 / L1746 | Run entry points. |
| `specialchar:` | L4459 | End-of-iteration: selection-length status, AltSpecial reset, then `LOOP` (L4510) back to `ideloop`. |
| `startmenu:` / `startmenu2:` / `showmenu:` | L4514 / L4517 / L4648 | Menu bar navigation / pull-down menu. |
| `menuChoiceMade:` | L5039 | Start of the menu item handler chain. |
| `quickexit:` | L6501 | Exit (window close or File > Exit). |
| `redrawItAll:` (GOSUB) | L6826 | Rebuild menubar$, frame, scrollbars, text, status. |

Return to the compiler happens only at these `EXIT FUNCTION`s: L188 (9), L200 (cmd 100), L862 (14), L1389 (2), L1629 (4), L1637 (5), L1885 (9). (L6747 is an unreachable guard before the GOSUB routines.) The process ends at L6525 `SYSTEM` (and L6738 `END` for an unimplemented menu item).

### 1.6 The `ideerror` trap

`ideerror` is `DIM SHARED ... AS LONG` in qb64pe.bas:396. It doubles as "we are inside the IDE" flag and "what kind of failure to report":

- The compiler sets `ideerror = 0` immediately after every `ide(0)` return (qb64pe.bas:856, 25527). `ide2` sets `ideerror = 1` near its top (L167). So while IDE code runs, `ideerror <> 0`.
- The compiler's global `ON ERROR` handler tests `IF ideerror THEN` (qb64pe.bas:14263): it appends ERR/message/line to `internal\temp\ideerror.txt`, sets `sendc$ = CHR$(255)` and `RESUME sendcommand` (qb64pe.bas:14273-14274), which calls `ide(0)` again **with `ideerror` still holding its value**. All non-STATIC locals of the interrupted `ide2` invocation are lost; the shared state (buffer, cursor) is kept.
- On re-entry `IDEerrorMessage:` (L104-165) maps the value to text:

| `ideerror` | Text | Line | Set by (examples) |
|---|---|---|---|
| 1 | "Internal IDE error" (+ "(module: X, on line: N)" or "(on line: N)" detail from `_INCLERRORFILE$`/`_INCLERRORLINE`/`_ERRORLINE`, L147-159) | L111 | default, L167; restored after risky sections (L631, L2506, idesave 12965) |
| 2 | "File not found" | L112 | L582 before `_FILEEXISTS` test |
| 3 | "File access error" (also `CLOSE #150`) | L113 | L586 before reading file |
| 4 | "Path not found" | L114 | set outside ide2 (file dialog / idezgetfilepath) (unverified where) |
| 5 | "Cannot create folder" | L115 | outside ide2 (unverified) |
| 6 | "Cannot save file" | L116 | `idesave` L12962 |
| 7 | "Cannot export file" | L117 | export code (other section) |
| -1 | fail quietly ("like ON ERROR RESUME NEXT") | L118 | L2481 around help copy-to-clipboard |

- For codes > 1 the title is "Error N (errline-inclerrline[-Version])" (L127-135). If the failure came from loading a recent file (`AttemptToLoadRecent`, STATIC, set L6590/L6617) the IDE instead offers "Remove broken links from recent files?" and runs `GOSUB CleanUpRecentList` (L136-145, routine L6805-6824).
- Code sets a specific value *before* a risky operation and resets to 1 afterwards; callers also test `IF ideerror > 1 THEN GOTO IDEerrorMessage` after dialogs/saves (L579, L854, L6521, L6550, L6659, L6668, L6706, L6729) — i.e. `ideerror` is also used as an ordinary error return from SUBs without any runtime error occurring.
- After the report, L167 resets to 1 and execution continues *down through the same entry code* (with `c$` = CHR$(255), which matches none of the command tests), `mustdisplay = _TRUE` (L110) forces a redraw, and the loop resumes. A compile in progress is not resumed explicitly: `idecompiling` stays as it was but `sendnextline` is 0, so compilation stalls until the next edit sets `idechangemade` (observation from code reading; not tested).

### 1.7 First-run initialisation (L203-652)

Guarded by `IF idelaunched = 0` (L204); `idelaunched` is shared, so it runs once (or again when forced by `idelaunched = 0: GOTO reInitIDE`, L1219).

1. L207-208: `WIDTH idewx, idewy`; `_FONT 8` or `_FONT 16` per `IDEUseFont8`. (`idewx`/`idewy` come from config; the SCREEN itself and palette are set before `ide` is first called — not in this range, **unverified** where.)
2. L211-217: codepage: for chars 128-255, `_MAPUNICODE` from the hex table `idecp(idecpindex)` (8 hex digits per char; 0 -> U+2610 (9744)).
3. L219-236: custom TTF font: `_LOADFONT(IDECustomFontFile$, IDECustomFontHeight, "MONOSPACE")`; on failure, message box, revert to defaults and write config.
4. L238-506: build all static menus (§6). L468 `menus = m` (count of visible top-level menus); hidden menus follow (contextual, line-number submenu, export submenu).
5. L508-516: `idepathsep$`, `ideroot$ = idezgetroot$`, `idepath$ = _STARTDIR$`.
6. L519-529: empty buffer `idet$ = MKL$(0) + MKL$(0): idel = 1: ideli = 1: iden = 1: IdeBmkN = 0`; reset breakpoints, skip lines, watch lists, call stack; `ideunsaved = -1`, `idechangemade = 1`.
7. L531-539: `redraweverything:` cursor/view to 1,1; `GOSUB redrawItAll`.
8. L541: `IF retval = 1 THEN GOTO skipload` (re-init case).
9. L545-572: **crash recovery** — if the flag file `AutosaveFile$` exists, `iderestore$` asks "Recover program from auto-saved backup?" (L12952-12959). On "Y": open `UndoFile$`, read 12-byte header, seek to `p2` (newest state) and load the whole editor state from it (same record layout as undo, §8.1); `ideunsaved = 1`.
10. L574-636: otherwise, if `c$` starts with CHR$(1): resolve path (`idezgetfilepath$`), check exists (`ideerror = 2`), `BinaryFormatCheck%` (old QB binary format; >0 skips loading), then load via `lineinput3load`/`lineinput3$` (compiler-side fast line reader, qb64pe.bas:28055) into `idet$`, expanding TABs to 4-column stops inline (L602-610). Honours `IDEStartAtLine` (the `-l:` command-line switch, qb64pe.bas:14426) to position the cursor (L624-629). Sets `ideprogname`, `_TITLE`, imports bookmarks (`IdeImportBookmarks`), adds to recent list (`AddToHistory "RECENT"`), `ideFirstCompileFromDisk = -1`.

Because `idechangemade = 1`, the first pass through the loop writes the first undo record and returns 2 (begin compile).

Two first-time message boxes are shown inside the loop, not in init: "import old configuration?" (`askToCopyOther`, L1203-1221; on yes `CopyFromOther: ReadInitialConfig`, then re-init) and the antivirus whitelist notice (`WhiteListQB64FirstTimeMsg`, L1223-1237).

---

## 2. As-you-type syntax checking and auto-layout

### 2.1 State variables

| Variable | Meaning |
|---|---|
| `idechangemade` | Set to 1 by every text edit (and by anything requiring a recompile: option toggles, undo/redo, file load). Consumed at L1239-1396. |
| `ideunsaved` | 0 saved, 1 modified, -1 "the pending `idechangemade` is not a user modification" (file just loaded / new / option toggled). L1252: `IF ideunsaved = -1 THEN ideunsaved = 0 ELSE ideunsaved = 1`. Shown as `*` in the title (L6968). Bookmark changes set it to 1 directly (L3034, L3044). |
| `idecompiling` | 1 while a syntax-check pass is running (L1384); cleared on codes 6, 8, 11, 254 and in manual mode (L1392). |
| `idecompiledline` / `idecompiledline$` | Number/text of the last line handed to the compiler. |
| `sendnextline` (local) | Set by codes 3, 7, 10: "compiler is waiting for a line". |
| `ready` / `failed` (locals) | Result of the pass for this invocation (code 6/11/254 vs 8). |
| `idelayoutallow` | Counter permitting the layout to be applied even to the line holding the cursor (set to 2 by paste L3736, file open L6671, layout-options change L5436, $NOPREFIX conversion L856; decremented once per change at L1241). |
| `idecurrentlinelayout` / `idecurrentlinelayouti` | Deferred formatted text for the cursor line and its line number. |
| `ideautorun` | "Run as soon as the check completes" (L1891/L1896; consumed L772). |
| `idemanualcheck` | Shift+F9 manual check in progress (L1699); enables % progress. |
| `idenoundo` | Next change must not create an undo record (undo/redo themselves). |
| `idefocusline` | Line to paint as error line. |
| `statusarealink` | Which hyperlink (if any) is live in the status area (§5.6). |

### 2.2 Detecting a change and starting a pass

Each edit path sets `idechangemade = 1` and then jumps to `specialchar` or `ideloop`. At the top of the next iteration, after the redraw, L1239-1396 runs:

1. L1241 decrement `idelayoutallow`; L1243-1251 clear `watchpointList$`, `idecurrentlinelayouti`, `idefocusline`, `idechangemade`, `idemanualcheck`, `ideautorun`, `ready`, `failed`, `compfailed`.
2. L1252 update `ideunsaved`.
3. L1254-1378 write an undo record unless `idenoundo` (§8.1).
4. L1380-1394: if `IDEShowErrorsImmediately` ("Auto-Check Syntax" option) -> `beginCompile:` sets `idecompiling = 1`, returns **2** with line 1. Otherwise (manual mode) `idecompiling = 0: idecompiled = 0` and the status shows "Manual Syntax Check Mode, press Shift+F9 to trigger..." (L1139). Shift+F9 (L1697-1703) or a run request (L1890-1894) jump to `beginCompile`.

There is **no debounce**: every keystroke that modifies text restarts the compile from line 1 on the following loop iteration. The cost is bounded because the compiler only gets one line per `ide()` call and yields to input each time.

### 2.3 Feeding lines / what interrupts a pass

- Compiler returns from line N with command 3. Fast path (`ide`, L40-70) for off-screen lines when no input is pending; else `ide2`.
- In `ide2` with `c$ = CHR$(3)`: layout handling (L654-766), then the loop is entered with `skipdisplay = _TRUE` (no repaint unless the layout changed a visible line, L752). At `waitforinput` (L1400) `GetInput` is polled; if nothing changed (`change = 0`, L1614) and `idecompiling AND sendnextline`, the next line is returned immediately (L1617-1629, return 4; or 5 at end, L1633). So during a pass the IDE processes **one input poll per compiled line**; there is no `_LIMIT` on this path.
- If there *was* input (`change = 1`) the event is handled normally. If it did not modify text, the loop comes round again and, when input goes quiet, L1617 resumes feeding lines (the compiler is simply left suspended inside its `ide(0)` call meanwhile). If it did modify text, `idechangemade` triggers return code **2**, which the compiler treats as "abandon everything, full recompile" (qb64pe.bas:860-863). That is the whole interruption mechanism: the compiler is never told "cancel", it is told "begin" again.
- `iCHECKLATER` (set at L70 and L4638): makes the next `GetInput` return immediately without polling (L18843), so that an event already fetched is seen again by the next consumer.
- Two-pass structure is driven by the compiler: after return 5 it sends 7 (repass) once (qb64pe.bas:905-912), the IDE rewinds (`idecompiledline = 0`, L848), and after the second 5 the compiler sends 6 (or 8).

### 2.4 Auto-layout write-back (L658-764, command 3 only)

The compiler leaves the formatted version of the line just compiled in its global `layout$` (leading spaces encode the indent *level*; internal token separators are the compiler's `sp` and `sp2` characters). Steps:

1. L663-669: count leading spaces -> `indent` (level), strip them.
2. L671-681 (`spacelayout:`): outside string literals replace `sp` with a space and delete `sp2`.
3. L683-697: `indent$` = `SPACE$(indent * IDEAutoIndentSize)` if `IDEAutoIndent`, else the line's existing leading whitespace.
4. L699-710: if `IDEAutoLayout = 0` (spacing/case layout off) the body is taken from the original line (only indent applied) ...
5. L712-743: ... but then a character-walk compares it with the compiler's layout (`olay$`) and still applies three things: expand `?` to `print` (L730-735), add a missing closing quote (L736-739), and adopt keyword **case** differences (L740-741, letters differing only by case take the compiler's case). So keyword capitalisation is applied even with auto-layout off (when auto-indent is on).
6. L745: `layout$ = indent$ + layout$`.
7. L747-762: if the line is **not** the cursor line (`idecy <> idecompiledline`) or `idelayoutallow <> 0`: write immediately with `idesetline idecompiledline, layout$` (and force a redraw only if the line is within the first 17 visible rows, L752 — note the hard-coded 16). Otherwise store in `idecurrentlinelayout`/`idecurrentlinelayouti` (L757-760) — the line being typed is never reformatted under the cursor.
8. L1403-1409 (`waitforinput`): as soon as the cursor leaves that line (`idecy <> idecurrentlinelayouti`) the deferred layout is written with `idesetline` and a redraw forced. Any new edit invalidates it (L1244).

Why this does not disturb cursor/undo/compile: `idesetline` (L12986-12994) only splices `idet$`; it does not touch `idecx/idecy`, selection, `idechangemade` or `ideunsaved`. Hence layout rewrites do **not** create undo records, do not mark the file modified and do not trigger a recompile. They become part of the next undo snapshot implicitly (snapshots are whole-buffer). The cursor column is not adjusted when the indent changes on a line the cursor later returns to. On the fast path (L47-48) only `apply_layout_indent$` (utilities/format.bas:2) is used — it reads `layout$` itself (not verified in detail) — so off-screen lines get indent only; full layout of such lines happens only when the line goes through `ide2` (i.e. when on screen or when input was pending).

When an error occurs the compiler clears `layout$` (qb64pe.bas:14310).

### 2.5 Showing results in the status area

The status area is rows `idewy-3 .. idewy-1` inside the lower box; row `idewy-4` is its top border carrying " Status " and the Find field.

| Situation | Display | Lines |
|---|---|---|
| Change made, auto-check on | `...` (three CHR$(250)) | L1136-1141 |
| Change made, manual mode | "Manual Syntax Check Mode, press Shift+F9 to trigger..." | L1139 |
| Run requested during check | "Checking program... (editing program will cancel request)" + `NNN%` progress in the bottom status bar via `IdeInfo = CHR$(0) + status.progress$` | L1900-1909; L56-65 |
| Code 6 | "OK"; plus, if `totalWarnings > 0`, cyan " (N warning(s) - click here or Ctrl+W to view)" with `statusarealink = 4` | L893-906 |
| Code 11 | ".EXE file created" + "Location: path" (cyan, `statusarealink = 3`) | L907-926 |
| Code 8, compile error | message via `printWrapStatus` (word-wrap, L21217); if line `l <> 0`, `idefocusline = l`; if cursor on that line append " on current line", else cyan " on line N (click here or Ctrl+Shift+G to jump there)" with `statusarealink = 2`; then, if room, "Caused by (or after): " + compiler global `linefragment` (with `sp` -> space; "SUB VWATCH..." shown as "End of Program") | L1066-1134 |
| Code 8, C++ failure | see §7.4 | L1073-1078 |
| `$DEBUG` required | yellow text on row `idewy-2` | e.g. L6252-6257 |

Colour markers embedded in the message are interpreted by `printWrapStatus`: CHR$(0), CHR$(1), CHR$(2) switch colour (L21244-21278: 0 -> toggle COLOR 11/7, 1 -> COLOR 7,1, 2 -> COLOR 12,6). L1089-1099 inserts CHR$(2) after " - Reference: " or "Expected " to highlight the referenced token. `errininc = INSTR(a$, "included")` (L1101) remembers that the error is inside an include file (used by F5 logic, L1821).

While a pass is running "File > Export As..." is disabled (`menu$(1, FileMenuExportAs) = "~#Export As..."`, L1140, L6890) and re-enabled on completion (L895, L1069).

---

## 3. Main loop structure

`DO` at L962 ... `LOOP` at L4510. One iteration = redraw (optional) + wait for one input event + handle it.

| Phase | Lines | Notes |
|---|---|---|
| Loop top | L963-967 | `maxLineNumberLength`, `idecontextualmenu = 0`, `idedeltxt`, `_RESIZE ON/OFF` depending on `idesubwindow`. |
| Resize handling | L969-1047 | Only after 1.5 s uptime. Inner loop recomputes `idewx/idewy` from `_RESIZEWIDTH\_FONTWIDTH` (min 80x25, max 1000), `WIDTH`, re-applies palette, redraws frame, "Resizing...", `_LIMIT 15`, until `_RESIZE` stops; writes config; `GOSUB redrawItAll`. Deferred (`ForceResize`) while the help subwindow is open (L970-971). |
| Redraw (unless `skipdisplay`) | L1049-1201 | title, QuickNav button, search bar, cursor shape (insert mode), error message, "..." if changed, `ideshowtext` (L1143: text, scroll-follow-cursor at L13125-13128, syntax colours, bracket highlight, selection), help pane (L1145-1171), cursor placement for IdeSystem 2/3 (L1176-1194), `PCOPY 3, 0`. All drawing is to page 3, shown by copying to page 0. |
| First-time dialogs | L1203-1237 | |
| Change processing | L1239-1396 | undo record + begin compile (may `EXIT FUNCTION`). |
| `waitforinput:` | L1400-1453 | pending run (`startPausedPending`), deferred layout, window-close (`_EXIT` -> `quickexit`), `GetInput`, compute `change`, `_RESIZE` check, window-position persistence (`IDEAutoPosition`), drag-and-drop (L1446-1453). |
| Hover / status-bar clicks | L1455-1565 | QuickNav back button (row 2, cols 4-6) popup + click; "Find" label hover; version string click -> About; line-number field click -> `idegotobox`. |
| Focus/cursor, Alt highlight | L1567-1612 | palette dims bracket/current-line colours when unfocused; Alt press highlights menu initials, Alt release -> `startmenu`. |
| Idle branch (`change = 0`) | L1614-1644 | feed compiler (return 4/5) or `_LIMIT 16: GOTO waitforinput`. |
| IdeSystem-independent keys | L1646-1993 | status links, Shift+F9, F7/F8, F9, F10, F11, F12, F4, F5 (run logic to L1910), menu-bar click, Alt+letter, Ctrl+F, Ctrl+K, Ctrl+F3, Alt+F3, F3, Shift+F1. |
| Help scrollbars | L1996-2123 | |
| Find-field click | L2130-2173 | |
| Window switching | L2177-2203 | F6; click in other pane; right-click -> `invokecontextualmenu`. |
| **IdeSystem = 2** (quick search field) | L2205-2380 | ends with `GOTO specialchar`. |
| **IdeSystem = 3** (help window) | L2382-2839 | ends with `GOTO specialchar`. |
| **IdeSystem = 1** (editor) | L2843-4458 | F1 help, bookmarks, mouse, Ctrl combos, undo/redo, clipboard, navigation, Enter, Delete, Backspace, Tab, character insert. |
| `specialchar:` | L4459-4509 | selection-length info in `IdeInfo`; `ideCurrentSingleLineSelection` for multi-highlight; reset `AltSpecial`. |
| Menu system | L4514-6744 | outside the DO loop; entered by GOTO, exits by `GOTO ideloop`/`specialchar`. |
| GOSUB routines | L6748-6959 | `DrawQuickNav`, `UpdateSearchBar`, `CleanUpRecentList`, `redrawItAll`, `HelpAreaShowBackLinks`, `showVarListReady`. |

`IdeSystem`: 1 = editor, 2 = Find field in the status border, 3 = help pane. `idehelp = 1` when the help pane exists; opening it sets `idesubwindow = idewy \ 2: idewy = idewy - idesubwindow` (L1984, L2913, L5670), i.e. **`idewy` is redefined as the height of the editor part** and all editor geometry is relative to it.

### Timing / throttling

- Idle: `_LIMIT 16` (L1642) per poll -> ~16 Hz input polling when nothing happens. Events are handled one per iteration with a full redraw each.
- While compiling: no limit; one `GetInput` per line.
- Resize loop `_LIMIT 15` (L1033); menus `_LIMIT 100` (L4552, L4826, L6742); held scrollbar arrows/drag-scroll `_DELAY 0.1` (e.g. L3361, L3453) with `idembmonitor = 1` forcing `change` while the button is held (L1429).
- Double-click window: 0.5 s (`timeElapsedSince#(last.TBclick#) > 0.5#`, L3133).
- **Auto-save cadence: there is no timer.** The backup is the undo file, written synchronously on *every* change (L1275-1363). See §8.2.
- `GetInput` (L18840+) flushes `INKEY$`, takes **one** `_KEYHIT` per call (remaining mouse/keys handled beyond L18905, not studied in detail).

---

## 4. Keyboard shortcuts

Input variables (set by `GetInput`): `K$` (INKEY$-style string: 1 char, or CHR$(0)+scancode), `KB` (`_KEYHIT` code), `KSHIFT`, `KALT`, `KCTRL` (physical Ctrl), `KCONTROL` (Ctrl, or the Apple/Cmd key on macOS, L18891-18898), `KSTATECHANGED`, `AltSpecial` (Alt+numpad ASCII entry, L18858-18871).

### 4.1 Global (any IdeSystem) — L1697-1993

| Key | Action | Line |
|---|---|---|
| Shift+F9 | Manual syntax check (only if auto-check off): `idemanualcheck = 1`, `GOTO beginCompile` | L1697 |
| F7 / F8 | Start paused (`startPausedMenuHandler`) | L1705 |
| F9 | Toggle breakpoint (`toggleBreakpoint`) | L1709 |
| F10 / Ctrl+F10 | Clear all breakpoints / unskip all lines | L1713 |
| F11 | Make EXE only (`idemexe`) | L1721 |
| F12 | Call stack dialog (message if empty) | L1727 |
| F4 | Watch list (`showWatchList`) | L1737 |
| F5 (also Shift+F5) | Run | L1741 |
| Alt (press+release) | Enter menu bar (`startmenu`) | L1577-1608 |
| Alt+initial letter | Open that menu (`showmenu`) | L1930-1938 |
| Ctrl+F | Focus Find field (IdeSystem = 2), select existing text | L1940 |
| Ctrl+K | Insert quick keycode (`ideQuickKeycode`, L5561) | L1947 |
| Ctrl+F3 | Find dialog (`idefindjmp`, L5927) | L1953 |
| Alt+F3 | Change dialog (`idefindchangejmp`, L5943) | L1958 |
| F3 or `K$ = CHR$(28)` (Ctrl+\\ ; **unverified** which physical key yields 28) ; Shift+F3 reverses (`idefindinvert = 1`) | Repeat find (`idemf3` -> `idefindagain -1`); opens Find dialog if no search text | L1963-1980 |
| Shift+F1 | Open/focus help pane | L1982 |
| F6 | Toggle focus editor <-> help | L2177 |

### 4.2 Find field (IdeSystem = 2) — L2205-2380

Up/Down/PgUp/PgDn/wheel return focus to editor (L2208-2213); Alt+Up/Alt+Down open the search-history box (L2207); Ctrl+V / Shift+Ins paste first line of clipboard (L2220-2245); Ctrl+A (L2247); Ctrl+C / Ctrl+Ins (L2257); Ctrl+X / Shift+Del (L2267); Backspace (L2283); Esc and Tab leave the field (L2301, L2305); Enter = find next + add to history (L2309); F2 = SUBs dialog (L2335); Del (L2340); Left/Right/Home/End with Shift-selection via `selectcheck` (L2361-2367); typed chars inserted/replacing selection (L2316-2331). State: `idefindtext`, STATIC `idesystem2.v1` (cursor), `.sx1` (selection anchor), `.issel`.

### 4.3 Help pane (IdeSystem = 3) — L2382-2839

Esc closes help (L2384-2393); Ctrl+A select all (L2465); Ctrl+C / Ctrl+Ins copy selection (L2479-2508); Shift+movement selects (L2536-2544); Tab = next match for incremental link search (L2547); typing letters/digits/`$`/space = incremental search of link text with 1 s timeout (L2551-2635); Ctrl+Home/End, Home/End, PgUp/PgDn, arrows (L2643-2666); wheel (L2669-2671); Backspace = history back (L2690); Enter on a link opens it (L2726; `EXTL:` external URL via SHELL, `PAGE:` wiki page with optional `#anchor`).

### 4.4 Editor (IdeSystem = 1)

| Key | Action | Line |
|---|---|---|
| F1 | Context help on word at cursor (`getWordAtCursor$`, `findHelpTopic$`, `idef1box$` if ambiguous); falls back to generated help page for a user SUB/FUNCTION found by scanning the buffer | L2843-3020 |
| Alt+Left | Toggle bookmark on current line | L3024 |
| Alt+Down / Alt+Up (no Shift) | Next / previous bookmark (wraps) | L3049 |
| Alt+Right | reserved, no-op | L3083 |
| Alt+0..9 | swallowed (Alt+numpad ASCII entry) | L3089 |
| Ctrl+A | Select all | L3464 |
| Ctrl+G | Go to line dialog (`idegotobox`) | L3473 |
| Ctrl+Shift+G | Jump to error line (`idefocusline`) | L3474 |
| Ctrl+L | Library Explorer (only if `LibExplorer$` set); waits for Ctrl release | L3486 |
| Ctrl+N / Ctrl+O / Ctrl+S | New / Open / Save (`ctrlNew` L6530, `ctrlOpen` L6639, `ctrlSave` L6693) | L3497, L3501, L3526 |
| Ctrl+P | Toggle skip line | L3506 |
| Ctrl+R / Ctrl+Shift+R / Ctrl+T | Add comment / remove comment / toggle comment | L3510, L3518, L3514 |
| Ctrl+D | Duplicate line or selected lines | L3522 (code L5075) |
| F2 | SUBs dialog (`idesubsjmp` L5811) | L3530 |
| Ctrl+F2 | Quick-navigation: back to previous position | L3531-3540 |
| Ctrl+W | Warnings dialog | L3546 |
| Ctrl+Z / Ctrl+Y | Undo / Redo | L3558 / L3645 |
| Shift+Del or Ctrl+X (selection) | Cut | L3706 |
| Del or Backspace (selection) | Delete selection (`delselect`) | L3713 |
| Shift+Ins or Ctrl+V | Paste | L3725 |
| Ctrl+Ins or Ctrl+C (selection) | Copy | L3783 |
| Ins | Toggle overwrite mode (`ideinsert`; note: `ideinsert = 1` means *overwrite*, cursor becomes block L1063) | L3791 |
| Alt+Shift+Up / Down | Move current line / selected lines up or down | L3797 / L3829 |
| Ctrl+Up / Ctrl+Down | Scroll view one line without moving cursor (cursor clamped into view) | L3816 / L3848 |
| Up / Down | Move cursor (Shift extends selection via `selectcheck`) | L3821 / L3853 |
| Left / Right | Move; cursor may go beyond end of line (virtual space); Left stops at column 1 | L3871 / L3913 |
| Ctrl+Left / Ctrl+Right | Previous / next word start (alphanumeric runs; crosses lines, skipping empty ones) | L3874 / L3916 |
| Ctrl+Home / Ctrl+End | Start / end of buffer | L3957 / L3964 |
| Home | Toggles between first non-blank column and column 1 | L3972 |
| End | End of line | L3980 |
| PgUp / PgDn | Move by `idewy - 9` lines | L3987 / L3994 |
| Shift+Enter | If the line contains `RGB(`/`RGB32(`/`RGBA(`/`RGBA32(` (or `EnteringRGB` hint is showing) open RGB mixer and insert result; else normal Enter | L4016-4053 |
| Enter | Split line; new line inherits indentation of current line; bookmarks on the line move down if cursor at column 1 | L4055-4089 |
| Del (no selection) | Delete char; at/after EOL joins next line (left-trimmed) | L4093 |
| Ctrl+Backspace (arrives as Ctrl+Del scancode on Windows, Ctrl+BS on macOS, unsupported on Linux) | Delete word left | L4113-4170 |
| Backspace | At col 1 joins with previous line (if previous line empty, keeps next line's indentation); in leading whitespace on a tab stop removes one indent unit; else one char | L4172-4241 |
| Tab (selection) / Shift+Tab (selection; CHR$(25) on macOS) | Block increase / decrease indent by `IDEAutoIndentSize` (default 4) | L4257-4361 |
| Tab (no selection) | Insert spaces to next tab stop (never a TAB char) | L4364-4367 |
| Shift+Tab (no selection) | nothing | L4369 |
| Esc | ignored | L4374 |
| Printable char | Insert / overwrite; replaces selection | L4381-4458 |

Filters before character insertion: `LEN(K$) <> 1` -> ignore (L4252); `block_chr(ASC(K$))` table rejects non-insertable codes (L4254); remaining Ctrl- or Alt-held keys ignored (L4378-4379).

### 4.5 Selection model

- `ideselect` (0/1), anchor `ideselectx1, ideselecty1`; the other end is the cursor `idecx, idecy`. View origin `idesx, idesy`. There is no separate "selection end".
- `selectcheck:` GOSUB (L4003-4011): called before every cursor movement. Shift held and no selection -> start one at the cursor; Shift not held -> `ideselect = 0`. The same routine serves the Find field.
- Selection is **stream-like but line-granular for multi-line**: if anchor and cursor are on different lines, whole lines are selected; the cursor line is included only if `idecx > 1` (see `getSelectedText$` L21048, `delselect` L21067). Single-line selections are column ranges and may extend into virtual space (padded with spaces on copy, L21044).
- An "empty" selection (anchor = cursor) is common (every plain click sets `ideselect = 1`, L3222); code checks for it explicitly (L3714-3721, L3279-3288).
- `specialchar` (L4462-4502) shows "Selection length = N character(s)/line(s)" in the bottom bar and stores a single-line selection without separators in `ideCurrentSingleLineSelection`, which `ideshowtext` uses to highlight all other occurrences (L13331+).

### 4.6 Clipboard

- Copy (L3784-3788): `clip$ = getSelectedText$(-1)`; `_CLIPBOARD$ = clip$` if non-empty. Multi-line text uses CRLF line ends, with a trailing CRLF when whole lines are selected.
- Cut (L3707-3710): sets `idechangemade = 1` then jumps to `copy2clip`; the copy code calls `delselect` when the triggering key was a cut key (L3787). The Edit-menu "Cut" fakes `K$ = CHR$(0) + "S"` to satisfy that test (L6163).
- Paste (L3726-3780): `clip$ = StripDiscordANSI$(_CLIPBOARD$)`. If it contains CR or LF -> **whole-line paste**: selection deleted, `idelayoutallow = 2`, then each line is inserted *above the current line* with `ideinsline idecy + i, converttabs$(...)` (L3741-3761; handles CRLF, LF, CR; `converttabs$` is at qb64pe.bas:28225). The text is never spliced into the middle of the current line. With `PasteCursorAtEnd` the cursor goes to the end of the pasted block (L3763-3771), otherwise it stays (now on the first pasted line). Single-line clip -> `insertAtCursor clip$` (L21087: replaces selection, pads virtual space, cursor advances only if `PasteCursorAtEnd`).
- Find field paste uses only the first line (L2222-2226).

### 4.7 Comment / indent blocks

- Add comment (L5043-5072): range = current line or selected lines (last line excluded when cursor at col 1); computes the minimum indentation `lhs` of non-empty lines and inserts `'` at that column on each non-empty line.
- Remove comment (L5098-5123): removes a leading `'` (first non-blank char) per line.
- Toggle (L5126-5166): per line — remove if it starts with `'`, else add at `lhs`.
- Block indent (L4314-4360) inserts `BlockIndentLevel` spaces at `lhs`; decrease (L4261-4312) removes `min(lhs, BlockIndentLevel)` chars and does nothing if any line is already at column 0. Single-line selections also shift `ideselectx1`/`idecx`.

### 4.8 Auto-close brackets, bracket highlight

- With `AutoCloseBrackets` (L4405-4442, insert mode only): typing `(`, `[`, `{` inserts the closer after the cursor; `"` inserts a pair only if the number of quotes before the cursor is even (L4428-4436); typing a closer or `"` when the same char is already under the cursor just steps over it (`skipInsert`, L4408-4414). No selection-wrapping and no special Backspace handling for pairs.
- Bracket matching/highlight is not in ide2: it is done during painting in `ideshowtext` (L13219-13266, palette colour 5), as are current-line highlight, `$INCLUDE` "--> Double-click to open" hint (L13302) and the Shift+Enter RGB hint (`EnteringRGB`, L13275-13289). `HideBracketHighlight` (L20861) is called before modal dialogs.

---

## 5. Mouse handling

`GetInput` provides `mX, mY` (text cell), `mB`/`mB2` (button states, swapped if `MouseButtonSwapped`), previous `mOB`/`mOB2`, `mCLICK`/`mCLICK2`, `mRELEASE`, `mWHEEL`. Editor text area = columns `2 + maxLineNumberLength .. idewx - 1`, rows `3 .. idewy - 6`. Cell -> buffer: `idecx = (mX - 1 + idesx - 1) - maxLineNumberLength`, `idecy = mY - 2 + idesy - 1` (L3223-3224).

| Gesture | Behaviour | Lines |
|---|---|---|
| Left click in text | `regularTextBox_click:` set cursor; `ideselect = 1`; anchor = cursor unless Shift held (Shift+click extends); `idemouseselect = 1`; remember position/time for double-click | L3220-3229 |
| Drag | While `mB AND idemouseselect = 1` the cursor follows the mouse (anchor fixed) | L3431-3441 |
| Drag outside text area | Auto-scroll one line/column per 0.1 s (`idembmonitor = 1` keeps the loop running) | L3443-3456 |
| Double click (same cell within 0.5 s) | Select the word under the cursor delimited by `char.sep$` (`" =<>+-/\^:;,*().`, L99); `wholeword.select = -1` | L3132-3217 |
| Double click + horizontal drag | Extend selection word-by-word (L3091-3124); moving to another row switches to normal drag (`wholeword.select = -2`, L3125-3128) and back | L3091-3128 |
| Triple click | **Not implemented** (no code found). | – |
| Double click on a `$INCLUDE` line (`ActiveINCLUDELink > 0`, set by `ideshowtext`) | Launches a second QB64-PE instance on the include file (`SHELL` blocking, with `-l:N` to jump to the error/warning line), then compares file content before/after and sets `idechangemade = 1` if changed | L3139-3196 |
| Click on line-number gutter (or left border when numbers hidden) | If `AutoAddDebugCommand` or `$DEBUG` active: toggle breakpoint on that line (Shift held -> toggle skip-line, L3240). Otherwise: select the whole line (Shift extends) | L3231-3264 |
| Right click in text | `invokecontextualmenu:` moves the cursor to the click unless the click is inside the current non-empty selection; then `IdeMakeContextualMenu`, `idecontextualmenu = 1`, `GOTO showmenu` | L3270-3325 |
| Right click in help pane | Contextual menu for help | L3326-3332 |
| Vertical scrollbar (col `idewx`) | click on thumb -> drag mode `idemouseselect = 2` (L3336-3345), drag maps row to `idecy` proportionally (L3402-3415); arrows = Up/Down key (L3361-3362); track = PgUp/PgDn (L3363-3374). The scrollbar position represents the **cursor line**, not the view. | |
| Horizontal scrollbar (row `idewy - 5`) | thumb drag `idemouseselect = 3` (L3348-3356, L3417-3429); arrows = Left/Right; track = cursor ±8 columns (L3379-3399). Range is hard-coded 608 columns. | |
| Wheel | Cursor jumps to top/bottom visible line then moves 3 lines per notch (the view scrolls because it follows the cursor); cancels/extends selection per Shift | L3860-3869 |
| Click on menu bar | open menu | L1915-1928 |
| Click on status-area link | see 5.6 | L1655-1695 |
| Click "Find" label / field / history arrow | F3 search or focus field / history box (`idesearchedbox`) | L2130-2173 |
| Click QuickNav arrow (row 2, cols 4-6) | go back | L1482-1490 |
| Click version / line-col in bottom bar | About box / Go-to-line | L1532, L1551 |
| Click in other pane | switches `IdeSystem` | L2187-2197 |
| Help pane | click = position/select start, drag = select, click on link opens it, `x` button closes, "View on Wiki" button, back-history strip on the top border | L2384-2534, L2726 |
| File drag-and-drop | `_TOTALDROPPEDFILES > 0` -> first file only: `IdeOpenFile$ = _DROPPEDFILE$(1)`, `_FINISHDROP`, `GOTO ctrlOpen` (goes through the unsaved prompt) | L1446-1453 |

### 5.6 Status-area hyperlinks (L1655-1695)

A click in rows `idewy-3..idewy-1` counts as a link click only if the character cell's attribute is cyan-on-blue: `SCREEN(mY, mX, 1) = 11 + 1 * 16` (L1657). `statusarealink` selects the action: 1 open `compilelog$` with the OS default handler; 2 jump to `idefocusline` (adds QuickNav history); 3 reveal the produced executable in the file manager (`explorer /select,` on Windows); 4 warnings dialog.

---

## 6. Menu system

### 6.1 Data model

- `menu$(1 TO 12, 0 TO 20)`, `menuDesc$()`, `menusize(1 TO 12)` (ide_global.bas:216-218). `menu$(m, 0)` = title; items `1..menusize(m)`. Hard limits: 12 menus, 20 items.
- `menus` = number of menus on the bar (9: File, Edit, View, Search, Run, Debug, Options, Tools, Help; L468). The last one (Help) is right-aligned (L1586, L6833). Hidden menus follow: `idecontextualmenuID` (L472), `ViewMenuShowLineNumbersSubMenuID` (L477), `FileMenuExportAsSubMenuID` (L494).
- Item string syntax: `#` precedes the hotkey letter; two spaces separate the label from the right-aligned shortcut text; `"-"` = separator; leading `~` = disabled; leading `CHR$(7)` = checkmark (bullet); trailing `CHR$(16)` = submenu arrow. Width calc/rendering at L4713-4770.
- Shortcut text in a menu string is **display only**; the actual key handling is the separate code in §4.
- Dispatch is by **string comparison of the item text**: `IF menu$(m, s) = "..." THEN` (L5042-6731). Checkmarked items are matched with `RIGHT$(...)`/`MID$`. An unmatched item prints "MENU ITEM [...] NOT IMPLEMENTED!" and ENDs (L6738).
- Dynamic rebuilds on every `showmenu` (L4649-4691): `IdeMakeFileMenu` (recent files, export enable), `IdeMakeEditMenu` (enable states by selection/clipboard/undo availability), warnings and call-stack items enabled/disabled.
- `menuDesc$` is shown in the bottom status bar via `UpdateMenuHelpLine` (L4736).
- `idehl = KALT` at selection time (L5040) tells dialogs whether to show hotkey highlights.

### 6.2 Navigation code

- `startmenu` (L4514-4644): menu-bar-only mode after Alt tap: Left/Right move, Up/Down/Enter open, letter opens menu, Esc/click-away leaves; other keys are re-queued (`iCHECKLATER = 1`, L4638).
- `showmenu` (L4648-6744): draws the drop-down on page 1 over a saved copy (page 2), waits for input (`_LIMIT 100`), supports hover tracking, click-release selection (L4859-4866), keyboard (L4964-5035), submenus (`idecontextualmenu` 2 = line numbers, 3 = export; Right opens, Left returns, L4973-4991), contextual menu positioned at the mouse (`idecontextualmenu = 1`). When invoked from the debugger (`IdeDebugMode = 2`) every exit path goes `GOTO EnterDebugMode`, and debugger actions are passed back by setting `IdeDebugMode` to an action code (3..16, see table).

### 6.3 Items, definitions and handlers

"Def" = line where the item string is assigned. "Handler" = line of the `IF menu$(m, s) = ...` test in ide2.

**File** (built by `SUB IdeMakeFileMenu`, L19414-19464)

| Item | Shortcut | Def | Handler | Calls |
|---|---|---|---|---|
| New | Ctrl+N | L19417 | L6528 | inline (`ctrlNew:`), `idesavenow`, `SaveFile$`/`idefiledialog$`, `idesave` |
| Open... | Ctrl+O | L19419 | L6635 | inline (`directopen:`/`ctrlOpen:`), `OpenFile$` (L21332) or `idefiledialog$("", 1)` (L12515) |
| Save | Ctrl+S | L19421 | L6691 | `idesave` (L12961) or save dialog |
| Save As... | – | L19423 | L6711 | `SaveFile$` (L21297) / `idefiledialog$(name, 2)` |
| Export As... > | – | L19427 | L5878 | opens submenu (`idecontextualmenu = 3`) |
| 1..N recent files | – | L19441 | L6585-6594 (loop over `IdeRecentLink`) | `directopen` |
| Recent... | – | L19440 | L6597 | `iderecentbox`, `AskClearHistory$`, `CleanUpRecentList` |
| Clear Recent... | – | L19456 | L6624 | `AskClearHistory$("RECENT")` |
| Exit | – | L19461 | L6499 | inline (`quickexit:`) |

Export submenu (L494-506): Hypertext (.htm) L496 -> L5883 `ExportCodeAs "html"`; Rich Text (.rtf) L498 -> L5890 `"rich"`; Discord codebox L500 -> L5897 `"disc"`; Forum codebox L502 -> L5904 `"foru"`; Wiki example L504 -> L5911 `"wiki"`.

**Edit** (built by `SUB IdeMakeEditMenu`, L19763-19895; enabled/disabled variants on alternative lines)

| Item | Shortcut | Def | Handler | Calls |
|---|---|---|---|---|
| Undo | Ctrl+Z | L19768 | L6171 | `idemundo` (L3559) |
| Redo | Ctrl+Y | L19770 | L6176 | `idemredo` (L3646) |
| Cut | Shift+Del / Ctrl+X | L19781 | L6160 | `idemcut` (L3707) / `cutToClipboardSearchField` |
| Copy | Ctrl+Ins / Ctrl+C | L19783 | L6152 | `copy2clip` (L3784) / `copysearchterm2clip` / `copyhelp2clip` |
| Paste | Shift+Ins / Ctrl+V | L19799 | L6146 | `idempaste` (L3726) / `pasteIntoSearchField` |
| Clear | Del | L19807 | L6134 | `delselect` / `deleteSelectionSearchField` |
| Select All | Ctrl+A | L19814 | L6182 | `idemselectall` / `selectAllInSearchField` / `selectAllInHelp` |
| Duplicate Line | Ctrl+D | L19818 | L5074 | inline |
| Toggle Comment | Ctrl+T | L19821 | L5125 | inline |
| Add Comment (') | Ctrl+R | L19823 | L5042 | inline |
| Remove Comment (') | Ctrl+Shift+R | L19825 | L5097 | inline |
| Increase Indent | TAB | L19846/19854 | L5168 | `IdeBlockIncreaseIndent` (L4314) |
| Decrease Indent | Shift+TAB | L19848/19856 | L5174 | `IdeBlockDecreaseIndent` (L4261) |
| New SUB... / New FUNCTION... | – | L19870 / L19872 | L5790 / L5797 | `idenewsf` (L12447) |

**View** (L245-255)

| Item | Shortcut | Def | Handler | Calls |
|---|---|---|---|---|
| SUBs... | F2 | L247 | L5805 | `idesubs` (L13701) |
| Line Numbers > | – | L249 | L5819 | submenu (`idecontextualmenu = 2`) |
| Compiler Warnings... | Ctrl+W | L253 | L5918 | `idewarningbox` (L14737) |

Line-number submenu (L475-491): Show/Hide Line Numbers L478 -> L5824 / L5835; Background Color L481 -> L5846; Show Separator L486 -> L5862 (all toggle a flag + `WriteConfigSetting`).

**Search** (L257-285)

| Item | Shortcut | Def | Handler | Calls |
|---|---|---|---|---|
| Find... | Ctrl+F3 | L259 | L5925 | `idefind` |
| Repeat Last Find | (Shift+) F3 | L261 | L6129 | `idemf3` -> `idefindagain` (L11986) |
| Change... | Alt+F3 | L263 | L5941 | `idechange`, then inline replace loop L5952-6114 with `idechangeit$` (per-match Y/N/C prompt), `FindQuoteComment`, `idechanged`, `idenomatch` |
| Clear Search History... | – | L266 | L6117 | `AskClearHistory$("SEARCH")` |
| Quick Navigation (check) | – | L270 | L5419 | toggle `EnableQuickNav` |
| Add/Remove Bookmark | Alt+Left | L276 | L5453 | inline |
| Next Bookmark / Previous Bookmark | Alt+Down / Alt+Up | L278 / L280 | L5479 | inline |
| Go To Line... | Ctrl+G | L283 | L5516 | `idegotobox` (L16262) |

**Run** (L287-326)

| Item | Shortcut | Def | Handler | Calls |
|---|---|---|---|---|
| Start | F5 | L289 | L6194 | `idemrun` (L1743) |
| Run Only (No EXE) | – | L291 | L6202 | `NoExeSaved = _TRUE`, `idemrun` |
| Make EXE Only ("Make Executable Only" on Linux) | F11 | L296 / L294 | L6241 | `idemexe` (L1722) |
| Output EXE to Source Folder (check) | – | L301 | L5339 | toggle `SaveExeWithSource`, `idecompiled = 0` |
| Generate License For EXE (check) | – | L307 | L5354 | toggle `GenerateLicenseFile` |
| Modify COMMAND$... | – | L313 | L6211 | `ideinputbox$` |
| Change Terminal... (Linux, non-mac only) | – | L316 | L5369 | `ideTerminalBox` |
| Set Default EXE Folder... | – | L319 | L6219 | `_SELECTFOLDERDIALOG$` |
| Configure Logging... | – | L321 | L5383 | `ideLoggingBox` |
| QBJS Web Build... | – | L324 | L5376 | `ideQBJSBuildBox` |

**Debug** (L328-364)

| Item | Shortcut | Def | Handler | Calls |
|---|---|---|---|---|
| Start Paused | F7 or F8 | L330 | L6246 | `startPausedMenuHandler:` inline |
| Toggle Breakpoint | F9 | L333 | L6378 | `toggleBreakpoint:` inline (`IdeBreakpoints()`) |
| Clear All Breakpoints | F10 | L335 | L6417 | `REDIM IdeBreakpoints` |
| Toggle Skip Line | Ctrl+P | L337 | L6429 | `toggleSkipLine:` inline (`IdeSkipLines()`) |
| Unskip All Lines | Ctrl+F10 | L339 | L6468 | `REDIM IdeSkipLines` |
| Watch List... | F4 | L342 | L6283 | `idevariablewatchbox$` (L8771) |
| Call Stack... | F12 | L345 | L6335 | `idecallstackbox` (L10677) |
| Auto-add $Debug Metacommand (check) | – | L349 | L5405 | toggle `AutoAddDebugCommand` |
| Output Watch List to Console (check) | – | L355 | L5391 | toggle `WatchListToConsole` |
| Set Base TCP/IP Port Number... | – | L360 | L6480 | `ideSetTCPPortBox`; closes `host&`, forces recompile |
| Purge C++ Libraries | – | L362 | L5227 | `PurgeTemporaryBuildFiles` |

**Options** (L366-424)

| Item | Def | Handler | Calls |
|---|---|---|---|
| Display... | L368 | L5187 | `ideDisplayBox` (ignored while help pane is open) |
| IDE Colors... | L370 | L5207 | `idechoosecolorsbox` |
| Code Layout... | L372 | L5433 | `ideLayoutBox`; on change `idechangemade = 1: idelayoutallow = 2` |
| Compiler Settings... | L374 | L5441 | `ideCompilerSettingsBox`; on change recompile, keep saved state |
| Language... | L377 | L5180 | `ideLanguageBox` |
| Undo/History... | L379 | L5523 | `ideLimitsBox` |
| Syntax Highlighter (check) | L383 | L5249 | toggle `DisableSyntaxHighlighter` |
| Swap Mouse Buttons (check) | L389 | L5235 | toggle `MouseButtonSwapped` |
| Cursor After Paste (check) | L395 | L5263 | toggle `PasteCursorAtEnd` |
| Auto-Close Brackets (check) | L401 | L5277 | toggle `AutoCloseBrackets` |
| Auto-Check Syntax (check) | L407 | L5291 | toggle `IDEShowErrorsImmediately`; forces recheck |
| Ignore Warnings (check) | L413 | L5308 | toggle `IgnoreWarnings`; forces recheck |
| GUI Dialogs (check) | L419 | L5325 | toggle `UseGuiDialogs` |

All toggles persist immediately with `WriteConfigSetting section$, key$, value$`.

**Tools** (L426-440)

| Item | Shortcut | Def | Handler | Calls |
|---|---|---|---|---|
| ASCII Chart... | – | L428 | L5544 | `ideASCIIbox$`, `insertAtCursor` |
| Insert Quick Keycode | Ctrl+K | L430 | L5559 | inline `_KEYHIT` capture, `insertAtCursor` |
| Library Explorer... (only if `LibExplorer$`) | Ctrl+L | L433 | L5606 | `SHELL _HIDE _DONTWAIT` |
| Math Evaluator... | – | L436 | L5699 | `ideinputbox$`, compiler `lineformat`, `Evaluate_Expression$`, `insertAtCursor` |
| RGB Color Mixer... | – | L438 | L5215 | `idergbmixer$` |

**Help** (L442-466)

| Item | Shortcut | Def | Handler | Calls |
|---|---|---|---|---|
| View | Shift+F1 | L444 | L5666 | inline open pane |
| Contents Page | – | L446 | L5640 | `OpenHelpLink` (L2862) with "QB64 Help Menu" |
| Keywords Index | – | L448 | L5645 | "Keyword Reference - Alphabetical" |
| Keywords by Usage | – | L450 | L5650 | "Keyword Reference - By usage" |
| Metacommands | – | L452 | L5655 | "Metacommand" |
| Variable Types | – | L454 | L5660 | "Variable Types" |
| Update Current Page | – | L457 | L5687 | `Wiki$` with `Help_IgnoreCache = 1`, `WikiParse` |
| Update All Pages... | – | L459 | L5773 | `ideupdatehelpbox` (L19959) |
| View Current Page On Wiki | – | L461 | L5681 | `launchWiki` (L2399) |
| About... | – | L464 | L5530 | `idemessagebox` |

**Contextual menu** (built by `SUB IdeMakeContextualMenu`, L19466-19761; content depends on `IdeSystem`, selection, word under cursor and debug mode)

| Item | Def | Handler | Action |
|---|---|---|---|
| Find 'selection' | L19516 | L5934 | `idefindtext = idecontextualSearch$`, `idemf3` |
| Go To SUB/FUNCTION name | L19601 | L5618 | jump to `CVL(MID$(SubFuncLIST(1), 1, 4))` |
| Go To Label name | L19621 | L5629 | jump to line stored in last `SubFuncLIST` entry |
| Help On 'word' | L19639 | L5613 | `contextualhelp` (L2844) |
| RGB Color Mixer... | L19661 | L5215 | as Tools |
| Cut / Copy / Paste / Clear / Select All | L19669-19685 | as Edit | |
| Toggle / Add / Remove Comment | L19688-19692 | as Edit | |
| Increase / Decrease Indent | L19706-19719 | as Edit | |
| New SUB... / New FUNCTION... | L19726 / L19728 | as Edit | |
| (help pane) Copy, Select All, Contents Page ... View Current Page On Wiki | L19732-19753 | as Edit/Help | |
| (help pane) Close Help ESC | L19756 | L6189 | `closeHelp` (L2386) |
| (debug) Continue F5 | L19474 | L6348 | `IdeDebugMode = 4` |
| (debug) Step Out F6 | L19476 | L6353 | `IdeDebugMode = 5` |
| (debug) Step Into F7 | L19478 | L6358 | `IdeDebugMode = 7` |
| (debug) Step Over F8 | L19480 | L6363 | `IdeDebugMode = 6` |
| (debug) Set Next Line Ctrl+G | L19483 | L6494 | `IdeDebugMode = 13` |
| (debug) Run To This Line Ctrl+Shift+G | L19485 | L6368 | `IdeDebugMode = 8` |
| (debug) Toggle Breakpoint / Clear All Breakpoints / Toggle Skip Line / Unskip All Lines | L19488-19494 | L6378 / L6417 / L6429 / L6468 | `IdeDebugMode = 10 / 11 / 12 / 15` |
| (debug) SUBs... / Watch List... / Call Stack... | L19497-19501 | L5805 / L6283 / L6335 | `IdeDebugMode = 14 / 16 / 3` |
| (debug) Exit $DEBUG mode ESC | L19504 | L6373 | `IdeDebugMode = 9` |

All debug items end with `GOTO EnterDebugMode` (L803), which calls `DebugMode` again; that SUB interprets the code.

---

## 7. Run flow (IDE side)

### 7.1 Entry points and flags

| User action | Code | Flags set |
|---|---|---|
| F5 / Run > Start | L1741-1746 / L6194 | `startPaused = 0`, `startPausedPending = 0`, `iderunmode = 1` |
| Run > Run Only (No EXE) | L6202-6208 | `NoExeSaved = _TRUE` (compiler global, qb64pe.bas:40), then as F5 |
| F11 / Make EXE Only | L1721-1724 / L6241 | `iderunmode = 2`, `GOTO idemrunspecial` |
| F7/F8 / Debug > Start Paused | L6248-6280 | requires `$DEBUG` (`GetRCStateVar(vWatchOn)`); if absent and `AutoAddDebugCommand`, offers to insert `$Debug` as line 1 (`ideinsline 1, SCase$("$Debug")`), sets `startPaused = -1: startPausedPending = -1` and lets the recompile happen; else `startPaused = -1: GOTO idemrun` |
| Code 6 with `ideautorun` | L772 | `GOTO idemrunspecial` (keeps previous `iderunmode`) |
| `startPausedPending` | L1401-1402 | `> 0` -> `idemrunspecial`; `< 0` -> `idemrun` |

`iderunmode` (ide_global.bas:240): only 1 and 2 are assigned. The comment at L1745 ("run detached; = 0 'standard run") shows mode 0 (blocking run) exists on the compiler side (qb64pe.bas:1064-1068) but nothing in ide2 selects it. There is **no "Start detached" menu item** in this version.

### 7.2 `idemrunspecial:` (L1746-1898)

1. L1748-1775: one-time information box about where the EXE will be written (`ExeToSourceFolderFirstTimeMsg`); Cancel aborts with "Compilation request canceled.".
2. L1779: proceed only if `(ready <> 0 AND idechangemade = 0) OR statusarealink = 2`.
   - `statusarealink = 2` means an error with a line link is displayed: if the error is not in an include file (`failed = 1 AND errininc = 0`) do nothing (L1821); if it is, force a recompile and re-request the run afterwards (`edReCompile:` L1822-1825: `startPausedPending = 1: idechangemade = 1`), because the include may have been fixed externally.
   - External dependency check (L1785-1819): the compiler-filled buffer `ExtDepBuf` lists dependency files (prefix `DECL:`, `INCL:`, `EMBE:` ...; `$EMBED` files appended here from `embedFileList$`, L1792-1796) each followed by its MD5. If a `DECL:`/`INCL:` file's MD5 changed -> full recompile (`edReCompile`); other changed files -> only rebuild EXE (`idecompiled = 0: GOTO mustGenerateExe`, L1819).
   - L1829: `NoExeSaved` forces a rebuild.
   - L1830-1861: if `idecompiled` (EXE already built for this source): mode 2 -> just report "Already created .EXE file!" + location link (L1832-1850); if the binary vanished -> rebuild; else show "Starting program...".
   - `mustGenerateExe:` L1862-1871: show "Creating .EXE file..." (screen darkened via `DarkenFGBG(1)`).
   - L1878-1885: **return 9** with `idereturn$` = program base name (`RemoveFileExtension$(ideprogname$)` or `"untitled" + tempfolderindexstr$`).
3. Not ready (L1887-1898): if `failed` do nothing. Manual-check mode: `idecompiled = 0: ideautorun = 1`, "Checking program...", `GOTO beginCompile`. Otherwise assume a check is in progress: `ideautorun = 1`, show "Checking program... (editing program will cancel request)" and wait; any edit clears `ideautorun` (L1248).

### 7.3 What comes back

Compiler handling of 9 is at qb64pe.bas:923-1082:

| Compiler reply | When | IDE reaction |
|---|---|---|
| 12 + newname | target EXE exists and cannot be deleted (still running); compiler picks `name(2)` etc. (qb64pe.bas:938-953) | L170-189: show "Creating .EXE file named ..." and return 9 again with the new name |
| 8 + "C++ Compilation failed ..." | C++ build produced no binary (qb64pe.bas:14194-14206) | §7.4 |
| 11 | `iderunmode = 2` (qb64pe.bas:962-964) | L838-844 + L907-926: ".EXE file created" + location link |
| 6 | after launching: detached run `SHELL _DONTWAIT` for mode 1 (qb64pe.bas:1063); for `NoExeSaved` a blocking `SHELL`, delete the temp EXE, reset `NoExeSaved` (qb64pe.bas:1039-1060) | L768-773: "OK" |
| 254 | program has `$DEBUG` (`vWatchOn`) | L797-836: enter `DebugMode` (uses `startPaused`, TCP `host&`) |

The compiler sets `idecompiled = 1` after a successful build (qb64pe.bas:959) and `idecompiled = 0` when sending 6 after a syntax pass (qb64pe.bas:919). IDE-side resets: L1393, L1819, L1829, L1852, L1891, L5350 (output-folder toggle), L6233 (default folder change).

### 7.4 "C++ Compilation failed" link

- Compiler: `idemessage$ = "C++ Compilation failed " + CHR$(0) + "(Check " + _TRIM$(compilelog$) + ")"` with `compfailed = 1` (qb64pe.bas:14200-14206), sent as code 8 with line 0.
- IDE: L1073-1078 — because `compfailed` is set, the message is printed with `printWrapStatus` and `statusarealink = 1`; no line number/"Caused by" processing. The CHR$(0) marker makes `printWrapStatus` switch to COLOR 11 (cyan) for the "(Check ...compilelog.txt)" part (L21244-21249, L21272-21273).
- Click: L1656-1669 — cyan attribute test, then `SHELL _DONTWAIT QuotedFilename$(compilelog$)` (Windows), `open` (macOS) or `xdg-open` (Linux).
- `compfailed` is cleared on the next edit (L1251). Note F5 again in this state: `ready = 0`, `failed = 1` -> ignored (L1888) until the source changes.

### 7.5 COMMAND$ and output location

- Run > Modify COMMAND$ (L6211-6217): `ModifyCOMMAND$ = " " + ideinputbox$(...)` (leading space so it can be appended to the command line; empty if blank). Used by the compiler when building `ExecuteLine$` (qb64pe.bas:1003, 1012, 1025-1031). Reset to "" on New (L6574), Open (L6671) and $NOPREFIX conversion (L856). Not persisted.
- Output EXE to Source Folder (L5339-5352): toggles `SaveExeWithSource`, persists it, sets `idecompiled = 0`. The compiler computes `path.exe$ = DefaultExeSaveFolder$`, overridden by `idepath$ + pathsep$` when `SaveExeWithSource` and the program has a name (qb64pe.bas:931-934).
- Set Default EXE Folder (L6219-6239): `_SELECTFOLDERDIALOG$`, persists `DefaultExeSaveFolder`, `idecompiled = 0`.
- `lastBinaryGenerated$`, `path.exe$` (compiler globals) are read by the IDE to print "Location:" and to check that the EXE still exists (L917-923, L1832-1853, L1680).

---

## 8. Buffer-level operations inlined in ide2

### 8.0 Buffer representation (context)

`idet$` is one string holding all lines, each stored as `MKL$(len) + text + MKL$(len)` (length prefix and suffix allow walking both ways; see L612, L12992). `iden` = line count; `idel`/`ideli` = cached "current line number / byte offset" used by `idegotoline` to seek relatively. `idegetline$` L12117, `idesetline` L12986 (RTRIMs the text), `ideinsline` L12252, `idedelline` L11401 (the last two also shift bookmarks and, when `$DEBUG` is on, breakpoint/skip arrays). Every single-line edit is an O(buffer) string rebuild.

### 8.1 Undo / redo

Storage: file `UndoFile$` = `<ConfigFolder>/undo3<instance>.bin` (cfg_global.bas:52), unit #150. It is a **ring buffer of full snapshots**:

- Header, 12 bytes at offset 1: `p1` (oldest record), `p2` (newest record), `plast` (top-most record offset, valid when wrapped) (L1277).
- Record (built L1258-1271): `MKL$(size)` + `idesx, idesy, idecx, idecy, ideselect, ideselectx1, ideselecty1, iden, idel, ideli` (10 LONGs) + `IdeBmkN` + per-bookmark `(y, x)` + `MKL$(compressedLen)` + `MKL$(LEN(idet$))` + `_DEFLATE$(idet$)` + trailing `MKL$(size)` (same value as the header, so the list can be walked backwards).
- Write (L1275-1363): first record at offset 13. Append after the newest record while `p2 < idebackupsize * 1000000` bytes (L1298; `idebackupsize` in MB from Options > Undo/History). When the limit is reached the write position wraps to 13 and old records are discarded by advancing `p1` until the new record fits (L1304-1310, L1326-1350). Header updated then record written with a single `PUT` (L1357-1361). The file is never truncated.
- Redo history is discarded implicitly: a new record is always written after the *file's* `p2`... **note**: the code uses `p2` from the header (newest), not `ideundopos`, so after undoing N steps and typing, the new state is appended after the old newest record rather than replacing the undone ones (L1295-1316). I did not find code that truncates `p2` back to `ideundopos`; behaviour after undo-then-edit was **not verified by running**.
- `ideundopos` = offset of the record representing the current state (L1365). `ideundobase` = offset of the first record belonging to the current file (L1366; reset to 0 on New/Open, L6579/L6671; -1 = "no restriction", set when the base record is overwritten by wrap, L1305).
- Coalescing typing (`ideundocombo`, `ideundocombochr`, `idemergeundo`): for consecutive alphanumeric characters (or repeats of the same char) and for consecutive backspaces, from the 2nd keystroke of a run `idemergeundo = 1` (L4387-4399, L4178-4184, L4120-4126). At write time `idemergeundo` moves `p2` back one record so the new snapshot **overwrites** the previous one (L1279-1290). `ideundocombo` is decremented on every handled event (L1648-1649), so any intervening event (cursor key, click) breaks the run.
- Undo (L3559-3640): find predecessor offset `u` using the trailing length (`GET #150, ideundopos - 4`) with wrap handling (L3566-3586). If `ideundopos = ideundobase` (about to undo into the *previous file's* content) ask "Undo through previous program content?", offer to save, then make the document untitled (L3590-3613). Load the snapshot (L3618-3631): all cursor/selection/view vars, bookmarks, `idet$ = _INFLATE$(...)`. Set `idechangemade = 1: idenoundo = 1` (recompile, no new record).
- Redo (L3646-3701): symmetric, forward via the leading length.
- Snapshots do not include breakpoints/skip-lines.
- Options > Undo/History can wipe the file (`_WRITEFILE UndoFile$, ""`, L16231-16232, outside ide2).

### 8.2 Autosave / crash recovery

- No separate autosave copy and no timer. The newest undo record *is* the backup, written on every change.
- `AutosaveFile$` = `<ConfigFolder>/autosave<instance>.bin` (cfg_global.bas:51) is an empty **flag file**: created on the first change of the session (`ideundoflag`, L1371-1374), deleted only on clean exit (L6524). If it exists at startup the previous session did not exit cleanly -> recovery prompt (L545-572, §1.7). Declining leaves the flag file in place (it is not deleted at L548-571), so it persists until a clean exit.
- Per-instance suffix `tempfolderindexstr$` keeps multiple IDE instances apart.
- A recovered buffer is untitled (`ideprogname` stays "") and `ideunsaved = 1`; the original file name is not stored in the record.

### 8.3 Unsaved-changes prompt

Shared pattern at Exit (L6502-6523), New (L6531-6552), Open (L6640-6661) and undo-through-base (L3600-3608): `IF ideunsaved THEN r$ = idesavenow` ("Program is not saved. Save it now?" Yes/No/Cancel, L12976-12984). "C" -> back to loop. "Y" -> if untitled, propose a name (`FindProposedTitle$` L20655, else `"untitled" + tempfolderindexstr$`) and show a save dialog (`SaveFile$` native or `idefiledialog$(name, 2)` text-mode per `UseGuiDialogs`), else `idesave idepath$ + idepathsep$ + ideprogname$`. Then `IF ideerror > 1 GOTO IDEerrorMessage`. Note `ideunsaved = -1` (non-zero) also triggers the prompt.

### 8.4 New / Open / Save

- **New** (L6528-6583): after the prompt: redraw, `ideunsaved = -1`, clear debug data, empty buffer, cursor/view reset, `idepath$ = _STARTDIR$`, `ideprogname$ = ""`, `ForceOptExpl = _FALSE`, trims `listOfCustomKeywords$` to `customKeywordsLength`, `QuickNavTotal = 0`, `ModifyCOMMAND$ = ""`, `idechangemade = 1`, `ideundobase = 0`, restore `IDEAutoIndent`/`IDEAutoLayout` defaults.
- **Open** (L6635-6689): `IdeOpenFile$` preset by recent-file items, drag-and-drop or empty; after the prompt, `OpenFile$(IdeOpenFile$)` (L21332) or `idefiledialog$("", 1)` performs the dialog **and the actual load** (not in my range). Returns "C" on cancel. Then L6670-6671: auto-include special mode (`ForceOptExpl = -2` when the file is `beforefirstline.bi`/`afterlastline.bm`), `ideFirstCompileFromDisk = -1: ideunsaved = -1: idechangemade = 1: idelayoutallow = 2: ideundobase = 0: QuickNavTotal = 0: ModifyCOMMAND$ = "": idefocusline = 0`.
- **Save** (L6691-6708) / **Save As** (L6711-6731): `idesave` (L12961-12974) writes each line + CRLF (Windows) or LF, saves bookmarks (`IdeSaveBookmarks`), `ideunsaved = 0`. Saving does not touch the undo file.
- **Exit** (L6499-6526): prompt, delete flag file, `SYSTEM`. The window close button and `ideexit` route here (L1411-1412).

### 8.5 Recent files / search history

- `AddToHistory "RECENT", path` (L634; SUB at L19900) and `AddToHistory "SEARCH", text` (L1974, L2137, L2311, L5936, L5950) maintain `RecentFile$` / `SearchedFile$` (one entry per line).
- File menu shows up to `UBOUND(IdeRecentLink, 1)` entries as `#N path` with `IdeRecentLink(n, 1)` = menu text and `(n, 2)` = full path (L19430-19453); "Recent..." appears when there are more. Handler L6585-6594 matches the menu text and opens with `AttemptToLoadRecent = _TRUE` (so a load failure offers cleanup, L136-145).
- `CleanUpRecentList` (L6805-6824) drops entries whose file no longer exists, using the buffer API (`FileToBuf%`, `ReadBufLine$`, `DeleteBufLine`, `BufToFile`).

### 8.6 Bookmarks

`IdeBmk()` array of `IdeBmkType` (`y`, `x`, `reserved`, `reserved2`), count `IdeBmkN`. Toggle at L3024-3047 (key) and L5453-5477 (menu; duplicated code): remove all bookmarks on the current line, or append one (array doubled when full). Next/previous L3049-3081 / L5479-5509: linear scan line by line with wrap-around; adds QuickNav history. Bookmark changes set `ideunsaved = 1` but not `idechangemade`. Line insert/delete adjusts them (`ideinsline`/`idedelline`); they are saved in undo records and persisted per file via `IdeSaveBookmarks`/`IdeImportBookmarks` (`BookmarksFile$`).

### 8.7 Quick-navigation history

`QuickNavHistory()` of `QuickNavType` (`idecy, idecx, idesy, idesx`), `QuickNavTotal`. `AddQuickNavHistory` (L20762-20775) pushes the current position (skipping duplicates of the top entry) before jumps: error-link jump (L1673, L3476), bookmark jumps (L3075, L5503), Go To SUB/Label (L5620, L5631), and in dialogs outside ide2. Pop = click on the arrow button (L1482-1490) or Ctrl+F2 (L3531-3540). Entries pointing beyond `iden` are discarded each iteration (L1457-1463). Cleared on New/Open. Enabled by `EnableQuickNav`.

### 8.8 Cursor/view variables

| Var | Meaning |
|---|---|
| `idecx`, `idecy` | cursor column/line (1-based; column may exceed line length) |
| `idesx`, `idesy` | first visible column/line; adjusted to follow the cursor in `ideshowtext` (L13125-13128) |
| `ideselect`, `ideselectx1`, `ideselecty1` | selection flag and anchor |
| `idemouseselect` | 0 none, 1 text drag, 2 vertical thumb drag, 3 horizontal thumb drag (reset when button released, L1430) |
| `idembmonitor` | keep iterating while a button is held |
| `idewx`, `idewy`, `idesubwindow` | window size in cells, editor-part height, help-pane height |
| `idefocusline` | error line highlight |
| `ideinsert` | 1 = overwrite mode |

---

## 9. Compiler-owned state and functions used by ide2

Ownership below is from the declarations I saw (marked "decl.") or inferred from names/usage (marked "inferred"); a systematic declaration sweep was not completed.

### 9.1 Protocol / control

| Name | Use in ide2 | Owner |
|---|---|---|
| `idecommand$`, `idereturn$` | protocol (L39, L101, L53, L187, L1387, L1624, L1884) | shared, inferred compiler-declared |
| `ideerror` | error trap (L106-167 etc.) | compiler, decl. qb64pe.bas:396 |
| `idecompiled` | EXE-up-to-date flag (L1393, L1819-1891, L5350, L6233) | compiler, decl. qb64pe.bas:397 |
| `compfailed` | C++ failure flag (L1073, L1251) | compiler, decl. qb64pe.bas:380 |
| `NoExeSaved` | "run only" (L1829, L6206) | compiler, decl. qb64pe.bas:40 |
| `IDEStartAtLine` | `-l:` switch (L624-628) | compiler, decl. qb64pe.bas:404 |
| `errorLineInInclude`, `warningInIncludeLine`, `warningInInclude` | include jump (L3157-3160) | compiler (first decl. qb64pe.bas:404; others inferred) |
| `layout$` | formatted line (L660-758) | compiler, inferred |
| `sp`, `sp2`, `sp$` | token separators (L678-679, L1115) | compiler, inferred |
| `prepass` | progress % (L57) | compiler, inferred |
| `linefragment` | "Caused by" text (L1112-1118) | compiler, inferred |
| `totalWarnings` | warning count (L897, L3177, L3547, L4681, L6899) | compiler, inferred |
| `Error_Happened` | reset before math evaluation (L5725) | compiler, inferred |
| `ForceOptExpl` | auto-include edit mode (L6571, L6670-6676) | compiler, inferred |
| `listOfCustomKeywords$`, `customKeywordsLength` | user SUB/FUNCTION names for highlighting, trimmed on New (L6572) | compiler, inferred |
| `ExtDepBuf`, `embedFileList$()`, `eflFile`, `eflUsed` | external dependency list (L1789-1815) | compiler, inferred |
| `lastBinaryGenerated$`, `path.exe$`, `compilelog$`, `extension$`, `os$`, `MacOSX`, `pathsep$`, `tempfolderindex`, `tempfolderindexstr$`, `Version$`, `IsCiVersion`, `WindowTitle`, `QB64_uptime#` | paths/platform/identity | compiler/global, inferred |
| `SaveExeWithSource`, `DefaultExeSaveFolder$`, `GenerateLicenseFile`, `IgnoreWarnings`, `IDEShowErrorsImmediately`, `IDEAutoIndent`, `IDEAutoIndentSize`, `IDEAutoLayout`, `DEFAutoIndent`, `DEFAutoLayout`, `AutoAddDebugCommand`, `WatchListToConsole`, `UseGuiDialogs`, ... | configuration read by both sides | config module (ide/config), inferred |
| `backupUsedVariableList()` (type `usedVarList`), `backupVariableWatchList$`, `backupTypeDefinitions$`, `variableWatchList$`, `watchpointList$`, `callstacklist$`, `callStackLength`, `vWatchReceivedData$()`, `host&`, `idebaseTcpPort` | debugger/watch data reset on New/Load (L522-526, L619-623, L6559-6563), TCP host (L776-788, L6480-6489) | mixed compiler/debugger, inferred |
| `IdeBreakpoints()`, `IdeSkipLines()`, `IdeDebugMode` | debug state | IDE (debugger section) |
| `SubFuncLIST()` | filled by `IdeMakeContextualMenu`, read L5621/L5632 | IDE, decl. ide_global.bas:169 |

Not referenced directly in L1-6975 (checked by reading, not by an exhaustive search): `usedVariableList`, `InvalidLine`, `SubNameLabels`, the `warning$()` arrays — these are consumed by `ideshowtext`, `idewarningbox`, `idevariablewatchbox$`, `IdeMakeContextualMenu` (`Labels()` at L19621) etc., outside this section. **Unverified.**

### 9.2 Compiler / shared functions called from ide2

| Call | Line(s) | Purpose |
|---|---|---|
| `GetRCStateVar(vWatchOn)` | L776, L3235, L6249, L6290, L6385, L6436 | is `$DEBUG` active in the last compiled source |
| `lineformat(retval$)` | L5724 | tokenise expression for the math evaluator |
| `Evaluate_Expression$(retval$, num)` with `DIM num AS ParseNum` | L5721-5727 | constant expression evaluator |
| `apply_layout_indent$` | L47 | utilities/format.bas:2 |
| `lineinput3load`, `lineinput3$`, `lineinput3buffer` | L588-615 | fast file reader, qb64pe.bas:28055 |
| `converttabs$` | L3752, L3756 | qb64pe.bas:28225 |
| `SCase$("$Debug")` | L6263, L6304, L6399, L6450 | keyword-case helper |
| `isalpha`, `alphanumeric` | L741, L2553, L3894, L3936, L4389 | character classes |
| `FileHasExtension`, `RemoveFileExtension$`, `RemoveDoubleSlashes$`, `QuotedFilename$`, `StrReplace$`, `BoolToTFString$` | various | string/file utilities |
| `WriteConfigSetting`, `ReadInitialConfig`, `CopyFromOther` | L230-232, L1037, L1217, toggles | config module |
| Buffer API: `SeekBuf&`, `ReadBufLine$`, `WriteBufLine`, `DeleteBufLine`, `EndOfBuf%`, `BufEolSeq$`, `FileToBuf%`, `BufToFile`, `GetBufPos&`, `DisposeBuf` | L1789-1815, L6806-6823 | simplebuffer library |
| `PurgeTemporaryBuildFiles (os$), (MacOSX)` | L5229 | build cache cleanup |
| `OfferNoprefixConversion%` | L853 | $NOPREFIX converter |
| `BinaryFormatCheck%` | L585 | old-QB binary source detection/conversion |
| `Wiki$`, `WikiParse`, `Back2BackName$`, `Help_ShowText` | help pane | wiki module |
| `DebugMode` | L818 | debugger |
| `getid` | not called in L1-6975 | – |

---

## 10. Observations relevant to a rewrite

- The IDE/compiler coupling is a hand-rolled coroutine: all loop-local state that must survive a yield has to be global/STATIC, and anything local is silently reset whenever the compiler is called back. A rewrite can replace this with an explicit incremental-compile job object polled by the UI loop.
- Run requests are encoded as return code 9 plus side-channel globals (`iderunmode`, `NoExeSaved`, `startPaused`, `idecompiled`, `ModifyCOMMAND$`).
- Menu dispatch by label string means renaming a menu item breaks its handler; several features exist twice (key path and menu path: bookmarks, comment blocks).
- Undo is whole-buffer snapshots compressed to a per-instance disk ring; it doubles as crash recovery. Layout rewrites bypass undo and dirty tracking by design.
- Hard-coded geometry: 608-column horizontal range (L3350, L3420), `idewy - 9` page size, status area 3 rows, 0.5 s double-click, visible-rows constant 16 at L752.

## 11. Not determined / not verified

- Where the SCREEN mode/palette is first set before `ide()` is called; `idezgetroot$`/`idezgetfilepath$` internals; where `ideerror` 4 and 5 are raised.
- Remainder of `GetInput` (mouse part, key repeat), `ideshowtext`, `idefind*`, `OpenFile$`/`idefiledialog$` load path — only headers/snippets read.
- Which physical key yields `K$ = CHR$(28)` (L1963).
- Behaviour of undo-then-edit with respect to redo records (§8.1) and compile state after an in-IDE runtime error (§1.6): read from code, not executed.
- Ownership (compiler vs IDE vs config) of variables marked "inferred" in §9; absence of `usedVariableList`/`InvalidLine`/`SubNameLabels`/`getid` references in L1-6975 was not confirmed by an exhaustive search.
- Where the compiler sends command 10 (include passback) and how `apply_layout_indent$` consumes `layout$`.
- Nothing was run; all statements are from reading source.

---

# Part C — Text buffer, file I/O, UI toolkit, input, editor rendering, search

*(Section numbers below are local to Part C.)*

Sources (all paths relative to `..\QB64pe\source\`):

| Abbrev. | File |
|---|---|
| `M:` | `ide\ide_methods.bas` (21,424 lines) |
| `G:` | `ide\ide_global.bas` (242 lines) |
| `SH:` | `subs_functions\syntax_highlighter_list.bas` (164 lines) |
| `Q:` | `qb64pe.bas` |
| `CFG:` | `ide\config\cfg_methods.bas`, `CFGG:` = `ide\config\cfg_global.bas` |
| `CONV:` | `ide\ide_converters.bas` |

Every line reference below was looked at in the working copy unless it is tagged "(not verified)". Items I could not determine are collected in section 8. Observations tagged **[quirk]** are behaviours that look like bugs or accidents; a rewrite should decide deliberately whether to reproduce them.

---

## 1. Text buffer

### 1.1 Storage format

The whole program lives in one string, `idet` (`DIM SHARED idet AS STRING, idel, ideli, iden`, G:143).

| Variable | Meaning |
|---|---|
| `idet$` | Concatenation of line records. |
| `iden` | Number of lines (always >= 1). |
| `idel` | 1-based number of the "cached" current line. |
| `ideli` | 1-based byte offset inside `idet$` of the first byte of line `idel`'s record. |

One line record is `MKL$(len) + text + MKL$(len)`: a 4-byte little-endian LONG length, the raw bytes of the line, and the same 4-byte length again (M:12287, M:12992, M:12884). The record size is `len + 8`. The trailing copy of the length is what lets the cursor walk backwards. There are no line terminators inside the buffer.

An empty document is `MKL$(0) + MKL$(0)` with `idel = 1: ideli = 1: iden = 1` (M:519, M:6564, M:12853, M:21370).

### 1.2 Primitives

| Routine | Line | Algorithm | Cost |
|---|---|---|---|
| `idegotoline (i)` | M:12128 | No-op if `idel = i`. Clamps `i < 1` to 1 (writes back into the caller's variable, parameters are by reference). Backward: repeat `idel = idel - 1: ideli = ideli - CVL(MID$(idet$, ideli - 4, 4)) - 8` (M:12133-12136). Forward: repeat `idel = idel + 1: ideli = ideli + CVL(MID$(idet$, ideli, 4)) + 8` (M:12140-12144). | O(distance in lines from `idel`) |
| `idegetline$ (i)` | M:12117 | `i = -1` means "the cached line, do not seek"; otherwise `idegotoline i`. Returns `MID$(idet$, ideli + 4, CVL(MID$(idet$, ideli, 4)))`. | seek + O(line length) |
| `idesetline (i, text$)` | M:12986 | `text$ = RTRIM$(text$)` (also modifies the caller's variable), seek unless `i = -1`, then rebuilds the whole string: `LEFT$(idet$, ideli - 1) + new record + RIGHT$(idet$, rest after old record)` (M:12992). | seek + O(size of whole buffer) |
| `ideinsline (i, text$)` | M:12252 | Shifts bookmarks/breakpoints (1.3), `RTRIM$`, `i = -1` means `idel`. If `i > iden` it delegates to `idesetline` (M:12280-12283). Otherwise seeks to `i` and splices a new record in front of it (M:12287), `iden = iden + 1`. The cache keeps pointing at line `i`, which is now the new line ("cursor remains on line i", M:12253). | seek + O(buffer) |
| `idedelline (i)` | M:11401 | Shifts bookmarks/breakpoints (1.3), seeks to `i`, cuts `textlen + 8` bytes (M:11428-11429), `iden = iden - 1`; if the last line was removed, `idegotoline iden` (M:11432). | seek + O(buffer) |
| `idecentercurrentline` | M:12122 | If the file is longer than the view (`iden > idewy - 8`), `idesy = idecy - (idewy - 8) \ 2`, min 1. | O(1) |

Important side effect: **seeking forward past the end grows the buffer.** Inside the forward loop, `IF idel = iden THEN idet$ = idet$ + MKL$(0) + MKL$(0): iden = iden + 1` (M:12141). So `idegetline(iden + 5)` or `idesetline iden + 1, x$` silently appends blank lines. `idenewsf` relies on this to append four lines (M:12464-12470); `ideshowtext` avoids it by testing `l <= iden` before reading (M:13196).

Complexity consequences for a rewrite:

- Sequential access is cheap (the cache makes `FOR y = 1 TO iden: idegetline(y)` linear), random access is O(distance).
- Every modification copies the entire buffer (`LEFT$ + ... + RIGHT$`), so a loop that edits k lines costs O(k x buffer size). "Change All" (M:11181) and block operations do exactly that.
- None of the primitives guard `iden = 1` on delete; callers do (e.g. `delselect`, M:21078: `IF iden = 1 AND y = 1 THEN idesetline y, "" ELSE idedelline y`).

### 1.3 Bookmarks, breakpoints, skip-lines on insert/delete

Types: `IdeBmkType { y, x, reserved, reserved2 }` (G:30-35), `IdeBmk()`/`IdeBmkN` (G:36-37); `IdeBreakpoints()` and `IdeSkipLines()` are `_BYTE` arrays indexed by line number (G:46-47).

Insert (`ideinsline`):

- Every bookmark with `y >= i` gets `y + 1` (M:12255-12260).
- Only when `GetRCStateVar(vWatchOn)` (i.e. `$DEBUG` is active): each array is `REDIM _PRESERVE`d to `iden + 1`, then the elements are bubbled down with `SWAP a(b), a(b - 1)` for `b = iden + 1 TO i STEP -1`, and element `i` is zeroed (M:12262-12274).
- **[quirk]** The shifting happens before `i = -1` is resolved to `idel` (M:12278), so a call with `i = -1` would shift every bookmark and index the arrays at -1/-2. (Whether any caller passes -1 was not checked.)

Delete (`idedelline`):

- Every bookmark with `y >= i` gets `y - 1`, minimum 1 (M:11403-11408). A bookmark on the deleted line therefore moves to the previous line.
- Only when vWatch is on: arrays are grown to `iden` if needed, breakpoints are bubbled up with `SWAP IdeBreakpoints(b), IdeBreakpoints(b + 1)` for `b = i TO iden - 1`, then truncated to `iden - 1` (M:11416-11419).
- **[quirk]** The skip-lines loop uses `SWAP IdeSkipLines(b), IdeSkipLines(b - 1)` (M:11422), i.e. `b - 1` where the breakpoint loop uses `b + 1`. That does not perform the same shift (it rotates flags the wrong way and touches index `i - 1`).

Outside debug mode the arrays are not shifted at all; `ideshowtext` just grows them in steps of 100 when it needs an index (M:13636-13642). Loading a file resets both to `REDIM (iden)` (M:12889-12890, M:21406-21407).

### 1.4 Limits

- No explicit maximum line length or line count in the primitives. The length field is a signed 32-bit LONG and the whole buffer is one QB64 string.
- The editor's horizontal scrollbar is hard-wired to 608 columns (`idehbar(2, idewy - 5, idewx - 2, idesx, 608)`, M:13615; also M:3350, 3420, 3425, 7380, 7421). That is a scrollbar range, not a stored-length limit.
- The status bar stops showing the column when `idecx >= 100000` (M:13622).
- The loader pre-allocates `LEN(file) * 8` bytes (M:12861), which bounds loadable file size to roughly an eighth of the maximum string size. (Exact practical limit not verified.)

### 1.5 Legal characters

- `block_chr(255)` (G:66-68) marks only CHR$(10) and CHR$(13). It is consulted for typed characters at M:4254 (`IF block_chr(ASC(K$)) THEN GOTO specialchar`); TAB bypasses it (M:4253). Any other byte 0-255 can be stored.
- Tabs never stay in the buffer: on load they are expanded to 4-column tab stops (hard-coded 4, M:12873-12882); `insertAtCursor` runs the line through `converttabs$` (M:21093), which uses `IDEAutoIndentSize` if `IDEAutoIndent` is on, else 4 (Q:28225-28233).
- CR and LF only exist as separators on disk/clipboard; `lineinput3$` consumes them (2.1).
- A single trailing CHR$(26) (DOS EOF) is stripped on load (Q:28060).
- Trailing spaces are removed by `RTRIM$` in `idesetline`/`ideinsline`, but the loaders write records straight into `idet$` (M:12884), so trailing spaces from disk survive until the line is next rewritten.
- The buffer is 8-bit. Display is CP437 by default; an optional code page (`idecpindex`, 27 tables, G:71-74) only remaps glyphs 128-255 via `_MAPUNICODE` at start-up (M:211-215). There is no transcoding of buffer contents.
- `GetInput` supports Alt+numpad entry of any code 1-255 (4.1).

### 1.6 State flags

The buffer primitives set **no** flags. `idesave` sets `ideunsaved = 0` (M:12973). Everything else is done by callers and by the main loop (owned by the ide2 section; summarised here only as far as I looked):

| Flag | Where | Meaning seen |
|---|---|---|
| `idechangemade` (G:166) | set to 1 by editing code, e.g. `insertAtCursor` M:21100, `idenewsf` M:12472, Change All M:11199, per-match change M:6055 | "buffer changed since the main loop last looked". Consumed at M:1239-1252: resets compile state, `idefocusline = 0`, then `IF ideunsaved = -1 THEN ideunsaved = 0 ELSE ideunsaved = 1` (M:1252). |
| `ideunsaved` (G:151) | 0 = saved, 1 = modified, -1 = "a change is pending but keep the saved state" (set together with `idechangemade = 1` after a load: M:527, M:6671; also M:5302 etc.) | `*` in the title (M:6968). |
| `idelayoutallow` (G:133) | set to 2 on load (M:6671, M:856) and M:3736/5436; decremented when a change is processed (M:1241); tested at M:747 | gates auto-layout of the line being left (ide2 section). |
| `idenoundo` (G:237) | set to 1 at M:3633/3696 together with `idechangemade`; tested at M:1254 to skip the undo snapshot; cleared at M:1377 | "do not record this change in undo". |
| `idemergeundo` | set at M:4124/4182/4395 when `ideundocombo = 2`; consumed at M:1279 | merge with previous undo step. |
| `startPausedPending` | zeroed by every edit path listed above | debugger section. |

---

## 2. File load / save

### 2.1 Reading

There are **three copies of the same loader**: the command-line load at start-up (M:574-620), `idefiledialog$` mode 1 (M:12832-12902) and `OpenFile$` (M:21368-21418). All do:

1. `BinaryFormatCheck%` (2.3). A result > 0 aborts the load.
2. Reset buffer, view, cursor, selection, `IdeBmkN`, `idefocusline` (M:12853-12859).
3. `lineinput3load path` (Q:28055): opens the file `FOR BINARY AS #1`, reads the whole file into `lineinput3buffer$`, strips one trailing CHR$(26).
4. `idet$ = SPACE$(LEN(lineinput3buffer) * 8)`, then loop `a$ = lineinput3$` until it returns the sentinel CHR$(13) (M:12867-12886). Each line has tabs expanded and is written in place with `MID$(idet$, i2, l + 8) = MKL$(l) + a$ + MKL$(l)` (M:12884).
5. `iden = n`; an empty file becomes one empty line; otherwise `idet$` is trimmed to the used length (M:12888).
6. Reset breakpoints/skip lines/watch lists (M:12889-12894), `ideprogname = f$`, `_TITLE ideprogname + " - " + WindowTitle`, cut `listOfCustomKeywords$` back to the user-configured part (M:12898), `idepath$ = path$`, `AddToHistory "RECENT"`, `IdeImportBookmarks` (M:12897-12901).

Note the loaders do not set `idel/ideli` again after step 2, which is valid because both are 1.

`lineinput3$` (Q:28065-28092) line-ending rules: the next line ends at the first CR or LF; if the character right after it is the *other* one of the pair it is consumed too. So LF, CR, CRLF and LFCR are all accepted, mixed freely. A final line without terminator is returned; a terminator at end of file does not produce an extra empty line. The original line-ending style is not remembered.

No BOM or UTF-8 handling exists anywhere in these paths (searched the loaders and `lineinput3*`); a UTF-8 BOM would simply become three characters at the start of line 1. No read-only detection exists either.

The variables `chr7$, chr11$, chr12$, chr28$...chr31$` assigned at M:12866 are unused in the loader (vestigial).

### 2.2 Writing

`idesave (f$)` (M:12961-12974):

```
ideerror = 6
OPEN f$ FOR OUTPUT AS #151: CLOSE #151      ' truncate
OPEN f$ FOR BINARY AS #151
ideerror = 1
IF INSTR(_OS$, "WIN") THEN LineEnding$ = CHR$(13) + CHR$(10) ELSE LineEnding$ = CHR$(10)
FOR i = 1 TO iden: outfile$ = idegetline(i) + LineEnding$: PUT #151, , outfile$: NEXT
CLOSE #151
IdeSaveBookmarks f$
ideunsaved = 0
```

- CRLF on Windows, LF elsewhere, regardless of what was loaded. Every line, including the last, gets a terminator.
- Bytes are written as stored (no tab re-insertion, no encoding conversion, no BOM, no trailing-space trimming beyond what editing already did).
- Failure reporting relies on the `ideerror` code that is current when a runtime error fires; the table is at M:111-117 (1 internal, 2 "File not found", 3 "File access error", 4 "Path not found", 5 "Cannot create folder", 6 "Cannot save file", 7 "Cannot export file"). A read-only target therefore surfaces as "Cannot save file". (The error-trap mechanism itself is in the ide2 section; not traced here.)

### 2.3 Binary / QB4.5 detection

`BinaryFormatCheck% (pathToCheck$, pathSepToCheck$, fileToCheck$)` (CONV:2-108):

- Reads the whole file; if it contains no CHR$(0) it is text, return 0 (CONV:10).
- Otherwise reads two INTEGERs (format, version) from offset 1 and switches on format: 2300 = VBDOS, "not supported", return 1; 764 = QBX 7.1, "not supported", return 1; 252 = QuickBASIC 4.5.
- For QB4.5: asks "QuickBASIC 4.5 binary format detected. Convert to plain text?". Converter is `internal\utilities\QB45BIN.exe` (Windows) or `./internal/utilities/QB45BIN`. If missing, it is compiled on the spot from `internal/support/converter/QB45BIN.bas` with `qb64pe -x ... -o internal/utilities/QB45BIN` (CONV:86-90). Conversion runs `QB45BIN "<in>" -o "<out>"` via `SHELL _HIDE` with the IDE dimmed (`DarkenFGBG(1)`); output name is `<name> (converted)<.ext>` or `<name> (converted).bas`.
- On success the function **rewrites its by-reference path and file arguments** to point at the converted file and returns 0, so the caller loads the converted text. Returns 2 if the output file did not appear, 1 if the user declined.
- A file containing NUL with any other header falls through the `SELECT` and returns 0, i.e. it is loaded as text.

### 2.4 Dialog front ends

`UseGuiDialogs` selects native or text-mode dialogs (M:6662-6666 for open; M:6510-6513, 6539-6542, 6648-6651, 6697-6700, 6716-6725 for save).

**`OpenFile$ (IdeOpenFile)`** (M:21332): `_OPENFILEDIALOG$("Open Source File", Default_StartDir$, "*.bas|*.BAS|*.Bas|*.bi|*.BI|*.Bi|*.bm|*.BM|*.Bm", "QB64(PE) Source Files", 0)` unless a file name was passed in. Returns "C" on cancel. Appends ".bas" if the file does not exist as given (M:21348-21355; `ideerror = 2` and silent `EXIT FUNCTION` if still missing). `Default_StartDir$` is a SHARED variable seeded from `_STARTDIR$` and updated to the last used directory. **[quirk]** `AllFiles` at M:21350 is an undeclared local (always 0), and the retry branch at M:21363-21364 is unreachable because `IdeOpenFile` was just assigned at M:21357.

**`SaveFile$ (IdeOpenFile)`** (M:21297): `_SAVEFILEDIALOG$("Save Source File", Default_StartDir$ + name, same filter, ...)`; "C" on cancel; appends ".bas" when there is no extension; resolves the directory with `idezgetfilepath$(ideroot$, f$)`; opens the target `FOR BINARY AS #150` as an access probe (`ideerror = 3`), sets `ideprogname$`, title, calls `idesave`, sets `idepath$`, adds to RECENT, saves bookmarks. No own overwrite prompt (left to the native dialog).

**`idefiledialog$ (programname$, mode)`** (M:12515-12941), the text-mode dialog. Modes: 1 "Open", 2 "Save As", 3 "Choose a custom font", 4 "Save logging to" (M:12545-12553). Size 70 x (`idewy + idesubwindow - 7`).

| # | Control | Notes |
|---|---|---|
| 1 | text box "File #Name" | pre-filled and selected for modes > 1 |
| 2 | list "#Files" (w 32) | from `idezfilelist$` |
| 3 | list "#Paths" (w 31) | from `idezpathlist$` |
| 4 | check box ".#BAS Only" | backed by `STATIC AllFiles`, so remembered between invocations |
| 5 | button "Ne#w Folder" | `idenewfolder$` (input box + `MKDIR`, `ideerror = 5`) |
| 6/7 | buttons "#OK", "#Cancel" | |

Behaviour: a "Path: " line is drawn at row 4, left-truncated with three CHR$(250) dots (M:12624-12629). Moving in the file list copies the item to the name box (M:12737-12741). Enter or double-click in the path list changes directory (`..` handled through `changepath`). Enter with an existing directory name changes into it (M:12783). Names with `?` or `*` re-filter the file list (`idezfilelist$(path$, 2, f$)`, M:12805-12826). Enter on an empty name box resets the filter. In mode 1, a file dropped on the window fills the name box (`_TOTALDROPPEDFILES`, M:12649-12657). `IdeOpenFile` non-empty makes mode 1 skip the UI entirely (`GOTO DirectLoad`, M:12601).

Mode 1 then runs the loader of 2.1. Mode 2 appends ".bas" when no extension, probes with `OPEN ... FOR BINARY AS #150`; if `LOF > 0` asks `idefileexists$` ("File "..." already exists. Overwrite?", Yes/No, M:11724-11737), then `idesave` etc. (M:12903-12924). Modes 3/4 return the full path.

Return value: "C" for cancel; modes 1 and 2 return "" on success (the function value is never assigned), modes 3/4 return the path. Errors leave `ideerror > 1` for the caller (M:6668).

Directory helpers (all shell out and parse a temp file):

| Routine | Line | Behaviour |
|---|---|---|
| `idezfilelist$ (path$, method, mask$)` | M:15562 | method 0 = `*.bas`, 1 = all, 2 = mask. Windows: `dir /b /ON /A-D ... > .\internal\temp\files.txt`; Linux/macOS: `find ... -maxdepth 1 -type f -name ... \| sort` (run twice for `*.bas` and `*.BAS`). Returns names joined by CHR$(0). Special case for `path$ = "internal/help"`. |
| `idezpathlist$ (path$)` | M:15652 | Sub-directories; `..` prepended when not at a root; on Windows appends drive letters from `logical_drives&` (M:15678-15688). |
| `idezchangepath$ (path$, newpath$)` | M:15517 | String-level `..`, drive change, or append. |
| `idezgetroot$` | M:15631 | Current directory via `cd` / `pwd` redirected to `internal\temp\root.txt`. |
| `ideztakepath$ (f$)` | M:15725 | Splits at the last `\` or `/`; returns the path and **rewrites `f$`** to the bare name. |
| `idezgetfilepath$ (root$, f$)` | M:15744 | Splits, makes relative paths relative to `root$`, validates with `_DIREXISTS` (`ideerror = 4`), `CHDIR`s there to obtain the canonical `_CWD$`, then `CHDIR ideroot$`. |

Defaults: `ideroot$ = idezgetroot$`, `idepath$ = _STARTDIR$` (M:515-516); File/New resets `idepath$ = _STARTDIR$` (M:6570). `ideprogname = ""` means "Untitled" (M:6965). `idepathsep$` is `\` or `/` (M:512).

Small prompts: `iderestore$` "Backup found / Recover program from auto-saved backup?" returns "Y"/"N" (M:12952); `idesavenow$` "Program is not saved. Save it now?" with Yes/No/Cancel returns "Y"/"N"/"C" (Esc = "C") (M:12976).

---

## 3. Dialog / control toolkit

### 3.1 Types

`idedbptype` - the dialog window (G:182-188):

| Field | Meaning |
|---|---|
| `x, y` | Screen column/row of the top-left border character. |
| `w, h` | Interior width/height; the frame is drawn `w + 2` by `h + 2` (M:11717). |
| `nam` | Index into the string pool for the title (0 = none). |

`idedbotype` - a control (G:190-213):

| Field | Meaning |
|---|---|
| `par` | A **copy** of the parent `idedbptype` (set by `FOR i = 1 TO 100: o(i).par = p: NEXT`). |
| `x, y` | Position relative to `par.x`/`par.y`. `x = 0` defaults to 2 at draw time. |
| `w, h` | Size; 0 = auto (text box: to the right edge; list: fill; buttons: `par.w - x`). |
| `typ` | Control type, 0 = unused slot. |
| `nam` | Pool index of the label/caption (`#` marks the hotkey letter). |
| `txt` | Pool index of the content: text-box value, CHR$(0)-separated list items, CHR$(0)-separated button captions, the symbol of a type-5 button. |
| `inv` | "Invalid" flag: text box / check box drawn in colour 4 (red) on 7 (M:11448, M:11684). |
| `blk` | Text box: columns reserved at the right (for adjacent symbol buttons). |
| `rpt` | Type 5: auto-repeat rate (per second) while held. |
| `dft` | Button row: 1-based index of the default button. |
| `cx, cy` | Output of `idedrawobj`: where the hardware cursor should be if this control has focus. |
| `foc` | Focus offset (`focus - f`); 0 = this control (or first button) has focus. |
| `sel` | List: selected item (negative = remembered but list unfocused); check box: 0/1. |
| `selY` | List: screen row of the selected item (output of draw). |
| `stx` | List: pool index that receives the selected item's text. |
| `issel` | Text box: selection active (-1). |
| `sx1` | Text box: selection anchor (0-based character offset). |
| `v1` | Text box: cursor offset (0..LEN); list: first visible item. |
| `num` | List: item count (output of draw). |

Control types (`typ`): 1 single-line text box, 2 list box with vertical scrollbar, 3 row of action buttons, 4 check box, 5 single-symbol button `(x)` (M:11444-11712).

String pool: `idetxt(1000) AS STRING`, `idetxtlast` (G:153-154). `idenewtxt(a$)` appends and returns the index (M:12493); `idedeltxt` resets `idetxtlast = 0` (M:11436) and is called by callers after a dialog returns (e.g. M:6040). There is no bounds check and no per-dialog scoping.

### 3.2 Canonical dialog pattern

Every dialog is a copy of this skeleton (e.g. `idefind$` M:11739-11984):

1. Header: `PCOPY 0, 2: PCOPY 0, 1: SCREEN , , 1, 0` (work on page 1, show page 0; page 2 keeps a backup, page 3 is the main IDE page), `focus = 1`, `DIM p AS idedbptype`, `DIM o(1 TO 100) AS idedbotype`, `sep = CHR$(0)`.
2. Init: `idepar p, w, h, title$` centres the box on the screen and calls `_RESIZE OFF` (M:12943-12950). Then `i = i + 1: o(i).typ = ...: o(i).y = ...: o(i).nam = idenewtxt("#Label"): o(i).txt = idenewtxt(...)`.
3. `FOR i = 1 TO 100: o(i).par = p: NEXT`.
4. Loop:
   - Draw: `idedrawpar p`; `f = 1: cx = 0: cy = 0`; for each used slot `o(i).foc = focus - f: o(i).cx = 0: o(i).cy = 0: idedrawobj o(i), f`; keep the last non-zero `cx/cy`; `lastfocus = f - 1`.
   - Custom drawing, `PCOPY 1, 0`, and if `cx` then `LOCATE cy, cx, 1` on page 0.
   - Input: loop `GetInput` with `_LIMIT 100` until wheel, key, click, release, held button, or an Alt state change (M:11871-11882). `idehl = 1` while Alt (without Ctrl) is down; `altletter$` = the A-Z key pressed with Alt (M:11883-11891).
   - Generic response: `info = 0`; `K$ = ""` becomes CHR$(255); Tab = `focus + 1`, Shift+Tab (or CHR$(25) on macOS) = `focus - 1`, wrapping in `1..lastfocus`; then `f = 1` and for each slot `focusoffset = focus - f: ideobjupdate o(i), focus, f, focusoffset, K$, altletter$, mB, mousedown, mouseup, mX, mY, info, mWHEEL` (M:11896-11909).
   - Dialog-specific logic keyed on `focus`, `info` and `K$` (Esc = cancel, Enter = default action), then `mousedown = 0: mouseup = 0`.

Focus model: `focus` is a 1-based index over focus *slots*. Types 1, 2, 4, 5 take one slot; a button row takes one slot per button (`f = f + n`, M:11677, M:15367). Dialog code therefore tests things like `focus = 9 AND info <> 0` for "OK was activated" (M:11943).

`info` is a single shared out-parameter: a button row sets it to the 1-based button index, a list sets 1 on double-click, a symbol button sets 1. Dialogs disambiguate by `focus`.

Drawing helpers: `idebox` single-line frame with CHR$(218/196/191/179/192/217) (M:10849); `ideboxshadow` adds the shadow by re-printing the characters underneath in `COLOR 2, 0` (M:10857-10881); `idedrawpar` frame + centred title in `COLOR 0, 7` (M:11716); `idehPRINT` prints a label, drops `#` and prints the following letter in colour 15 when `idehl` is set (M:12239); `idehlen` = length minus one if the label contains `#` (M:12235).

### 3.3 `idedrawobj (o, f)` per type

- **Text box** (M:11445-11490): `label:` then a 3-row frame of width `o.w + 4`. `o.v1` is clamped to the text length. When focused and the text is longer than `o.w - o.blk`, the view scrolls so the cursor is visible (`tx = o.v1 - (o.w - o.blk) + 1`); unfocused shows the left part. Selection (`issel` and focused) is painted `COLOR 7, 0` for offsets in `[min(sx1, v1), max(sx1, v1))`.
- **List** (M:11493-11626): frame `o.w + 2` by `o.h + 2` with the caption centred on the top border. Normalises `sel = 0` to -1 and `v1 = 0` to 1, scrolls `v1` to keep `ABS(sel)` visible, and negates a positive `sel` when unfocused (M:11507-11514), so an unfocused list shows no highlight bar. Items are RTRIMmed and prefixed with a space; the selected one is `COLOR 7, 0`. Over-long items end in CHR$(26). Inline colour codes: CHR$(16)+CHR$(0..15) sets foreground, CHR$(16)+CHR$(16) restores, CHR$(17)+CHR$(n) background, CHR$(17)+CHR$(17) restores (M:11543-11591). Special cases for the SUBs list: leading tree characters CHR$(195)/CHR$(192) stay unhighlighted and shift the cursor by 2; a `*` after CHR$(196) is printed in colour 2. Writes `o.num`, `o.selY`, draws `idevbar` on the right border.
- **Buttons** (M:11629-11678): captions separated by CHR$(0); default text "#OK". `spacing = (o.w - (visible chars + 4 * n)) \ (n + 1)`; each button is `< caption >`. The brackets of the focused button are colour 15; if focus is not in this row the `dft` button is shown that way instead.
- **Check box** (M:11681-11697): `[X] ` or `[ ] ` then the label.
- **Symbol button** (M:11700-11712): `(c)` at a fixed position, colour 15 when focused.

### 3.4 `ideobjupdate` (M:14929-15442)

`SUB ideobjupdate (o AS idedbotype, focus, f, focusoffset, kk$, altletter$, mb, mousedown, mouseup, mx, my, info, mw)`

| Parameter | Direction | Meaning |
|---|---|---|
| `o` | in/out | the control |
| `focus` | in/out | dialog focus slot; set when the control takes focus (click or hotkey) |
| `f` | in/out | running slot counter, advanced by the control |
| `focusoffset` | in | `focus - f` at call time; 0 = focused |
| `kk$` | in/out | key as INKEY$-style string; CHR$(255) = none. List scrollbar clicks overwrite it with a synthetic key; type 5 clears it. |
| `altletter$` | in | upper-case letter pressed with Alt, or "" |
| `mb` | in | left button currently down |
| `mousedown`, `mouseup` | in | click / release edge flags (`mouseup` unused, M:14934) |
| `mx, my` | in | mouse cell |
| `info` | out | activation code (see 3.2) |
| `mw` | in | wheel delta |

It also reads the globals `KB`, `KSHIFT`, `KCTRL`, `KCONTROL`, `KALT` directly.

**Text box** (M:14935-15105)

- Mouse-down in the frame focuses it; on the text row the cursor moves to the clicked offset and the selection is cleared. **[quirk]** If the click lands exactly on the current cursor offset (and that is not the end of the text) the field is cleared, labelled "dbl-click text=clear field text" (M:14950-14953).
- Keys when focused:

| Key | Action |
|---|---|
| Shift+Ins, Ctrl/Cmd+V | Paste `StripDiscordANSI$(_CLIPBOARD$)`, cut at the first CR or LF; replaces the selection; cursor goes to the end of the pasted text only if `PasteCursorAtEnd` (M:14968-14990) |
| Ctrl/Cmd+A | Select all (M:14992) |
| Ctrl+Ins, Ctrl/Cmd+C | Copy selection (M:15001) |
| Shift+Del, Ctrl/Cmd+X | Cut selection (M:15010) |
| Backspace | Delete selection, else the character before the cursor (M:15025-15047) |
| Del (`CHR$(0)+"S"`) | Delete selection, else the character at the cursor (M:15066-15083) |
| Left/Right/Home/End | Move; with Shift start/extend selection from `sx1`, without Shift drop it (`selectcheck`, M:15438-15441); selection ends when `v1 = sx1` (M:15092) |
| Printable | Any single-byte key except 8, 9, 0, 10, 13, 26, 255, and only when Alt and Ctrl are both up or both down (AltGr) (M:15048); replaces the selection; always insert mode |

- There is no maximum length, numeric filter or validation in the control; dialogs post-process `idetxt(o.txt)` themselves.
- Alt+hotkey of the label moves focus here (M:15096-15103).

**List box** (M:15107-15304)

- Each call re-splits `idetxt(o.txt)` into `ListBoxITEMS()` (trimmed, leading non-printable characters removed, used for matching) and `OriginalListBoxITEMS()` (verbatim) (M:15112-15153), and clears `idetxt(o.stx)`.
- Mouse-down inside: focus; clicking a row selects it; the same row again within 0.3 s sets `info = 1` (double-click, M:15163); clicking below the last item gives `sel = -num`.
- Button held on the scrollbar column (focused only): above/below the thumb = PgUp/PgDn, top/bottom arrow cells = Up/Down, each with `_DELAY 0.1` (M:15171-15203).
- Wheel: selection jumps to the top (or bottom) visible row and then moves 3 items per notch (M:15206-15216).
- Keys: Up/Down (a negative `sel` first just becomes positive), PgUp/PgDn by `h - 1`, Ctrl+Home (`CHR$(0)+"w"`) first, Ctrl+End (`CHR$(0)+"u"`) last (M:15217-15252). Plain Home/End are not handled.
- Type-to-search (M:15254-15289): printable keys append to the global `fileDlgSearchTerm$` (G:28), which resets after 1.0 s of inactivity. Two identical letters in a row are treated as "cycle through items starting with that letter". The search is a case-insensitive prefix match from the current item (or the next one after a reset) to the end, then once more from the top.
- Finally `idetxt(o.stx)` = the verbatim text of the selected item when `sel > 0` (M:15291). Alt+hotkey focuses.

**Button row** (M:15306-15368): Alt+hotkey of a caption sets `focus` to that button and `info` to its index (activates immediately). Mouse-down on a button's cells does the same. Enter or Space while one of the buttons has focus sets `info = focusoffset + 1`.

**Check box** (M:15370-15404): click on box or label focuses and toggles; Space toggles; Up sets 1, Down sets 0; Alt+hotkey only focuses.

**Symbol button** (M:15406-15435): click (or held button) on its three cells sets `info = 1`; Enter/Space when focused sets `info` and clears `kk$`. With `rpt > 0`: first activation fires at once, then nothing for 0.625 s, then at most `rpt` times per second.

### 3.5 Scrollbars

`idevbar (x, y, h, i2, n2)` (M:15444) and `idehbar (x, y, h, i2, n2)` (M:12147) only **draw** a bar of length `h` (arrows CHR$(24)/CHR$(25) or CHR$(27)/CHR$(26), track CHR$(176), thumb CHR$(219)) for item `i` of `n`, and **return the screen coordinate of the thumb** (row for vertical, column for horizontal). Callers compare the mouse coordinate with the return value to decide "page before/after thumb" or "drag".

| Bar length | Thumb |
|---|---|
| 2 | none; returns `y` (or `x`) |
| 3 | none; returns start + 1 |
| 4 | none for `n = 1`; else nearer of the two cells |
| > 4 | `n = 1`: none, returns start + `h \ 4`; `i = 1`: first cell; `i = n`: last cell; else `start + 2 + INT((i - 1) / (n - 1) * (h - 4))` |

### 3.6 Stock dialogs

**`idemessagebox (titlestr$, messagestr$, buttons$)`** (M:16969-17120)

- Literal `\n` in the message becomes a line break (M:16983). At most 9 lines (`DIM FullMessage$(1 TO 9)`, M:16985); extra text stays in line 9.
- There is **no word wrap**. Width = max(longest line + 2, title + 4, `LEN(buttons$) + 6 * buttons`), capped at `idewx - 4`; a line that is still too long is cut and ends in three CHR$(250) (M:17051-17053). Lines are centred.
- `buttons$` is `;`-separated with `#` hotkeys, e.g. `"#Yes;#No;#Cancel"`; empty means `"#OK"`. One button row, default = first.
- Any plain letter key acts as a hotkey, no Alt needed (M:17090).
- Returns the 1-based index of the activated button (it returns `focus`, M:17111), or 0 for Esc. Enter activates the focused button. Calls `ClearMouse` on exit.
- The by-reference `messagestr$`/`buttons$` arguments are modified.

**`ideyesnobox$ (title$, message$)`** (M:17122): "Y" if button 1, otherwise "N" (Esc = "N").

**`ideinputbox$ (title$, caption$, initialvalue$, validinput$, boxwidth, maxlength, ok)`** (M:12291-12445): one text box (pre-selected) plus OK/Cancel in a box of height 5. After each event the text is filtered to characters contained in `validinput$` (if non-empty) and truncated to `maxlength` (if non-zero) (M:12412-12425). Enter/OK returns the text with `ok = -1`; Esc/Cancel returns "" with `ok = 0`.

Other small dialogs in range: `idechangeit$` (3-button strip near the bottom of the screen, 6.3), `idefileexists$`, `idesavenow$`, `iderestore$`, `idechanged`, `idenomatch`.

---

## 4. Input layer

### 4.1 `GetInput` (M:18840-18922)

Globals (G:49-64): `iCHECKLATER`, `iCHANGED`, `mX, mY`, `mB, mB2` (left/right button state), `mOB, mOB2` (previous state), `mCLICK, mCLICK2`, `mRELEASE, mRELEASE2`, `mWHEEL`, `KB` (`_KEYHIT` value or 0), `K$` (INKEY$-style string), `KSTATECHANGED`, `KSHIFT`, `KCTRL`, `KCONTROL`, `KALT, KOALT, KALTPRESS, KALTRELEASE`.

Sequence per call:

1. `IF iCHECKLATER THEN iCHECKLATER = 0: EXIT SUB` - the caller asked for the current event to be kept for the next consumer; nothing is polled or cleared (set at M:70 and M:4638).
2. Clear per-call outputs: `iCHANGED`, `KSTATECHANGED`, click/release flags, `mWHEEL`, `K$`, `KB`; remember `mOB = mB`, `mOB2 = mB2`, `KOALT = KALT`; clear `KALTPRESS/KALTRELEASE`. Flush `INKEY$`.
3. `k = _KEYHIT`.
4. Alt+numpad entry (M:18858-18873): while either Alt is down, key *releases* of the digits (k = -48..-57) are collected in a STATIC string. When neither Alt is down and digits were collected, `KB = VAL(last 3 digits)`; if 1..255 then `K$ = CHR$(KB)`, `iCHANGED = -1`, `AltSpecial = _TRUE`; the sub exits. **[quirk]** This early exit happens before the modifier handling, so the Alt release event that triggered it is not processed there.
5. If `k <> 0`: negative means release. Modifier tracking:
   - Left/right Shift: `KSHIFT`.
   - `_KEY_LALT` only: `KALT` plus `KALTPRESS`/`KALTRELEASE`. Right Alt is not tracked.
   - Left/right Ctrl: `KCTRL` and `KCONTROL`.
   - `_KEY_LAPPLE`/`_KEY_RAPPLE`: `KCONTROL` only. So `KCONTROL` means "Ctrl on PC or Cmd on Mac" and is what clipboard shortcuts test, while `KCTRL` is the physical Ctrl key.
   - Each sets `iCHANGED = -1: KSTATECHANGED = -1`.
   - On a press: `iCHANGED = -1`; `K$ = CHR$(k)` for `k <= 255`; `K$ = CHR$(0) + CHR$(k \ 256)` for extended codes 256..65535 with a zero low byte; `KB = k`.
   - If anything changed, exit (mouse is not polled this call).
6. Mouse: `DO WHILE _MOUSEINPUT`: `iCHANGED = 1`; read buttons (swapped if `MouseButtonSwapped`), accumulate `mWHEEL`, read `mX/mY`; exit the sub immediately on the first press or release edge of either button (setting `mCLICK`, `mCLICK2`, `mRELEASE` or `mRELEASE2` to -1). Pure movement drains the queue.

So one call yields at most one key event or one mouse button transition; `iCHANGED` is -1 for keyboard, 1 for mouse, 0 for nothing.

### 4.2 Small helpers

- `ClearMouse` (M:18924): zero `iCHANGED`, `mB`, `mB2`, `mCLICK`, `mRELEASE`, then drain `_MOUSEINPUT` until both buttons are up.
- `CTRL2` (M:18831): on `MacOSX`, returns 1 if `_KEYDOWN(100309)` or `_KEYDOWN(100310)`; 0 elsewhere.
- `iderestrict417 (p417)` (M:18815): if both bit 4 and bit 8 are set in the value, clear both (`AND 243`).
- `CountItems (SearchString$, Item$)` (M:18805): number of occurrences found by repeated `INSTR(Found + 1, ...)`.
- `timeElapsedSince# (t#)` (M:12996): `TIMER(0.001) - t#` with midnight wrap correction.

---

## 5. Editor rendering: `ideshowtext` (M:13002-13699)

### 5.1 Screen geometry

`idewx` x `idewy` (+ `idesubwindow` rows for the help pane) is the window size in cells. `idesx/idesy` are the first visible column/line, `idecx/idecy` the cursor, `maxLineNumberLength = LEN(STR$(iden)) + 1` when line numbers are shown, else 0 (M:964).

| Screen area | Position |
|---|---|
| Menu bar | row 1 |
| Title row (file name, `*`, current SUB) | row 2, `UpdateTitleOfMainWindow` (M:6962) |
| Text rows | rows 3 .. `idewy - 6` (`FOR y = 0 TO idewy - 9`, printed at row `y + 3`) |
| Left border / bookmark mark | column 1 |
| Line-number gutter | columns 2 .. `1 + maxLineNumberLength` (the last of these is the separator) |
| Text | columns `2 + maxLineNumberLength` .. `idewx - 1` |
| Vertical scrollbar | column `idewx`, `idevbar(idewx, 3, idewy - 8, idecy, iden)` (M:13614) |
| Horizontal scrollbar | row `idewy - 5`, `idehbar(2, idewy - 5, idewx - 2, idesx, 608)` (M:13615) |
| Status title row with the "Find[...]" field | row `idewy - 4` (`UpdateSearchBar`, M:6758) |
| Status window, 3 lines | rows `idewy - 3` .. `idewy - 1` |
| Bottom status bar | row `idewy + idesubwindow` |

### 5.2 Palette

The IDE is a 16-colour text screen; `ideshowtext` reprograms the palette on every call unless `ideshowtextBypassColorRestore` is set (M:13004-13018):

| Index | Variable | Used for | Default (CFG:557-567) |
|---|---|---|---|
| 1 | `IDEBackgroundColor` | editor background | 0, 0, 39 |
| 2 | fixed `_RGB32(84, 84, 84)` | dark grey for help/interface details, shadows | - |
| 4 | `IDEErrorColor` | error line / breakpoint background | 170, 0, 0 |
| 5 | `IDEBracketHighlightColor` | bracket match, multi-highlight, found-line number | 0, 88, 108 |
| 6 | `IDEBackgroundColor2` | current-line background, optional gutter background | 0, 49, 78 |
| 7 | `IDEChromaColor` | frames, menus, dialog background | 170, 170, 170 |
| 8 | `IDENumbersColor` | numeric literals | 216, 98, 78 |
| 10 | `IDEMetaCommandColor` | metacommands **and** user SUB/FUNCTION names and custom keywords | 85, 206, 85 |
| 11 | `IDECommentColor` | comments | 98, 98, 98 |
| 12 | `IDEKeywordColor` | keywords | 69, 118, 147 |
| 13 | `IDETextColor` | plain text | 216, 216, 216 |
| 14 | `IDEQuoteColor` | string literals | 255, 167, 0 |

Colours 0, 3, 9, 15 are not reprogrammed here (0 on 7 for dialogs, 0 on 3 for the status bar, 15 for highlights). `DarkenFGBG(1)` halves all of the above while compiling/converting and `DarkenFGBG(0)` restores them (M:20819-20859). **[quirk]** M:20832 reads the misspelt `IDEErrroColor` for the green component; CFG:585 uses `IDENumbersColor` as the fallback when parsing the error colour.

### 5.3 Before the line loop

1. Custom-keyword maintenance, only when the highlighter is enabled (M:13030-13120):
   - If compilation is halted on an error (`idefocusline <> 0`) the IDE scans the buffer itself once per change (`manualList`): every line starting with `SUB ` or `FUNCTION ` (ignoring lines flagged in `InvalidLine()`), strips a trailing ` STATIC`, takes the name up to `(` (or `cleanSubName`: cut at `'`, `:` or space, M:20967), drops the type sigil (`removesymbol2$`, M:20952) and appends `@NAME@` unless it starts with `_IKW_` (M:13039-13071).
   - When the list changed since the last call (`prevListOfCustomWords$`) and it was built by the compiler, every entry beyond `customKeywordsLength` is checked with `HashFind` against FUNCTION and SUB flags and blanked out if no longer registered (M:13079-13100). Runs of `@` are collapsed and `fix046$` is turned back into `.` (M:13103-13116).
2. Scroll the view so the cursor is visible (M:13125-13128).
3. Normalise the selection rectangle `sy1..sy2`, `sx1..sx2` (M:13130-13135).
4. Find the extent of a `_` line-continuation group around the cursor (`idecy_multilinestart/end`) by walking up and down while lines end in `_` outside a comment (M:13140-13175). **[quirk]** M:13160 assigns `idecy_multilinestart = iden` where "end" was presumably meant.
5. `IF idechangemade THEN idefocusline = 0` (M:13180).

### 5.4 Per-line pipeline (highlighter on)

For each visible row (M:13182-13546):

1. Reset column 1 to CHR$(179), then `GOSUB ShowLineNumber` (5.6).
2. Choose the row background: red (`COLOR 7, 4`) for `idefocusline` when the cursor is elsewhere (in debug mode: when the cursor is on it); `COLOR 7, 6` for the cursor line and its continuation group if `HideCurrentLineHighlight = 0` and `IdeSystem = 1`; else `COLOR 7, 1` (M:13188-13194).
3. Fetch the text. For the cursor line only (M:13204-13305):
   - `cc` = ASCII code under the cursor for the status bar (-1 if the cursor is in leading whitespace or past the end).
   - `FindQuoteComment` tells whether the cursor is inside a string or comment.
   - Bracket matching, only if `BracketHighlight` and not in a string/comment: triggered by `(` at or just before the cursor, or `)` at or just before it; scans the **same line only**, skipping quoted text and stopping at a `'` comment when scanning forward (M:13222-13268). Results `bracket1`, `bracket2`.
   - RGB helper: if the line ends in `_RGB(`, `_RGB32(`, `_RGBA(` or `_RGBA32(` and the cursor is right after it, the text " --> Shift+ENTER to open the RGB mixer" is appended for display and `EnteringRGB = -1` (M:13275-13291).
   - `$INCLUDE` link: if the line contains `$INCLUDE` and the quoted file exists (relative to `idepath$` or as given), " --> Double-click to open" is appended and `ActiveINCLUDELink = idecy`, `ActiveINCLUDELinkFile` = the name (M:13293-13304).
4. `a2$` = the text padded with spaces to `idesx + idewx - 3` columns (M:13307-13308); rows past the end of the file are blank.
5. Character loop `FOR m = 1 TO LEN(a2$)` (M:13322-13500). It always starts at column 1, so the tokeniser state is correct when the view is scrolled right; characters left of `idesx` are scanned but not printed, and the loop exits past the right edge (M:13330).

Token rules, in evaluation order per character:

| Step | Rule | Lines |
|---|---|---|
| Watchdog | If the whole call has taken more than 1.0 s: message box, `DisableSyntaxHighlighter = _TRUE`, written to the config, menu item updated, jump to the plain renderer. | M:13323-13329 |
| Multi-highlight | If there is a single-line selection (`ideCurrentSingleLineSelection`, set at M:4469) and `MultiHighlight`, every case-insensitive occurrence that is delimited by separators (or `.` on the right, or line ends) gets the bracket-highlight background for its length. | M:13331-13354 |
| String/comment state | Outside a comment: `"` toggles `inquote`; `'` outside a string starts a comment. | M:13358-13363 |
| Default | `COLOR 13`. Lines beyond `iden` or flagged `InvalidLine(l)` get `COLOR 7` and skip all highlighting. | M:13365-13367 |
| Number | At a token start, outside strings, if the character is in `0123456789-.&`, the previous character is a separator or `?` and not `)`: collect characters from the set `0123456789EDed+-.` plus backtick, `%&!#~`, and `HBOhboACFacf` (M:13022); reject a lone `-`, `.` or `&`; accept if `isnumber()` (M:21152: `&H/&O/&B` prefixes, digits, one decimal point, D/E/F exponent with sign, optional valid type suffix). Otherwise, if it matches a flag in `UserDefineList$`, continue as a word. | M:13370-13396 |
| Word | At a token start (previous char is a separator or `?`, this one is not): collect up to the next separator. Separators (`char.sep$`, M:13020) are: double quote, space, `= < > + - / \ ^ : ; , * ( ) '`. A token starting with `?` outside a comment colours just the `?` as a keyword. | M:13398-13406 |
| Keyword lookup | Upper-cased word looked up as `@WORD@` in `listOfKeywords$`. `$END` followed by ` IF` is treated as `$END IF`; `THEN` on a line starting with `$IF`/`$ELSEIF` is a metacommand. | M:13408-13418 |
| User names | Else, sigil-stripped word found in `listOfCustomKeywords$`: custom keyword. Else, on `$IF`/`$ELSEIF` lines, a word in `UserDefineList$`: custom keyword. | M:13419-13427 |
| Colour | If `KeywordHighlight`: number = 8, custom = 10, keyword = 12; a keyword starting with `$` sets `metacommand`. | M:13433-13442 |
| Comment override | In a comment everything is 11, except the legacy in-comment metacommands `$INCLUDE`, `$DYNAMIC`, `$STATIC`, `$FORMAT` (coloured 10 when it is the last such word on the line) and `ON`/`OFF` right after `$FORMAT:`. | M:13444-13464 |
| Metacommand / string | Else metacommand = 10; else inside a string or on a quote character = 14. | M:13465-13469 |
| Hints | The appended `$INCLUDE` / RGB hint text is colour 10. | M:13473-13474 |
| Background | Bracket cells and multi-highlight cells get background 5, otherwise the row background. | M:13476-13483 |
| Print | One `_PRINTSTRING` per character at column `2 + m - idesx + maxLineNumberLength`, clipped to the text area. | M:13485-13493 |
| After | Count down the token length; when the keyword `REM` ends, `comment = -1`; at token end reset `checkKeyword$`, `metacommand`, `is_Number`, `isCustomKeyword`. | M:13496-13499 |

Things that do **not** exist: there is no special handling of `DATA` statements in `ideshowtext` (unquoted DATA items are tokenised like code), no multi-line state other than the current-line background for `_` continuations, and bracket matching never crosses lines.

6. Selection overlay (M:13503-13543), only when `IdeSystem` is 1 or 2:
   - Single-line selection: cells in `[sx1, sx2)` are re-read from the screen with `SCREEN()` and re-printed in `COLOR 1, 7` (there is a special case that keeps a `?` readable).
   - Multi-line selection: each selected row is re-printed entirely in `COLOR 1, 7` (whole rows, not character ranges); the last row is skipped when the cursor is in column 1 of the lower end.

### 5.5 Plain renderer (highlighter disabled) (M:13548-13604)

Each row is printed with one `_PRINTSTRING` in `COLOR 13, 1` (error line `COLOR 13, 4`), then the same selection overlay. No current-line highlight, brackets, hints or multi-highlight.

### 5.6 Gutter: `ShowLineNumber` (M:13635-13697)

With `ShowLineNumbers`:

- Background 6 if `ShowLineNumbersUseBG`.
- `COLOR 13, 5` for the line where the last search hit landed (`searchStringFoundOn`, cleared after one paint) or the debugger's next line (`debugnextline`, when vWatch is on).
- Breakpoint: background 4 and CHR$(7) in column 2. Skip-line: foreground 14 and `!` in column 2.
- Number right-aligned (`STR$(l)`), only for `l <= iden`.
- Separator column: CHR$(179) if `ShowLineNumbersSeparator`; replaced by a colour-10 CHR$(16) arrow on `debugnextline`.

Without line numbers, column 1 shows CHR$(7) on red (breakpoint), `!` (skip), or CHR$(16) (next line) when vWatch is on.

After the loop: bookmarks are drawn as CHR$(197) in column 1 (M:13606-13612), the scrollbars are drawn, and the position indicator is built: row right-aligned in 10 columns, `:`, then the column and, when the cursor is on a character, its ASCII code in parentheses (`lineNumberStatus$`, printed at column `idewx - 20` of the bottom row in `COLOR 0, 3`, M:13617-13630). Finally the hardware cursor is placed on page 0 (M:13632).

### 5.7 The keyword list

`SH:1` declares `listOfKeywords$, listOfCustomKeywords$, customKeywordsLength`. The file builds `listOfKeywords$` at program start by string concatenation: one `@`-delimited block of metacommands (SH:4-7), then one block per letter, each with three source lines (QB64 keywords, QB4.5 keywords, OpenGL `_GL...` names). Lookup is always `INSTR(list$, "@" + UCASE$(word) + "@")`. It is included from G:135.

`listOfCustomKeywords$`:

- Prefix: user-configured words from the config key `CustomKeywords$` in section "CUSTOM DICTIONARIES" (CFGG:71), upper-cased and `@`-wrapped; its length is `customKeywordsLength` (CFG:504-516).
- Suffix: the compiler appends `@NAME@` for each user SUB/FUNCTION it registers when `idemode` is set (Q:25900-25904), and `ideshowtext` appends names itself when compilation is stuck (5.3).
- Reset to the prefix on File/New (M:6572) and on every load (M:12898, M:21415).
- Other consumer: the export code (`ide\ide_export.bas:388`, `:397`).

`UserDefineList$` (Q:1566, Q:3260, Q:28349) holds precompiler flag names for highlighting in `$IF` lines.

### 5.8 Colour schemes: `LoadColorSchemes` (M:20880-20950)

Format of one scheme: `Name|` followed by 10 colours of 9 decimal digits each (`RRRGGGBBB`), 90 digits in total. Order, as consumed by `ApplyScheme` (M:18014-18035): Text, Keyword, Numbers, Quote, Metacommand, Comment, Background, Background2 (current line), BracketHighlight, Chroma. The error colour is not part of a scheme.

Built-in schemes (`PresetColorSchemes = 14`, M:20887-20902): Super Dark Blue, Dark Blue, QB64 Original, Classic QB4.5, Dark Side, Camouflage, Plum, Cornfield, CF Dark, Broadcast, VS Code, X11 SgiColors, Light Green, All White.

User schemes are read from config section "IDE COLOR SCHEMES" (CFGG:70), keys `Scheme1$`, `Scheme2$`, ... until the first missing key. Older formats are upgraded and written back: 81 digits (9 colours) gets `170170170` appended; 54 digits (6 colours, "version 1.1") gets default keyword/number colours inserted and `000147177170170170` appended; anything else is stored as `"0"` (discarded). `LastValidColorScheme` tracks the last usable index.

### 5.9 Status helpers

| Routine | Line | Behaviour |
|---|---|---|
| `clearStatusWindow (whichLine)` | M:20973 | Blank status line 1-3 (row `idewy - 4 + n`) or all three for 0, in `COLOR 7, 1`. |
| `setStatusMessage (row, text$, fg)` | M:20984 | Print at column 2 of that status row in colour `fg`, then `PCOPY 3, 0`. |
| `printWrapStatus (x, y, initialX, text$)` | M:21217 | Word-wraps at spaces inside the status window (stops after its third row). Inline markers: CHR$(0) toggles colour 11/7, CHR$(1) = `COLOR 7, 1`, CHR$(2) = `COLOR 12, 6`; a marker at the start of a word applies before it, elsewhere after it. |
| `UpdateIdeInfo` | M:20777 | Bottom bar: `IdeInfo` padded/truncated to `idewx - 20` in `COLOR 0, 3`. If `IdeInfo` starts with CHR$(0), the next 3 characters are a percentage and a progress line of `_` is drawn on row `idewy - 1`. Prints `" v" + Version$ + " "` in colour 2 left of the row:col field, then `PCOPY 3, 0`. |
| `UpdateMenuHelpLine (a$)` | M:20812 | Replaces the whole bottom bar with a menu hint. |
| `UpdateTitleOfMainWindow` | M:6962 | Row 2: ` name[*][:CurrentSub] `, centred, truncated with dots; inverse when the editor has focus (`IdeSystem = 1`). "Untitled" + `tempfolderindexstr$` when unnamed. The OS window title is set separately with `_TITLE ideprogname + " - " + WindowTitle` on load/save. |
| `HideBracketHighlight` | M:20861 | Repaints the editor with bracket, multi-highlight, gutter background and current-line highlight off, for dialogs that need palette entries 5/6. |

---

## 6. Search and replace

### 6.1 State

`idefindtext`, `idefindcasesens`, `idefindwholeword`, `idefindbackwards`, `idefindnocomments`, `idefindnostrings`, `idefindonlycomments`, `idefindonlystrings`, `idefindinvert`, `idechangeto` (G:158-165). These are session globals; the dialogs copy them in and out.

### 6.2 Find dialog `idefind$` (M:11739-11984)

60 x 11 box "Find": text box "#Find What"; check boxes "#Match Upper/Lowercase", "#Whole Word", "#Search Backwards", "#Ignore 'comments", "#Look only in 'comments", `Ignore "s#trings"`, `Look only in "st#rings"`; buttons OK / Cancel.

- Initial text: the current single-line selection, else the previous `idefindtext` (M:11756-11769); pre-selected.
- The four scope boxes are made mutually consistent after each event (M:11929-11941): "only comments" clears the other three; "only strings" clears the other three; "ignore comments" clears "only comments"; "ignore strings" clears "only strings". So the valid combinations are: none, ignore comments, ignore strings, ignore both, only comments, only strings.
- Up/Down in the text box walk the search history (`SearchHistory()`, newest first) (M:11958-11976).
- OK/Enter: store options, `AddToHistory "SEARCH"`, `idefindagain 0`, return (function value unset). Esc/Cancel returns "C".
- An empty search string is not rejected by the dialog (behaviour of `idefindagain` with an empty string not verified).

### 6.3 `idefindagain (showFlags)` (M:11986-12115)

1. If `idefindinvert` (set by Shift+F3, M:1973), temporarily flip `idefindbackwards`; it is flipped back and cleared on every exit path.
2. `s$` = search text, upper-cased unless case-sensitive; start at `y = idecy`.
3. For each line: fetch, upper-case if needed. On the start line only part of the line is searched (M:12002-12017):
   - First visit, forward: columns up to and including `idecx` are overwritten with CHR$(255) so that only matches starting after the cursor are found (positions are preserved).
   - First visit, backward: the line is cut to `idecx - 1 + (LEN(s$) - 1)` characters, so only matches starting before the cursor are found.
   - Second visit (after wrapping, `looped = 1`): the complementary part.
4. Forward uses `INSTR(x1, l$, s$)`; backward is a manual right-to-left scan comparing bytes (M:12027-12044).
5. Whole word: the characters before and after the match must not be A-Z or 0-9 (case-folded). Underscore, `$` and other sigils count as boundaries. On failure the search resumes one column further in the same line (M:12047-12066).
6. Scope filter: `FindQuoteComment l$, x, comment, quote` (6.5) and reject according to the four flags (M:12068-12074).
7. Match: `ideselect = 1`, cursor at the match start (`idecx = x: idecy = y`), selection anchor at the match end (`ideselectx1 = x + LEN(s$)`), `searchStringFoundOn = idecy` (gutter flash), `idecentercurrentline`.
8. No match in the line (or the match was rejected by the scope filter - the rest of that line is then not searched): move to the next/previous line, wrapping at the ends with `looped = 1`; when the walk passes the start line a second time, `idenomatch showFlags` (M:12090-12114).

`idenomatch` (M:12499): message box "Search complete / Match not found."; with `showFlags` it adds a second line listing the active flags ("match case", "whole word", "no comments", "only comments", "no strings", "only strings").

Complexity: one pass over the buffer with sequential `idegetline`, plus `UCASE$` of every line when case-insensitive.

### 6.4 Change

**Dialog `idechange$`** (M:10883-11248): 60 x 14 "Change": "#Find What", "Change #To", the same seven check boxes, and buttons "Find and #Verify", "#Change All", "Cancel". Returns "V" (verify; options and both strings stored, M:11206-11217) or "C".

**Change All** is executed inside the dialog (M:11103-11203):

- Scans every line 1..`iden` regardless of cursor position and of "Search Backwards"; draws a progress bar of CHR$(219)/CHR$(176) in the dialog.
- Per line, repeated `INSTR` from `x1`; whole-word and scope filters as in 6.3. A replaced match appends `idechangeto$` and continues after the match, so replacements are non-overlapping and replacement text is not rescanned.
- A changed line is written with `idesetline` (each one copies the whole buffer).
- Afterwards the editor is repainted behind the dialog, `idechanged n` reports "n substitution(s)." or `idenomatch 0`; `idechangemade = 1`; the third button caption becomes "Close" and the dialog stays open.
- **[quirk]** A match rejected by the comment/string filter is not skipped individually: `x = 0` ends processing of that line (M:11164-11179), so later matches on the same line are not replaced.

**Find and Verify** runs in the main loop (M:5949-6106): it searches from the cursor with the same matching code (a third copy of the algorithm), selects each hit, repaints, and asks `idechangeit$`.

`idechangeit$` (M:11278-11399) is a 45-wide, 2-high strip placed at `p.y = idewy - 4` with buttons "#Change", "#Skip", "Cancel". Plain `C` and `S` keys work without Alt (M:11361-11362). Returns "Y", "N" or "C" (Esc = "C").

- "Y": the line is rebuilt as `LEFT$(l$, idecx - 1) + idechangeto$ + RIGHT$(l$, LEN(l$) - ideselectx1 + 1)`, `idesetline`, `idechangemade = 1`, counter + 1; the scan continues after the replacement and the wrap-around end column `startx` is adjusted by the length difference (M:6053-6071).
- "N": continue from the next column. "C": restore the cursor, report the count if any.
- When the walk returns to the start, the cursor is restored and `idechanged` reports the count (M:6098-6106).

### 6.5 `FindQuoteComment (text$, cursor, c, q)` (M:11250-11271)

Scans `text$` from column 1 to `cursor` (clamped to the length). `"` toggles `q`. Outside a string, `'` sets `c = -1` and stops; so does `REM` when it appears as ` REM `, `:REM `, ` REM`/`:REM` at the end of the line, `REM ` at the start, or the whole line. Outputs are `_BYTE` flags (0 / -1). Because the scan includes the cursor column, a match that starts on an opening quote is reported as "in string". Used by search, by the renderer and by `FindProposedTitle$`.

### 6.6 History and the quick search bar

- History file: `SearchedFile$ = ConfigFolder$ + pathsep$ + "searched.bin"` (CFGG:50), one entry per line, newest first. `AddToHistory "SEARCH", entry$` (M:19900) removes a case-insensitive duplicate and any entries beyond `ideMaxSearch`, then inserts at the top. `RetrieveSearchHistory arr$()` (M:19945) loads up to `ideMaxSearch` entries. The same routine handles "RECENT" with `RecentFile$` (CFGG:49) and `ideMaxRecent`.
- Quick search bar ("Find[...]" in the status title row, width `idesystem2.w = 20`, G:139): `IdeSystem = 2`. Entered with Ctrl+F (M:1940-1945) or by clicking the field (M:2130-2168). It edits `idefindtext` directly with its own inline copy of the text-box logic (`idesystem2.issel/sx1/v1`, M:84-86, M:2216-2367). Enter adds to history and jumps to `idemf3` (M:2309-2313); Esc or Tab return to the editor; Up/Down/PgUp/PgDn/wheel return to the editor and are then processed there; Alt+Up/Down opens the recent-searches box (M:2207); F2 opens the SUBs dialog.
- F3 / Shift+F3 (`idemf3`, M:1963-1978): if there is search text, `AddToHistory`, `idefindagain -1` (Shift sets `idefindinvert`); with no text it opens the Find dialog.
- Menu entries: "#Find...  Ctrl+F3" (M:259, handled M:5925), "#Repeat Last Find  (Shift+) F3" (M:261), "#Change...  Alt+F3" (M:263, handled M:5941), "Clear Search #History..." (M:266); the contextual menu "Find '...'" sets `idefindtext` and jumps to `idemf3` (M:5934-5938). The debugger has its own F3 hook (M:7737-7742).

---

## 7. Helpers

| Routine | Line | Behaviour |
|---|---|---|
| `getWordAtCursor$` | M:20990 | Looks at the cursor column; if it is a space (or one past the end) and the previous character is not, uses the previous character. If alphanumeric (`alphanumeric()`): extends left and right over alphanumerics and `$`. Otherwise returns the run of that same symbol, with special cases: `~` and backtick return themselves, `%&` is returned as a pair. Returns "" on blank space. |
| `getSelectedText$ (multiline)` | M:21031 | Single-line selection: characters `[sx1, sx2)`, padded with spaces past the end of the line. Multi-line: "" unless `multiline`; otherwise **whole lines** joined with CRLF; the last line is omitted if the cursor is in column 1 of the lower end; if the cursor is further right the final CRLF is dropped. |
| `delselect` | M:21059 | Single-line: removes `[sx1, sx2)`. Multi-line: deletes whole lines `sy1..sy2` bottom-up (excluding the last one when the cursor is in column 1 of it); if only one line remains it is emptied instead. Cursor to `(sx1, sy1)`, or column 1 for multi-line; `ideselect = 0`. Does not set `idechangemade`. |
| `insertAtCursor (tempk$)` | M:21087 | Deletes the selection, pads the line with spaces up to the cursor, inserts, `idesetline idecy, converttabs$(...)`; moves the cursor past the insert only if `PasteCursorAtEnd`; sets `idechangemade = 1`. Single-line inserts only. |
| `FindCurrentSF$ (whichline)` | M:20687 | Walks upward from `whichline` to the nearest line starting with `SUB ` or `FUNCTION ` (stops with "" at an earlier `END SUB`/`END FUNCTION`). Strips ` STATIC` and the argument list. Then rejects the hit if it sits inside a `DECLARE ... LIBRARY` block (upward scan), and scans downward from `whichline` to confirm an `END SUB`/`END FUNCTION` follows before the next procedure header. Up to three linear scans; called for every title repaint (M:6963). |
| `FindProposedTitle$` | M:20655 | First line containing `_TITLE "` that is not inside a string (per `FindQuoteComment`); takes the quoted text, removes `: / \ ? * > < \| "`, trims. Used as the suggested file name for Save As (M:6511 etc.). |
| `findHelpTopic$ (topic$, lnks, firstOnly)` | M:21104 | Looks `topic$` up (case-insensitive) in `internal\help\links.bin`, a text file of `keyword,page` lines. Returns the matching page names separated by CHR$(0) (leading CHR$(0); an exact-name page is put first) and the count in `lnks`; with `firstOnly` returns the first page name directly. If the file is missing or empty it offers to initialise the help system by downloading "Keyword Reference - Alphabetical" (`Wiki$`, `WikiParse`). |
| `AddQuickNavHistory` | M:20762 | Appends `{idesx, idesy, idecx, idecy}` to `QuickNavHistory()` unless the last entry has the same `idecy`. Unbounded growth; `QuickNavTotal` is reset on load (M:6671). |
| `isnumber (a$)` | M:21152 | Numeric-literal test used by the highlighter (see 5.4). |
| `removesymbol2$`, `cleanSubName` | M:20952, M:20967 | Strip a type sigil (`~`, backtick, `% & ! # $`) and anything after it; cut a name at `'`, `:` or space. |
| `GetBytes$ (value$, n&)` | M:21282 | Stateful sequential reader over a string: returns the next `n&` bytes; restarts when called with a different string; `n& = 0` just resets/primes. |
| `idenewsf (sf$)` | M:12447 | "New SUB/FUNCTION": input box (max 40 chars, default = single-line selection), then appends a blank line, `SUB name`, a blank line and `END SUB` at the end of the buffer and moves the cursor there. |

---

## 8. Not determined / not verified

- The runtime error trap that turns `ideerror` codes into message boxes (`IDEerrorMessage`, M:104 onward) was only read as far as the code table; how file errors resume was not traced (ide2 section).
- Behaviour of `idefindagain` and Change with an **empty** search string (the Find dialog does not block it).
- Whether any caller passes `i = -1` to `ideinsline`/`idedelline` (no literal `-1` call was found; variable arguments were not traced).
- Practical maximum file size / line length: no explicit limit was found; the figures in 1.4 are inferences from the code, not tested.
- BOM/UTF-8: absence of handling is concluded from reading the three loaders and `lineinput3*`; it was not tested with a file.
- What key codes 100309/100310 in `CTRL2` are, and where `CTRL2`/`iderestrict417` are called.
- The undo system (`ideundotxt`, undo file format), auto-layout (`idelayoutallow` consumer at M:747) and `IdeImportBookmarks`/`IdeSaveBookmarks` formats were only touched at their call sites.
- macOS: the `idez*` helpers were read for `os$` = "WIN" and "LNX" only; I did not check what `os$` is on macOS.
- `StripDiscordANSI$` (`ide\ide_export.bas:633`) and `alphanumeric()` were not read.
- The complete contents of `listOfKeywords$` (lines are very long) were sampled, not read in full.
- The inline quick-search-bar editor (M:2216-2367) and the Find-and-Verify loop (M:5949-6106) were read only as needed to describe the hooks.
- None of the **[quirk]** items were confirmed by running the IDE; they are readings of the source.

---

# Part D — Dialogs and menu builders

*(Section numbers below are local to Part D.)*

Scope: `..\QB64pe\source\ide\ide_methods.bas` lines 13701-14928, 15773-16968, 17127-18804, 18936-20654. All `file:line` references are to that file unless another file is named. Line numbers were read directly from the source.

Out of scope (documented by other sections, only referenced here): the main loop `ide2` (82-6960), debugger dialogs (6976-10848), toolkit primitives (`idedrawobj`, `ideobjupdate`, `idemessagebox`, `idefiledialog$`, `idefind$`, `idechange$`, `ideshowtext`, `GetInput`, `OpenFile$`/`SaveFile$`), wiki/config/export files.

## 0. Shared conventions

### 0.1 The dialog skeleton

Every dialog in this section follows the same roughly 100-line template, written out in each function. There is no shared dialog runner; each function contains its own event loop.

| Step | Code (example from `ideLayoutBox`) | Meaning |
|---|---|---|
| Header | `PCOPY 0, 2 : PCOPY 0, 1 : SCREEN , , 1, 0` (15776-15778) | Page 2 = saved background, page 1 = work page, page 0 = visible |
| Objects | `DIM p AS idedbptype`, `DIM o(1 TO 100) AS idedbotype` (15780-15781) | One parent box and up to 100 controls |
| Parent | `idepar p, w, h, "Title"` (15788) | Size and title of the box (centred by `idepar`, 12943) |
| Controls | `o(i).typ = n` | 1 = text box, 2 = list box, 3 = button row, 4 = check box, 5 = symbol button (spinner arrow / one-character button) |
| Strings | `o(i).nam = idenewtxt("...")`, `o(i).txt = idenewtxt("...")` | Label and content. `#` marks the Alt hot-key letter. Button rows and list boxes hold several items separated by `sep = CHR$(0)` |
| Draw | `idedrawpar p` then `idedrawobj o(i), f` for each control (15852-15871) | `f` counts focusable sub-items; `lastfocus = f - 1` |
| Input | `GetInput` loop at `_LIMIT 100` until something changes (15885-15896) | Alt+letter is converted to `altletter$` |
| Dispatch | Tab / Shift+Tab move `focus`; `ideobjupdate` per control sets `info` when a control is activated (15910-15922) | `focus` numbers count each button of a button row separately |
| Close | Esc or Cancel = `EXIT FUNCTION` (return 0 / empty); Enter or OK = validate, apply, write config | |

Common patterns in the newer dialogs (Layout, Limits, Compiler, Logging, Terminal, Language, Display):

- Text boxes get their content fully selected when they receive focus (15859-15866).
- Validation runs on every loop pass and sets `o(x).inv = 1` on a bad field (drawn highlighted) plus `invdata = 1`. OK with `invdata` set shows `idemessagebox("Warning", "Confirmation has been blocked due to invalid settings.\nPlease check your inputs, look for highlighted boxes.", "#OK")` and stays in the dialog (15976-15979).
- On OK each control is compared with its global; `optChg%` records whether anything differs. Config keys are written and the function returns 1 only when something changed; otherwise it returns 0, the same as Cancel.
- Spinner arrows are `typ = 5` symbol buttons with `CHR$(30)` / `CHR$(31)` and an auto-repeat value in `.rpt`.

### 0.2 Config sections

Defined in `source\ide\config\cfg_global.bas:68-77`:

| Variable | Section name in `settings/config.ini` |
|---|---|
| `generalSettingsSection$` | `GENERAL SETTINGS` |
| `displaySettingsSection$` | `IDE DISPLAY SETTINGS` |
| `debugSettingsSection$` | `DEBUG SETTINGS` |
| `compilerSettingsSection$` | `COMPILER SETTINGS` |
| `loggingSettingsSection$` | `LOGGING SETTINGS` |
| `mouseSettingsSection$` | `MOUSE SETTINGS` |
| `windowSettingsSection$` | `IDE WINDOW` + instance number (per IDE instance) |
| `colorSettingsSection$` | `IDE COLOR SETTINGS` + instance number (per IDE instance) |

Data files (`cfg_global.bas:45-52`): `settings/config.ini`, `settings/bookmarks.bin`, `settings/recent.bin`, `settings/searched.bin`, `settings/undo3<instance>.bin`.

## 1. View menu dialogs

### 1.1 `idesubs$` — "SUBs" (13701-14257)

**Opened from:** View menu item `#SUBs...  F2` (built at 247, handled at 5805-5816; label `idesubsjmp:` at 5811). Also F2 in the main editor (3542) and F2 while the Find field of the status area has focus (2335-2337), both via `GOTO idesubsjmp`. The debugger calls it too (7718). While the debuggee is paused (`IdeDebugMode = 2`) the menu item re-enters debug mode with `IdeDebugMode = 14` instead of opening the dialog directly (5806-5808).

**How the list is built: it scans the edit buffer itself, not compiler data.** A single pass `FOR y = 1 TO iden` reads each line with `idegetline(y)` (13763-13870):

- The line is trimmed and upper-cased. A line starting `DECLARE ` and containing ` LIBRARY` sets `InsideDECLARE = -1`; a line starting `END DECLARE` clears it (13769-13770).
- A line starting `SUB ` (type text `"SUB   "`) or `FUNCTION ` (type text `"FUNC  "`) starts a new entry (13771-13772). Only the start of the trimmed line is tested, so a SUB that follows a colon or a line label on the same line is not detected.
- A trailing ` STATIC` is stripped (13787-13789).
- The name is everything before the first `(` that is not inside a quote or comment (`FindQuoteComment`, 13807-13811). Arguments are the balanced-parenthesis text from that `(` (13812-13818). With no parenthesis the arguments are shown as `()` (13820-13821). `cleanSubName` (20967) tidies the name.
- Entries inside `DECLARE LIBRARY` get the name prefix `*` and set `FoundExternalSUBFUNC` (13837-13839).
- Line counts: the scanner counts lines from the SUB/FUNCTION line until a line containing `END SUB` or `END FUNCTION` outside quotes/comments (13859-13868, `AddLineCount` at 14251-14256). `TotalLines(0)` receives the count of main-module lines before the first procedure, but it is never displayed. External entries keep 0 lines.
- Parallel arrays: `SubNames()`, `SubLines()`, `Args()`, `SF()`, `TotalLines()`, plus `SortedSubsList()` / `CaseBkpSubsList()` as `STRING * 998` records whose bytes 992-997 carry `MKL$(line) + MKI$(itemLength)` (13945-13946).

**Preselection:** `CurrentlyViewingWhichSUBFUNC` is the last non-external procedure that starts at or before the cursor line (13796-13798). If the word under the cursor (`getWordAtCursor$`, sigil stripped, 13715-13721) equals a procedure name, that entry wins (`PreferCurrentCursorSUBFUNC`, 13829-13835). This makes "cursor on a call, press F2, Enter" a jump-to-definition.

**What is displayed:** the first row is the program name (`ideprogname$`, or `Untitled` + instance suffix), truncated with three `CHR$(250)` dots past 20 characters (42 when the IDE is 100+ columns wide) (13725-13740). Each procedure is a tree row: `├─name  [line count]  Type  Arguments` (13933-13939), last row uses `└` (13949-13950).

| Column | Content |
|---|---|
| Name | Procedure name, padded/truncated to `maxModuleNameLen`; `*` prefix for external (DECLARE LIBRARY) entries |
| Line count (optional) | Header `Line count` (10 wide) or `Lines` (5 wide) depending on the largest count; external rows show `external` or a `─` character (13886-13896, 13936) |
| Type | `SUB` or `FUNC`, drawn in a different colour via in-band `CHR$(16)+CHR$(2)` colour codes |
| Arguments | Raw argument text including parentheses, truncated to the window width (13923-13929) |

No return type, no file/module information and no `$INCLUDE` contents are shown; only the current edit buffer is scanned. A legend `* external` is printed on the bottom border when any external entry exists (14089-14092).

Four list strings are prepared up front: `l$` (source order), `lSized$` (source order with line counts), `lSorted$`, `lSortedSized$` (13885-13977). Sorting uses the local `sort` routine (20642) on the upper-cased rows, so the order is case-insensitive alphabetical by name; an inner loop restores the original casing (13952-13977). `ly$` and `lySorted$` are packed `MKL$` strings mapping list row to source line; row 1 (the program name) maps to line 1 (13742-13743).

**Controls**

| # | Control | Type | Bound to | Notes |
|---|---|---|---|---|
| 1 | `Program Items` | list box | — | Size adapts to the number of procedures and the window (13985-13999) |
| 2 | `#Line Count` | check box | `IDESubsLength` | Switches list text immediately (14166-14182) |
| 3 | `#Sort` | check box | `IDESortSubs` (via `SortedSubsFlag`) | Only honoured when there is more than one procedure (13980, 14184); selection is preserved across the switch by source-line lookup (14188-14226) |
| 4-5 | `#Edit` / `#Cancel` | buttons | — | The first button reads `#View` when `IdeDebugMode <> 0` (14056-14060) |

**On Enter / Edit / double-click (14148-14164):** `AddQuickNavHistory` is called first, so the previous cursor position is pushed to the quick-navigation history and the back arrow returns to it. Then `idecy` and `idesy` are set to the entry's line and `idecx = idesx = 1`. The function returns an empty string.

**On Esc / Cancel (14141-14146):** returns `"C"`.

**Both exits** run `SaveSortSettings` (14236-14249): `IDESortSubs = SortedSubsFlag` and writes `[IDE DISPLAY SETTINGS] IDE_SortSUBs` and `IDE_SUBsLength` as `True`/`False`. The check boxes are therefore persisted even on Cancel. Defaults (from `cfg_methods.bas:349-386`): sort off, line count on.

**Return value:** `"C"` = cancelled, `""` = jumped. The caller clears the selection when not cancelled (`IF r$ <> "C" THEN ideselect = 0`, 5813).

### 1.2 `idewarningbox` — "Compilation status" (14737-14927)

**Opened from:** View menu `Compiler #Warnings...  Ctrl+W` (built 253, handled 5918-5922); Ctrl+W in the editor when `totalWarnings > 0` (3546-3550); a click on the warning link in the status area (1687-1691).

**Data source: compiler-owned shared arrays**, not a re-scan:

| Array / variable | Use |
|---|---|
| `warningListItems` | Number of rows (14756, 14768) |
| `warning$(x)` | Message text. When `warningLines(x) = 0` the row is a heading row and is shown as-is (14769-14771) |
| `warningLines(x)` | Line number in the main buffer; 0 marks a heading row |
| `warningIncLines(x)` | Greater than 0 when the warning is inside an include file; this is the line within that file (14774) |
| `warningIncFiles(x)` | Include file name for such rows (14777) |
| `totalWarnings` | Shown in the list caption `Warnings (n)` (14815) |
| `maxLineNumber` | Sets the width of the line-number column (14775, 14779) |

Row format for a real warning: `├─<file>:<line>: <text>` where `<file>` is the include file name or the current program name, padded to the longest name, in dark grey via in-band colour codes (14773-14790). The last row under each heading gets `└` (14771, 14795-14797). Dialog width grows to the longest row, capped at `idewx - 8`.

**Controls:** list box `Warnings (n)`; buttons `#Go to` / `#Close`. A hint line reads `Double-click on an item to jump to the line indicated` (14851).

**Go to (14905-14919):** if the selected row has `warningLines(y) > 0`: `idegotobox_LastLineNum` is set (so the Go To Line dialog later offers that number), `AddQuickNavHistory`, `idecy = line`, `idecentercurrentline`, `ideselect = 0`. For include-file warnings the jump goes to the `$INCLUDE` line in the main buffer and `warningInInclude = idecy`, `warningInIncludeLine = warningIncLines(y)` are set; the main loop uses these at 3159 (status-bar display of the line inside the include file). Enter on a heading row does nothing.

**Return value:** always 0 (not used).

## 2. Run menu dialogs

### 2.1 `ideQBJSBuildBox` — "QBJS Web Build" (14260-14583)

**Opened from:** Run menu `#QBJS Web Build...` (built 324, handled 5376-5381). It is a SUB (exits with `EXIT FUNCTION` statements at 14492/14519/14528, which is how the source is written).

**Controls**

| Control | Type | Stored in | Default |
|---|---|---|---|
| `QB64 Language Support` | button | — | Opens `https://github.com/boxgaming/qbjs/wiki/QBasic-Language-Support` with `start` / `open` / `xdg-open` (14573-14582) |
| `Port` | text box, width 5 | `[QBJS] Port` | `8080` |
| `Compile only` | check box | `[QBJS] CompileOnly` (`0`/`1`) | 0 |
| `Copy project files` | check box | `[QBJS] CopyProjectFiles` (`0`/`1`) | 1; drawn greyed when Compile only is on (14360-14367) |
| `Complier Warnings` (spelling as in source, 14313) | list box | — | Filled from the build's warnings file |
| `#Run Web Build` / `#Go to Selected` / `#Close` | buttons | — | `Go to Selected` is drawn greyed when no warning row is selected (14370-14377) |

The port is not validated. Settings use a literal section name `"QBJS"` and are read with `ReadConfigSetting` when the dialog opens (14555-14562) and written by `UpdateSettings` on Run and on Close/Esc (14564-14571). There are no IDE globals for them.

**Build flow** (a small state machine on `webBuildStatus$`, shown as `Status: ...`):

1. Run pressed: if `internal/support/converter/qbjs-build[.exe]` does not exist, status becomes `Compiling qbjs-build tool...` and the IDE shells its own executable: `qb64pe -x internal/support/converter/qbjs-build.bas -o ...` (14388-14395).
2. `Building...`: the edit buffer is written to `<idepath>/.qbjs-temp.bas` with a first line `'$Include: 'lib/compatibility/qb64pe.bi'` (14404-14415). Then `qbjs-build -port:<port> "-warnings:<file>" [-compileOnly] [-noProjectFiles] "<temp file>"` runs, with stdout redirected to `.qbjs-build-out` and the exit code to `.qbjs-exit-code` in the QB64-PE folder (14416-14425).
3. Exit code 1 = Node.js not found, 2 = no network; otherwise the warnings file `.qbjs-warnings-<TIMER>` is read line by line into the list and deleted (14427-14435, 14530-14553). Status becomes `Build Complete.`.

**Go to Selected (14511-14522):** the line number is the text between the first and second `:` of the selected warning; `idecy` is set to it and the dialog closes. No quick-nav entry is added and no range check is made.

**Return value:** none.

### 2.2 `ideTerminalBox` — "Default Terminal" (16826-16967)

**Opened from:** Run menu `Change #Terminal...` (handled 5369-5374). The item only exists on Linux (`os$ = "LNX" AND MacOSX = 0`, 315-318).

| Control | Type | Global | Config key |
|---|---|---|---|
| `Terminal Command` | text box | `DefaultTerminal$` | `[GENERAL SETTINGS] DefaultTerminal` |
| `#OK` / `#Cancel` | buttons | | |

Help text: `Placeholder $$ will be replaced with the executable name.` and `Placeholder $@ will be replaced with the COMMAND$ string.` (16889-16890). No validation. Returns 1 if changed, else 0.

### 2.3 `ideLoggingBox` — "Logging Configuration" (16541-16824)

**Opened from:** Run menu `Configure #Logging...` (built 321, handled 5383-5388). These settings apply to programs run from the IDE.

| Control | Type | Global | Range / notes |
|---|---|---|---|
| `#Level` | text box + up/down spinners | `LogMinLevel$` | 1-5 mapped to `Trace`, `Information`, `Warning`, `Error`, `None` (16561-16564, 16782-16784). A description is shown beside it: `=> Trace, Info, Warnings, Errors` ... `=> None (logging disabled)` (16627-16633) |
| `QB64 user #program _LOG.. statements (qb64)` | check box | `LogScopes$` contains `qb64` | |
| `QB64 C++ #general runtime internals  (libqb)` | check box | `LogScopes$` contains `libqb` | |
| `QB64 C++ #audio subsystem internals  (libqb-audio)` | check box | `LogScopes$` contains `libqb-audio` | |
| `QB64 C++ #image subsystem internals  (libqb-image)` | check box | `LogScopes$` contains `libqb-image` | |
| `Console #Window` | check box | `LogHandlers$` contains `console` | |
| `Log#file (specify below)` | check box | `LogHandlers$` contains `file` | |
| `Logf#ile` | text box | `LogFileName$` | |
| `≡` (`CHR$(240)`) | symbol button | — | Opens `idefiledialog$(current, 4)`; result is passed through `RemoveDoubleSlashes$` (16730-16737) |
| `#OK` / `#Cancel` | buttons | | |

Group captions drawn by hand: ` Show logging from (Scope) ` and ` Output logging to (Handler) ` (16668-16671).

**Validation (16739-16768):** level must be an unsigned integer 1-5. When the level is below 5, at least one scope and at least one handler must be ticked. If the file handler is ticked and the file does not exist, the dialog tries to create and delete it (`_WRITEFILE`, `KILL`, guarded by `ON ERROR GOTO _NEWHANDLER qberror_test`); failure marks the file box invalid. This file probe runs on every loop pass, not only on OK.

**On OK (16779-16817):** `LogScopes$` and `LogHandlers$` are rebuilt as comma-separated lists; `LoggingEnabled = (LogMinLevel$ <> "None")`; `LogToConsole = LoggingEnabled AND console handler`. Writes `[LOGGING SETTINGS] LogMinLevel`, `LogScopes`, `LogHandlers`, `LogFileName`. Returns 1 if changed. The caller ignores the result; no recompile is triggered.

## 3. Options menu dialogs

### 3.1 `ideLayoutBox` — "Code Layout" (15773-16025)

**Opened from:** Options menu `#Code Layout...` (built 372, handled 5433-5439).

| Control | Type | Global | Config key (`[IDE DISPLAY SETTINGS]`) | Notes |
|---|---|---|---|---|
| `Auto #Indent lines` | check box | `IDEAutoIndent` | `IDE_AutoIndent` | Turning it off clears "Indent SUBs" and resets spacing to 4 (15927-15930) |
| `Indent #Spacing` | text box + up/down spinners | `IDEAutoIndentSize` | `IDE_IndentSize` | Unsigned integer 1-64 (15943-15945). Valid typing or a spinner click turns Auto Indent on |
| `Indent SUBs and #FUNCTIONs` | check box | `IDEIndentSubs` | `IDE_IndentSUBs` | Turning it on turns Auto Indent on (15950-15952) |
| `#Auto Single-spacing code elements` | check box | `IDEAutoLayout` | `IDE_AutoFormat` | |
| `#UPPER` / `Ca#MeL` / `#lower` | three check boxes acting as a radio group, under the caption ` Show Keywords as ` | `IDEAutoLayoutKwStyle` (1 / 0 / -1) | `IDE_KeywordCapital` (true when style is 1), `IDE_KeywordLowercase` (true when style is -1) | Ticking one clears the other two (15957-15970). Unticking all three leaves `v%` at whatever the previous assignment left it (15994-16000), so "none ticked" has no defined meaning |
| `#OK` / `#Cancel` | buttons | | | |

**On OK with changes (16003-16017):** also sets `DEFAutoIndent = IDEAutoIndent` and `DEFAutoLayout = IDEAutoLayout` (the values restored after a `'$FORMAT:OFF` region). Returns 1. A changed indent size only counts as a change when auto-indent is on (15987-15990), although the global is updated either way.

**Caller effect (5436):** `idechangemade = 1 : idelayoutallow = 2 : startPausedPending = 0` — the whole buffer is re-checked and re-laid-out by the compiler pass.

### 3.2 `ideCompilerSettingsBox` — "Compiler Settings" (16300-16539)

**Opened from:** Options menu `Co#mpiler Settings...` (built 374, handled 5441-5451).

These are all the options that exist in this dialog:

| Control | Type | Global | Config key (`[COMPILER SETTINGS]`) | Validation |
|---|---|---|---|---|
| `Compile #program with C++ optimization flag` | check box | `OptimizeCppProgram` | `OptimizeCppProgram` | |
| `#Strip C++ symbols from executable` | check box | `StripDebugSymbols` | `StripDebugSymbols` | |
| `#Add C++ Debug Information` | check box | `IncludeDebugInfo` | `IncludeDebugInfo` | |
| `Use a#bsolute source paths in debug info` | check box | `AbsoluteDebugPaths` | `AbsoluteDebugPaths` | |
| `C++ Compiler #Flags` | text box | `ExtraCppFlags$` | `ExtraCppFlags` | free text |
| `C++ #Linker Flags` | text box | `ExtraLinkerFlags$` | `ExtraLinkerFlags` | free text |
| `#Max C++ Compiler Processes` | text box + up/down spinners | `MaxParallelProcesses` | `MaxParallelProcesses` | unsigned integer 1-128 (16469-16471) |
| `#Use system C++ compiler` | check box, Windows only (16363-16369) | `UseSystemMinGW` | `UseSystemMinGW` | Turning it on shows `Using the system MinGW compiler may cause problems.` (16521-16524) |
| `#OK` / `#Cancel` | buttons | | | |

The dialog is 17 rows on Windows and 16 elsewhere (16315).

**Not in this dialog:** "Generate License For EXE" and "Output EXE to Source Folder" are Run-menu toggles (300-311), and "Set Default EXE Folder..." is a separate Run-menu item (319).

**On OK with changes (16507-16531):** writes all keys, then calls `PurgeTemporaryBuildFiles (os$), (MacOSX)` to delete compiled intermediate files, and returns 1.

**Caller effect (5444-5448):** `idechangemade = 1` (forces a recompile), `IF ideunsaved = 0 THEN ideunsaved = -1` (keeps the "saved" state), `startPausedPending = 0`.

### 3.3 `ideLanguageBox` — "Language Settings" (14586-14735)

**Opened from:** Options menu `#Language...` (built 377, handled 5180-5185).

| Control | Type | Global | Notes |
|---|---|---|---|
| `Code Pages` | list box | `idecpindex` | Items are `idecpname(1..idecpnum)` in upper case (14606-14608). Caption above: `Codepage for ASCII-UNICODE mapping (Default = CP437):` |
| `#OK` / `#Cancel` | buttons | | Double-click on the list also confirms (14702) |

**On OK with a different selection (14709-14727):** writes `[IDE DISPLAY SETTINGS] IDE_CodePage` = index, then applies the mapping immediately: for characters 128-255 the Unicode value is read as 8 hex digits from `idecp(idecpindex)` at offset `x * 8 + 1` and applied with `_MAPUNICODE u TO x`; a zero entry maps to U+2610 (9744). Returns 1. The code page table itself (`idecp()`, `idecpname()`) is defined outside this range (not examined here).

### 3.4 `ideLimitsBox` — "Backup/Undo & History Limits" (16027-16260)

**Opened from:** Options menu `#Undo/History...` (built 379, handled 5523-5528).

| Control | Type | Global | Config key (`[GENERAL SETTINGS]`) | Range | Spinner step |
|---|---|---|---|---|---|
| `Max. #Undo Limit (10-2000MB)` | text box + spinners | `idebackupsize` | `BackupSize` | 10-2000 | 5 |
| `Max. #Recent Files (5-200)` | text box + spinners | `ideMaxRecent` | `MaxRecentFiles` | 5-200 | 1 |
| `Max. #Search Strings (5-200)` | text box + spinners | `ideMaxSearch` | `MaxSearchStrings` | 5-200 | 1 |
| `#OK` / `#Cancel` | buttons | | | | |

**On OK:** if the undo limit is reduced, the undo file is emptied (`_WRITEFILE UndoFile$, ""`) and `ideundobase = 0 : ideundopos = 0` (16229-16233) — all undo history is lost. The new recent/search limits do not trim the existing history files at this point; trimming happens on the next `AddToHistory` (see 6.5). Default for `MaxRecentFiles` is 20 (`cfg_methods.bas:272-275`). Returns 1 if changed.

### 3.5 Simple input boxes (16262-16298)

All three use the toolkit function `ideinputbox$(title$, caption$, initial$, validchars$, boxwidth, maxlength, ok)` (12291).

| Routine | Title / label | Opened from | Behaviour |
|---|---|---|---|
| `idegotobox` (16262-16275) | `Go To Line` / `#Line` | Search menu `#Go To Line...  Ctrl+G` (283, 5516-5521); Ctrl+G (3480); a second key path at 1553 | Digits only, max 8 characters. Initial value is `idegotobox_LastLineNum` if set. The value is clamped to 1..`iden`, then: `AddQuickNavHistory`, `idecy = v&`, `idecentercurrentline`, `ideselect = 0`. Empty input cancels |
| `ideSetTCPPortBox` (16277-16285) | `Base TCP/IP Port Number` / `#Port number for $DEBUG mode` | Debug menu `Set Base #TCP/IP Port Number...` (360, 6480-6486) | Digits only, max 5 characters. `idebaseTcpPort = VAL(v$)`; 0 becomes 9000. No upper bound check (values above 65535 are accepted). Writes `[DEBUG SETTINGS] BaseTCPPort`. The caller closes the host socket when the port changed (6484-6486) |
| `idegetlinenumberbox(title$, initialValue&)` (16287-16298) | caller's title / `#Line` | Debugger: `Run To Line` (8124), `Set Next Line` (8139), `Skip Line` (8151) | Returns the clamped line number, or 0 when cancelled. Does not move the cursor |

### 3.6 `ideDisplayBox` — "Display Settings" (17127-17539)

**Opened from:** Options menu `#Display...` (built 368, handled 5187-5205). The dialog is not opened while the help window is visible (`IF idehelp = 0`, 5189). The box is positioned as if the screen were 80x25 (17144) so it stays reachable when the window was made too large.

| Control | Type | Global | Config key | Range | Default (`cfg_methods.bas`) |
|---|---|---|---|---|---|
| `Window #width` | text box + spinners | `idewx` | `[IDE WINDOW n] IDE_Width` | 80-999 | 120 (line 550) |
| `Window #height` | text box + spinners | `idewy` (shown as `idewy + idesubwindow`) | `[IDE WINDOW n] IDE_Height` | 25-999 | 40 (line 554) |
| `#Remember position + size` (Windows/macOS) or `#Remember size` (Linux) | check box | `IDEAutoPosition` | `[IDE DISPLAY SETTINGS] IDE_AutoPosition` | | not checked |
| `Cursor #start` | text box + spinners | `IDENormalCursorStart` | `IDE_NormalCursorStart` | 0-31 | 6 (line 406) |
| `Cursor #end` | text box + spinners | `IDENormalCursorEnd` | `IDE_NormalCursorEnd` | 0-31 | 8 (line 413) |
| `#Use _FONT 8` | check box | `IDEUseFont8` | `IDE_UseFont8` | | False (476) |
| `Use monospace #TTF, TTC, OTF, FNT, FON, PCF, BDF font:` | check box | `IDECustomFont` | `IDE_CustomFont` | | False (468) |
| `#Font file` | text box | `IDECustomFontFile$` | `IDE_CustomFont$` | 1-1024 characters and the file must exist (17418-17420) | Windows: `<fonts dir>lucon.ttf`; Linux: `truetype/liberation/LiberationMono-Regular.ttf`; macOS: `Courier New.ttf` (480-490) |
| `CHR$(240)` (three-bar symbol) | symbol button | — | — | Opens `idefiledialog$("*.tt*", 3)` (17407-17416) | |
| `Font size in #pixels` | text box + spinners | `IDECustomFontHeight` | `IDE_CustomFontSize` | 8-99 | 19 (line 495) |
| `#OK` / `#Cancel` | buttons | | | | |

Behaviour worth keeping in mind:

- The cursor start/end values are previewed live: every loop pass runs `LOCATE , , , tmpNormalCursorStart, tmpNormalCursorEnd` (17286), so the text cursor inside the dialog changes shape as the values are edited.
- "Use _FONT 8" and "custom font" are mutually exclusive (17397-17404). With both off the built-in 16-pixel font is used. Editing the font file, picking a file, or changing the size turns the custom font check box on.
- The font file field is validated even when the custom font is off, so a missing font file blocks OK for unrelated changes.
- `IDE_Height` is written as `idewy`, which is the entered height minus `idesubwindow` (17467, 17517).

**On OK (17460-17532):** if the custom font was switched, or its file/size changed while enabled (`fonChg%`): turning it off selects `_FONT 8` or `_FONT 16` and frees the old handle; turning it on calls `_LOADFONT(IDECustomFontFile$, IDECustomFontHeight, "MONOSPACE")`. A failed load shows the message box `Custom font not found!`, keeps the old handle and stays in the dialog. At that point the globals (`IDECustomFont`, `IDECustomFontFile$`, `IDECustomFontHeight`, `idewx`, `idewy`, ...) have already been overwritten (17464-17491) and are not rolled back. Returns 1 if anything changed.

**Caller effect (5191-5201):** on return value 1 the screen is re-initialised: `WIDTH idewx, idewy + idesubwindow`, the font is re-applied, and `GOSUB redrawItAll`. No recompile.

### 3.7 `idechoosecolorsbox` — "IDE Colors" (17541-18289)

**Opened from:** Options menu `IDE C#olors...` (built 370, handled 5207-5213; `HideBracketHighlight` is called first).

**Colour model.** Ten colourable items, each a 32-bit RGB global (`GetCurrentColor~&`, 18292-18305):

| # | List label | Global | Config key (`[IDE COLOR SETTINGS n]`) | Text-mode palette slot (17728-17738) |
|---|---|---|---|---|
| 1 | Normal Text | `IDETextColor` | `TextColor` | 13 |
| 2 | Keywords | `IDEKeywordColor` | `KeywordColor` | 12 |
| 3 | Numbers | `IDENumbersColor` | `NumbersColor` | 8 |
| 4 | Strings | `IDEQuoteColor` | `QuoteColor` | 14 |
| 5 | Metacommand/custom keywords | `IDEMetaCommandColor` | `MetaCommandColor` | 10 |
| 6 | Comments | `IDECommentColor` | `CommentColor` | 11 |
| 7 | Background | `IDEBackgroundColor` | `BackgroundColor` | 1 |
| 8 | Current line background | `IDEBackgroundColor2` | `BackgroundColor2` | 6 |
| 9 | Bracket/selection highlight | `IDEBracketHighlightColor` | `HighlightColor` | previewed through slot 6 |
| 10 | Menus and dialogs | `IDEChromaColor` | `ChromaColor` | 7 |

The IDE is a 16-colour text screen whose palette entries are remapped with `_PALETTECOLOR`; a colour scheme is ten RGB values assigned to those slots. Inside this dialog slot 2 is fixed dark grey `_RGB32(84,84,84)`, slot 4 is `IDEErrorColor` (not editable here) and slot 5 is forced to green `&HFF00A800` (17574).

**Scheme storage format.** `"<name>|"` followed by 90 digits: ten colours in the order of the table above, each as `RRRGGGBBB` with zero-padded 3-digit decimals (written at 17901-17908, parsed at 18014-18035). Example (20889): `Super Dark Blue|216216216069118147...170170170`.

**Built-in schemes** (`LoadColorSchemes`, 20880-20950; `PresetColorSchemes = 14`): Super Dark Blue, Dark Blue, QB64 Original, Classic QB4.5, Dark Side, Camouflage, Plum, Cornfield, CF Dark, Broadcast, VS Code, X11 SgiColors, Light Green, All White. Scheme 1 (Super Dark Blue) is what Restore Defaults loads.

**User schemes** live in the global section `[IDE COLOR SCHEMES]` (`colorSchemesSection$`, `cfg_global.bas:70`) under keys `Scheme1$`, `Scheme2$`, ... They are appended after the presets, so user scheme *i* has `SchemeID = 14 + i`. Loading stops at the first missing key. Older formats are upgraded and rewritten on load: 81 digits (9 colours) gets `170170170` appended; 54 digits (6 colours, "Version 1.1") gets default keyword/number colours inserted and highlight/chrome colours appended (20919-20936). Anything else, including erased schemes (value `"0"`), becomes the placeholder `"0"` and is skipped during navigation (17991-18007).

**Active scheme:** `[IDE COLOR SETTINGS n] SchemeID` (0 = user-defined/unsaved; default 1, `cfg_methods.bas:570-571`), plus the ten colour keys written through `rgbs$()` (`qb64pe.bas:28333`; the exact text format was not examined). The active colours are stored per IDE instance, independent of the scheme table.

**Controls** (focus numbers as used by the code)

| Control | Type | Notes |
|---|---|---|
| `#Item:` [1] | list box, 10 rows | A `CHR$(16)` arrow marks the selected item (18096-18112). Left/Right arrow keys while the list has focus step through schemes (17969, 17978) |
| R, G, B [2-4] | three unlabeled text boxes | Digits only, at most 3 characters, clamped to 0-255 (18124-18141). Up/Down keys add/subtract 1 (18082-18094) |
| R/G/B sliders | hand-drawn 26-cell tracks | Mouse drag sets the value; holding Ctrl moves all three together (18039-18079) |
| Sample area | hand-drawn | Sample code, or a sample menu item for item 10, in the edited colours (17764-17813) |
| `#Highlight brackets` [5] | check box | `BracketHighlight`, `[GENERAL SETTINGS] BracketHighlight` |
| `#Multi-highlight (selection)` [6] | check box | `MultiHighlight`, `[GENERAL SETTINGS] MultiHighlight` |
| `Highlight #keywords and numbers` [7] | check box | `KeywordHighlight`, `[GENERAL SETTINGS] KeywordHighlight` |
| `#OK` [8] / `Restore #Defaults` [9] / `#Cancel` [10] | buttons | |
| `#Scheme` [11] | text box, width 38 | Scheme name. Pipe characters are removed as typed (18143-18149). Typing a different name sets `SchemeID = 0` (18151-18157) |
| left / right arrows | hand-drawn, mouse only | Previous / next scheme; greyed at the ends (17708-17715) |
| ` Save ` / ` Erase ` | hand-drawn, mouse only | Save is greyed for preset schemes; Erase is greyed for presets and for the unsaved scheme (17717-17724) |

**Editing rules.** The edited colour is written to its global on every loop pass (18159-18172) and the palette is re-applied, so the dialog previews live. Changing a colour while a preset is selected switches to `SchemeID = 0` named `User-defined` (`NewUserScheme`, 18272-18280). User schemes are edited in place.

- **Save** (17885-17946): an unsaved scheme goes to the first free `Scheme<i>$` key (missing, empty or `"0"`); an existing user scheme overwrites its own key unless the name was changed, in which case a new one is created. Schemes are then reloaded and the saved one becomes current.
- **Erase** (17947-17961): after `ideyesnobox("Erase color scheme", "This cannot be undone. Erase scheme?")` the key is set to `"0"`; the slot becomes reusable and other indices do not shift.
- **Restore Defaults** (18188-18196): re-enables the syntax highlighter and loads scheme 1.
- **OK / Enter** (18198-18261): re-enables the syntax highlighter if it was disabled (writes `[GENERAL SETTINGS] DisableSyntaxHighlighter = False` and re-ticks the Options menu item, 18282-18288); writes `SchemeID` and the ten colour keys; sets and writes the three highlight flags.
- **Cancel / Esc** (18174-18186): restores the ten colours from the backups taken on entry (17550-17559). Schemes saved or erased during the session stay saved or erased.

**Return value:** always 0. No recompile; the caller repaints.

### 3.8 `idergbmixer$ (editing)` — "RGB Color Mixer" (18307-18803)

**Opened from:**

| Call site | Argument | Condition |
|---|---|---|
| Tools menu `#RGB Color Mixer...` (built 438, handled 5215-5225) | `-1` | always available |
| Contextual menu `#RGB Color Mixer...` (built 19661) | same handler | only offered when the current line contains `RGB(`, `RGB32(`, `RGBA(` or `RGBA32(` |
| Shift+Enter in the editor (4015-4053) | `0` when `EnteringRGB` is set (4019-4024); `-1` when the line contains an RGB call (4030-4040) | otherwise Shift+Enter acts as Enter |

**Controls:** three unlabeled text boxes R, G, B (initial `127`), three 46-cell sliders (hint: `Hold CTRL to drag all sliders at once.`), a 10x7 colour swatch drawn in palette slot 12, and buttons `#Insert` / `C#opy` / `#Cancel`. Same 0-255 handling and Up/Down keys as the colours dialog.

**Parsing an existing colour (`editing <> 0`, 18362-18534):** uses the single-line selection if there is one, otherwise looks for `RGB` followed by `(` ... `)` on the current line and takes the occurrence nearest the cursor. Recognised prefixes after upper-casing: `RGB(`, `RGB32(`, `RGBA(`, `RGBA32(` (the leading underscore is not required because matching starts at `RGB`). Red, green and blue are read as literal digits around the first two commas. `_RGB32(intensity)` and `_RGB32(intensity, alpha)` are read as grey (`newSyntax`). Arguments that are not numeric literals produce 0 or partial numbers.

**Insert (18750-18796):** when editing an existing call, the text between its parentheses is replaced in the line with `idesetline`, keeping a trailing alpha argument; a single grey value is written in the one-argument form when the original was `RGB32`. `idechangemade = 1`, the new text is selected, and the function returns `""`. Otherwise it returns `_RGB32(r, g, b)` (or `_RGB32(v)` when all three are equal) and the caller inserts it with `insertAtCursor`. In the `EnteringRGB` case the caller inserts only the part after `(` (4048).

**Copy (18737-18748):** puts the `_RGB32(...)` text on the clipboard and returns `""`.

**Return value:** text to insert, or `""` (cancelled, copied, or already written into the line). Nothing is persisted.

## 4. Tools and Help menu dialogs

### 4.1 `ideASCIIbox$ (relaunch)` — "ASCII Chart" (20236-20512)

**Opened from:** Tools menu `#ASCII Chart...` (built 428, handled 5544-5557).

**Layout:** a 16-column grid of characters 1-255 (character 0 is not offered), each drawn as a 3-cell caption (20267-20284). A hidden text box (object 1) exists only so the chart can hold focus. Buttons: `#Insert character` / `Insert C#HR$` / `#Close`. Status lines `Selected: n` and `Hovered: n`.

**Navigation (20443-20503):** arrows move by 1 or 16 with wrap-around; Home = 1, End = 255; Ctrl+arrows jump to the row/column edge. Mouse hover highlights; click selects; double-click (two clicks on the same cell within 0.3 s) inserts.

**Insertion behaviour**

| Action | Return value | Effect at caller |
|---|---|---|
| Enter on chart, or `Insert character` | `CHR$(Selected)` — the raw character | `insertAtCursor retval$` (5549) |
| Double-click on a cell | `CHR$(Selected)` and `relaunch = -1` | Character is inserted and the dialog reopens (loop `DO ... LOOP WHILE relaunch`, 5547-5554). The reopened dialog starts again at character 1 |
| `Insert CHR$` | the text `CHR$(n)` | inserted as source text |
| Close / Esc | `""` | nothing |

For codes below 32 a one-time warning is shown (`STATIC ASCIIWarningShown`, per IDE session): `Inserting ASCII control characters (1-32) may cause unexpected IDE behavior. Consider inserting CHR$ instead. Proceed anyway?` with `#Yes;#No;#Cancel` — No closes the dialog, Cancel returns to the chart (20423-20428).

### 4.2 `ideupdatehelpbox` — "Update Help" (19959-20234)

**Opened from:** Help menu `Update All #Pages...` (built 459; also in the help-window contextual menu, 19751; handled 5773-5788) after `ideyesnobox("Update Help", "This can take up to 10 minutes.\nRedownload all cached help content from the wiki?")`. The caller sets `Help_Recaching = 1 : Help_IgnoreCache = 1` around the call. A second, headless caller is the hidden command-line switch `-u` (`qb64pe.bas:14574-14581`) with `Help_Recaching = 2`, which skips all drawing and input (19961-19966, 20011).

**UI:** a 60x6 box with two centred message lines, a 52-cell progress bar made of `CHR$(219)` / `CHR$(176)` plus a percentage, and a single `#Cancel` button that becomes `#Close` when done. Esc, Enter, the button or the `C` key asks `Cancel download?` (20110-20113). Input is polled once per loop pass without blocking, and one page is processed per pass, so the dialog stays responsive between downloads but not during one.

**Steps (state machine on `UpdateStep`, 20118-20228).** Steps 1-4 are each padded to at least 1.25 s so the messages can be read.

| Step | Message | Work |
|---|---|---|
| 1 | `Preparing help update...` / `Generating list of cached content...` | `idezfilelist$("internal/help", 2, "*.txt")` gives the list of cached pages as a `CHR$(0)`-separated string (skipped in `-u` mode) |
| 2 | `Adding core help pages to list...` | Nine fixed pages are put at the front if missing: `Metacommand`, `Keyword_Reference_-_By_usage`, `Keywords_currently_not_supported_by_QB64`, `Quick_Reference_-_Tables`, `ERROR_Codes`, `Variable_Types`, `Data_types`, `QB64_Help_Menu`, `QB64_FAQ` (20134-20151) |
| 3 | `Regenerating keyword list...` | Downloads `Keyword Reference - Alphabetical` with `Wiki$` and runs `WikiParse`, which rebuilds `internal\help\links.bin` |
| 4 | `Building download queue...` | Reads `links.bin` (lines of `keyword,PageName[#anchor]`), converts each page name to a cache file name (space to `_`; the characters `"$&*+/:<>?\|` to `%` + hex) and appends pages not yet listed. Pages starting `_gl` are skipped when `Help_Recaching >= 1`. Counts the total `c` |
| 5 | `Updating help content file n/c...` / `Page: <name>` | Pops one name per pass, calls `Wiki$(name)` (which downloads because the cache is ignored) and `WikiParse` |
| 6 | `All pages updated.` | Button becomes `#Close`; in `-u` mode the loop exits |

**Return value:** 0 = finished or cancelled by the user; 1 = a download returned `{{PageInternalError}}` (20159, 20216). On 1 the IDE caller opens the help page `Update All` (5782-5785); the `-u` caller prints a message and exits with code 1.

### 4.3 `idef1box$ (lnks$, lnks)` — "Contextual help" (20515-20639)

**Opened from:** F1 handling at `contextualhelp:` (2843-2858), also reached from the contextual menu item `#Help On '...'` (5613-5615). `findHelpTopic$` (21104-21150) looks up the word under the cursor in `internal\help\links.bin` and returns all distinct target pages. With exactly one hit the page opens directly; with more than one this dialog asks which.

**Controls:** list box `Which?` with one row per candidate page (an exact name match is listed first, 21139-21143), and a single `#OK` button. Height is `lnks + 3`.

**Return value:** the selected page name (`idetxt(o(1).stx)`), or `"C"` on Esc. A result containing `PARENTHESIS` is ignored by the caller (2860).

Related: if `links.bin` is missing or empty, `findHelpTopic$` first asks `The help system is not yet initialized, do it now? (Make sure you're online.)` and downloads the alphabetical keyword page (21111-21128).

## 5. Help view renderer: `Help_ShowText` (18936-19015)

**Called from:** the main redraw path when `idehelp` is set (1150) and once more near the end of `ide2` (6877).

**Data structures** (declared in `source\ide\wiki\wiki_global.bas:6-51`, filled by `WikiParse` / `Help_AddTxt` / `Help_NewLine` in `wiki_methods.bas:137-208`):

| Variable | Content |
|---|---|
| `Help_Txt$` | The rendered page as a flat cell buffer, 4 bytes per character cell: `[char][colour][link low byte][link high byte]`. Preallocated to 1,000,000 bytes; `Help_Txt_Len` is the used length |
| colour byte | `foreground + background * 16` for normal cells. A value above 127 marks the end-of-line cell (`char = 13`, `colour = 128 + background * 16`) |
| link bytes | 16-bit index into `Help_Link$`; 0 = not a link |
| `Help_Line$` | Packed `MKL$` values: the byte offset in `Help_Txt$` of the first cell of each line |
| `help_h`, `help_w` | Number of lines and widest line of the page |
| `Help_Link$` | Link targets separated by `CHR$(13)`; each is `PAGE:<wiki page>`, `EXTL:<url>` or `SECT:<anchor>`. Entry 1 is always `SECT:dummylink` |
| `Help_sx`, `Help_sy` | Scroll position (first visible column and line) |
| `Help_wx1/wy1/wx2/wy2`, `Help_ww`, `Help_wh` | Screen rectangle of the help text area |
| `Help_Select`, `Help_SelX1/X2/Y1/Y2` | Selection rectangle; drawn only when `IdeSystem = 3 AND Help_Select = 2` |
| `Help_LineLen()` | Output of this routine: the length of each visible line |
| `Back$()`, `Back_Name$()`, `Help_Back()`, `Help_Back_Pos` | Page history; `Back$(1)` starts as `QB64 Help Menu` |

Colour constants (`wiki_global.bas:25-29`): normal 7, link 9, bold 15, italic 3, section heading 8.

**Rendering:** for each visible row `y = Help_sy .. Help_sy + Help_wh - 1` (18950):

1. If `y <= help_h`, the start offset is read from `Help_Line$` and cells are walked 4 bytes at a time until a colour byte above 127 is met (18959-18975). Each cell is printed with `COLOR col AND 15, col \ 16`, unless it lies in the selection rectangle, where `COLOR 0, 7` is used. Cells left of `Help_sx` or right of `Help_wx2` are skipped.
2. The rest of the row is padded with spaces in the line's background colour, taken from the end-of-line cell as `(col - 128) \ 16` (18979-18990).
3. Rows past the end of the page are filled with spaces in `COLOR 7, 0` (18992-19009).

The renderer does not look at the link bytes; links are visible only because the parser stored them with the link colour. Hit-testing of links happens in the main loop.

**Lazy first load (18938-18945):** on the very first call, if the history holds only its initial entry and `IdeContextHelpSF` is false, the start page is fetched and parsed (`Wiki$(Back$(1))`, `WikiParse`).

## 6. History, recent files and bookmarks

### 6.1 `iderecentbox$` — "Open" / Recent Programs (19273-19410)

**Opened from:** File menu `#Recent...` (handled 6597-6622), which exists only when the recent file holds more entries than the menu shows (see 7.1).

**Controls:** list box `Recent Programs` (one full path per row; dialog width grows to the longest path) and buttons `#Open` / `#Cancel` / `Clear #list` / `#Remove broken links`. At most `ideMaxRecent` lines are read from `RecentFile$` (19287-19294).

| Return | Meaning | Caller action |
|---|---|---|
| path | Enter on the list, double-click, or Open | `IdeOpenFile$ = f$`, `AttemptToLoadRecent = _TRUE`, `GOTO directopen` (6615-6619) |
| `""` | Esc / Cancel | nothing |
| `"<C>"` | Clear list | `AskClearHistory$("RECENT")`; Yes truncates `RecentFile$`, No reopens the dialog (6601-6609) |
| `"<R>"` | Remove broken links | `GOSUB CleanUpRecentList` (6805, part of `ide2`, not examined) and reopen the dialog |

### 6.2 `idesearchedbox$` — search history drop-down (19019-19184)

**Opened from:** a click on the drop-down arrow at column `idewx - 3` of the Find field in the status area (`showrecentlysearchedbox:`, 2141-2152). It is a popup rather than a full dialog: no title, no buttons, positioned at `p.x = idewx - 24`, just above the search bar (19053-19060).

**Content:** `RetrieveSearchHistory` fills an array (newest first); the list string is built by prepending, so the list shows the oldest entry at the top and the newest at the bottom (19043-19045). Because `ln` is never set (19039, 19049), the height always collapses to the minimum of 3 rows; the list scrolls.

**Return:** the selected text on Enter, double-click, or any click inside the box (19163-19175); `""` on Esc or a click outside (19151-19161). The caller puts the result into `idefindtext` and runs Find Next (`GOTO idemf3`).

### 6.3 `AddToHistory (which$, entry$)` (19900-19930), `RetrieveSearchHistory` (19945-19957), `AskClearHistory$` (19934-19941)

| History | File | Limit | Entry |
|---|---|---|---|
| `"RECENT"` | `settings/recent.bin` (`RecentFile$`) | `ideMaxRecent` (5-200, default 20) | Full path, passed through `RemoveDoubleSlashes$` |
| `"SEARCH"` | `settings/searched.bin` (`SearchedFile$`) | `ideMaxSearch` (5-200, default 50) | The search string as typed |

**Format:** despite the `.bin` extension both are plain text, one entry per line, newest first. They are read and written through the text-buffer helpers `FileToBuf%`, `ReadBufLine$`, `WriteBufLine`, `DeleteBufLine`, `BufToFile` (defined outside this file; the line-ending convention was not verified). On first run after an upgrade `cfg_methods.bas:40-52` copies and converts older files.

**Algorithm (19916-19929):** walk all lines; delete any line equal to the new entry (case-insensitive) and any line at position `>= limit`; rewind and insert the new entry as line 1. This keeps the file at most `limit` entries and also trims it after the limit was lowered.

**Callers:** `"RECENT"` on file open/save (634, 12900, 12922, 21326, 21417); `"SEARCH"` on every find (1974, 2137, 2311, 5936, 5950, 7738, 11115, 11953).

`RetrieveSearchHistory shArr$()` loads up to `ideMaxSearch` lines into a 1-based array; an empty file yields one empty element. It is used by `idefind$` (11771), `idechange$` (10915) and `idesearchedbox$`.

`AskClearHistory$(which$)` shows `Clear recent files` or `Clear search history` with `This cannot be undone. Proceed?` and `#Yes;#No`; returns `"Y"` or `"N"`. The callers do the clearing themselves by opening the file `FOR OUTPUT` (6119-6124 from Search menu `Clear Search #History...`, which also empties `idefindtext`; 6603-6606; 6626-6630 from File menu `#Clear Recent...`).

### 6.4 `IdeImportBookmarks (f2$)` (19187-19228) and `IdeSaveBookmarks (f2$)` (19230-19271)

**Location:** `settings/bookmarks.bin` (`BookmarksFile$`), one global file for all source files.

**Format:** a concatenation of records with no header and no index:

```
CRLF + <full path of source file> + CRLF + MKL$(dataLength) + data
data = dataLength \ 16 entries of: MKL$(y) + MKL$(x) + MKL$(reserved) + MKL$(reserved2)
```

- Lookup is a case-insensitive `INSTR` for `CRLF + path + CRLF` over the whole file content (19191, 19233).
- Save removes the old record for that path and prepends the new one (19234-19246), so the most recently saved file is first. Records for files that no longer exist are never removed.
- Import discards bookmarks whose line is beyond `iden` (19200) and doubles `IdeBmk()` as needed.

**Breakpoints and skip lines ride along**, stored in a different file: `DebugFile$`, an INI file with one section per source path and keys `total breakpoints`, `breakpoint 1..n`, `total skips`, `skip 1..n` (key names are built with `STR$`, so there is a space before the number) (19211-19227, 19249-19270). They are written only when `GetRCStateVar(vWatchOn)` is true, that is, when the program uses `$DEBUG`.

**Callers:** import after a file is loaded (633, 12901, 21418); save on save (12923, 12972, 21327).

## 7. Menu builders

Menu items are strings in `menu$(menuIndex, itemIndex)`, with item 0 as the menu title and `menusize(m)` as the item count. Conventions: `#` precedes the hot-key letter; two spaces separate the label from the shortcut text; a leading `~` means disabled (greyed); a leading `CHR$(7)` is a check mark; `"-"` is a separator; a trailing `CHR$(16)` marks a submenu. `menuDesc$(m, i)` is the status-line description. Handlers in `ide2` match on the exact item text.

### 7.1 `IdeMakeFileMenu (eaa%)` (19414-19464)

**Called from:** menu initialisation with `0` (239), and every time a menu is about to be shown, with the current enabled state of the Export item preserved (`showmenu:`, 4649) so that the recent list is always fresh.

| # | Item | Condition |
|---|---|---|
| 1 | `#New  Ctrl+N` | always |
| 2 | `#Open...  Ctrl+O` | always |
| 3 | `#Save  Ctrl+S` | always |
| 4 | `Save #As...` | always |
| | separator | |
| 5 | `#Export As...  ►` (submenu, `FileMenuExportAs`) | Prefixed `~` (disabled) when `eaa%` is false |
| | separator | only when the recent file has at least one line |
| 6.. | `#1 <path>` ... `#6 <path>` | One per line of `recent.bin`, at most 6 (`IdeRecentLink(1 TO 6, 1 TO 2)`, `ide_global.bas:26`). Paths longer than 35 characters are shown as three dots plus the last 32 characters. `IdeRecentLink(r, 1)` = menu text, `(r, 2)` = full path. Description: `Open '<path>'` |
| | `#Recent...` | Only when a 7th line exists (19440) |
| | `#Clear Recent...` | Only when the last item added was a recent file, that is, 1-6 entries exist (19455-19458). With 7 or more entries clearing is done from the Recent dialog |
| | separator | |
| | `E#xit` | always |

**Export As activation.** The item is enabled only while the program is in a compiled-OK state. It is disabled at init, whenever a change triggers a new check (1140), and while compiling (3172, 6890); it is enabled when the status becomes `OK` (895, 3175, 6892). The submenu (built once at 493-506, opened via `idecontextualmenu = 3`, 4979-4981 and 5878) contains: `#Hypertext document (.htm)`, `#Rich Text document (.rtf)`, `#Discord codebox (to Clipboard)`, `#Forum codebox (to Clipboard)`, `#Wiki example (to Clipboard)`.

### 7.2 `IdeMakeEditMenu` (19763-19895)

**Called from:** init (243) and at every `showmenu:` (4679). `IdeSystem` is the focus: 1 = code editor, 2 = search field, 3 = help window.

| Item | Enabled when | Otherwise |
|---|---|---|
| `#Undo  Ctrl+Z`, `#Redo  Ctrl+Y` | `IdeSystem = 1` | `~` disabled |
| separator | | |
| `Cu#t  Shift+Del or Ctrl+X` | (`IdeSystem = 1` and `ideselect = 1`) or `IdeSystem = 2` | disabled |
| `#Copy  Ctrl+Ins or Ctrl+C` | same as Cut, or (`IdeSystem = 3` and `Help_Select = 2`) | disabled |
| `#Paste  Shift+Ins or Ctrl+V` | (clipboard not empty and `IdeSystem = 1`) or `IdeSystem = 2` | disabled |
| `Cl#ear  Del` | same as Cut | disabled |
| `Select #All  Ctrl+A` | always | |
| `#Duplicate Line  Ctrl+D` | present only when `IdeSystem = 1` | item absent |
| separator | | |
| `To#ggle Comment  Ctrl+T`, `Add Co#mment (')  Ctrl+R`, `Remove Comme#nt (')  Ctrl+Shift+R` | `IdeSystem = 1` | disabled |
| `#Increase Indent  TAB`, `#Decrease Indent` (with `  Shift+TAB` appended on Windows and macOS only) | `IdeSystem = 1` and a selection exists that is multi-line or a non-empty single-line span (19827-19860) | disabled |
| separator | | |
| `New #SUB...`, `New #FUNCTION...` | `IdeSystem = 1` | disabled |

There is no enabled-state check on Undo/Redo availability; they are enabled whenever the editor has focus.

### 7.3 `IdeMakeContextualMenu` (19466-19761)

**Called from:** right-click in the code area (3323; the click first moves the cursor or adjusts the selection, 3305-3321), right-click in the help area (3330, after setting `IdeSystem = 3`), and right-click while the debugger is paused (829, `IdeDebugMode = 2`). The menu is stored in the hidden slot `idecontextualmenuID` and shown at the mouse position (`idecontextualmenu = 1`, 4660-4664).

**Case A: debugger paused (`IdeDebugMode = 2`, 19473-19505)** — fixed list:

`#Continue  F5`, `Step O#ut  F6`, `Ste#p Into  F7`, `Step #Over  F8`, separator, `Set #Next Line  Ctrl+G`, `#Run To This Line  Ctrl+Shift+G`, separator, `Toggle #Breakpoint  F9`, `Clear All Breakpoints  F10`, `Toggle #Skip Line  Ctrl+P`, `#Unskip All Lines  Ctrl+F10`, separator, `SUBs...  F2`, `#Watch List...  F4`, `Call Stack...  F12`, separator, `#Exit $DEBUG mode  ESC`.

Breakpoint and skip-line toggles appear in the contextual menu only in this case; in normal editing they are in the Debug menu (333-340).

**Case B: code editor or search field (`IdeSystem = 1 OR 2`, 19507-19729)**

| Order | Item | Condition | Handler |
|---|---|---|---|
| 1 | `Find '<selection>'` | A single-line selection exists (`getSelectedText$(0)` not empty). Text is truncated to 19 characters + three dots when longer than 22. Full text kept in `idecontextualSearch$` | 5934-5939: sets `idefindtext`, adds to search history, runs Find Next |
| 2 | `#Go To SUB <name>` / `#Go To FUNCTION <name>` | The word under the cursor (trailing `$` removed) or the selection (sigil removed) equals the name of a SUB/FUNCTION found by scanning the buffer (same line-prefix scan as `idesubs$`, without the DECLARE LIBRARY handling, 19521-19561), and that procedure is not the one the cursor is currently inside (`FindCurrentSF$`, 19581-19605). The target line is stashed in `SubFuncLIST(1)` | 5618-5627: `AddQuickNavHistory`, `idecy = idesy = line`, column 1, selection cleared |
| 3 | `Go To #Label <name>` | The word under the cursor is a valid name found in the compiler's hash table with `HASHFLAG_LABEL` (`HashFind`, `Labels(r).SourceLineNumber`), the label's line is known and is not the current line; when several labels share the name, one in the current procedure scope is preferred (19609-19626). Depends on compiler data from the last check pass. The target line is stashed in the last element of `SubFuncLIST` | 5629-5638: same jump with quick-nav history |
| 4 | `#Help On '<topic>'` | The word under the cursor has at least one entry in `links.bin` (`findHelpTopic$(a2$, lnks, -1)`), and the page name does not contain `PARENTHESIS`. The label is truncated to 12 characters + three dots when longer than 15 (19629-19643). Side effect: if the help system is not initialised, a right-click triggers the "initialize now?" prompt | 5613-5616: `GOTO contextualhelp` (same as F1) |
| | separator | Only if any of items 1-4 was added |
| 5 | `#RGB Color Mixer...` + separator | The current line contains `RGB(`, `RGB32(`, `RGBA(` or `RGBA32(` and the selection, if any, is not multi-line (19650-19665) | 5215 |
| 6 | `Cu#t  Shift+Del or Ctrl+X`, `#Copy  Ctrl+Ins or Ctrl+C` | `ideselect <> 0` | |
| 7 | `#Paste  Shift+Ins or Ctrl+V` | Clipboard is not empty | |
| 8 | `Cl#ear  Del` | `ideselect` | |
| 9 | `Select #All  Ctrl+A` | always | |
| | separator | always | |
| 10 | `To#ggle Comment  Ctrl+T`, `Add Co#mment (')  Ctrl+R`, `Remove Comme#nt (')  Ctrl+Shift+R` | always | |
| 11 | `#Increase Indent  TAB`, `#Decrease Indent` (+ `  Shift+TAB` on Windows/macOS), separator | A selection exists that is multi-line or a non-empty single-line span. With no selection only a separator is added. With an empty single-line selection neither the items nor a separator is added (19694-19725) | |
| 12 | `New #SUB...`, `New #FUNCTION...` | always | 5790-5803 |

Unlike the Edit menu, unavailable items are omitted here rather than greyed. Undo/Redo and Duplicate Line are not in the contextual menu.

**Case C: help window (`IdeSystem = 3`, 19730-19757)**

`#Copy  Ctrl+Ins or Ctrl+C` (only when `Help_Select = 2`), `Select #All  Ctrl+A`, separator, `#Contents Page`, `Keywords #Index`, `#Keywords by Usage`, `#Metacommands`, `Variable #Types`, separator, `#Update Current Page`, `Update All #Pages...`, `View Current Page On #Wiki`, separator, `Clo#se Help  ESC`.

## 8. Small helpers in range

| Routine | Lines | Purpose |
|---|---|---|
| `GetCurrentColor~& (Selection)` | 18292-18305 | Maps colour item 1-10 to its `IDE...Color` global (see 3.7) |
| `sort (arr() AS STRING * 998)` | 20642-20653 | Insertion sort of fixed-length strings, ascending; used only by `idesubs$`. Quadratic time |

## 9. Consolidated table

| Dialog title | Routine | Lines | Opened from (item; build line / call line) | Settings touched |
|---|---|---|---|---|
| SUBs | `idesubs$` | 13701 | View `#SUBs...  F2` (247 / 5812); F2 (3542, 2337); debugger (7718) | `IDESortSubs`, `IDESubsLength`: `[IDE DISPLAY SETTINGS] IDE_SortSUBs`, `IDE_SUBsLength`. Moves cursor; adds quick-nav entry |
| QBJS Web Build | `ideQBJSBuildBox` | 14260 | Run `#QBJS Web Build...` (324 / 5379) | `[QBJS] Port`, `CompileOnly`, `CopyProjectFiles`; may set `idecy` |
| Language Settings | `ideLanguageBox` | 14586 | Options `#Language...` (377 / 5182) | `idecpindex`: `[IDE DISPLAY SETTINGS] IDE_CodePage`; applies `_MAPUNICODE` |
| Compilation status | `idewarningbox` | 14737 | View `Compiler #Warnings...  Ctrl+W` (253 / 5920); Ctrl+W (3548); status link (1689) | None persisted. Sets `idecy`, `idegotobox_LastLineNum`, `warningInInclude`, `warningInIncludeLine` |
| Code Layout | `ideLayoutBox` | 15773 | Options `#Code Layout...` (372 / 5435) | `IDEAutoIndent`, `IDEAutoIndentSize`, `IDEIndentSubs`, `IDEAutoLayout`, `IDEAutoLayoutKwStyle`: `IDE_AutoIndent`, `IDE_IndentSize`, `IDE_IndentSUBs`, `IDE_AutoFormat`, `IDE_KeywordCapital`, `IDE_KeywordLowercase`. Triggers re-check and re-layout |
| Backup/Undo & History Limits | `ideLimitsBox` | 16027 | Options `#Undo/History...` (379 / 5525) | `idebackupsize`, `ideMaxRecent`, `ideMaxSearch`: `[GENERAL SETTINGS] BackupSize`, `MaxRecentFiles`, `MaxSearchStrings`. May wipe the undo file |
| Go To Line | `idegotobox` | 16262 | Search `#Go To Line...  Ctrl+G` (283 / 5518); keys (3480, 1553) | None persisted. `idegotobox_LastLineNum`, cursor, quick-nav |
| Base TCP/IP Port Number | `ideSetTCPPortBox` | 16277 | Debug `Set Base #TCP/IP Port Number...` (360 / 6483) | `idebaseTcpPort`: `[DEBUG SETTINGS] BaseTCPPort` |
| (caller-supplied: Run To Line, Set Next Line, Skip Line) | `idegetlinenumberbox` | 16287 | Debugger (8124, 8139, 8151) | None |
| Compiler Settings | `ideCompilerSettingsBox` | 16300 | Options `Co#mpiler Settings...` (374 / 5443) | `[COMPILER SETTINGS] OptimizeCppProgram`, `StripDebugSymbols`, `IncludeDebugInfo`, `AbsoluteDebugPaths`, `ExtraCppFlags`, `ExtraLinkerFlags`, `MaxParallelProcesses`, `UseSystemMinGW` (Windows). Purges build files; forces recompile |
| Logging Configuration | `ideLoggingBox` | 16541 | Run `Configure #Logging...` (321 / 5385) | `[LOGGING SETTINGS] LogMinLevel`, `LogScopes`, `LogHandlers`, `LogFileName`; `LoggingEnabled`, `LogToConsole` |
| Default Terminal | `ideTerminalBox` | 16826 | Run `Change #Terminal...` (316, Linux only / 5371) | `DefaultTerminal$`: `[GENERAL SETTINGS] DefaultTerminal` |
| Display Settings | `ideDisplayBox` | 17127 | Options `#Display...` (368 / 5190) | `[IDE WINDOW n] IDE_Width`, `IDE_Height`; `[IDE DISPLAY SETTINGS] IDE_AutoPosition`, `IDE_NormalCursorStart`, `IDE_NormalCursorEnd`, `IDE_UseFont8`, `IDE_CustomFont`, `IDE_CustomFont$`, `IDE_CustomFontSize`. Screen re-init |
| IDE Colors | `idechoosecolorsbox` | 17541 | Options `IDE C#olors...` (370 / 5210) | `[IDE COLOR SETTINGS n] SchemeID` + 10 colour keys; `[IDE COLOR SCHEMES] Scheme<i>$`; `[GENERAL SETTINGS] BracketHighlight`, `MultiHighlight`, `KeywordHighlight`, `DisableSyntaxHighlighter` |
| RGB Color Mixer | `idergbmixer$` | 18307 | Tools `#RGB Color Mixer...` (438 / 5221); contextual menu (19661); Shift+Enter (4024, 4040) | None. Edits the current line or returns text; clipboard on Copy |
| (untitled popup, list "Find") | `idesearchedbox$` | 19019 | Search-bar drop-down arrow (2145) | None; reads `searched.bin` |
| Open (Recent Programs) | `iderecentbox$` | 19273 | File `#Recent...` (19440 / 6600) | None directly; caller may truncate or clean `recent.bin` |
| Clear recent files / Clear search history | `AskClearHistory$` | 19934 | File `#Clear Recent...` (6626), Recent dialog (6603), Search `Clear Search #History...` (266 / 6119) | Caller truncates `recent.bin` or `searched.bin` |
| Update Help | `ideupdatehelpbox` | 19959 | Help `Update All #Pages...` (459, 19751 / 5779); `qb64pe -u` (`qb64pe.bas:14576`) | Rewrites `internal/help/*.txt` cache and `links.bin` (through `Wiki$` / `WikiParse`) |
| ASCII Chart | `ideASCIIbox$` | 20236 | Tools `#ASCII Chart...` (428 / 5548) | None; returns text to insert |
| Contextual help | `idef1box$` | 20515 | F1 / `#Help On '...'` with several matches (2856) | None |

Non-dialog routines in range: `Help_ShowText` (18936), `IdeImportBookmarks` (19187), `IdeSaveBookmarks` (19230), `IdeMakeFileMenu` (19414), `IdeMakeContextualMenu` (19466), `IdeMakeEditMenu` (19763), `AddToHistory` (19900), `RetrieveSearchHistory` (19945), `GetCurrentColor~&` (18292), `sort` (20642).

## 10. Observations relevant to a rewrite

- Roughly 100 lines of identical event-loop code are repeated in each of about 20 dialogs; only the control list, the per-pass validation and the OK handler differ. A declarative dialog description plus one runner would cover all of them. The exceptions that need custom drawing or input are the colour dialogs (sliders, hand-drawn buttons, live palette preview), the ASCII chart (grid), the QBJS and Update Help dialogs (background work between input polls) and the search-history popup (click-outside-to-close).
- Menu handlers dispatch on the literal item text, including hot-key markers and shortcut text. Menu builders and handlers must therefore be changed together.
- Two independent SUB/FUNCTION scanners exist (`idesubs$` and `IdeMakeContextualMenu`), both line-prefix based and independent of the compiler's symbol data, while label lookup in the contextual menu does use compiler data.
- Settings are applied by comparing each control with its global and writing every key of the dialog when any one changed; there is no settings object.
- Several dialogs persist on paths other than OK: `idesubs$` saves its two check boxes on Cancel; the QBJS dialog saves on Close; the colour dialog saves and erases schemes immediately.

## 11. Not determined / not verified

- The exact text format written by `rgbs$()` for the ten colour keys (`qb64pe.bas:28333` was not read).
- Line-ending convention and exact behaviour of the buffer helpers (`FileToBuf%`, `ReadBufLine$`, `WriteBufLine`, `DeleteBufLine`, `BufToFile`) used for `recent.bin` and `searched.bin`; they are defined outside `ide_methods.bas`.
- `CleanUpRecentList` (6805, inside `ide2`) — what "Remove broken links" does in detail.
- The path and format details of `DebugFile$` beyond the key names used here.
- Defaults for the `[QBJS]` keys are taken from the dialog code; defaults for `IDE_AutoPosition`, the Code Layout keys other than indent size (4), the compiler check boxes, and the logging keys were not looked up in `cfg_methods.bas`.
- The code page tables `idecp()` / `idecpname()` and the list of available code pages.
- The second key path that calls `idegotobox` at line 1553 and the debugger call of `idesubs` at 7718 were not read in context.
- How the main loop hit-tests help links and uses `Help_LineLen()`; only the renderer was examined.
- `Wiki$` / `WikiParse` internals (download method, cache file layout) — owned by another section.
- Behaviour was derived from reading the source only; nothing was run.

---

# Part E — Help/wiki, config, export, converters, support files, command line

*(Section numbers below are local to Part E.)*

Scope: read-only study of `..\QB64pe`. All paths are relative to that repo root. `file:line` references were checked against the file unless marked "(not verified)". Abbreviation: `ide_methods.bas` = `source\ide\ide_methods.bas`.

---

## 1. Help system / Wiki

### 1.1 Files and moving parts

| File | Role |
|---|---|
| `source\ide\wiki\wiki_global.bas` (156 lines) | All help globals, colour constants, back-history arrays, entity table, UTF-8 to CP437 table. Executed as main-module init code. |
| `source\ide\wiki\wiki_methods.bas` (1367 lines) | `Back2BackName$`, `Wiki$` (cache/download), `Help_AddTxt`/`Help_NewLine`/... (output buffer writers), `WikiParse` (markup renderer + links.bin generator), `wikiSafeName$`, `wikiDLPage$` (HTTP), `wikiLookAhead$`, `wikiBuildCIndent$`. |
| `source\ide\ide_methods.bas` | Help window UI: input handling 2382-2839, F1 contextual help 2843-3020, menu handlers 5640-5696 and 5773-5788, back-link bar 6911-6953, `Help_ShowText` 18936-19015, `ideupdatehelpbox` 19959-20234, `idef1box$` 20515, `getWordAtCursor$` 20990, `findHelpTopic$` 21104-21150. |
| `internal\help\` | Cache folder (page files `*.txt`, `links.bin`, transient `curlResponse.txt`). |

### 1.2 Wiki URL and page fetch

- Base address default: `https://qb64phoenix.com/qb64wiki` (`source\ide\config\cfg_methods.bas:284`), overridable by `[GENERAL SETTINGS] WikiBaseAddress` (cfg_methods.bas:285-289). Global: `wikiBaseAddress$` (`cfg_global.bas:31`).
- Fetch URL: `wikiBaseAddress$ + "/index.php?title=" + PageName2$ + "&qbide=1&action=edit"` (wiki_methods.bas:73). I.e. the IDE downloads the MediaWiki **edit page HTML** and scrapes the raw wikitext out of the `<textarea name="wpTextbox1">` ... `</textarea>` element (delimiters at wiki_methods.bas:76-77, extraction 88-90). `&qbide=1` is a marker so the server can distinguish IDE requests (comment 69-72).
- Transport (`wikiDLPage$`, wiki_methods.bas:1155-1229):
  1. Rate limit: at least 0.5 s between calls (STATIC `lastCall#`, 1161-1163).
  2. Primary: built-in HTTP client `_OPENCLIENT(url$)` (1167), 3 attempts with 3.33 s delay (1166-1169). Requires `_STATUSCODE = 200` (1186); reads with `GET ch&, , rec$` in a 0.05 s polling loop until `EOF` or inactivity timeout (15 s passed by `Wiki$`, line 83; loop 1188-1197, with midnight wrap fix 1196).
  3. If the connect fails and the URL is https and no external `curl` is on PATH (`_SHELLHIDE("curl --version >NUL")`), a native `_MESSAGEBOX` offers falling back to `http://`, optionally persisting it by rewriting `WikiBaseAddress` in config (1170-1183).
  4. Last chance: external `curl --silent --retry 3 -o "internal/help/curlResponse.txt" "<url>"` via `SHELL _HIDE`, then `_READFILE$` and `KILL` the response file (1215-1228). Skipped for `Help_Recaching = 2` (`qb64pe -u`).
- A response containing "Login required" is treated as an empty page (wiki_methods.bas:84).
- Dummy page names `Initialize` and `Update All` never download; they exist to show the error page (80-81; set at ide_methods.bas:21124 and 5783).

### 1.3 Cache location, file naming, invalidation

- Folder: `Cache_Folder$ = "internal\help"` (Windows) / `"internal/help"` (wiki_global.bas:1-3); created if missing (line 5). If `internal` itself is missing execution jumps to `NoInternalFolder` (line 4).
- File name derivation (`Wiki$`, wiki_methods.bas:30-43):
  1. space -> `_`; chars `" $ & * + / : < > ? \ |` (ASCII 34,36,38,42,43,47,58,60,62,63,92,124) -> `%` + `HEX$(c)` (so `$` -> `%24`). Result is `PageName2$`, also used verbatim as the `title=` URL parameter.
  2. `wikiSafeName$` (1142-1153) appends `_` plus a "spelling label": one char per source char, `1` for upper-case letter, `0` for lower-case letter, other chars copied as-is. This makes names unique on case-insensitive file systems (e.g. `Dim` vs `DIM`).
  3. File = `internal/help/<PageName2>_<label>.txt`.
- Cache hit: if `Help_IgnoreCache = 0` and the file exists, return its content with CRLF normalised to LF (46-52). No expiry/TTL exists: cached pages live forever until explicitly updated.
- On download the wikitext is post-processed and stored (92-112): `&amp;` (looped for multi-escapes), `&lt;`, `&gt;`, `&quot;` decoded; `#REDIRECT` replaced with `See page`; CRLF->LF; leading blank lines stripped; trailing LF ensured; and two pseudo-templates are **prepended**: `{{QBDLDATE:mm-dd-yyyy}}` and `{{QBDLTIME:hh:mm:ss}}` (line 107). These are later read by `WikiParse` for the "Last updated" header.
- Empty wikitext: if a cached file already exists it is `KILL`ed (page was removed from the wiki); otherwise a synthetic `{{PageInternalError}}` page "not yet available" is returned, not cached (113-124). Download failure (delimiters not found) returns another synthetic `{{PageInternalError}}` page (125-132), not cached.
- `Help_PageLoaded$` is set to the page name for all non-`Template:` pages (line 28).
- **Update Current Page** (Help menu, ide_methods.bas:5687-5696): sets `Help_IgnoreCache = 1`, re-calls `Wiki$(Back$(Help_Back_Pos))` and `WikiParse`, resets flag. Only active when the help window is open.
- **Update All Pages...** (ide_methods.bas:5773-5788): yes/no box ("can take up to 10 minutes"), then `Help_Recaching = 1: Help_IgnoreCache = 1`, runs `ideupdatehelpbox` (19959-20234), a 6-step state machine with a progress dialog:
  1. list existing `internal/help/*.txt` (`idezfilelist$` mode 2, which strips the spelling label, ide_methods.bas:15578/15621);
  2. prepend the core pages: `QB64_FAQ`, `QB64_Help_Menu`, `Data_types`, `Variable_Types`, `ERROR_Codes`, `Quick_Reference_-_Tables`, `Keywords_currently_not_supported_by_QB64`, `Keyword_Reference_-_By_usage`, `Metacommand` (20134-20151);
  3. download + parse `Keyword Reference - Alphabetical`, which regenerates `links.bin` (20158-20160);
  4. add every page referenced in `links.bin` (page part after the comma, without `#anchor`; `_gl*` pages skipped when recaching) (20167-20193);
  5. re-download each listed page and `WikiParse` it (so `Template:` plugins get fetched too) (20207-20221);
  6. done / "Close".
  Any `{{PageInternalError}}` aborts with return 1, after which the IDE opens the dummy page `Update All` to show the error (5782-5785). Each of steps 1-4 is padded to >= 1.25 s for visual feedback.
- **`qb64pe -u`** (hidden CI switch, qb64pe.bas:14574-14575): `Help_Recaching = 2`, same routine without GUI and without a directory scan, progress printed to console (wiki_methods.bas:64-67); no curl fallback and no message boxes. Used to pre-populate `internal/help` for release builds. NOTE: with `Help_IgnoreCache = 0` here, already-cached pages are not re-fetched.
- **Offline behaviour**: cached pages work fully offline. Uncached pages produce the internal error page. If `links.bin` is missing/empty, F1 asks "The help system is not yet initialized, do it now? (Make sure you're online.)" (ide_methods.bas:21111-21128).

### 1.4 Global variables (wiki_global.bas)

| Variable | Line | Meaning |
|---|---|---|
| `Cache_Folder$` | 1 | cache directory |
| `Help_sx, Help_sy` | 6 | scroll origin (1-based column/line of top-left visible cell) |
| `Help_cx, Help_cy` | 6 | cursor column/line in the rendered page |
| `Help_Select` | 7 | 0 none, 1 anchor set, 2 active selection |
| `Help_cx1, Help_cy1` | 7 | selection anchor |
| `Help_SelX1/X2/Y1/Y2` | 7 | normalised selection rectangle (multi-line selections are whole lines: ide_methods.bas:2818-2835) |
| `Help_MSelect` | 8 | selection started by mouse |
| `Help_wx1, Help_wy1, Help_wx2, Help_wy2` | 10 | screen rectangle of help text area |
| `Help_ww, Help_wh` | 11 | width/height of text area; `Help_ww` is the wrap width used by the parser |
| `help_h, help_w` | 12 | rendered page height (lines) and max width |
| `Help_Txt$`, `Help_Txt_Len` | 13-14 | output cell buffer and used length |
| `Help_Pos`, `Help_Wrap_Pos` | 15 | current output column; buffer offset of last space (wrap point) |
| `Help_Line$` | 16 | line index (packed LONGs) |
| `Help_Link$`, `Help_Link_Sep$` (=CHR$(13)) | 17-18 | link table |
| `Help_LinkN`, `Help_LinkL` | 19 | link count; "local link pending" flag |
| `Help_BG_Col` | 24 | current background colour (0 normal, 1 code, 2 output, 6 text/fixed) |
| `Help_Col_Normal=7, _Link=9, _Bold=15, _Italic=3, _Section=8` | 25-29 | palette indexes |
| `Help_Bold, Help_Italic, Help_LinkTxt, Help_Heading` | 30 | style state |
| `Help_Underline, Help_ChkBlank` | 31 | pending heading underline (1 single, 2 double); pending "ensure blank line" |
| `Help_LockWrap, Help_LockParse` | 32 | wrap lock; parser lock (2 code, 1 output, -1 text, -2 pre/fixed) |
| `Help_DList, Help_LIndent$` | 33 | definition list state; indent string for continuation lines |
| `Help_Center, Help_CIndent$` | 34 | centering active; per-line centre indents (one byte per upcoming line) |
| `Help_LineLen()` | 35 | visible-line lengths (filled by `Help_ShowText`) |
| `Back$()`, `Back_Name$()`, `Help_Back()` (TYPE `Help_Back_Type` sx,sy,cx,cy), `Help_Back_Pos` | 36-49 | history: page names, abbreviated tab names, saved view positions, current index. Initialised with entry 1 = `QB64 Help Menu`. |
| `Help_Search_Time#`, `Help_Search_Str$` | 50-51 | type-to-search state |
| `Help_PageLoaded$` | 52 | name of the current page |
| `Help_Recaching`, `Help_IgnoreCache` | 53-59 | cache mode flags (see 1.3) |
| `wpEntRepl()`, `wpEntReplCnt` | 63-77 | entity table |
| `wpUtfRepl()`, `wpUtfReplCnt` | 80-155 | UTF-8 table |

Related globals elsewhere: `idehelp` (help window open), `idesubwindow` (height of the bottom sub-window), `IdeSystem = 3` (help has focus), `IdeContextHelpSF` (`ide_global.bas:11`, current page is an auto-generated SUB/FUNCTION page), `Back_Str$`, `Back_Str_I$`, `Back_Str_Pos` (rendered history bar, ide_methods.bas:6911-6953), `wikiBaseAddress$`.

### 1.5 Output buffer format (exact)

- `Help_Txt$` is pre-allocated as `SPACE$(1000000)` per parse (wiki_methods.bas:267) and trimmed at the end (1030). Hard limit: 250,000 cells per page; no overflow check exists.
- Each **cell is 4 bytes** (written at 149-152 / 176-179):
  - byte 0: CP437 character code;
  - byte 1: attribute = `fg + bg*16` (fg 0-15, bg 0-7); 
  - bytes 2-3: 16-bit little-endian link number (0 = no link; n = n-th entry in `Help_Link$`).
- **End-of-line cell** (`Help_NewLine`, 188-191): char 13, attribute `128 + bg*16` (bit 7 marks EOL and also carries the background used to fill the rest of the row, see `Help_ShowText` 18980), link 0. All scanners loop `DO UNTIL ASC(Help_Txt$, x + 1) > 127`.
- `Help_Line$`: concatenated `MKL$` values; entry n (bytes `(n-1)*4+1..`) = 1-based byte offset in `Help_Txt$` of the first cell of line n. Seeded with `MKL$(1)` (271); each newline appends `MKL$(Help_Txt_Len + 1)` (194). `help_h` counts lines.
- `Help_Link$`: `CHR$(13)`-terminated records, seeded with `"SECT:dummylink" + CHR$(13)` as link #1 (268). Types: `PAGE:<wiki page>[#anchor]`, `EXTL:<url>`, `SECT:` (marker only). Link number n is found by scanning for the (n-1)th separator (ide_methods.bas:2719-2724). Link #1 is special: text tagged with link=1 is a section heading/anchor (headings are emitted with link `hl`=1, wiki_methods.bas:984 with `hl` set at 785-798; template headings at 640; page-top anchor at 337), which makes headings findable by the link-only search used for `#anchor` jumps.
- Word wrap (`Help_AddTxt`, 137-183): only when not wrap-locked and parser lock is 0 or -1. A space at the right margin becomes a newline; otherwise overflow backtracks to `Help_Wrap_Pos`, emits a newline (which re-applies indent) and re-appends the partial word.
- `Help_NewLine` also emits pending heading underline rows (char 205 double / 196 single in section colour, 197-207) and applies centre indent or `Help_LIndent$` (colour 11) to the new line (210-218).
- Helpers: `Help_CheckFinishLine` (221), `Help_CheckBlankLine` (227), `Help_CheckRemoveBlankLine` (235), `Help_Col` (251: precedence link > heading > bold > italic > normal; inside blocks link colour is the italic colour).
- Rendering: `Help_ShowText` (ide_methods.bas:18936-19015) prints cells with `COLOR col AND 15, col \ 16`, selection shown as `COLOR 0,7`, horizontal scroll via `Help_sx`. On first display it lazily loads `Back$(1)` (18939-18945).

### 1.6 WikiParse: every construct handled

`SUB WikiParse (a$)` wiki_methods.bas:263-1140. A single-pass character loop with a 20-character look-ahead array `c$(1..20)` (341-364). Line-start state `nl`; parser lock `Help_LockParse`.

**Page header** (315-338): a boxed title (from `{{DISPLAYTITLE:...}}` if present, else `Help_PageLoaded$`) plus right-aligned status: "Last updated: <date>, at <time>" from QBDLDATE/QBDLTIME, "Page not yet updated, expect visual glitches." when missing, "Page not found." for internal error pages, "Auto-generated temporary page." when the title starts with `agp@`. The first three cells of the third header row carry link=1 and are the `#toc`/`#top` anchor (337).

| Construct | Lines | Behaviour |
|---|---|---|
| `__NOEDITSECTION__`, `__NOTOC__`, `__TOC__` (with/without trailing LF) | 367-372 | dropped |
| `<nowiki>`, `</nowiki>` | 373-374 | tags dropped (content is NOT protected from parsing) |
| `<gallery ...>...</gallery>` | 375-409 | only galleries with both `48px` and `nolines` attributes (the "availability" icon galleries) are converted to a bullet line: `File:Qb64.png`, `Qbpe.png`, `Win.png`, `Lnx.png`, `Osx.png` become bold labels QB64 / QB64-PE / Windows / Linux / macOS, `File:Apix.png` removed, `none`/`all` -> "no versions"/"all versions". Other galleries are skipped entirely. |
| `<center>...</center>` | 414-429 | centred lines (`wikiBuildCIndent$` precomputes one indent per wrapped line, 1241-1366) |
| `<p style="...center...">...</p>` | 430-453 | centred paragraph if style contains "center" |
| `<!-- ... -->` | 455-463 | skipped |
| `'''bold'''` / `''italic''` | 468-483 | toggles; not in code/output blocks |
| `[http... text]` | 489-514 | external link; URL ends at first space; `EXTL:` record |
| `[[page]]`, `[[page|text]]`, `[[page#anchor]]`, `[[#anchor]]` | 516-564 | `PAGE:` record. `#toc`/`#top` redirect to page head anchor (542-543); bare `#x` gets current page prefixed (544); `Category:` links are dropped (546-549); `Wikipedia:x` becomes `EXTL:https://en.wikipedia.org/wiki/x` (552-553). Per-character colours are preserved in `lcol$`. |
| `{{Cb|x}}`, `{{Cl|x}}`, `{{Cm|x}}` | 593-598 | treated as internal links (work even in code blocks; closing `}}` accepted at 526) |
| `{| ... |}` tables | 570-588 | NOT rendered. Replaced by a centred clickable box "The original page has a table here, please click inside this box to load the page into your standard browser." linked to `EXTL:<wiki>/index.php?title=<page>`. TOC-only tables are silently dropped; nothing emitted inside blocks. |
| `{{PageSyntax}}`, `{{PageParameters}}`, `{{PageDescription}}`, `{{PageAvailability}}`, `{{PageExamples}}`, `{{PageSeeAlso}}` | 629-634 | section headings "Syntax:", "Parameters:", "Description:", "Availability:", "Examples:", "See also:" in section colour with double underline, tagged link=1 |
| `{{PageInternalError}}` | 636 | heading "Sorry, an error occurred:" (IDE-internal) |
| `{{InlineCode}}` / `{{InlineCodeEnd}}` | 645-652 | bg 1, hard lock 2 |
| `{{CodeStart}}` / `{{CodeEnd}}` | 653-665 | framed "Code Block" (bg 1, lock 2) |
| `{{OutputStart}}` (also `OutputStartBGn`) / `{{OutputEnd}}` | 667-681 | framed "Output Block" (bg 2, lock 1); footer "This block does not reflect the actual output colors" |
| `{{TextStart}}` / `{{TextEnd}}` | 683-695 | "Text Block" (bg 6, soft lock -1, still wraps) |
| `{{PreStart}}` / `{{PreEnd}}` | 697-709 | 2-space indented preformatted (lock -2, no frame) |
| `{{FixedStart}}` / `{{FixedEnd}}` | 711-723 | "Fixed Block" (bg 6, lock -2) |
| `{{PageNavigation}}`, `{{PageReferences}}`, `{{*Plugin}}` | 726-732 | template transclusion: downloads/caches `Template:<name>` via `Wiki$` and splices its text into the input stream |
| `{{Parameter|x}}` | 611-612, 735-737 | italic text |
| `{{Small|x}}` | 613-621, 740-768 | centred line; directly after a Code/Text/Fixed block end it is merged into the block's closing ruler as a caption |
| any other `{{Name|p1|p2..}}` | 599-610, 773-775 | name swallowed; first parameter emitted as text; further parameters discarded (607-609). `{{DISPLAYTITLE:..}}`, `{{QBDLDATE:..}}`, `{{QBDLTIME:..}}` have no `|` so they vanish. |
| `==h2==`, `===h3===`, `====h4====` (line start, optional inner space) | 780-800 | section colour; h2 double underline, h3 single, h4 none; not inside any block |
| `----`, `<hr>`, `<hr />` | 804-821 | full-width ruler (char 196, colour 14) |
| `;def:desc`, `:desc`, `;* def:desc`, `:* desc` | 825-864 | definition lists with indent marker and bold term |
| `*`, `**`, `#`, `##` at line start | 868-884 | bullet lists (CHR$(4) diamond, colour 14); ordered lists are rendered as bullets (no numbering) |
| HTML entities | 888-907 | table (wiki_global.bas:70-77): `&apos;` `&#91;` `&#93;` `&#123;` `&#125;` `&pi;` `&theta;` `&nbsp;`. Unknown `&xxx;` is printed raw in colour 8 and sets flag `ue`. Not processed in code/output blocks. |
| UTF-8 sequences (2/3/4 byte) | 909-936 | looked up in `wpUtfRepl` (wiki_global.bas:90-155: pilcrow, section, (c), accented vowels, cent, fractions, pi, nbsp, ellipses, euro, box-drawing, triangles, arrows, faces, card suits, bullets, block, etc. and two emoji -> `:)` `;)`). Unknown sequences become CHR$(168) (inverted ?) in colour 8 and set flag `uu`. Always processed, even in code blocks. |
| LF, `<br>`, `<br />` | 938-974 | newline; outside fixed/pre blocks at most one blank line is kept; styles reset at EOL inside blocks; list indent reset |

Post-pass: if unknown entities/UTF-8 were seen, warning lines ("Page uses unknown UTF-8 characters / HTML entities, please report it in the Wiki Forum." linking to `https://qb64phoenix.com/forum/forumdisplay.php?fid=25`) are generated at the end and then **moved to just under the header** by rewriting `Help_Txt$` and `Help_Line$` in place (994-1028).

Not supported: real table rendering, images, `<ref>`, nested templates in general, numbered lists, `<pre>`/`<code>` HTML tags (the wiki uses the templates above), `<nowiki>` protection.

### 1.7 Keyword to page mapping (`links.bin`) and contextual F1

- Whenever `WikiParse` renders the page `Keyword Reference - Alphabetical`, it rewrites `internal\help\links.bin` (wiki_methods.bas:1032-1139). For every rendered line whose first cell is the bullet CHR$(4) and whose link starts at column 3 (1052-1053), it writes text lines `KEYWORD,Page name`:
  - keyword = link text cut at the first `(` or space; `A...B` forms are split into two entries (1068-1086);
  - the same derivation is applied to the page name itself, written if different (1099-1126);
  - entries containing lower-case letters or commas are skipped unless they start with `_GL` (1075-1081 etc.).
  - Format: plain text, one `keyword,pagename` per line (written with `PRINT #`), despite the `.bin` extension. Hard-coded backslash path `internal\help\links.bin`.
- `findHelpTopic$(topic$, lnks, firstOnly)` (ide_methods.bas:21104-21150): case-insensitive linear scan of `links.bin`; returns CHR$(0)-separated page list, exact-name match placed first; offers initialisation if the file is missing/empty.
- F1 flow (ide_methods.bas:2843-3020): `getWordAtCursor$` (20990-21029; words include `$`; symbol runs; special cases `~`, backtick, `%&`) -> `findHelpTopic$` -> if several pages match, `idef1box$` "Contextual help" picker (20515) -> `OpenHelpLink` (2862): handles `#anchor`, pushes history, `Wiki$` + `WikiParse`, opens the help sub-window at half the IDE height (2913-2914). Results containing "PARENTHESIS" are ignored (2860).
- If no keyword matches, the IDE scans the program for a `SUB`/`FUNCTION` with that name and synthesises a wiki page in memory (`{{DISPLAYTITLE:agp@name}}`, `{{PageSyntax}}`, the signature, See also) (2933-3015); `IdeContextHelpSF = _TRUE`.
- The right-click contextual menu adds "Help On '<page>'" using `findHelpTopic$(.., -1)` (19629-19642; handler at 5613).

### 1.8 Navigation, history, search, selection

- History: `Back$()`/`Back_Name$()`/`Help_Back()` grow with `REDIM _PRESERVE`; a new page is **inserted after the current position** (entries after it are shifted up, not discarded) (2764-2792, 2875-2906). If the next entry is already the target it is reused. Backspace goes back one (2690-2706). The history is drawn as a row of tab names on the help window's top border, centred on the current one, and each is clickable (6911-6950, click handling 2420-2444). `Back2BackName$` (wiki_methods.bas:1-25) abbreviates long titles. History is not persisted.
- Link activation: Enter or click on a cell whose link word is non-zero (2708-2810). `EXTL:` -> `start` / `open` / `xdg-open` with minimal escaping (2729-2748); `PAGE:` -> load page; with `#anchor` the anchor (underscores -> spaces) is put in `Help_Search_Str`, `Help_LinkL` set, and the search routine jumps to the matching heading then scrolls it to 3 lines from the top (2751-2755, 2636-2640).
- "View on Wiki" button on the border and menu item: opens `wikiBaseAddress$/index.php?title=<page>` in the browser (2398-2417).
- Help search ("type to search", 2551-2635): typing alphanumerics, `$` or space while help has focus accumulates `Help_Search_Str` (reset after 1 s pause); the search only matches **text that is part of a link** (non-link cells are replaced by CHR$(0), 2581), case-insensitive, also trying a `_` prefix; on the keyword index pages only links starting within the first 3 columns count (2610-2615). Tab = next match (2547-2549). The matched link is reverse-selected. Status bar shows `[text] (TAB=next)` (1159-1172). There is no full-text search across pages.
- Selection/copy: Shift+arrows or mouse drag; Ctrl+A select all (2465-2477); Ctrl+C / Ctrl+Ins copies plain text (2479-2508). Multi-line selections are whole lines.
- **Code examples are not directly "insertable"**: there is no "insert example into editor" command. The only path is select + copy from the help window and paste into the editor. Inside code blocks only `{{Cb|}}`/`{{Cl|}}`/`{{Cm|}}` keyword links are clickable (they navigate to the keyword page).
- Keys: arrows, PgUp/PgDn, Home/End, Ctrl+Home/End, mouse wheel (3 lines), scrollbars (1997-2080), ESC or the `x` on the border closes (2385-2393), Shift+F1 reopens (5666-5679).
- Help menu items (ide_methods.bas:443-465): View (Shift+F1), Contents Page (`QB64 Help Menu`), Keywords Index (`Keyword Reference - Alphabetical`), Keywords by Usage (`Keyword Reference - By usage`), Metacommands (`Metacommand`), Variable Types (`Variable Types`), Update Current Page, Update All Pages..., View Current Page On Wiki, About. Handlers 5640-5696, 5773-5788. The help window's right-click menu repeats these plus Copy / Select All / Close Help (19730-19756).

### 1.9 Rewrite notes (help)

- The page scrape depends on MediaWiki edit-form HTML; a rewrite should prefer `action=raw` or the API but must keep the `&qbide=1` convention if the server relies on it (server side not verifiable here).
- Cache file naming (`%HEX` escaping + spelling label) must be reproduced exactly to stay compatible with shipped `internal/help` contents.
- `links.bin` is derived data regenerated as a side effect of rendering one specific page; the heuristics depend on the rendered layout (bullet in column 1, link at column 3).
- The parser rewrites its own input (`a$`) during parsing (gallery conversion, template splice).

---

## 2. Configuration

### 2.1 Files

All under `settings\` relative to the qb64pe folder (`ConfigFolder$ = "settings"`, `source\ide\config\cfg_global.bas:45`). Paths are relative, so they depend on the process CWD being the qb64pe folder.

| File | Variable | cfg_global.bas | Scope | Format |
|---|---|---|---|---|
| `settings/config.ini` | `ConfigFile$` | 46 | global, with per-instance sections | INI |
| `settings/debug.ini` | `DebugFile$` | 47 | global, with per-instance and per-source-file sections | INI |
| `settings/bookmarks.bin` | `BookmarksFile$` | 48 | global, keyed by source file path | binary records |
| `settings/recent.bin` | `RecentFile$` | 49 | global | text, one path per line |
| `settings/searched.bin` | `SearchedFile$` | 50 | global | text, one search string per line |
| `settings/autosave<idx>.bin` | `AutosaveFile$` | 51 | per instance | zero-length flag file |
| `settings/undo3<idx>.bin` | `UndoFile$` | 52 | per instance | binary undo ring |

`<idx>` is `tempfolderindexstr$`: empty for instance 1, `(2)`, `(3)`, ... for further concurrently running instances (`source\qb64pe.bas:373-374`). `tempfolderindex` is determined at startup by locking `internal\temp[N]\temp.bin` (Windows/macOS: first folder whose `temp.bin` can be opened `LOCK WRITE`, qb64pe.bas:325-338; Linux: PID table in `internal/temp/tempfoldersearch.bin`, qb64pe.bas:287-324).

Startup housekeeping (cfg_global.bas:54-64): if `settings` does not exist it is created and `askToCopyOther = _TRUE`; otherwise an obsolete first/second generation `undo<idx>.bin` is deleted.

### 2.2 INI engine and write policy

- Reduced copy of Fellippe Heitor's INI-Manager v1.01 (`source\utilities\ini-manager\ini.bi`, `ini.bm`, `readme.txt`). API: `ReadSetting$(file$, section$, key$)` (ini.bm:157), `WriteSetting file$, section$, key$, value$` (291), `IniDeleteKey` (4), `IniCommit` (39), `IniGetSection$` (65), `IniFormatSection$` (138), `IniCurrentSection$` (259), `IniLoad` (488) and the mode switches `IniSetAddQuotes` (452), `IniSetForceReload` (461), `IniSetAllowBasicComments` (470), `IniSetAutoCommit` (479). Status is returned in the global `IniCODE`.
- Wrappers in `source\utilities\strings.bas`: `WriteConfigSetting` (84), `ReadConfigSetting` (88; returns true only if the value is non-empty, so an empty value is indistinguishable from a missing key), `ReadWriteBooleanSettingValue%` (96; only literal True/False accepted, anything else rewrites the default), `ReadWriteStringSettingValue$` (116), `ReadWriteLongSettingValue&` (135; values <= 0 are replaced by the default), `BoolToTFString$` (58), `TFStringToBool%` (71; returns -1/0/-2).
- Modes set at startup (cfg_global.bas:81-84): AddQuotes off, **ForceReload on** (the file is re-read from disk on every access, which is what lets several IDE instances share one config.ini), BASIC `'` comments allowed, **AutoCommit on** (every `WriteSetting` rewrites the whole file immediately).
- Consequence: **settings are persisted immediately when changed** (dialog OK, menu toggle, window resize/move), never batched at exit. There is no "save settings on exit" step. `ReadInitialConfig` itself writes back every missing/invalid key with its default, so the file is self-healing and fully populated after first start.
- Key lookup is case-insensitive (ini.bm `CheckKey`, lower-cased INSTR). The line-ending style of an existing file is preserved (`IniLoad`, ini.bm:488-537).

### 2.3 Reading at startup

`ReadInitialConfig` (`source\ide\config\cfg_methods.bas:79-651`) is called once from `source\qb64pe.bas:409`, immediately after `'$INCLUDE:'ide\config\cfg_global.bas'` (408) and **before** command-line parsing (411), so `-s`/`-f` switches override loaded values. It is called a second time after `CopyFromOther` (ide_methods.bas:1217).

Boolean parsing in the hand-written blocks: `UCASE$(value$) = "TRUE" OR VAL(value$) <> 0` is true; everything else false.

### 2.4 config.ini: complete key table

Section name variables are defined in cfg_global.bas:68-77. "Line" = cfg_methods.bas line where the key is read.

**[GENERAL SETTINGS]** (`generalSettingsSection$`)

| Key | Type | Default | Range / validation | Global variable | Line |
|---|---|---|---|---|---|
| DisableSyntaxHighlighter | bool | False | | `DisableSyntaxHighlighter` | 81 |
| PasteCursorAtEnd | bool | True | | `PasteCursorAtEnd` | 90 |
| AutoCloseBrackets | bool | True | | `AutoCloseBrackets` | 102 |
| ExeToSourceFolderFirstTimeMsg | bool | False | "don't show again" flag | `ExeToSourceFolderFirstTimeMsg` | 114 |
| WhiteListQB64FirstTimeMsg | bool | False | "don't show again" flag | `WhiteListQB64FirstTimeMsg` | 126 |
| DefaultExeSaveFolder | path | "" (= qb64pe folder) | must exist (`_DIREXISTS`), trailing separator added; never auto-written | `DefaultExeSaveFolder$` | 138-144 |
| SaveExeWithSource | bool | False | | `SaveExeWithSource` | 146 |
| EnableQuickNav | bool | True | | `EnableQuickNav` | 158 |
| ShowErrorsImmediately | bool | True | | `IDEShowErrorsImmediately` | 170 |
| ShowLineNumbers | bool | True | | `ShowLineNumbers` | 182 |
| ShowLineNumbersSeparator | bool | True | | `ShowLineNumbersSeparator` | 194 |
| ShowLineNumbersUseBG | bool | True | | `ShowLineNumbersUseBG` | 206 |
| BracketHighlight | bool | True | | `BracketHighlight` | 218 |
| KeywordHighlight | bool | True | | `KeywordHighlight` | 230 |
| MultiHighlight | bool | True | | `MultiHighlight` | 242 |
| IgnoreWarnings | bool | False | | `IgnoreWarnings` | 254 |
| BackupSize | int (MB) | 500 | 10..2000 | `idebackupsize` (undo file size limit, used as `idebackupsize * 1000000` bytes, ide_methods.bas:1298) | 266 |
| MaxRecentFiles | int | 20 | 5..200 | `ideMaxRecent` | 272 |
| MaxSearchStrings | int | 50 | 5..200 | `ideMaxSearch` | 278 |
| WikiBaseAddress | string | `https://qb64phoenix.com/qb64wiki` | none | `wikiBaseAddress$` | 284 |
| UseGuiDialogs | bool | True | | `UseGuiDialogs` | 291 |
| DefaultTerminal | string | auto-detected by `findWorkingTerminal$` | Linux only (not macOS, not Windows) | `DefaultTerminal$` | 293-299 |

**[MOUSE SETTINGS]**: `SwapMouseButton` bool, default False -> `MouseButtonSwapped` (302).

**[DEBUG SETTINGS]**

| Key | Type | Default | Validation | Variable | Line |
|---|---|---|---|---|---|
| BaseTCPPort | int | 9000 | 0 -> 9000 (no upper check here) | `idebaseTcpPort` | 312 |
| WatchListToConsole | bool | False | | `WatchListToConsole` | 316 |
| AutoAddDebugCommand | bool | True | | `AutoAddDebugCommand` | 325 |

The debugger listens on `idebaseTcpPort + tempfolderindex` and exports it to the child through `ENVIRON "QB64DEBUGPORT=..."` (ide_methods.bas:778-780). The port is changed via a dialog that writes `BaseTCPPort` (ide_methods.bas:16284).

**[LOGGING SETTINGS]** (cfg_methods.bas:338-344)

| Key | Default | Variable | Notes |
|---|---|---|---|
| LogMinLevel | None | `LogMinLevel$` | `LoggingEnabled = (LogMinLevel$ <> "None")`. Other value seen: "Information" (cfg_methods.bas:67); the full list of legal values lives in the logging dialog (ide_methods.bas ~16810), not verified here. |
| LogScopes | qb64 | `LogScopes$` | comma list, e.g. `qb64,libqb,libqb-image,libqb-audio` |
| LogHandlers | console | `LogHandlers$` | comma list; `LogToConsole` true if it contains `console` |
| LogFileName | `_STARTDIR$ + "qb64pe-log.txt"` | `LogFileName$` | |

**[IDE DISPLAY SETTINGS]** (`displaySettingsSection$`)

| Key | Type | Default | Range | Variable | Line |
|---|---|---|---|---|---|
| IDE_SortSUBs | bool | False | | `IDESortSubs` | 347 |
| IDE_KeywordCapital | bool | False | pair with next: both equal = CaMeL (`_EQUAL`), Capital = UPPER (`_GREATER`), Lowercase = lower (`_LESS`) | `IDEAutoLayoutKwStyle` | 359-376 |
| IDE_KeywordLowercase | bool | False | see above | `IDEAutoLayoutKwStyle` | 363 |
| IDE_SUBsLength | bool | True | | `IDESubsLength` | 378 |
| IDE_AutoPosition | bool | True | | `IDEAutoPosition` | 390 |
| IDE_NormalCursorStart | int | 6 | 0..31 | `IDENormalCursorStart` | 402 |
| IDE_NormalCursorEnd | int | 8 | 0..31 | `IDENormalCursorEnd` | 409 |
| IDE_AutoFormat | bool | True | anything not False/0 is True | `IDEAutoLayout` (+ `DEFAutoLayout`) | 416 |
| IDE_AutoIndent | bool | True | same | `IDEAutoIndent` (+ `DEFAutoIndent`) | 430 |
| IDE_IndentSUBs | bool | True | same | `IDEIndentSubs` | 444 |
| IDE_IndentSize | int | 4 | 1..64 | `IDEAutoIndentSize` | 457 |
| IDE_CustomFont | bool | False | | `IDECustomFont` | 464 |
| IDE_UseFont8 | bool | False | | `IDEUseFont8` | 472 |
| IDE_CustomFont$ | path | Win: `_DIR$("fonts")+"lucon.ttf"`; Linux: `.../truetype/liberation/LiberationMono-Regular.ttf`; macOS: `Courier New.ttf` | | `IDECustomFontFile$` | 480 |
| IDE_CustomFontSize | int | 19 | 8..99 | `IDECustomFontHeight` | 492 |
| IDE_CodePage | int | 0 | 0..`idecpnum` | `idecpindex` | 499 |

**[CUSTOM DICTIONARIES]**: `CustomKeywords$` = `@`-separated list, upper-cased and wrapped in `@` into `listOfCustomKeywords$` / `customKeywordsLength` (504-516). If absent, two quoted help keys `Instructions1`, `Instructions2` and `CustomKeywords$=@` are written (518-522).

**[IDE COLOR SCHEMES]** (global): `Instructions1`, `Instructions2` (always rewritten, 526-529) and user schemes `Scheme1$`, `Scheme2$`, ... (see 2.6).

**[IDE WINDOW n]** (per instance; n = `STR$(tempfolderindex)`, i.e. literally `IDE WINDOW 1` with one space)

| Key | Default | Range | Variable | Line |
|---|---|---|---|---|
| IDE_TopPosition | none (absent -> 0 and `IDEBypassAutoPosition = _TRUE`) | | `IDETopPosition` | 532 |
| IDE_LeftPosition | same | | `IDELeftPosition` | 540 |
| IDE_Width | 120 | 80..999 columns | `idewx` | 548 |
| IDE_Height | 40 | 25..999 rows | `idewy` | 552 |

**[IDE COLOR SETTINGS n]** (per instance). Values are stored as text `_RGB32(r, g, b)` (`rgbs$`, qb64pe.bas:28333) and parsed by `VRGBS~&` (qb64pe.bas:28313; requires the `_RGB` prefix, otherwise the default is kept).

| Key | Default (Super Dark Blue) | Variable | Line |
|---|---|---|---|
| SchemeID | 1 (0 = user-defined) | local in colour dialog (ide_methods.bas:17653) | 570 |
| TextColor | 216,216,216 | `IDETextColor` | 574 |
| KeywordColor | 69,118,147 | `IDEKeywordColor` | 579 |
| ErrorColor | 170,0,0 | `IDEErrorColor` (NOTE: the fallback passed to VRGBS is `IDENumbersColor`, line 585, apparently a bug; the colour dialog never writes this key) | 584 |
| NumbersColor | 216,98,78 | `IDENumbersColor` | 589 |
| QuoteColor | 255,167,0 | `IDEQuoteColor` | 594 |
| CommentColor | 98,98,98 | `IDECommentColor` | 599 |
| ChromaColor | 170,170,170 | `IDEChromaColor` | 604 |
| MetaCommandColor | 85,206,85 | `IDEMetaCommandColor` | 609 |
| HighlightColor | 0,88,108 | `IDEBracketHighlightColor` | 614 |
| BackgroundColor | 0,0,39 | `IDEBackgroundColor` | 619 |
| BackgroundColor2 | 0,49,78 | `IDEBackgroundColor2` | 624 |

**[COMPILER SETTINGS]** (630-650)

| Key | Type | Default | Range | Variable |
|---|---|---|---|---|
| OptimizeCppProgram | bool | False | | `OptimizeCppProgram` |
| StripDebugSymbols | bool | True | | `StripDebugSymbols` |
| IncludeDebugInfo | bool | False | | `IncludeDebugInfo` |
| AbsoluteDebugPaths | bool | False | | `AbsoluteDebugPaths` |
| MaxParallelProcesses | int | 3 | 1..128 | `MaxParallelProcesses` |
| ExtraCppFlags | string | "" | | `ExtraCppFlags$` |
| ExtraLinkerFlags | string | "" | | `ExtraLinkerFlags$` |
| GenerateLicenseFile | bool | False | | `GenerateLicenseFile` |
| UseSystemMinGW | bool | False | Windows only; forced True elsewhere | `UseSystemMinGW` |

**[QBJS]** (not in ReadInitialConfig; read/written by the QBJS Web Build dialog, ide_methods.bas:14556-14570): `Port` (default 8080), `CompileOnly` (0/1, default 0), `CopyProjectFiles` (0/1, default 1).

### 2.5 Where keys are written (all immediate)

| Trigger | Keys | ide_methods.bas |
|---|---|---|
| Font fallback at start | IDE_CustomFont, IDE_CustomFont$, IDE_CustomFontSize | 230-232 |
| Window resize | IDE_Width, IDE_Height | 1037-1038 |
| Window moved (exact trigger not examined) | IDE_TopPosition, IDE_LeftPosition | 1439-1440 |
| First-time message boxes | WhiteListQB64FirstTimeMsg, ExeToSourceFolderFirstTimeMsg | 1234, 1766 |
| Options menu toggles | SwapMouseButton, DisableSyntaxHighlighter, PasteCursorAtEnd, AutoCloseBrackets, ShowErrorsImmediately, IgnoreWarnings, UseGuiDialogs, SaveExeWithSource, GenerateLicenseFile, WatchListToConsole, AutoAddDebugCommand, EnableQuickNav | 5239-5426 |
| View menu | ShowLineNumbers, ShowLineNumbersUseBG, ShowLineNumbersSeparator | 5827-5870 |
| Set default EXE folder | DefaultExeSaveFolder | 6230 |
| $DEBUG auto-add declined | AutoAddDebugCommand=False | 6272, 6310, 6406, 6457 |
| Highlighter auto-disabled | DisableSyntaxHighlighter=True | 13326 |
| SUBs dialog | IDE_SortSUBs, IDE_SUBsLength | 14239-14247 |
| Language/code page dialog | IDE_CodePage | 14717 |
| Code layout dialog | IDE_AutoIndent, IDE_IndentSize, IDE_IndentSUBs, IDE_AutoFormat, IDE_KeywordCapital, IDE_KeywordLowercase | 16007-16014 |
| Backup/undo dialog | BackupSize, MaxRecentFiles, MaxSearchStrings (shrinking BackupSize truncates the undo file, 16229-16233) | 16245-16249 |
| Debug port dialog | BaseTCPPort | 16284 |
| Compiler settings dialog | all COMPILER SETTINGS keys | 16509-16520 |
| Logging dialog | Log* | 16810-16813 |
| Terminal dialog | DefaultTerminal | 16956 |
| Display dialog | IDE_Width/Height, IDE_AutoPosition, IDE_NormalCursorStart/End, IDE_UseFont8, IDE_CustomFont, IDE_CustomFont$, IDE_CustomFontSize | 17516-17528 |
| Colours dialog | SchemeID, 10 colour keys, BracketHighlight, MultiHighlight, KeywordHighlight, SchemeN$ | 17911-18257 |
| Wiki http fallback | WikiBaseAddress | wiki_methods.bas:1177 |
| `-s` command line | IncludeDebugInfo, SaveExeWithSource, DefaultExeSaveFolder | qb64pe.bas:14478-14495 |

(The dialog names in the first column are inferred from the keys written; the dialogs themselves belong to the ide_methods section.)

### 2.6 Colour schemes

- 14 built-in schemes are hard-coded in `LoadColorSchemes` (ide_methods.bas:20880-20904): Super Dark Blue, Dark Blue, QB64 Original, Classic QB4.5, Dark Side, Camouflage, Plum, Cornfield, CF Dark, Broadcast, VS Code, X11 SgiColors, Light Green, All White.
- Scheme string format: `Name|` followed by ten 9-digit groups `RRRGGGBBB` in the order Text, Keyword, Numbers, Quote, MetaCommand, Comment, Background, Background2, Highlight, Chroma (order confirmed against the Super Dark Blue string at 20889, the defaults in cfg_methods.bas:557-567 and the key order at ide_methods.bas:18212-18223).
- User schemes are stored in `[IDE COLOR SCHEMES]` as `Scheme1$`, `Scheme2$`, ... and loaded until the first missing index (20907-20948). Migration on load: 54-digit (6-colour, v1.1) and 81-digit (9-colour) strings are upgraded to 90 digits and written back; invalid entries become `"0"` (a deleted scheme is also stored as `0`, ide_methods.bas:17954).
- `SchemeID` in the per-instance colour section is an index into built-ins + user schemes (user schemes therefore start at 15); 0 means "User-defined" and the individual colour keys are authoritative.

### 2.7 debug.ini

- `[VWATCH PANEL n]` (per instance, cfg_global.bas:79): `vWatchPanel.x`, `vWatchPanel.y`, `vWatchPanel.w`, `vWatchPanel.h` (read ide_methods.bas:7061-7070, written 7489-7495; all four reset to 0 at 7511-7514).
- One section **per source file**, named by the file path string passed as `f2$`: `total breakpoints`, `breakpoint N` (line numbers), `total skips`, `skip N` (read in `IdeImportBookmarks` ide_methods.bas:19211-19224, written in `IdeSaveBookmarks` 19249-19269, only when `$DEBUG`/vWatch is on). Key names use `"breakpoint" + STR$(i)`, i.e. with a space before the number.

### 2.8 Side files

- **recent.bin**: plain text, one full path per line, most recent first, native line endings. Maintained by `AddToHistory "RECENT", path$` (ide_methods.bas:19900-19930) using the s-buffer API (`FileToBuf%`, `ReadBufLine$`, `DeleteBufLine`, `WriteBufLine`, `BufToFile`): case-insensitive de-duplication, new entry inserted at top, entries beyond `ideMaxRecent` dropped. Read by the File menu builder (19430) and `iderecentbox$` (19288). "Clear recent" truncates the file (6606, 6629); "Remove Broken Links" drops non-existent files (6805-6822).
- **searched.bin**: same format/logic with `ideMaxSearch` (`AddToHistory "SEARCH"`, `RetrieveSearchHistory` 19945-19957; cleared at 6122).
- **bookmarks.bin**: concatenated records `CRLF + <path> + CRLF + MKL$(len) + data`, data = 16 bytes per bookmark: `MKL$(y) + MKL$(x) + MKL$(reserved) + MKL$(reserved2)`. Lookup is a case-insensitive INSTR for the path framed by CRLF; the newest record is prepended (ide_methods.bas:19187-19246).
- **autosave<idx>.bin**: empty flag file created on the first undo event of a session (ide_methods.bas:1370-1374) and deleted on clean exit (6524). If it exists at startup the IDE asks "Recover program from auto-saved backup?" (`iderestore$`, message at 12957) and restores the newest state from the undo file (545-573).
- **undo3<idx>.bin**: 12-byte header of three LONGs `[offset of oldest entry][offset of newest entry][offset of top-most entry when wrapped]` (first entry at offset 13), followed by entries framed as `MKL$(len) + payload + MKL$(len)` (a list navigable in both directions). Payload: scroll/cursor/selection state (`idesx, idesy, idecx, idecy, ideselect, ideselectx1, ideselecty1`), `iden, idel, ideli`, bookmark count and (y,x) pairs [v2], then `MKL$(compressed size) + MKL$(original size) + _DEFLATE$(idet$)` [v3] (reader ide_methods.bas:550-570, writer 1268-1300). It is a ring buffer limited to `BackupSize` MB. `idet$` is the IDE's internal whole-document buffer (format owned by the ide_methods section).

### 2.9 Migration from older versions

- Layout history: before v3.14.0 config lived in `internal\config.ini` and `internal\temp\` (debug.ini, bookmarks.bin, recent.bin, searched.bin); since v3.14.0 everything is in `settings\` (comments at cfg_methods.bas:9, 13).
- No silent auto-migration: when `settings\` is missing, the IDE shows a welcome box offering to import from another installation (ide_methods.bas:1203-1221) and runs `CopyFromOther` (cfg_methods.bas:2-76): folder picker, folder must contain `qb64pe`/`qb64pe.exe`; copies config.ini, debug.ini, bookmarks.bin, recent.bin, searched.bin and converts when the source is pre-3.14.0:
  - `SchemeID` remap: custom IDs > 10 shift by +4 (four new built-ins); built-ins 1..10 map through the lookup string `"12349567de"` (hex digits) (16-31). Note: only the first `SchemeID=` occurrence in the file is patched.
  - debug.ini `[settings]` -> `[VWATCH PANEL 1]` (35-39);
  - recent.bin: blank lines removed, CRLF -> native EOL (42-47); searched.bin: CRLF -> native (49-53).
- v4.2.0 key moves, applied after every import (54-70): `[GENERAL SETTINGS] DebugInfo` -> `[COMPILER SETTINGS] IncludeDebugInfo`; `DefaultTerminal` deleted on Windows; `[GENERAL SETTINGS] LoggingEnabled` -> `[LOGGING SETTINGS] LogMinLevel` (`None`, or `Information` plus scopes `qb64,libqb,libqb-image,libqb-audio` and handler `console`).
- Undo files: generation 1/2 (`undo<idx>.bin`, uncompressed) are deleted; generation 3 is `undo3<idx>.bin`.
- Colour scheme string upgrades: see 2.6.

---

## 3. Export (`source\ide\ide_export.bas`)

### 3.1 Entry points

- Single routine `SUB ExportCodeAs (docFormat$)` (ide_export.bas:1-631).
- Menu: File -> "Export As..." (a sub-menu, marked with `CHR$(16)`; item index kept in `FileMenuExportAs`, sub-menu ID in `FileMenuExportAsSubMenuID`, `ide_global.bas:227`; menus built at ide_methods.bas:494-504 and 19426-19428). The item is disabled (`~` prefix) while compiling (ide_methods.bas:1140, 6890) and re-enabled afterwards (895, 1069, 6892).

| Menu item | Call | ide_methods.bas | Output |
|---|---|---|---|
| Hypertext document (.htm) | `ExportCodeAs "html"` | 5883-5885 | file `<idepath>/<progname>.htm` (i.e. `foo.bas.htm`) |
| Rich Text document (.rtf) | `ExportCodeAs "rich"` | 5890-5892 | file `<progname>.rtf` |
| Discord codebox (to Clipboard) | `ExportCodeAs "disc"` | 5897-5899 | clipboard |
| Forum codebox (to Clipboard) | `ExportCodeAs "foru"` | 5904-5906 | clipboard |
| Wiki example (to Clipboard) | `ExportCodeAs "wiki"` | 5911-5913 | clipboard |

Unsaved programs export as `Untitled<idx>.bas.<ext>` (ide_export.bas:15). Existing files prompt for overwrite (21-25).

### 3.2 Generation

- Input: the current selection (`getSelectedText$(-1)`), else the whole buffer joined with CRLF (LF on Linux) (31-43). Progress is shown in the status bar as a percentage (35-37, 80-82).
- The exporter does **not** reuse the IDE's on-screen highlighter code. It is a separate single-pass character state machine (79-250) with its own flags (comment, quote, number, keyword, metacommand, legacy `'$` / `REM $` metacommand, DATA line, pre-compiler line, line break, parenthesis nesting). It does share the same **word lists**: `listOfKeywords$` (from `syntax_highlighter_list.bas`), `listOfCustomKeywords$` (config) and `UserDefineList$` ($LET names) (`VerifyKeyword`, 385-405), plus `ids()` to detect user functions with arguments (129-134).
- Keywords **are linked to the wiki**. `FindWikiPage` (407-498) maps a token to a page name: upper-cased keyword by default, plus context rules: OPEN clause words -> `OPEN#File_Access_Modes` / `OPEN#File_ACCESS_and_LOCK_Permissions`; `(function)` suffix for the dual statement/function keywords in `fu$` (line 62) when in expression context; `(boolean)` for AND/OR/XOR in conditions; derived math functions (line 60) -> `Mathematical Operations#Derived_Mathematical_Functions`; `_STATIC`/`_DYNAMIC` -> `(type field)`; two-word keywords merged by look-ahead (END IF, EXIT DO, SELECT CASE, DO...LOOP, LINE INPUT, PRINT USING, ON ERROR, ON KEY(n), DECLARE LIBRARY, DEF SEG, OPTION BASE, `$END IF`, ...); `GET`/`PUT` -> `(general)` when not followed by `#`; `INPUT`/`PRINT`/`WRITE`/`LINE INPUT`/`PRINT USING` -> `(file statement)` when followed by `#`.
- The link URL is **hard-coded** as `https://qb64phoenix.com/qb64wiki/index.php?title=<page>` (510, 514, 523); it ignores the configurable `wikiBaseAddress$`.

| Format | Wrapper (275-302) | Colours | Keyword output (500-536) | Escaping (566-605) |
|---|---|---|---|---|
| html | `<!DOCTYPE html>...<meta charset="UTF-8"><title>name</title>...<pre style="font-size: 18px; background-color: BG; color: TEXT;">` ... `</pre></body></html>` | current IDE theme (`GetThemeColors`, 615-630) | `<a style="text-decoration: none; color: C;" href="URL" title="page">kw</a>` | `"` `&` `<` `>` -> entities; bytes > 127 -> UTF-8 via `_MAPUNICODE` (U+FFFD if unmapped) |
| rich | `{\rtf1\ansi\deff0{\fonttbl{\f0 Courier New;}}{\colortbl ...}\pard\f0\fs32\cbpat6\paperh23811\paperw16838\marg?142` ... `}`; `\par` per line (607-613) | IDE theme as colour table: cf0 text, cf1 comment, cf2 meta, cf3 keyword, cf4 number, cf5 quote, entry 6 = background | `{\field{\*\fldinst HYPERLINK "URL"}{\fldrslt{\cfN\ul0 kw}}}\cf0 ` | `\ { }` backslash-escaped; bytes > 127 -> `\uN\'bf` |
| disc | three backticks + `ansi`, then ESC`[0;0;1;38m` ... three backticks | fixed ANSI SGR: comment 37, number 31, quote 33, keyword 34, metacommand 32, custom keyword 36, reset 38 | colour only, **no links** (517) | none (bytes copied as-is) |
| foru | `[qb=export]` ... `[/qb]` | fixed: comment #919191, number #F580B1, quote #FFB100, keyword #4593D8, meta #55FF55 | `[url=URL][color=C]kw[/color][/url]` | bytes > 127 -> `&#N;` with the raw code-page value |
| wiki | `{{CodeStart}}` ... `{{CodeEnd}}` | fixed (same hex values) via `{{Text|...|#colour}}`; comments/strings wrapped in `<nowiki>` | `{{Cl|kw}}` / `{{Cm|meta}}` or `{{Cl|page|kw}}` | `& < >` -> entities; bytes > 127 -> `&#N;` |

- Custom keywords and `$LET` names are coloured (meta colour) but not linked (`CustomNoLink`, 538-559); unknown identifiers are copied unchanged (561-564).
- Encoding: only html/rtf convert code-page bytes to Unicode, using the **currently active IDE code page mapping** (`_MAPUNICODE`), through `UnicodeToUtf8Char$` (657-676) and `AnsiTextToUtf8Text$` (678-695). Forum/wiki deliberately keep the raw byte values as numeric entities so examples can be pasted back into the IDE (comments at 9-13, 589-600). Discord does no conversion.
- Output buffer: `eTxt$` pre-sized to 1 MB and grown by 1 MB when within 10 kB of the end (46, 249). Output is written with fixed-length `MID$` assignments whose lengths are hand-computed (e.g. `(2 * pal%) + lkl% + 120`), so they must match the literal text exactly.
- Completion message boxes: 261 (file), 264-268 (Discord: reports block size and the 2000/4000 character message limits), 271 (forum/wiki).
- Reverse path: `StripDiscordANSI$` (633-655) removes the code fence and the ESC sequences; it is applied to clipboard pastes into the editor and the search field (ide_methods.bas:2222, 3728, 14969).

---

## 4. Converters

### 4.1 `source\ide\ide_converters.bas`

Two functions.

**`BinaryFormatCheck% (pathToCheck$, pathSepToCheck$, fileToCheck$)`** (lines 2-105). Called on every file-open path: startup/command-line load (ide_methods.bas:585), the old file dialog (12842) and `OpenFile$` (21359). A return value > 0 means "do not load".
- Detection: read the whole file; if it contains no NUL byte it is treated as text (line 10). Otherwise the first two INTEGERs are read as `Format%`, `Version%` (12-13):
  - 2300 = VBDOS -> "not supported" (17-19); 764 = QBX 7.1 -> "not supported" (20-22); 252 = QuickBASIC 4.5 (23). Any other binary file falls through (returns 0) and is loaded as-is.
- QB4.5: expected tool `internal\utilities\QB45BIN.exe` (Windows) or `./internal/utilities/QB45BIN` (25-27).
  - If present: ask "QuickBASIC 4.5 binary format detected. Convert to plain text?"; on Yes run `QB45BIN "<file>" -o "<name> (converted).<ext>"` (33-43, 52-53) with `SHELL _HIDE`. On success the **by-reference** arguments `pathToCheck$` / `fileToCheck$` are rewritten to point at the converted file and the function returns 0, so the caller simply loads the converted file (59-70). Failure returns 2; "No" returns 1.
  - If absent: **yes, the IDE compiles the tool on demand with itself**: `SHELL _HIDE "qb64pe -x internal/support/converter/QB45BIN.bas -o internal/utilities/QB45BIN"` (92-94; creates `internal/utilities` if needed, 83) and then jumps to the conversion code (`GOTO ConvertIt`, 96). If the tool source is missing too: "Conversion utility not found" (75-79).
  - Observation (from reading only): the `GOTO ConvertIt` at 96 jumps into the other IF branch, so on that path the trailing `BinaryFormatCheck% = 1` (102) is not executed.

**`OfferNoprefixConversion% (file$)`** (107-141).
- Trigger: not at file-open but at compile time. When the compiler meets `$NOPREFIX` in IDE mode it sends message 13 to the IDE (qb64pe.bas:2020-2024; protocol comments ide_methods.bas:29-30). The IDE handler (ide_methods.bas:852-864) offers the conversion only if `ideFirstCompileFromDisk`; if not applicable or declined it answers with message 14 and the compiler raises "$NOPREFIX is a deprecated feature, QB64(PE) specific keywords MUST have the underscore" (qb64pe.bas:1084-1087). In no-IDE mode the error adds "To convert this program, open it in the IDE." (2026).
- Tool: `internal\utilities\AddPREFIX.exe` / `./internal/utilities/AddPREFIX`; compiled on demand with `qb64pe -x internal/support/converter/AddPREFIX.bas -o <tool>` (122-129). Invoked as `AddPREFIX "<file>"`; exit code 0 followed by a successful `OpenFile$(file$)` reloads the converted source and a new compilation is triggered (131-133; ide_methods.bas:855-858).

### 4.2 Programs in `internal\support\converter\`

| Program | Purpose | CLI contract | Built when |
|---|---|---|---|
| `QB45BIN.bas` (3231 lines; by qarnos, CLI by FellippeHeitor, header lines 1-3) | Detokenises QuickBASIC 4.5 "fast load" binary `.bas` into text using embedded parse rules (`LoadParseRules`) | `QB45BIN <source.bas> [-o output.bas]` (152-164). Without `-o` the output is `<input>.converted.bas` (180-191; the usage text claims the original is overwritten, the code does not do that). Exit 1 for usage / file not found / 30 s timeout ("Conversion failed.", 224-227); exit 0 on success (241). Options `OmitIncludedLines` and `SortProceduresAZ` hard-wired to -1 (41-42). | On first use by the IDE via `qb64pe -x`; output `internal/utilities/QB45BIN[.exe]` |
| `AddPREFIX.bas` (787 lines; `$CONSOLE`, `$SCREENHIDE`) | Removes `$NOPREFIX` by tokenising the program and all its `$INCLUDE` files and adding `_` to QB64 keywords (keyword list frozen "as of v3.14.1", line 6; also builds `$COLOR:0/32` name lists, 157-160) | `AddPREFIX <file>` (console output). With no argument it shows a window and prompts for a file (107-118). Each processed file is renamed to `<name>-noprefix.<ext>` as backup and rewritten under the original name (179-194). Prints "Program does not use $NOPREFIX, no changes made" when nothing to do. Ends via `SYSTEM` (exit 0). | On first use by the IDE; output `internal/utilities/AddPREFIX[.exe]` |
| `qbjs-build.bas` (446 lines; `$CONSOLE:ONLY`) | Builds a program for the web with QBJS (github.com/boxgaming/qbjs) and serves it locally | `qbjs-build source.bas [-port:8080] [-mode:auto|play] [-compileOnly] [-noProjectFiles] [-clean] [-warnings:<file>]` (71-123). Exit codes (2-3): 1 no node.js, 2 no network, 3 compile warnings/errors, 4 no source, 5 multiple sources, 6 invalid option, 7 invalid mode. Requires `node` on PATH (291). Checks the latest QBJS release at most once per day by scraping `https://github.com/boxgaming/qbjs/releases/latest`, caches tag + date in `.qbjs-config` (125-162), downloads the zip from codeload.github.com into `.qbjs/qbjs-<ver>` and unzips it with PowerShell / `unzip` (164-182), runs `node qbc.js <src> <srcdir>/_web/program.js` (184-223), copies web dependencies and optionally all non-.exe/.bas project files (225-273), starts `qbjs-webserver.js` on the port and opens `http://localhost:<port>/index.html` (57-64, 275-289). | By the "QBJS Web Build..." dialog (menu item ide_methods.bas:324, handler 5376-5379, `ideQBJSBuildBox` 14260): if `internal/support/converter/qbjs-build[.exe]` is missing it runs `qb64pe -x ...qbjs-build.bas -o ...qbjs-build[.exe]` (14388-14393). Note the output is placed next to the source, not in `internal/utilities`. |

IDE side of QBJS: the dialog writes the buffer to `<idepath>/.qbjs-temp.bas` prefixed with the line `'$Include: 'lib/compatibility/qb64pe.bi'` (14405-14415), runs the tool with stdout redirected to `.qbjs-build-out` and the exit code to `.qbjs-exit-code` (14417-14424), and shows the warnings file in a list (14531-14551). Settings are in `[QBJS]` (see 2.4). Language-support link: `https://github.com/boxgaming/qbjs/wiki/QBasic-Language-Support` (14574).

Rewrite implication: the converters are ordinary QB64 programs compiled by the product itself through its own CLI (`-x ... -o`). A rewrite must keep that CLI contract (switches, `-o` semantics, exit codes) and either keep compiling these on demand or ship them prebuilt / fold them in.

---

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
