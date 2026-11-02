$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-mem row): _MEM of an element of a member array whose elements are of a TYPE.
TYPE q
    k AS LONG
    j AS LONG
END TYPE
TYPE u
    a AS LONG
    n(1 TO 3) AS q
END TYPE
DIM x(2) AS u
DIM m AS _MEM
m = _MEM(x(1).n(2))
PRINT m.SIZE; m.ELEMENTSIZE; m.TYPE
SYSTEM
