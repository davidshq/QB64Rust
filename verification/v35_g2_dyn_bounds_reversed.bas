$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): REDIM of a _DYNAMIC member with an upper bound below the
' lower one, from variables (constant bounds are "Invalid array bounds" at compile time).
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
ON ERROR GOTO h
DIM x AS t
DIM lo AS LONG, hi AS LONG: lo = 3: hi = 1
PRINT "2. x.v(lo TO hi):";: REDIM x.v(lo TO hi) AS LONG: PRINT LBOUND(x.v); UBOUND(x.v)
PRINT "done"
SYSTEM
h:
PRINT "[error"; ERR; "at"; _ERRORLINE; "]";
RESUME NEXT
