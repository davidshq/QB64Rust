# _SNDRATE

`_SNDRATE` gives the sample rate of the machine's audio device, in frames a second.

## Syntax

<!-- begin generated: syntax -->
```text
_SNDRATE
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG, above 0.

## Behavior

The rate is the device's, so a program prints it only as `_SNDRATE > 0`.
<!-- src: tests\corpus\slice\s129_sound_files_raw_midi; tests\reference\examples\_sndrate_1 -->

A sound opened from a file is decoded at this rate, so the size of its `_MEMSOUND` block depends on it.
<!-- src: tests\corpus\slice\s128_sound_new_memory -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `l = _SNDRATE(1)`: "`_SNDRATE` takes no arguments".
  - `l = _SNDRATE()`: "`_SNDRATE` with empty parentheses".
  - `CALL _SNDRATE`: "`_SNDRATE` is a function, not a statement".
  <!-- src: language/sound "Forms of the sound handle functions"; verification\v39_x13_sndrate_arg; verification\v39_x14_sndrate_empty; verification\v39_x45_call_function -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sndrate_1 -->
```qb64
PRINT "The device's sample rate is over 0:"; _SNDRATE > 0
END
```

Output:

```text
The device's sample rate is over 0:-1 

Press any key to continue
```
<!-- end generated -->

## See also

`_SNDRAW`, `_SNDNEW`.
