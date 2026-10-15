$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (02_marker_after_type)
TYPE t
    v(1) AS LONG _DYNAMIC
END TYPE
DIM x AS t
REDIM x.v(2 TO 3) AS LONG
PRINT LBOUND(x.v); UBOUND(x.v)
SYSTEM
