# _SNDCOPY

`_SNDCOPY(handle)` makes a copy of a sound, which plays on its own, and gives the copy's handle.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDCOPY(handle&)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG handle, or 0.

## Behavior

A copy of a sound of half a second opened as handle 1 is handle 2, of length .5.
<!-- src: language/sound "Copies of a sound"; tests\corpus\slice\s127_sound_handles; tests\reference\examples\_sndcopy_1 -->

A copy is not playing and is paused, even of a sound that is playing.
<!-- src: tests\corpus\slice\s127_sound_handles -->

A handle that is not open, 0 and negative numbers included, gives 0, with no error.
<!-- src: language/sound "A handle that is not open"; tests\corpus\slice\s127_sound_handles -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `h = _SNDCOPY`: "`_SNDCOPY` takes 1 argument".
  <!-- src: language/sound "Forms of the sound handle functions"; verification\v39_x33_sndcopy_bare -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndcopy_1 -->
```qb64
h = _SNDOPEN(Wav$(4000, 8000), "memory")
c = _SNDCOPY(h)
PRINT "Copy:"; c; _SNDLEN(c)
PRINT "Of no sound:"; _SNDCOPY(0)
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
Copy: 2  .5 
Of no sound: 0 

Press any key to continue
```
<!-- end generated -->

## See also

[`_SNDPLAYCOPY`](_sndplaycopy.md), [`_SNDOPEN`](_sndopen.md).
