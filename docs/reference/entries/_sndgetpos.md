# _SNDGETPOS

`_SNDGETPOS(handle)` gives the position of a sound in seconds, and `_SNDSETPOS handle, seconds` sets it.

## Syntax

```text
seconds = _SNDGETPOS(handle)
_SNDSETPOS handle, seconds
```

## Result

A DOUBLE.

## Behavior

A sound just opened is at 0. After `_SNDSETPOS h, .25` it is at .25, and after `_SNDSTOP` at 0 again.
<!-- src: language/sound "Playing and its state"; tests\corpus\slice\s127_sound_handles; tests\reference\examples\_sndgetpos_1 -->

A negative position is ignored. A position past the end stops the sound and leaves its position as it was.
<!-- src: tests\corpus\slice\s127_sound_handles -->

A handle that is not open, 0 and negative numbers included, gives 0, with no error.
<!-- src: language/sound "A handle that is not open"; tests\corpus\slice\s127_sound_handles -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_SNDSETPOS h`: "`_SNDSETPOS` takes 2 arguments".
  - `d = _SNDGETPOS()`: "`_SNDGETPOS` with empty parentheses".
  <!-- src: language/sound "Forms of the sound handle functions"; verification\v39_x31_sndsetpos_one; verification\v39_x41_sndgetpos_empty -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndgetpos_1 -->
```qb64
h = _SNDOPEN(Wav$(4000, 8000), "memory")
PRINT "At the start:"; _SNDGETPOS(h)
_SNDSETPOS h, .25
PRINT "After _SNDSETPOS h, .25:"; _SNDGETPOS(h)
_SNDSTOP h
PRINT "After _SNDSTOP:"; _SNDGETPOS(h)
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
At the start: 0 
After _SNDSETPOS h, .25: .25 
After _SNDSTOP: 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDLEN`, `_SNDSTOP`, `_SNDLIMIT`.
