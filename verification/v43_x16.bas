$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: a string times a number
CONST z = CHR$(65) * 2
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
