$CONSOLE:ONLY
' Verification (m2-numeric-types, D1): does the old compiler accept this? (bit_param)
DIM b AS _BIT * 5
b = 3
s b
PRINT b
SUB s (x AS _BIT * 5)
PRINT x
x = 17
END SUB
