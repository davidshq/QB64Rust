# language/sound Specification

## Purpose
The sound statements of a program, `SOUND`, `PLAY` and `BEEP`, the function `PLAY`, and the sound handles
(`_SNDOPEN` and the `_SND*` family, `_MIDISOUNDBANK`, `_WAVE`): which forms compile, what a statement queues, when
it holds the program, what a handle gives a program and which arguments are a run-time error, as the old
compiler's program behaves.

## Requirements

### Requirement: Forms of SOUND
`SOUND` SHALL compile in every form the old compiler accepts, each with the runtime call the old compiler writes
for it: no argument; a frequency and a duration; after them, in this order and each optional, a volume, a pan
position, a waveform, a waveform parameter and a voice; and `WAIT` or `RESUME` as the last word, alone or after
the arguments. A form the old compiler rejects SHALL be a compile error that is not "not supported yet".

#### Scenario: Frequency and duration
- **WHEN** `SOUND 440, 1` and `SOUND 37 + 50 * RND, .03` are compiled
- **THEN** each compiles to the call the old compiler writes for it

#### Scenario: Optional arguments
- **WHEN** `SOUND 440, 1, .5`, `SOUND 440, 1, , 0`, `SOUND 440, 1, , , 4`, `SOUND 440, 1, , , , .5`,
  `SOUND 440, 1, , , , , 1` and `SOUND 440, 1, .5, 0, 4, .5, 1` are compiled
- **THEN** each compiles to the call the old compiler writes for it

#### Scenario: WAIT and RESUME
- **WHEN** `SOUND WAIT`, `SOUND RESUME`, `SOUND 440, 1 WAIT` and `SOUND 440, 1, .5 RESUME` are compiled
- **THEN** each compiles to the call the old compiler writes for it

#### Scenario: Arguments of any numeric type
- **WHEN** `SOUND i%, l&`, `SOUND q&&, d#` and `SOUND 440, 1, d#, q&&, s!, i%, d#` are compiled
- **THEN** each argument is converted as the old compiler converts it

#### Scenario: A trailing comma
- **WHEN** `SOUND 440, 1,` is compiled
- **THEN** it compiles as `SOUND 440, 1` does, as with the old compiler

#### Scenario: With CALL, and a colon after the name
- **WHEN** a program has `CALL SOUND(440, 1)` and a line `SOUND: l = 1`
- **THEN** the first compiles as `SOUND 440, 1` does and the second is `SOUND` with no argument and then the
  assignment

#### Scenario: Rejected forms
- **WHEN** a program has one of `SOUND 440`, `SOUND , 1`, `SOUND 440, , .5`, `SOUND 440, 1, WAIT`,
  `SOUND WAIT RESUME`, `SOUND t$, 1`, `SOUND(440, 1)`, `SOUND 440, 1, , , , ,` or `SOUND` with eight arguments
- **THEN** the compiler reports an error that is not "not supported yet"

### Requirement: Forms of the PLAY statement
`PLAY` SHALL compile with one to four string expressions, the first required and each later one optional, each
with the runtime call the old compiler writes for it. A fixed-length string SHALL be a string. A form the old
compiler rejects SHALL be a compile error that is not "not supported yet".

#### Scenario: One string
- **WHEN** `PLAY "c"`, `PLAY t$`, `PLAY "c" + t$` and `PLAY (t$)` are compiled
- **THEN** each compiles to the call the old compiler writes for it

#### Scenario: Several voices
- **WHEN** `PLAY t$, t$`, `PLAY t$, , t$`, `PLAY t$, , , t$` and `PLAY t$, t$, t$, t$` are compiled
- **THEN** each compiles to the call the old compiler writes for it, which says which strings were given

#### Scenario: A trailing comma
- **WHEN** `PLAY t$,` is compiled
- **THEN** it compiles as `PLAY t$` does, as with the old compiler

#### Scenario: Rejected forms
- **WHEN** a program has one of `PLAY`, `PLAY 1`, `PLAY , t$`, `PLAY(t$, t$)`, `PLAY ON`, `PLAY: l = 1` or `PLAY`
  with five strings
- **THEN** the compiler reports an error that is not "not supported yet" (`PLAY ON` is no event switch here)

### Requirement: The PLAY function
`PLAY(voice)` and `PLAY` without an argument SHALL compile as the old compiler compiles them: a DOUBLE, the
seconds of sound left to play in the voice (voice 0 without an argument and for a voice that does not exist).
The argument SHALL be a number of any type, converted as the old compiler converts it.

#### Scenario: With and without an argument
- **WHEN** `d# = PLAY(0)`, `d# = PLAY` and `l& = PLAY(1) + 1` are compiled
- **THEN** each compiles to the call the old compiler writes for it

#### Scenario: Nothing queued
- **WHEN** a program that has played nothing runs `PRINT PLAY(0); PLAY(1); PLAY(4) = PLAY(0)`
- **THEN** it prints ` 0  0 -1 `, as with the old compiler

#### Scenario: A voice that does not exist
- **WHEN** a program with sound queued in voice 0 and in no other voice runs
  `PRINT PLAY(4) > 0; PLAY(-1) > 0; PLAY(1)`
- **THEN** it prints `-1 -1  0 `, as with the old compiler

#### Scenario: Rejected forms
- **WHEN** a program has one of `d# = PLAY()`, `d# = PLAY(t$)`, `d# = PLAY(0, 1)` or `PLAY = 1`
- **THEN** the compiler reports an error that is not "not supported yet"

#### Scenario: As the number of ON GOSUB
- **WHEN** a program has `ON PLAY(5) GOSUB done` and a label `done`
- **THEN** it compiles as `ON … GOSUB` with the function's value as its number, as with the old compiler, and
  not as an event handler

#### Scenario: No event switch
- **WHEN** a program has `PLAY ON`, `PLAY OFF` or `PLAY STOP`
- **THEN** the compiler reports an error that is not "not supported yet", as the old compiler rejects each

### Requirement: BEEP
`BEEP` SHALL compile to the runtime call the old compiler writes for it, with `CALL` and with a colon after its
name too. `BEEP` with an argument or with parentheses SHALL be a compile error that is not "not supported yet".

#### Scenario: The statement
- **WHEN** `BEEP`, `CALL BEEP`, `IF l THEN BEEP ELSE BEEP` and a line `BEEP: l = 1` are compiled
- **THEN** each `BEEP` compiles to the call the old compiler writes for it

#### Scenario: Rejected forms
- **WHEN** a program has `BEEP 1`, `BEEP()` or `l = BEEP`
- **THEN** the compiler reports an error that is not "not supported yet"

### Requirement: Sound in the foreground holds the program
A `SOUND` or `PLAY` statement SHALL hold the program while its sound plays, and return at once after
`PLAY "MB"`, after `SOUND WAIT`, and for a sound shorter than about a quarter of a second, as the old compiler's
program does. `PLAY "MF"` SHALL make statements hold the program again. A `PLAY` with more than one string SHALL
never hold the program, and SHALL end a `SOUND WAIT`, as in that program.

#### Scenario: Foreground
- **WHEN** a program times `SOUND 440, 18.2` (one second) and `PLAY "T120 L1 C"` (two seconds)
- **THEN** the first takes over 0.4 seconds and the second over 1.2, as with the old compiler

#### Scenario: Foreground again
- **WHEN** a program with three seconds of sound queued in the background runs `PLAY "MF"` and then times
  `SOUND 440, 1`
- **THEN** the statement takes over 1.5 seconds: it waits for what is queued

#### Scenario: Background
- **WHEN** a program runs `PLAY "MB"` and then times `PLAY "T120 L1 C"`
- **THEN** the statement takes under 0.3 seconds and `PLAY(0)` right after it is over 1.5

#### Scenario: Held back
- **WHEN** a program runs `SOUND WAIT`, then `SOUND 440, 18.2`, then `SOUND RESUME`
- **THEN** neither `SOUND 440, 18.2` nor `SOUND RESUME` holds the program, `PLAY(0)` is 0 before `SOUND RESUME`
  and over 0.5 after it

#### Scenario: Several strings
- **WHEN** a program whose voice 2 is in the foreground times `PLAY "", , "V0 T120 L1 C"` (two seconds)
- **THEN** the statement takes under 0.3 seconds, as with the old compiler

#### Scenario: Several strings end a SOUND WAIT
- **WHEN** a program runs `SOUND WAIT`, then `SOUND 440, 18.2, 0`, then `PLAY "", ""`
- **THEN** `PLAY(0)` is 0 before the `PLAY` and over 0 after it

### Requirement: What a statement queues
In the background, each `SOUND` and `PLAY` statement SHALL add to `PLAY(0)` the length of its sound, as the old
compiler's program does: a `SOUND` its duration in clock ticks of 1/18.2 second, a `PLAY` string the lengths of
its notes and pauses by its tempo, lengths and dots. The tempo, length and octave of one `PLAY` SHALL hold for
the next.

#### Scenario: SOUND durations
- **WHEN** a program in the background runs `SOUND 440, 18.2`, `SOUND 880, 9.1`, `SOUND 0, 18.2` and
  `SOUND 440, 0`, printing after each what it added to `PLAY(0)` in tenths of a second
- **THEN** it prints 10, 5, 10 and 0, as with the old compiler

#### Scenario: PLAY strings
- **WHEN** a program in the background runs `PLAY "T120 L4 C"`, `PLAY "C"`, `PLAY "T120 L4 C D E"`,
  `PLAY "T60 L1 C"`, `PLAY "T120 P4"` and `PLAY ""`, printing the same
- **THEN** it prints 5, 5, 15, 40, 5 and 0, as with the old compiler

#### Scenario: A voice of its own
- **WHEN** a program whose voice 0 is in the background runs `SOUND 440, 18.2, , , , , 1`
- **THEN** the statement holds the program (the background is set per voice) and adds nothing to `PLAY(0)`

#### Scenario: A voice put in the background
- **WHEN** the same program runs `PLAY "", "MB"` and then `SOUND 440, 18.2, , , , , 1` and `PLAY "", "T120 L4 C"`
- **THEN** neither holds the program and they add 10 and 5 tenths of a second to `PLAY(1)`

#### Scenario: Volume 0
- **WHEN** a program runs `SOUND 440, 18.2, 0` and `PLAY "V0 T120 L4 C"`, in the background and in the foreground
- **THEN** each adds to `PLAY(0)` what it adds at any volume, and in the foreground holds the program as long

### Requirement: Run-time errors of sound
An argument outside its range SHALL raise error 5 where the old compiler's program raises it, and the program
SHALL go on after a handler as that program does: `SOUND` with no argument; a frequency below 20 other than 0, or
above 32767; a duration below 0 or above 65535; a volume outside 0 to 1; a pan position outside -1 to 1; a
waveform outside 1 to 10; a waveform parameter outside 0 to 1 in a statement that gives waveform 9; a voice outside 0 to 3.

#### Scenario: The waveform parameter has a range with waveform 9 only
- **WHEN** a program under a handler runs `SOUND 440, 1, , , 9, 0`, then `SOUND 440, 1, , , 4, 2` and
  `SOUND 440, 1, , , , 2` after `SOUND WAIT`
- **THEN** none raises an error: the parameter is checked against the waveform given in the same statement

#### Scenario: Several arguments out of range
- **WHEN** a program under a handler runs `SOUND 10, 1, 2` and `SOUND 440, 1, 2, , , , 4`
- **THEN** each raises error 5 once

#### Scenario: SOUND arguments
- **WHEN** a program under a handler runs `SOUND 19, 1`, `SOUND 32768, 1`, `SOUND 440, -1`, `SOUND 440, 65536`,
  `SOUND 440, 1, 1.5`, `SOUND 440, 1, , 2`, `SOUND 440, 1, , -1.1`, `SOUND 440, 1, , , 11`,
  `SOUND 440, 1, , , -1`, `SOUND 440, 1, , , 9, 2`, `SOUND 440, 1, , , 9, -.1`, `SOUND 440, 1, , , , , 4`,
  `SOUND 440, 1, , , , , -1` and `SOUND`
- **THEN** each raises error 5

#### Scenario: Accepted limits
- **WHEN** the same program runs `SOUND 0, 1`, `SOUND 20, 1`, `SOUND 32767, 1`, `SOUND 440, 0`,
  `SOUND 440, 65535`, `SOUND 440, 1, 0`, `SOUND 440, 1, 1`, `SOUND 440, 1, , 1`, `SOUND 440, 1, , -1`,
  `SOUND 440, 1, , , 10`, `SOUND 440, 1, , , 9, 0`, `SOUND 440, 1, , , 9, 1`, `SOUND 440, 1, , , 4, 2`,
  `SOUND 440, 1, , , , 2` and `SOUND 440, 1, , , , , 3` after `SOUND WAIT`
- **THEN** none raises an error

### Requirement: Run-time errors of a PLAY string
A `PLAY` string, of any voice, with a command that does not exist, a length of 0 or above 64, a note number
above 84 or a volume above 100 SHALL raise error 5, as the old compiler's program does. A tempo or an octave outside QuickBASIC's
range (`T31`, `T256`, `O7`) SHALL raise no error, as in that program.

#### Scenario: PLAY strings
- **WHEN** a program under a handler runs `PLAY "H"`, `PLAY "L0 C"`, `PLAY "L65 C"`, `PLAY "N85"`, `PLAY "V101"`
  and `PLAY "C", "H"`
- **THEN** each raises error 5, and `PLAY "O6 C"`, `PLAY "O7 C"`, `PLAY "T31 C"`, `PLAY "T256 C"`, `PLAY "V0"`,
  `PLAY "V100"` and `PLAY` of an empty string raise none

### Requirement: Forms of the sound handle functions
The functions `_SNDOPEN`, `_SNDCOPY`, `_SNDLEN`, `_SNDGETPOS`, `_SNDPLAYING`, `_SNDPAUSED`, `_SNDRATE`,
`_SNDOPENRAW`, `_SNDRAWLEN` and `_SNDNEW` SHALL compile in every form the old compiler accepts, each to the call it
writes, with its arguments converted as the old compiler converts them. A form it rejects SHALL be an error that
is not "not supported yet".

#### Scenario: Required and optional arguments
- **WHEN** `h = _SNDOPEN(f$)`, `h = _SNDOPEN(w$, "memory")`, `n = _SNDNEW(800, 1, 16, 8000)` and `n = _SNDNEW(800)`
  are compiled
- **THEN** each compiles to the call the old compiler writes for it, which says which arguments were given

#### Scenario: Called bare
- **WHEN** `r = _SNDOPENRAW`, `PRINT _SNDRATE` and `d = _SNDRAWLEN` are compiled
- **THEN** each compiles to the call the old compiler writes for it, the default raw stream where a handle is
  left out

#### Scenario: Arguments of any numeric type
- **WHEN** `PRINT _SNDLEN(d#)`, `PRINT _SNDPLAYING(i%)` and `n = _SNDNEW(q&&, s!)` are compiled
- **THEN** each argument is converted as the old compiler converts it

#### Scenario: Rejected function forms
- **WHEN** a program has one of `h = _SNDOPEN()`, `h = _SNDOPEN(1)`, `h = _SNDOPEN(f$, "", 1)`, `h = _SNDPLAYING`
  or `l = _SNDLEN(f$)`
- **THEN** the compiler reports an error that is not "not supported yet"

### Requirement: Forms of the sound handle statements
The statements `_SNDCLOSE`, `_SNDPLAY`, `_SNDPLAYCOPY`, `_SNDPLAYFILE`, `_SNDPAUSE`, `_SNDSTOP`, `_SNDLOOP`,
`_SNDVOL`, `_SNDBAL`, `_SNDSETPOS`, `_SNDLIMIT`, `_SNDRAW`, `_SNDRAWBATCH`, `_SNDRAWDONE`, `_MIDISOUNDBANK` and
`_WAVE` SHALL compile in every form the old compiler accepts, each to the call it writes. A form it rejects SHALL be
an error that is not "not supported yet".

#### Scenario: Statements with optional arguments
- **WHEN** `_SNDBAL h, 1`, `_SNDPLAYCOPY h, 0`, `_SNDPLAYFILE f$, , 0`, `_SNDRAW 0, 0, r`, `_SNDRAW 0`,
  `_SNDRAWDONE` and `_MIDISOUNDBANK f$` are compiled
- **THEN** each compiles to the call the old compiler writes for it, which says which arguments were given

#### Scenario: Statement arguments of any numeric type
- **WHEN** `_SNDVOL i%, d#` and `_SNDSETPOS q&&, s!` are compiled
- **THEN** each argument is converted as the old compiler converts it

#### Scenario: Rejected statement forms
- **WHEN** a program has one of `_SNDPLAY`, `_SNDPLAY h, 1`, `_SNDVOL h`, `_SNDPLAY f$` or `x = _SNDPLAY(h)`
- **THEN** the compiler reports an error that is not "not supported yet"

### Requirement: The arrays of _WAVE and _SNDRAWBATCH
`_WAVE voice, a()` SHALL take only an array of `_BYTE` and `_SNDRAWBATCH a()` only an array of `SINGLE`, whole
(also named without its parentheses) or from one of its elements, as the old compiler does; any other argument
there, a member array included, SHALL be a compile error that is not "not supported yet".

#### Scenario: The arrays accepted
- **WHEN** `DIM w(1 TO 64) AS _BYTE, f(1 TO 16) AS SINGLE` and then `_WAVE 0, w()`, `_WAVE 0, w(1), 32`,
  `_SNDRAWBATCH f()` and `_SNDRAWBATCH f(), 1, r, 8` are compiled
- **THEN** each compiles to the call the old compiler writes for it

#### Scenario: Another type or no array
- **WHEN** a program has `_WAVE 0, i()` for an `INTEGER` array, `_WAVE 0, u()` for an `_UNSIGNED _BYTE` array,
  `_WAVE 0, b` for a `_BYTE` variable, `_SNDRAWBATCH d()` for a `DOUBLE` array, `_SNDRAWBATCH s` for a `SINGLE`
  variable or `_WAVE 0, x.m()` for a `_BYTE` member array
- **THEN** the compiler reports an error that is not "not supported yet"

### Requirement: Opening a sound
`_SNDOPEN(name$)` SHALL give a handle above 0 for a file libqb can decode and 0 for any other name, the empty one
included; `_SNDOPEN(data$, "memory")` SHALL decode the string itself, as the old compiler's program does.
`_SNDLEN` SHALL give the sound's length in seconds.

#### Scenario: No file
- **WHEN** a program runs `PRINT _SNDOPEN("no such file.wav"); _SNDOPEN("")`
- **THEN** it prints ` 0  0 `, as with the old compiler

#### Scenario: A WAV from memory
- **WHEN** a program builds in a string a WAV of 4000 frames, mono, 16 bits, 8000 Hz, and runs
  `h = _SNDOPEN(w$, "memory"): PRINT h; _SNDLEN(h)`
- **THEN** it prints ` 1  .5 `, as with the old compiler

#### Scenario: A WAV file
- **WHEN** a program writes the same WAV to a file and runs `h = _SNDOPEN("t.wav"): PRINT h > 0; _SNDLEN(h)`
- **THEN** it prints `-1  .5 `, as with the old compiler

#### Scenario: A MIDI from memory
- **WHEN** a program builds in a string a MIDI file of one note of one second and runs
  `h = _SNDOPEN(m$, "memory"): PRINT _SNDLEN(h)` with the default sound bank
- **THEN** it prints ` 1 `, as with the old compiler

### Requirement: A handle that is not open
A statement given a handle that is not open, 0 and negative numbers included, SHALL do nothing, and a function
given one SHALL give 0, with no run-time error, as the old compiler's program does.

#### Scenario: Functions
- **WHEN** a program runs `PRINT _SNDLEN(99); _SNDPLAYING(-1); _SNDCOPY(0); _SNDGETPOS(5); _SNDPAUSED(7)`
- **THEN** it prints ` 0  0  0  0  0 ` and `ERR` is 0, as with the old compiler

#### Scenario: Statements
- **WHEN** a program under an error handler runs `_SNDPLAY 0`, `_SNDSTOP 99`, `_SNDVOL -1, 1`, `_SNDSETPOS 5, 1`
  and `_SNDCLOSE 42`
- **THEN** none raises an error, as with the old compiler

### Requirement: Playing and its state
`_SNDPLAY`, `_SNDLOOP`, `_SNDPAUSE` and `_SNDSTOP` SHALL change what `_SNDPLAYING`, `_SNDPAUSED` and `_SNDGETPOS`
give as the old compiler's program does, at any volume: a sound just opened or stopped is not playing and is
paused; `_SNDSETPOS` moves the position of a sound that is not playing.

#### Scenario: Play, pause, stop and loop
- **WHEN** a program opens a sound of 10 seconds, runs `_SNDVOL h, 0` and prints `_SNDPLAYING(h); _SNDPAUSED(h)`
  after the open, after `_SNDPLAY h`, after `_SNDPAUSE h`, after `_SNDSTOP h` and after `_SNDLOOP h`
- **THEN** it prints `0 -1`, `-1 0`, `0 -1`, `0 -1` and `-1 0`, as with the old compiler

#### Scenario: Position
- **WHEN** a program opens a sound of half a second and prints `_SNDGETPOS(h)` before and after `_SNDSETPOS h, .25`,
  and after `_SNDSTOP h`
- **THEN** it prints ` 0 `, ` .25 ` and ` 0 `, as with the old compiler

### Requirement: Copies of a sound
`_SNDCOPY(h)` SHALL give a new handle to a sound of the same length, or 0 when `h` is not open; `_SNDPLAYCOPY`
SHALL play such a copy and close it when it ends, as the old compiler's program does.

#### Scenario: A copy
- **WHEN** a program opens a sound of half a second as handle 1 and runs `c = _SNDCOPY(1): PRINT c; _SNDLEN(c)`
- **THEN** it prints ` 2  .5 `, as with the old compiler

### Requirement: New sounds and their memory
`_SNDNEW(frames[, channels][, bits][, rate])` SHALL give a handle to a silent sound, or 0 for no frames, a channel
count other than 1 or 2, or a bit depth other than 8, 16 or 32; `_MEMSOUND` of it SHALL give a block of its
samples, as the old compiler's program does.

#### Scenario: A new sound
- **WHEN** a program runs `n = _SNDNEW(800, 1, 16, 8000): m = _MEMSOUND(n): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE;
  _SNDLEN(n)`
- **THEN** it prints ` 1600  130  2  .1000000014901161 `, as with the old compiler

#### Scenario: Arguments refused
- **WHEN** a program prints `_SNDNEW(0, 2, 32)`, `_SNDNEW(1024, 0, 32)`, `_SNDNEW(1024, 1, 0)` and
  `_SNDNEW(-1024, 1, 16)`, each closed after
- **THEN** each is 0, and `_MEMSOUND` of it a block of size 0, as with the old compiler

### Requirement: Raw sound
`_SNDOPENRAW` SHALL give a handle to a stream of samples; `_SNDRAW` and `_SNDRAWBATCH` SHALL queue samples on it,
or on the default stream when no handle is given, and `_SNDRAWLEN` SHALL give the seconds still queued, as the old
compiler's program does.

#### Scenario: Samples queued
- **WHEN** a program opens a raw stream, sets its volume to 0, queues one second of samples of 0 and reads
  `_SNDRAWLEN` right after
- **THEN** the number is over 0 and at most 1, as with the old compiler

### Requirement: MIDI sound banks
`_MIDISOUNDBANK` SHALL set the sound bank later MIDI sounds are opened with: a file, a string with `"memory"` and
the bank's format, or the default bank for an empty name; a file that is not there SHALL leave the bank as it was,
with no run-time error, as the old compiler's program does.

#### Scenario: The default bank and a missing file
- **WHEN** a program runs `_MIDISOUNDBANK "no such bank.sf2"`, then `_MIDISOUNDBANK ""`, and opens a MIDI of one
  second from memory after each
- **THEN** each open gives a handle and a length of 1, and no error is raised, as with the old compiler

### Requirement: $MIDISOUNDFONT
The metacommand `$MIDISOUNDFONT:` with any value, in any case, SHALL be a compile error that names
`_MIDISOUNDBANK`, as in the old compiler; in a comment it SHALL be a comment.

#### Scenario: The deprecated metacommand
- **WHEN** a program has `$MIDISOUNDFONT:DEFAULT` or `$midisoundfont: "a.sf2"`
- **THEN** the compiler reports an error that is not "not supported yet"

#### Scenario: In a comment
- **WHEN** a program has `'$MIDISOUNDFONT:DEFAULT` or `REM $MIDISOUNDFONT:DEFAULT`
- **THEN** there is no diagnostic for that line
