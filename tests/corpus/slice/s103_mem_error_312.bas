$CONSOLE:ONLY
' Slice (m3-mem task 4.2): runtime error 312, _MEMCOPY with neither block set, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d23_copy_both_uninitialised).
ON ERROR GOTO handler
DIM SHARED gm AS _MEM
DIM m AS _MEM, n AS _MEM, a AS LONG, o AS _OFFSET
PRINT "before"
_MEMCOPY m, 0, 4 TO n, 0
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
SUB keep
    DIM l AS LONG
    gm = _MEM(l)
END SUB
