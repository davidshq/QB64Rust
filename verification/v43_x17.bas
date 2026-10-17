$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: a comparison of strings
CONST z = CHR$(65) = "A"
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
