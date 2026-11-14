# BEEP

`BEEP` rings the system's bell.

## Syntax

<!-- begin generated: syntax -->
```text
BEEP
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

`BEEP`, `CALL BEEP` and `BEEP:` followed by another statement compile to QB64pe's call. It uses the system's beep, not
the sound stream of `SOUND` and `PLAY`.
<!-- src: language/sound "BEEP"; D-062 -->

No test reaches it, since the bell would be heard.
<!-- src: tests\corpus\slice\s74_sound_queue -->

## Errors

- `BEEP 1`, `BEEP()` and `l = BEEP` are compile errors.
  <!-- src: language/sound "BEEP" -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN BEEP -->

## See also

[`SOUND`](sound.md), [`PLAY`](play.md).
