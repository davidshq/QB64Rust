$CONSOLE:ONLY
img& = _NEWIMAGE(40, 30, 256)
_DEST img&
WINDOW (0, 0)-(100, 100)
x0 = PMAP(0, 0): x1 = PMAP(100, 0): y0 = PMAP(0, 1): y1 = PMAP(100, 1)
_DEST _CONSOLE
PRINT "x from 0 to 100 maps to pixels"; x0; "to"; x1
PRINT "y from 0 to 100 maps to pixels"; y0; "to"; y1
END
