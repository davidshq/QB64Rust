$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: an argument of -0.5
CONST z = CHR$(-0.5)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
