$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (46_put_variable)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
OPEN "v35_x46.tmp" FOR BINARY AS #1
PUT #1, 1, x
CLOSE #1
KILL "v35_x46.tmp"
PRINT "ok"
SYSTEM
