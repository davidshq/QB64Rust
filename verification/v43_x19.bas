$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: CHR without the dollar
CONST z = CHR(65)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
