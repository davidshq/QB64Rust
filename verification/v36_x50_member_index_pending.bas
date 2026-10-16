$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (50_member_index_pending)
TYPE t
    v(1) AS LONG
END TYPE
ON ERROR GOTO h
DIM e(2) AS t, a(1) AS LONG
a(1) = 3
ERROR 7: e(1).v() = a()
PRINT e(1).v(1)
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
