$CONSOLE:ONLY
' Verification (m2-numeric-types, D1): does the old compiler accept this? (len_bit_40)
DIM t AS _BIT * 40
PRINT LEN(t)
