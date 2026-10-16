# Design

## Context

See `proposal.md` for why. What shapes the approach:

- A plain built-in comes in as a row, a call-site program and a slice line (`DECISIONS.md` 2026-10-10). The four
  sound rows are plain: the C++ of all 43 accepted programs was read (`qb64pe -z`, outside the repository)
  and every form in them is the call of its table entry and template, with the argument conversions `sema` already makes for
  a slot (`sub_sound( 440 , 1 ,NULL,NULL,NULL,NULL,NULL, 1 ,1)`; `qbr(*__SINGLE_S)` for a SINGLE in the LONG
  waveform slot; `func_play( 0 ,0|1)`).
- The build already follows the program: `sema` records the optional parts of the runtime a program's built-ins
  need (`RuntimePart`, today only the icon part of `_ICON`), the IR carries them, and `driver\src\build.rs` turns
  them into make flags that are part of the build key. A built-in whose part no build provides is "not supported
  yet" today; that is the mark `SOUND` and `PLAY` would get after their rows are added.
- `$EXEICON` reaches `sema` as a metacommand token and gets the general mark for a metacommand. The parser's line
  rules for metacommands (a colon before it, a tab before it) already hold for it.
- `sema` reads no file. Included files come through the parser's loader, which only loads source text.
- Sound is not part of the screen state (`tools\screen\README.md`), and a program's sound comes from libqb, the
  same library for both compilers.
- The machine the tests run on plays what a test program plays, and tier 2 runs with eight jobs.

Measured facts are in `verification\v29_*` (110 programs) and go into `study\00` §5 in task 1.1.

## Goals / Non-Goals

**Goals:**

- Civil War Strategy compiles with no diagnostic and its executable builds.
- Every accepted form of `SOUND`, `PLAY` and `BEEP` writes the old compiler's call; every rejected form is
  rejected.
- Sound that runs is checked as far as a program can observe it, without noise and without results that depend
  on the load.

**Non-Goals:**

- Checking what is heard (the samples libqb writes to the device).
- Running the game in a test.
- The other sound built-ins, `$VERSIONINFO`, and `$EXEICON` names the old compiler looks for in its own folder.

## Decisions

### D1. Four plain rows

`SOUND`, `PLAY` and `BEEP` join `STATEMENTS` as `StmtRule::Plain`, `PLAY` joins `SUPPORTED` as `Rule::Plain`
(believed and held DOUBLE, the table's return type; one optional slot, so it is also called bare, like `RND`).
Nothing in `check\builtins.rs` is written for them. If a form does not come out as the old compiler's (the
trailing comma of `SOUND 440, 1,` and `PLAY t,`, the trailing `WAIT`/`RESUME` choice after a list that ends in
left-out arguments, `SOUND` with no argument at all), the fix goes into the template plan
(`qb64rust_builtins::passing`) or the parser's reading of the template, for every statement, not into a rule
for `SOUND`; task 2.1 says for each which it was.

`ON PLAY(5) GOSUB 10` is `ON <number> GOSUB` with a function call in the old compiler (measured `v29_x64`:
`ongo = qbr_float_to_long(func_play( 5 ,0|1))`), and `PLAY ON`, `PLAY OFF` and `PLAY STOP` are errors there
(`x61`–`x63`, "Name already in use"). The parser here reads both as QuickBASIC's event forms, because `PLAY` is
one of its event words (`syntax\src\parser\errors.rs`, `EVENT_WORDS`): today the first gets "not supported yet:
event handlers" and the others "not supported yet: event switches". So `PLAY` leaves the event words: `ON PLAY`
then parses as `ON <expression> GOSUB`, and `PLAY ON` goes to the statement's template, where `ON` is not a
string expression and the line is an error that is not "not supported yet". Task 2.1 checks that no accepted
program of the corpus, the upstream set or `tests\programs` relied on the old reading (the shrink-only lists
show it). The target of `x64` is a line number, which is not compiled yet for any `ON … GOSUB`; the tests here
use a label, and `x64` itself stays held back by that mark alone.

*Alternative considered:* a `StmtRule::Sound` that reads the list itself. Rejected: the C++ shows no special
case, and the template plan is what the call-site check exercises for every other statement.

### D2. The name and a colon

`BEEP:` and `SOUND:` at the start of a line are the statement with no argument (measured `v29_x72`, `x24`, `x81`,
`x82`), through the rule that exists for `CLS:` (`check\builtins.rs`, "a built-in statement's name and a
colon"). `PLAY:` is the statement without its required string and so a compile error (`x65`, `x83`). `GOTO BEEP`
and `GOTO SOUND` are "Invalid label!" in the old compiler (`x78`, `x79`); here they keep the mark a jump to a
built-in statement's name has (`SOMEDAY.md`, with `GOTO CLS`); these two are the only rejected programs of
`v29_x*` that are not a real error after this change (`x80`, `GOTO PLAY`, has the real error of its `PLAY:`
line). The entry of `SOMEDAY.md` that names `BEEP: x = 1`
as a label named like a built-in that is not compiled yet gets another example.

### D3. The audio part of the runtime

`RuntimePart` gets `Audio`, for the table's `DEPENDENCY_MINIAUDIO`; the IR's copy and its dump too (`audio`).
`make_flags` writes `DEP_AUDIO_MINIAUDIO=y` last, after `DEP_CONSOLE=y` and the icon resource's flag (D5), which
is the old compiler's order (`qb64pe.bas` 13706–13721). `BEEP` has no dependency in the table and adds no flag.
The libqb variant with audio and libqb's audio library are built by the clone's `Makefile` into its git-ignored
folders the first time, as the window variant was.

Looked at in task 1.2 (four programs built outside the repository: `SOUND`; `$EXEICON`; both; both with a
window): the old compiler records its `make` line nowhere (`internal\temp\debug_win.bat` holds the debugger's
call only), so the order of the flags is the source's. Its compile log shows what they do: a program with
`SOUND` is compiled with `-DDEPENDENCY_AUDIO_MINIAUDIO` and linked with `audio.a`; one with `$EXEICON` is compiled
with `-DDEPENDENCY_ICON` (the `_ICON` statement the line becomes, D4) where the others have
`-DDEPENDENCY_NO_ICON`, and `windres` makes `icon.o` from `internal\temp\icon.rc`, which holds the one line
`0 ICON "icon.ico"`.

The other built-ins with that dependency (27 names in `tools\builtins\builtins.json`) (`_SNDOPEN`, `_SNDPLAY`, …) have no row in `SUPPORTED` or
`STATEMENTS` and stay "not supported yet" by name, as before.

### D4. $EXEICON in the front end

`sema` reads the metacommand as the old compiler does (`qb64pe.bas` 3829–3880; measured `v29_e01`–`e28`):

1. The line's text in upper case starts with `$EXEICON` (so `$EXEICONS:'…'` and `$EXEICON '…'` are the same
   metacommand, `e20`, `e08`).
2. A second one in the program is an error that names the first one's line, before anything else of the line is
   read (`e05`, `e27`). "Second" counts the lines that are compiled, in source order, across included files
   (`e29`: the second inside an included file; `e30`: one in a `$IF` branch that is not taken does not count).
   The line the error names is a line of the main source file: for a first one inside an included file, the
   line of its `$INCLUDE` (`e31`).
3. The name is the text between the first two `'`, without blanks at its ends; no two quotes or an empty name is
   "expected `$EXEICON:'filename'`" (`e06`, `e07`, `e17`, `e19`).
4. The rest of the line is dropped (`e09`, `e22`, `e24`). How the lexer cuts such a line today (the token may end
   at the second quote, at a `'` it takes for a comment, or at a colon) is found in task 4.1; whatever tokens of
   the line follow the name are not statements of the program.
5. The line becomes the statement `_ICON` with no argument (the same IR statement and call, `sub__icon(NULL,
   NULL,0)`), in the body it stands in, and the program needs the icon part of the runtime.
6. The program's settings get the icon: the name as written, the span of the line, and the class of the name:
   - starts with `./` or `.\`: relative to the folder of the main source file (`e01`, `e12`, `e21`);
   - absolute (a drive letter and a colon, or two leading slashes or backslashes of a share): as written. Not
     recorded in `verification\` (it would hold a local path); read from the source and tried outside the
     repository in task 1.2: a name with a drive letter is found, and a missing one is "File '…' not found"
     with the name's folder;
   - anything else: "not supported yet" at the line. The old compiler looks for such a name in its own folder
     (`e02`, `e03`, `e08`, `e18`: "File 'icon.ico' not found" for a file that lies beside the program), which
     has no counterpart here. A name with one leading slash is in this class: on Windows it is relative to a
     drive, and the old compiler takes the drive of its own folder (tried in task 1.2: such a name was found
     on the compiler's drive, and a missing one reported with that drive's letter). Listed in `SOMEDAY.md`
     under "QB64pe behaviours to review".

A `$EXEICON` line inside a comment (`'$EXEICON:'…'`, `REM $EXEICON:'…'`) is a comment and nothing else (`e14`),
which is how the compiler reads it today; a test pins it.

**Whether the file is there** is checked by the driver, not by `sema`: after the check, for every command that
reports diagnostics (compile, `--dump`, the C++-only mode, `--report unsupported`), a missing file is an error
at the line's span, counted with the others, naming the file as written. The old compiler's message names the
folder it resolved (`e04`) or only the file's last part (`e02`); naming the file as written keeps local paths
out of recorded output and is a decided difference, a row of `DIVERGENCES.md` (messages are not compared by any
test). The language server does not make this
check in this change (it would need the file system behind its loader; listed in `SOMEDAY.md`).

The IR's settings hold whether the executable carries an icon and the name as written, never a folder; the
driver resolves the name against the main source file's folder when it builds. So no dump holds a local path.

*Alternative considered:* the check in `sema` through a new method of the parser's loader. Rejected for now: the
loader's one job is source text, three implementations would grow a method for one metacommand, and the driver
has the path already.

*Alternative considered:* take a relative name without `./` as relative to the source file or the working
folder. Rejected: it would accept programs the old compiler rejects and find another file than the old compiler
finds when it accepts one.

### D5. $EXEICON in the build

For a program with an icon the driver, before `make`:

- copies the file to `<build>\temp\icon.ico` and writes `<build>\temp\icon.rc` with the old compiler's one line,
  `0 ICON "icon.ico"` (`qb64pe.bas` 13551–13604);
- passes `DEP_ICON_RC=y` after `DEP_CONSOLE=y`; the clone's `Makefile` then runs `windres` on
  `$(PATH_INTERNAL_TEMP)\icon.rc`, which the driver already points at the build folder, and links `icon.o`;
- puts the bytes of `icon.ico` and of `icon.rc` into the build key (today the key reads only the `.txt`
  fragments of that folder).

A file that is not an icon fails in `windres`, and the build fails as the old compiler's does (`e16`).

### D6. What is checked of sound, and how

| What | Check | Where |
|---|---|---|
| The call of every accepted form | equal to the old compiler's | `tests\callsite\`, one program per form |
| Rejected forms | an error that is not "not supported yet" | a `check-fail` front-end test, from `v29_x*` |
| The slots, the mask and the part of the runtime | pinned | an `ir` and a `typed` front-end test |
| What a statement queues | `PLAY(0)` read directly before and directly after each statement in the background, the difference printed in tenths of a second | slice program, console |
| When a statement holds the program | times printed as yes or no against one-sided limits far from the expected value | slice program, console |
| Run-time errors | `ERR` under a handler after `SOUND WAIT` | slice program, console |
| The game's own `PLAY` lines | `g03_capitol` compiles and its screen matches (the lines are not reached) | slice list, screen program |
| What is heard | not checked | |

Found in task 1.2 and not known when the table was written (`v29_g_voice`, `v29_h_sound_ranges`; `PlayMML` in
`audio.cpp` pauses all voices, reads the strings and resumes them): a `PLAY` with more than one string never
holds the program, whatever its voices' modes, and it ends a `SOUND WAIT`. A voice other than 0 is put in the
background by `MB` in its own string (`PLAY "", "MB"`). The wait program has a line for each; the error
program uses no `PLAY` with two strings before its last line, so that nothing it queued is released.

Three rules for the programs that run:

- **No noise.** Each slice program plays at volume 0: `SOUND`'s third argument in every `SOUND`, and `V0` at
  the start of the first `PLAY` string of each voice it uses. Measured in task 1.2 (`v29_d_volume_zero`,
  `v29_f_queue_slice`): the queue lengths and the waits at volume 0 are those at the default volume. The
  volume is a setting of the voice, kept from one statement to the next and shared by `SOUND` and `PLAY`
  (read in `audio.cpp`, `SetAmplitude` on the voice's generator; a program cannot observe it). `g03_capitol`
  plays nothing: it was cut from the game with
  the sound option off, so its `PLAY` lines are compiled and not reached.
- **`BEEP` is never run.** It is the operating system's bell, not libqb's audio. It joins the explicit list of
  built-ins that are compiled and not reached in a slice program and that must have call-site programs. That
  list is `WINDOW_BUILTINS` today; it is renamed for what it now holds (`NEVER_RUN`), with the window built-ins
  and `BEEP` each under a comment that says why.
- **No number that depends on the load.** The queue drains while the program runs, so the time between two
  readings of `PLAY(0)` is taken off what a statement added. The slice program therefore reads `PLAY(0)`
  directly before and directly after each statement (nothing between the readings but the statement; the
  `PRINT` comes after), and uses only lengths that are a whole number of tenths of a second (1.0, 0.5, 1.5,
  4.0 s), so a reading may be up to 50 ms late before the rounded number changes. `v29_a_sound_queue` does
  neither: it prints between its readings, and three of its lines sit at a rounding edge (`T120 L4 C.`, 7.5
  tenths; `T120 C8`, 2.5; `SOUND 440, 1`, 0.55). Its numbers were equal in two runs alone, which says nothing
  about a run with eight jobs; those three forms are not taken into the slice program as they are (a dotted
  note at a whole number of tenths is `T60 L4 C.`, 15). Measured in task 1.2: `v29_f_queue_slice`, written
  this way with 21 statements, gave the same output in three recordings alone and in 24 runs made eight at a
  time. A time is never printed, only compared with a
  limit far from the expected value and on the side load cannot reach: a 2-second note in the foreground must
  take over 1.2 s (expected 1.65) and has no upper limit; the same note in the background must take under
  0.3 s, many times what the call takes.

The verification programs `v29_a`–`v29_c` are the models; the slice programs are written fresh in the slice
group's style and recorded with `qb64pe.exe`.

*Alternative considered:* record the samples a program queues by a second file linked into the test program,
as the screen state is. Rejected: both compilers' programs call the same libqb with the same arguments, which
the call-site check proves form by form; the samples would test libqb.

### D7. The game

- **Tier 1** (`driver\tests\inputs.rs`, set `programs/`): the rule changes from "no error other than not
  supported yet" to "no diagnostic at all", and the set is lowered, validated and emitted like the accepted
  inputs of the other sets. The icon check of D4 is part of it (`cws.ico` lies beside the game).
- **The build**: one more `#[ignore]`d test in `driver\tests\cli.rs` builds `CWSTRAT.BAS` into a scratch folder
  and asserts that the executable is there and holds the first image of `cws.ico` (the same assertion as the
  small icon test of D5). It does not start the game. It runs where the other tests that need the clone run.
- **Once, by hand, in task 5.2**: `tools\callsite\callsite.py` on `CWSTRAT.BAS` against the old compiler's `-z`
  output. The tool compares libqb calls line by line under its normalisation rules; for a program of 4,900
  lines this is the widest comparison so far. Each difference is fixed, or explained and listed in the change's
  `tasks.md`. Whether the game then becomes a standing program of the call-site check is decided there by its
  run time (rule 8: machine time is what counts).
- **Once, by hand, in task 5.3**: the built game is started from a copy of its folder next to the old
  compiler's build, left for ten seconds and closed; that it did not end by itself with an error is recorded.
  The user is told how to start both to look and listen.

### D8. Order

1. The rest of the measurements (group 1).
2. The four rows, the audio part, the call-site and front-end tests (group 2). After it the game's report holds
   only `$EXEICON`.
3. The slice programs for sound and `g03_capitol` (group 3).
4. `$EXEICON`, front end and build (group 4).
5. The game: tier 1, the build test, the two one-off comparisons (group 5).
6. The closing runs and documents (group 6).

## Risks / Trade-offs

- [A machine without a sound device: `PLAY(0)` stays 0 and nothing holds the program, so the queue and wait
  programs fail there, and GitHub's Windows runner may be such a machine] → not known until the `tier2` job is
  run by hand (CI is not a gate, `CLAUDE.md` working rule 9). If it is, the two programs move from `slice.list`
  to a list run locally, as planned for the screen programs. The error program is written so that it passes
  either way where the old compiler's order of checks allows. Read in `audio.cpp` (`PlayWaveform`, `PlayMML`;
  no such machine is at hand): `SOUND` with no argument and a frequency, a duration or a voice out of range
  are checked before the device is asked for and raise error 5 on any machine; a volume, a pan position, a
  waveform and a waveform parameter are checked after it and raise nothing without a device; without one no
  `PLAY` string is read at all, so none of its errors is raised. The error program therefore has the four
  checks of any machine in a part of their own, which is what stays on `slice.list` if the runner has no
  device.
- [A wait measured under eight jobs is longer than alone] → the limits are one-sided where load can only add
  time ("took over 1.2 s", with no "under" beside it as `v29_b_sound_wait` has), and the "returned at once"
  limits are many times the expected value. The closing
  runs are made without `--pass-cache`, which cannot see a result that depends on time.
- [The first build with sound compiles libqb's audio part: minutes, once per clone] → named in the proposal;
  the build cache and the clone's objects keep it.
- [The slice run gets longer: the wait program must wait for its sounds] → they add up to under six seconds.
- [The call-site comparison of the whole game finds differences that are not about sound] → each is a bug or a
  gap of an earlier step; it is fixed here when small, or listed with its line and taken as its own change
  when not, and the user is told.
- [`$EXEICON` semantics differ for a relative name without `./`] → marked, not guessed (D4).

## Open Questions

None open. The three put to the user in task 1.3 were each answered with the default the design is written for
(`DECISIONS.md` 2026-10-13):

1. Tests of sound play at volume 0, and `BEEP` is never run (D6): yes.
2. A relative `$EXEICON` name without `./` stays "not supported yet" (D4): yes.
3. The game is built by a test and started once by hand, not run by a test (D7): yes.
