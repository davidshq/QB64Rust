$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (SOMEDAY.md, the m3-typefields row): _MEMGET into a whole _DYNAMIC member array; one form, on the last line, as code that is not reached.
TYPE dy
    k AS LONG
    _DYNAMIC d(1 TO 3) AS LONG
END TYPE
DIM x AS dy
DIM m AS _MEM
PRINT "ok"
SYSTEM
_MEMGET m, m.OFFSET, x.d()
