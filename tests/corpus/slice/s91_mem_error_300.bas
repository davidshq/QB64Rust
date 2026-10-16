$CONSOLE:ONLY
' Slice (m3-mem task 4.2): runtime error 300, a _MEMPUT that reaches beyond the block, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d02_put_out_of_range).
ON ERROR GOTO handler
DIM SHARED gm AS _MEM
DIM m AS _MEM, n AS _MEM, a AS LONG, o AS _OFFSET
PRINT "before"
m = _MEM(a)
_MEMPUT m, m.OFFSET + 2, 1 AS LONG
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
SUB keep
    DIM l AS LONG
    gm = _MEM(l)
END SUB
