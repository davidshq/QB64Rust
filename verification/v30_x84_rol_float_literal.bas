$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (84_rol_float_literal)
PRINT _ROL(1.5, 1); _ROR(2.5, 1)
PRINT "ok"
SYSTEM
