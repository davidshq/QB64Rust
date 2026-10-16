$CONSOLE:ONLY
' Slice (m3-mem task 4.2): runtime error 310, _MEMCOPY from a block that was never set, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d21_copy_source_uninitialised).
ON ERROR GOTO handler
DIM SHARED gm AS _MEM
DIM m AS _MEM, n AS _MEM, a AS LONG, o AS _OFFSET
PRINT "before"
n = _MEMNEW(8)
_MEMCOPY m, 0, 4 TO n, n.OFFSET
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
SUB keep
    DIM l AS LONG
    gm = _MEM(l)
END SUB
