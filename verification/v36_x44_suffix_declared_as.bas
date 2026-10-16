$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (44_suffix_declared_as)
DIM a(2) AS INTEGER, b(2) AS INTEGER
b(0) = 5: b(1) = 6
a%() = b%()
PRINT a(0); a(1)
PRINT "ok"
SYSTEM
