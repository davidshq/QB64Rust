' _WINDOWSIZELIMIT read back by the sizes the window gives Windows for a resize: Windows' WM_GETMINMAXINFO,
' sent to the program's window, less the window's frame (tests/sandbox/README.md). A size the window does not set
' stays as it was sent, -7.
DECLARE DYNAMIC LIBRARY "user32"
    FUNCTION SendMessageA%& (BYVAL hwnd AS _OFFSET, BYVAL msg AS _UNSIGNED LONG, BYVAL wp AS _OFFSET, BYVAL lp AS _OFFSET)
    FUNCTION GetWindowRect& (BYVAL hwnd AS _OFFSET, BYVAL rect AS _OFFSET)
    FUNCTION GetClientRect& (BYVAL hwnd AS _OFFSET, BYVAL rect AS _OFFSET)
END DECLARE
DIM SHARED h AS _OFFSET
SCREEN _NEWIMAGE(320, 200, 32)
_DELAY 1
h = _WINDOWHANDLE
OPEN "out.txt" FOR OUTPUT AS #1

_WINDOWSIZELIMIT (200, 150)-(400, 300)
_DELAY 0.5
PRINT #1, "after (200, 150)-(400, 300): "; limits$

_WINDOWSIZELIMIT
_DELAY 0.5
PRINT #1, "after no arguments: "; limits$

_WINDOWSIZELIMIT (100, 80)
_DELAY 0.5
PRINT #1, "after (100, 80): "; limits$

_WINDOWSIZELIMIT -(500, 400)
_DELAY 0.5
PRINT #1, "after -(500, 400): "; limits$

SCREEN 0
WIDTH 80, 25
_DELAY 1
_WINDOWSIZELIMIT (40, 10)-(80, 25)
_DELAY 0.5
PRINT #1, "in SCREEN 0 after (40, 10)-(80, 25): "; limits$
CLOSE #1
SYSTEM

' The smallest and largest sizes the window asks Windows for, without the frame: min w, h; max w, h.
FUNCTION limits$
    DIM mmi(9) AS LONG, wr(3) AS LONG, cr(3) AS LONG
    FOR i = 0 TO 9: mmi(i) = -7: NEXT
    r%& = SendMessageA(h, &H24, 0, _OFFSET(mmi(0)))
    r& = GetWindowRect(h, _OFFSET(wr(0)))
    r& = GetClientRect(h, _OFFSET(cr(0)))
    fw = (wr(2) - wr(0)) - cr(2)
    fh = (wr(3) - wr(1)) - cr(3)
    limits$ = "min" + size$(mmi(6), mmi(7), fw, fh) + " max" + size$(mmi(8), mmi(9), fw, fh)
END FUNCTION

FUNCTION size$ (w AS LONG, ht AS LONG, fw, fh)
    IF w = -7 THEN a$ = " -7" ELSE a$ = STR$(w - fw)
    IF ht = -7 THEN b$ = " -7" ELSE b$ = STR$(ht - fh)
    size$ = a$ + "," + b$
END FUNCTION
