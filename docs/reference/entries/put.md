# PUT

`PUT #n, position, variable` writes the bytes of a variable into a binary or random file. `PUT (x, y), array` with a
point is the graphics form, which draws an image saved by `GET`.

## Syntax

```text
PUT [#]filenumber[, [position][, variable]]
```

## Arguments

- `position`: a byte from 1 in a binary file, a record from 1 in a random file. Left out, the one after the last read
  or write.
- `variable`: a number, a string, a `TYPE` variable, an array element or a whole array `a()`.

## Behavior

`PUT` writes the variable's bytes: a LONG's 4, a `STRING * n`'s n, a `TYPE` variable's size, a whole array's elements
from the first. A variable-length string writes its characters.
<!-- src: language/record-files "What a variable reads and writes"; tests\reference\examples\put_1 -->

In a binary file, a `PUT` past the end fills the gap with zero bytes.
<!-- src: language/record-files "The position in a binary file" -->

In a random file, `PUT` writes only the variable's bytes at the record's start. A variable-length string is written as
two bytes of length and its characters.
<!-- src: language/record-files "Records of a random file" -->

With no variable, `PUT #n, record` writes the record buffer that `FIELD` names.
<!-- src: language/record-files "GET and PUT with no variable" -->

## Errors

- It raises error 52 for a file that is not open, 54 for one open for input, output or append, 63 for a position of 0
  or below, 59 for a value longer than a random file's record, and 75 for a file open with `ACCESS READ`.
  <!-- src: language/record-files "Run-time errors of GET and PUT" -->
- A literal or an expression as the variable is a compile error: "`PUT` needs a variable here".
  <!-- src: language/record-files "The forms of GET and PUT on a file" -->

## Availability

<!-- begin generated: availability -->
Supported. All 58 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:put_1 -->
```qb64
DIM n AS LONG, s AS STRING * 4
OPEN "put_1.tmp" FOR BINARY AS #1
n = 1000000: s = "abcd"
PUT #1, 1, n
PUT #1, , s
PRINT "LOF is"; LOF(1)
CLOSE #1
KILL "put_1.tmp"
END
```

Output:

```text
LOF is 8 

Press any key to continue
```
<!-- end generated -->

## See also

`GET`, `OPEN`, `SEEK`, `FIELD`.
