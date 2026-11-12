# LOC

`LOC(n)` gives the position of the last read or write of file `n`: in a binary file the last byte, in a random file
the last record.

## Syntax

<!-- begin generated: syntax -->
```text
LOC(fileOrPortNumber%)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `filenumber`: the number of an open file.

## Result

An `_INTEGER64`.

## Behavior

After writing 6 bytes to a binary file, `LOC` is 6; after reading the byte at 2, it is 2.
<!-- src: language/file-io "File functions"; tests\reference\examples\loc_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:loc_1 -->
```qb64
OPEN "loc_1.tmp" FOR BINARY AS #1
s$ = "abcdef"
PUT #1, , s$
PRINT "After writing 6 bytes, LOC is"; LOC(1)
c$ = " "
GET #1, 2, c$
PRINT "After reading byte 2, LOC is"; LOC(1)
CLOSE #1
KILL "loc_1.tmp"
END
```

Output:

```text
After writing 6 bytes, LOC is 6 
After reading byte 2, LOC is 2 

Press any key to continue
```
<!-- end generated -->

## See also

`SEEK`, `LOF`, `GET`.
