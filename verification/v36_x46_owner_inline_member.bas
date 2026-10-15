$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (46_owner_inline_member)
TYPE own
    v(1) _DYNAMIC AS LONG
    k AS LONG
END TYPE
TYPE box
    l(1) AS own
END TYPE
DIM b AS box, a(1) AS own
a(1).k = 5
b.l() = a()
PRINT b.l(1).k
a() = b.l()
PRINT a(1).k
PRINT "ok"
SYSTEM
