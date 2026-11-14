$CONSOLE:ONLY
img& = _NEWIMAGE(40, 30, 256)
_DEST img&: _SOURCE img&
WINDOW (0, 0)-(100, 100)
PSET (50, 50), 6
x = PMAP(50, 0): y = PMAP(50, 1)
WINDOW
_DEST _CONSOLE
PRINT "(50, 50) of the WINDOW is pixel"; x; y; "and holds"; POINT(x, y)
END
