$CONSOLE:ONLY
img& = _NEWIMAGE(80, 30, 32)
_DEST img&
h = _UFONTHEIGHT: s = _ULINESPACING
_DEST _CONSOLE
PRINT "The font is"; h; "pixels high, with"; s; "between lines."
END
