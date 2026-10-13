$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (89_const_iif)
CONST c = _IIF(1, 2, 3)
PRINT c
PRINT "ok"
SYSTEM
