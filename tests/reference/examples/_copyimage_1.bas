$CONSOLE:ONLY
a& = _NEWIMAGE(10, 10, 32)
_DEST a&: PSET (2, 3), _RGB32(1, 2, 3)
b& = _COPYIMAGE(a&)
_DEST a&: PSET (2, 3), _RGB32(9, 9, 9)
_SOURCE b&
_DEST _CONSOLE
PRINT "The copy kept the old pixel: "; HEX$(POINT(2, 3)); _WIDTH(b&)
END
