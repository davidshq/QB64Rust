$CONSOLE:ONLY
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
