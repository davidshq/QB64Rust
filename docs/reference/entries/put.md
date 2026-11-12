# PUT

`PUT #n, position, variable` writes the bytes of a variable into a binary or random file. `PUT (x, y), array` with a
point is the graphics form, which draws an image saved by `GET`.

## Syntax

```text
PUT [#]filenumber[, [position][, variable]]
PUT [STEP](x, y), array[, [_CLIP] [PSET|PRESET|AND|OR|XOR][, color]]
```

## Arguments

- `position`: a byte from 1 in a binary file, a record from 1 in a random file. Left out, the one after the last read
  or write.
- `variable`: a number, a string, a `TYPE` variable, an array element or a whole array `a()`.
- `(x, y)`: in the graphics form, where the image's top left corner goes.
- `array`: in the graphics form, the array that `GET` filled, whole (`a` or `a()`) or from an element (`a(i)`).

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

The graphics form draws the rectangle that `GET` saved, with its top left corner at `(x, y)`. With `PSET` each pixel
replaces the one under it. With `XOR` the two colors are combined bit by bit: color 5 put with `XOR` over color 6
gives 3.
<!-- src: language/screen-output "Graphics GET and PUT"; tests\reference\examples\put_2 -->

The array of the graphics form is numeric, static or dynamic, or an array parameter. A plain variable, a string, a
`TYPE` variable or member, and an array of strings or of a `TYPE` are taken too, as the bytes from that place on.
<!-- src: language/screen-output "Forms of graphics GET and PUT"; verification\v56_d_get_put_kinds -->

## Errors

- It raises error 52 for a file that is not open, 54 for one open for input, output or append, 63 for a position of 0
  or below, 59 for a value longer than a random file's record, and 75 for a file open with `ACCESS READ`.
  <!-- src: language/record-files "Run-time errors of GET and PUT" -->
- A literal or an expression as the variable is a compile error: "`PUT` needs a variable here".
  <!-- src: language/record-files "The forms of GET and PUT on a file" -->
- In the graphics form, a value too small to hold the image's size raises error 5: `x = 1: PUT (0, 0), x`.
  <!-- src: language/screen-output "Forms of graphics GET and PUT"; verification\v16_m5_put_graphics_no_array; verification\v56_d_get_put_kinds -->
- Two action words, as `PUT (1, 2), a, PSET XOR`, or `_CLIP` after the action word are a compile error: "this `PUT`
  statement matches none of its forms".
  <!-- src: language/screen-output "Forms of graphics GET and PUT" -->
- A `_BIT` array is a compile error: "`PUT` of a `_BIT` variable".
  <!-- src: language/screen-output "Forms of graphics GET and PUT"; verification\v66_a18_put_bit_array -->

## Differences

- An index that calls a FUNCTION, as `PUT (0, 0), a(f(1))`, calls it once, as in QuickBASIC 4.5. QB64pe calls it
  twice (`DIVERGENCES-QB64PE.md` D-061).
  <!-- src: D-061; tests\corpus\slice\s162_get_put_index_once -->

## Availability

<!-- begin generated: availability -->
Supported. All 59 programs that use it compile (call-site programs, examples and probes).
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

The graphics form:

<!-- begin generated: example:put_2 -->
```qb64
' Take a 3-by-2 rectangle of an image with GET, put it back twice with PUT, and print each pixel's color.
DIM pic(10) AS INTEGER
canvas& = _NEWIMAGE(10, 4, 13)
_DEST canvas&
_SOURCE canvas&
LINE (0, 0)-(2, 1), 5, BF
PSET (1, 0), 7
GET (0, 0)-(2, 1), pic()
PUT (4, 0), pic(), PSET
LINE (4, 2)-(9, 3), 6, BF
PUT (7, 2), pic(), XOR
_DEST _CONSOLE
PRINT "Width in bits:"; pic(0); " Height:"; pic(1)
FOR y = 0 TO 3
    row$ = ""
    FOR x = 0 TO 9
        row$ = row$ + LTRIM$(STR$(POINT(x, y)))
    NEXT
    PRINT row$
NEXT
END
```

Output:

```text
Width in bits: 24  Height: 2 
5750575000
5550555000
0000666313
0000666333

Press any key to continue
```
<!-- end generated -->

## See also

`GET`, `OPEN`, `SEEK`, `FIELD`, `_PUTIMAGE`.
