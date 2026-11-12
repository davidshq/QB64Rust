' _ICON read back by the window's icons: Windows' WM_GETICON gives the big and the small icon, and the size of
' each is that of its color bitmap, read with GetIconInfo and GetObjectA (tests/sandbox/README.md).
DECLARE DYNAMIC LIBRARY "user32"
    FUNCTION SendMessageA%& (BYVAL hwnd AS _OFFSET, BYVAL msg AS _UNSIGNED LONG, BYVAL wp AS _OFFSET, BYVAL lp AS _OFFSET)
    FUNCTION GetIconInfo& (BYVAL hicon AS _OFFSET, BYVAL info AS _OFFSET)
END DECLARE
DECLARE DYNAMIC LIBRARY "gdi32"
    FUNCTION GetObjectA& (BYVAL h AS _OFFSET, BYVAL size AS LONG, BYVAL buf AS _OFFSET)
    FUNCTION DeleteObject& (BYVAL h AS _OFFSET)
END DECLARE
DIM SHARED h AS _OFFSET
SCREEN _NEWIMAGE(320, 200, 32)
_DELAY 1
h = _WINDOWHANDLE
OPEN "out.txt" FOR OUTPUT AS #1
PRINT #1, "at the start: "; icons$

i40& = _NEWIMAGE(40, 40, 32)
i20& = _NEWIMAGE(20, 20, 32)
_ICON i40&
_DELAY 0.5
PRINT #1, "after _ICON of a 40 x 40 image: "; icons$

_ICON i40&, i20&
_DELAY 0.5
PRINT #1, "after _ICON of 40 x 40 and 20 x 20: "; icons$

_ICON
_DELAY 0.5
PRINT #1, "after _ICON with no argument: "; icons$
CLOSE #1
SYSTEM

' The big icon's size and the small one's, or "none".
FUNCTION icons$
    icons$ = "big" + icon$(1) + ", small" + icon$(0)
END FUNCTION

FUNCTION icon$ (which AS LONG)
    DIM ii(3) AS _OFFSET, bm(7) AS LONG, hi AS _OFFSET
    hi = SendMessageA(h, &H7F, which, 0)
    IF hi = 0 THEN icon$ = " none": EXIT FUNCTION
    IF GetIconInfo(hi, _OFFSET(ii(0))) = 0 THEN icon$ = " not read": EXIT FUNCTION
    n& = GetObjectA(ii(3), 32, _OFFSET(bm(0)))
    icon$ = STR$(bm(1)) + " x" + STR$(bm(2))
    r& = DeleteObject(ii(2))
    r& = DeleteObject(ii(3))
END FUNCTION
