# _SNDLOOP

`_SNDLOOP handle` plays a sound over and over, until `_SNDSTOP` or `_SNDPAUSE`.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDLOOP handle&
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

After `_SNDLOOP`, `_SNDPLAYING` is -1 and `_SNDPAUSED` 0.
<!-- src: language/sound "Playing and its state"; tests\corpus\slice\s127_sound_handles; tests\reference\examples\_sndloop_1 -->

A handle that is not open, 0 and negative numbers included, does nothing, with no error.
<!-- src: language/sound "A handle that is not open"; tests\corpus\slice\s127_sound_handles -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_SNDLOOP`: "`_SNDLOOP` takes 1 argument".
  - `_SNDLOOP h, 1`: "`_SNDLOOP` takes 1 argument".
  <!-- src: language/sound "Forms of the sound handle statements"; verification\v71_x03_sndloop_bare; verification\v71_x04_sndloop_two -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndloop_1 -->
```qb64
k = _SNDOPEN(Wav$(80000, 8000), "memory")
_SNDVOL k, 0
_SNDLOOP k
PRINT "Playing, paused:"; _SNDPLAYING(k); _SNDPAUSED(k)
_SNDSTOP k
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
Playing, paused:-1  0 

Press any key to continue
```
<!-- end generated -->

## See also

[`_SNDPLAY`](_sndplay.md), [`_SNDSTOP`](_sndstop.md).
