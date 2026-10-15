$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (25_bit_member)
TYPE t
    v(1) _DYNAMIC AS _BIT
END TYPE
DIM x AS t
x.v(1) = -1
PRINT x.v(1)
SYSTEM
