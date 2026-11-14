$CONSOLE:ONLY
img& = _NEWIMAGE(20, 20, 256)
PRINT "In a 256-color image, _RGB(255, 0, 0) is entry"; _RGB(255, 0, 0, img&)
t& = _NEWIMAGE(20, 20, 32)
PRINT "In a 32-bit image it is &H"; HEX$(_RGB(255, 0, 0, t&))
PRINT HEX$(_RGBA(0, 0, 255, 128, t&))
END
