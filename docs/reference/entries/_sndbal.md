# _SNDBAL

`_SNDBAL handle, x, y, z, channel` places a sound in space: `x` from left (-1) to right (1), `y` and `z` up and away.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDBAL handle&[, x!][, y!][, z!]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

Each of `x`, `y`, `z` and `channel` may be left out. The balance changes neither the sound's length nor its position.
<!-- src: language/sound "Forms of the sound handle statements"; tests\corpus\slice\s127_sound_handles; tests\reference\examples\_sndbal_1 -->

A handle that is not open, 0 and negative numbers included, does nothing, with no error.
<!-- src: language/sound "A handle that is not open"; tests\corpus\slice\s127_sound_handles -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_SNDBAL h`: "this `_SNDBAL` statement matches none of its forms".
  - `_SNDBAL h, 1, 2, 3, 4, 5`: "this `_SNDBAL` statement matches none of its forms".
  <!-- src: language/sound "Forms of the sound handle statements"; verification\v39_x29_sndbal_handle_only; verification\v39_x66_sndbal_six -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndbal_1 -->
```qb64
k = _SNDOPEN(Wav$(80000, 8000), "memory")
_SNDVOL k, 0
_SNDBAL k, 1
_SNDBAL k, , 2
PRINT "Length and position:"; _SNDLEN(k); _SNDGETPOS(k)
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
Length and position: 10  0 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDVOL`.
