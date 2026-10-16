$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (92_const_rol_ror)
CONST c = _ROL(1, 4), d = _ROR(1, 1)
PRINT c; d
PRINT "ok"
SYSTEM
