# OPEN

`OPEN` opens a file and gives it a number, for `PRINT #`, `INPUT #`, `GET` and the other file statements: `OPEN
"scores.txt" FOR INPUT AS #1`.

## Syntax

```text
OPEN name$ [FOR mode] [ACCESS access] [SHARED | LOCK lock] AS [#]filenumber [LEN = recordlength]
OPEN mode$, [#]filenumber, name$[, recordlength]
```

## Arguments

- `mode`: `INPUT` to read text, `OUTPUT` to write a new text file, `APPEND` to add to one, `BINARY` to read and write
  bytes, `RANDOM` (the default) for records.
- `filenumber`: a number from 1, which the other statements name. `FREEFILE` gives a free one.
- `recordlength`: the length of a record of a `RANDOM` file, 128 when left out.

## Behavior

`OPEN` opens the file as QB64pe's runtime does, for every mode, `ACCESS` and lock word. `OUTPUT` makes a new, empty
file; `APPEND` writes after what the file holds.
<!-- src: language/file-io "OPEN and CLOSE"; tests\reference\examples\open_1 -->

The file number may be any expression. The old form `OPEN "O", #1, name$` works too.
<!-- src: language/file-io "OPEN and CLOSE" -->

A file opened `FOR INPUT` that does not exist raises error 53.
<!-- src: language/file-io "OPEN and CLOSE"; tests\reference\examples\kill_1 -->

## Errors

- Opening a file that cannot be found, for input, raises run-time error 53.
  <!-- src: language/file-io "OPEN and CLOSE" -->

## Availability

<!-- begin generated: availability -->
Supported. All 38 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:open_1 -->
```qb64
OPEN "open_1.tmp" FOR OUTPUT AS #1
PRINT #1, "first line"
CLOSE #1
OPEN "open_1.tmp" FOR APPEND AS #1
PRINT #1, "added line"
CLOSE #1
OPEN "open_1.tmp" FOR INPUT AS #1
DO UNTIL EOF(1)
    LINE INPUT #1, l$
    PRINT l$
LOOP
CLOSE #1
KILL "open_1.tmp"
END
```

Output:

```text
first line
added line

Press any key to continue
```
<!-- end generated -->

## See also

`CLOSE`, `PRINT`, `INPUT`, `LINE INPUT`, `GET`, `PUT`, `FREEFILE`.
