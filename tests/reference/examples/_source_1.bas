$CONSOLE:ONLY
img& = _NEWIMAGE(20, 20, 256)
_DEST img&: PSET (2, 2), 11
_DEST _CONSOLE
_SOURCE img&
PRINT "POINT reads the source:"; POINT(2, 2); "and _SOURCE is img&:"; _SOURCE = img&
END
