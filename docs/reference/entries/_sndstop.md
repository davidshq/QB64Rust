# _SNDSTOP

`_SNDSTOP handle` stops a sound and moves it back to its start.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDSTOP handle&
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

After `_SNDSTOP`, `_SNDPLAYING` is 0, `_SNDPAUSED` -1 and `_SNDGETPOS` 0.
<!-- src: language/sound "Playing and its state"; tests\corpus\slice\s127_sound_handles; tests\reference\examples\_sndstop_1 -->

It also clears a limit set by `_SNDLIMIT`.
<!-- src: tests\corpus\slice\s127_sound_handles -->

A handle that is not open, 0 and negative numbers included, does nothing, with no error.
<!-- src: language/sound "A handle that is not open"; tests\corpus\slice\s127_sound_handles -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/sound "Forms of the sound handle statements" -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndstop_1 -->
```qb64
k = _SNDOPEN(Wav$(80000, 8000), "memory")
_SNDVOL k, 0
_SNDPLAY k
_SNDSTOP k
PRINT "Playing, paused, position:"; _SNDPLAYING(k); _SNDPAUSED(k); _SNDGETPOS(k)
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
Playing, paused, position: 0 -1  0 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDPAUSE`, `_SNDPLAY`, `_SNDCLOSE`.
