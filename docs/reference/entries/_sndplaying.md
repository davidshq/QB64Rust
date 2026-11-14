# _SNDPLAYING

`_SNDPLAYING(handle)` tells whether a sound is playing, and `_SNDPAUSED(handle)` whether it is paused.

## Syntax

```text
playing = _SNDPLAYING(handle)
paused = _SNDPAUSED(handle)
```

## Result

-1 or 0, a LONG.

## Behavior

A sound just opened or stopped is not playing and is paused: 0 and -1. After `_SNDPLAY` or `_SNDLOOP` they are -1 and
0, at any volume.
<!-- src: language/sound "Playing and its state"; tests\corpus\slice\s127_sound_handles; tests\reference\examples\_sndplaying_1 -->

A handle that is not open, 0 and negative numbers included, gives 0, with no error.
<!-- src: language/sound "A handle that is not open"; tests\corpus\slice\s127_sound_handles -->

## Errors

- `_SNDPLAYING` with no handle is a compile error.
  <!-- src: language/sound "Forms of the sound handle functions" -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndplaying_1 -->
```qb64
k = _SNDOPEN(Wav$(80000, 8000), "memory")
_SNDVOL k, 0
PRINT "Just opened:"; _SNDPLAYING(k); _SNDPAUSED(k)
_SNDPLAY k
PRINT "Played:     "; _SNDPLAYING(k); _SNDPAUSED(k)
_SNDSTOP k
PRINT "Not open:   "; _SNDPLAYING(99); _SNDPAUSED(99)
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
Just opened: 0 -1 
Played:     -1  0 
Not open:    0  0 

Press any key to continue
```
<!-- end generated -->

## See also

[`_SNDPLAY`](_sndplay.md), [`_SNDPAUSE`](_sndpause.md), [`_SNDSTOP`](_sndstop.md).
