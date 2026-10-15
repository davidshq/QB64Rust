$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (48_put_whole_array_varstring)
TYPE t
    v(1) _DYNAMIC AS STRING
END TYPE
REDIM p(1) AS t
OPEN "v35_x48.tmp" FOR BINARY AS #1
PUT #1, 1, p()
CLOSE #1
KILL "v35_x48.tmp"
PRINT "ok"
SYSTEM
