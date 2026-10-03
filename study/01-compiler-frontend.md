# QB64-PE compiler front end and pipeline — study report

Source studied: `..\QB64pe` (version string `4.7.0-GLFW`, `source\global\version.bas:13`).
All `file:line` references are to `source\qb64pe.bas` unless another file is named.

**This tree is not stock QB64-PE.** It carries extensions that a rewrite must decide on explicitly:
TYPE member arrays with `_Static` / `_Dynamic` storage markers (`$UNSTABLE:TYPEFIELDS`), `REDIM _RETAIN`, `_ARRAYCOPY`,
`$USELIBRARY`, `$ERRORLOCATION`, `-f:TargetBits` cross-bitness, `#line` emission (`WriteBufLineCpp`), and a GLFW runtime.

**Coverage note.** Read in full: lines 1–7300 (setup, IDE dispatcher, prepass, main-pass head, SUB/FUNCTION, CONST, DEFxxx,
IF/SELECT/CASE), 9685–10165, 12425–14730, `lineformat` (24588–25555), `findid`, `regid`, `validlabel`, `validname`,
preprocessor helpers, and all of `utilities\` except the UDT-dynamic emitters in `type.bas` and `arrcpy.bm`.
Located by grep and skimmed only (line ranges are accurate, internal behavior is not fully verified): FOR/NEXT, DO/LOOP,
WHILE/WEND bodies, ON STRIG/TIMER/KEY, SHARED, ASC/MID$/ERASE statements, the whole DIM/REDIM/COMMON block (8767–9685),
`_MEMGET/_MEMPUT/_MEMFILL/_ARRAYCOPY`, and the generic sub-call path (10808–12431).
Nothing here was executed; behavior is inferred from reading the code.

---

## 1. File map

| File | Lines | Role |
|---|---|---|
| `source\qb64pe.bas` | 28,828 | Everything: setup, IDE dispatch loop, prepass, main pass, finalization, C++ build, plus 116 SUB/FUNCTIONs from line 14354 |
| `source\global\version.bas` | 31 | `Version$`, CI label from `internal/version.txt` |
| `source\global\settings.bas` | 3 | `CONST Debug = 0` |
| `source\global\constants.bas` | 28 | `sp`/`sp2`/`sp3` separators, `OS_BITS`, `TARGET_BITS`, line endings |
| `source\emit\logging.bas` | 34 | `EmitLoggingStatement` (`_LOGTRACE/INFO/WARN/ERROR`) |
| `source\utilities\hash.bi/.bas` | 85 / 333 | Symbol hash table and flags |
| `source\utilities\type.bi/.bas` | 94 / 2332 | Type flag constants, UDT tables, symbol/type conversions, UDT layout + emit helpers |
| `source\utilities\elements.bas` | 642 | Element-list (token string) primitives, string/number element codecs |
| `source\utilities\const_eval.bi/.bas` | 20 / 1136 | CONST expression evaluator (prepass only) |
| `source\utilities\statevars.bi/.bas` | 13 / 47 | `RCStateVar` "feature needs a recompile" tracker |
| `source\utilities\file.bas`, `strings.bas` | 176 / 151 | Path helpers, `StrReplace$`, config read/write wrappers |
| `source\utilities\format.bas` | 106 | `apply_layout_indent$` (the `-y` formatter back end) |
| `source\utilities\build.bas` | 37 | make executable / compiler path, `nm` output file names, purge |
| `source\utilities\s-buffer\*` | ~1100 | In-memory "files" (`OpenBuffer%`, `WriteBufLine`, `WriteBuffers`) + `#line` emission |
| `source\utilities\ini-manager\*` | ~560 | INI reader/writer for config and `$USELIBRARY` descriptors |
| `source\utilities\arrcpy.bm` | 994 | `_ARRAYCOPY` statement (fork-specific) |
| `source\utilities\give_error.*` | 10 | `Give_Error` sets `Error_Happened` / `Error_Message` |

Include order matters: `.bi` files at 24–33 and 428, IDE globals at 38, method bodies at 28806–28827.

---

## 2. End-to-end control flow

The main program is one flat module (lines 1–14352) driven by `GOTO`. There is no call structure between the phases.

### 2.1 One-time startup (1–843)

| Lines | What |
|---|---|
| 8–22 | The compiler's own `$CONSOLE`, `$SCREENHIDE`, `$EXEICON`, `$VERSIONINFO` |
| 51–73 | `$IF` state arrays; preset precompiler defines `WINDOWS WIN LINUX MAC MACOSX 32BIT 64BIT VERSION _QB64PE_ _ARM_`. `VERSION` must stay at index 7 (71–72) |
| 79–91 | Abort if `internal` folder is missing |
| 93–109 | `DEPENDENCY_*` constants (12) and `DEPENDENCY()` array |
| 185–214 | `Opt()/T()/Lev()/...` tables for the custom-syntax argument parser (`seperateargs`) |
| 242–269 | `os$` is only `"WIN"` or `"LNX"`; macOS is `os$="LNX"` plus `MacOSX=1` |
| 272–355 | Temp-folder locking: `internal\temp\temp.bin` opened `LOCK WRITE` as file #26. If locked, try `temp2`, `temp3`… up to 999. Linux uses a PID table in `tempfoldersearch.bin` and `ps -p`. For index >1 a patched `internal\c\qbxN.cpp` is generated with `../temp/` rewritten |
| 408–411 | `ReadInitialConfig`, then `ParseCMDLineArgs$` |
| 420–426 | Console vs. window |
| 428–820 | Hash tables, `Label_Type`, `idstruct`, control stack, lexer char tables, bit masks |
| 834 | `gl_scan_header` (OpenGL extension scan) |

### 2.2 Command line (`ParseCMDLineArgs$`, 14354–14611)

Only the first two characters of each token select the switch.

| Switch | Effect |
|---|---|
| `-?` `-h` `--help` `/?` `/h` `/help` | Help, exit |
| `-v` | Version, exit |
| `-o <file>` | `CMDLineOutFile$`; later forces exe-with-source (13535) |
| `-l:<n>` | IDE start line |
| `-c` | `NoIDEMode` (compile, own window) |
| `-x` | `NoIDEMode` + `ConsoleMode` |
| `-y` | `FormatMode` + console + quiet; requires `-o` (14600) |
| `-z` | `NoCCompileMode` (generate C++ only) |
| `-p` | Purge build files (`make clean`) |
| `-e` | `ForceOptExpl` (dropped again if the IDE is used, 14598) |
| `-s[:setting[=value]]` | Persistent settings: `DebugInfo`, `ExeWithSource`, `ExeDefaultDir` |
| `-f[:setting=value]` | Per-run settings: `OptimizeCppProgram`, `StripDebugSymbols`, `AbsoluteDebugPaths`, `ExtraCppFlags`, `ExtraLinkerFlags`, `MaxCompilerProcesses` (1–128), `TargetBits` (32/64), `GenerateLicenseFile`, `UseSystemCompiler`, `AutoIndent`, `AutoIndentSize` (1–64), `IndentSubs`, `AutoLayout`, `KeywordCapitals`, `KeywordLowercase` |
| `-w` `-q` `-m` | Show warnings / quiet / monochrome |
| `-u` | Hidden CI option: refresh help cache |
| anything else | First such token is the source file name |

Boolean values accepted: `true on yes 1 -1` / `false off no 0` (14686–14707).

### 2.3 IDE mode vs. command-line mode: one compile loop, two drivers

The compile code is written once. In IDE mode it is entered and left by `GOTO`, with the IDE acting as a coroutine through
`ide(0)`:

```
sendcommand:  idecommand$ = sendc$ : C = ide(0) : c$ = idereturn$      (853–858)
```

Messages the compiler sends (`sendc$` first byte): 1 load file, 3 next line please, 6 ready, 7 repass request,
8 error (+message +`MKL$(line)`), 10 "here is an included line" passback, 11 exe created, 12 exe renamed, 13 offer
`$NOPREFIX` conversion, 100 fetch a continuation line (sent from inside `lineformat`, 25525), 254 launch debugger,
255 internal runtime error.

Messages the IDE returns (`C`): 0 leave IDE, 2 begin (first line), 4 next line, 5 end of program, 9 run, 14 `$NOPREFIX`
refused.

| IDE reply | Path |
|---|---|
| `C=2` (860) | `idepass=1` → `GOTO fullrecompile` → state reset → `ideret1` → `GOTO ideprepass` → `ideret2` → ask for next line |
| `C=4` (873) | pass 1: `GOTO ideprepass`; pass 2: `a3$=c$` → `GOTO compileline` → returns at `ideret4` |
| `C=5` (889) | Sets `lastLine=1`, runs one more (empty) line through the current pass. After pass 1: `GOTO ide3` (open output buffers) → `ideret3` → send 7 (repass). After pass 2: `GOTO ide5` (finalize) → `ideret5` → send 6 |
| `C=9` (923) | If not yet built: `GOTO ide6` (make/g++) → `ideret6`. Then build `ExecuteLine$` and `SHELL` it |

Command-line mode (`noide:`, 1109) resolves the file name, falls into `fullrecompile:` (1159), `recompile:` (1227),
`lineinput3load` (1709), then runs the two `DO…LOOP`s directly: prepass 1803–3225, main pass 3337–12694.

Re-entry points and what they mean:

| Label | Line | Meaning |
|---|---|---|
| `fullrecompile` | 1159 | New compilation. Clears dependencies, `cmemlist`, `sflistn`, `SubNameLabels`, `useLibList$`, all `RCStateVar`s |
| `recompile` | 1227 | Another attempt at the same source. Applies pending `RCStateVar`s, clears hash table, re-registers keywords and internals, resets ~80 globals. Keeps `sfidlist/sfarglist/sfelelist`, `sfcmemargs`, `cmemlist`, `arrayelementslist`, `SubNameLabels`, `useLibList$` |
| `ideprepass` | 1804 | Process one line in the prepass |
| `ide3` | 3245 | Between passes: open all output buffers |
| `compileline` | 3338 | Process one line (or the rest of a line) in the main pass |
| `ide5` | 12704 | Post-pass finalization |
| `do_recompile` | 13001 | Close files, set `iderecompile`, `GOTO recompile` |
| `ide6` | 13492 | Resource files, `embedded.cpp`, make |
| `errmes` | 14308 | Report compile error |

**The compiler is a fixpoint loop.** A "recompile" restarts both passes from scratch. Triggers:

1. Any `RCStateVar` change when not yet locked: `$COLOR`, `OPTION _EXPLICIT/_EXPLICITARRAY`, `$ASSERTS`, `$CONSOLE`,
   `$DEBUG`, `$ERRORLOCATION`, sockets dependency (`statevars.bas:16`). `$COLOR` restarts immediately (1905, 1911); the
   others restart at the next `compileline` (3340) or at end of pass.
2. `$USELIBRARY` naming a new library (1975).
3. A SUB/FUNCTION parameter that must live in conventional memory (`cmemlist`/`sfcmemargs`, 12811–12840).
4. Array-parameter dimension counts pinned by a later call site (12885–12940).
5. COMMON arrays whose element count became known (12967–12997).
6. A label that turned out to be a one-word SUB call or vice versa (`PossibleSubNameLabels`/`SubNameLabels`, 13029–13034).

### 2.4 End of compilation and C++ build

`ide5` (12704–13491): `closemain`, open-block checks, `clear.txt`, recompile checks, label checks, global flags, DATA,
COMMON/CHAIN code, unused-variable warnings. `ide6` (13492–14219): output path, icon/manifest/rc, `WriteBuffers ""`
(13644 — the first moment anything reaches disk), `-y` exit (13646–13651), `embedded.cpp`, make line, `nm` symbol
resolution, `SHELL` make. Success is decided only by whether the exe file exists afterwards (14196–14201).
Exit codes: 14228–14230.

---

## 3. Line reading, elements, and layout

### 3.1 Reading lines

- CLI: whole file slurped by `lineinput3load` (28055), trailing `CHR$(26)` stripped; `lineinput3$` (28065) splits on CR, LF,
  CRLF or LFCR and returns `CHR$(13)` as the EOF sentinel.
- IDE: lines arrive one at a time through messages 2/4/5.
- `$INCLUDE` files: `LINE INPUT` from file handles 200–299 (`fh = 199 + inclevel`), max depth 100.
- Both passes add one synthetic empty "last line" (3227–3233, 12696–12702) so end-of-file auto-includes have a line to hang on.

### 3.2 `lineformat$` — the lexer (24588–25555)

Input: one raw source line. Output: a string of *elements* separated by `sp` (`CHR$(13)`; `CHR$(250)` when `Debug`).
All later parsing is string surgery on this form via `getelement$`, `getelements$`, `numelements`
(`utilities\elements.bas`). Every `getelement$(a$, n)` is an O(n) scan, so statement parsing is quadratic in line length.

| Input | Element produced | Lines |
|---|---|---|
| `"text"` | `"escaped text",<len>` (octal escapes for <32, `"`, >126; `\\`). Unterminated string is silently closed | 24607–24615, `elements.bas:593–629` |
| Decimal number | Normalized `[whole][.frac][E|D|F][+|-][exp][suffix]`, leading/trailing zeros culled. If a suffix was written, `,original-text` is appended for layout | 24618–24850 |
| Float with no suffix | Type inferred from digits: ≤7 significant digits and in SINGLE range → `E`; ≤16 → `D`; else `F` (`_FLOAT`) | 24765–24819 |
| `&H` `&O` `&B` | Converted to a decimal element plus suffix, `,original` appended. Default type from value width: ≤4 hex digits `%`, ≤8 `&`, else `&&`; signed wraparound applied, emitted as `-` `sp` `n` | 24854–25174 |
| Number directly after `ELSE` | A `GOTO` element is inserted | 24629–24631 |
| Identifier | Case preserved; type suffix glued on (`~%% ~%& ~&& ~% ~& ~\` %% %& && % & ! ## # $ \``); digits allowed after `$` and `` ` ``. Max 40 chars | 25182–25387 |
| `a.0b` | Emits `a` `.` `0b` | 25353–25369 |
| Trailing `_` | Line continuation: fetches the next physical line (file, include, or IDE message 100) and continues lexing. Sets `linecontinuation=1`, which disables layout for that line | 25500–25543 |
| `REM` | Comment; if preceded by `THEN`, a `'` no-op element is added first | 25196–25207 |
| `DATA` | Rest of statement is consumed raw. In the main pass the bytes are appended to `data.bin` and `DataOffset` advances. Element becomes `DATA` + `_<hex>` (nibble-swapped hex of the text) | 25211–25310 |
| `< = >` pairs | Fused across spaces; `><`→`<>`, `=>`→`>=`, `=<`→`<=` | 25396–25413 |
| Single chars | `( ) * + - / < = > \ ^ , . : ; # $ ? _` | 510–531, 25395–25421 |
| `'` | Comment. If the first non-blank char of the comment is `$`, scan for `$STATIC`, `$DYNAMIC`, `$INCLUDE:'file'`, `$FORMAT:ON/OFF` | 25426–25496 |
| Trailing `:` | A `'` no-op element is appended | 25549 |
| Anything else | `Unexpected character on line` | 25424 |

Side effects of the lexer (not just a pure function): writes `data.bin`, sets `addmetainclude$`, `addmetastatic`,
`addmetadynamic`, `layoutcomment$`, `linecontinuation`, `linenumber`, `inclinenumber()`, `IDEAutoIndent/IDEAutoLayout`,
and calls into the IDE.

`eleucase$` (`elements.bas:418`) upper-cases everything except string elements. Most code keeps two parallel strings:
`a$` (upper) and `ca$` (original case).

### 3.3 Layout (the auto-formatter) is produced by the parser

There is no separate formatter. Each statement handler builds its own pretty-printed text:

- `layout$` — the formatted line so far; handlers append `l$`. `tlayout$` is returned by `fixoperationorder`, `validlabel`,
  `assign` etc. for the expression/label they handled.
- Separators inside `layout$`: `sp` = "one space here", `sp2` (`CHR$(10)`) = "no space here".
- `SCase$` / `SCase2$` (28733–28776) apply the keyword-case style (`IDEAutoLayoutKwStyle` <0 lower, >0 upper, 0 CamelCase).
- `layoutdone` must be set to 1 by the handler; otherwise `layoutok=0` and the original text is kept (12493, 12666).
- `layoutcomment$` is appended after the statement text (12662).
- Indent = `min(lhscontrollevel, controllevel)`, +1 inside `TYPE` or `DECLARE LIBRARY` (12672–12677). `lhscontrollevel` is
  the control depth at the start of the line; ELSE/ELSEIF/CASE/`$ELSE` decrement it.
- `fix046$` replacements are reversed (12670).
- `apply_layout_indent$` (`format.bas`) converts `sp`/`sp2`, applies indent, and when `AutoLayout` is off it merges only
  the keyword-case changes into the user's own spacing.
- IDE gets `layout$` back; `-y` mode writes to `format.out` (12684–12692) and copies it to the `-o` path (13648).

Consequence: a statement that parses but whose handler forgot `layoutdone`, or any line with `_` continuation, is passed
through unformatted. A formatter in the rewrite has to reproduce each handler's spacing decisions to stay byte-compatible.

---

## 4. The two passes

### 4.1 Prepass (1802–3238)

Per line (`ideprepass`):

1. Auto-include check (1814–1819).
2. Raw-text preprocessor checks on the trimmed, upper-cased line (1836–2100) — see §5.
3. `lineformat` (2102), then skip a leading line number or `label:` (2116–2136).
4. Split on `:`, `ELSE`, `THEN` into statements (`ppblda`, 2143–3084). For each statement:

| Collected | Lines | Notes |
|---|---|---|
| `END SUB/FUNCTION` | 2155 | Resets `subfuncn=0` so CONST scoping works |
| `DECLARE LIBRARY` body | 2160–2173, 2751–2758 | Only SUB/FUNCTION lines allowed inside |
| `OPTION BASE` | 2179–2187 | Tracked separately in `udtparse_optionbase` for TYPE member arrays |
| `TYPE … END TYPE` | 2190–2524 | Full UDT registration, sizes, member arrays, hash entries, `g_tmp_udt_<NAME>` SWAP buffer in `global.txt` |
| `CONST` | 2530–2732 | Evaluated with `Evaluate_Expression$`; stored in `const*()` arrays; hash `HASHFLAG_CONSTANT`. Redefinition with the same value in the same scope is a warning, not an error |
| `DEFINT/DEFLNG/DEFSNG/DEFDBL/DEFSTR/_DEFINE` | 2736–2748 | Jumps into the *main-pass* code at `predefine` (6110) with `predefining=1` and jumps back via `predefined:` (2746) |
| `SUB` / `FUNCTION` headers | 2760–3074 | Registers an id with arg types, sizes, return type, callname (`SUB_NAME` / `FUNC_NAME` or the ALIAS), `hr_syntax` help text |

Things the prepass does **not** do: variables, labels, DIM, any code generation other than `global.txt` lines for TYPEs.

Special names in the prepass:
- `_IKW_name` prefix registers the routine as an internal keyword `name` (3026–3029, 3065) — used by the auto-included support files.
- `SUB _GL` with no parameters sets `UseGL` and `DEPENDENCY_GL` (3066).
- `DECLARE SUB/FUNCTION` (QB-style forward declarations) is ignored in both passes (4443–4445, 5320).

After the loop: constants are marked undefined again (3263) so the main pass can enforce define-before-use;
`defineaz` is reset to SINGLE (3265); `$IF` defines are reset (3260).

### 4.2 Main pass (3337–12694)

Per physical line (`compileline`):

1. Apply `$STATIC/$DYNAMIC` from the previous line (3361–3362).
2. Read line, auto-include check (3365–3379), progress bar (3386–3405).
3. Skip if `InvalidLine(linenumber)` — set by the prepass for `$IF`-excluded lines (3411).
4. `$` metacommands on raw text (3427–3904).
5. `lineformat` (3914); rewrite `CASE ELSE` → `CASE C-EL` (3920–3938).
6. Line number (3942–3995) and/or `label:` (4000–4054): register label, emit `LABEL_x:;`, `last_line=`, event check.
7. Statement splitting (4063–4178) — see §4.3.
8. `gotcommand` (4181): period fix-up, then the handler chain §7.
9. `finishedline` (12461): emit the per-statement event/error check. `finishednonexec` (12491): Include Manager #2,
   then loop for `continuelinefrom`, then finalize layout.

### 4.3 Single-line IF and `:` splitting (4093–4178)

The splitter re-enters `compileline` once per statement, carrying `continuelinefrom` (element index to resume at).

- `:` ends a statement.
- `THEN` (or `GOTO` directly in an `IF`) ends the IF header. If anything follows on the line, `endifs` is incremented:
  the IF is treated as a block IF and synthetic `END IF`s are generated at end of line (4172–4178) with `impliedendif=1`
  so they do not appear in the layout.
- `ELSE` mid-line: flush what precedes it; nested single-line `IF…ELSE…ELSE` is resolved with `lineelseused` by
  injecting an `END IF` first (4141–4164).
- `THEN 100` / `ELSE 100`: `THENGOTO=1` lets a lone number be a GOTO target (9686–9691); the lexer inserts `GOTO` after `ELSE`.

### 4.4 The `.` hack (`fix046$`)

QB allows periods in ordinary names. The lexer always splits on `.`; afterwards, for each `a . b` the main pass looks
up `a` with `findid`. If `a` is a UDT variable/array, or the preceding element is `)`, the period stays. Otherwise the
two elements are fused with the literal text `__ASCII_CHR_046__` (4196–4241; constant at 475–476). The prepass does the
same blindly for CONST and SUB/FUNCTION lines (2536–2545, 2777–2786). The marker shows up in C names and must be reversed
for layout, warnings and the variable list. It also interacts with the 40-char name limit (28259–28265).

### 4.5 `$INCLUDE` and auto-includes

`$INCLUDE` is not processed where it appears. The lexer records `addmetainclude$`; after the *whole physical line* is
finished, "Include Manager #1" (prepass, 3094–3221) or "#2" (main pass, 12497–12647) opens the file and feeds its lines
back by `GOTO ideprepass` / `GOTO compileline` (or by IDE message 10 so the IDE drives). The two managers are
near-duplicates.

- Lookup: relative to the including file's directory, then as given (cwd = qb64pe folder). Auto-includes skip the relative try.
- `linenumber` stays at the including line; `inclinenumber(inclevel)` counts inside the file; `incerror$` builds the
  " in line N of F included (through A then B)" suffix.
- `$INCLUDEONCE`: implemented by reading the whole file as binary, upper-casing it, and searching for the text
  `$INCLUDEONCE` followed by a line ending at file start or after an LF (3142–3164, 12557–12581). Full paths of every
  included file are kept in the `incone.txt` buffer; a second include of a file containing the marker is skipped.
  In the main pass the metacommand itself is a no-op (3547–3551). Only the last `$INCLUDE` on a line counts.

Auto-include manager (`autoIncludeManager`, 1725–1799), invoked by `GOSUB` at three source positions:

| Position | Trigger | Files (in order) |
|---|---|---|
| Before first line | `firstLine=1` | `internal\support\include\beforefirstline.bi`; `color0.bi` or `color32.bi` if `$COLOR`; each used library's `IncAtTop` (reverse order); `vwatch\vwatch.bi` if `$DEBUG` |
| After main module | `mainEndLine=1` — in this tree it is set only together with `lastLine=1` at EOF (1812, 3369; no other assignment exists), so "after main" files are in practice included at end of file, just before the bottom files | `include\aftermain.bas`; libraries' `IncAfterMain` |
| After last line | `lastLine=1` | marker `-----`; `vwatch.bm` or `vwatch_stub.bm`; libraries' `IncAtBottom`; `include\afterlastline.bm` |

The manager writes file names to the `autoinc.txt` buffer and then jumps (`ON … GOSUB autoInclude, autoInclude_prepass`,
1792) *into the middle of* the corresponding include manager; that code `RETURN`s when the buffer is drained (3218, 12643).
State values 1 → 2 → 3 for `firstLine`, `mainEndLine`, `lastLine`. `setPrecompFlags` (1713–1723) publishes
`_EXPLICIT_ _EXPLICITARRAY_ _ASSERTS_ _CONSOLE_ _DEBUG_ _SOCKETS_` for `$IF` use in those files.

Name rule tied to this (`validname`, 28269–28285): user code may not define names with a single leading underscore;
`beforefirstline.bi`/`afterlastline.bm` are exempt, and when those two files are edited directly in the IDE, TYPE/CONST/
SUB names are *required* to start with `_`.

---

## 5. Preprocessor and metacommands

Two families with different syntax:

- **`$` at start of line** — matched on the raw, trimmed, upper-cased line by string comparison, before lexing.
- **Inside a comment (`'$X` / `REM $X`)** — found by the lexer (legacy QB style).

| Metacommand | Prepass | Main pass | Effect |
|---|---|---|---|
| `$IF expr THEN` | 1836–1851 | 3440–3475 | Pushes `ExecLevel/DefineElse`; main pass also pushes control type 6 for indent |
| `$ELSE IF expr THEN` (also `$ELSEIF`) | 1865–1885 | 3491–3518 | |
| `$ELSE` | 1853–1863 | 3477–3489 | |
| `$END IF` / `$ENDIF` | 1887–1892 | 3430–3438 | |
| `$LET name = value` | 2037–2069 | 3525–3534 | `SetPreLET`. Value chars limited to ASCII 48–90 plus `.` and a leading `-`; quotes stripped |
| `$ERROR text` | 2031–2035 | — | Compile error "Compilation check failed: …" |
| `$COLOR:0` / `$COLOR:32` | 1903–1913 | 3536–3545 | `ColorSet` state var → immediate recompile → auto-includes color constant file. Both together is an error |
| `$USELIBRARY:'author/lib'` | 1915–1976 | 3821–3827 | Reads `libraries/descriptors/<lib>.ini` `[LIBRARY INCLUDES]` keys `IncAtTop`, `IncAfterMain`, `IncAtBottom`; registers and recompiles |
| `$ASSERTS` / `$ASSERTS:CONSOLE` | 1978–1990 | 3604–3613 | `AssertsOn` (and `ConsoleOn`); emits `int32 asserts=1` |
| `$CONSOLE` | 1992–1996 | 3590–3594 | `ConsoleOn=1` → `DEP_CONSOLE` |
| `$CONSOLE:ONLY` | 1998–2002 | 3595–3602 | `ConsoleOn=2`, `DEPENDENCY_CONSOLE_ONLY`; emits `sub__dest/sub__source(func__console())` at that point in the program |
| `$DEBUG` | 2004–2008 | 3553–3559 | `vWatchOn`; warns outside the IDE |
| `$ERRORLOCATION:ON/OFF` | 2010–2018 | 3575–3588 | Whole-program; `ON` emits `error_track_line(...)` before every statement (6201–6209) |
| `$NOPREFIX` | 2020–2029 | — | Removed feature: error (CLI) or offer to convert (IDE) |
| `$UNSTABLE:MIDI|HTTP|TYPEFIELDS` | 2073–2090 | 3883–3902 | Sets `unstableFlags()`; MIDI/HTTP only warn "no longer required" |
| `$MIDISOUNDFONT:` | 2092–2095 | — | Error: deprecated |
| `$INCLUDEONCE` | (file scan) | 3547–3551 | See §4.5 |
| `$CHECKING:OFF/ON` | — | 3561–3573 | `CheckingOn`: removes per-statement event/error checks, `S_n` labels, array bounds checks |
| `$SCREENHIDE` / `$SCREENSHOW` | — | 3615–3624 | `ScreenHideOn` → `screen_hide_startup` |
| `$RESIZE:OFF/ON/STRETCH/SMOOTH` | — | 3626–3645 | `ResizeOn`, `ResizeScale` → `dyninfo.txt` |
| `$VERSIONINFO:key=value` | — | 3647–3741 | Keys: `FILEVERSION#`, `PRODUCTVERSION#` (must be `n,n,n,n`), `CompanyName`, `FileDescription`, `FileVersion`, `InternalName`, `LegalCopyright`, `LegalTrademarks`, `OriginalFilename`, `ProductName`, `ProductVersion`, `Comments`, `Web`. Warns if value lacks `'…'` |
| `$EMBED:'file','handle'` | — | 3743–3819 | Registers file; only embedded if `_EMBEDDED$("handle")` is used |
| `$EXEICON:'file'` | — | 3829–3881 | Once only; `DEPENDENCY_ICON`; emits `sub__icon(NULL,NULL,0);` |
| `'$INCLUDE:'file'` | lexer | lexer | 25467–25487 |
| `'$STATIC` / `'$DYNAMIC` | lexer | lexer, applied next line | `DynamicMode` 0/1 (25461–25466, 3361–3362) |
| `'$FORMAT:ON/OFF` | lexer | lexer | Toggles IDE auto indent/layout (25454–25459) |

Not handled anywhere in the front end as far as I found: no `$DEFINE`, `$MACRO`, etc.

**`$IF` evaluation (`EvalPreIF`, 28352–28536).** Text substitution, not a parser. Repeatedly finds the leftmost
`= < >` operator, takes the whitespace-delimited word on each side, compares, replaces with ` -1 ` or ` 0 `; then
folds ` AND ` / ` OR ` / ` XOR ` strictly left to right. No parentheses, no precedence, no `NOT`. Special words
`DEFINED` / `UNDEFINED` on the right side. `VERSION` comparisons use `CompareVersions` (dotted numeric). Numeric vs.
string comparison is decided by `VerifyNumber`. A bare name is true when its value is neither `"0"` nor `""`.
Nesting depth is limited to 255 (`ExecLevel(255)`).

---

## 6. Core data structures

### 6.1 Identifier table

`TYPE idstruct` (560–615), stored in `ids(1 TO ids_max)` (doubles from 1024, 25837–25843). `id` is a single shared
scratch record; `currentid` is the index last found/registered. `clearid` copies `cleariddata`.

| Field | Type | Meaning |
|---|---|---|
| `n` | `STRING*256` | Upper-case name |
| `cn` | `STRING*256` | Name as written |
| `arraytype` | LONG | Type bits if this id is an array (else 0) |
| `arrayelements` | INTEGER | Dimension count; -1 = not yet known |
| `staticarray` | INTEGER | Main-module array with constant bounds |
| `dynudt`, `dynudtmode` | INTEGER | Fork: descriptor-aware UDT lifecycle / layout mode |
| `mayhave` | `STRING*8` | Suffix that *may* be written (variables declared `AS type`) |
| `musthave` | `STRING*8` | Suffix that *must* be written (implicit / suffix-declared variables). Mutually exclusive with `mayhave` |
| `t` | LONG | Type bits if a scalar variable (else 0) |
| `tsize` | LONG | Fixed string length (other uses not traced) |
| `subfunc` | INTEGER | 1 = FUNCTION, 2 = SUB |
| `Dependency` | INTEGER | `DEPENDENCY_*` to set when used |
| `internal_subfunc` | INTEGER | Built-in (registered with `reginternalsubfunc`) |
| `callname` | `STRING*256` | C name. For library functions may carry a cast prefix `(  ctype  )name` removed by `removecast$` (28218) |
| `ccall` | INTEGER | Direct C call (library or runtime) |
| `overloaded`, `args`, `minargs` | | Argument counts |
| `arg` | `STRING*400` | 100 × `MKL$(type)` |
| `argsize` | `STRING*400` | 100 × `MKL$(fixed string size)` |
| `specialformat` | `STRING*256` | Custom syntax template, e.g. `[{READ|WRITE}]` |
| `secondargmustbe` / `secondargcantbe` | `STRING*256` | Disambiguate overloads by the 2nd token (`DEF SEG`, `SHELL _HIDE`, `VIEW PRINT`…) |
| `ret` | LONG | Return type |
| `insubfunc`, `insubfuncn` | `STRING*256`, LONG | Owning procedure name / number |
| `share` | INTEGER | bit 1 = `DIM SHARED`; bit 2 = temporarily shared via `SHARED` statement (cleared at END SUB, 6030–6032) |
| `nele`, `nelereq` | `STRING*100` | Per-argument array dimension count: declared / required by callers |
| `dynudtargmode` | `STRING*100` | Fork: per-argument UDT layout mode |
| `linkid`, `linkarg` | | Not traced |
| `staticscope` | INTEGER | `STATIC` variable |
| `sfid`, `sfarg` | | For a parameter variable: owning procedure id and argument number |
| `hr_syntax` | STRING | Human-readable syntax for IDE help |

Parallel per-id arrays: `cmemlist()` (must be in conventional memory), `sfcmemargs()`, `arrayelementslist()` (623–625).
The 100-argument limit (2906) comes from the fixed-width fields.

### 6.2 Hash table (`utilities\hash.bi`, `hash.bas`)

- `HashTable(16777215) AS LONG` — a 64 MB direct-index table (`hash.bi:43`), plus `HashList()` chain items and
  `HashListName()` (`STRING*256`).
- Hash = packed 5-bit codes of first, second, last, second-to-last characters + `(len AND 7)` + underscore bit
  (`hash.bas:2–40`). Names are case-insensitive (stored upper-case).
- API: `HashAdd name, flags, ref`; `HashFind`/`HashFindRev` return 0 = none, 1 = found and last, 2 = found and more may
  follow; `HashFindCont` continues using hidden globals (`HashFind_NextListItem` etc.) — **not re-entrant**;
  `HashRemove` removes the last found; `HashClear`.

| Flag | Value | Reference points to |
|---|---|---|
| (all ids) | 1 | `ids()` index — `regid` always sets bit 0; `findid` searches flag 1 (25874, 23314) |
| `HASHFLAG_LABEL` | 2 | `Labels()` |
| `HASHFLAG_TYPE` | 4 | built-in type word |
| `HASHFLAG_RESERVED` | 8 | reserved word |
| `HASHFLAG_OPERATOR` | 16 | |
| `HASHFLAG_CUSTOMSYNTAX` | 32 | words used inside custom syntax (`AS`, `TO`, `STEP`, `USING`…) |
| `HASHFLAG_SUB` / `FUNCTION` | 64 / 128 | `ids()` |
| `HASHFLAG_UDT` | 256 | UDT index |
| `HASHFLAG_UDTELEMENT` | 512 | owning UDT index |
| `HASHFLAG_CONSTANT` | 1024 | `const*()` index |
| `HASHFLAG_VARIABLE` / `ARRAY` | 2048 / 4096 | `ids()` |
| `HASHFLAG_XELEMENTNAME` | 8192 | reserved word may not be a TYPE element name |
| `HASHFLAG_XTYPENAME` | 16384 | reserved word may not be a TYPE name |

Reserved-word registration with per-word exceptions: 1254–1389.

### 6.3 Name lookup rules (`findid`, 23254–23411; `regid`, 25834–26066)

- A name may map to several ids. Callers loop: `try = findid(n$)`; while `try = 2`, set `findanotherid = 1` and call again.
- Scope: an id matches if it is a SUB/FUNCTION, or `share <> 0`, or `insubfunc` equals the current `subfunc` (23332–23334).
- Suffix: `musthave` ids match only if the same suffix was written; `mayhave` ids match with no suffix or the same one.
  `name$` matches a fixed-length string id declared as `$10` (23381–23387). `` ` `` normalizes to `` `1 ``.
- `findid` has a side effect: it marks the variable as used for unused-variable warnings (23397–23398).
- `regid` conflict rules (25908–26061): same name may coexist as scalar and array; as different types when declared by
  suffix; with a reserved word when the variable is a `$` string; with an internal `$` function when numeric; never with
  a user SUB/FUNCTION. Hard-coded exception: a variable named `WIDTH` (25934, 26014).
- `FindArray` (23413–23497) tries: as written (local) → with DEFxxx suffix (local) → as written (global) → with suffix (global).

### 6.4 UDT tables (`type.bi:73–86`, re-dimmed at 1429–1527)

Types are numbered `1..lasttype`; type 1 is always `_MEM`, created by hand at 1588–1653 (members `OFFSET`, `SIZE`,
`$_LOCK_ID`, `$_LOCK_OFFSET`, `TYPE`, `ELEMENTSIZE`, `IMAGE`, `SOUND`).

| Array | Meaning |
|---|---|
| `udtxname`, `udtxcname` | Name upper / as written |
| `udtxsize` | Size **in bits** |
| `udtxnext` | First element index |
| `udtxvariable` | Contains variable-length strings |
| `udtxfdynsize`, `udtxcanonmode` | Fork: descriptor-layout size / mode |
| `udtename`, `udtecname` | Element name |
| `udtesize` | Element size in bits (× element count for member arrays) |
| `udtetype`, `udtetypesize` | Type bits, fixed string length |
| `udtenext` | Next element (0 = end) |
| `udtearrayelements/base/dims/desc/fieldmode`, `udtefdynoffset/size` | Fork: member arrays; `desc` is `"lower,count;lower,count"` |

Rules enforced in the prepass: no `_BIT` members (2340); no self-reference (2291); max `UDTMASK` = 4095 types (2485);
a type must be defined before use (lookup is a linear scan by name, `type.bas:712–717`). Two member syntaxes:
`name AS type` and `AS type name1, name2` (2210 vs. 2364). Element linking uses `udtenext(i2 - 1) = i2` (2358), which
assumes elements of one TYPE are allocated contiguously.

### 6.5 Arrays

An array is an id with `arraytype <> 0`. `arrayelements` = number of dimensions, or -1 while unknown (array parameters
`a()` and COMMON arrays). The runtime descriptor is a `ptrszint*` (parameters are emitted as `ptrszint*name`, 5689);
`name[0]` is the data pointer and `name[2] & 1` means "defined" (13350, 13369). Descriptor creation is in `allocarray`
(14897) and `dim2` (17624) — covered by the code-generation study. `staticarraylist`/`commonarraylist` are `sp`-separated
strings of (name, type, dimmethod, dimshared) tuples.

### 6.6 Labels (430–447)

`Label_Type`: `State` (0 referenced, 1 defined), `cn`, `Scope` (procedure number, 0 main, -1 "not yet bound"),
`Data_Offset`, `Data_Referenced`, `Error_Line`, `Scope_Restriction`, `SourceLineNumber`.

- Line numbers and alphanumeric labels share one table. `validlabel` (27121–27249) canonicalizes: numeric labels may be
  like `1.5`, `10#`, `10!` and become `1p5`, `10d`, `10s`; dotted labels `a.b` are joined with `fix046$`.
  It rejects reserved words and internal SUB names.
- Labels are scoped per procedure. Every reference site (GOTO 9699–9725, GOSUB, RETURN, RESUME, RUN, ON ERROR, RESTORE,
  ON…GOTO) repeats the same find-or-create block.
- `ON ERROR GOTO` and `RUN label` targets must be in the main module (`Scope = 0`, `Scope_Restriction` checked at 13013–13024).
- `RESTORE label` is scope-less (`Scope = -1`) and must be unique program-wide (13041–13053).
- Undefined labels are reported after the pass (13027–13038).
- Ambiguity `name:` vs. a call to a parameterless SUB followed by `:`: first guess is "sub call" and the name is recorded
  in `PossibleSubNameLabels`; if a GOTO to it is left unresolved the compiler adds it to `SubNameLabels` and recompiles (27158–27160, 13029–13034).

### 6.7 Constants (`hash.bi:66–83`)

Parallel arrays `constname`, `constcname`, `constnamesymbol`, `consttype`, `constinteger`, `constuinteger`, `constfloat`,
`conststring`, `constsubfunc` (scope), `constdefined`. Values are computed only in the prepass by
`utilities\const_eval.bas` — a separate recursive-descent evaluator over elements with its own function list
(`Set_ConstFunctions`, `const_eval.bas:815–869`: trig, `_PI`, `ABS`, `SGN`, `INT`, `_ROUND`, `FIX`, `_RGB*`, color
component functions, `CHR$`, `ASC`). The main pass only re-parses for layout and sets `constdefined` (6047–6108).
Lookup is innermost-first with `HashFindRev` and requires `constdefined` (`const_eval.bas:714–757`).
The type of an unsuffixed CONST comes from the value, not from DEFxxx (2532–2533). "range check required here (noted in
todo)" at 2623 — a suffix does not range-check the value.

### 6.8 Scope, STATIC, SHARED, COMMON

| Concept | State | Notes |
|---|---|---|
| Current procedure | `subfunc` (C name, `""` = main), `subfuncn`, `subfuncid`, `subfuncnlast` | Numbering must be identical in both passes; prepass counts at 2770, main pass at 5419 |
| C name scoping | `scope$` = `module$ + "_" + subfunc$ + "_"`, or `module$ + "__"` when shared (26206–26209) | `module$` is declared (700) and never assigned, so it is always empty |
| `DIM SHARED` | `dimshared=1` → `id.share=1` | Main module only (8798) |
| `SHARED x` in a procedure | 8058–8310 | Sets bit 2 of `share` until END SUB; implicit creation honors OPTION _EXPLICIT (8186) |
| `STATIC` statement | `dimoption=3`, `dimstatic=1` | Procedure only (8789) |
| `SUB … STATIC` | `staticsf=2` → `dimstatic=2` for the whole body (5530–5535, 5827) | In the prepass the keyword is stripped and ignored (2794) |
| Static variable storage | `dim2` redirects declarations to `maindata.txt`/`mainfree.txt`/`global.txt` (17654–17660) | |
| `COMMON` | `commonoption=1`; main module only (8790) | Arrays recorded in `commonarraylist`; save/load code for `CHAIN` generated at 13145–13446 into `chainN.txt`/`inpchainN.txt` |
| Parameters | `dim2` with `dimsfarray=1`, `glinkid/glinkarg` | `AllowLocalName=1` lets a parameter shadow a global CONST (5539, 25955) |

### 6.9 Defaults and options

- `defineaz(1..27)` type name and `defineextaz(1..27)` suffix per initial letter; index 27 is `_`. Reset to `SINGLE`/`!`
  at the start of each pass. `DEFxxx` are rewritten into `_DEFINE … AS type` (6113–6117). They apply from their source
  position onward and are processed in both passes because SUB signatures depend on them.
- `OPTION BASE 0|1` → `optionbase` (11513–11518); prepass shadow `udtparse_optionbase`.
- `OPTION _EXPLICIT` → `OptExpl`; `OPTION _EXPLICITARRAY` → `OptExplArr`. Both are `RCStateVar`s, detected textually in the
  prepass (2099–2100) to avoid a late restart. Errors raised at 8186, 19611, 19834. `-e` forces `OptExpl`.
- `RCStateVar` (`statevars.bi`): `wanted`, `actual`, `locked`, `forced`. Once a value has been applied on a recompile it
  is locked for the rest of the compilation.

---

## 7. Type system

Types are a single LONG (`type.bi:6–61`).

| Flag | Value | Meaning |
|---|---|---|
| `ISSTRING` | 2^30 | String |
| `ISFLOAT` | 2^29 | Floating point |
| `ISUNSIGNED` | 2^28 | |
| `ISPOINTER` | 2^27 | Refers to storage (a variable) rather than a value. Stripped for BYVAL library args (2944) and function returns (3008–3010) |
| `ISFIXEDLENGTH` | 2^26 | Fixed-length string (size held separately in `tsize`/`typname2typsize`) |
| `ISINCONVENTIONALMEMORY` | 2^25 | Lives in emulated DOS memory (`cmem`) |
| `ISOFFSETINBITS` | 2^24 | `_BIT` types: array offsets are in bits |
| `ISARRAY` | 2^23 | |
| `ISREFERENCE` | 2^22 | Expression result is a reference string (id number, or id/UDT/element/offset joined by `sp3`) that must go through `refer$` |
| `ISUDT` | 2^21 | Low bits hold the UDT index |
| `ISOFFSET` | 2^20 | `_OFFSET` |
| low 12 bits (`UDTMASK` = 4095) | | Size in bits for primitives, or UDT index |

| QB type | Suffix | Value | C type (`typ2ctyp$`, `type.bas:425`) |
|---|---|---|---|
| `_BIT` / `_BIT * n` (n ≤ 64) | `` ` `` / `` `n `` | n + POINTER + OFFSETINBITS | `int32` (n ≤ 32) / `int64` |
| `_UNSIGNED _BIT` | `` ~` `` | + UNSIGNED | `uint32` / `uint64` |
| `_BYTE` | `%%` | 8 | `int8` |
| `_UNSIGNED _BYTE` | `~%%` | | `uint8` |
| `INTEGER` | `%` | 16 | `int16` |
| `_UNSIGNED INTEGER` | `~%` | | `uint16` |
| `LONG` | `&` | 32 | `int32` |
| `_UNSIGNED LONG` | `~&` | | `uint32` |
| `_INTEGER64` | `&&` | 64 | `int64` |
| `_UNSIGNED _INTEGER64` | `~&&` | | `uint64` |
| `SINGLE` | `!` | 32 + FLOAT | `float` |
| `DOUBLE` | `#` | 64 + FLOAT | `double` |
| `_FLOAT` | `##` | 256 + FLOAT | `long double` |
| `_OFFSET` | `%&` | 64 or 32 + OFFSET (by `TARGET_BITS`) | `ptrszint` |
| `_UNSIGNED _OFFSET` | `~%&` | | `uptrszint` |
| `STRING` | `$` | STRING + POINTER | `qbs` |
| `STRING * n` | `$n` | + FIXEDLENGTH | `qbs` |
| UDT | — | UDT + POINTER + index | `void` |
| `_MEM` | — | UDT #1 | |

Notes:
- `_BIT * n` size is encoded as `BITTYPE - 1 + n` (`type.bas:594, 693`).
- `_FLOAT` is "256 bits" as a tag; storage is `long double`. `SELECT CASE` treats size > 64 as long double (6848).
- `_OFFSET` suffixes are rejected on numeric literals (24706, 24715).
- `STRING * constname` is allowed if the CONST is already defined (`type.bas:617–658`).
- Type names are compared as upper-case strings with single spaces (`"_UNSIGNED LONG"`); `ANY` is reserved (1266).
- Conversions: `typname2typ&` (name or suffix → bits; sets global `typname2typsize`), `type2symbol$`,
  `typevalue2symbol$`, `symboltype` (sets global `symboltype_size`), `symbol2fulltypename$`, `removesymbol$`,
  `id2fulltypename$`, `id2shorttypename$`. Newer helpers `Type_*` at `type.bas:2174–2331`.
- `symboltype` returns 64 + FLOAT for `##` (`type.bas:370`), i.e. the same as `#`, while `typname2typ` maps `##` to
  `FLOATTYPE`. This looks like a latent inconsistency; I did not trace which callers depend on it.
- Numeric literal elements with no suffix are typed `_INTEGER64` by `elementGetNumericValue&` (`elements.bas:527`); any
  literal containing `.` or `D` is DOUBLE, `E` SINGLE, `F` `_FLOAT`.

---

## 8. Statement handling in the main pass

### 8.1 Control-structure stack (768–790)

`controllevel` indexes parallel arrays (size 1000):
`controltype` (kind), `controlid` (unique number for C labels), `controlvalue` (IF: count of extra `}` from ELSEIFs;
SELECT: id of a simple variable used directly), `controlstate` (IF: 1 after ELSEIF, 2 after ELSE), `controlref`
(source line, for "X without Y" errors).

| Type | Block | C labels |
|---|---|---|
| 1 | IF | braces only |
| 2 | FOR | `fornext_continue_<id>`, exit label |
| 3 / 4 | DO / DO WHILE·UNTIL | `dl_continue_<id>`, `dl_exit_<id>` |
| 5 | WHILE | `ww_continue_<id>` |
| 6 | `$IF` (layout only) | — |
| 10–17 | SELECT CASE on qbs, int64, uint64, long double, float, double, int32, uint32 | `sc_<id>`, `sc_<id>_end`, `sc_<id>_var` (EVERYCASE) |
| 18 | inside CASE | `sc_ec_<n>_end` |
| 19 | inside CASE ELSE | |
| 32 | SUB/FUNCTION (only when `IDEIndentSubs`) | |

Separate side stacks: `SelectCaseCounter`, `EveryCaseSet()`, `SelectCaseHasCaseBlock()` (51–52); `ExecLevel()`,
`DefineElse()`, `ExecCounter` for `$IF` (53–56). The "open block" check is written three times (5397–5408, 5935–5946,
12711–12724). Note 5406: the error path overwrites `linenumber` with the opening line.

### 8.2 Handler chain, in the order it is tested

The first matching handler wins and jumps to `finishedline` (executable) or `finishednonexec`.

**Non-executable section (4249–6191)**

| Statement | Lines | Notes |
|---|---|---|
| lone `'` | 4251–4253 | no-op |
| `DATA` | 4255–4275 | Layout only; data already captured by the lexer |
| inside `DECLARE LIBRARY`: `END DECLARE`, SUB/FUNCTION | 4279–4297 | |
| inside `TYPE`: members, `END TYPE` | 4299–4432 | Layout and re-validation only |
| `TYPE name` | 4434–4441 | |
| `DECLARE LIBRARY` / `DYNAMIC LIBRARY` / `CUSTOMTYPE LIBRARY` / `STATIC LIBRARY` | 4443–5318 | Library and header search (about 700 lines of per-OS path probing: `.lib .a .o .dll .so .dylib .h .hpp`, system32, `/usr/lib`, `/usr/lib64`, `/usr/include`, source folder); version suffix `name:1.0`; emits `#include`, `LoadLibrary`/`dlopen` code |
| `DECLARE SUB/FUNCTION` (QB forward decl) | 4445, 5320 | Ignored |
| `SUB` / `FUNCTION` header | 5324–5922 | Requires prepass registration ("Unregistered SUB/FUNCTION encountered"); opens `mainN/dataN/freeN/retN.txt`; emits prototype to `regsf.txt` and definition header; creates parameter ids; `ALIAS`, `BYVAL`, `STATIC` |
| `END SUB` / `END FUNCTION` | 5924–6043 | Emits epilogue, switches buffers back to main, un-shares temp shared ids, reverts `revertmaymusthave` |
| `CONST` | 6047–6108 | Layout + mark defined |
| `DEFINT` `DEFLNG` `DEFSNG` `DEFDBL` `DEFSTR` `_DEFINE` | 6110–6191 | Shared with prepass |

**Executable section (6193–12459).** `statementn` is incremented per statement (6195).

| Statement | Lines |
|---|---|
| `NEXT [var[, var…]]` | 6212–6272 |
| `WHILE` | 6274–6309 |
| `WEND` | 6311–6328 |
| `DO [WHILE|UNTIL]` | 6330–6376 |
| `LOOP [WHILE|UNTIL]` | 6378–6433 |
| `FOR … = … TO … [STEP …]` | 6435–6592 |
| `ELSE` | 6594–6641 (includes a raw-text sanity check on `wholeline`, 6603–6624) |
| `ELSEIF … THEN` | 6643–6683 |
| `IF … THEN` / `IF … GOTO` | 6685–6740 (string condition is an error) |
| `ENDIF` / `END IF` | 6742–6784 |
| `SELECT CASE` / `SELECT EVERYCASE` | 6788–6895 |
| `END SELECT` | 6898–6937 (warns on empty block) |
| guard: code between SELECT and first CASE | 6939–6942 |
| `CASE` (list, `a TO b`, `IS op x`, bare operator, `CASE ELSE`) | 6945–7211 |
| *(from here each statement is wrapped in `do{ … }while(r)` when checking is on, 7224–7234)* | |
| `PALETTE USING` | 7237–7278 |
| `KEY OFF/ON/LIST`, `KEY n, str`; `KEY(n) ON/OFF/STOP` falls through to sub call | 7281–7336 |
| `FIELD` | 7340–7443 |
| `EXIT DO/FOR/WHILE/SELECT/CASE` | 7445–7532 |
| `ON STRIG(…) GOSUB|sub` | 7534–7724 |
| `ON TIMER(…) GOSUB|sub` | 7726–7905 |
| `ON KEY(…) GOSUB|sub` | 7907–8056 |
| `SHARED` (in procedure) | 8058–8310 |
| `EXIT SUB/FUNCTION` | 8311–8328 |
| `_ECHO` guard (needs `$CONSOLE`) | 8332–8336 |
| `ASC(str[, pos]) = value` statement | 8339–8445 |
| `MID$(str, start[, len]) = value` statement | 8447–8527 |
| `ERASE` | 8529–8765 |
| `DIM` / `REDIM [_PRESERVE|_RETAIN]` / `STATIC` / `COMMON` [`SHARED`], both `name AS type` and `AS type list` forms | 8767–9685 |
| `THEN <number>` implied GOTO | 9685–9692 |
| `GOTO` | 9694–9732 |
| `_CONTINUE` | 9734–9756 |
| `CHAIN` (warning only, then falls to sub call) | 9758–9762 |
| `RUN [label|file$]` | 9764–9840 |
| `END [code]` | 9846–9868 |
| `SYSTEM [code]` | 9870–9902 |
| `STOP [n]` | 9904–9926 |
| `GOSUB` | 9928–9935 (`xgosub`, 27574) |
| `RETURN [label]` | 9937–9984 |
| `RESUME [NEXT|0|label]` | 9986–10042 |
| `ON ERROR GOTO [_NEWHANDLER] label | 0 | _LASTHANDLER` | 10044–10114 |
| `RESTORE [label]` | 10116–10154 |
| `ON expr GOTO|GOSUB list` | 10158–10165 (`xongotogosub`, 27618) |
| `_MEMGET` | 10170–10265 |
| `_MEMPUT` | 10267–10404 |
| `_MEMFILL` | 10406–10519 |
| `_ARRAYCOPY` (fork) | 10521–10600 |
| `INTERRUPT` / `INTERRUPTX` rewritten to `CALL …` | 10603–10615 |
| `CALL name[(args)]`, `CALL INTERRUPT[X]`, `CALL ABSOLUTE` | 10617–10806 |
| **Generic SUB call** (any id with `subfunc = 2`, built-in or user); `?` → `PRINT` | 10808–12429 |
| `LET` | 12433–12443 |
| assignment `x = expr` (needs ` = ` element) | 12445–12455 (`assign`, 17343) |
| otherwise | `Syntax error` 12459 |

Special cases embedded in the generic SUB-call path (they are found by `findid` first, then intercepted by name):

| Name | Lines |
|---|---|
| reject direct `_GL`, `VWATCH` | 10863–10869 |
| `OPEN` (QB vs. GW-BASIC form selects a different overload) | 10871–10888 |
| `CLOSE` / `RESET` | 10893–10970 |
| `READ` | 10972–11015 (`xread`, 27936) |
| `LINE INPUT` / `INPUT #` | 11017–11121 |
| `INPUT` | 11123–11239 |
| `WRITE #` / `WRITE` | 11241–11255 (`xfilewrite`, `xwrite`) |
| `PRINT #` / `PRINT` / `LPRINT` (`USING`, `;`, `,`, `TAB`, `SPC`) | 11257–11316 (`xfileprint`, `xprint`) |
| `CLEAR` | 11318–11325 |
| `LSET` / `RSET` | 11327–11375 |
| `SWAP` | 11377–11505 |
| `OPTION BASE/_EXPLICIT/_EXPLICITARRAY` | 11507–11535 |
| `_LOGTRACE/_LOGINFO/_LOGWARN/_LOGERROR` | 11537–11559 |
| everything else: `seperateargs` against `specialformat`, per-argument evaluation and by-reference passing | 11561–12389 |
| `GET` / `PUT` (file, `FIELD` and UDT variants → `sub_get2/sub_put2`, `field_get/put`) | 11881–11965 |
| `_WAVE`, `_SNDRAWBATCH` argument tweaks | 11810, 11830 |
| `SLEEP` (debugger hooks) | 12391–12410 |

Not implemented: `DEF FN` (the id `Def` is a stub unless followed by `SEG`, `subs_functions.bas:70, 3142–3148`),
`TRON/TROFF`, `ON COM/PEN/PLAY/UEVENT` (words reserved only).

### 8.3 What each executable statement emits around itself

With `$CHECKING:ON` (default):
- Simple statements: `do{ <code> if(!qbevent)break; evnt(<line>[,incline,"incfile"]);}while(r);` (7229–7231, 12487).
  `r` set by `evnt` re-executes the statement (RESUME).
- Block-opening statements (IF, ELSEIF, SELECT, CASE, FOR, DO, LOOP, WHILE) cannot be wrapped in `do{}`; they emit a
  label `S_<statementn>:;`, set `dynscope=1`, and end with `if(qbevent){evnt(...);if(r)goto S_<n>;}` (12485).
- Conditions get `||is_error_pending()` appended (6725–6727, 7200–7202).
- `stringprocessinghappened` wraps the expression in `qbs_cleanup(qbs_tmp_base, …)`.
- With `$DEBUG`: `VWATCH_LABEL_<line>` labels, a `SUB_VWATCH(...)` call per line, and dispatch `switch`es.

---

## 9. Errors, warnings, global flags

### 9.1 Error reporting

Three mechanisms coexist:

1. **Main module:** `a$ = "message": GOTO errmes`.
2. **Inside SUB/FUNCTIONs:** `Give_Error msg` sets `Error_Happened`/`Error_Message` and the routine exits; *every* caller
   must then test `IF Error_Happened THEN GOTO errmes` (or `EXIT …`). There are hundreds of these checks; a missed one
   means compilation continues on garbage.
3. **Runtime errors of the compiler itself:** `ON ERROR GOTO qberror` (14236). `qberrorhappened` set to -1/-2/-3 before a
   risky `OPEN` turns the trap into a `RESUME` at `qberrorhappened1/2/3` (1681, 12554, 3139). `ON ERROR GOTO qberror_test`
   with flag `E` is used for "try KILL / try OPEN" (14232–14234). Any other trap becomes
   `UNEXPECTED INTERNAL COMPILER ERROR!` (14297) or, in the IDE, "Compiler error (check for syntax errors) (…)".

`errmes` (14308–14352): invalidates layout; appends `CHR$(1) + incerror$` when inside an include; rewrites some messages
when the error is in an auto-included file (14313–14318); IDE gets message 8 with `ideerrorline`; CLI prints the message,
`Caused by (or after):` + `linefragment` (the current statement's elements), and `LINE n:` + `wholeline`, then exits 1.
Only the first error is ever reported. No column information exists.

Line attribution: `linenumber` is the main-file line (the `$INCLUDE` line while inside an include). Post-pass errors set
`linenumber` manually from `controlref`, `Labels().Error_Line`, `definingtypeerror`, `ExeIconSet`.

### 9.2 Warnings (`addWarning`, 28668–28731)

| Warning | Where |
|---|---|
| Duplicate constant definition | 2679 |
| `$DEBUG features only work from the IDE` | 3556 |
| `$DEBUG features won't work in these blocks` (`$CHECKING:OFF`) | 3565 |
| Missing string bracket delimiters in `$VERSIONINFO` | 3722 |
| `$UNSTABLE:MIDI/HTTP` no longer required | 3891, 3895 |
| Empty SELECT CASE block | 6927 |
| Feature incompatible with `$DEBUG` (`CHAIN`, `RUN`) | 9760, 9766 |
| Unused variable | 13466–13489 |

CLI prints them only with `-w`; the IDE stores them in `warning$()` grouped by header. In windowed CLI mode any warning
makes the process exit with code 1 (14228).

### 9.3 Global state inventory (main groups)

| Group | Variables |
|---|---|
| Mode | `idemode`, `NoIDEMode`, `ConsoleMode`, `FormatMode`, `NoCCompileMode`, `QuietMode`, `ShowWarnings`, `MonochromeLoggingMode`, `ForceOptExpl`, `Debug` |
| Pass | `prepass`, `idepass`, `recompile`, `iderecompile`, `firstLine`, `mainEndLine`, `lastLine`, `lastLineReturn` |
| Position | `linenumber`, `reallinenumber`, `totallinenumber`, `inclevel`, `incname()`, `inclinenumber()`, `incIsInternal()`, `wholeline`, `linefragment`, `continuelinefrom`, `statementn` |
| Line splitter | `endifs`, `lineelseused`, `impliedendif`, `newif`, `THENGOTO`, `continueline` |
| Layout | `layout`, `layoutok`, `layoutdone`, `tlayout`, `layoutcomment`, `layoutoriginal$`, `layoutcontinuations`, `linecontinuation`, `lhscontrollevel` |
| Scope | `subfunc`, `subfuncn`, `subfuncnlast`, `subfuncid`, `subfuncret$`, `definingtype`, `declaringlibrary` (0/1/2), `dynamiclibrary`, `customtypelibrary`, `indirectlibrary`, `staticlinkedlibrary`, `sfdeclare`, `sfheader` |
| Declaration modifiers ("hidden parameters" of `dim2`/`regid`) | `dimshared`, `dimstatic`, `dimoption`, `redimoption`, `commonoption`, `dimmethod`, `dimsfarray`, `glinkid`, `glinkarg`, `sf_udt_dynmode`, `reginternalsubfunc`, `reginternalvariable`, `AllowLocalName`, `autoIncForceUScore` |
| Lookup protocol | `id`, `currentid`, `findanotherid`, `findidsecondarg`, `findidinternal`, `HashFind_*` |
| Expression side channels | `stringprocessinghappened`, `arrayprocessinghappened`, `inputfunctioncalled`, `dynscope`, `typname2typsize`, `symboltype_size`, `dim2typepassback`, `udt_allow_bare_array`, `asg_*`, `dynmemlockexpr` |
| Features | `CheckingOn`, `DynamicMode`, `optionbase`, `ScreenHideOn`, `ResizeOn`, `ResizeScale`, `UseGL`, `unstableFlags()`, `DEPENDENCY()`, the eight `RCStateVar`s |
| Counters for C names | `uniquenumbern`, `gosubid`, `errorlabels`, `everycasenewcase`, `nextrunlineindex`, `ontimerid`, `onkeyid`, `onstrigid`, `DataOffset` |
| Output buffer handles | `MainTxtBuf`, `DataTxtBuf`, `FreeTxtBuf`, `RetTxtBuf`, `GlobTxtBuf`, `RegTxtBuf`, `ErrTxtBuf`, `RunTxtBuf`, `ChainTxtBuf`, `InpChainTxtBuf`, `TimeTxtBuf`, `TimejTxtBuf`, `KeyTxtBuf`, `KeyjTxtBuf`, `StrigTxtBuf`, `StrigjTxtBuf`, `DataBinBuf`, `ExtDepBuf`, `IncOneBuf`, `FormatBuf`, `defdatahandle` |

`MainTxtBuf`, `DataTxtBuf`, `FreeTxtBuf`, `RetTxtBuf` and `RegTxtBuf` are *retargeted* at SUB entry/exit and in several
other places (5430–5433, 5446, 5496, 6023–6027, 12735, 13204…); every emitter writes to "whatever the handle currently
points at".

The main module runs under `DEFLNG A-Z` with implicit variables (`a$`, `e$`, `l$`, `i`, `x`, `n`, `t`…) reused across
thousands of lines; `E` is even reused as a loop scratch in `_DEFINE` parsing (6142) while also being the `qberror_test` flag.

---

## 10. Files written to `internal\temp[N]` and final assembly

All `OpenBuffer%` "files" live in memory until `WriteBuffers ""` (13644). Modes: `O` create/replace, `A` append,
`B`/`I` read (`sb_qb64pe_extension.bm:13–67`). `WriteBufLineCpp` prefixes generated code with `#line N "file"`
(`sb_qb64pe_extension.bm:237–290`).

### 10.1 Generated C++ fragments

| File | Opened | Content | Included by |
|---|---|---|---|
| `global.txt` | 1695 | Global declarations: `qb_safe_idiv/mod` templates, shared/static variables, `g_tmp_udt_*`, `data_at_LABEL_*`, `console`, `screen_hide_startup`, `asserts`, `vwatch`, `data_size`, `inline_data[]`, COMMON buffers, `vwatch_global_vars`, `extern` prototypes found via `nm` (appended directly on disk, 13826…) | `qbx.cpp:500` |
| `regsf.txt` | 3277 | Prototypes of user SUB/FUNCTIONs; library `#include`s, `DLL_*` handles, `DLLCALL_*`/`CUSTOMCALL_*` typedefs; `#include "externtypeN.txt"` | `qbx.cpp:501` |
| `regsf_ignore.txt` | 5446 | Sink for prototypes that a real header already declares | — |
| `externtypeN.txt` | 5450, 13859 | Empty or `extern "C" ` — linkage prefix decided after `nm` | `regsf.txt` |
| `dyninfo.txt` | 1196 | `ScreenResize=1;`, `ScreenResizeScale=n;` | `qbx.cpp:507` (`set_dynamic_info`) |
| `clear.txt` | 12735 | Body of `CLEAR`: reset every global/static variable and array | `qbx.cpp:513` |
| `main.txt` | 17592 | `#include "main0.txt"` … `"mainN.txt"`, plus `func__compdate/_comptime/_compvers` | `qbx.cpp:1606` (inside `QBMAIN`; main0 closes the function) |
| `main0.txt` | 3269 | Main module body; starts with `error_track_line(0,0,NULL);` and `S_0:;` | `main.txt` |
| `mainN.txt` | 5430 | One per SUB/FUNCTION N: full C++ function | `main.txt` |
| `maindata.txt` | 3273 | Main-module local declarations/initialization; also static variables of procedures and library-load code | `qbx.cpp:1588` |
| `dataN.txt` | 5431 | Locals of procedure N | `mainN.txt` (5788) |
| `mainfree.txt` | 3279 | Main-module cleanup | No `#include` of it exists under `internal\c` (grep) — appears to be written but unused |
| `freeN.txt` | 5432 | Cleanup of procedure N locals | `mainN.txt` (6011) |
| `ret0.txt`, `retN.txt` | 3311, 5433 | `RETURN` dispatcher: `switch(return_point[--next_return_point])` with `goto RETURN_<id>` cases; textually included at every plain `RETURN` (9940) | `mainN.txt` |
| `mainerr.txt` | 3282 | `ON ERROR` dispatcher: `if (error_goto_line==k){… goto LABEL_x;}` | `qbx.cpp:1589` |
| `runline.txt` | 3280 | `RUN label` from inside procedures | `qbx.cpp:1590` |
| `ontimer.txt` / `ontimerj.txt` | 3292–3293 | ON TIMER handler table / jump code | `qbx.cpp:1484`, `1593` |
| `onkey.txt` / `onkeyj.txt` | 3297–3298 | ON KEY | `qbx.cpp:1456`, `1597` |
| `onstrig.txt` / `onstrigj.txt` | 3300–3301 | ON STRIG | `qbx.cpp:1422`, `1601` |
| `chain.txt`, `chainN.txt` | 3289, 9369–9373, 13267 | Save COMMON data for `CHAIN` | `qbx.cpp:783` |
| `inpchain.txt`, `inpchainN.txt` | 3290, 9375–9379, 13204 | Load COMMON data | `qbx.cpp:578` |
| `vw_main_dispatch.txt`, `vw_main_skip.txt` | 3274–3275 | `$DEBUG` set-next-line / skip-line `switch` cases for the main module | `main0.txt` (17575, 17583) |
| `embedded.cpp` | 13655 | `$EMBED` data arrays (zlib-deflated when it saves ≥20 %) and `func__embedded` | compiled separately when `DEP_EMBED=y` |

### 10.2 Other files

| File | Purpose |
|---|---|
| `temp.bin` | Instance lock (file #26) |
| `tempfoldersearch.bin`, `checkpid.bin` | Linux temp-folder allocation |
| `data.bin` | Raw DATA text (comma-terminated items); re-read and emitted as `inline_data[]` in `global.txt` (13113–13143) |
| `autoinc.txt` | Buffer only: pending auto-include list |
| `incone.txt` | Buffer: full paths of included files |
| `extdep.txt` | External dependency list (`ICON:`, `DECL:`, `INCL:` + full path) |
| `format.out` | `-y` output before copy |
| `icon.rc`, `icon.ico`, `manifest.h`, `<exe>.manifest` | Windows resources (13551–13640) |
| `compilelog.txt` | make/g++ output |
| `nm_output_<lib>[_dynamic].txt`, `nm_error.txt` | Symbol dumps for header-less `DECLARE LIBRARY` |
| `debug_win.bat`, `debug_lnx.sh`, `debug_osx.command`, `recompile_lnx.sh`, `recompile_osx.command`, `log.command` | Helper scripts |
| `embed.bin` | Scratch for compressed embed data |
| `debug.txt` | Compiler trace when `Debug` |
| `ideerror.txt` | IDE crash log (hard-coded `internal\temp\`, 14265) |
| `<exe>.sym` | Written by the Makefile when stripping |

### 10.3 The translation unit

`internal\c\qbx.cpp` (or `qbxN.cpp`) is a fixed skeleton that `#include`s the fragments above from `../temp/`. User
code is therefore one C++ TU whose pieces are stitched by the preprocessor; `main0.txt` is included in the middle of
`QBMAIN` and supplies the closing brace. The Makefile makes `qbx.o` depend on `temp/*.txt` (Makefile:286) and the
compiler also deletes `qbx.o` before each build (13787–13794).

### 10.4 Header-less library symbol resolution (13777–14090)

For each `DECLARE [CUSTOMTYPE] LIBRARY` routine without a header, run `nm --demangle -g` (then `nm -D`) on the library,
text-search the output for ` name(` (C++ — copy the demangled signature into an `extern void …;`) or a line ending in
` name` (C — write `extern "C"`). Windows 32-bit also accepts ` _name`. macOS skips demangling. Duplicate matches are an error.

### 10.5 make invocation (13695–13774, 13796–13802, 13936, 13961–13967, 14167)

```
<make> [DEP_…=y …] [TEMP_ID=n] EXE=<escaped path>
       "CXXFLAGS_EXTRA=<flags>" "CFLAGS_EXTRA=<flags>" "CXXLIBS_EXTRA=<flags> <mylib$> <mylibopt$>"
       -j"<MaxParallelProcesses>" "BITS=<TARGET_BITS>" [STRIP_SYMBOLS=n] [GENERATE_LICENSE=y]
       ["USE_SYSTEM_MINGW=y"] OS=win|lnx|osx   1>> compilelog.txt 2>&1
```

`<make>` = `internal\c\c_compiler\bin\mingw32-make.exe` on Windows (bare name with system MinGW), `make` elsewhere;
Windows runs it through `cmd /c`. Flags: `-O2` if optimize; `-Og -g` if optimize + debug info; `-g` if debug info only.

| Front-end condition | make variable |
|---|---|
| `DEPENDENCY_GL` | `DEP_GL=y` |
| `DEPENDENCY_IMAGE_CODEC` | `DEP_IMAGE_CODEC=y` |
| `DEPENDENCY_CONSOLE_ONLY` | `DEP_CONSOLE_ONLY=y` |
| `DEPENDENCY_SOCKETS` | `DEP_SOCKETS=y DEP_HTTP=y` |
| `DEPENDENCY_PRINTER` | `DEP_PRINTER=y` |
| `DEPENDENCY_ICON` | `DEP_ICON=y` |
| `DEPENDENCY_SCREENIMAGE` | `DEP_SCREENIMAGE=y` |
| `DEPENDENCY_LOADFONT` | `DEP_FONT=y` |
| `DEPENDENCY_DEVICEINPUT` | `DEP_DEVICEINPUT=y` |
| `DEPENDENCY_ZLIB` | `DEP_ZLIB=y` |
| `DEPENDENCY_EMBED` | `DEP_EMBED=y` |
| `DEPENDENCY_MINIAUDIO` | `DEP_AUDIO_MINIAUDIO=y` |
| `ConsoleOn` | `DEP_CONSOLE=y` |
| `$EXEICON` or `$VERSIONINFO` | `DEP_ICON_RC=y` |
| temp folder index > 1 | `TEMP_ID=n` |

Dependencies are set by `SetDependency` (28195) — from each built-in's `id.Dependency` when it is used (in the
expression/call code), and directly for `_GL`, `$EXEICON`, `$CONSOLE:ONLY`, `ON STRIG`, `$EMBED`.
`DEPENDENCY_SOCKETS` is special: it is also an `RCStateVar` (`SockDepOn`) so that `beforefirstline.bi`/
`afterlastline.bm` can `$IF _SOCKETS_` — the first use of a socket feature costs a full recompile.
A change of `TARGET_BITS` triggers `make clean` using the marker `internal/c/.qb64_target_bits` (1174–1186).

---

## 11. Pain points, quirks and compatibility behavior

### 11.1 Behavior a rewrite must preserve (user-visible language semantics)

1. **Lexical rules:** 40-char identifier limit; names need a letter before any digit, no trailing `_`; user names may
   not start with a single `_`; all suffix forms; `&H/&O/&B` typing by value width with signed wraparound; float literal
   typing by significant digits; `REM`/`DATA` followed by `.` are identifiers (`rem.x`, `data.x`); `><`, `=>`, `=<`
   spellings, also with spaces between the two characters; `?` = `PRINT`.
2. **DATA** is captured raw by the lexer, including the rule that `:` ends it only outside quotes, and that
   `RESTORE label` uses the DATA offset at the *start of the labelled line* (`linedataoffset`, 3912).
3. **Periods in names** vs. UDT member access, decided by whether the left name is a UDT variable *at that point*.
4. **Labels:** numeric labels incl. `1.5`, `10#`, `10!`; per-procedure scope; ON ERROR/RUN targets in main only;
   RESTORE labels global and unique; numeric label followed by an alphanumeric label on one line.
5. **Single-line IF** forms: `IF a THEN 10`, `IF a GOTO 10`, `… ELSE 20`, nested single-line IF/ELSE, `THEN REM`,
   `ELSE` at start of a line, `: ELSE`.
6. **Name coexistence rules** in `regid` (scalar + array with the same name, same name with different suffixes,
   `$` variables named like reserved words, the `WIDTH` exception, parameters shadowing global CONSTs).
7. **musthave / mayhave** suffix matching, and `name$` matching `name$10`.
8. **DEFxxx** position-dependent; default type of `_`-initial names is its own slot.
9. **CONST:** type from value; same-value redefinition is only a warning; CONST inside a procedure is local;
   forward use is rejected even though the prepass knows the value; the restricted function set.
10. **Declaration order:** SUB/FUNCTION/TYPE/CONST usable before their definition *textually*, but TYPEs must be
    defined before the TYPE/parameter that uses them, and `STRING * const` needs the CONST earlier.
11. **Array parameter dimension pinning** (first call decides; mismatches are errors).
12. **`'$DYNAMIC`/`'$STATIC` take effect on the following line;** `$INCLUDE` is processed after its whole line.
13. **Auto-include positions and files,** `_IKW_` internal keyword mechanism, `_GL` detection, `$COLOR` constant files.
14. **`$IF` semantics** including left-to-right boolean folding, `DEFINED/UNDEFINED`, version comparison, and the
    preset names. Existing programs depend on the sloppy grammar (e.g. `$IF WIN THEN`).
15. **Emitted names and file contract** if the existing `qbx.cpp`/libqb/Makefile are kept: `SUB_X`, `FUNC_X`,
    `LABEL_x`, `S_n`, `RETURN_n`, `sc_n`, `__ASCII_CHR_046__`, variable prefixes (`__LONG_…`, `__STRING_…`,
    `__ARRAY_…`, `UDT_…`), the temp file set in §10, and the `DEP_*` variables.
16. **`$DEBUG` protocol** (`VWATCH_*` labels, `vwatch_global_vars/local_vars`, `usedVariableList`) if the IDE debugger is kept.
17. **Formatter output:** keyword casing table, spacing rules (`sp`/`sp2`), indentation, and "leave the line alone if
    not understood / if it has `_` continuation".

### 11.2 Implementation hacks that can be dropped (replace with normal compiler structure)

1. **Whole-program restarts.** Six unrelated reasons re-run both passes, sometimes several times. A real symbol
   table + deferred emission (or a proper semantic pass before codegen) removes all of them. `RCStateVar` exists only
   to manage this.
2. **GOTO re-entrancy** between IDE dispatcher and compiler, `GOSUB`/`RETURN` across the include managers, the prepass
   jumping into main-pass code (`predefine`/`predefined`), `GOSUB NormalTypeBlock` into the middle of an IF arm (2468).
3. **Duplicate code:** two include managers; TYPE parsing in both passes; CONST parsing in both passes; SUB header
   parsing in both passes; `$IF/$ELSE` logic in both passes; label find-or-create block ×8; open-block check ×3;
   include-line-number string building ×10+; `nm` logic duplicated for Windows and Linux.
4. **String-of-elements representation** with O(n) element access, parallel upper/original copies, type info encoded
   in text (`"str",len`, `123&&,&H7B`), hex-in-element DATA, and `CASE C-EL`.
5. **Global scratch `id` + `findanotherid` protocol,** non-re-entrant `HashFindCont`, and `findid` marking variables as used.
6. **Hidden parameters** via globals (`dimshared`, `dimstatic`, `dimsfarray`, `reginternalsubfunc`, `AllowLocalName`, …).
7. **Fixed limits:** 100 parameters, 100 include levels, 1000 control levels, 255 `$IF` levels, 4095 UDTs, 25,000
   array-parameter fix-ups, `STRING*256` names, 300 file handles closed blindly (1192, 1671, 13005).
8. **64 MB hash table** allocated at startup.
9. **Error handling by flag polling** and `ON ERROR` + `RESUME label` as control flow for file opens.
10. **Success detection by "does the exe exist".** make's exit status is not used.
11. **`os$ = "LNX"` meaning "not Windows",** with `MacOSX` as a second flag.
12. **Text-file stitching as the code generator's output model,** with buffer handles swapped under emitters, and
    `ret N.txt` textually included at every `RETURN`.
13. **`$INCLUDEONCE` detection by substring search of the whole upper-cased file** (it would also match inside a
    comment or string at the start of a line).
14. **Layout computed inside semantic handlers.**

### 11.3 Fragile spots and probable defects noticed while reading (not tested)

*Update: five of these were later run against the old compiler; results in `09-verification.md` ("Probable
defects"). The label + CONST prepass bug and the ELSE level check are confirmed and reachable; `symboltype("##")`
is harmless; the `nm` retry and the warning exit code were confirmed by reading only.*

| Where | Observation |
|---|---|
| 2130–2131 | Prepass label stripping: `wholeline$` is shortened first, then `cwholeline$ = RIGHT$(cwholeline$, LEN(wholeline$) - x3)` uses the *already shortened* length, so the case-preserved copy loses `x3` extra characters. Affects `label: CONST …` / `label: TYPE …` / `label: SUB …` lines (original-case names would be misaligned). Worth a test before relying on it either way |
| 2118 | "Starts with a digit" test uses `ASC <= 59`, which also accepts `:` and `;` |
| 2146 | The prepass splits statements on `ELSE`/`THEN`/`:` without regard to context; it relies on declarations never containing those words |
| 2794, 3017, 3056 | `STATIC` detection for `hr_syntax` uses `INSTR(UCASE$(line), "STATIC")` — a parameter named e.g. `staticVal` would truncate the help text |
| 3013–3023 | `hr_syntax` strips *all* underscores from the parameter text |
| 3525–3534 | Main-pass `$LET` re-runs `SetPreLET` on unvalidated upper-cased text (validation happened only in the prepass) |
| 3512 | `$ELSEIF` layout looks only for `=`, unlike `$IF` (3449–3457) |
| 6603–6624 | ELSE validity is checked by scanning raw line text for `IF`/`ELSE` prefixes |
| 6627–6638 | ELSE/ELSEIF search down the control stack for an IF but then test and modify `controlstate(controllevel)` (the top), not `controlstate(i)` |
| 6916–6917 | END SELECT emits the `sc_…_end` label before checking it is actually inside a SELECT |
| 13875, 14038 | The "C++ dynamic library" retry opens `nm_output_file$` (the static dump) instead of `nm_output_file_dynamic$` |
| `type.bas:370` | `symboltype("##")` returns the DOUBLE code |
| `type.bas:293` | Tests `LEN(typ$)`, an unset variable, instead of `s$` |
| 23457–23458 vs 2915 | Two different formulas map `_` to slot 27 (`95→91→27` and explicit `v = 27`); comment says "_=28" |
| 28339 | `SetPreLET` cannot overwrite preset names, so `$LET WIN = 0` silently creates a second, shadowed entry |
| 14228 | Any warning makes a windowed (`-c`) compile exit with status 1 |
| 1192, 1671 | Closing handles 1–300 would also close include handles 200–299 if a recompile happened mid-include (it relies on state reset) |
| 2358 | UDT element chaining assumes contiguous element indices |
| IDE + `_` continuation | Lexer calls back into the IDE (25524–25529); comment says continuation "in idemode is illegal" |

### 11.4 Coupling that makes incremental replacement hard

- The IDE reads compiler globals directly (`layout$`, `idereturn$`, `warning$()`, `usedVariableList()`, `InvalidLine()`,
  `UserDefineList$`, `listOfCustomKeywords$`, `ids()`), and the compiler calls `ide(0)` from inside the lexer.
- The compiler is self-hosted: `qb64pe.bas` must remain compilable by whichever compiler builds it, and the auto-included
  support files (`beforefirstline.bi`, `afterlastline.bm`, `vwatch.bm`, color files) are QB64 source that the new front
  end must accept, including the `_IKW_` convention.
- The built-in SUB/FUNCTION table (`reginternal`, `subs_functions\subs_functions.bas`) is data expressed as code against
  `idstruct`; statement parsing of ~all non-control statements depends on its `specialformat` strings.
- The C++ runtime contract (`evnt`, `qbevent`, `error_goto_line`, `return_point[]`, `data`/`data_offset`,
  `mem_static_pointer`, `cmem_sp`, `new_mem_lock`) is assumed by the skeleton `qbx.cpp`.

---

## 12. Open questions / not determined

- Internals of the DIM/REDIM/STATIC/COMMON block (8767–9685), `dim2` (17624–18945) and `allocarray`: only the entry
  conditions were read. Exact rules for `$DYNAMIC`, static vs. dynamic arrays, `REDIM _PRESERVE/_RETAIN`, and
  `DIM AS type list` need a dedicated read.
- Internals of FOR/NEXT, DO/LOOP, WHILE/WEND, ON TIMER/KEY/STRIG, FIELD, SHARED, ERASE, ASC/MID$ statements, `_MEM*`.
- `seperateargs` and the `specialformat` grammar (26211–26813) — belongs to the expression/codegen study.
- Meaning of `linkid/linkarg` and of `tsize` for non-string types.
- IDE side of the message protocol (only the compiler side was read).
- Whether the defects listed in §11.3 are reachable in practice (none were executed).
- Which behaviors are upstream QB64-PE and which belong to this fork, beyond the items flagged at the top.
