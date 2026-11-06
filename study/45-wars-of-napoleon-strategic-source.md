# 45. Recovering the source of Wars of Napoleon's strategic game: the plan

Written 2026-11-06, for the user's review. Section 6 holds six decisions, each with the panel's view. The user
asked the panel to decide them the same day. Section 7 records what it decided, which changes D2, D4 and D5
(`DECISIONS.md` 2026-11-06). The study stays open until the source is recovered (`CLAUDE.md` working rule 7).

The game is W.R. Hutsell's *Wars of Napoleon* (`Hutsell-Games/wars-of-napoleon`), cloned beside this repo as
`..\wars-of-napoleon`. It has two programs:

- the **strategic game**, `WON.EXE`: the map of Europe, armies, money, ships and turns;
- the **tactical game**, `NAPOLEON.EXE`: one battle, started by `WON.EXE` through the files `BATTLE.$$$` and
  `OUTCOME.&&&`.

The repo has the tactical game's source (`NAPOLEON.BAS`, `NAP1A.BAS`, `NAP1C.BAS`, `NAP10.BI`). It has no source
for `WON.EXE`. This plan is for getting that source back.

The scratch work of this study is in `..\won-hutsellgames\`, outside every repo:

- `won.zip`, as downloaded, and `unzipped\`;
- `work\`, holding the unpacked `WON.EXE` and the compiled objects;
- `tools\`, holding the scripts that made the measurements below. Copies of the five that are kept are in the
  game's branch `won-source`, under `tools\recover\` (section 8).

## 1. What we have

### 1.1 The download from hutsellgames.com

The game's page (`https://hutsellgames.com/wars-of-napoleon/`) offers one file,
`wp-content/uploads/2021/07/won.zip` (522,444 bytes, 58 files, dated 1996 to 1998). Compared with the repo:

- **The programs are the same.** `WON.EXE`, `NAPOLEON.EXE`, `VIC.EXE`, `ARCH2.EXE`, `NAPIC.EXE` and the `.EGA` and
  `.VGA` pictures are byte for byte equal in both.
- **Seven of the repo's files are damaged, and the zip has good copies of all seven.** `GO.BAT`, `REGISTER.DOC`,
  `EURO1796.MAP`, `EURO1812.MAP` and `EURO1813.MAP` hold Windows search-suggestion JSON. `EURO1807.MAP` holds a
  bitmap and `EURO1808.MAP` binary junk. They look like files restored from a damaged disk. Without the five maps,
  five of the seven scenarios cannot start.
- **Two files are only in the zip.** `FILE_ID.DIZ` is the shareware description. `NWS.LNK` is the linker's response
  file for the strategic game (section 1.2).
- **Most text files differ only in line ends.** The repo's `.gitattributes` has `* text=auto`, so git stores them
  with LF. `BATTLE.$$$` and `OUTCOME.&&&` really differ; the game rewrites them for every battle.
- **Only in the repo:** the tactical source, `TACTICAL.EXE`, `NAPOLEON.TXT`, `NAPOLEON.PAG`, `NAPOLEON.MAK`,
  `~QBLNK.TMP`, `LICENSE` and `README.md`.

A web search found no other copy of the strategic source. The repo's other branches have none either:
`canonical-source` and `minimal-changes` hold only the tactical source. Section 1.5 covers `cursor-assisted`.

### 1.2 How WON.EXE was built

`NWS.LNK` names the lost modules and the libraries:

```
nws+nws2+nwsmenu+
c:\pds71\noisam+c:\pds71\noems+c:\pds71\nocga+
c:\pds71\nocom+c:\pds71\noherc+c:\pds71\nooga /E /NOE /PACKCODE
nws
nul
c:\pds71\bcl71enr.lib c:\pds71\qbx.lib;
```

It is Civil War Strategy's `CWS.LNK` with `cws` changed to `nws`, and without `noega`. Civil War Strategy's
`CWSMAK.BTM` shows the rest of that pipeline: `bc /o /e` for each module (`/o` alone for the menu module), `link
@cws.lnk`, then the LZEXE packer.

`WON.EXE` matches that pipeline:

- **It is packed twice**, by LINK's EXEPACK (the `/E` above) and then by LZEXE 0.91 (the mark `LZ91` at offset
  0x1C), so its text cannot be read as it is. `tools\unlzexe91.py` undoes both, from 114,795 bytes to 210,976.
- **The unpacked program names its three modules** at the start of their code: `WON` (46,176 bytes of code),
  `WON2` (46,656) and `WONMENU` (about 19,000). So the sources were `WON.BAS`, `WON2.BAS` and `WONMENU.BAS` when
  it was built; `NWS.LNK` is an older name for the same build.
- **Its text says** "Wars of Napoleon / STRATEGY GAME / (c) 1997 W. R. Hutsell / v1.16" and "This is your
  registered copy of the game", so every scenario and the linked tactical game are enabled.
- **It runs other programs:** `NAPOLEON.EXE` for a battle, `VIC.EXE` at the end, `ARCH2.EXE` and `rrs.exe`, and
  `copy nws.his oldhist.ory`. It reads `gamedata.ini`, `nws*.ini`, `nws.cfg`, `mtn.vga`, `napicon.vga` and `*.sav`.

The first version of the unpacker undid only LZEXE and found no relocations; section 8 has how that was fixed.
The unpacked file has not yet been run.

### 1.3 The strategic game is a fork of Civil War Strategy

Hutsell's Civil War Strategy has its original source in `..\civil-war-strategy`, at commit `5d75791` ("Initial
commit of Hutsell's source code"): `CWSTRAT.BAS`, `CWSTRAT2.BAS`, `CWSMENU.BAS`, `CWS14.BI`, version 1.57 of 1998.
That commit also has the three `.OBJ` files Hutsell compiled. (The repo's working tree is the QB64 port, with the
modules turned into `.BM` files. This study did not touch it; it read the commit with `git archive`.)

Three measurements show how close the two games are:

- **Text.** Of 395 strings of eight or more letters in `WON.EXE`, 169 are in Civil War Strategy's source word for
  word ("No eligible armies in same city to combine", "has drilled to reach experience level"). Only 11 are in the
  tactical source.
- **Code.** `tools\normalign.py` turns machine code into instruction shapes: each instruction's name and
  registers, with every number replaced by `N`. That removes the addresses the linker fills in. It then aligns
  the shapes and counts instructions that match in runs of four or more:

  | WON.EXE module | Compared with | Instructions that align |
  |---|---|---|
  | `WON` (16,551 instructions) | `CWSTRAT` | 71% |
  | `WON2` (17,108) | `CWSTRAT2` | 42% |
  | `WONMENU` (7,759) | `CWSMENU` | 56% |

  So between half and two thirds of the strategic game is Civil War Strategy's code. The rest is new or
  changed: the scenarios, nationalities and cohesion, Napoleon himself, the link to the tactical game, the
  naval rules.
- **Procedures.** Civil War Strategy has 76 SUBs and FUNCTIONs (28, 42 and 6 per module). The strategic game is
  probably of the same order, 80 to 110.

### 1.4 The compiler we have is the compiler that built it

PDS 7.1 is installed for `tools\qb45\qb45.py` (`..\pds71\install`, with `BCL71ENR.LIB`, the library `NWS.LNK`
names). `tools\calibrate_bc.py` compiled Civil War Strategy's three modules with BC 7.10 under DOSBox, using
`CWSMAK.BTM`'s switches, and compared the results with Hutsell's `.OBJ` files:

- **The layout is the same.** Every segment has the same size except code and string constants: `COMMON` 2,238
  bytes and `BC_DATA` 3,136 bytes in `CWSMENU`, for example.
- **The code nearly matches byte for byte.** After alignment, 99.5% of `CWSMENU`'s code bytes and 98.9% of
  `CWSTRAT2`'s are equal. The rest are small runs: string offsets, and a few edits between the source at that
  commit and the one Hutsell compiled. As instruction shapes, `CWSMENU` is 7,132 of 7,160, 100% in runs.

BC 7.1 therefore turns a given source into the same code every time, and we can check a guessed source by
compiling it.

### 1.5 The earlier attempt

The branch `cursor-assisted` (11 commits, last 2025-12-26, "What a freaking mess") rebuilt the strategic game as a
new design: about 60 `.bas` files under `src\`, a single program joining the strategic and tactical games, and
plans such as `docs\FULL_RECREATION_PLAN.md`. Its analysis listed which features to take from Civil War Strategy
and World War 2 and which to write anew. It never compared anything with `WON.EXE`. So it had no way to tell a
right guess from a wrong one: the computer player, the combat sums and the random events are not visible from
outside. This plan uses `WON.EXE` itself as the check.

### 1.6 Other sources of facts

- **`WON.DOC`**, the manual (56 KB): sequence of play, every menu, combat, cohesion, income, and in section 7 the
  formats of `NWSxxxx.INI` (a scenario), `LEADxxxx.DAT` (commanders) and `NWS.CFG`.
- **The data files**, which the recovered code must read: `GAMEDATA.INI`, seven scenarios each with a map,
  commanders and an `.INI`, `NWS9.SAV`, `HISCORE.NWS`.
- **The tactical source**, which also shows the interface files and the menu code `NAP1C.BAS` ("CIVAR MENU 2.2").

## 2. The approach: recompile until it matches

Rebuild the source by editing BASIC until BC 7.1 compiles it into the code of `WON.EXE`. This is "matching
decompilation", as done for old console games. Here it is easier than usual, for three reasons:

- we have the exact compiler, and it is deterministic (section 1.4). The runtime library we have is not quite
  Hutsell's (section 8), which matters only for the exact match below;
- BC's code follows fixed patterns, and every runtime call goes to a named routine of `BCL71ENR.LIB`;
- more than half the code already exists as Civil War Strategy's source (section 1.3).

A procedure is done at one of two levels:

- **Shape match:** its compiled instruction shapes equal those in `WON.EXE`. The logic is then the same: every
  operator, call and jump, and every constant held in an instruction. Strings and floating-point constants are
  not covered, because the code reaches them by address; the panel closed that gap (section 7, D2).
- **Exact match:** the three modules, linked with `NWS.LNK`, give a program equal to the unpacked `WON.EXE`
  byte for byte. That also fixes the order of variables, the string constants and the `COMMON` layout. It needs
  Hutsell's runtime library, which has to be rebuilt first (section 8).

The goal is the exact match for the whole program. A procedure that only reaches the shape match is listed with
the reason.

**Alternatives, and why not:**

- **Rewrite from the manual and from playing.** That is what `cursor-assisted` did. Playing shows screens, not
  the sums behind them, and nothing says when the rewrite is right.
- **Translate the disassembly to BASIC by hand, without compiling it.** It reads the same code, but leaves the
  result unchecked. Each mistake would show only in play, if at all.
- **Run `WON.EXE` in an emulator inside a QB64 program.** That gives a playable game but no source, which is
  what the user asked for.

## 3. Where playing comes in

Playing does not reveal formulas; the code does. Playing `WON.EXE` serves three purposes:

1. **A map of the game.** Which screens, menus and messages exist, and in what order. This guides the reading of
   the new code (section 1.3), the part with no Civil War Strategy counterpart.
2. **Reference recordings.** For each session: the keys pressed, a screenshot after each screen, and the files
   the game writes (`*.SAV`, `NWS.HIS`, `BATTLE.$$$`, `OUTCOME.&&&`, `HISCORE.NWS`). The recovered source, built
   with PDS 7.1, must play these the same way. Later the QB64Rust build must too, as in the World War 2 port.
3. **Paths that are rare.** Random events, naval invasions, commerce raids, end conditions and Napoleon's exile
   need games set up to reach them. Recordings that reach them become the tests that cover them.

### 3.1 The harness

- **DOSBox 0.74-3 cannot type keys for us.** DOSBox-X and DOSBox Staging can: the `AUTOTYPE` command in the
  `[autoexec]` section types a list of keys with set delays. Either is a new program on this machine (decision
  D5).
- **Hidden runs.** DOSBox runs with SDL's dummy video driver, as `qb45.py` does, so no window shows. Step 2 first
  checks that screenshots are still captured that way. Inside DOSBox, the programs the game runs (`NAPOLEON.EXE`,
  `VIC.EXE`, `copy`) are DOS programs, so they open no Windows console (`CLAUDE.md` working rule 6).
- **A fresh folder for each run,** copied from the zip's files, so that one run's saves never reach the next.
- **The same random numbers each time.** The game seeds them with `RANDOMIZE TIMER`, so two runs differ. Step 2
  makes `WONSEED.EXE`: the unpacked `WON.EXE` with the one call to the seeding routine changed to pass a fixed
  value. The original stays as it is. QB64's `RND` uses QuickBASIC's sequence; whether its `RANDOMIZE n` seeds it
  the same way as PDS is measured with `qb45.py --compiler pds71` before any comparison relies on it.
- **Claude plays by script.** Each step of a session adds keys to the script, runs it from the start, and reads
  the last screenshot. With a fixed seed, every run of the same keys gives the same game.

### 3.2 The sessions

About fifteen sessions, each to the end of the game or as far as its purpose needs:

| # | Scenario | Setup | What it is for |
|---|---|---|---|
| 1 | 1796 | 1 player, French, defaults | The whole turn cycle, every menu once |
| 2 | 1796 | 1 player, Allies | The computer playing France |
| 3 | 1805 | 2 players | Both sides' screens; no computer moves |
| 4–8 | 1807, 1808, 1812, 1813, 1815 | 1 player, defaults | Each scenario's start and end conditions |
| 9 | 1805 | tactical game on | The battle files, `NAPOLEON.EXE` and the result read back |
| 10 | 1805 | random events on, vary start on | Events and the varied start |
| 11 | 1812 | aggression Passive, then Reckless | The computer player's settings |
| 12 | 1796 | balance French ++, then Allied ++ | Play balance |
| 13 | any | save, quit, load | `*.SAV` and the autosave |
| 14 | any | ships, invasions, raids | The naval rules |
| 15 | any | play to defeat and to victory | `VIC.EXE`, Napoleon's exile, high scores |

Each session's keys and recordings are kept, as `tests\data\` keeps them in the World War 2 port.

## 4. The steps

Steps 1 and 2 can run in either order. Each step ends with its result written down where decision D3 puts the
work.

1. **Tools, and a check that they work.**
   - Fix the unpacker's relocations. Unpack `CWS.EXE` (also LZEXE 0.91) with it.
   - Link Hutsell's three Civil War Strategy objects with `CWS.LNK` and compare the result with the unpacked
     `CWS.EXE`. Byte for byte equal shows that the link and the unpacker are right. If not, find why before going
     on.
   - Compile `NAPOLEON.BAS` with `NAPOLEON.MAK`'s modules and compare it with `NAPOLEON.EXE` (also LZEXE 0.91). This
     says whether the tactical source in the repo is the version that shipped.
   - **A map of the runtime.** Find each routine of `BCL71ENR.LIB` and `QBX.LIB` in `WON.EXE`, so that every call
     in the game's code gets a name, such as `B$PESD` or `B$OPEN`. Hutsell's library is not quite ours (section 8),
     so some routines may not be found byte for byte; those are matched by shape.
   - **A procedure splitter** that finds where each SUB and FUNCTION starts and ends in a module's code, from BC's
     entry and exit sequences.
   - **The compare tool:** compile a module under DOSBox, then report each procedure as exact, shape, or the first
     instruction that differs.
2. **The harness and the sessions** of section 3, with `WONSEED.EXE`.
3. **Pair the procedures.** Match each procedure of `WON.EXE` with one of Civil War Strategy, by its runtime calls,
   strings and shape. Output: a table with one row per procedure (same, changed or new) and the size of each
   change. Work out the `COMMON SHARED` layout from the code and from `CWS14.BI`, giving the `.BI` file.
4. **Write the source,** module by module, smallest first: `WONMENU`, then `WON2`, then `WON`.
   - Start from the Civil War Strategy module, renamed. Edit each paired procedure until it matches.
   - Write each new procedure from its disassembly, read with the runtime map. BC's patterns make most statements
     plain: a call to `B$PESD` after loading a string is a `PRINT`.
   - Compile after each procedure. The compare tool shows the first difference, so each round fixes one thing.
5. **The exact match.** Order variables and constants until the linked program equals the unpacked `WON.EXE`.
   Procedures that stay at the shape match are listed with the reason.
6. **Play it.** Build the recovered source with PDS 7.1, seed it as in step 2, and replay every session. The
   screens and files must equal the recordings. The build is not packed with LZEXE: the comparison is with the
   unpacked program.
7. **The port to QB64Rust** is a separate plan, in the form of `study\44` and `docs\PORTING-PDS.md`. It covers
   both programs, the battle files between them, and `SHELL`, which needs the care of `CLAUDE.md` working rule 6.

**Effort, in Claude's time.** Steps 1 and 2: about one session each. Step 3: part of a session. Steps 4 and 5 are
the bulk: each paired procedure takes a few compile rounds, and each new one more. Roughly half the code is new
(section 1.3), so steps 4 and 5 take several sessions. What remains costly is not Claude's time:

- **Machine time:** each compile of a module under DOSBox. The three Civil War Strategy modules compiled within
  one short DOSBox run, so this is small.
- **The user's time:** reading this plan, the procedure table of step 3, and the result.
- **New programs on this machine:** DOSBox-X or DOSBox Staging (D5), and the scratch `capstone` disassembler,
  installed only in `..\won-hutsellgames\venv`.

## 5. Risks

- **The switches may differ.** `WON.EXE` may have been compiled with other switches than `CWSMAK.BTM`'s. Each
  switch leaves marks in the code (`/E` keeps line tables for `RESUME`, `/D` adds checks), so step 1 reads them
  from the binary before step 4 starts.
- **Civil War Strategy 1.57 is a year newer than the fork.** Some of its code changed after Wars of Napoleon was
  taken from it. Matching handles this: such a procedure is "changed", and its edits are undone.
- **Names and comments are lost.** The binary keeps neither. Paired procedures take Civil War Strategy's names.
  New ones get names chosen to fit that style (D6). Comments are lost for good.
- **A procedure may never match exactly.** BC's choice of code can depend on the order of earlier statements or
  on spare temporaries. The shape match is the fallback, and it still proves the logic.
- **Playing and comparing depend on the fixed seed.** If patching the seed changes anything else, the recordings
  are wrong. The patch is one call, and the patched program is compared with the original apart from that call.

## 6. Decisions for the user

The panel: a pragmatic engineer, a QB64 engineer, a compiler engineer, a Rust engineer, a test engineer, and a
**reverse engineer**. The reverse engineer is added because the core of this work is reading a packed DOS binary
and matching compiler output, which none of the other roles does day to day.

**D1. The approach: recompile until it matches (section 2), or rewrite from the manual and from play?**
Recommended: recompile until it matches.
- *Pragmatic:* the measured overlap with Civil War Strategy is large, and every step has a yes or no answer. The
  earlier rewrite shows the cost of guessing.
- *Compiler:* the CWS calibration is the deciding fact. A deterministic compiler and its exact library turn
  decompilation into a search with a test.
- *Reverse engineer:* this is the most reliable way known for compiled BASIC and C of that era. Its risks are in
  section 5, and none is fatal.
- *Test:* playing alone can never show the computer player's rules are right; matching shows it for every branch.
- No one disagreed.

**D2. The target: the exact match of the whole program, or the shape match per procedure?**
Recommended: aim for the exact match, and accept the shape match with a stated reason where the exact one is not
reached.
- *Compiler:* the shape match already proves the logic. The exact match adds variable order and string offsets,
  which matter only for proving the whole.
- *Pragmatic:* the exact match is a single clear "done". But a few procedures that refuse it should not block the
  port.
- *QB64 engineer:* the port needs correct logic, not byte order. The shape match is enough for the port.

**D3. Where the work lives.**
Recommended:
- a branch `won-source` of `wars-of-napoleon`, made from `main`, holding `WON.BAS`, `WON2.BAS`, `WONMENU.BAS`, the
  `.BI` file, the build files, and the tools under `tools\recover\`;
- the recordings under `tests\data\`;
- a `docs\recovery.md` with the procedure table and the status of each procedure;
- `..\won-hutsellgames\` removed when the work moves.

The QB64Rust repo keeps only this study and what the port later finds.
- *Pragmatic:* the recovered source belongs with the game, like the other ports.
- *Rust engineer:* nothing of this belongs in QB64Rust's crates or `tools\`. These are one-game tools in Python.
- *Test:* the recordings go with the source they test.

**D4. Repair `main` of `wars-of-napoleon`?** Its `README.md` says the branch holds the original source and
"should not be edited". Seven of its files are damaged (section 1.1).
Recommended: replace the seven files with the zip's copies, and add `NWS.LNK` and `FILE_ID.DIZ`, in one commit
that says where they came from. Also mark the data files binary in `.gitattributes` (`*.MAP`, `*.DAT`, `*.INI`,
`*.SAV`, `*.$$$`, `*.&&&`), so that git keeps their CRLF line ends as the game wrote them.
- *Pragmatic:* the files are restored, not edited. Without the maps, five scenarios cannot start, and no test can
  be recorded.
- *QB64 engineer:* keeping CRLF is right for an archive. Whether the old programs would also read the LF copies
  with `INPUT #` has not been measured, and with the files restored it need not be.
- *Test:* the byte comparison of section 1.1 is the evidence; the commit message should name it.
- Another choice, if `main` must stay untouched: do the repair on `won-source` only.

**D5. Which DOS emulator for playing?** DOSBox 0.74-3, which is installed, cannot type keys.
Recommended: DOSBox-X, installed beside DOSBox 0.74-3, used only by the harness, always hidden.
- *Reverse engineer:* DOSBox-X also has a debugger with memory dumps and breakpoints. That helps check a
  recovered procedure against the original while it runs.
- *Pragmatic:* DOSBox Staging would also do; DOSBox-X is chosen for the debugger.
- *Test:* the emulator's version is recorded with the recordings, so they can be made again.

**D6. Names in the recovered source.** Paired procedures keep Civil War Strategy's names. For new procedures and
variables:
Recommended: short names in Hutsell's style (as `CWSTRAT.BAS` and `NAPOLEON.BAS` have), chosen to say what each
thing is. Renaming for clarity is a later refactor.
- *QB64 engineer:* the recovered source should read like Hutsell's other games, so the ports and any later
  refactor treat all three alike.
- *Pragmatic:* `docs\PORTING-PDS.md` section 0 says a port is not a refactor. The same holds for recovery.

## 7. The panel's decision (2026-11-06)

The user asked the panel to decide how to proceed. It met with the six roles of section 6. It kept D1, D3 and D6
as recommended, and changed D2, D4 and D5.

**D1, the approach: recompile until it matches.** Kept, with one checkpoint. After step 3, the procedure table
shows how much code is new. If more than about 60% of the program has no counterpart in Civil War Strategy, or the
compiler switches cannot be pinned down in step 1, the panel meets again before step 4. *Pragmatic:* the
measurements are strong, but they are averages over modules; the table is the first count per procedure.

**D2, the target: changed.** The test engineer found a gap in the shape match. It masks every number that is an
address, and BC reads string literals and floating-point constants from the module's constant segment by
address. So a wrong string, or `1.5` written for `1.25`, would still pass. The target is now, per module:

- the instruction shapes equal those of `WON.EXE`, as before; and
- the module's constant segment (`BC_CN`: strings and floating-point constants, in source order) equals the bytes
  of that segment in `WON.EXE`.

Together these prove the logic and every literal. The byte-for-byte match of the whole linked program is still
pursued, after every module meets the target, but it does not hold up the port. *Compiler engineer:* `BC_CN` is
laid out in order of first use, so matching it also checks the order of statements that the shapes alone might
let slip. *Reverse engineer:* the constant segment of each module is easy to find in the unpacked program once
the link check of step 1 gives the segment layout.

**D3, where the work lives: kept.** A branch `won-source` of `wars-of-napoleon`, made from `main`. The tools go
under `tools\recover\`, the recordings under `tests\data\`, the procedure table in `docs\recovery.md`. Claude
creates the branch and the files; the user stages and commits them (`CLAUDE.md` human rule 1).

**D4, the damaged files: changed.** `main` stays untouched. Its `README.md` says that branch "should not be
edited". That is the user's own rule, and the panel did not take it on itself to set the rule aside. The seven good
files from the zip, `NWS.LNK`, `FILE_ID.DIZ` and the binary marks in `.gitattributes` go into the first commit of
`won-source` instead, with a message that names the zip and the byte comparison of section 1.1. Whether `main`
takes them too is the user's choice when `won-source` is merged. *Pragmatic:* nothing is lost either way, and the
branch can be played from its first commit.

**D5, the emulator: changed.** DOSBox-X, but not installed. Claude downloads the portable ZIP of a release from
the project's own GitHub releases and unpacks it into `..\won-hutsellgames\dosbox-x\`. There is no installer and
nothing changes system-wide, and deleting the folder removes it. *Test engineer:* the harness records the
DOSBox-X version with each recording. *Reverse engineer:* the portable build has the same debugger. The first
thing tried with it is a hidden run that saves a screenshot. If that fails, the panel meets again.

**D6, names: kept.** Paired procedures keep Civil War Strategy's names. New ones get short names in Hutsell's style.

**The order of work.** Step 1 comes first, because its link check can still show that the approach does not hold.
Within step 1:

1. fix the unpacker's relocations;
2. link Hutsell's Civil War Strategy objects and compare them with the unpacked `CWS.EXE`;
3. compare the tactical source with `NAPOLEON.EXE`;
4. map the runtime library;
5. write the procedure splitter and the compare tool.

Step 2 (the harness and the sessions) follows. The panel saw no reason to run the two side by side.

## 8. Step 1 so far (2026-11-06)

**The branch.** `won-source` exists in `..\wars-of-napoleon`, made from `main`. Nothing is staged or committed.
In its working tree:
- the seven damaged files are restored from the zip, and `NWS.LNK` is added;
- the user removed the files no program uses: `GO.BAT`, `PRINTDOC.BAT`, `REGISTER.DOC`, `SHAREW.TXT`,
  `WRHGAMES.DOC`, `TACTICAL.EXE`, `NAPOLEON.MAK` and `~QBLNK.TMP` (all still on `main`). `FILE_ID.DIZ` was
  never added;
- 34 more files that differed from the zip only in line ends now have the zip's bytes;
- `.gitattributes` is `* -text`.

The old `.gitattributes` line `.exe binary .vga binary .obj binary` was one pattern followed by attributes, so it
marked nothing binary. `BATTLE.$$$` and `OUTCOME.&&&` keep the repo's later game state. The five scripts are in
`tools\recover\`, with a `README.md`.

**The unpacker: fixed.** `WON.EXE` is packed twice: by LINK's EXEPACK (the `/E` of `NWS.LNK`), then by LZEXE 0.91.
That is why the LZEXE layer had no relocations. `unlzexe91.py` now undoes both. The relocation counts are 4,438 for
`WON.EXE`, 6,299 for `CWS.EXE` and 3,481 for `NAPOLEON.EXE`.

**The link check: the game code matches, the runtime does not.** Hutsell's three Civil War Strategy objects were
linked with `CWS.LNK` and compared with the unpacked `CWS.EXE`:
- **The game code matches.** Each module's code segment has the same size and the same place in both. 97% of
  `CWSMENU`'s instruction shapes align and 98% of `CWSTRAT2`'s. The remainder looks like data inside the code
  read as instructions, which the procedure splitter will remove.
- **The runtime does not match.** Our program is 213,600 bytes and Hutsell's 220,912. The runtime routines are in
  a different order: both programs hold the same runtime strings, in another sequence. The data segment holds
  1,102 more bytes before the game's strings. Leaving out `noega` changes nothing.

So the `BCL71ENR.LIB` that `tools\qb45\pds71_install.py` builds is not the one Hutsell linked with. It differs in
module order and in some content.
- **For the panel's target this does not matter.** The target compares each module's code shapes and constant
  segment, and both are independent of the runtime's layout.
- **For the byte match of the whole program it does.** That needs Hutsell's library. It can be rebuilt by finding
  each library module's code in the shipped `CWS.EXE`, which gives their set and order, and joining them in that
  order with `LIB`. This is added to step 5.

**The tactical source is not the shipped `NAPOLEON.EXE`.** The repo's `NAPOLEON.BAS`, `NAP1A.BAS` and `NAP1C.BAS`
compile with `/O/E` and are compared with the matching segments of the unpacked `NAPOLEON.EXE` (1998-06-24):

| Module | Compiled code | In the shipped program | Shapes that align |
|---|---|---|---|
| `NAPOLEON` | 49,261 bytes | 48,096 bytes | 92% |
| `NAP1A` | 34,748 bytes | 33,776 bytes | 88% |
| `NAP1C` | 2,695 bytes | (module end not yet located) | 84% |

The source is a different revision, about 1 KB larger per module, probably later. `TACTICAL.EXE` on `main`
(PKLITE-packed, not in the zip, removed on `won-source`) may be its build; unpacking PKLITE would show it. The port of section 4 step 7
uses the source as it is (user, 2026-11-06). Matching it to the shipped program is a `SOMEDAY.md` row: the same
method gets there from the program's bytes whenever it is wanted.

**Which files the programs use.** Each file name in the repo was looked for in the unpacked `WON.EXE` and
`NAPOLEON.EXE`, in the tactical source and in the batch files (`..\won-hutsellgames\tools\filerefs.py`).
- **Used:**
  - the five programs (`WON.EXE` starts `NAPOLEON.EXE`, `VIC.EXE` and `ARCH2.EXE`; `NAPOLEON.EXE` starts
    `NAPIC.EXE`);
  - `GAMEDATA.INI`, `NWS.CFG`, `MTN.VGA`, `NAPICON.VGA`, `HISCORE.NWS` and `BATTSUMM`;
  - for each scenario, an `NWSxxxx.INI`, `EUROxxxx.MAP` and `LEADxxxx.DAT`, whose names `WON.EXE` builds from the
    year;
  - `BATTLE.$$$` and `OUTCOME.&&&`;
  - for the tactical game, `ALLIES.DAT`, `FRENCH.DAT`, `EQUIP.DAT`, `QUOTES.DAT` and the four `.EGA` icon sets.
- **Optional:** `NWS9.SAV`, a saved game.
- **Not used by any program:**
  - the documents: `WON.DOC`, `NAPOLEON.DOC`, `NAPOLEON.PAG` (differs from `NAPOLEON.DOC` from byte 51),
    `NAPOLEON.TXT` (a tactical-only manual), `REGISTER.DOC`, `SHAREW.TXT`, `WRHGAMES.DOC` and `FILE_ID.DIZ`;
  - the launchers `GO.BAT` and `PRINTDOC.BAT`;
  - the build files `NAPOLEON.MAK`, `~QBLNK.TMP` and `NWS.LNK` (which the recovery needs);
  - `TACTICAL.EXE`.
- **Not named by `WON.EXE` or `NAPOLEON.EXE`, not yet settled:**
  - `PREFER.CFG`, `GENERALZ` and `MAXSKORS` are Civil War Battleset's file names: its `CIVWAR25.BAS` opens all
    three. They are probably leftovers.
  - `SETUP.INI`, `NAP1.EGA` and `NAP2.EGA` may be read by `NAPIC.EXE`, `VIC.EXE` or `ARCH2.EXE`. Those three are
    packed with PKLITE, so their strings cannot be read yet. Unpacking them, or a traced run in DOSBox, settles
    it.

Nothing is removed from `won-source` until these are settled. Pruning belongs to the port (`docs\PORTING-PDS.md`
section 0).

**Next in step 1:** the runtime map, the procedure splitter and the compare tool.
