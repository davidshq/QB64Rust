$CONSOLE:ONLY
' Verification (m3-mem, task 1.2): error 301, _MEM(offset, size) with a negative size, under ON ERROR (errors 300 to 315 are not trapped).
ON ERROR GOTO handler
DIM SHARED gm AS _MEM
DIM m AS _MEM, n AS _MEM, a AS LONG, o AS _OFFSET
PRINT "before"
o = _OFFSET(a)
m = _MEM(o, -1)
PRINT m.SIZE; m.TYPE
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
SUB keep
    DIM l AS LONG
    gm = _MEM(l)
END SUB
