$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (30_erase_list)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t, y AS t
ON ERROR GOTO h
ERASE x.v, y.v
PRINT UBOUND(x.v); UBOUND(y.v)
SYSTEM
h:
PRINT "[error"; ERR; "]";
RESUME NEXT
SYSTEM
