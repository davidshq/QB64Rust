$CONSOLE:ONLY
img& = _NEWIMAGE(320, 200, 32)
PRINT "A handle is negative:"; img& < -1
PRINT _WIDTH(img&); _HEIGHT(img&); _PIXELSIZE(img&)
t& = _NEWIMAGE(80, 25, 0)
PRINT "A text image of"; _WIDTH(t&); "by"; _HEIGHT(t&); "characters"
END
