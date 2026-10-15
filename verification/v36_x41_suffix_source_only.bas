$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (41_suffix_source_only)
DIM a(2) AS INTEGER, b%(2)
b%(0) = 5: b%(1) = 6
a() = b%()
PRINT a(0); a(1)
PRINT "ok"
SYSTEM
