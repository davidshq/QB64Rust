$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (49_member_index_order)
TYPE t
    v(1) AS LONG
END TYPE
DIM SHARED e(2) AS t
e(1).v(1) = 9
e(f(2)).v() = e(f(1)).v()
PRINT e(2).v(1)
PRINT "ok"
SYSTEM
FUNCTION f (n)
    PRINT "f"; n
    f = n
END FUNCTION
