$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: an argument of 256
CONST z = CHR$(256)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
