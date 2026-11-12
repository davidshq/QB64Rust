# EOF

`EOF(n)` tells whether file `n` has been read to its end: -1 when it has, 0 when there is more.

## Syntax

<!-- begin generated: syntax -->
```text
EOF(fileNumber&)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `filenumber`: the number of an open file.

## Result

-1 or 0.

## Behavior

`EOF` is 0 while there is more to read and -1 after the last line, so `WHILE NOT EOF(1)` reads a whole file.
<!-- src: language/file-io "File functions"; tests\reference\examples\eof_1 -->

In a binary file, a `GET` at or past the end sets `EOF`.
<!-- src: language/record-files "The position in a binary file" -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:eof_1 -->
```qb64
OPEN "eof_1.tmp" FOR OUTPUT AS #1
PRINT #1, "one": PRINT #1, "two"
CLOSE #1
OPEN "eof_1.tmp" FOR INPUT AS #1
n = 0
WHILE NOT EOF(1)
    LINE INPUT #1, l$
    n = n + 1
WEND
CLOSE #1
PRINT "Read"; n; "lines."
KILL "eof_1.tmp"
END
```

Output:

```text
Read 2 lines.

Press any key to continue
```
<!-- end generated -->

## See also

`OPEN`, `LINE INPUT`, `LOF`.
