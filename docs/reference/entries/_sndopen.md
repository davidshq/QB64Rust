# _SNDOPEN

`_SNDOPEN(file$)` opens a sound file and gives its handle, for `_SNDPLAY` and the other sound statements.
`_SNDOPEN(data$, "memory")` decodes the string itself.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDOPEN(fileName$[, capabilities$])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG handle above 0, or 0 when the sound cannot be opened.

## Behavior

A file libqb cannot decode gives 0, the empty name included.
<!-- src: language/sound "Opening a sound"; tests\corpus\slice\s127_sound_handles; tests\reference\examples\_sndopen_1 -->

A WAV of half a second built in a string and opened with `"memory"` gives handle 1 and a `_SNDLEN` of .5. Other words
in the second string, such as `"stream"` and `"nodecode"`, are libqb's.
<!-- src: language/sound "Opening a sound"; tests\corpus\slice\s128_sound_new_memory; tests\reference\examples\_sndopen_1 -->

A file being played stays open until libqb's sound loop frees it.
<!-- src: tests\corpus\slice\s129_sound_files_raw_midi -->

## Errors

- `_SNDOPEN()`, `_SNDOPEN(1)` and `_SNDOPEN(f$, "", 1)` are compile errors.
  <!-- src: language/sound "Forms of the sound handle functions" -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndopen_1 -->
```qb64
PRINT "No such file:"; _SNDOPEN("no such file.wav")
h = _SNDOPEN(Wav$(4000, 8000), "memory")
PRINT "From memory:"; h; _SNDLEN(h)
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
No such file: 0 
From memory: 1  .5 

Press any key to continue
```
<!-- end generated -->

## See also

[`_SNDCLOSE`](_sndclose.md), [`_SNDPLAY`](_sndplay.md), [`_SNDLEN`](_sndlen.md), [`_SNDCOPY`](_sndcopy.md).
