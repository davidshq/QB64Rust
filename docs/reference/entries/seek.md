# SEEK

`SEEK #n, position` sets where the next read or write of file `n` happens, and `SEEK(n)` gives that position. In a
binary file it is a byte, from 1; in a random file a record.

## Syntax

```text
SEEK [#]filenumber, position
position = SEEK(filenumber)
```

## Behavior

After `SEEK #1, 3`, `SEEK(1)` is 3 and the next `GET` reads the third byte.
<!-- src: language/file-io "File functions"; tests\reference\examples\seek_1 -->

In a binary file a position is a byte from 1; in a random file a record number from 1.
<!-- src: language/record-files "The position in a binary file"; language/record-files "Records of a random file" -->

## Availability

<!-- begin generated: availability -->
Supported. All 14 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:seek_1 -->
```qb64
OPEN "seek_1.tmp" FOR BINARY AS #1
s$ = "abcdef"
PUT #1, , s$
SEEK #1, 3
PRINT "The position is"; SEEK(1)
c$ = " "
GET #1, , c$
PRINT "The byte there is "; c$
CLOSE #1
KILL "seek_1.tmp"
END
```

Output:

```text
The position is 3 
The byte there is c

Press any key to continue
```
<!-- end generated -->

## See also

`LOC`, `GET`, `PUT`.
