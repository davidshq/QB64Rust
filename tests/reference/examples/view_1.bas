$CONSOLE:ONLY
img& = _NEWIMAGE(40, 30, 256)
_DEST img&: _SOURCE img&
VIEW (10, 10)-(29, 19)
PSET (0, 0), 7
VIEW
_DEST _CONSOLE
PRINT "PSET (0, 0) in the viewport lit"; POINT(10, 10); "at (10, 10); (0, 0) is"; POINT(0, 0)
END
