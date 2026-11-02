$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (SOMEDAY.md, the m3-typefields row): _MEMPUT of an element of a _DYNAMIC member array.
TYPE dy
    k AS LONG
    _DYNAMIC d(1 TO 3) AS LONG
END TYPE
DIM x AS dy
DIM m AS _MEM
m = _MEMNEW(4)
x.d(2) = 55
_MEMPUT m, m.OFFSET, x.d(2)
PRINT _MEMGET(m, m.OFFSET, LONG)
SYSTEM
