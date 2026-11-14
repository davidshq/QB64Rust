$CONSOLE:ONLY
img& = _NEWIMAGE(40, 30, 256)
_DEST img&: _SOURCE img&
LINE (5, 5)-(25, 20), 9, B
PAINT (10, 10), 14, 9
_DEST _CONSOLE
PRINT "Inside:"; POINT(10, 10); "border:"; POINT(5, 5); "outside:"; POINT(30, 25)
END
