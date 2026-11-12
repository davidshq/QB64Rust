# _MIDISOUNDBANK

`_MIDISOUNDBANK file$` sets the sound bank that MIDI sounds opened later play with.

## Syntax

<!-- begin generated: syntax -->
```text
_MIDISOUNDBANK fileName$[, capabilities$]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

The bank is a file, or a string with `"memory"` and the bank's format as the second argument. The empty name sets the
default bank.
<!-- src: language/sound "MIDI sound banks"; tests\corpus\slice\s129_sound_files_raw_midi -->

A file that is not there leaves the bank as it was, with no error.
<!-- src: language/sound "MIDI sound banks"; tests\corpus\slice\s129_sound_files_raw_midi; tests\reference\examples\_midisoundbank_1 -->

A MIDI file of one second, opened from memory with `_SNDOPEN`, has a `_SNDLEN` of 1.
<!-- src: language/sound "Opening a sound"; tests\corpus\slice\s129_sound_files_raw_midi -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/sound "Forms of the sound handle statements" -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_midisoundbank_1 -->
```qb64
_MIDISOUNDBANK "no such bank.sf2"
PRINT "A bank that is not there, ERR is"; ERR
_MIDISOUNDBANK ""
PRINT "The default bank again, ERR is"; ERR
END
```

Output:

```text
A bank that is not there, ERR is 0 
The default bank again, ERR is 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDOPEN`, `$MIDISOUNDFONT`.
