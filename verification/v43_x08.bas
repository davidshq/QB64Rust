$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: a string plus a number
CONST z = CHR$(65) + 1
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
