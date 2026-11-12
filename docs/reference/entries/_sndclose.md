# _SNDCLOSE

`_SNDCLOSE handle` closes a sound.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDCLOSE handle&
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

After the close, the functions give 0 for the handle at once.
<!-- src: tests\corpus\slice\s127_sound_handles; tests\reference\examples\_sndclose_1 -->

libqb frees the handle later, in its sound loop, so the number of the next handle opened is not known in advance.
<!-- src: tests\corpus\slice\s127_sound_handles -->

A handle that is not open, 0 and negative numbers included, does nothing, with no error.
<!-- src: language/sound "A handle that is not open"; tests\corpus\slice\s127_sound_handles -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_SNDCLOSE`: "`_SNDCLOSE` takes 1 argument".
  <!-- src: language/sound "Forms of the sound handle statements"; verification\v39_x32_sndclose_none -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndclose_1 -->
```qb64
h = _SNDOPEN(Wav$(4000, 8000), "memory")
_SNDCLOSE h
PRINT "After the close:"; _SNDLEN(h)
_SNDCLOSE h
PRINT "Closed twice, ERR is"; ERR
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
After the close: 0 
Closed twice, ERR is 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDOPEN`, `_SNDSTOP`.
