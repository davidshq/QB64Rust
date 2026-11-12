# _SNDVOL

`_SNDVOL handle, volume` sets the volume of a sound, from 0 (silent) to 1.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDVOL handle&, volume!
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

Volume 0 changes nothing but what is heard: a sound at volume 0 plays, and its state changes, as at any other.
<!-- src: language/sound "Playing and its state"; tests\corpus\slice\s127_sound_handles; tests\reference\examples\_sndvol_1 -->

A handle that is not open, 0 and negative numbers included, does nothing, with no error.
<!-- src: language/sound "A handle that is not open"; tests\corpus\slice\s127_sound_handles -->

## Errors

- `_SNDVOL h`, with no volume, is a compile error.
  <!-- src: language/sound "Forms of the sound handle statements" -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndvol_1 -->
```qb64
k = _SNDOPEN(Wav$(80000, 8000), "memory")
_SNDVOL k, 0
_SNDPLAY k
PRINT "Playing at volume 0:"; _SNDPLAYING(k)
_SNDSTOP k
_SNDVOL -1, 1
PRINT "A handle that is not open, ERR is"; ERR
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
Playing at volume 0:-1 
A handle that is not open, ERR is 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDBAL`, `_SNDPLAY`.
