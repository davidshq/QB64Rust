$CONSOLE:ONLY
DIM source(1 TO 5) AS INTEGER, target(1 TO 5) AS INTEGER
FOR i = 1 TO 5: source(i) = i * 10: NEXT
DIM ms AS _MEM, mt AS _MEM
ms = _MEM(source())
mt = _MEM(target())
' Copy elements 2 to 4 (three INTEGERs of 2 bytes each) to the start of target().
_MEMCOPY ms, ms.OFFSET + 2, 3 * 2 TO mt, mt.OFFSET
FOR i = 1 TO 5: PRINT target(i);: NEXT
PRINT
_MEMFREE ms
_MEMFREE mt
END
