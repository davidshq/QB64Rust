$CONSOLE:ONLY
src& = _NEWIMAGE(4, 4, 32)
_DEST src&: LINE (0, 0)-(3, 3), _RGB32(255, 0, 0), BF
dst& = _NEWIMAGE(20, 20, 32)
_PUTIMAGE (10, 10), src&, dst&
_PUTIMAGE (0, 0)-(7, 7), src&, dst&
_SOURCE dst&
_DEST _CONSOLE
PRINT HEX$(POINT(11, 11)); " "; HEX$(POINT(7, 7)); " "; HEX$(POINT(15, 15))
END
