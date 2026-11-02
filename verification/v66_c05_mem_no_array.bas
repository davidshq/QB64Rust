$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m4-names-arrays row): _MEM(x()) of a name that is no array, then a use of x(2).
DIM m AS _MEM
m = _MEM(x())
PRINT m.SIZE; m.ELEMENTSIZE; m.TYPE
x(2) = 5
PRINT x(2); UBOUND(x)
SYSTEM
