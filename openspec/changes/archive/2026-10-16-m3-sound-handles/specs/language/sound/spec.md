# Spec Delta

## ADDED Requirements

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
