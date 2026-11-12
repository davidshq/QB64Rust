# _SNDRAWDONE

`_SNDRAWDONE handle` once marked the end of what a raw stream was given. In this libqb it does nothing.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDRAWDONE [pipeHandle&]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

It compiles with or without a handle, and does nothing: libqb's routine is empty. The samples queued play without it.
<!-- src: language/sound "Forms of the sound handle statements"; read: libqb\internal\c\libqb\include\audio.h:62-66; tests\corpus\slice\s129_sound_files_raw_midi; tests\reference\examples\_sndrawdone_1 -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/sound "Forms of the sound handle statements" -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndrawdone_1 -->
```qb64
r = _SNDOPENRAW
_SNDVOL r, 0
FOR i = 1 TO 100: _SNDRAW 0, 0, r: NEXT
_SNDRAWDONE r
PRINT "Done, ERR is"; ERR
END
```

Output:

```text
Done, ERR is 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDRAW`, `_SNDRAWLEN`.
