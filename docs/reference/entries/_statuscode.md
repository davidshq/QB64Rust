# _STATUSCODE

`_STATUSCODE(handle)` gives the HTTP status of a request opened with `_OPENCLIENT("HTTP:url")`, such as 200.

## Syntax

<!-- begin generated: syntax -->
```text
_STATUSCODE(httpHandle&)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG.

## Behavior

A network program may make Windows ask the user to let it through the firewall, so no test runs one on the host; the
recorded programs run in the suites VM, and this page has no example.
<!-- src: tests\corpus\slice\s141_tcp_loopback -->

## Errors

- A handle that was never opened or was closed, and a handle of another kind (a host or a TCP/IP connection), raise
  error 52.
  <!-- src: language/network "_STATUSCODE of a handle that is not open"; tests\corpus\slice\s139_net_errors -->

## Differences

QB64pe read past its list for a negative handle that is not open, and crashed with an access violation.
<!-- src: language/network "_STATUSCODE of a handle that is not open"; D-051 -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN s& = _STATUSCODE(c&) -->

## See also

[`_OPENCLIENT`](_openclient.md).
