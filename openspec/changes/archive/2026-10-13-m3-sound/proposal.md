# Proposal

## Why

This is step 3 of M3's order of work (`DECISIONS.md` 2026-10-12): sound, and with it `$EXEICON` and the first
build of the whole of Civil War Strategy, the real-program target of M3. After `m3-get-put-bload` the game has 66
marked errors in 4 messages and nothing else (`qb64rust --report unsupported tests\programs`, 2026-10-13):

| Uses | Not supported yet | In this change |
|---|---|---|
| 33 | `SOUND` | yes |
| 31 | `PLAY` | yes |
| 1 | `BEEP` | yes |
| 1 | the metacommand `$EXEICON` | yes |

One screen program of the corpus waits on the same thing: `g03_capitol` (`PLAY`).

Measured before this proposal (`verification\v29_*`, 110 programs, and the old compiler's C++ read with `qb64pe -z`
outside the repository; the facts go into `study\00` §5 in task 1.1):

- **Every sound form is a plain call** of its row and template: `sub_sound` with eight slots and a mask,
  `sub_play` with four strings and a mask, `func_play` with one optional slot, `sub_beep()`. 43 of the 79
  programs tried are accepted, among them `SOUND` with no argument (error 5 when it runs), `SOUND WAIT`,
  `SOUND RESUME`, a trailing comma (`SOUND 440, 1,` and `PLAY t,`), and `ON PLAY(5) GOSUB 10`, which is
  `ON … GOSUB` with the function `PLAY` as its number. `PLAY ON`, `PLAY OFF` and `PLAY STOP` are rejected. The
  parser here reads `ON PLAY` and `PLAY ON` as QuickBASIC's event forms today and marks them "not supported yet".
- **`SOUND` and `PLAY` need the audio part of libqb** (`DEP_AUDIO_MINIAUDIO=y`), `BEEP` does not.
- **What can be observed of sound from a program:** `PLAY(n)`, the seconds left to play of a voice; how long a
  statement holds the program; and run-time errors. A sound in the foreground (the default) holds the program
  for about 0.95 times its length minus a quarter of a second; after `PLAY "MB"` the statement returns at once
  and `PLAY(0)` gives what is queued, equal in tenths of a second from run to run. Sound held back by
  `SOUND WAIT` is not in that number (`PLAY(0)` is 0 until `SOUND RESUME`). What is heard cannot be observed.
- **`$EXEICON:'file'` is a statement and a build setting.** Where it stands, the old compiler writes the call of
  `_ICON` with no argument; the build gets a resource file naming a copy of the icon, and one more make flag. A
  name that starts with `./` or `.\` is looked for in the folder of the main source file; any other relative
  name in the old compiler's own folder. What follows the closing quote on the line is dropped, a statement
  after a colon included.

## What Changes

- **`SOUND`, `PLAY`, `BEEP`** as statement rows and **`PLAY`** as a function row, in every form the old compiler
  accepts; its rejected forms are rejected. A line `BEEP:` or `SOUND:` is the statement, as `CLS:` is. `PLAY`
  leaves the parser's event words: `ON PLAY(n) GOSUB label` is `ON … GOSUB`, and `PLAY ON`, `PLAY OFF` and
  `PLAY STOP` are errors, as in the old compiler.
- **The audio part of the runtime**: a program that names `SOUND` or `PLAY` (statement or function) is built
  with the make flag the old compiler passes, against the libqb variant with audio, built once into the clone's
  git-ignored folders.
- **`$EXEICON`**: the forms the old compiler accepts with a name that starts with `./` or `.\`, or an absolute
  name (a drive letter or a share); its errors (no name, a second `$EXEICON`, a file that is not there). The statement, the resource and the
  make flag as the old compiler's. A relative name without `./` stays "not supported yet": it means "in the
  compiler's own folder" to the old compiler, which has no counterpart here.
- **Civil War Strategy compiles and builds**: tier 1 requires no diagnostic at all for it and emits its C++; a
  test that needs the clone builds its executable. Its C++ is compared once with the old compiler's by the
  call-site tool, and what differs is fixed or listed. It is started once by hand next to the old compiler's
  build. It is not run by any test (stage 3 of `tests\programs\README.md`, a scripted game, stays later).
- **Tests of sound that make no noise**: console programs on `tests\corpus\slice.list` that play at volume 0
  and print what can be observed (the queue in tenths of a second, which statements hold the program as yes or
  no against wide limits, the run-time errors). `BEEP` is compiled, compared by the call-site check and held in
  a slice program as code that is not reached: it rings the machine's bell. `g03_capitol` joins the slice list
  (its five `PLAY` lines are compiled and not reached: the program was cut with the game's sound option off).
- **Tests and lists**: `verification\v29_*`; a call-site program per form; front-end tests; the shrink-only
  lists regenerated.
- **Two leftovers of the last change** (`STATUS.md`): the purpose of the main spec `language\conventional-memory`
  written in place of its placeholder; the stale open item about its uncommitted archive removed.

## Capabilities

### New Capabilities

- `language/sound`: `SOUND`, `PLAY` (statement and function) and `BEEP`: the forms that compile, what a statement
  queues and when it holds the program, the run-time errors, as the old compiler's program behaves.

### Modified Capabilities

- `language/builtin-statements`: `SOUND`, `PLAY`, `BEEP` join the list (the scenario of an unsupported statement
  names another one).
- `language/builtin-functions`: `PLAY` joins the list.
- `language/console-and-window`: `$EXEICON`.
- `compiler/pipeline`: the icon of the executable among a program's settings; the audio part and the icon
  resource in the build.
- `testing/compiler-tests`: a real program taken whole compiles with no diagnostic; a built-in that no test runs
  need not be a window built-in (`BEEP`).

## Impact

- Changed: `crates\sema` (`builtins.rs`: four rows; the list of built-ins no test runs; `lib.rs`,
  `check\mod.rs`: the audio part, `$EXEICON` and the icon among the settings), `crates\ir` (the part, the icon),
  `crates\driver` (`build.rs`: the two make flags, the icon's copy and resource file, the build key; the check
  that the icon file is there; `tests\cli.rs`, `tests\inputs.rs`), `crates\syntax` (`PLAY` out of the event
  words; else only where a form does not parse today), `tools\callsite` only if a program with a file beside it needs it, `tests\corpus\slice.list`,
  the shrink-only lists; new `tests\callsite\*.bas`, `tests\frontend\*.bas`, `tests\corpus\slice\*`,
  `verification\v29_*`.
- No change to libqb or the reference clone (its git-ignored folders get the audio variant of libqb, as they
  got the window variant). No change to `tools\screen\screen_state.cpp`: sound stays outside the screen state.
- The first build of a program with sound compiles libqb's audio part where it is not built yet (minutes,
  once; on CI in the `tier2` job).
- Not in this change: the other sound built-ins (`_SNDOPEN`, `_SNDPLAY`, `_SNDRAW`, `_WAVE`, `_SNDRATE`, …);
  `$VERSIONINFO`; `ON PLAY(n) GOSUB` as QuickBASIC's event (QB64pe has none; the line above is all it is); a line number
  as the target of `ON … GOSUB` (not compiled yet, whatever the number is);
  `VARPTR$` in a `PLAY` string (`"X" + VARPTR$(a$)`); a relative `$EXEICON` name without `./`; a check of what
  is heard; running the game in a test; the console programs of step 4.
- `STATUS.md`, `ROADMAP.md`, `crates\README.md`, `tests\corpus\README.md`, `tests\programs\README.md`,
  `study\00` §5, `SOMEDAY.md` (what stays marked; questionable behaviours), `DIVERGENCES.md` (the wording of
  the missing-icon error), `DIVERGENCES-QB45.md` (`PLAY` values QuickBASIC rejects) and `CLAUDE.md`'s layout
  table updated when done.
