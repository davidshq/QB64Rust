$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: a _FLOAT argument
CONST z = CHR$(65.5##)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
