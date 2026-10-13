$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (99_iif_array_parts)
DIM a(3) AS LONG, b(3) AS LONG
PRINT _IIF(1, a(1), b(2)); _IIF(0, a(), b())
PRINT "ok"
SYSTEM
