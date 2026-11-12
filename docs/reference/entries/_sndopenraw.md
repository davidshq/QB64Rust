# _SNDOPENRAW

`_SNDOPENRAW` opens a stream of samples that `_SNDRAW` and `_SNDRAWBATCH` write to, and gives its handle.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDOPENRAW
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG handle above 0.

## Behavior

A stream just opened has nothing queued: `_SNDRAWLEN` is 0.
<!-- src: language/sound "Raw sound"; tests\corpus\slice\s129_sound_files_raw_midi; tests\reference\examples\_sndopenraw_1 -->

Its `_SNDLEN`, `_SNDGETPOS`, `_SNDPLAYING`, `_SNDPAUSED` and `_SNDCOPY` are 0.
<!-- src: tests\corpus\slice\s127_sound_handles -->

Without a handle, `_SNDRAW` and the others use a default stream, which needs no open.
<!-- src: language/sound "Raw sound" -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `r = _SNDOPENRAW(1)`: "`_SNDOPENRAW` takes no arguments".
  <!-- src: language/sound "Forms of the sound handle functions"; verification\v39_x15_sndopenraw_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndopenraw_1 -->
```qb64
r = _SNDOPENRAW
PRINT "A handle:"; r > 0; "nothing queued:"; _SNDRAWLEN(r)
_SNDCLOSE r
END
```

Output:

```text
A handle:-1 nothing queued: 0 

Press any key to continue
```
<!-- end generated -->

## See also

[`_SNDRAW`](_sndraw.md), [`_SNDRAWLEN`](_sndrawlen.md), [`_SNDRAWDONE`](_sndrawdone.md), [`_SNDCLOSE`](_sndclose.md).
