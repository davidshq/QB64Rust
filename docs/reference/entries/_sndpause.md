# _SNDPAUSE

`_SNDPAUSE handle` stops a sound where it is; `_SNDPLAY` then goes on from there.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDPAUSE handle&
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

After `_SNDPAUSE`, `_SNDPLAYING` is 0 and `_SNDPAUSED` -1.
<!-- src: language/sound "Playing and its state"; tests\corpus\slice\s127_sound_handles; tests\reference\examples\_sndpause_1 -->

A handle that is not open, 0 and negative numbers included, does nothing, with no error.
<!-- src: language/sound "A handle that is not open"; tests\corpus\slice\s127_sound_handles -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_SNDPAUSE`: "`_SNDPAUSE` takes 1 argument".
  <!-- src: language/sound "Forms of the sound handle statements"; verification\v71_x05_sndpause_bare -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndpause_1 -->
```qb64
k = _SNDOPEN(Wav$(80000, 8000), "memory")
_SNDVOL k, 0
_SNDPLAY k
_SNDPAUSE k
PRINT "Playing, paused:"; _SNDPLAYING(k); _SNDPAUSED(k)
_SNDPLAY k
PRINT "Playing again:"; _SNDPLAYING(k)
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
Playing, paused: 0 -1 
Playing again:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`_SNDPLAY`](_sndplay.md), [`_SNDPAUSED`](_sndplaying.md), [`_SNDSTOP`](_sndstop.md).
