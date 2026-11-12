# _OPENHOST

`_OPENHOST("TCP/IP:port")` opens a host that listens for connections on a TCP/IP port, and gives its handle, for
`_OPENCONNECTION`.

## Syntax

<!-- begin generated: syntax -->
```text
_OPENHOST(TCP/IP:portNumber)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG: a negative handle, or 0 when the port cannot be opened.

## Behavior

A host on a free port gives a handle below 0.
<!-- src: language/network "The network functions"; tests\corpus\slice\s141_tcp_loopback -->

A port that is not a number gives -2.
<!-- src: tests\corpus\slice\s139_net_errors -->

A program that uses it is built with libqb's sockets and HTTP part.
<!-- src: language/network "The sockets flag and part" -->

A network program may make Windows ask the user to let it through the firewall, so no test runs one on the host; the
recorded programs run in the suites VM, and this page has no example.
<!-- src: tests\corpus\slice\s141_tcp_loopback -->

## Errors

- A text that is not `TCP/IP:` and a port, such as `foobar`, and the empty string raise error 5.
  <!-- src: language/network "The network functions"; tests\corpus\slice\s139_net_errors -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN h& = _OPENHOST("TCP/IP:47392") -->

## See also

`_OPENCONNECTION`, `_OPENCLIENT`, `CLOSE`.
