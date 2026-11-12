$CONSOLE:ONLY
img& = _NEWIMAGE(80, 25, 0)
_DEST img&: _SOURCE img&
LOCATE 5, 10
PRINT "x";
r = CSRLIN: c = POS(0)
_DEST _CONSOLE
PRINT "After the x the cursor is at row"; r; "column"; c
END
