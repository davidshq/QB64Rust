$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (43_field_with_dyn)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
OPEN "v35_x43.tmp" FOR RANDOM AS #1 LEN = 8
PUT #1, 1, x
CLOSE #1
KILL "v35_x43.tmp"
PRINT "ok"
SYSTEM
