# _SNDRAWLEN

`_SNDRAWLEN(handle)` gives the seconds of samples still queued on a raw stream.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDRAWLEN [pipeHandle&]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A DOUBLE.

## Behavior

A stream just opened gives 0. After a second of samples is queued it gives over 0 and at most 1, since playing has
already begun.
<!-- src: language/sound "Raw sound"; tests\corpus\slice\s129_sound_files_raw_midi; tests\reference\examples\_sndrawlen_1 -->

Without a handle it reads the default stream.
<!-- src: language/sound "Forms of the sound handle functions"; tests\corpus\slice\s129_sound_files_raw_midi -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `d = _SNDRAWLEN()`: "`_SNDRAWLEN` with empty parentheses".
  - `d = _SNDRAWLEN(1, 2)`: "`_SNDRAWLEN` takes 0 or 1 arguments".
  <!-- src: language/sound "Forms of the sound handle functions"; verification\v39_x43_sndrawlen_empty; verification\v39_x63_sndrawlen_two -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndrawlen_1 -->
```qb64
r = _SNDOPENRAW
PRINT "Nothing queued:"; _SNDRAWLEN(r)
_SNDVOL r, 0
FOR i = 1 TO _SNDRATE \ 2: _SNDRAW 0, 0, r: NEXT
PRINT "Half a second queued, over 0:"; _SNDRAWLEN(r) > 0
END
```

Output:

```text
Nothing queued: 0 
Half a second queued, over 0:-1 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDRAW`, `_SNDOPENRAW`.
