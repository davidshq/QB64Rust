$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (22_bit_signed)
DIM a(2) AS _BIT, b(2) AS _UNSIGNED _BIT
a() = b()
PRINT "ok"
SYSTEM
