$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (33_member_of_element_array)
TYPE t
    v(2) AS LONG
END TYPE
DIM e(2) AS t, a(2) AS LONG
a(1) = 4
e(1).v() = a()
PRINT e(1).v(1)
PRINT "ok"
SYSTEM
