$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (85_rol_single_variable)
DIM s AS SINGLE, d AS DOUBLE
s = 3: d = 3
PRINT _ROL(s, 1); _ROR(d, 1)
PRINT "ok"
SYSTEM
