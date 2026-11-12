# _SNDPLAYCOPY

`_SNDPLAYCOPY handle, volume` plays a copy of a sound and closes the copy when it ends, so that a sound can play over
itself.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDPLAYCOPY handle&[, volume!][, x!][, y!][, z!]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

The sound itself is not played: `_SNDPLAYING` of it stays 0.
<!-- src: language/sound "Copies of a sound"; tests\corpus\slice\s129_sound_files_raw_midi; tests\reference\examples\_sndplaycopy_1 -->

The copy plays at the volume given, and at the place given by the last three arguments.
<!-- src: language/sound "Forms of the sound handle statements"; tests\corpus\slice\s129_sound_files_raw_midi -->

A handle that is not open, 0 and negative numbers included, does nothing, with no error.
<!-- src: language/sound "A handle that is not open"; tests\corpus\slice\s127_sound_handles -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_SNDPLAYCOPY h, 1, 2, 3, 4, 5`: "this `_SNDPLAYCOPY` statement matches none of its forms".
  <!-- src: language/sound "Forms of the sound handle statements"; verification\v39_x40_sndplaycopy_six -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndplaycopy_1 -->
```qb64
h = _SNDOPEN(Wav$(4000, 8000), "memory")
_SNDPLAYCOPY h, 0
PRINT "The sound itself is playing:"; _SNDPLAYING(h); "ERR is"; ERR
END

' A WAV of silence: mono, 16 bits.
FUNCTION Wav$ (frames AS LONG, rate AS LONG)
    DIM d AS LONG
    d = frames * 2
    Wav$ = "RIFF" + MKL$(36 + d) + "WAVE" + "fmt " + MKL$(16) + MKI$(1) + MKI$(1) + MKL$(rate) + MKL$(rate * 2) + MKI$(2) + MKI$(16) + "data" + MKL$(d) + STRING$(d, 0)
END FUNCTION
```

Output:

```text
The sound itself is playing: 0 ERR is 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDCOPY`, `_SNDPLAY`, `_SNDPLAYFILE`.
