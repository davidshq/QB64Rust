$CONSOLE:ONLY
' Slice (m3-mem task 4.2): runtime error 309, a _MEMPUT through a block that was never set, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d04_put_uninitialised).
ON ERROR GOTO handler
DIM SHARED gm AS _MEM
DIM m AS _MEM, n AS _MEM, a AS LONG, o AS _OFFSET
PRINT "before"
_MEMPUT m, 0, 1 AS LONG
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
SUB keep
    DIM l AS LONG
    gm = _MEM(l)
END SUB
