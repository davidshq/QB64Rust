# LOF

`LOF(n)` gives the length in bytes of open file `n`.

## Syntax

<!-- begin generated: syntax -->
```text
LOF(fileNumber&)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `filenumber`: the number of an open file.

## Result

An `_INTEGER64`.

## Behavior

`LOF` is the file's size: after six bytes are written to a binary file, it is 6.
<!-- src: language/file-io "File functions"; tests\reference\examples\lof_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:lof_1 -->
```qb64
OPEN "lof_1.tmp" FOR BINARY AS #1
s$ = "abcdef"
PUT #1, , s$
PRINT "The file holds"; LOF(1); "bytes."
CLOSE #1
KILL "lof_1.tmp"
END
```

Output:

```text
The file holds 6 bytes.

Press any key to continue
```
<!-- end generated -->

## See also

[`EOF`](eof.md), [`LOC`](loc.md), [`SEEK`](seek.md).
