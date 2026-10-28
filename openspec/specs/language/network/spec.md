# language/network Specification

## Purpose
The network functions: TCP/IP hosts, clients and connections and HTTP clients through libqb's network part, with
the precompiler flag `_SOCKETS_` and the URL functions of QB64pe's auto-included file, as programs built with the
old compiler have them.

## Requirements
### Requirement: The network functions
`_OPENHOST(s$)`, `_OPENCLIENT(s$)`, `_OPENCONNECTION(h)`, `_CONNECTED(h)` and `_STATUSCODE(h)` SHALL compile as
LONG functions and `_CONNECTIONADDRESS$(h)` (also without the `$`) as a string function, each the libqb call the
old compiler writes, with its argument checks. Their run-time results and errors SHALL be libqb's.

#### Scenario: Results where nothing listens
- **WHEN** a program with `ON ERROR` calls `_OPENCLIENT("nonsense")`, `_OPENCLIENT("TCP/IP:1:localhost")`,
  `_OPENCLIENT("HTTP:NotARealURL")` and `_STATUSCODE(0)`
- **THEN** it gets error 5, 0, 0 and error 52, as the old compiler's program does

#### Scenario: Wrong arguments
- **WHEN** `_OPENCLIENT` has no argument, two, or a number, or `_STATUSCODE` a string
- **THEN** it is a compile error, as the old compiler's

#### Scenario: Not a statement
- **WHEN** a program has `_OPENHOST "TCP/IP:1"` as a statement
- **THEN** it is a compile error

### Requirement: Files on a network handle
`GET`, `PUT`, `EOF`, `LOF` and `CLOSE` SHALL take a network handle where they take a file number, written as the
old compiler writes them, so that a program reads and writes a connection as libqb provides.

#### Scenario: A handle that is not open
- **WHEN** a program with `ON ERROR` runs `GET #-5, , s$`, `PUT #-5, , s$`, `EOF(-5)` and `LOF(-5)`, then
  `CLOSE #-5`
- **THEN** each of the first four raises error 52 and `CLOSE` raises nothing, as with the old compiler

#### Scenario: An exchange over the loopback address
- **WHEN** a program opens a host on a TCP/IP port, a client to it on `localhost`, accepts the connection, and
  sends a string each way with `PUT` and reads it with `GET`
- **THEN** it prints what the old compiler's program prints

### Requirement: The sockets flag and part
`_SOCKETS_` SHALL be 1 for the whole program when `_OPENHOST` or `_OPENCLIENT` stands in code that is compiled,
wherever it stands, or when `$DEBUG` is in force; such a program SHALL link libqb's sockets and HTTP part. The
other network functions SHALL set neither.

#### Scenario: In a SUB never called
- **WHEN** `_OPENCLIENT` stands only in a SUB that is never called, below a `$IF _SOCKETS_ THEN`
- **THEN** the branch is taken

#### Scenario: In a branch not taken
- **WHEN** `_OPENCLIENT` stands only in a `$IF 0 THEN` branch
- **THEN** `_SOCKETS_` is 0

#### Scenario: The other functions
- **WHEN** a program uses `_OPENCONNECTION`, `_CONNECTED` and `_STATUSCODE` only
- **THEN** `_SOCKETS_` is 0 and the program builds without the sockets part

### Requirement: The URL functions
`_ENCODEURL$(s$)` and `_DECODEURL$(s$)` SHALL be functions of every program, computed as QB64pe's auto-included
file computes them, also under `OPTION _EXPLICIT` and called without the `$`. A program SHALL NOT call
`_WhatIsMyIP$`, nor name a procedure as one of these functions.

#### Scenario: Encoding and decoding
- **WHEN** a program prints `_ENCODEURL$` and `_DECODEURL$` of the strings of `verification\v44_d_url`
- **THEN** it prints what the old compiler's program prints

#### Scenario: A procedure of that name
- **WHEN** a program declares `FUNCTION _EncodeURL$ (u$)`
- **THEN** it is a compile error, as the old compiler's "Invalid name"

#### Scenario: The helper
- **WHEN** a program calls `_WHATISMYIP$`, with or without a network function
- **THEN** it is a compile error, as the old compiler's

### Requirement: _STATUSCODE of a handle that is not open
`_STATUSCODE(h)` of a negative handle that was never opened, or was opened and closed, SHALL raise error 52 as the
other network functions do, and of a handle of another kind (a host or a TCP/IP connection) error 52 as with the
old compiler (`study\39` row 11; a `QB64PE-DIVERGENCES.md` row).

#### Scenario: Never opened
- **WHEN** a program with a handler that prints `ERR` and resumes next runs `PRINT _STATUSCODE(-5)`
- **THEN** the handler prints 52 and the program goes on

#### Scenario: Opened and closed
- **WHEN** a program opens a host with `_OPENHOST`, closes it, and calls `_STATUSCODE` of its handle under a
  handler
- **THEN** the handler prints 52
