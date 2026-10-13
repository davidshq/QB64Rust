$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (90_const_min_max)
CONST c = _MIN(1, 2), d = _MAX(1, 2)
PRINT c; d
PRINT "ok"
SYSTEM
