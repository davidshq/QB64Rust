$CONSOLE:ONLY
img& = _NEWIMAGE(64, 48, 256)
PRINT _WIDTH(img&); _HEIGHT(img&)
_DEST img&
w = _WIDTH: h = _HEIGHT
_DEST _CONSOLE
PRINT "Of the destination:"; w; h
END
