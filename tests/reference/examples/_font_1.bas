$CONSOLE:ONLY
img& = _NEWIMAGE(100, 40, 32)
_DEST img&
f0 = _FONT
_FONT 8
f8 = _FONT
_DEST _CONSOLE
PRINT "The default font is"; f0; "and after _FONT 8 it is"; f8
END
