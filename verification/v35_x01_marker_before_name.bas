$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (01_marker_before_name)
TYPE t
    _DYNAMIC v(1) AS LONG
END TYPE
DIM x AS t
REDIM x.v(2 TO 3) AS LONG
x.v(3) = 3
PRINT LBOUND(x.v); UBOUND(x.v); x.v(3)
SYSTEM
