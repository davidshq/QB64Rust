# LINE

`LINE` draws a straight line between two points, or a box with the two points as its corners, on a graphics
screen or image.

## Syntax

<!-- begin generated: syntax -->
```text
LINE [STEP] [(column1, row1)]-[STEP] (column2, row2), color[, [{B|BF}], style%]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

Each part may be left out where it is in brackets. For example: `LINE (0, 0)-(100, 50)`, `LINE -(200, 10), 4`,
`LINE (10, 10)-STEP(20, 20), 2, BF` and `LINE (0, 0)-(99, 0), , , &HF0F0`.

## Arguments

- `(column1, row1)`: the first point. Left out, the line starts at the last point drawn.
- `(column2, row2)`: the second point.
- `STEP`: before a point, makes it relative: `STEP(dx, dy)` is `dx` and `dy` away from the last point drawn.
- `color`: the color to draw with. Left out, the current drawing color, as `COLOR` set it.
- `B`: draw a box whose opposite corners are the two points. `BF`: draw it filled. Either may be written in lower
  case.
- `style%`: a 16-bit mask that makes a dashed line, such as `&HF0F0`.

The coordinates and the color may be of any numeric type (`language/screen-output`, "Drawing statements").

## Behavior

Without `B` or `BF`, `LINE` draws a line from the first point to the second. `B` draws the four sides of the box
whose opposite corners are the two points, and `BF` fills that box.
<!-- src: language/screen-output "Drawing statements"; tests\corpus\screen\d06_pset_line -->

The second point becomes the last point drawn, so the next `LINE -(x, y)` or `STEP` starts from there. A chain of
`LINE -(x, y)` statements draws joined lines.
<!-- src: read: libqb\internal\c\libqb.cpp:8716-8733; tests\corpus\screen\d06_pset_line; tests\corpus\screen\g04_ironclad -->

`STEP` before the first point counts from the last point drawn. `STEP` before the second point counts from the
first point of the same `LINE`.
<!-- src: read: libqb\internal\c\libqb.cpp:8716-8733; tests\corpus\screen\d06_pset_line -->

A style draws the line, or each side of a box, as a dash pattern. The first pixel takes the mask's top bit
(`&H8000`), the next pixel the bit below it, and so on, starting again after 16 pixels. A pixel is drawn where its
bit is 1 and left as it was where it is 0. `BF` takes no style: the box is filled solid.
<!-- src: read: libqb\internal\c\libqb.cpp:8639-8689; read: libqb\internal\c\libqb.cpp:8758-8764; tests\corpus\screen\d06_pset_line -->

Points outside the screen or image are allowed: only the part inside is drawn.
<!-- src: tests\corpus\screen\d06_pset_line -->

## Errors

These are compile errors:

- No second point, as in `LINE (1, 2), 3`, or coordinates without parentheses, as in `LINE 0, 0, 9, 9`.
  <!-- src: language/screen-output "Drawing statements"; verification\v27_x03_line_no_second_point; verification\v16_m5_line_no_parens -->
- A word other than `B` or `BF` after the color, a comma after `B` with no style, and an argument after the style.
  <!-- src: verification\v27_x41_line_box_word; verification\v27_x19_line_comma_after_box; verification\v16_m5_line_extra_arg -->
- A string as the color: "argument 5 of `LINE` needs a number".
  <!-- src: verification\v27_x42_line_color_string -->

A form that matches none of these is reported as "this `LINE` statement matches none of its forms", followed by the
form.
<!-- src: verification\v27_x03_line_no_second_point -->

At run time, `LINE` on a text screen (`SCREEN 0`) raises error 5, "Illegal function call".
<!-- src: tests\corpus\screen\d14_errors; read: libqb\internal\c\libqb.cpp:8704-8707 -->

## Differences

From QuickBASIC 4.5 and BASIC PDS 7.1: QB 4.5 lights other pixels for many sloping lines (492 of 625 lines
measured), and a dash pattern starts at the left end of the line where QB64pe starts it at the first point
written. QB64Rust draws as QB64pe does (`DIVERGENCES-QB45.md` Q-024 and Q-025).

## Availability

<!-- begin generated: availability -->
Supported. All 36 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

The example draws on an image in memory and prints its pixels as digits, so that it runs in a console.

<!-- begin generated: example:line_1 -->
```qb64
' Draw on an image 12 pixels wide and 5 high, then print each pixel's color as a digit.
canvas& = _NEWIMAGE(12, 5, 13)
_DEST canvas&
LINE (0, 0)-(11, 4), 1, B
LINE (2, 1)-(4, 3), 2, BF
LINE (6, 1)-(10, 3), 3
LINE -STEP(0, -2), 4
_DEST _CONSOLE
_SOURCE canvas&
FOR y = 0 TO 4
    row$ = ""
    FOR x = 0 TO 11
        row$ = row$ + LTRIM$(STR$(POINT(x, y)))
    NEXT
    PRINT row$
NEXT
END
```

Output:

```text
111111111111
102220300041
102220033041
102220000341
111111111111

Press any key to continue
```
<!-- end generated -->

## See also

`PSET`, `CIRCLE`, `PAINT`, `DRAW`, `COLOR`, `SCREEN`, `_NEWIMAGE`.
