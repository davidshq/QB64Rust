# _SNDRAWBATCH

`_SNDRAWBATCH samples!(), channels, handle, frames` queues a SINGLE array of samples on a raw stream at once.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDRAWBATCH sampleFrameArray!([index&])[, channels&][, pipeHandle&][, frameCount&]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

The array is a SINGLE array, whole or from one of its elements; `channels` is 1 or 2.
<!-- src: language/sound "The arrays of _WAVE and _SNDRAWBATCH"; tests\corpus\slice\s129_sound_files_raw_midi; tests\reference\examples\_sndrawbatch_1 -->

## Errors

- Any other argument for the array, a member array included, is a compile error.
  <!-- src: language/sound "The arrays of _WAVE and _SNDRAWBATCH" -->

## Differences

An index that calls a FUNCTION (`b(f(1))`) is computed once; QB64pe computes it twice.
<!-- src: language/sound "The arrays of _WAVE and _SNDRAWBATCH"; D-061 -->

A program that plays raw samples and ends soon after could crash in QB64pe; qb64rust takes a lock.
<!-- src: language/sound "Queued samples and the audio thread"; D-062 -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndrawbatch_1 -->
```qb64
r = _SNDOPENRAW
_SNDVOL r, 0
DIM b(1 TO 1024) AS SINGLE
_SNDRAWBATCH b(), 1, r
d# = _SNDRAWLEN(r)
PRINT "1024 samples queued:"; d# > 0
END
```

Output:

```text
1024 samples queued:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`_SNDRAW`](_sndraw.md), [`_SNDRAWLEN`](_sndrawlen.md).
