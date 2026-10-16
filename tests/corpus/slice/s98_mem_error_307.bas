$CONSOLE:ONLY
' Slice (m3-mem task 4.2): runtime error 307, _MEMFREE of a block freed already, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d25_free_twice).
ON ERROR GOTO handler
DIM SHARED gm AS _MEM
DIM m AS _MEM, n AS _MEM, a AS LONG, o AS _OFFSET
PRINT "before"
m = _MEMNEW(8): _MEMFREE m
_MEMFREE m
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
SUB keep
    DIM l AS LONG
    gm = _MEM(l)
END SUB
