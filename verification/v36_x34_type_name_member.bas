$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (34_type_name_member)
TYPE t
    v(2) AS LONG
END TYPE
DIM e(2) AS t, f(2) AS t
f() = e(1).v()
PRINT "ok"
SYSTEM
