$CONSOLE:ONLY
' Verification (m3-mem, task 1.2): the block of a dynamic array after ERASE, under ON ERROR (errors 300 to 315 are not trapped).
ON ERROR GOTO handler
DIM SHARED gm AS _MEM
DIM m AS _MEM, n AS _MEM, a AS LONG, o AS _OFFSET
PRINT "before"
REDIM d(5) AS LONG: m = _MEM(d())
ERASE d
PRINT _MEMEXISTS(m)
_MEMPUT m, m.OFFSET, 1 AS LONG
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
SUB keep
    DIM l AS LONG
    gm = _MEM(l)
END SUB
