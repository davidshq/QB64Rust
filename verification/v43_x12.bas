$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: an argument of -0.6
CONST z = CHR$(-0.6)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
