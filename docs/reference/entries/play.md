# PLAY

`PLAY commands$` plays music written in a string of notes and commands. The function `PLAY(voice)` gives the seconds
of sound still to play in a voice.

## Syntax

<!-- begin generated: syntax -->
```text
PLAY voiceCommands1$[, voiceCommands2$][, voiceCommands3$][, voiceCommands4$]
PLAY [voice&]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

```text
seconds = PLAY[(voice)]
```

## Arguments

- `commands$`: notes `A` to `G` with `#`, `+` or `-`, `N` and a note number up to 84, `P` a pause, `L` a length (1 to
  64), `T` a tempo, `O` an octave, `<` and `>`, `V` a volume (0 to 100), `MF`, `MB`, `MN`, `ML`, `MS`, and a dot
  after a note for half as long again.
- Up to three more strings play in voices 1 to 3 at the same time.

## Result

The function: a DOUBLE, in seconds. Voice 0 with no argument and for a voice that does not exist.

## Behavior

A string adds the lengths of its notes and pauses, by its tempo, lengths and dots: `PLAY "T120 L4 C D E"` adds 1.5
seconds to `PLAY(0)`. The tempo, length and octave hold for the next `PLAY`.
<!-- src: language/sound "What a statement queues"; tests\corpus\slice\s74_sound_queue; tests\reference\examples\play_1 -->

In the foreground, `PLAY` holds the program while the music plays. `MB` puts the voice in the background, so that
later statements return at once, and `MF` back in the foreground. A `PLAY` with more than one string never holds the
program.
<!-- src: language/sound "Sound in the foreground holds the program"; tests\corpus\slice\s75_sound_wait -->

A program that has played nothing gives `PLAY(0)` = 0.
<!-- src: language/sound "The PLAY function"; tests\reference\examples\play_1 -->

`ON PLAY(n) GOSUB` is `ON number GOSUB` with the function's value as the number. There is no music event.
<!-- src: language/sound "The PLAY function"; Q-012 -->

A fixed-length string, a trailing comma and `CALL PLAY(t$)` are accepted.
<!-- src: language/sound "Forms of the PLAY statement"; tests\corpus\slice\s74_sound_queue -->

## Errors

- A command that does not exist, a length of 0 or above 64, a note above 84 or a volume above 100 raises error 5:
  `PLAY "H"` does.
  <!-- src: language/sound "Run-time errors of a PLAY string"; tests\corpus\slice\s76_sound_errors; tests\reference\examples\play_1 -->
- `PLAY` with no string, with a number, or with five strings, and `PLAY ON`, `PLAY OFF` and `PLAY STOP`, are compile
  errors.
  <!-- src: language/sound "Forms of the PLAY statement"; language/sound "The PLAY function" -->

## Differences

A tempo or an octave outside QuickBASIC's range (`T31`, `T256`, `O7`) raises no error, as in QB64pe; QuickBASIC raises
error 5.
<!-- src: language/sound "Run-time errors of a PLAY string"; Q-011 -->

A program that plays sound and ends soon after could crash in QB64pe; qb64rust takes a lock.
<!-- src: language/sound "Queued samples and the audio thread"; D-062 -->

## Availability

<!-- begin generated: availability -->
Supported. All 28 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:play_1 -->
```qb64
ON ERROR GOTO trap
PRINT "Nothing queued:"; PLAY(0)
PLAY "MB V0"
b# = PLAY(0): PLAY "T120 L4 C D E": a# = PLAY(0)
PRINT "Three quarter notes at T120:"; CINT((a# - b#) * 10); "tenths of a second"
PLAY "H"
END
trap:
PRINT "error"; ERR
RESUME NEXT
```

Output:

```text
Nothing queued: 0 
Three quarter notes at T120: 15 tenths of a second
error 5 

Press any key to continue
```
<!-- end generated -->

## See also

[`SOUND`](sound.md), [`BEEP`](beep.md), [`ON`](on.md).
