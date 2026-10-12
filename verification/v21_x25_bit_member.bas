$CONSOLE:ONLY
' Verification (m2-numeric-types, D1): does the old compiler accept this? (bit_member)
TYPE r
    b AS _BIT * 4
END TYPE
DIM v AS r
