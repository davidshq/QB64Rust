# _WHATISMYIP$

`_WHATISMYIP$`, which once asked a web service for the machine's address, is not part of the language: a call is a
compile error.

## Syntax

```text
address$ = _WHATISMYIP$
```

## Behavior

A program may not call `_WhatIsMyIP$`, nor name a procedure as it, as with QB64pe.
<!-- src: language/network "The URL functions" -->

## Availability

<!-- begin generated: availability -->
Rejected, as in QB64pe. The compiler gives:

- `_WHATISMYIP$` is a name of QB64pe's auto-included file, reserved
<!-- end generated -->

<!-- probe: a$ = _WHATISMYIP$ -->

## See also

[`_OPENCLIENT`](_openclient.md).
