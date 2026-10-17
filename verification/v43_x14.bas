$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: a variable as the argument
x = 65: CONST z = CHR$(x)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
