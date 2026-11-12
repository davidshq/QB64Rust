# $MIDISOUNDFONT

`$MIDISOUNDFONT:` is a metacommand that QB64pe has replaced by the statement `_MIDISOUNDBANK`; it is now a compile
error.

## Syntax

```text
$MIDISOUNDFONT: value
```

## Behavior

With any value, in any case, it is a compile error that names `_MIDISOUNDBANK`. In a comment or after `REM` it is a
comment.
<!-- src: language/sound "$MIDISOUNDFONT" -->

## Availability

<!-- begin generated: availability -->
Rejected, as in QB64pe. The compiler gives:

- $MIDISOUNDFONT is a deprecated keyword, use _MIDISOUNDBANK instead
<!-- end generated -->

<!-- probe: $MIDISOUNDFONT:DEFAULT -->

## See also

`_MIDISOUNDBANK`.
