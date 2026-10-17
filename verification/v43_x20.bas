$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: a constant with a suffix as the argument
CONST n = 65: CONST z = CHR$(n%)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
