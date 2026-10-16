$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (50_put_element)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
REDIM p(1) AS t
OPEN "v35_x50.tmp" FOR BINARY AS #1
PUT #1, 1, p(0)
CLOSE #1
KILL "v35_x50.tmp"
PRINT "ok"
SYSTEM
