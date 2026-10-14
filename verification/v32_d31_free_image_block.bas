$CONSOLE:ONLY
' Verification (m3-mem, task 1.2): _MEMFREE of an image's block, then a read through it, under ON ERROR (errors 300 to 315 are not trapped).
ON ERROR GOTO handler
DIM SHARED gm AS _MEM
DIM m AS _MEM, n AS _MEM, a AS LONG, o AS _OFFSET
PRINT "before"
a = _NEWIMAGE(2, 2, 32): m = _MEMIMAGE(a)
_MEMFREE m
PRINT _MEMEXISTS(m)
PRINT _MEMGET(m, m.OFFSET, LONG)
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
SUB keep
    DIM l AS LONG
    gm = _MEM(l)
END SUB
