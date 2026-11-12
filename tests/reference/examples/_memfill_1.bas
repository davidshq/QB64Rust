$CONSOLE:ONLY
DIM b(10) AS INTEGER, m AS _MEM
m = _MEM(b())
_MEMFILL m, m.OFFSET, m.SIZE, 3 AS INTEGER
PRINT "b(0), b(10):"; b(0); b(10)
_MEMFILL m, m.OFFSET, 2, 255 AS _UNSIGNED _BYTE
PRINT "b(0) after two bytes of 255:"; b(0)
END
