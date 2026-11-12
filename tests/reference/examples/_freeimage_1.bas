$CONSOLE:ONLY
img& = _NEWIMAGE(10, 10, 32)
_FREEIMAGE img&
ON ERROR GOTO handler
PRINT _WIDTH(img&)
END

handler:
PRINT "The freed handle is no image: error"; ERR
RESUME NEXT
