$CONSOLE:ONLY
img& = _NEWIMAGE(80, 25, 0)
_DEST img&: _SOURCE img&
PRINT "abc";
c = POS(0)
_DEST _CONSOLE
PRINT "After three characters POS(0) is"; c
END
