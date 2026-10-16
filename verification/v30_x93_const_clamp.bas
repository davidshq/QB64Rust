$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (93_const_clamp)
CONST c = _CLAMP(5, 1, 3)
PRINT c
PRINT "ok"
SYSTEM
