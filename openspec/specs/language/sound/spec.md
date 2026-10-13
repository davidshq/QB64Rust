# language/sound Specification

## Purpose
The sound statements of a program, `SOUND`, `PLAY` and `BEEP`, and the function `PLAY`: which forms compile, what
a statement queues, when it holds the program and which arguments are a run-time error, as the old compiler's
program behaves.

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
