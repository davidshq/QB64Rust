$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (24_suffix_mismatch)
DIM a&(2), b%(2)
a&() = b%()
PRINT "ok"
SYSTEM
