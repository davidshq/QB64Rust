$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: CONST then the string used as a number
CONST z = CHR$(65): y = z + 1
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
