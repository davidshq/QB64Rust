# _SNDPLAYFILE

`_SNDPLAYFILE file$, , volume` opens a sound file, plays it once and closes it.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDPLAYFILE fileName$[, , volume!]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

A file that is not there, or the empty name, does nothing, with no error.
<!-- src: tests\corpus\slice\s129_sound_files_raw_midi; tests\reference\examples\_sndplayfile_1 -->

The second argument is accepted and has no effect in libqb.
<!-- src: read: libqb\internal\c\parts\audio\audio.cpp:3087-3109; tests\corpus\slice\s129_sound_files_raw_midi -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/sound "Forms of the sound handle statements" -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndplayfile_1 -->
```qb64
_SNDPLAYFILE "no such file.wav", , 0
PRINT "No such file, ERR is"; ERR
END
```

Output:

```text
No such file, ERR is 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDOPEN`, `_SNDPLAYCOPY`.
