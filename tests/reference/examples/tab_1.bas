$CONSOLE:ONLY
img& = _NEWIMAGE(80, 25, 0)
_DEST img&: _SOURCE img&
PRINT "Name"; TAB(12); "Men"
PRINT "Ney"; TAB(12); 3000
_DEST _CONSOLE
PRINT "["; RowText$(1); "]"
PRINT "["; RowText$(2); "]"
END

FUNCTION RowText$ (r)
    t$ = ""
    FOR c = 1 TO 30
        t$ = t$ + CHR$(SCREEN(r, c))
    NEXT c
    RowText$ = RTRIM$(t$)
END FUNCTION
