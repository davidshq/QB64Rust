$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (29_redim_list)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t, y AS t
REDIM x.v(0 TO 2) AS LONG, y.v(0 TO 3) AS LONG
PRINT UBOUND(x.v); UBOUND(y.v)
SYSTEM
