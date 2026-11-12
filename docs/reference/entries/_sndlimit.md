# _SNDLIMIT

`_SNDLIMIT handle, seconds` sets a time at which a playing sound stops.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDLIMIT handle&, numberOfSeconds#
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

The limit is a time on libqb's sound clock, not a position in the sound. It changes neither the sound's length nor its
position.
<!-- src: tests\corpus\slice\s127_sound_handles; tests\reference\examples\_sndlimit_1 -->

A limit past the sound's length is ignored. One set stays until `_SNDSTOP`, `_SNDSETPOS` or a `_SNDPLAY` of the sound
while it plays.
<!-- src: tests\corpus\slice\s127_sound_handles -->

A handle that is not open, 0 and negative numbers included, does nothing, with no error.
<!-- src: language/sound "A handle that is not open"; tests\corpus\slice\s127_sound_handles -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_SNDLIMIT h`: "`_SNDLIMIT` takes 2 arguments".
  <!-- src: language/sound "Forms of the sound handle statements"; verification\v39_x30_sndlimit_one -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndlimit_1 -->
```qb64
k = _SNDOPEN(Wav$(80000, 8000), "memory")
_SNDVOL k, 0
_SNDLIMIT k, 1
PRINT "Length, position, playing:"; _SNDLEN(k); _SNDGETPOS(k); _SNDPLAYING(k)
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
Length, position, playing: 10  0  0 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDPLAY`, `_SNDGETPOS`.
