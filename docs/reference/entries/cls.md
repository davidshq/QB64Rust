# CLS

`CLS` clears the screen, or the text or graphics viewport, and puts the cursor at the top left.

## Syntax

<!-- begin generated: syntax -->
```text
CLS [method%][, bgColor&][, dest&]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

After `CLS`, the screen is empty and the cursor is at row 1.
<!-- src: language/screen-output "Text-screen statements"; tests\reference\examples\cls_1 -->

`CLS` is compiled in every form QB64pe accepts, with its arguments, and changes the screen as its program does.
<!-- src: language/screen-output "Text-screen statements" -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `CLS 1, 2, 3, 4`: "this `CLS` statement matches none of its forms".
  <!-- src: language/screen-output "Text-screen statements"; verification\v27_x37_cls_four -->

## Availability

<!-- begin generated: availability -->
Supported. All 24 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:cls_1 -->
```qb64
img& = _NEWIMAGE(80, 25, 0)
_DEST img&: _SOURCE img&
PRINT "This line is cleared."
CLS
r = CSRLIN
_DEST _CONSOLE
PRINT "After CLS, row 1 holds ["; RowText$(1); "] and the cursor is at row"; r
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
After CLS, row 1 holds [] and the cursor is at row 1 

Press any key to continue
```
<!-- end generated -->

## See also

[`LOCATE`](locate.md), [`COLOR`](color.md), [`VIEW PRINT`](view-print.md).
