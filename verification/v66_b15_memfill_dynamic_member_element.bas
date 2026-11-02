$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (SOMEDAY.md, the m3-typefields row): _MEMFILL with an element of a _DYNAMIC member array.
TYPE dy
    k AS LONG
    _DYNAMIC d(1 TO 3) AS LONG
END TYPE
DIM x AS dy
DIM m AS _MEM
m = _MEMNEW(8)
x.d(1) = 66
_MEMFILL m, m.OFFSET, 8, x.d(1)
PRINT _MEMGET(m, m.OFFSET, LONG); _MEMGET(m, m.OFFSET + 4, LONG)
SYSTEM
