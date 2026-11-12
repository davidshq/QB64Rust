# SOUND

`SOUND frequency, duration` plays a tone: `frequency` in hertz, `duration` in clock ticks of 1/18.2 second. QB64pe
adds a volume, a pan position, a waveform and a voice.

## Syntax

<!-- begin generated: syntax -->
```text
SOUND [frequency!, duration![, volume!][, panPosition!][, waveform&][, waveformParameters!][, voice&]] [{WAIT|RESUME}]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `frequency`: 0 (a rest) or 20 to 32767.
- `duration`: 0 to 65535 clock ticks; 18.2 is one second.
- `volume`: 0 to 1. `pan`: -1 (left) to 1 (right). `waveform`: 1 to 10, with 10 the one `_WAVE` defines. `parameter`:
  0 to 1 when `waveform` is 9 in the same statement. `voice`: 0 to 3.
- `WAIT` holds back what later statements queue; `RESUME` lets it all play.

## Behavior

In the foreground, the default, `SOUND` holds the program while its tone plays, except for a tone shorter than about a
quarter of a second. After `PLAY "MB"` it returns at once, and the tone plays in the background.
<!-- src: language/sound "Sound in the foreground holds the program"; tests\corpus\slice\s75_sound_wait -->

In the background each `SOUND` adds its duration to `PLAY(0)`: `SOUND 440, 18.2` adds one second, `SOUND 440, 0`
nothing. The background is set per voice.
<!-- src: language/sound "What a statement queues"; tests\corpus\slice\s74_sound_queue; tests\reference\examples\sound_1 -->

After `SOUND WAIT`, statements queue sound without playing it, and none holds the program, until `SOUND RESUME`. A
`PLAY` with more than one string also ends the wait.
<!-- src: language/sound "Sound in the foreground holds the program"; tests\corpus\slice\s75_sound_wait -->

Volume 0 queues and holds as long as any other volume, but nothing is heard.
<!-- src: language/sound "What a statement queues" -->

`CALL SOUND(440, 1)` is `SOUND 440, 1`, and a trailing comma is ignored.
<!-- src: language/sound "Forms of SOUND"; tests\corpus\slice\s74_sound_queue -->

## Errors

- An argument outside its range raises error 5, once per statement: `SOUND 19, 1`, `SOUND 440, -1`, `SOUND` with no
  argument, a voice of 4.
  <!-- src: language/sound "Run-time errors of sound"; tests\corpus\slice\s76_sound_errors; tests\reference\examples\sound_1 -->
- libqb checks the volume, pan and waveform only once it has the audio device, so on a machine with none they raise no
  error.
  <!-- src: read: libqb\internal\c\parts\audio\audio.cpp:2675-2770; tests\corpus\slice\s76_sound_errors -->
- `SOUND 440`, `SOUND , 1`, `SOUND(440, 1)`, `SOUND 440, 1, WAIT` and a string argument are compile errors.
  <!-- src: language/sound "Forms of SOUND" -->

## Differences

A program that plays sound and ends soon after could crash in QB64pe, about 1 run in 40, since its audio thread
changed a buffer the program was writing; qb64rust takes a lock.
<!-- src: language/sound "Queued samples and the audio thread"; D-062 -->

## Availability

<!-- begin generated: availability -->
Supported. All 39 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:sound_1 -->
```qb64
ON ERROR GOTO trap
PLAY "MB V0"
b# = PLAY(0): SOUND 440, 18.2, 0: a# = PLAY(0)
PRINT "SOUND 440, 18.2 queued"; CINT((a# - b#) * 10); "tenths of a second"
SOUND 19, 1
SOUND 440, -1
END
trap:
PRINT "error"; ERR
RESUME NEXT
```

Output:

```text
SOUND 440, 18.2 queued 10 tenths of a second
error 5 
error 5 

Press any key to continue
```
<!-- end generated -->

## See also

[`PLAY`](play.md), [`BEEP`](beep.md), [`_WAVE`](_wave.md), [`_SNDRAW`](_sndraw.md).
