# LOCATE

`LOCATE row, column` moves the text cursor, so that the next `PRINT` starts there: `LOCATE 3, 5`.

## Syntax

<!-- begin generated: syntax -->
```text
LOCATE [row%][, column%] [, cursor%][, cursorStart%, cursorStop%]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

`LOCATE 3, 5: PRINT "Row 3, column 5"` prints from row 3, column 5.
<!-- src: language/screen-output "Text-screen statements"; tests\reference\examples\locate_1 -->

An argument may be left out where QB64pe allows: `LOCATE , 20` changes only the column. After it, `CSRLIN` and
`POS(0)` give the new place.
<!-- src: language/screen-output "Text-screen statements"; language/screen-output "Functions that read the screen and the keyboard"; tests\reference\examples\csrlin_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `LOCATE 1, 2, 3, 4, 5, 6`: "this `LOCATE` statement matches none of its forms".
  - `LOCATE , , , ,`: "this `LOCATE` statement matches none of its forms".
  <!-- src: language/screen-output "Text-screen statements"; verification\v27_x04_locate_six; verification\v27_x18_locate_four_commas -->

## Availability

<!-- begin generated: availability -->
Supported. All 24 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:locate_1 -->
```qb64
img& = _NEWIMAGE(80, 25, 0)
_DEST img&: _SOURCE img&
LOCATE 3, 5
PRINT "Row 3, column 5"
_DEST _CONSOLE
PRINT "["; RowText$(3); "]"
END

FUNCTION RowText$ (r)
    t$ = ""
    FOR c = 1 TO 30
        t$ = t$ + CHR$(SCREEN(r, c))
    NEXT c
    RowText$ = RTRIM$(t$)
END FUNCTION
```

Output:

```text
[    Row 3, column 5]

Press any key to continue
```
<!-- end generated -->

## See also

[`CSRLIN`](csrlin.md), [`POS`](pos.md), [`PRINT`](print.md), [`CLS`](cls.md).
