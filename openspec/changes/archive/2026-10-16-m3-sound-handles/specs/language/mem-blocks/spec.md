# Spec Delta

## MODIFIED Requirements

### Requirement: New memory, images and sounds
`_MEMNEW(bytes)` SHALL give a block of new memory and `_MEMIMAGE[(handle)]` the pixels of an image (the
destination when no handle is given), as libqb gives them; `_MEMSOUND(handle[, channel])` SHALL give the block of a sound's samples (`language/sound`).
`_MEMEXISTS(block)` SHALL be -1 while the block is valid and 0 after it is freed.

#### Scenario: New memory freed
- **WHEN** `DIM n AS _MEM: n = _MEMNEW(16): PRINT _MEMEXISTS(n); n.SIZE; n.TYPE: _MEMFREE n: PRINT _MEMEXISTS(n)`
  runs
- **THEN** it prints `-1  16  16384 ` and then ` 0 `

#### Scenario: Pixels of an image
- **WHEN** a program makes a 4 by 2 image with `_NEWIMAGE(4, 2, 32)`, `PSET`s one pixel and reads it with
  `_MEMGET(_MEMIMAGE(img), …, _UNSIGNED LONG)` at its offset
- **THEN** it prints the colour it set (`FF0A141E` for `_RGB32(10, 20, 30)`), `SIZE` is 32 and `IMAGE` is the
  image's handle, as with the old compiler

#### Scenario: Samples of a new sound
- **WHEN** `n = _SNDNEW(800, 1, 16, 8000): m = _MEMSOUND(n): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE; m.SOUND = n` runs
- **THEN** it prints ` 1600  130  2 -1 `, as with the old compiler
