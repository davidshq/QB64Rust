$CONSOLE:ONLY
' Slice (m3-mem task 4.2): runtime error 301, _MEM(offset, size) with a negative size, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d10_mem_negative_size).
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
