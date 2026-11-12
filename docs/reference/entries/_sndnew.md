# _SNDNEW

`_SNDNEW(frames, channels, bits, rate)` makes a new silent sound, whose samples `_MEMSOUND` then gives as a memory
block.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDNEW(frames&[, channels&][, bits&][, sampleRate&])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG handle, or 0.

## Behavior

`_SNDNEW(800, 1, 16, 8000)` is a sound of 800 frames, mono, 16 bits, at 8000 Hz: its block is 1600 bytes, of type 130,
with 2 bytes a sample, and its length is .1 second.
<!-- src: language/sound "New sounds and their memory"; tests\corpus\slice\s128_sound_new_memory; tests\reference\examples\_sndnew_1 -->

Left out, the channels are 1 and the bits 32.
<!-- src: tests\corpus\slice\s128_sound_new_memory -->

A copy of it holds the samples written into its block.
<!-- src: tests\corpus\slice\s128_sound_new_memory -->

## Errors

- No frames, a channel count other than 1 or 2, or a bit depth other than 8, 16 or 32 gives 0 and no error.
  <!-- src: language/sound "New sounds and their memory"; tests\corpus\slice\s128_sound_new_memory; tests\reference\examples\_sndnew_1 -->
- A form QB64pe rejects is a compile error.
  <!-- src: language/sound "Forms of the sound handle functions" -->

## Availability

<!-- begin generated: availability -->
Supported. All 11 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndnew_1 -->
```qb64
n = _SNDNEW(800, 1, 16, 8000)
DIM m AS _MEM
m = _MEMSOUND(n)
PRINT "Bytes, type, bytes per sample:"; m.SIZE; m.TYPE; m.ELEMENTSIZE
PRINT "Length in milliseconds:"; CINT(_SNDLEN(n) * 1000)
PRINT "Three channels:"; _SNDNEW(1024, 3, 16)
END
```

Output:

```text
Bytes, type, bytes per sample: 1600  130  2 
Length in milliseconds: 100 
Three channels: 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_MEMSOUND`, `_SNDOPEN`.
