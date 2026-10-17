$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: a number plus a string
CONST z = 1 + CHR$(65)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
