$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (23_offset_long)
DIM a(2) AS _OFFSET, b(2) AS _INTEGER64
a() = b()
PRINT "ok"
SYSTEM
