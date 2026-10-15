$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (31_mem_array)
DIM a(2) AS _MEM, b(2) AS _MEM
a() = b()
PRINT "ok"
SYSTEM
