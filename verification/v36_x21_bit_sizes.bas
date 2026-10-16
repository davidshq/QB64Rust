$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (21_bit_sizes)
DIM a(2) AS _BIT * 2, b(2) AS _BIT * 3
a() = b()
PRINT "ok"
SYSTEM
