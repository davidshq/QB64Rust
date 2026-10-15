$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (32_member_scalar)
TYPE t
    v(2) AS LONG
    k AS LONG
END TYPE
DIM x AS t
x.v() = x.k
PRINT "ok"
SYSTEM
