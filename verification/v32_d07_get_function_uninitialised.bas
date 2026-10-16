$CONSOLE:ONLY
' Verification (m3-mem, task 1.2): error 309, the _MEMGET function through a block that was never set, under ON ERROR (errors 300 to 315 are not trapped).
ON ERROR GOTO handler
DIM SHARED gm AS _MEM
DIM m AS _MEM, n AS _MEM, a AS LONG, o AS _OFFSET
PRINT "before"
PRINT _MEMGET(m, 0, LONG)
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
SUB keep
    DIM l AS LONG
    gm = _MEM(l)
END SUB
