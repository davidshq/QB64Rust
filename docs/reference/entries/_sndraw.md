# _SNDRAW

`_SNDRAW left, right, handle` queues one frame of samples, each from -1 to 1, on a raw stream.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDRAW leftSample[, rightSample][, pipeHandle&]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

Without `right` the frame is mono; without `handle` it goes to the default stream. `_SNDRATE` frames make one second.
<!-- src: language/sound "Raw sound"; tests\corpus\slice\s129_sound_files_raw_midi; tests\reference\examples\_sndraw_1 -->

Samples of 0 are silence.
<!-- src: tests\corpus\slice\s129_sound_files_raw_midi -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/sound "Forms of the sound handle statements" -->

## Differences

A program that plays raw samples and ends soon after could crash in QB64pe; qb64rust takes a lock.
<!-- src: language/sound "Queued samples and the audio thread"; D-062 -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndraw_1 -->
```qb64
r = _SNDOPENRAW
_SNDVOL r, 0
FOR i = 1 TO _SNDRATE: _SNDRAW 0, 0, r: NEXT
d# = _SNDRAWLEN(r)
PRINT "One second of samples queued, over 0 and at most 1:"; d# > 0; d# <= 1
END
```

Output:

```text
One second of samples queued, over 0 and at most 1:-1 -1 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDRAWBATCH`, `_SNDRAWLEN`, `_SNDRATE`, `_SNDOPENRAW`.
