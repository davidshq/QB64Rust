$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, by the review of group 2): does the old compiler accept this? (60_marker_before_as_list)
TYPE t
    _STATIC AS LONG v(1), w(2)
END TYPE
DIM x AS t
x.w(2) = 4
PRINT x.w(2)
SYSTEM
