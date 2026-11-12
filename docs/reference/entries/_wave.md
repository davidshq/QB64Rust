# _WAVE

`_WAVE voice, samples%%()` defines the shape of waveform 10 of a voice from a `_BYTE` array, for `SOUND`.

## Syntax

<!-- begin generated: syntax -->
```text
_WAVE voice&, waveDefinition%%([index&])[, frameCount&]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

After `_WAVE 0, w()`, `SOUND 440, 1, 0, , 10` plays the wave in voice 0.
<!-- src: language/sound "The arrays of _WAVE and _SNDRAWBATCH"; tests\corpus\slice\s129_sound_files_raw_midi; tests\reference\examples\_wave_1 -->

The third argument takes only that many frames of the array.
<!-- src: tests\corpus\slice\s129_sound_files_raw_midi -->

## Errors

- A voice outside 0 to 3, or fewer than two frames, raises error 5.
  <!-- src: read: libqb\internal\c\parts\audio\audio.cpp:2777-2808; tests\corpus\slice\s129_sound_files_raw_midi; tests\reference\examples\_wave_1 -->
- An array of another type, `_UNSIGNED _BYTE` included, a variable or a member array is a compile error.
  <!-- src: language/sound "The arrays of _WAVE and _SNDRAWBATCH" -->

## Differences

An index that calls a FUNCTION (`w(f(1))`) is computed once; QB64pe computes it twice.
<!-- src: language/sound "The arrays of _WAVE and _SNDRAWBATCH"; D-061 -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_wave_1 -->
```qb64
ON ERROR GOTO trap
DIM w(1 TO 64) AS _BYTE, w1(1 TO 1) AS _BYTE
FOR i = 1 TO 64: w(i) = 127 * SIN(i / 64 * 6.2832): NEXT
_WAVE 0, w()
SOUND 440, 1, 0, , 10
PRINT "A wave of 64 samples, ERR is"; ERR
_WAVE 0, w1()
_WAVE 4, w()
END
trap:
PRINT "error"; ERR
RESUME NEXT
```

Output:

```text
A wave of 64 samples, ERR is 0 
error 5 
error 5 

Press any key to continue
```
<!-- end generated -->

## See also

[`SOUND`](sound.md).
