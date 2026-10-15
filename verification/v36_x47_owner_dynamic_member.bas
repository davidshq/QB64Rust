$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (47_owner_dynamic_member)
TYPE own
    v(1) _DYNAMIC AS LONG
    k AS LONG
END TYPE
TYPE box
    d(1) _DYNAMIC AS own
END TYPE
DIM b AS box, a(1) AS own
a(1).k = 6
b.d() = a()
PRINT b.d(1).k
a() = b.d()
PRINT a(1).k
PRINT "ok"
SYSTEM
