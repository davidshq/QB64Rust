# _MEMSOUND

`_MEMSOUND(handle[, channel])` gives the block of a sound's samples.

## Syntax

<!-- begin generated: syntax -->
```text
_MEMSOUND(soundHandle&)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A `_MEM`.

## Behavior

The block of `_SNDNEW(800, 1, 16, 8000)` is 1600 bytes, of `TYPE` 130 with 2 bytes a sample; `SOUND` is the sound's
handle.
<!-- src: language/mem-blocks "New memory, images and sounds"; language/sound "New sounds and their memory"; tests\corpus\slice\s128_sound_new_memory; tests\reference\examples\_memsound_1 -->

A sound opened from a file is decoded at the device's rate, so its block's size depends on the machine. A stream, or a
sound not decoded, has a block of size 0.
<!-- src: tests\corpus\slice\s128_sound_new_memory -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `m = _MEMSOUND`: "`_MEMSOUND` takes 1 or 2 arguments".
  - `m = _MEMSOUND(1, 2, 3)`: "`_MEMSOUND` takes 1 or 2 arguments".
  - `m = _MEMSOUND`: "`_MEMSOUND` takes 1 or 2 arguments".
  <!-- src: language/mem-blocks "New memory, images and sounds"; verification\v32_x87_memsound_bare; verification\v32_x88_memsound_three; verification\v39_x62_memsound_none -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_memsound_1 -->
```qb64
n = _SNDNEW(800, 1, 16, 8000)
DIM m AS _MEM
m = _MEMSOUND(n)
PRINT "SIZE, TYPE, ELEMENTSIZE:"; m.SIZE; m.TYPE; m.ELEMENTSIZE
PRINT "SOUND is the handle:"; m.SOUND = n
END
```

Output:

```text
SIZE, TYPE, ELEMENTSIZE: 1600  130  2 
SOUND is the handle:-1 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDNEW`, `_MEM`, `_MEMGET`.
