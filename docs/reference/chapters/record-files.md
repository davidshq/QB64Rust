# Binary and random files

`GET` and `PUT` read and write the bytes of a variable in a file open `FOR BINARY`, at a byte position, or `FOR
RANDOM`, at a record number. `FIELD` names parts of a random file's record.

The pages of this area: [GET](../entries/get.md), [PUT](../entries/put.md), [FIELD](../entries/field.md),
[LSET](../entries/lset.md), [LOCK](../entries/lock.md).

## Bytes and records

`PUT` writes and `GET` reads the bytes of a variable: a number its type's size, a fixed-length string its length, a
`TYPE` value its size.
<!-- src: language/record-files "What a variable reads and writes"; tests\reference\examples\put_1 -->

In a binary file the position is a byte from 1; in a random file a record number from 1, of the length `LEN =` gives,
128 when it is left out. Left out, it is the one after the last.
<!-- src: language/record-files "The position in a binary file"; language/record-files "Records of a random file" -->

A file that is not open raises error 52, one open for text error 54, and a position of 0 or below error 63.
<!-- src: language/record-files "Run-time errors of GET and PUT" -->

## The record buffer

`FIELD #n, width AS s$, …` names parts of the record buffer; `GET #n` with no variable fills them and `PUT #n` writes
them. `LSET` and `RSET` fill a field without changing its length.
<!-- src: language/record-files "FIELD"; language/record-files "GET and PUT with no variable"; language/record-files "LSET and RSET"; tests\reference\examples\field_1 -->
