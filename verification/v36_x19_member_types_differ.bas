$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (19_member_types_differ)
TYPE t
    v(2) AS LONG
    w(2) AS INTEGER
END TYPE
DIM x AS t
x.v() = x.w()
PRINT "ok"
SYSTEM
