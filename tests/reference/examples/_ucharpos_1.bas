$CONSOLE:ONLY
img& = _NEWIMAGE(80, 30, 32)
_DEST img&
DIM p(1 TO 3) AS LONG
n = _UCHARPOS("abc", p())
_DEST _CONSOLE
PRINT n; "characters at"; p(1); p(2); p(3)
END
