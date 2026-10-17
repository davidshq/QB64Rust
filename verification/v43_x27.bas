$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: a SINGLE half after an expression
CONST z = CHR$(131 / 2)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
