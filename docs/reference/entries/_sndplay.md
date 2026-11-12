# _SNDPLAY

`_SNDPLAY handle` plays a sound from its position, once.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDPLAY handle&
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

After `_SNDPLAY`, `_SNDPLAYING` is -1 and `_SNDPAUSED` 0. A paused sound goes on from where it was.
<!-- src: language/sound "Playing and its state"; tests\corpus\slice\s127_sound_handles; tests\reference\examples\_sndplay_1 -->

The sound plays through the machine's audio device; at volume 0 nothing is heard.
<!-- src: tests\corpus\slice\s127_sound_handles -->

A handle that is not open, 0 and negative numbers included, does nothing, with no error.
<!-- src: language/sound "A handle that is not open"; tests\corpus\slice\s127_sound_handles -->

## Errors

- `_SNDPLAY` with no handle or two, with a string, or as a function, is a compile error.
  <!-- src: language/sound "Forms of the sound handle statements" -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndplay_1 -->
```qb64
k = _SNDOPEN(Wav$(80000, 8000), "memory")
_SNDVOL k, 0
_SNDPLAY k
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

[`_SNDLOOP`](_sndloop.md), [`_SNDPAUSE`](_sndpause.md), [`_SNDSTOP`](_sndstop.md), [`_SNDPLAYING`](_sndplaying.md).
