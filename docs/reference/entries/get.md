# GET

`GET #n, position, variable` reads bytes of a binary or random file into a variable. `GET (x1, y1)-(x2, y2), array` is
the graphics form, which saves part of the screen.

## Syntax

```text
GET [#]filenumber[, [position][, variable]]
```

## Arguments

- `position`: a byte from 1 in a binary file, a record from 1 in a random file. Left out, the one after the last read
  or write.
- `variable`: a number, a string, a `TYPE` variable, an array element or a whole array `a()`.

## Behavior

`GET` reads as many bytes as the variable holds: a `TYPE` variable reads one record of its size.
<!-- src: language/record-files "What a variable reads and writes"; tests\reference\examples\get_1 -->

A variable-length string reads as many characters as it holds. In a random file it reads the length and characters
that `PUT` wrote.
<!-- src: language/record-files "What a variable reads and writes"; language/record-files "Records of a random file" -->

In a binary file, a `GET` at or past the end sets the bytes it cannot read to 0 and sets `EOF`, without growing the
file.
<!-- src: language/record-files "The position in a binary file" -->

With no variable, `GET #n, record` reads the record into the buffer that `FIELD` names.
<!-- src: language/record-files "GET and PUT with no variable" -->

## Errors

- It raises error 52 for a file that is not open, 54 for one open for input, output or append, 63 for a position of 0
  or below, and 75 for a file open with `ACCESS WRITE`.
  <!-- src: language/record-files "Run-time errors of GET and PUT" -->

## Availability

<!-- begin generated: availability -->
Supported. All 61 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:get_1 -->
```qb64
TYPE Rec
    nm AS STRING * 8
    men AS LONG
END TYPE
DIM r AS Rec, back AS Rec
OPEN "get_1.tmp" FOR RANDOM AS #1 LEN = LEN(r)
r.nm = "Davout": r.men = 26000: PUT #1, 1, r
r.nm = "Lannes": r.men = 20000: PUT #1, 2, r
GET #1, 2, back
PRINT back.nm; back.men
GET #1, 1, back
PRINT back.nm; back.men
CLOSE #1
KILL "get_1.tmp"
END
```

Output:

```text
Lannes   20000 
Davout   26000 

Press any key to continue
```
<!-- end generated -->

## See also

`PUT`, `OPEN`, `SEEK`, `FIELD`, `EOF`.
