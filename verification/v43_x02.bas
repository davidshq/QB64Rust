$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: an argument of -1
CONST z = CHR$(-1)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
