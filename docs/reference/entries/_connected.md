# _CONNECTED

`_CONNECTED(handle)` tells whether a connection is still open at both ends.

## Syntax

<!-- begin generated: syntax -->
```text
_CONNECTED(connectionHandle&)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

-1 or 0, a LONG.

## Behavior

Both ends of a connection just made give -1; the host's end gives 0 after the client closes.
<!-- src: language/network "The network functions"; tests\corpus\slice\s141_tcp_loopback -->

A network program may make Windows ask the user to let it through the firewall, so no test runs one on the host; the
recorded programs run in the suites VM, and this page has no example.
<!-- src: tests\corpus\slice\s141_tcp_loopback -->

## Errors

- A handle that is no connection, such as 0 or a negative number never opened, raises error 52.
  <!-- src: language/network "The network functions"; tests\corpus\slice\s139_net_errors -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN k& = _CONNECTED(c&) -->

## See also

`_OPENCLIENT`, `_OPENCONNECTION`.
