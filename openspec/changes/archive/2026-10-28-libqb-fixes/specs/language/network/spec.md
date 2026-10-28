# Spec Delta

## ADDED Requirements

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
