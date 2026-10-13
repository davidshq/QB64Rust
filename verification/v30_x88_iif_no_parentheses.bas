$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (88_iif_no_parentheses)
DIM l AS LONG
l = _IIF 1, 2, 3
PRINT "ok"
SYSTEM
