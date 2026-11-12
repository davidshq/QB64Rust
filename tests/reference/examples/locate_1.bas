$CONSOLE:ONLY
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
