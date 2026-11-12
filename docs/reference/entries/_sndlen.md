# _SNDLEN

`_SNDLEN(handle)` gives the length of a sound in seconds.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDLEN(handle&)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A DOUBLE.

## Behavior

A WAV of 4000 frames at 8000 Hz is .5 seconds long.
<!-- src: language/sound "Opening a sound"; tests\reference\examples\_sndlen_1 -->

A handle that is not open, 0 and negative numbers included, gives 0, with no error.
<!-- src: language/sound "A handle that is not open"; tests\corpus\slice\s127_sound_handles -->

## Errors

- `_SNDLEN(f$)` is a compile error.
  <!-- src: language/sound "Forms of the sound handle functions" -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndlen_1 -->
```qb64
h = _SNDOPEN(Wav$(4000, 8000), "memory")
PRINT "Half a second:"; _SNDLEN(h)
PRINT "A handle that is not open:"; _SNDLEN(99)
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
Half a second: .5 
A handle that is not open: 0 

Press any key to continue
```
<!-- end generated -->

## See also

[`_SNDOPEN`](_sndopen.md), [`_SNDGETPOS`](_sndgetpos.md).
