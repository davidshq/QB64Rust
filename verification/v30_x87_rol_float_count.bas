$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (87_rol_float_count)
DIM l AS LONG
l = 1
PRINT _ROL(l, 1.5); _ROL(l, 2.5); _ROR(l, 1.5)
PRINT "ok"
SYSTEM
