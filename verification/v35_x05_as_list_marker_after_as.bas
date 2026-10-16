$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (05_as_list_marker_after_as)
TYPE t
    AS _DYNAMIC LONG v(1)
END TYPE
DIM x AS t
REDIM x.v(0 TO 5) AS LONG
PRINT UBOUND(x.v)
SYSTEM
