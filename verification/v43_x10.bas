$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: an argument of 255.5
CONST z = CHR$(255.5)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
