$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (52_owner_bounds_differ)
TYPE own
    v(1) _DYNAMIC AS LONG
    k AS LONG
END TYPE
ON ERROR GOTO h
DIM s(1 TO 2) AS own, d(0 TO 1) AS own, e(1 TO 3) AS own
s(1).k = 4: s(2).k = 5: d(0).k = 9: e(1).k = 8
PRINT "moved:";: d() = s(): PRINT d(0).k; d(1).k
PRINT "more:";: e() = s(): PRINT e(1).k; e(2).k; e(3).k
REDIM r(1 TO 2) AS own
ERASE r
PRINT "erased:";: r() = s(): PRINT " done"
PRINT "ok"
SYSTEM
h:
PRINT " error"; ERR;
RESUME NEXT
