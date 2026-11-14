# _CONNECTIONADDRESS

`_CONNECTIONADDRESS$(handle)` gives the address of a connection or host, as `TCP/IP:port:address`.

## Syntax

```text
address$ = _CONNECTIONADDRESS$(handle)
address$ = _CONNECTIONADDRESS(handle)
```

## Result

A STRING.

## Behavior

A client's address is the one it was opened with: `TCP/IP:47392:localhost`. A connection a host took gives the other
end's port and its address, such as `127.0.0.1`.
<!-- src: language/network "The network functions"; tests\corpus\slice\s141_tcp_loopback -->

It may be written without its `$`.
<!-- src: language/network "The network functions"; tests\corpus\slice\s139_net_errors -->

A network program may make Windows ask the user to let it through the firewall, so no test runs one on the host; the
recorded programs run in the suites VM, and this page has no example.
<!-- src: tests\corpus\slice\s141_tcp_loopback -->

## Errors

- A handle that is no connection, such as 0 or a negative number never opened, raises error 52.
  <!-- src: language/network "The network functions"; tests\corpus\slice\s139_net_errors -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN a$ = _CONNECTIONADDRESS$(c&) + _CONNECTIONADDRESS(c&) -->

## See also

[`_OPENCLIENT`](_openclient.md), [`_OPENHOST`](_openhost.md).
