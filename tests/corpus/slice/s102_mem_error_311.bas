$CONSOLE:ONLY
' Slice (m3-mem task 4.2): runtime error 311, _MEMCOPY into a block that was never set, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d22_copy_dest_uninitialised).
ON ERROR GOTO handler
DIM SHARED gm AS _MEM
DIM m AS _MEM, n AS _MEM, a AS LONG, o AS _OFFSET
PRINT "before"
m = _MEMNEW(8)
_MEMCOPY m, m.OFFSET, 4 TO n, 0
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
SUB keep
    DIM l AS LONG
    gm = _MEM(l)
END SUB
