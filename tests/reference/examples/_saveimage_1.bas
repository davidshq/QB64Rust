$CONSOLE:ONLY
img& = _NEWIMAGE(8, 8, 32)
_DEST img&: PSET (3, 3), _RGB32(0, 255, 0)
_DEST _CONSOLE
_SAVEIMAGE "_saveimage_1.png", img&
back& = _LOADIMAGE("_saveimage_1.png", 32)
_SOURCE back&
PRINT "Loaded"; _WIDTH(back&); "by"; _HEIGHT(back&); ", pixel (3, 3) is "; HEX$(POINT(3, 3))
KILL "_saveimage_1.png"
END
