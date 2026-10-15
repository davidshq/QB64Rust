$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (51_mem_of_owner_member)
TYPE own
    v(1) _DYNAMIC AS LONG
    k AS LONG
END TYPE
REDIM s(1) AS own, d(1) AS own
DIM d2(1) AS own
s(1).k = 4
DIM m AS _MEM, m2 AS _MEM
m = _MEM(d(1).k): m2 = _MEM(d2(1).k)
d() = s(): d2() = s()
PRINT _MEMEXISTS(m); _MEMEXISTS(m2); d(1).k; d2(1).k
PRINT "ok"
SYSTEM
