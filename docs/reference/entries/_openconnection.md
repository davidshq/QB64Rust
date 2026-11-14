# _OPENCONNECTION

`_OPENCONNECTION(host)` takes the next connection a host has waiting, and gives its handle, or 0 when none is waiting.

## Syntax

<!-- begin generated: syntax -->
```text
_OPENCONNECTION(hostHandle)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG.

## Behavior

A program calls it in a loop until it gives a handle. The handle differs from the host's and the client's.
<!-- src: language/network "The network functions"; tests\corpus\slice\s141_tcp_loopback -->

A network program may make Windows ask the user to let it through the firewall, so no test runs one on the host; the
recorded programs run in the suites VM, and this page has no example.
<!-- src: tests\corpus\slice\s141_tcp_loopback -->

## Errors

- A handle that is no host, such as 0 or -5, raises error 258.
  <!-- src: language/network "The network functions"; tests\corpus\slice\s139_net_errors -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN c& = _OPENCONNECTION(h&) -->

## See also

[`_OPENHOST`](_openhost.md), [`_CONNECTED`](_connected.md).
