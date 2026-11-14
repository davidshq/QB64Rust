$CONSOLE:ONLY
img& = _NEWIMAGE(4, 4, 32)
PRINT "Blending is on:"; _BLEND(img&)
_DONTBLEND img&
PRINT "After _DONTBLEND:"; _BLEND(img&)
_BLEND img&
PRINT "After _BLEND:"; _BLEND(img&)
END
