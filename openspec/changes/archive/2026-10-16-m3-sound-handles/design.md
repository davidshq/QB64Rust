# Design

## Context

See `proposal.md` for why and for what was read and tried before it. What shapes the approach:

- A plain built-in comes in as a row, a call-site program and a slice line (`DECISIONS.md` 2026-10-10). The C++ of
  every form tried is the call of its table entry and template (proposal), as it was for `SOUND` and `PLAY`
  (design D1 of `m3-sound`), and `sema` already has every slot kind these entries use: `LONG`, `SINGLE`, `DOUBLE`,
  `_UNSIGNED LONG`, `STRING`, and the array of `offset+size (largest safe block…)` (`Slot::Block`, which graphics
  `GET`/`PUT` and `_UCHARPOS` use).
- `RuntimePart::Audio` exists: `sema` maps the table's `DEPENDENCY_MINIAUDIO` to it (`lib.rs`), and the driver
  writes `DEP_AUDIO_MINIAUDIO=y` for it. The libqb variant with audio and `audio.a` are built in the clone's
  git-ignored folders. MIDI, its default sound bank and the other decoders are in `audio.a` (`audio\build.mk`).
- `_MEMSOUND` has a row since `m3-mem` and is on `NEVER_RUN` because no supported built-in gave a handle.
- A corpus program may have data files in `<name>.files\`, and the runner runs each program in a fresh folder. An
  upstream program on `pass.list` runs from the clone, where its `.mid` and `.sf2` files lie.
- The tests' machine has a sound device; whether GitHub's Windows runner has one is still unseen (`STATUS.md`).

## Goals / Non-Goals

**Goals:**

- Every name of the audio part has a row; each accepted form writes the old compiler's call and each rejected form
  is an error that is not "not supported yet".
- What a program can observe of a sound handle is compared with the old compiler's program, with nothing heard and
  no number that depends on the machine or the load.
- The seven upstream programs the proposal names pass.

**Non-Goals:**

- What is heard; the samples libqb writes to the device (they come from the same libqb, called with the same
  arguments, which the call-site check proves form by form; design D6 of `m3-sound`).
- `$EMBED` and `_EMBEDDED$`.
- The behaviour on a machine without a sound device.

## Decisions

### D1. Plain rows, and _WAVE with them

The ten functions join `SUPPORTED` as `Rule::Plain` and the sixteen statements join `STATEMENTS` as
`StmtRule::Plain`; their result types are the table's (`LONG` handles, `DOUBLE` lengths and positions). Where a
form does not come out as the old compiler's, the fix goes into the template plan or the parser for every
statement, as in `m3-sound`, and task 2.1 says which.

`_WAVE` is not a `_SND` name, but it is the last built-in of the audio part without a row (with it every name of
`DEPENDENCY_MINIAUDIO` has one, and the row of `SOMEDAY.md` for "the other sound built-ins" goes), it uses
nothing the others do not, and its checked form is the same as `_SNDRAWBATCH`'s (D2). Leaving it out would leave
one name of the family marked for no saving (working rule 8: it is a row, a check shared with `_SNDRAWBATCH`, a
call-site program and a line of a slice program).

*Alternative considered:* a rule of its own for the handle functions (a `Handle` slot that checks nothing more
than a `LONG`). Rejected: the C++ shows no special case, and a handle is a `LONG` to the old compiler.

### D2. The arrays of _WAVE and _SNDRAWBATCH

The old compiler checks these two itself, after the argument is read (`qb64pe.bas` 11809–11847): `_WAVE`'s second
argument must be an array reference whose type is exactly `_BYTE` (signed, 8 bits, no string, float, offset or
user type), else "Expected _BYTE array name"; `_SNDRAWBATCH`'s first must be an array reference of a 32-bit float,
else "Expected SINGLE array name". The slot stays `Slot::Block` (the bytes from an element to the array's end, or
the whole array); `check\builtins.rs` adds the element type check for these two calls by name, after the slot is
read, with the old compiler's message. Measured by task 1.1 (`study\00` §5, `v39`): a whole array `w()`, an
element `w(1)`, the array's name alone `w` (the whole array), a two-dimensional array, an array named with its
suffix and an array parameter of a `SUB` are accepted; a member array `x.m()` is "Expected array index" (as for
graphics `PUT`, whatever `Slot::Block` already says), a number is the old compiler's internal error (any error
here), and every other type, `_UNSIGNED _BYTE` and `_FLOAT` included, a variable and a member of an element are
the message of the check.

*Alternative considered:* a `Slot::Block` with an element type taken from the table. Rejected: the table has no such
type (`arg_types` is the same placeholder for graphics `PUT`), and two names do not repay a new slot kind.

### D3. $MIDISOUNDFONT

`sema` gives the old compiler's error for a metacommand whose text in upper case starts with `$MIDISOUNDFONT:`
(its message, not "not supported yet"), where it reads the other metacommands. Measured by task 1.1 (`v39_x73`–
`x81`): any case and anything after the colon; after `'` or `REM` it is a comment. `$MIDISOUNDFONT` without the
colon (alone, `$MIDISOUNDFONT :x`, `$MIDISOUNDFONTS:x`) is the old compiler's "Syntax error" for an unknown
metacommand, which `sema` reports "not supported yet" for every unknown metacommand; that stays as it is (not
this change's scope).

### D4. _MEMSOUND leaves NEVER_RUN

It stays the row it is; the slice program of D5 runs it on a `_SNDNEW` sound and on a sound opened from memory
(the block of a sound that is decoded), and the coverage check then wants it run like any other built-in. Its
`SOUND` member is read from the block as for `_MEMIMAGE`. A decoded sound's block holds its samples at the
device's rate (`v39_b`), so of that block only what does not depend on the rate is printed (`TYPE`,
`ELEMENTSIZE`, `SIZE` compared with `_SNDRATE`).

### D5. What is checked, and how

| What | Check | Where |
|---|---|---|
| The call of every accepted form | equal to the old compiler's | `tests\callsite\snd*.bas`, one program per form |
| Rejected forms, the arrays of D2, `$MIDISOUNDFONT` | an error that is not "not supported yet" | a `check-fail` front-end test, from `v39_x*` |
| Slots, result types, the audio part | pinned | an `ir` and a `typed` front-end test |
| Open, length, position, copy, playing and paused, a handle that is not open | printed exactly | slice program, console |
| `_SNDNEW` and `_MEMSOUND`, from memory and new | printed exactly | slice program, console |
| `_SNDOPEN` of a file, `_SNDPLAYFILE`, `_SNDLIMIT`, `_SNDBAL` | the file written by the program; what is exact printed | slice program, console |
| Raw sound, `_SNDRATE` | one-sided limits (`> 0`, `<= 1`) | slice program, console |
| MIDI: the default bank, a bank that is not there, a MIDI from memory | handle and length | slice program, console |
| A bank from a file and from memory, a MIDI file | the upstream programs | `tests\upstream\pass.list` |
| What is heard | not checked | |

Rules for the programs that run, from those of `m3-sound` (its design D6):

- **No noise.** A sound is played only after `_SNDVOL h, 0`, or by `_SNDPLAYCOPY h, 0` and `_SNDPLAYFILE f$, , 0`;
  raw streams get `_SNDVOL r, 0` and samples of 0 (the default stream, which takes no volume, gets samples of 0
  only). The data are silence: a WAV of zero samples, a `_SNDNEW` sound (silent until written), and a MIDI note
  played at volume 0. Task 1.1 measured that volume 0 changes none of the values printed (`v39_a`, `v39_c` run by
  hand at the default volume).
- **No binary file in the repository.** The WAV and MIDI data are built in strings by the program (as the
  proposal's two programs did); the WAV file `_SNDOPEN` and `_SNDPLAYFILE` read is written by the program into its
  folder. **Its open is tried again until it gives a handle** (a few tenths of a second at most): once in 16 runs
  made eight at a time the open of a file just written gave 0 (`study\00` §5, `v39`). **It is removed with `KILL`
  tried again until it raises no error**: a file `_SNDPLAYFILE` plays stays open until the sound loop frees it
  (error 75 until then).
- **No number that depends on the machine or the load.** `_SNDRATE` is printed only as `> 0`. `_SNDRAWLEN` is read
  directly after the samples are queued and compared with one-sided limits. A playing state is read right after
  the statement that sets it, on a sound of ten seconds, so it cannot end first. **Lengths and positions of
  decoded sounds are whole half-seconds** (they are counted in frames of the device's rate, exact for those at
  44100 and 48000 Hz; `.125` reads `.1249886602163315` at 44100 Hz); a `_SNDNEW` sound's length does not depend on
  the device. **Handle numbers are printed only before the first handle is closed or given to `_SNDPLAYCOPY` or
  `_SNDPLAYFILE`**: libqb frees a closed handle in its sound loop, so which number the next open gets depends on
  time (`CloseSound` sets `autoKill`); after that a program prints `h > 0`. Nor is `_MEMEXISTS` of a closed sound's
  block printed (freed in the same loop). Task 1.1 ran each measurement program eight at a time, as `m3-sound`
  did; task 3.1 does the same for each slice program before it is recorded.

Slice programs (numbers from `s127`; task 3.1 names them): handles and their state; new sounds and memory blocks;
files, raw sound and MIDI. Each recorded twice with `qb64pe.exe`, the recordings equal.

### D6. The upstream programs

The seven programs of the proposal are run from the clone after group 2 and join `pass.list`, or
`tests\known_clean_not_passing.list` with the reason. `audio_test/test` has no `.output`; how the upstream runner
counts such a program is read in task 3.3, which says it. `midisoundbank_embed_test` stays held back by `$EMBED`.

### D7. Order

1. The measurements (group 1).
2. The rows, the array checks, `$MIDISOUNDFONT`, the call-site and front-end tests (group 2).
3. The slice programs, the upstream programs, the shrink-only lists (group 3).
4. The closing runs and documents (group 4).

## Risks / Trade-offs

- [A machine without a sound device: libqb's engine is not initialised, every function gives 0 and every
  statement does nothing, so the slice programs of D5 fail there] → as for `s75` and `s76` of `m3-sound`: not
  known until the `tier2` job runs by hand (CI is not a gate, working rule 9). If the runner has no device, these
  programs move with `s74`–`s76` to a list run locally, as the screen programs did.
- [A value printed exactly depends on decoding done in a thread (`MA_SOUND_FLAG_ASYNC` is libqb's default for
  `_SNDOPEN`)] → the proposal's two programs gave equal output in two runs; task 1.1 runs each slice program eight
  at a time, and a value that differs is printed with a limit instead, or read after a `"noasync"` open.
- [Handles freed in the sound loop] → handle numbers printed only before the first close (D5).
- [An access violation after the last line: two of about 50 runs of `v39_a`, both the first after the executable
  was copied, none in the 48 after] → libqb's, not the compiler's (the old compiler's program); if a slice
  program shows it, its runs are counted and the fact goes into `study\00` §5, and the program is not changed to
  hide it.
- [The arrays of D2 are a check by name] → the two names are the old compiler's own special cases; a test pins
  every accepted and rejected form task 1.1 finds.
- [A questionable behaviour kept] → `_SNDPAUSED` of a sound just opened or stopped is -1; `_SNDLEN` of a `_SNDNEW`
  sound is a SINGLE widened; `_SNDPLAYFILE`'s second argument is read and ignored; a handle that is not open raises
  no error. Each implemented the old compiler's way and listed in `SOMEDAY.md` ("QB64pe behaviours to review").
