$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (35_redim_target_type)
REDIM a(2) AS INTEGER
DIM b(2) AS LONG
a() = b()
PRINT "ok"
SYSTEM
