$CONSOLE:ONLY
src& = _NEWIMAGE(10, 10, 32)
_DEST src&: CLS , _RGB32(0, 0, 255)
dst& = _NEWIMAGE(20, 20, 32)
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), src& TO(0, 0)-(19, 0)-(0, 19), dst&
_SOURCE dst&
_DEST _CONSOLE
PRINT HEX$(POINT(2, 2)); " "; HEX$(POINT(18, 18))
END
