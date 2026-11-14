# _OPENCLIENT

`_OPENCLIENT("TCP/IP:port:address")` opens a connection to a host, and `_OPENCLIENT("HTTP:url")` a request for a web
address. It gives the connection's handle.

## Syntax

<!-- begin generated: syntax -->
```text
_OPENCLIENT(TCP/IP:port:address)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG: a negative handle, or 0 when it cannot connect.

## Behavior

A client of a host on the same machine gives a handle below 0, which `GET` and `PUT` read and write.
<!-- src: language/network "The network functions"; language/network "Files on a network handle"; tests\corpus\slice\s141_tcp_loopback -->

A name in lower case (`tcp/ip:`), a port that is not a number, an HTTP address that names no host and one that is not
`http`, each give 0.
<!-- src: tests\corpus\slice\s139_net_errors -->

A program that uses it is built with libqb's sockets and HTTP part.
<!-- src: language/network "The sockets flag and part" -->

A network program may make Windows ask the user to let it through the firewall, so no test runs one on the host; the
recorded programs run in the suites VM, and this page has no example.
<!-- src: tests\corpus\slice\s141_tcp_loopback -->

## Errors

- A text with no kind, such as `nonsense`, and the empty string raise error 5.
  <!-- src: language/network "The network functions"; tests\corpus\slice\s139_net_errors -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN c& = _OPENCLIENT("TCP/IP:47392:localhost") -->

## See also

[`_OPENHOST`](_openhost.md), [`_CONNECTED`](_connected.md), [`_STATUSCODE`](_statuscode.md), [`GET`](get.md), [`PUT`](put.md).
