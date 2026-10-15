$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (04_as_list_marker_before_name)
TYPE t
    AS LONG _DYNAMIC v(1), w(1)
END TYPE
DIM x AS t
REDIM x.v(0 TO 5) AS LONG
PRINT UBOUND(x.v)
SYSTEM
