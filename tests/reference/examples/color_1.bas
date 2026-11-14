$CONSOLE:ONLY
img& = _NEWIMAGE(40, 30, 256)
_DEST img&: _SOURCE img&
COLOR 12, 1
CLS
PSET (1, 1)
_DEST _CONSOLE
PRINT "After CLS the background is"; POINT(30, 20); "and PSET draws in"; POINT(1, 1)
END
