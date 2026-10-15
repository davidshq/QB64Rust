$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (03_as_list_marker_first_only)
TYPE t
    AS LONG v(1) _DYNAMIC, w(1)
END TYPE
DIM x AS t
REDIM x.v(0 TO 5) AS LONG
PRINT UBOUND(x.v)
REDIM x.w(0 TO 5) AS LONG
PRINT UBOUND(x.w)
SYSTEM
