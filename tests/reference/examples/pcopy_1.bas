$CONSOLE:ONLY
img& = _NEWIMAGE(80, 25, 0)
SCREEN img&, , 1, 0
PRINT "page 1";
PCOPY 1, 0
SCREEN , , 0, 0
_SOURCE img&
s$ = CHR$(SCREEN(1, 1)) + CHR$(SCREEN(1, 2)) + CHR$(SCREEN(1, 3)) + CHR$(SCREEN(1, 4))
_DEST _CONSOLE
PRINT "Page 0 after PCOPY 1, 0 starts with: "; s$
END
