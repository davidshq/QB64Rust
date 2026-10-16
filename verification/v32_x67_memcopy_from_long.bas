$CONSOLE:ONLY
' Verification (m3-mem, task 1.1): does the old compiler accept this? (67_memcopy_from_long)
TYPE t
    a AS INTEGER
    v(0 TO 3) AS LONG
END TYPE
DIM m AS _MEM, n AS _MEM, u AS t, k AS LONG, s AS STRING, o AS _OFFSET, a(3) AS LONG
DIM bt AS _BIT, ba(3) AS _BIT, sa(3) AS STRING
_MEMCOPY k, 0, 8 TO n, 0
PRINT "ok"
SYSTEM
SUB x (p AS _MEM)
END SUB
FUNCTION f (p AS _MEM)
END FUNCTION
