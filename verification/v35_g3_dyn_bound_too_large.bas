$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): REDIM of a _DYNAMIC member to more elements than memory
' holds.
TYPE t
    v(1) _DYNAMIC AS _INTEGER64
END TYPE
ON ERROR GOTO h
DIM x AS t
DIM hi AS _INTEGER64: hi = 2147483647
PRINT "2. x.v(0 TO hi, 0 TO hi):";: REDIM x.v(0 TO hi, 0 TO hi) AS _INTEGER64: PRINT UBOUND(x.v)
PRINT "done"
SYSTEM
h:
PRINT "[error"; ERR; "at"; _ERRORLINE; "]";
RESUME NEXT
