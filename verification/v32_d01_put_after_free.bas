$CONSOLE:ONLY
' Verification (m3-mem, task 1.2): error 308, a _MEMPUT into freed memory, under ON ERROR (errors 300 to 315 are not trapped).
ON ERROR GOTO handler
DIM SHARED gm AS _MEM
DIM m AS _MEM, n AS _MEM, a AS LONG, o AS _OFFSET
PRINT "before"
m = _MEMNEW(8): _MEMFREE m
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
