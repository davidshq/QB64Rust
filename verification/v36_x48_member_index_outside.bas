$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (48_member_index_outside)
TYPE t
    v(1) AS LONG
    w(1) AS STRING
END TYPE
ON ERROR GOTO h
DIM e(2) AS t, a(1) AS LONG, s(1) AS STRING
a(1) = 7: s(1) = "z"
i = 5
e(i).v() = a(): e(i).w() = s()
PRINT e(0).v(1); "["; e(0).w(1); "]"
a() = e(i).v(): s() = e(i).w()
PRINT a(1); "["; s(1); "]"
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
