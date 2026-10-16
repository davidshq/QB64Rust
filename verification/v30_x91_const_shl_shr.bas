$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (91_const_shl_shr)
CONST c = _SHL(1, 4), d = _SHR(16, 2)
PRINT c; d
PRINT "ok"
SYSTEM
