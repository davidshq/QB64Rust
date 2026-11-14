# The network

A program opens a TCP/IP host or client, or an HTTP request, as a handle, and reads and writes it with `GET` and `PUT`
as it would a file.

The pages of this area: [_OPENHOST](../entries/_openhost.md), [_OPENCLIENT](../entries/_openclient.md),
[_OPENCONNECTION](../entries/_openconnection.md), [_CONNECTED](../entries/_connected.md),
[_CONNECTIONADDRESS](../entries/_connectionaddress.md), [_STATUSCODE](../entries/_statuscode.md),
[_WHATISMYIP$](../entries/_whatismyip.md).

## Hosts, clients and connections

`_OPENHOST("TCP/IP:port")` listens on a port. `_OPENCONNECTION` takes the next connection waiting, and
`_OPENCLIENT("TCP/IP:port:address")` connects to a host. Each gives a negative handle.
<!-- src: language/network "The network functions"; tests\corpus\slice\s141_tcp_loopback -->

`GET`, `PUT`, `EOF`, `LOF` and `CLOSE` take such a handle where they take a file number.
<!-- src: language/network "Files on a network handle"; tests\corpus\slice\s141_tcp_loopback -->

A handle that is no connection raises error 52, and a text `_OPENHOST` or `_OPENCLIENT` cannot read raises error 5.
<!-- src: language/network "The network functions"; tests\corpus\slice\s139_net_errors -->

## Building and testing

A program that uses `_OPENHOST` or `_OPENCLIENT` is built with libqb's sockets part.
<!-- src: language/network "The sockets flag and part" -->

A network program may make Windows ask the user to let it through the firewall, so the tests run them in the suites
VM, never on the host.
<!-- src: tests\corpus\slice\s141_tcp_loopback -->
