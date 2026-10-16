$CONSOLE:ONLY
' Verification (m3-mem, task 1.2): error 301, _MEMCOPY with a negative number of bytes, under ON ERROR (errors 300 to 315 are not trapped).
ON ERROR GOTO handler
DIM SHARED gm AS _MEM
DIM m AS _MEM, n AS _MEM, a AS LONG, o AS _OFFSET
PRINT "before"
m = _MEMNEW(8): n = _MEMNEW(8)
_MEMCOPY m, m.OFFSET, -1 TO n, n.OFFSET
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
SUB keep
    DIM l AS LONG
    gm = _MEM(l)
END SUB
